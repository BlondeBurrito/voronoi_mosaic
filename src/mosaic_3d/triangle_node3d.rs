//! Defines an ID based triangle
//!
//!

use std::collections::BTreeMap;

use bevy::math::Vec3;

use crate::mosaic_3d::edge_node3d::EdgeNode3d;

/// Describes a triangle where the vertices are represented by vertex IDs
#[derive(Eq, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct TriangleNode3d([usize; 3]);

impl PartialEq for TriangleNode3d {
	fn eq(&self, other: &Self) -> bool {
		let (self_a, self_b, self_c) = (self.0[0], self.0[1], self.0[2]);

		other.0.contains(&self_a) && other.0.contains(&self_b) && other.0.contains(&self_c)
	}
}

impl TriangleNode3d {
	/// Create a new [TriangleNode2d] from a series of vertex IDs
	pub fn new(a: usize, b: usize, c: usize) -> Self {
		TriangleNode3d([a, b, c])
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
	// /// If possible compute the circumcircle of this triangle
	// pub fn compute_circumcircle(&self, vertex_lookup: &BTreeMap<usize, Vec3>) -> Option<Circumcircle> {
	// 	let Some(vertex_a) = vertex_lookup.get(&self.0[0]) else {
	// 		return None
	// 	};
	// 	let Some(vertex_b) = vertex_lookup.get(&self.0[1]) else {
	// 		return None
	// 	};
	// 	let Some(vertex_c) = vertex_lookup.get(&self.0[2]) else {
	// 		return None
	// 	};
	// 	Circumcircle::new(*vertex_a, *vertex_b, *vertex_c)
	// }
	/// Get the edges of the triangle in ID form of [EdgeNode3d]
	pub fn get_edges(&self) -> [EdgeNode3d; 3] {
		[
			EdgeNode3d::new(self.0[0], self.0[1]),
			EdgeNode3d::new(self.0[1], self.0[2]),
			EdgeNode3d::new(self.0[2], self.0[0]),
		]
	}
	// /// Reorder the vertex IDs so they are in anti-clockwise order, angle around their midpoint running negative to positive
	// pub fn sort_vertices_anti_clockwise(&mut self, vertex_lookup: &BTreeMap<usize, Vec2>) {
	// 	let ids = self.get_vertex_ids_mut();
	// 	let midpoint = {
	// 		let pos_a = vertex_lookup.get(&ids[0]).unwrap();
	// 		let pos_b = vertex_lookup.get(&ids[1]).unwrap();
	// 		let pos_c = vertex_lookup.get(&ids[2]).unwrap();
	// 		(pos_a + pos_b + pos_c) / 3.0
	// 	};
	// 	ids.sort_by(|a, b| {
	// 		let a_pos = vertex_lookup.get(a).unwrap();
	// 		let b_pos = vertex_lookup.get(b).unwrap();
	// 	if let Some(ordering) = Vec2::Y
	// 		.angle_to(*a_pos - midpoint)
	// 		.partial_cmp(&Vec2::Y.angle_to(*b_pos - midpoint))
	// 	{
	// 		ordering
	// 	} else {
	// 		warn!("Unable to find Ordering between {} and {}", a, b);
	// 		Ordering::Less
	// 	}
	// });
	// }

	/// Check if an edge/segment intersects the triangle - effectively testing
	/// to see if a ray intersects a triangular face
	pub fn does_edge_intersect_id(
		&self,
		edge: &EdgeNode3d,
		vertex_lookup: &BTreeMap<usize, Vec3>,
	) -> bool {
		let tri_vertex_a = vertex_lookup.get(&self.get_vertex_a_id()).unwrap();
		let tri_vertex_b = vertex_lookup.get(&self.get_vertex_b_id()).unwrap();
		let tri_vertex_c = vertex_lookup.get(&self.get_vertex_c_id()).unwrap();

		let edge_vertex_a = vertex_lookup.get(&edge.get_vertex_a_id()).unwrap();
		let edge_vertex_b = vertex_lookup.get(&edge.get_vertex_b_id()).unwrap();

		self.does_edge_intersect(
			tri_vertex_a,
			tri_vertex_b,
			tri_vertex_c,
			edge_vertex_a,
			edge_vertex_b,
		)
	}

	/// Check if an edge/segment intersects the triangle - effectively testing
	/// to see if a ray intersects a triangular face
	pub fn does_edge_intersect(
		&self,
		tri_vertex_a: &Vec3,
		tri_vertex_b: &Vec3,
		tri_vertex_c: &Vec3,
		edge_vertex_a: &Vec3,
		edge_vertex_b: &Vec3,
	) -> bool {
		// equation of a plane: ax + by + cz = d, `a, b, c` come from the normal of the plane

		// Parametric solution: https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection

		let tri_a_to_b = tri_vertex_b - tri_vertex_a;
		let tri_a_to_c = tri_vertex_c - tri_vertex_a;
		let edge_a_to_b = edge_vertex_b - edge_vertex_a;

		let normal = tri_a_to_b.cross(tri_a_to_c);
		let denom = (-edge_a_to_b).dot(normal);
		if denom != 0.0 {
			let t = ((tri_a_to_b.cross(tri_a_to_c)).dot(edge_vertex_a - tri_vertex_a)) / denom;
			let u = ((tri_a_to_c.cross(-edge_a_to_b)).dot(edge_vertex_a - tri_vertex_a)) / denom;
			let v = (((-edge_a_to_b).cross(tri_a_to_b)).dot(edge_vertex_a - tri_vertex_a)) / denom;

			// if `t` is [0, 1] then intersection is on line
			// however it IS ok for an edge to touch a face, so
			// check `t` is more than 0 but less than 1 instead
			if t > 0.0 && t < 1.0 {
				// if u,v [0, 1] then intersecction is on parallelogram
				if (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v) {
					// if sum of `u`, `v` is <= 1 then intersection is within points of triangle
					return u + v <= 1.0;
				}
			}
		}
		false
	}
	//TODO add degenerate bool check like 2d version?
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn equality() {
		let a = 1;
		let b = 2;
		let c = 3;

		let tri_i = TriangleNode3d::new(a, b, c);
		let tri_j = TriangleNode3d::new(b, c, a);
		let tri_k = TriangleNode3d::new(c, a, b);
		let tri_h = TriangleNode3d::new(a, c, b);
		let tri_l = TriangleNode3d::new(b, a, c);

		assert!(
			tri_i == tri_j && tri_j == tri_k && tri_k == tri_h && tri_h == tri_l && tri_l == tri_i
		)
	}

	#[test]
	fn does_intersect_face() {
		let vertex_lookup = BTreeMap::from([
			(0, Vec3::new(0.0, 0.0, 0.0)),
			(1, Vec3::new(-5.0, 12.0, 0.0)),
			(2, Vec3::new(5.0, 12.0, 0.0)),
			// edge
			(3, Vec3::new(0.0, 6.0, -5.0)),
			(4, Vec3::new(0.0, 6.0, 5.0)),
		]);
		let triangle = TriangleNode3d::new(0, 1, 2);
		let edge = EdgeNode3d::new(3, 4);

		assert!(triangle.does_edge_intersect_id(&edge, &vertex_lookup));
	}
	#[test]
	fn does_not_intersect_face() {
		let vertex_lookup = BTreeMap::from([
			(0, Vec3::new(0.0, 0.0, 0.0)),
			(1, Vec3::new(-5.0, 12.0, 0.0)),
			(2, Vec3::new(5.0, 12.0, 0.0)),
			// edge
			(3, Vec3::new(0.0, 6.0, -5.0)),
			(4, Vec3::new(0.0, 6.0, -3.0)),
		]);
		let triangle = TriangleNode3d::new(0, 1, 2);
		let edge = EdgeNode3d::new(3, 4);

		assert!(!triangle.does_edge_intersect_id(&edge, &vertex_lookup));
	}
	#[test]
	fn touches_face_but_not_intercept() {
		let vertex_lookup = BTreeMap::from([
			(0, Vec3::new(0.0, 0.0, 0.0)),
			(1, Vec3::new(-5.0, 12.0, 0.0)),
			(2, Vec3::new(5.0, 12.0, 0.0)),
			// edge
			(3, Vec3::new(0.0, 6.0, -5.0)),
			(4, Vec3::new(0.0, 6.0, 0.0)),
		]);
		let triangle = TriangleNode3d::new(0, 1, 2);
		let edge = EdgeNode3d::new(3, 4);

		assert!(!triangle.does_edge_intersect_id(&edge, &vertex_lookup));
	}
}
