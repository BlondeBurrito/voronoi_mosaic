//! This library is for generating Bevy meshes from a series of points in space
//! via Delaunay Triangulation and Voronoi Tessellation
//!

pub mod prelude;
pub mod utilities;

#[cfg(feature = "2d")]
pub mod mosaic_2d;

#[cfg(feature = "3d")]
pub mod mosaic_3d;
