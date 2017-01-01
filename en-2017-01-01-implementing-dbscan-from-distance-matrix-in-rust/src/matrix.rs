//! A generic [symmetric
//! matrix](https://en.wikipedia.org/wiki/Symmetric_matrix) to be used for
//! DBSCAN.

/// A generic [symmetric matrix](https://en.wikipedia.org/wiki/Symmetric_matrix).
#[derive(Debug)]
pub struct SymmetricMatrix<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> SymmetricMatrix<T>
    where T: Default + Copy
{
    /// Constructs a new symmetric matrix with the given number of rows and
    /// columns.
    ///
    /// Each cell in the matrix is initialized to `T::default()`. For example,
    /// when `T` is an integral type, each cell is initialized to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dbscan::SymmetricMatrix;
    ///
    /// let m: SymmetricMatrix<i32> = SymmetricMatrix::new(5);
    /// ```
    pub fn new(size: usize) -> Self {
        SymmetricMatrix {
            size: size,
            data: vec![T::default(); (size + 1) * size / 2],
        }
    }

    /// Returns the number of rows and columns of the matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// use dbscan::SymmetricMatrix;
    ///
    /// let m: SymmetricMatrix<i32> = SymmetricMatrix::new(5);
    /// assert_eq!(m.size(), 5);
    /// ```
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the value of the given cell.
    ///
    /// Since the matrix is symmetric, `get(x, y)` returns the same value as
    /// `get(y, x)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dbscan::SymmetricMatrix;
    ///
    /// let mut m = SymmetricMatrix::new(5);
    /// m.set(1, 2, 42);
    /// assert_eq!(m.get(1, 2), 42);
    /// assert_eq!(m.get(2, 1), 42);
    /// ```
    ///
    /// # Panics
    ///
    /// When the given cell does not exist. For example, when the size of the
    /// matrix is `5`, calling `get(5, 1)` will panic.
    pub fn get(&self, row: usize, col: usize) -> T {
        let index = self.index_for(row, col);
        self.data[index]
    }

    /// Sets the value of the given cell.
    ///
    /// Since the matrix is symmetric, `set(x, y, value)` automatically sets
    /// the value of `(y, x)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dbscan::SymmetricMatrix;
    ///
    /// let mut m = SymmetricMatrix::new(5);
    /// m.set(1, 2, 42);
    /// assert_eq!(m.get(1, 2), 42);
    /// assert_eq!(m.get(2, 1), 42);
    /// ```
    ///
    /// # Panics
    ///
    /// When the given cell does not exist. For example, when the size of the
    /// matrix is `5`, calling `set(5, 1, X)` will panic.
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let index = self.index_for(row, col);
        self.data[index] = value;
    }

    fn index_for(&self, row: usize, col: usize) -> usize {
        if col > row {
            col * (col + 1) / 2 + row
        } else {
            row * (row + 1) / 2 + col
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_returns_correct_value() {
        let m = SymmetricMatrix::<i8>::new(5);

        assert_eq!(m.size(), 5);
    }

    #[test]
    fn test_get_after_set_returns_value_that_has_been_set() {
        let mut m = SymmetricMatrix::new(5);
        m.set(2, 1, 10);

        assert_eq!(m.get(2, 1), 10);
    }

    #[test]
    fn test_matrix_is_symmetric() {
        let mut m = SymmetricMatrix::new(5);
        m.set(3, 2, 10);
        m.set(4, 0, 11);

        assert_eq!(m.get(2, 3), 10);
        assert_eq!(m.get(0, 4), 11);
    }

    #[test]
    fn test_all_cells_are_initialized_to_default_when_matrix_is_created() {
        let m = SymmetricMatrix::<i8>::new(2);

        assert_eq!(m.get(0, 0), 0);
        assert_eq!(m.get(0, 1), 0);
        assert_eq!(m.get(1, 0), 0);
        assert_eq!(m.get(1, 1), 0);
    }

    #[test]
    #[should_panic]
    fn test_get_panics_when_index_is_out_of_bounds() {
        let m = SymmetricMatrix::<i8>::new(5);

        m.get(100, 100);
    }

    #[test]
    #[should_panic]
    fn test_set_panics_when_index_is_out_of_bounds() {
        let mut m = SymmetricMatrix::new(5);

        m.set(100, 100, 1);
    }

    #[test]
    fn test_matrix_can_be_debug_printed() {
        let m = SymmetricMatrix::<i8>::new(2);

        let repr = format!("{:?}", m);

        assert_eq!(repr, "SymmetricMatrix { size: 2, data: [0, 0, 0] }");
    }
}
