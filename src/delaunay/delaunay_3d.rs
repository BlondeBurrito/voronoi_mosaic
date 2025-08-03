//! Use Tetrahedralization to find tetrahedrons that comply to Delaunay in 3d
//!
//!


use crate::{prelude::{sort_vertices_3d, DelaunayData}, tetrahedron};
use bevy::prelude::*;

impl DelaunayData<tetrahedron::Tetrahedron> {
	/// From a series of 3d points in space calculate the Delaunay Tetrahedralization
	pub fn compute_triangulation_3d(points: &Vec<Vec3>) -> Option<Self> {
		if points.len() < 4 {
			warn!(
				"Minimum of 4 points required for tetrahedralization, supplied {} points",
				points.len()
			);
			return None;
		}
		//TODO ensure no dupciates in points
		// idenitfy spacial boundaries
		let (minimum_world_dimensions, maximum_world_dimensions) = compute_dimension_bounds(points);

		// compute the positions of a super tetrahedron that encompasses all points in space
		let super_tetra =
			compute_super_tetrahedron(&minimum_world_dimensions, &maximum_world_dimensions);
		// store tetrahedrons starting with the super one
		let mut tetrahedrons = vec![tetrahedron::Tetrahedron::new(
			super_tetra[0],
			super_tetra[1],
			super_tetra[2],
			super_tetra[3],
		)];
		// add each point at a time to the triangulation
		for point in points {
			// record tetrahedrons that don't qualify as Delaunay
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
				// we have a polyhedral hole around the point,
				// by using the known bad tetra we can join the point to
				// the vertex of each edge near it, thereby creating new polygons
				// that can undergo triangulation
				//
				// store the vertices of the bad tetra
				let mut vertices = Vec::new();
				for bad_tetra in bad_tetras.iter() {
					if !vertices.contains(bad_tetra.get_vertex_a()) {
						vertices.push(*bad_tetra.get_vertex_a());
					}
					if !vertices.contains(bad_tetra.get_vertex_b()) {
						vertices.push(*bad_tetra.get_vertex_b());
					}
					if !vertices.contains(bad_tetra.get_vertex_c()) {
						vertices.push(*bad_tetra.get_vertex_c());
					}
					if !vertices.contains(bad_tetra.get_vertex_d()) {
						vertices.push(*bad_tetra.get_vertex_d());
					}
				}
				// sort the vertices in anti-clockwise order by comparing the
				// angle between the point and a vertex
				//TODO both vertices len squared cannot be zero
				//TODO test explcitly it works?
				sort_vertices_3d(&mut vertices, point);
				// walk through vertices creating new tetrahedrons
				for i in 0..vertices.len() {
					if i < vertices.len() - 2 {
						tetrahedrons.push(tetrahedron::Tetrahedron::new(
							*point,
							vertices[i],
							vertices[i + 1],
							vertices[i + 2],
						));
					} else if i < vertices.len() - 1 {
						tetrahedrons.push(tetrahedron::Tetrahedron::new(
							*point,
							vertices[i],
							vertices[0],
							vertices[1],
						));
					} else {
						tetrahedrons.push(tetrahedron::Tetrahedron::new(
							*point,
							vertices[i],
							vertices[1],
							vertices[2],
						));
					}
				}
			}
		}
		// remove any tetra containing super tetra verts as that isn't a
		// real point supplied
		let mut final_tetragedrons = vec![];
		for tetra in tetrahedrons.iter_mut() {
			let a = *tetra.get_vertex_a();
			let b = *tetra.get_vertex_b();
			let c = *tetra.get_vertex_c();
			let d = *tetra.get_vertex_d();
			let s_a = super_tetra[0];
			let s_b = super_tetra[1];
			let s_c = super_tetra[2];
			let s_d = super_tetra[3];
			if (a != s_a && a != s_b && a != s_c && a != s_d)
				&& (b != s_a && b != s_b && b != s_c && b != s_d)
				&& (c != s_a && c != s_b && c != s_c && c != s_d)
				&& (d != s_a && d != s_b && d != s_c && d != s_d)
			{
				final_tetragedrons.push(tetra.clone());
			}
		}
		Some(DelaunayData {
			shapes: final_tetragedrons,
		})
	}
	/// Get a refernce to the tetrahedron list
	pub fn get(&self) -> &Vec<tetrahedron::Tetrahedron> {
		&self.shapes
	}
}
/// Find the minimum `x-y-z` and maximum `x-y-z` of space containing all points
fn compute_dimension_bounds(points: &[Vec3]) -> (Vec3, Vec3) {
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
/// Compute the vertices of a tetrahedron which encompassess all points
fn compute_super_tetrahedron(
	minimum_world_dimensions: &Vec3,
	maximum_world_dimensions: &Vec3,
) -> [Vec3; 4] {
	let delta_x = maximum_world_dimensions.x - minimum_world_dimensions.x;
	let delta_y = maximum_world_dimensions.y - minimum_world_dimensions.y;
	let delta_z = maximum_world_dimensions.z - minimum_world_dimensions.z;
	let midpoint = (maximum_world_dimensions + minimum_world_dimensions) / 2.0;

	let a = Vec3::new(midpoint.x, midpoint.y + (2.0 * delta_y), midpoint.z);
	let b = Vec3::new(
		midpoint.x + (2.0 * delta_x),
		midpoint.y - (2.0 * delta_y),
		midpoint.z + (2.0 * delta_z),
	);
	let c = Vec3::new(
		midpoint.x - (2.0 * delta_x),
		midpoint.y - (2.0 * delta_y),
		midpoint.z + (2.0 * delta_z),
	);
	let d = Vec3::new(
		midpoint.x - (2.0 * delta_x),
		midpoint.y - (2.0 * delta_y),
		midpoint.z - (2.0 * delta_z),
	);

	[a, b, c, d]
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
