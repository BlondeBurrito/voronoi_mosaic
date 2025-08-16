//! Use Tetrahedralization to find tetrahedrons that comply to Delaunay in 3d
//!
//! General process is to generate tetrahedra from data points, find the
//! circumspheres of these tetrahedra and ensure that the spheres do not
//! contain any data points
//!

use std::collections::{BTreeMap, BTreeSet};

use crate::{mosaic_3d::circumsphere::Circumsphere, mosaic_3d::tetrahedron_node::TetrahedronNode};
use bevy::prelude::*;

/// Describes the tetrahedralization of a series of data points. Tetrahedra and
/// vertices are stored with unique IDs
pub struct Delaunay3d {
	/// Uniquely ID'ed tetrahedra nodes
	tetrahedra: BTreeMap<usize, TetrahedronNode>,
	/// Uniquely ID'ed positions in space of each vertex
	vertex_lookup: BTreeMap<usize, Vec3>,
}

impl Delaunay3d {
	/// From a series of 3d points in space calculate the Delaunay Tetrahedralization
	pub fn compute_triangulation_3d(points: &[Vec3]) -> Option<Self> {
		if points.len() < 4 {
			error!(
				"Minimum of 4 points required for tetrahedralization, supplied {} points",
				points.len()
			);
			return None;
		}
		//TODO ensure no duplicates in points?
		// identify spacial boundaries
		let (minimum_world_dimensions, maximum_world_dimensions) = compute_dimension_bounds(points);

		// compute the positions of a super tetrahedron that encompasses all points in space
		// [mid_up, bottom_right, top_right, top_left, bottom_left, mid_down]
		// [ up, down, top, bottom, left, right]
		let super_tetra =
			compute_super_tetrahedra(points, &minimum_world_dimensions, &maximum_world_dimensions);

		// store vertices with a unique id
		let mut vertex_lookup = BTreeMap::from([
			(0, super_tetra[0]),
			(1, super_tetra[1]),
			(2, super_tetra[2]),
			(3, super_tetra[3]),
			(4, super_tetra[4]),
			(5, super_tetra[5]),
		]);

		// store tetrahedra starting with the super 4
		let mut tetrahedra = BTreeSet::from([
			TetrahedronNode::new(0, 5, 2, 4),
			TetrahedronNode::new(0, 5, 3, 4),
			TetrahedronNode::new(1, 5, 2, 4),
			TetrahedronNode::new(1, 5, 3, 4),
		]);

		let mut problematic_points = vec![];
		// add each point at a time to the triangulation
		for point in points {
			// find tetrahedra that are not Delaunay
			let bad_tetrahedra = find_bad_tetrahedra(point, &tetrahedra, &vertex_lookup);

			if bad_tetrahedra.is_empty() {
				// if empty then the point is in a hole where previous tetrahedralizations failed to produce delaunay tetras that could fill the void
				// store the point to be retried at the end
				problematic_points.push(point);
			} else {
				// store the point with a unique ID
				let new_point_id = vertex_lookup.len();
				vertex_lookup.insert(new_point_id, *point);

				// remove any bad tetrahedrons from the set
				tetrahedra.retain(|t| !bad_tetrahedra.contains(t));

				// removing bad tetras creates a hole around the point, we want
				// to build new tetrahedrons with the point to
				// progress the tetrahedralization
				//
				// store each triangle face
				let mut face_triangles = vec![];
				// store each duplicate triangle face
				let mut duplicate_face_triangles = vec![];
				for bad_tetra in bad_tetrahedra.iter() {
					let faces = bad_tetra.get_triangle_node_3d_faces();
					for face in faces.iter() {
						if !face_triangles.contains(face) {
							face_triangles.push(*face);
						} else {
							duplicate_face_triangles.push(*face);
						}
					}
				}
				// remove duplicate faces as that face crosses the polyhedral hole
				face_triangles.retain(|f| !duplicate_face_triangles.contains(f));

				// construct new tetrahedra from the point and each triangle face
				let mut new_tetras = vec![];
				for tri in face_triangles {
					new_tetras.push(TetrahedronNode::new(
						new_point_id,
						tri.get_vertex_a_id(),
						tri.get_vertex_b_id(),
						tri.get_vertex_c_id(),
					));
				}

				//TODO is this needed?
				// only store a new tetra if it is Delaunay - test to
				// // ensure it doesn't intersect with any existing tetras
				while let Some(n_tet) = new_tetras.pop() {
					let mut is_valid = true;

					// // if an edge of a proposed tetra intersects an existing tetra
					// // then the proposed is not Delaunay
					// for edge in n_tet.get_edges() {
					// 	for tetra in tetrahedra.iter() {
					// 		for face in tetra.get_triangle_node_3d_faces() {
					// 			if face.does_edge_intersect_id(&edge, &vertex_lookup) {
					// 				is_valid = false;
					// 			}
					// 		}
					// 	}
					// }
					// // as edges are allowed to touch faces/vertices we
					// // perform an additional check to verify that a face
					// // doesn't slice into another face - use a bisecting
					// // line down the middle of each face of the
					// // proposed new tetra and check for intersection
					// for face in n_tet.get_triangle_node_3d_faces().iter() {
					// 	let a = vertex_lookup.get(&face.get_vertex_a_id()).unwrap();
					// 	let b = vertex_lookup.get(&face.get_vertex_b_id()).unwrap();
					// 	let c = vertex_lookup.get(&face.get_vertex_c_id()).unwrap();

					// 	let edge_vertex_a = a;
					// 	let edge_vertex_b = (b + c) / 2.0;
					// 	for tetra in tetrahedra.iter() {
					// 		for tetra_face in tetra.get_triangle_node_3d_faces() {
					// 			let tri_vertex_a =
					// 				vertex_lookup.get(&tetra_face.get_vertex_a_id()).unwrap();
					// 			let tri_vertex_b =
					// 				vertex_lookup.get(&tetra_face.get_vertex_b_id()).unwrap();
					// 			let tri_vertex_c =
					// 				vertex_lookup.get(&tetra_face.get_vertex_c_id()).unwrap();
					// 			if tetra_face.does_edge_intersect(
					// 				tri_vertex_a,
					// 				tri_vertex_b,
					// 				tri_vertex_c,
					// 				edge_vertex_a,
					// 				&edge_vertex_b,
					// 			) {
					// 				is_valid = false;
					// 			}
					// 		}
					// 	}
					// }
					// if the tetra has no circumsphere then consider it invalid,
					// i.e its vertices are coplanar so it is degenerate
					if n_tet.compute_circumsphere(&vertex_lookup).is_none() {
						is_valid = false;
					}
					if is_valid {
						tetrahedra.insert(n_tet);
					}
				}
			}
		}

		//TODO retry adding the problematic points
		warn!(
			"Number of problematic points ignored {}",
			problematic_points.len()
		);

		// remove any tetrahedra that use vertices of the starting
		// super-tetrahedra - these were not real points in the data set,
		// merely an initialisation to kick start triangulation
		let mut count: usize = 0;
		let mut final_tetrahedra = BTreeMap::new();
		for tet in tetrahedra {
			// IDs of starting vertices
			let s_a = 0;
			let s_b = 1;
			let s_c = 2;
			let s_d = 3;
			let s_e = 4;
			let s_f = 5;

			if !tet.get_vertex_ids().contains(&s_a)
				&& !tet.get_vertex_ids().contains(&s_b)
				&& !tet.get_vertex_ids().contains(&s_c)
				&& !tet.get_vertex_ids().contains(&s_d)
				&& !tet.get_vertex_ids().contains(&s_e)
				&& !tet.get_vertex_ids().contains(&s_f)
			{
				final_tetrahedra.insert(count, tet);
				count += 1;
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

		// remove the super tetra vertices from the vertex lookup
		vertex_lookup.remove(&0);
		vertex_lookup.remove(&1);
		vertex_lookup.remove(&2);
		vertex_lookup.remove(&3);
		vertex_lookup.remove(&4);
		vertex_lookup.remove(&5);

		if !final_tetrahedra.is_empty() {
			Some(Delaunay3d {
				tetrahedra: final_tetrahedra,
				vertex_lookup,
			})
		} else {
			warn!("No tetrahedralization found");
			None
		}
	}
	/// Get a refernce to the tetrahedron map
	pub fn get_tetrahedra(&self) -> &BTreeMap<usize, TetrahedronNode> {
		&self.tetrahedra
	}
	/// Get a refernce to the map of vertex IDs and their position
	pub fn get_vertex_lookup(&self) -> &BTreeMap<usize, Vec3> {
		&self.vertex_lookup
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

/// Compute the vertices of 4 tetrahedra aligned in a diamond formation to ensure that all data points sit within the tetrahedra and that all possible circumspheres between data points sit within the tetrahedra too
fn compute_super_tetrahedra(
	points: &[Vec3],
	minimum_world_dimensions: &Vec3,
	maximum_world_dimensions: &Vec3,
) -> [Vec3; 6] {
	let mut largest_radius_sq = 0.0;

	//TODO if out duplicate points to avoid calling circumcpshere code, expensive
	for vertex_a in points.iter() {
		for vertex_b in points.iter() {
			for vertex_c in points.iter() {
				for vertex_d in points.iter() {
					if let Some(sphere) =
						Circumsphere::new(*vertex_a, *vertex_b, *vertex_c, *vertex_d)
						&& sphere.get_radius_squared() > largest_radius_sq
					{
						largest_radius_sq = sphere.get_radius_squared();
					}
				}
			}
		}
	}
	let largest_radius = largest_radius_sq.sqrt();
	let diff = Vec3::new(largest_radius, largest_radius, largest_radius);
	let new_min = minimum_world_dimensions - diff;
	let new_max = maximum_world_dimensions + diff;
	compute_super_tetra_vertices(&new_min, &new_max)
}

/// Given minimum and maximum bounds of space find the shared vertices of 4 super tetrahedra
pub fn compute_super_tetra_vertices(min_dimensions: &Vec3, max_dimensions: &Vec3) -> [Vec3; 6] {
	let delta_x = max_dimensions.x - min_dimensions.x;
	let delta_y = max_dimensions.y - min_dimensions.y;
	let delta_z = max_dimensions.z - min_dimensions.z;
	let midpoint = (max_dimensions + min_dimensions) / 2.0;

	// let safety_factor = 1.0;

	// let mid_up = midpoint + Vec3::new(0.0, delta_y * safety_factor, 0.0);
	// let mid_down = midpoint + Vec3::new(0.0, -delta_y * safety_factor, 0.0);
	// // in the -x, z plane
	// let top_left = midpoint + Vec3::new(-delta_x * safety_factor, 0.0, delta_z * safety_factor);
	// // in the x, z plane
	// let top_right = midpoint + Vec3::new(delta_x * safety_factor, 0.0, delta_z * safety_factor);
	// // in the x, -z plane
	// let bottom_right = midpoint + Vec3::new(delta_x * safety_factor, 0.0, -delta_z * safety_factor);
	// // in the -x, -z plane
	// let bottom_left = midpoint + Vec3::new(-delta_x * safety_factor, 0.0, -delta_z * safety_factor);

	// [
	// 	[top_left, bottom_right, top_right, mid_up],
	// 	[top_left, bottom_right, bottom_left, mid_up],
	// 	[top_left, bottom_right, top_right, mid_down],
	// 	[top_left, bottom_right, bottom_left, mid_down],
	// ];
	// [
	// 	mid_up,
	// 	bottom_right,
	// 	top_right,
	// 	top_left,
	// 	bottom_left,
	// 	mid_down,
	// ];
	let up = midpoint + Vec3::new(0.0, delta_y, 0.0);
	let down = midpoint + Vec3::new(0.0, -delta_y, 0.0);
	let top = midpoint + Vec3::new(0.0, 0.0, delta_z);
	let bottom = midpoint + Vec3::new(0.0, 0.0, -delta_z);
	let left = midpoint + Vec3::new(-delta_x, 0.0, 0.0);
	let right = midpoint + Vec3::new(delta_x, 0.0, 0.0);
	[up, down, top, bottom, left, right]
}

/// Search through tetrahedra and identify any that do not qualify as Delaunay with respect to `point`
fn find_bad_tetrahedra(
	point: &Vec3,
	tetrahedra: &BTreeSet<TetrahedronNode>,
	vertex_lookup: &BTreeMap<usize, Vec3>,
) -> BTreeSet<TetrahedronNode> {
	let mut set = BTreeSet::new();
	// check if the point lies within the circumsphere of a tetrahedron
	for tet in tetrahedra.iter() {
		if let Some(circumsphere) = tet.compute_circumsphere(vertex_lookup)
			&& circumsphere.is_point_within_sphere(point)
		{
			// if a point is within then it is not a delaunay tetrahedron,
			// record this tetrahedron for removal
			set.insert(*tet);
		}
	}
	set
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
	#[test]
	fn delaunay_too_few_points() {
		let points = vec![
			Vec3::new(50.0, 45.0, 0.0),
			Vec3::new(-23.0, -11.0, 64.0),
			Vec3::new(32.0, -3.0, -12.0),
		];
		let result = Delaunay3d::compute_triangulation_3d(&points);
		assert!(result.is_none());
	}
	#[test]
	fn super_tetra() {
		let points = vec![
			Vec3::new(-50.0, -50.0, -50.0),
			Vec3::new(50.0, -50.0, -50.0),
			Vec3::new(50.0, -50.0, 50.0),
			Vec3::new(-50.0, -50.0, 50.0),
			//
			Vec3::new(-50.0, 50.0, -50.0),
			Vec3::new(50.0, 50.0, -50.0),
			Vec3::new(50.0, 50.0, 50.0),
			Vec3::new(-50.0, 50.0, 50.0),
			//
			Vec3::new(0.0, 0.0, 0.0),
		];
		let (min, max) = compute_dimension_bounds(&points);
		let super_tetrahedra = compute_super_tetrahedra(&points, &min, &max);

		let t1 = Vec3::new(0.0, 361.80762, 0.0);
		assert_eq!(t1, super_tetrahedra[0]);
		let t2 = Vec3::new(0.0, -361.80762, 0.0);
		assert_eq!(t2, super_tetrahedra[1]);
		let t3 = Vec3::new(0.0, 0.0, 361.80762);
		assert_eq!(t3, super_tetrahedra[2]);
		let t4 = Vec3::new(0.0, 0.0, -361.80762);
		assert_eq!(t4, super_tetrahedra[3]);
		let t5 = Vec3::new(-361.80762, 0.0, 0.0);
		assert_eq!(t5, super_tetrahedra[4]);
		let t6 = Vec3::new(361.80762, 0.0, 0.0);
		assert_eq!(t6, super_tetrahedra[5]);
	}
	#[test]
	fn delaunay() {
		// ensure that the super tetra is sized correctly so that the points
		// are perfectly triangulated.
		// cube with a point in the middle
		let points = vec![
			Vec3::new(-50.0, -50.0, -50.0),
			Vec3::new(50.0, -50.0, -50.0),
			Vec3::new(50.0, -50.0, 50.0),
			Vec3::new(-50.0, -50.0, 50.0),
			//
			Vec3::new(-50.0, 50.0, -50.0),
			Vec3::new(50.0, 50.0, -50.0),
			Vec3::new(50.0, 50.0, 50.0),
			Vec3::new(-50.0, 50.0, 50.0),
			//
			Vec3::new(0.0, 0.0, 0.0),
		];
		let delaunay = Delaunay3d::compute_triangulation_3d(&points).unwrap();
		assert_eq!(12, delaunay.get_tetrahedra().len());
	}
}
