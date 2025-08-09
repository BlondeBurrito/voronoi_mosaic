//! Describes a triangle shape in 3d space
//!

use bevy::prelude::*;

use crate::prelude::Edge3d;


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
	/// Check if an edge/segment intersects the triangle - effectively testing
	/// to see if a ray intersects a triangular face
	pub fn does_edge_intersect(&self, edge: &Edge3d) -> bool {
		// // calcualte surface normal of the triangle
		// let tri_normal = ((self.vertex_b - self.vertex_a).cross(self.vertex_c-self.vertex_a)).normalize();
		// // apply dot product to between normal and edge, describes
		// // orthogonality
		// let line = edge.get_vertex_b() - edge.get_vertex_a();
		// // get the distance from each end of the edge to vertex a of the triangle
		// let edge_start_to_a = edge.get_vertex_a() - self.get_vertex_a();
		// let edge_end_to_a = edge.get_vertex_b()- self.vertex_a;
		// // find the distance to the plane of the face
		// let edge_start_dist_plane = edge_start_to_a.dot(tri_normal);
		// let edge_end_dist_plane = edge_end_to_a.dot(tri_normal);

		// // if the multiplicative distance is positive then the edge doesn not pass through the plane defined by the face
		// //TODO need to handle -0.0?
		// if edge_start_dist_plane * edge_end_dist_plane >= 0.0 {
		// 	return false;
		// }
		// // if distances are the same then edge is parallel to plane
		// //TODO surely this never fires
		// if edge_start_dist_plane == edge_end_dist_plane {
		// 	return false;
		// }

		// equation of a plane: ax + by + cz = d, `a, b, c` come from the normal of the plane

		// Parametric solution: https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
		let tri_a_to_b = self.vertex_b - self.vertex_a;
		let tri_a_to_c = self.vertex_c - self.vertex_a;
		let edge_a_to_b = edge.get_vertex_b() - edge.get_vertex_a();

		let normal = tri_a_to_b.cross(tri_a_to_c);
		let denom = (-edge_a_to_b).dot(normal);
		if denom != 0.0 {
			let t = ((tri_a_to_b.cross(tri_a_to_c)).dot(*edge.get_vertex_a() - self.vertex_a)) / denom;
			let u = ((tri_a_to_c.cross(-edge_a_to_b)).dot(*edge.get_vertex_a() - self.vertex_a)) /denom;
			let v = (((-edge_a_to_b).cross(tri_a_to_b)).dot(*edge.get_vertex_a() - self.vertex_a)) / denom;

			// if `t` is [0, 1] then intersection is on line
			// however it IS ok for an edge to touch a face, so
			// check `t` is more than 0 but less than 1 instead
			if t > 0.0 && t < 1.0 {
				// if u,v [0, 1] then intersecction is on parallelogram
				if (u >= 0.0 && u <= 1.0) && (v >= 0.0 && v <= 1.0) {
					// if sum of `u`, `v` is <= 1 then intersection is within points of triangle
				if u + v <= 1.0 {
					true
				} else {
					false
				}
				} else {
					false
				}
			} else {
				false
			}
		} else {
			false
		}

		// // find point where edge intersects plane
		// let intersection = 
	}
}

#[cfg(test)]
mod tests {
	use crate::{edge_3d, triangle_3d};

use super::*;

	#[test]
	fn does_intersect_face() {
		let vertex_a = Vec3::new(0.0, 0.0, 0.0);
		let vertex_b = Vec3::new(-5.0, 12.0, 0.0);
		let vertex_c = Vec3::new(5.0, 12.0, 0.0);
		let triangle = triangle_3d::Triangle3d::new(vertex_a, vertex_b, vertex_c);
		let vertex_a = Vec3::new(0.0, 6.0, -5.0);
		let vertex_b = Vec3::new(0.0, 6.0, 5.0);
		let edge = edge_3d::Edge3d::new(vertex_a, vertex_b);

		assert!(triangle.does_edge_intersect(&edge));
	}
	#[test]
	fn does_not_intersect_face() {
		let vertex_a = Vec3::new(0.0, 0.0, 0.0);
		let vertex_b = Vec3::new(-5.0, 12.0, 0.0);
		let vertex_c = Vec3::new(5.0, 12.0, 0.0);
		let triangle = triangle_3d::Triangle3d::new(vertex_a, vertex_b, vertex_c);
		let vertex_a = Vec3::new(0.0, 6.0, -5.0);
		let vertex_b = Vec3::new(0.0, 6.0, -3.0);
		let edge = edge_3d::Edge3d::new(vertex_a, vertex_b);

		assert!(!triangle.does_edge_intersect(&edge));
	}
	#[test]
	fn touches_face_but_not_intercept() {
		let vertex_a = Vec3::new(0.0, 0.0, 0.0);
		let vertex_b = Vec3::new(-5.0, 12.0, 0.0);
		let vertex_c = Vec3::new(5.0, 12.0, 0.0);
		let triangle = triangle_3d::Triangle3d::new(vertex_a, vertex_b, vertex_c);
		let vertex_a = Vec3::new(0.0, 6.0, -5.0);
		let vertex_b = Vec3::new(0.0, 6.0, 0.0);
		let edge = edge_3d::Edge3d::new(vertex_a, vertex_b);

		assert!(!triangle.does_edge_intersect(&edge));
	}
}