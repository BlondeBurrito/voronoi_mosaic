//! TODO
//!
//!

use std::collections::BTreeMap;

pub mod voronoi_2d;
pub mod voronoi_3d;

/// Describes the Voronoi Cells
pub struct VoronoiData<T> {
	/// Each cell is uniquely identified by [u32]
	cells: BTreeMap<u32, T>,
}
