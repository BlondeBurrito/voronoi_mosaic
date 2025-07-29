//! `use voronoi_mosaic::prelude::*;` to import common structures and methods
//!

#[doc(hidden)]
pub use crate::{
	circumcircle::*,
	circumsphere::*,
	delaunay::*,
	tetrahedron::*,
	triangle_2d::*,
	utilities::*,
	voronoi::{voronoi_2d::*, voronoi_3d::*, *},
};
