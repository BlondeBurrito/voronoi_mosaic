//! Delaunay and Voronoi in 2d space
//!
//!

use bevy::math::Vec2;

pub mod circumcircle;
pub mod delaunay;
pub mod edge_node2d;
pub mod triangle_node2d;
pub mod voronoi;

/// Defines the Delaunay-Voronoi dual
pub struct Mosaic2d {
	/// Generated Delaunay Triangulation
	delaunay: Option<delaunay::Delaunay2d>,
	/// Generated Voronoi Tesselation
	voronoi: Option<voronoi::Voronoi2d>,
}

impl Mosaic2d {
	/// Generate the Delaunay and Voronoi for a series of 2d points
	pub fn new(data_points: &Vec<Vec2>) -> Self {
		if let Some(delaunay) = delaunay::Delaunay2d::compute_triangulation_2d(data_points) {
			if let Some(voronoi) = voronoi::Voronoi2d::from_delaunay_2d(&delaunay) {
				Mosaic2d {
					delaunay: Some(delaunay),
					voronoi: Some(voronoi),
				}
			} else {
				Mosaic2d {
					delaunay: Some(delaunay),
					voronoi: None,
				}
			}
		} else {
			Mosaic2d {
				delaunay: None,
				voronoi: None,
			}
		}
	}
	/// Get the computed Delaunay Triangulation, if it exists
	pub fn get_delaunay(&self) -> Option<&delaunay::Delaunay2d> {
		self.delaunay.as_ref()
	}
	/// Get the computed Voronoi Tesselation, if it exists
	pub fn get_voronoi(&self) -> Option<&voronoi::Voronoi2d> {
		self.voronoi.as_ref()
	}
}
