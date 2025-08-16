//! Defines an ID based triangle
//!
//!

use bevy::prelude::*;
use std::{cmp::Ordering, collections::BTreeMap};

use crate::{mosaic_2d::edge_node2d::EdgeNode2d, prelude::Circumcircle};

/// Describes a triangle where the vertices are represented by vertex IDs
#[derive(Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct TriangleNode2d([usize; 3]);

impl PartialEq for TriangleNode2d {
	fn eq(&self, other: &Self) -> bool {
		let (self_a, self_b, self_c) = (self.0[0], self.0[1], self.0[2]);

		other.0.contains(&self_a) && other.0.contains(&self_b) && other.0.contains(&self_c)
	}
}

impl TriangleNode2d {
	/// Create a new [TriangleNode2d] from a series of vertex IDs
	pub fn new(a: usize, b: usize, c: usize) -> Self {
		TriangleNode2d([a, b, c])
	}
	/// Get the vertex IDs
	pub fn get_vertex_ids(&self) -> &[usize; 3] {
		&self.0
	}
	/// Get a mutable refernce to the vertex IDs
	pub fn get_vertex_ids_mut(&mut self) -> &mut [usize; 3] {
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
	/// If possible compute the circumcircle of this triangle
	pub fn compute_circumcircle(
		&self,
		vertex_lookup: &BTreeMap<usize, Vec2>,
	) -> Option<Circumcircle> {
		let vertex_a = vertex_lookup.get(&self.0[0])?;
		let vertex_b = vertex_lookup.get(&self.0[1])?;
		let vertex_c = vertex_lookup.get(&self.0[2])?;
		Circumcircle::new(*vertex_a, *vertex_b, *vertex_c)
	}
	/// Get the edges of the triangle in ID form of [EdgeNode2d]
	pub fn get_edges(&self) -> [EdgeNode2d; 3] {
		[
			EdgeNode2d::new(self.0[0], self.0[1]),
			EdgeNode2d::new(self.0[1], self.0[2]),
			EdgeNode2d::new(self.0[2], self.0[0]),
		]
	}
	/// Reorder the vertex IDs so they are in anti-clockwise order, angle around their midpoint running negative to positive
	pub fn sort_vertices_anti_clockwise(&mut self, vertex_lookup: &BTreeMap<usize, Vec2>) {
		let ids = self.get_vertex_ids_mut();
		let midpoint = {
			let pos_a = vertex_lookup.get(&ids[0]).unwrap();
			let pos_b = vertex_lookup.get(&ids[1]).unwrap();
			let pos_c = vertex_lookup.get(&ids[2]).unwrap();
			(pos_a + pos_b + pos_c) / 3.0
		};
		ids.sort_by(|a, b| {
			let a_pos = vertex_lookup.get(a).unwrap();
			let b_pos = vertex_lookup.get(b).unwrap();
			if let Some(ordering) = Vec2::Y
				.angle_to(*a_pos - midpoint)
				.partial_cmp(&Vec2::Y.angle_to(*b_pos - midpoint))
			{
				ordering
			} else {
				warn!("Unable to find Ordering between {} and {}", a, b);
				Ordering::Less
			}
		});
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

		let tri_i = TriangleNode2d::new(a, b, c);
		let tri_j = TriangleNode2d::new(b, c, a);
		let tri_k = TriangleNode2d::new(c, a, b);
		let tri_h = TriangleNode2d::new(a, c, b);
		let tri_l = TriangleNode2d::new(b, a, c);

		assert!(
			tri_i == tri_j && tri_j == tri_k && tri_k == tri_h && tri_h == tri_l && tri_l == tri_i
		)
	}

	#[test]
	fn sorting_vertices() {
		let a = 0;
		let b = 1;
		let c = 2;
		let vertex_lookup = BTreeMap::from([
			(a, Vec2::new(-5.0, 0.0)),
			(b, Vec2::new(0.0, 10.0)),
			(c, Vec2::new(5.0, 0.0)),
		]);
		let mut triangle = TriangleNode2d::new(a, b, c);
		triangle.sort_vertices_anti_clockwise(&vertex_lookup);
		assert_eq!([c, b, a], *triangle.get_vertex_ids());
	}
}
