//! Construct a series of triangles between points such that no point sits within a
//! the circumcircle of any triangle
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
