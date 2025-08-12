//! `use voronoi_mosaic::prelude::*;` to import common structures and methods
//!

#[doc(hidden)]
pub use crate::{circumcircle::*, triangle_2d::*, utilities::*, *};

#[doc(hidden)]
#[cfg(feature = "2d")]
pub use crate::mosaic_2d::{Mosaic2d, delaunay::*, edge_node2d::*, triangle_node2d::*, voronoi::*};

#[doc(hidden)]
#[cfg(feature = "3d_unstable")]
pub use crate::{
	circumsphere::*,
	edge_3d::*,
	mosaic_3d::{
		Mosaic3d, delaunay::*, edge_node3d::*, tetrahedron_node::*, triangle_node3d::*, voronoi::*,
	},
	tetrahedron::*,
	triangle_3d::*,
};
