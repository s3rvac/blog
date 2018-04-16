//! A sample implementation of `multiprocessing.pool.ThreadPool` from Python in
//! Rust.
//!
//! Only a single method is implemented (`imap()`). The implementation of other
//! methods is straightforward and is left to the reader.

extern crate num_cpus;
extern crate threadpool;

use std::collections::BTreeMap;
use std::sync::Arc;
use std::sync::mpsc;

/// A thread pool with an interface similar to that of
/// `multiprocessing.pool.ThreadPool` from Python.
pub struct ThreadPool {
    /// Internal representation of the thread pool.
    pool: threadpool::ThreadPool,
}

impl ThreadPool {
    /// Creates a new thread pool, where the number of worker threads is
    /// detected automatically based on the number of available CPUs in the
    /// system.
    pub fn new() -> Self {
        let worker_count = num_cpus::get();
        ThreadPool::with_workers(worker_count)
    }

    /// Creates a new thread pool with the given number of worker threads.
    ///
    /// # Panics
    ///
    /// When `count` is negative or zero.
    pub fn with_workers(count: usize) -> Self {
        assert!(count > 0, format!("worker count cannot be {}", count));

        ThreadPool {
            pool: threadpool::ThreadPool::new(count),
        }
    }

    /// Returns the number of workers in the pool.
    pub fn worker_count(&self) -> usize {
        self.pool.max_count()
    }

    /// Applies `f` to all values in `inputs` in parallel and returns an
    /// iterator over the results.
    ///
    /// The order of the returned results matches the order of inputs. That is,
    /// if you pass it the increment function and `[1, 2, 3]`, you will always
    /// get the results in this order: `2`, `3`, `4`.
    pub fn imap<F, I, T, R>(&self, f: F, inputs: I) -> IMapIterator<R>
        where F: Fn(T) -> R + Send + Sync + 'static,
              I: IntoIterator<Item = T>,
              T: Send + 'static,
              R: Send + 'static,
    {
        // We need to wrap the function with Arc so it can be passed to
        // multiple threads.
        let f = Arc::new(f);

        // We use a multiple-producers single-consumer (MPSC) channel to
        // transfer the results from the thread pool to the user.
        let (tx, rx) = mpsc::channel();

        // Submit `f(input)` for computation in the thread pool, for each
        // input. We need to keep the total count of submitted computations so
        // we know when to stop reading results from the channel in
        // IMapIterator::next().
        let mut total = 0;
        for (i, input) in inputs.into_iter().enumerate() {
            total += 1;
            let f = f.clone();
            let tx = tx.clone();
            self.pool.execute(move || {
                let result = f(input);
                if let Err(_) = tx.send((i, result)) {
                    // Sending of the result has failed, which means that the
                    // receiving side has hung up. There is nothing to do but
                    // to ignore the result.
                }
            });
        }
        IMapIterator::new(rx, total)
    }
}

/// An iterator over results from `ThreadPool::imap()`.
pub struct IMapIterator<T> {
    /// The receiving end of the channel created in `ThreadPool::imap()`.
    rx: mpsc::Receiver<(usize, T)>,

    /// As `imap()` preserves the order of the returned results (in contrast to
    /// `imap_unordered()`), we need a mapping of "indexes" into the original
    /// input iterator to their corresponding results.
    results: BTreeMap<usize, T>,

    /// Index of the next result that we should yield in `next()`.
    next: usize,

    /// The total number of results that should be yielded (so we know when to
    /// stop reading results from the channel.
    total: usize,
}

impl<T> IMapIterator<T> {
    /// Creates a new iterator.
    fn new(rx: mpsc::Receiver<(usize, T)>, total: usize) -> Self {
        IMapIterator {
            rx: rx,
            results: BTreeMap::new(),
            next: 0,
            total: total,
        }
    }
}

impl<T> Iterator for IMapIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.next < self.total {
            // Do we already have a result for the next index?
            if let Some(result) = self.results.remove(&self.next) {
                self.next += 1;
                return Some(result);
            }

            // We do not, so try receiving the next result.
            let (i, result) = match self.rx.recv() {
                Ok((i, result)) => (i, result),
                Err(_) => {
                    // Receiving has failed, which means that the sending side
                    // has hung up. There will be no more results.
                    self.next = self.total;
                    break;
                },
            };
            assert!(i >= self.next, format!("got {}, next is {}", i, self.next));
            assert!(!self.results.contains_key(&i), format!("{} already exists", i));
            self.results.insert(i, result);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_number_of_workers_automatically() {
        let pool = ThreadPool::new();

        assert_eq!(pool.worker_count(), num_cpus::get());
    }

    #[test]
    fn with_workers_creates_pool_with_given_number_of_workers() {
        let pool = ThreadPool::with_workers(4);

        assert_eq!(pool.worker_count(), 4);
    }

    #[test]
    fn imap_returns_results_in_correct_order() {
        let pool = ThreadPool::new();
        let mut results = Vec::new();

        for result in pool.imap(|n| n * 2, &[1, 2, 3, 4, 5]) {
            results.push(result);
        }

        assert_eq!(results, &[2, 4, 6, 8, 10]);
    }

    #[test]
    fn imap_can_be_called_twice_in_row() {
        let pool = ThreadPool::new();
        let mut results = Vec::new();

        for result in pool.imap(|n| n * 2, &[1, 2, 3]) {
            results.push(result);
        }
        for result in pool.imap(|n| n * 2, &[4, 5, 6]) {
            results.push(result);
        }

        assert_eq!(results, &[2, 4, 6, 8, 10, 12]);
    }

    #[test]
    fn imap_can_be_called_with_vec() {
        let pool = ThreadPool::new();
        let mut results = Vec::new();
        let inputs = vec![1, 2, 3, 4, 5];

        for result in pool.imap(|n| n * 2, inputs) {
            results.push(result);
        }

        assert_eq!(results, &[2, 4, 6, 8, 10]);
    }

    #[test]
    fn does_not_panic_when_imap_is_destroyed_before_it_is_consumed() {
        let pool = ThreadPool::new();

        // This immediately drops the returned IMapIterator.
        pool.imap(|n| n * 2, vec![1_000; 0]);

        // There is no assertion. We just want to check that we do not panic
        // when the returned IMapIterator is destroyed before it is consumed.
    }

    #[test]
    fn does_not_panic_when_pool_is_destroyed_before_imap_is_consumed() {
        let pool = ThreadPool::new();
        let results = pool.imap(|n| n * 2, vec![1_000; 0]);

        ::std::mem::drop(pool);

        // There is no assertion. We just want to check that we do not panic
        // when the pool is destroyed before we consume the result from imap().
        results.collect::<Vec<_>>();
    }
}
