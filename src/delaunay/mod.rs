//! Construct a series of triangles between points such that no point sits within a
//! triangle, they all become vertices
//!
//! Bowyer-Watson algorithm - use circumcircles to determine if a triangulation is valid

use bevy::prelude::*;

pub mod delaunay_2d;
pub mod delaunay_3d;

/// Describes the triangles in Delaunay Triangulation
pub struct DelaunayData<T> {
	shapes: Vec<T>,
}
