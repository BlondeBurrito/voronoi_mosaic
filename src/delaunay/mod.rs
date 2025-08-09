//! Delaunay describes the partitioning of a plane based on a series of data
//! points into a set of triangles whereby no circumcircle contains any of the
//! data points
//!

use bevy::prelude::*;

pub mod delaunay_2d;
#[cfg(feature = "3d_unstable")]
pub mod delaunay_3d;

/// Describes the triangles in Delaunay Triangulation
pub struct DelaunayData<T> {
	/// List of triangulations/tetrahedralizations
	shapes: Vec<T>,
}
