//! TODO
//!
//!

use std::collections::HashMap;

pub mod voronoi_2d;
pub mod voronoi_3d;

/// Describes the Voronoi Cells
pub struct VoronoiData<T> {
	/// Each cell is uniquely identified by [u32]
	cells: HashMap<u32, T>,
}
