//! From a series of 2d points in space compute a Delaunay Triangulation.
//!
//! The Bowyer-Watson algorithm is used - we describe a triangle that is large
//! enough to enclose all the points of a data set within it. This starting
//! triangle is stored in a mutable list which grows as more Delaunay Triangles
//! are calculated.
//!
//! The first data point is added to the triangulation and we use the
//! cirumcircles of any triangles in the list to determine if they are still
//! Delaunay. If a triangles circumcircle contains a data point then the
//! triangle is not Delaunay, the triangle is removed from the list and its
//! edges are used to construct new triangles with the inserted data point -
//! this makes new Delaunay Triangles. Step by step we add all points to the
//! triangulation, computing new triangles as we go until all points have been
//! processed and we arrive with a list of true Delaunay Triangles. Note that
//! the original "super triangle" to kick-start the triangulation gets removed
//! at the end as they were imaginary data points.
//!

use bevy::prelude::*;

use crate::{
	prelude::{DelaunayData, sort_vertices_2d},
	triangle_2d,
};

impl DelaunayData<triangle_2d::Triangle2d> {
	/// From a series of 2d points in a plane compute the Delaunay
	/// Triangulation with the Bowyer-Watson algorithm.
	pub fn compute_triangulation_2d(points: &Vec<Vec2>) -> Option<Self> {
		if points.len() < 3 {
			error!(
				"Minimum of 3 points required for triangulation, supplied {} points",
				points.len()
			);
			return None;
		}
		//TODO ensure no dupciates in points?
		// find the dimensions of a plane that the points occupy
		let (minimum_world_dimensions, maximum_world_dimensions) = compute_dimension_bounds(points);
		// compute the vertices of a super triangle which encompassess all the points
		let super_vertices =
			compute_super_triangle(&minimum_world_dimensions, &maximum_world_dimensions);
		// store triangles generaterd starting with the super triangle
		let mut triangles = vec![triangle_2d::Triangle2d::new(
			super_vertices[0],
			super_vertices[1],
			super_vertices[2],
		)];
		// add each point at a time to the triangulation
		for point in points {
			// record triangles that don't qualify as Delaunay
			let mut bad_triangles = vec![];
			// check if the point lies within the circumcircle of a triangle
			for tri in triangles.iter() {
				if let Some(circumcircle) = tri.compute_circumcircle() {
					if circumcircle.is_point_within_circle(point) {
						// if a point is within then it is not a delaunay triangle,
						// record this triangle for removal
						bad_triangles.push(tri.clone());
					}
				} else {
					warn!("Unable to compute circumcircle of triangle {:?}", tri);
				}
			}
			// remove any bad triangles from the triangle list
			if !bad_triangles.is_empty() {
				triangles.retain(|t| !bad_triangles.contains(t));
				// we have a polyhedral hole around the point,
				// by using the known bad triangles we can join the point to
				// the vertex of each edge near it, thereby creating new triangles
				// that can undergo triangulation
				//
				// store the vertices of the bad triangles
				let mut vertices = Vec::new();
				for bad_tri in bad_triangles.iter() {
					if !vertices.contains(bad_tri.get_vertex_a()) {
						vertices.push(*bad_tri.get_vertex_a());
					}
					if !vertices.contains(bad_tri.get_vertex_b()) {
						vertices.push(*bad_tri.get_vertex_b());
					}
					if !vertices.contains(bad_tri.get_vertex_c()) {
						vertices.push(*bad_tri.get_vertex_c());
					}
				}
				//TODO instead of sorting having to do arccos calcs could it be replaced with simply taking each edge of the bad triangles, ensuring they are all unique (remove shared edges as they cross the polyhedral hole) and joining each valid edge to the data point to create the new triangles??
				// sort the vertices in anti-clockwise order by comparing the
				// angle between the point and a vertex
				sort_vertices_2d(&mut vertices, point);
				// walk through vertex pairs creating new triangles
				for i in 0..vertices.len() {
					if i < vertices.len() - 1 {
						let (a, b, c) = (*point, vertices[i], vertices[i + 1]);
						let new_tri = triangle_2d::Triangle2d::new(a, b, c);
						triangles.push(new_tri);
					} else {
						let (a, b, c) = (*point, vertices[i], vertices[0]);
						let new_tri = triangle_2d::Triangle2d::new(a, b, c);
						triangles.push(new_tri);
					}
				}
			}
		}
		// remove any triangle containing super triangle verts as that isn't a
		// real point supplied
		let mut final_triangles = vec![];
		for triangle in triangles.iter_mut() {
			let s_a = super_vertices[0];
			let s_b = super_vertices[1];
			let s_c = super_vertices[2];
			let tri_verts = triangle.get_vertices();
			if !tri_verts.contains(&&s_a)
				&& !tri_verts.contains(&&s_b)
				&& !tri_verts.contains(&&s_c)
			{
				final_triangles.push(triangle.clone());
			}
		}
		Some(DelaunayData {
			shapes: final_triangles,
		})
	}
	/// Get a refernce to the triangle list
	pub fn get(&self) -> &Vec<triangle_2d::Triangle2d> {
		&self.shapes
	}
}

