//! Use Tetrahedralization to find tetrahedrons that comply to Delaunay in 3d
//!
//! Generla process is to find circumspheres of tetrahedrons and ensure that
//! the sphere contains no data points
//!


use crate::{prelude::{DelaunayData, Edge3d}, tetrahedron};
use bevy::prelude::*;

impl DelaunayData<tetrahedron::Tetrahedron> {
	/// From a series of 3d points in space calculate the Delaunay Tetrahedralization
	pub fn compute_triangulation_3d(points: &Vec<Vec3>) -> Option<Self> {
		if points.len() < 4 {
			error!(
				"Minimum of 4 points required for tetrahedralization, supplied {} points",
				points.len()
			);
			return None;
		}
		//TODO ensure no duplicates in points
		// idenitfy spacial boundaries
		let (minimum_world_dimensions, maximum_world_dimensions) = compute_dimension_bounds(points);

		// compute the positions of a super tetrahedron that encompasses all points in space
		let super_tetra =
			compute_super_tetrahedra(&minimum_world_dimensions, &maximum_world_dimensions);
		// store tetrahedrons starting with the super one
		let mut tetrahedrons = vec![];
		for s_tetra in super_tetra.iter() {
			tetrahedrons.push(tetrahedron::Tetrahedron::new(s_tetra[0], s_tetra[1], s_tetra[2], s_tetra[3]));
		}
		// // record any points that after re-tetrahedralization fail to produce
		// // Delaunay tetrahedra, these are discarded
		// let mut problematic_points: Vec<Vec3> = vec![];
		// add each point at a time to the triangulation
		for point in points.iter() {
			// record tetraheda that don't qualify as Delaunay
			let mut bad_tetras = vec![];
			// check if the point lies within the circumsphere of the tetrahedron
			for tetra in tetrahedrons.iter() {
				if let Some(circumsphere) = tetra.compute_circumsphere() {
					if circumsphere.is_point_within_sphere(point) {
						// if a point is within then it is not a delaunay triangle,
						// record this tetra for removal
						bad_tetras.push(tetra.clone());
					}
				}
			}
			// remove any bad tetrahedrons from the list
			if !bad_tetras.is_empty() {
				tetrahedrons.retain(|t| !bad_tetras.contains(t));
				// removing bad tetras creates a hole around the point, we want
				// to build new tetrahedrons with the point to
				// progress the tetrahedralization

				// store each triangle face
				let mut face_triangles = vec![];
				// store each duplicate triangle face
				let mut duplicate_face_triangles = vec![];
				for bad_tetra in bad_tetras.iter() {
					let faces = bad_tetra.get_triangle_3d_faces();
					for face in faces.iter() {
						if !face_triangles.contains(face) {
							face_triangles.push(face.clone());
						} else {
							duplicate_face_triangles.push(face.clone());
						}
					}
				}
				// remove duplicate faces as that face crosses the polyhedral hole
				face_triangles.retain(|f| !duplicate_face_triangles.contains(&f));
				// construct new tetrahedrons from the point and each triangle face
				let mut new_tetras = vec![];
				for tri in face_triangles {
					new_tetras.push(tetrahedron::Tetrahedron::new(
							*point,
							*tri.get_vertex_a(),
							*tri.get_vertex_b(),
							*tri.get_vertex_c(),
						));
				}
				while let Some(n_tet) = new_tetras.pop() {
					// only store a new tetra if it is Delaunay - test to
					// ensure it doesn't intersect with any existing tetras
					let mut is_valid = true;

					for pairs in n_tet.get_edges() {
						let edge = Edge3d::new(pairs.0, pairs.1);

						for tetra in tetrahedrons.iter() {
							for face in tetra.get_triangle_3d_faces() {
								if face.does_edge_intersect(&edge) {
									is_valid = false;
								}
							}
						}
					}
					// as edges are allowed to touch faces/vertices we
					// perform an additional check to verify that a face
					// doesn't slice into another face - use a bisecting
					// line down the middle of each face of the
					//  proposed new tetra and check to intersection
					for face in n_tet.get_triangle_3d_faces().iter() {
						let bisecting_edge = Edge3d::new(*face.get_vertex_a(), (*face.get_vertex_b() + *face.get_vertex_c()) / 2.0);
						for tetra in tetrahedrons.iter() {
							for tetra_face in tetra.get_triangle_3d_faces() {
								if tetra_face.does_edge_intersect(&bisecting_edge) {
									is_valid = false;
								}
							} 
						}
					}
					if is_valid {
						tetrahedrons.push(n_tet);
					}
				}
			}
		}
		// remove any tetra containing super tetra verts as that isn't a
		// real point supplied
		let mut final_tetrahedra = vec![];
		for tetra in tetrahedrons.iter_mut() {
			let a = *tetra.get_vertex_a();
			let b = *tetra.get_vertex_b();
			let c = *tetra.get_vertex_c();
			let d = *tetra.get_vertex_d();
			let mut is_valid = true;
			for s_tetra in super_tetra.iter() {
				let s_a = s_tetra[0];
				let s_b = s_tetra[1];
				let s_c = s_tetra[2];
				let s_d = s_tetra[3];
				if (a == s_a || a == s_b || a == s_c || a == s_d)
				|| (b == s_a || b == s_b || b == s_c || b == s_d)
				|| (c == s_a || c == s_b || c == s_c || c == s_d)
				|| (d == s_a || d == s_b || d == s_c || d == s_d)
			{
				is_valid = false;
			}
			}
			if is_valid {
				final_tetrahedra.push(tetra.clone());
			}
		}

		//verify all tetra are Delaunay qualified
		// for t in final_tetrahedra.iter() {
		// 	for (p, point) in points.iter().enumerate() {
		// 		if p == points.len() -1 {
		// 		// break;
		// 		}
		// 		if let Some(c) = t.compute_circumsphere() {
		// 			if c.is_point_within_sphere(point) {
		// 				error!("point is in a sphere {}", point);
		// 				error!("centre {}", c.get_centre());
		// 				error!("length {}", (point-c.get_centre()).length());
		// 				error!("radius {}", c.get_radius_squared().sqrt());
		// 			}
		// 		}
		// 	}
		// }
		Some(DelaunayData {
			shapes: final_tetrahedra,
		})
	}
	/// Get a refernce to the tetrahedron list
	pub fn get(&self) -> &Vec<tetrahedron::Tetrahedron> {
		&self.shapes
	}
}
/// Find the minimum `x-y-z` and maximum `x-y-z` of space containing all points
pub fn compute_dimension_bounds(points: &[Vec3]) -> (Vec3, Vec3) {
	let mut minimum_world_dimensions = Vec3::ZERO;
	let mut maximum_world_dimensions = Vec3::ZERO;

	for point in points.iter() {
		if point.x < minimum_world_dimensions.x {
			minimum_world_dimensions.x = point.x;
		}
		if point.y < minimum_world_dimensions.y {
			minimum_world_dimensions.y = point.y;
		}
		if point.z < minimum_world_dimensions.z {
			minimum_world_dimensions.z = point.z;
		}
		if point.x > maximum_world_dimensions.x {
			maximum_world_dimensions.x = point.x;
		}
		if point.y > maximum_world_dimensions.y {
			maximum_world_dimensions.y = point.y;
		}
		if point.z > maximum_world_dimensions.z {
			maximum_world_dimensions.z = point.z;
		}
	}
	// ensure points are within and not ON the bounardy
	(
		minimum_world_dimensions - Vec3::ONE,
		maximum_world_dimensions + Vec3::ONE,
	)
}

