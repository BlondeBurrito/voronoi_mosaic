//! Voronoi Tessellation is a series of regions (also called cells) which are
//! defined by a point in space (known as a site , seed or generating point)
//! for which the cell consists of all points in space that are closer to the
//! cell-site than any other cell-site
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
