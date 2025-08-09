//! TODO
//!
//!

use std::collections::BTreeMap;

pub mod voronoi_2d;
#[cfg(feature = "3d_unstable")]
pub mod voronoi_3d;

/// Describes the Voronoi Cells
pub struct VoronoiData<T> {
	/// Each cell is uniquely identified by [u32]
	cells: BTreeMap<u32, T>,
}
