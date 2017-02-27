//! Implementation of the [DBSCAN](https://en.wikipedia.org/wiki/DBSCAN)
//! clustering algorithm.

use matrix::SymmetricMatrix;
use std::collections::VecDeque;

/// Implementation of the [DBSCAN](https://en.wikipedia.org/wiki/DBSCAN)
/// clustering algorithm.
#[derive(Debug)]
pub struct DBSCAN<T> {
    eps: T,
    min_points: usize,
    clusters: Vec<Option<usize>>,
    visited: Vec<bool>,
    current_cluster: usize,
}

impl<T> DBSCAN<T>
    where T: Default + Copy + PartialOrd
{
    /// Creates a new DBSCAN instance.
    ///
    /// # Parameters
    ///
    /// * `eps` - The maximum distance between two points for them to be in the
    ///   same neighborhood.
    /// * `min_points` - The minimal number of points in a neighborhood for a
    ///   point to be considered as a core point.
    pub fn new(eps: T, min_points: usize) -> Self {
        DBSCAN {
            eps: eps,
            min_points: min_points,
            clusters: Vec::new(),
            visited: Vec::new(),
            current_cluster: 0,
        }
    }

    /// Performs DBSCAN clustering from the given distance matrix.
    ///
    /// # Returns
    ///
    /// Returns cluster labels for each point in the dataset. Noisy samples are
    /// set to `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dbscan::DBSCAN;
    /// use dbscan::SymmetricMatrix;
    ///
    /// let mut dbscan = DBSCAN::new(1, 2);
    /// let mut m = SymmetricMatrix::<i8>::new(5);
    /// m.set(0, 1, 1);
    /// m.set(0, 2, 9);
    /// m.set(0, 3, 9);
    /// m.set(0, 4, 9);
    /// m.set(1, 2, 9);
    /// m.set(1, 3, 9);
    /// m.set(1, 4, 9);
    /// m.set(2, 3, 1);
    /// m.set(2, 4, 9);
    /// m.set(3, 4, 9);
    ///
    /// let clustering = dbscan.perform_clustering(&m);
    ///
    /// assert_eq!(clustering[0], Some(0));
    /// assert_eq!(clustering[1], Some(0));
    /// assert_eq!(clustering[2], Some(1));
    /// assert_eq!(clustering[3], Some(1));
    /// assert_eq!(clustering[4], None);
    /// ```
    ///
    /// In the above example, points `0` and `1` form a single cluster, points
    /// `2` and `3` form a different cluster, and point `4` does not belong any
    /// cluster (it is a noise point).
    pub fn perform_clustering(&mut self,
                              matrix: &SymmetricMatrix<T>) -> &Vec<Option<usize>> {
        self.clusters = vec![None; matrix.size()];
        self.visited = vec![false; matrix.size()];
        self.current_cluster = 0;

        for point in 0..matrix.size() {
            if self.visited[point] {
                continue;
            }

            self.visited[point] = true;
            let neighbors = self.region_query(matrix, point);
            if neighbors.len() >= self.min_points {
                self.expand_cluster(matrix, point, neighbors);
                self.current_cluster += 1;
            }
        }

        self.clusters.as_ref()
    }

    fn expand_cluster(&mut self,
                      matrix: &SymmetricMatrix<T>,
                      point: usize,
                      mut neighbors: VecDeque<usize>) {
        self.clusters[point] = Some(self.current_cluster);

        while let Some(other_point) = neighbors.pop_front() {
            if !self.visited[other_point] {
                self.visited[other_point] = true;
                let mut other_neighbors = self.region_query(matrix, other_point);
                if other_neighbors.len() >= self.min_points {
                    neighbors.append(&mut other_neighbors);
                }
            }
            if self.clusters[other_point].is_none() {
                self.clusters[other_point] = Some(self.current_cluster);
            }
        }
    }

    fn region_query(&self,
                    matrix: &SymmetricMatrix<T>,
                    point: usize) -> VecDeque<usize> {
        let mut neighbors = VecDeque::new();
        for other_point in 0..matrix.size() {
            let dist = matrix.get(point, other_point);
            if dist <= self.eps {
                neighbors.push_back(other_point);
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use matrix::SymmetricMatrix;

    #[test]
    fn test_all_points_are_in_single_cluster_when_their_distance_is_zero() {
        let mut dbscan = DBSCAN::new(1, 2);
        let m = SymmetricMatrix::<i8>::new(2);

        let clustering = dbscan.perform_clustering(&m);

        assert_eq!(clustering[0], Some(0));
        assert_eq!(clustering[1], Some(0));
    }

    #[test]
    fn test_points_are_correctly_clustered_based_on_their_distance() {
        let mut dbscan = DBSCAN::new(1, 2);
        let mut m = SymmetricMatrix::<i8>::new(5);
        m.set(0, 1, 1);
        m.set(0, 2, 9);
        m.set(0, 3, 9);
        m.set(0, 4, 9);
        m.set(1, 2, 9);
        m.set(1, 3, 9);
        m.set(1, 4, 9);
        m.set(2, 3, 1);
        m.set(2, 4, 9);
        m.set(3, 4, 9);

        let clustering = dbscan.perform_clustering(&m);

        assert_eq!(clustering[0], Some(0));
        assert_eq!(clustering[1], Some(0));
        assert_eq!(clustering[2], Some(1));
        assert_eq!(clustering[3], Some(1));
        assert_eq!(clustering[4], None);
    }

    #[test]
    fn test_neighboring_points_are_put_into_cluster_even_if_they_have_been_visited() {
        // In 2D, the points in this test are placed as follows:
        //
        //    0
        //      1
        //        2
        //
        // Epsilon is set to 1 and min_points to 3. When the first point is
        // checked (0), it is marked as visited. Since it has only a single
        // neighbor, the two points (0 and 1) cannot form a cluster because
        // min_points is 3. Then, the algorithm continues to point 1. It has
        // two neighbors (0 and
        // 2), so the three points (0, 1, 2) can form a cluster. In this test,
        // we ensure that even when the first point (0) has already been
        // marked as visited, it is put into the cluster because it is not
        // yet a member of any other cluster.
        let mut dbscan = DBSCAN::new(1, 3);
        let mut m = SymmetricMatrix::<i8>::new(3);
        m.set(0, 1, 1);
        m.set(0, 2, 2);
        m.set(1, 2, 1);

        let clustering = dbscan.perform_clustering(&m);

        assert_eq!(clustering[0], Some(0));
        assert_eq!(clustering[1], Some(0));
        assert_eq!(clustering[2], Some(0));
    }

    #[test]
    fn test_points_that_do_not_belong_to_any_cluster_are_none() {
        let mut dbscan = DBSCAN::new(1, 2);
        let m = SymmetricMatrix::<i8>::new(1);

        let clustering = dbscan.perform_clustering(&m);

        assert_eq!(clustering[0], None);
    }
}
