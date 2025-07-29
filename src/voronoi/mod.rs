//! TODO
//!
//!

use bevy::prelude::*;

pub mod voronoi_2d;
pub mod voronoi_3d;

/// Describes the Voronoi Cells
pub struct VoronoiData<T> {
	cells: Vec<T>,
}
