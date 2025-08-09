//! `use voronoi_mosaic::prelude::*;` to import common structures and methods
//!

#[doc(hidden)]
pub use crate::{
	circumcircle::*,
	circumsphere::*,
	delaunay::*,
	edge_3d::*,
	tetrahedron::*,
	triangle_2d::*,
	triangle_3d::*,
	utilities::*,
	voronoi::{voronoi_2d::*, voronoi_3d::*, *},
	*,
};