/// Find the minimum `x-y` and maximum `x-y` of a plane that contains all points
fn compute_dimension_bounds(points: &[Vec2]) -> (Vec2, Vec2) {
	let mut minimum_world_dimensions = Vec2::ZERO;
	let mut maximum_world_dimensions = Vec2::ZERO;
	for point in points.iter() {
		if point.x < minimum_world_dimensions.x {
			minimum_world_dimensions.x = point.x;
		}
		if point.y < minimum_world_dimensions.y {
			minimum_world_dimensions.y = point.y;
		}
		if point.x > maximum_world_dimensions.x {
			maximum_world_dimensions.x = point.x;
		}
		if point.y > maximum_world_dimensions.y {
			maximum_world_dimensions.y = point.y;
		}
	}
	// ensure points are within and not ON the bounardy
	(
		minimum_world_dimensions - Vec2::ONE,
		maximum_world_dimensions + Vec2::ONE,
	)
}

/// Compute the vertices of a triangle that encompasses all points, to ensure all points are contained we use the boundaries of the plane which all points sit within
fn compute_super_triangle(
	minimum_world_dimensions: &Vec2,
	maximum_world_dimensions: &Vec2,
) -> [Vec2; 3] {
	// we place an imaginary triangle over the plane so that all cell origins lie within it
	// plane looks like:
	// _________
	// |       |
	// |_______|
	// for the furthest point of the super triangle imagine the bottom two plane
	// vertices being joined to a vertex to make a triangle
	// __________
	// |        |
	// |________|
	//  \      /
	//   \    /
	//    \  /
	//     \/
	// by computing this furthest point we have one vertex of the super triangle:
	let bottom_left = minimum_world_dimensions;
	let bottom_right = Vec2::new(maximum_world_dimensions.x, minimum_world_dimensions.y);
	let x = bottom_left.x + (bottom_right.x - bottom_left.x) / 2.0;
	// we actually scale it away from the corners of the plane by a factor of 2 as if the plane is wide but thin then a very acute super triangle is produced which can cause holes in the triangualtion (all triangles formed with super verts that get removed at the end) when the data set is very small
	let y = minimum_world_dimensions.y
		- 2.0 * (maximum_world_dimensions.y - minimum_world_dimensions.y);
	let sup_triangle_vert_a = Vec2::new(x, y);
	// by treating the maximum y of the plane as a striahgt line parallel to x we can
	// take line equations from the furthest point sup_triangle_vert_a with the
	// two conrers that helped construct and idenitfy where those lines cross the straight
	// parallel line to determine the two remaining vertices of the super triangle
	//________________________________
	//     b\  __________  /c
	//       \ |        | /
	//        \|________|/
	//          \       /
	//           \     /
	//            \   /
	//             \a/

	let gradient_b =
		(sup_triangle_vert_a.y - bottom_left.y) / (sup_triangle_vert_a.x - bottom_left.x);
	let intercept_b = bottom_left.y - gradient_b * bottom_left.x;
	// using y=mx + c we can find the point of y = max (plus a bit of wiggle room) for x giving us another
	// vertex of the super triangle
	let sup_triangle_vert_b = Vec2::new(
		(2.0 * maximum_world_dimensions.y - intercept_b) / gradient_b,
		2.0 * maximum_world_dimensions.y,
	);
	// repeat for the final vertex
	let gradient_c =
		(sup_triangle_vert_a.y - bottom_right.y) / (sup_triangle_vert_a.x - bottom_right.x);
	let intercept_c = bottom_right.y - gradient_c * bottom_right.x;
	let sup_triangle_vert_c = Vec2::new(
		(2.0 * maximum_world_dimensions.y - intercept_c) / gradient_c,
		2.0 * maximum_world_dimensions.y,
	);
	// we now have the vertices of a triangle that contains all cell origins
	[
		sup_triangle_vert_a,
		sup_triangle_vert_b,
		sup_triangle_vert_c,
	]
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dimension_bounds() {
		let points = vec![
			Vec2::new(50.0, 45.0),
			Vec2::new(-23.0, -11.0),
			Vec2::new(32.0, -3.0),
		];
		let (min_bounds, max_bounds) = compute_dimension_bounds(&points);
		assert_eq!(Vec2::new(-24.0, -12.0), min_bounds);
		assert_eq!(Vec2::new(51.0, 46.0), max_bounds);
	}
	#[test]
	fn super_trinagle() {
		let minimum_world_dimensions = Vec2::new(-100.0, -200.0);
		let maximum_world_dimensions = Vec2::new(100.0, 200.0);
		let s = compute_super_triangle(&minimum_world_dimensions, &maximum_world_dimensions);
		let a = Vec2::new(0.0, -1000.0);
		let b = Vec2::new(-175.0, 400.0);
		let c = Vec2::new(175.0, 400.0);
		assert_eq!([a, b, c], s);
	}
	#[test]
	fn edge_count() {
		let points = vec![
			Vec2::new(50.0, 0.0),
			Vec2::new(-50.0, 0.0),
			Vec2::new(0.0, 50.0),
		];
		let data = DelaunayData::compute_triangulation_2d(&points).unwrap();
		// should only be 1 triangle
		assert_eq!(1, data.get().len());
		assert_eq!(3, data.get().first().unwrap().get_edges().len());
	}
}
