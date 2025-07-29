//! Describes a tetrahedron - a shape made of 4 vetices with 4 triangular faces
//! and 6 edges
//!

use bevy::prelude::*;

use crate::circumsphere::Circumsphere;

/// Describes a tetrahedron
#[derive(Clone)]
pub struct Tetrahedron {
	vertex_a: Vec3,
	vertex_b: Vec3,
	vertex_c: Vec3,
	vertex_d: Vec3,
}

impl PartialEq for Tetrahedron {
	fn eq(&self, other: &Self) -> bool {
		(self.vertex_a == other.vertex_a
			&& self.vertex_b == other.vertex_b
			&& self.vertex_c == other.vertex_c
			&& self.vertex_d == other.vertex_d)
			|| (self.vertex_a == other.vertex_b
				&& self.vertex_b == other.vertex_c
				&& self.vertex_c == other.vertex_d
				&& self.vertex_d == other.vertex_a)
			|| (self.vertex_a == other.vertex_c
				&& self.vertex_b == other.vertex_d
				&& self.vertex_c == other.vertex_a
				&& self.vertex_d == other.vertex_b)
			|| (self.vertex_a == other.vertex_d
				&& self.vertex_b == other.vertex_a
				&& self.vertex_c == other.vertex_b
				&& self.vertex_d == other.vertex_c)
	}
}
impl Tetrahedron {
	/// Init a triangle from vertices
	pub fn new(vertex_a: Vec3, vertex_b: Vec3, vertex_c: Vec3, vertex_d: Vec3) -> Self {
		Tetrahedron {
			vertex_a,
			vertex_b,
			vertex_c,
			vertex_d,
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
	/// Get vertex d
	pub fn get_vertex_d(&self) -> &Vec3 {
		&self.vertex_d
	}
	/// Get all vertices of the [Tetrahedron] as an array
	pub fn get_vertices(&self) -> [&Vec3; 4] {
		[
			self.get_vertex_a(),
			self.get_vertex_b(),
			self.get_vertex_c(),
			self.get_vertex_d(),
		]
	}
	/// Get the edges along each face
	pub fn get_edges(&self) -> [(Vec3, Vec3); 6] {
		[
			(self.vertex_a, self.vertex_b),
			(self.vertex_b, self.vertex_c),
			(self.vertex_c, self.vertex_a),
			(self.vertex_a, self.vertex_d),
			(self.vertex_d, self.vertex_b),
			(self.vertex_d, self.vertex_c),
		]
	}
	/// Compute the circumsphere of this tetrehedron
	pub fn compute_circumsphere(&self) -> Option<Circumsphere> {
		Circumsphere::new(self.vertex_a, self.vertex_b, self.vertex_c, self.vertex_d)
	}
}
