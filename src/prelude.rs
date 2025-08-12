//! `use voronoi_mosaic::prelude::*;` to import common structures and methods
//!

#[doc(hidden)]
pub use crate::{
	circumcircle::*,
	triangle_2d::*,
	utilities::*,
	*,
};

#[doc(hidden)]
#[cfg(feature = "2d")]
pub use crate::{
	mosaic_2d::{
		delaunay::*,
		voronoi::*,
		edge_node2d::*,
		triangle_node2d::*,
		*
	},
};

#[doc(hidden)]
#[cfg(feature = "3d_unstable")]
pub use crate::{
	circumsphere::*, edge_3d::*, tetrahedron::*, triangle_3d::*,
	mosaic_3d::{
		delaunay::*,
		voronoi::*,
		edge_node3d::*,
		triangle_node3d::*,
		tetrahedron_node::*,
		*
	},
};
