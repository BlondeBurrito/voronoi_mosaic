//! Describes a triangle shape in 2d
//!

use bevy::prelude::*;

use crate::circumcircle::Circumcircle;

/// Describes the vertices and edges of a triangle
#[derive(Clone, Debug)]
pub struct Triangle2d {
	/// A vertex
	vertex_a: Vec2,
	/// B vertex
	vertex_b: Vec2,
	/// C vertex
	vertex_c: Vec2,
}
impl PartialEq for Triangle2d {
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
impl Triangle2d {
	/// Init a triangle from vertices
	pub fn new(vertex_a: Vec2, vertex_b: Vec2, vertex_c: Vec2) -> Self {
		Triangle2d {
			vertex_a,
			vertex_b,
			vertex_c,
		}
	}
	/// Get vertex a
	pub fn get_vertex_a(&self) -> &Vec2 {
		&self.vertex_a
	}
	/// Get vertex b
	pub fn get_vertex_b(&self) -> &Vec2 {
		&self.vertex_b
	}
	/// Get vertex c
	pub fn get_vertex_c(&self) -> &Vec2 {
		&self.vertex_c
	}
	/// Get all vertices as an array
	pub fn get_vertices(&self) -> [&Vec2; 3] {
		[
			self.get_vertex_a(),
			self.get_vertex_b(),
			self.get_vertex_c(),
		]
	}
	/// Get the edges of the triangle
	pub fn get_edges(&self) -> [(Vec2, Vec2); 3] {
		[
			(self.vertex_a, self.vertex_b),
			(self.vertex_b, self.vertex_c),
			(self.vertex_c, self.vertex_a),
		]
	}
	/// Compute the circumcircle of this triangle
	pub fn compute_circumcircle(&self) -> Option<Circumcircle> {
		Circumcircle::new(self.vertex_a, self.vertex_b, self.vertex_c)
	}
}
