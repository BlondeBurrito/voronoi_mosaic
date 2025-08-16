//! Delaunay and Voronoi in 3d
//!
//!

use bevy::math::Vec3;

pub mod circumsphere;
pub mod delaunay;
pub mod edge_node3d;
pub mod tetrahedron_node;
pub mod triangle_node3d;
pub mod voronoi;

/// Defines the Delaunay-Voronoi dual
pub struct Mosaic3d {
	/// The generated Delaunay Tetrahedralization
	delaunay: Option<delaunay::Delaunay3d>,
	/// The generated Voronoi Tessellation
	voronoi: Option<voronoi::Voronoi3d>,
}

impl Mosaic3d {
	/// Generate the Delaunay and Voronoi for a series of 3d points
	pub fn new(data_points: &[Vec3]) -> Self {
		if let Some(delaunay) = delaunay::Delaunay3d::compute_triangulation_3d(data_points) {
			if let Some(voronoi) = voronoi::Voronoi3d::from_delaunay_3d(&delaunay) {
				Mosaic3d {
					delaunay: Some(delaunay),
					voronoi: Some(voronoi),
				}
			} else {
				Mosaic3d {
					delaunay: Some(delaunay),
					voronoi: None,
				}
			}
		} else {
			Mosaic3d {
				delaunay: None,
				voronoi: None,
			}
		}
	}
	/// Get the computed Delaunay Tetrahedralization, if it exists
	pub fn get_delaunay(&self) -> Option<&delaunay::Delaunay3d> {
		self.delaunay.as_ref()
	}
	/// Get the computed Voronoi Tesselation, if it exists
	pub fn get_voronoi(&self) -> Option<&voronoi::Voronoi3d> {
		self.voronoi.as_ref()
	}
}