//TODO test this
/// Compute the vertices of 4 tetrahedra aligned in a diamond formation to ensure that all data points sit within the tetrahedra
pub fn compute_super_tetrahedra(
	minimum_world_dimensions: &Vec3,
	maximum_world_dimensions: &Vec3,
) -> [[Vec3; 4]; 4] {
	let delta_x = maximum_world_dimensions.x - minimum_world_dimensions.x;
	let delta_y = maximum_world_dimensions.y - minimum_world_dimensions.y;
	let delta_z = maximum_world_dimensions.z - minimum_world_dimensions.z;
	let midpoint = (maximum_world_dimensions + minimum_world_dimensions) / 2.0;

	let safety_factor = 2.0;

	let mid_up = midpoint + Vec3::new(0.0, delta_y * safety_factor, 0.0);
	let mid_down = midpoint + Vec3::new(0.0, -delta_y * safety_factor, 0.0);
	// in the -x, z plane
	let top_left = midpoint + Vec3::new(-delta_x * safety_factor, 0.0, delta_z * safety_factor);
	// in the x, z plane
	let top_right = midpoint + Vec3::new(delta_x * safety_factor, 0.0, delta_z * safety_factor);
	// in the x, -z plane
	let bottom_right = midpoint + Vec3::new(delta_x * safety_factor, 0.0, -delta_z * safety_factor);
	// in the -x, -z plane
	let bottom_left = midpoint + Vec3::new(-delta_x * safety_factor, 0.0, -delta_z * safety_factor);

	[
		[top_left, bottom_right, top_right, mid_up],
		[top_left, bottom_right, bottom_left, mid_up],
		[top_left, bottom_right, top_right, mid_down],
		[top_left, bottom_right, bottom_left, mid_down]
	]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dimension_bounds() {
		let points = vec![
			Vec3::new(50.0, 45.0, 0.0),
			Vec3::new(-23.0, -11.0, 64.0),
			Vec3::new(32.0, -3.0, -12.0),
		];
		let (min_bounds, max_bounds) = compute_dimension_bounds(&points);
		assert_eq!(Vec3::new(-24.0, -12.0, -13.0), min_bounds);
		assert_eq!(Vec3::new(51.0, 46.0, 65.0), max_bounds);
	}
}
