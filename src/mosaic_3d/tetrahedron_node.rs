//! Defines an ID based tetrahedron
//!

use std::collections::BTreeMap;

use bevy::math::Vec3;

use crate::{
	mosaic_3d::{edge_node3d::EdgeNode3d, triangle_node3d::TriangleNode3d},
	prelude::Circumsphere,
};

/// Describes a tetrahedron where the vertices are represented by vertex IDs
#[derive(Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct TetrahedronNode([usize; 4]);

impl PartialEq for TetrahedronNode {
	fn eq(&self, other: &Self) -> bool {
		let (self_a, self_b, self_c, self_d) = (self.0[0], self.0[1], self.0[2], self.0[3]);
		let (other_a, other_b, other_c, other_d) = (other.0[0], other.0[1], other.0[2], other.0[3]);

		(self_a == other_a && self_b == other_b && self_c == other_c && self_d == other_d)
			|| (self_a == other_b && self_b == other_c && self_c == other_d && self_d == other_a)
			|| (self_a == other_c && self_b == other_d && self_c == other_a && self_d == other_b)
			|| (self_a == other_d && self_b == other_a && self_c == other_b && self_d == other_c)
	}
}

impl TetrahedronNode {
	/// Create a new [TetrahedronNode] from a series of vertex IDs
	pub fn new(a: usize, b: usize, c: usize, d: usize) -> Self {
		TetrahedronNode([a, b, c, d])
	}
	/// Get the vertex IDs
	pub fn get_vertex_ids(&self) -> &[usize; 4] {
		&self.0
	}
	/// Get a mutable refernce to the vertex IDs
	pub fn get_vertex_ids_mut(&mut self) -> &mut [usize; 4] {
		&mut self.0
	}
	/// Get the ID of vertex a
	pub fn get_vertex_a_id(&self) -> usize {
		self.0[0]
	}
	/// Get the ID of vertex b
	pub fn get_vertex_b_id(&self) -> usize {
		self.0[1]
	}
	/// Get the ID of vertex c
	pub fn get_vertex_c_id(&self) -> usize {
		self.0[2]
	}
	/// Get the ID of vertex d
	pub fn get_vertex_d_id(&self) -> usize {
		self.0[3]
	}
	/// If possible compute the circumsphere of this tetrahedron
	pub fn compute_circumsphere(
		&self,
		vertex_lookup: &BTreeMap<usize, Vec3>,
	) -> Option<Circumsphere> {
		let Some(vertex_a) = vertex_lookup.get(&self.0[0]) else {
			return None;
		};
		let Some(vertex_b) = vertex_lookup.get(&self.0[1]) else {
			return None;
		};
		let Some(vertex_c) = vertex_lookup.get(&self.0[2]) else {
			return None;
		};
		let Some(vertex_d) = vertex_lookup.get(&self.0[3]) else {
			return None;
		};
		Circumsphere::new(*vertex_a, *vertex_b, *vertex_c, *vertex_d)
	}
	/// Get the edges of the tetrahedron in ID form of [EdgeNode3d]
	pub fn get_edges(&self) -> [EdgeNode3d; 6] {
		[
			EdgeNode3d::new(self.0[0], self.0[1]),
			EdgeNode3d::new(self.0[0], self.0[2]),
			EdgeNode3d::new(self.0[0], self.0[3]),
			EdgeNode3d::new(self.0[1], self.0[2]),
			EdgeNode3d::new(self.0[2], self.0[3]),
			EdgeNode3d::new(self.0[3], self.0[1]),
		]
	}
	/// Get [TriangleNode3d] representations of each face of the
	/// tetrahedron
	pub fn get_triangle_node_3d_faces(&self) -> [TriangleNode3d; 4] {
		[
			TriangleNode3d::new(
				self.get_vertex_a_id(),
				self.get_vertex_b_id(),
				self.get_vertex_c_id(),
			),
			TriangleNode3d::new(
				self.get_vertex_a_id(),
				self.get_vertex_c_id(),
				self.get_vertex_d_id(),
			),
			TriangleNode3d::new(
				self.get_vertex_a_id(),
				self.get_vertex_d_id(),
				self.get_vertex_b_id(),
			),
			TriangleNode3d::new(
				self.get_vertex_b_id(),
				self.get_vertex_c_id(),
				self.get_vertex_d_id(),
			),
		]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn equality() {
		let a = 1;
		let b = 2;
		let c = 3;
		let d = 4;

		let tet_i = TetrahedronNode::new(a, b, c, d);
		let tet_j = TetrahedronNode::new(b, c, d, a);
		let tet_k = TetrahedronNode::new(c, d, a, b);
		let tet_h = TetrahedronNode::new(d, a, b, c);

		assert!(tet_i == tet_j && tet_j == tet_k && tet_k == tet_h && tet_h == tet_i)
	}
}