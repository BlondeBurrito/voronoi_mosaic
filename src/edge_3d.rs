//! Defines an edge/segment in 3d space
//!

use bevy::prelude::*;

/// A segemnt in space
#[derive(Clone, Debug)]
pub struct Edge3d(Vec3, Vec3);

impl PartialEq for Edge3d {
	fn eq(&self, other: &Self) -> bool {
		(self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
	}
}

impl Edge3d {
	/// Create a new [Edge3d] from two points/vertices
	pub fn new(vertex_a: Vec3, vertex_b: Vec3) -> Self {
		Edge3d(vertex_a, vertex_b)
	}
	/// Get vertex a
	pub fn get_vertex_a(&self) -> &Vec3 {
		&self.0
	}
	/// Get vertex b
	pub fn get_vertex_b(&self) -> &Vec3 {
		&self.1
	}
	/// Get the edge vertices as an array
	pub fn get_vertices(&self) -> [&Vec3; 2] {
		[&self.0, &self.1]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn equality() {
		let a = Vec3::new(0.0, 0.0, 0.0);
		let b = Vec3::new(1.0, 0.0, 0.0);

		let edge_i = Edge3d::new(a, b);
		let edge_j = Edge3d::new(b, a);
		assert!(edge_i == edge_j)
	}
	#[test]
	fn vert_array() {
		let a = Vec3::new(0.0, 0.0, 0.0);
		let b = Vec3::new(1.0, 0.0, 0.0);
		let edge = Edge3d::new(a, b);
		assert!([&a, &b] == edge.get_vertices())
	}
}
