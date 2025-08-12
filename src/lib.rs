//! This library is for generating Bevy meshes from a series of points in space
//! via Delaunay Triangulation and Voronoi Tessellation
//!

pub mod circumcircle;
#[cfg(feature = "3d_unstable")]
pub mod circumsphere;
#[cfg(feature = "3d_unstable")]
pub mod edge_3d;
pub mod prelude;
#[cfg(feature = "3d_unstable")]
pub mod tetrahedron;
pub mod triangle_2d;
#[cfg(feature = "3d_unstable")]
pub mod triangle_3d;
pub mod utilities;

#[cfg(feature = "2d")]
pub mod mosaic_2d;

#[cfg(feature = "3d_unstable")]
pub mod mosaic_3d;
