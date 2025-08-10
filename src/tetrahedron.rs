//! Describes a tetrahedron - a shape made of 4 vetices with 4 triangular faces
//! and 6 edges
//!

use bevy::prelude::*;

use crate::{circumsphere::Circumsphere, triangle_3d};

/// Describes a tetrahedron
#[derive(Clone, Debug)]
pub struct Tetrahedron {
	/// A vertex
	vertex_a: Vec3,
	/// B vertex
	vertex_b: Vec3,
	/// C vertex
	vertex_c: Vec3,
	/// D vertex
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
			(self.vertex_a, self.vertex_c),
			(self.vertex_a, self.vertex_d),
			(self.vertex_b, self.vertex_c),
			(self.vertex_c, self.vertex_d),
			(self.vertex_d, self.vertex_b),
		]
	}
	/// Get the vertices in sets of 3 for each face of the tetrahedron
	pub fn get_face_vertices(&self) -> [[Vec3; 3]; 4] {
		[
			[self.vertex_a, self.vertex_b, self.vertex_c],
			[self.vertex_a, self.vertex_c, self.vertex_d],
			[self.vertex_a, self.vertex_d, self.vertex_b],
			[self.vertex_b, self.vertex_c, self.vertex_d],
		]
	}
	/// Get [triangle_3d::Triangle3d] representations of each face of the
	/// tetrahedron
	pub fn get_triangle_3d_faces(&self) -> [triangle_3d::Triangle3d; 4] {
		[
			triangle_3d::Triangle3d::new(self.vertex_a, self.vertex_b, self.vertex_c),
			triangle_3d::Triangle3d::new(self.vertex_a, self.vertex_c, self.vertex_d),
			triangle_3d::Triangle3d::new(self.vertex_a, self.vertex_d, self.vertex_b),
			triangle_3d::Triangle3d::new(self.vertex_b, self.vertex_c, self.vertex_d),
		]
	}
	/// Compute the circumsphere of this tetrehedron
	pub fn compute_circumsphere(&self) -> Option<Circumsphere> {
		Circumsphere::new(self.vertex_a, self.vertex_b, self.vertex_c, self.vertex_d)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn tetra_equality() {
		let a = Vec3::new(0.0, 0.0, 0.0);
		let b = Vec3::new(1.0, 0.0, 0.0);
		let c = Vec3::new(0.0, 0.0, 1.0);
		let d = Vec3::new(0.0, 1.0, 0.0);

		let tet_i = Tetrahedron::new(a, b, c, d);
		let tet_j = Tetrahedron::new(b, c, d, a);
		let tet_k = Tetrahedron::new(c, d, a, b);
		let tet_h = Tetrahedron::new(d, a, b, c);
		assert!(tet_i == tet_j && tet_j == tet_k && tet_k == tet_h && tet_h == tet_i)
	}

	#[test]
	fn sphere_is_some() {
		let tet = Tetrahedron::new(
			Vec3::new(0.0, 1.0, 0.5),
			Vec3::new(0.0, 0.0, 0.0),
			Vec3::new(1.0, 0.0, 1.0),
			Vec3::new(-1.0, 0.0, 1.0),
		);
		assert!(tet.compute_circumsphere().is_some());
	}

	#[test]
	fn sphere_is_none() {
		// coplanar
		let tet = Tetrahedron::new(
			Vec3::new(0.0, 3.0, 0.0),
			Vec3::new(1.0, 0.0, 0.0),
			Vec3::new(-1.0, 0.0, 0.0),
			Vec3::new(0.0, 0.0, 0.0),
		);
		assert!(tet.compute_circumsphere().is_none());
	}
	#[test]
	fn vert_array() {
		let a = Vec3::new(-5.0, 0.0, 3.0);
		let b = Vec3::new(5.0, 0.0, 3.0);
		let c = Vec3::new(5.0, 0.0, 5.0);
		let d = Vec3::new(0.0, 8.0, 1.0);
		let tet = Tetrahedron::new(a, b, c, d);
		assert!([&a, &b, &c, &d] == tet.get_vertices())
	}
	#[test]
	fn edges() {
		let a = Vec3::new(-5.0, 0.0, 3.0);
		let b = Vec3::new(5.0, 0.0, 3.0);
		let c = Vec3::new(5.0, 0.0, 5.0);
		let d = Vec3::new(0.0, 8.0, 1.0);
		let tet = Tetrahedron::new(a, b, c, d);

		let actual = [(a, b), (a, c), (a, d), (b, c), (c, d), (d, b)];
		assert!(actual == tet.get_edges());
	}
	#[test]
	fn face_verts() {
		let a = Vec3::new(-5.0, 0.0, 3.0);
		let b = Vec3::new(5.0, 0.0, 3.0);
		let c = Vec3::new(5.0, 0.0, 5.0);
		let d = Vec3::new(0.0, 8.0, 1.0);
		let tet = Tetrahedron::new(a, b, c, d);

		let actual = [[a, b, c], [a, c, d], [a, d, b], [b, c, d]];
		assert!(actual == tet.get_face_vertices());
	}
	#[test]
	fn triangle_faces() {
		let a = Vec3::new(-5.0, 0.0, 3.0);
		let b = Vec3::new(5.0, 0.0, 3.0);
		let c = Vec3::new(5.0, 0.0, 5.0);
		let d = Vec3::new(0.0, 8.0, 1.0);
		let tet = Tetrahedron::new(a, b, c, d);

		let actual = [
			triangle_3d::Triangle3d::new(a, b, c),
			triangle_3d::Triangle3d::new(a, c, d),
			triangle_3d::Triangle3d::new(a, d, b),
			triangle_3d::Triangle3d::new(b, c, d),
		];
		assert!(actual == tet.get_triangle_3d_faces());
	}
}
