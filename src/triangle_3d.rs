//! Describes a triangle shape in 3d space
//!

use bevy::prelude::*;


/// Describes the vertices and edges of a triangle
#[derive(Clone, Debug)]
pub struct Triangle3d {
	/// A vertex
	vertex_a: Vec3,
	/// B vertex
	vertex_b: Vec3,
	/// C vertex
	vertex_c: Vec3,
}
impl PartialEq for Triangle3d {
	fn eq(&self, other: &Self) -> bool {
		(self.vertex_a == other.vertex_a
			&& self.vertex_b == other.vertex_b
			&& self.vertex_c == other.vertex_c)
			|| (self.vertex_a == other.vertex_b
				&& self.vertex_b == other.vertex_c
				&& self.vertex_c == other.vertex_a)
			|| (self.vertex_a == other.vertex_c
				&& self.vertex_b == other.vertex_a
				&& self.vertex_c == other.vertex_b)
	}
}
impl Triangle3d {
	/// Init a triangle from vertices
	pub fn new(vertex_a: Vec3, vertex_b: Vec3, vertex_c: Vec3) -> Self {
		Triangle3d {
			vertex_a,
			vertex_b,
			vertex_c,
		}
	}
	/// Get vertex a
	pub fn get_vertex_a(&self) -> &Vec3 {
		&self.vertex_a
	}
	/// Get vertex b
	pub fn get_vertex_b(&self) -> &Vec3 {
		&self.vertex_b
	}
	/// Get vertex c
	pub fn get_vertex_c(&self) -> &Vec3 {
		&self.vertex_c
	}
	/// Get all vertices as an array
	pub fn get_vertices(&self) -> [&Vec3; 3] {
		[
			self.get_vertex_a(),
			self.get_vertex_b(),
			self.get_vertex_c(),
		]
	}
	/// Get the edges of the triangle
	pub fn get_edges(&self) -> [(Vec3, Vec3); 3] {
		[
			(self.vertex_a, self.vertex_b),
			(self.vertex_b, self.vertex_c),
			(self.vertex_c, self.vertex_a),
		]
	}
}
