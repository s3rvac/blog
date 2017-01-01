//! Implementation of the [DBSCAN](https://en.wikipedia.org/wiki/DBSCAN)
//! clustering algorithm from distance matrix.
//!
//! The implementation is described in [this blog
//! post](https://blog.petrzemek.net/2017/01/01/implementing-dbscan-from-distance-matrix-in-rust/).

pub mod dbscan;
pub use dbscan::DBSCAN;

pub mod matrix;
pub use matrix::SymmetricMatrix;
