//! `use voronoi_mosaic::prelude::*;` to import common structures and methods
//!

#[doc(hidden)]
pub use crate::{
	circumcircle::*,
	delaunay::*,
	triangle_2d::*,
	utilities::*,
	voronoi::{voronoi_2d::*, *},
	*,
};

#[cfg(feature = "3d_unstable")]
pub use crate::{
	circumsphere::*,
	triangle_3d::*,
	edge_3d::*,
	voronoi::voronoi_3d::*,
	delaunay::delaunay_3d::*,
	tetrahedron::*,
};