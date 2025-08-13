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

use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::*;

use crate::{mosaic_2d::triangle_node2d::*, prelude::Circumcircle};

/// Describes the triangulation of a series of data points. Triangles and
/// vertices are stored with unique IDs
pub struct Delaunay2d {
	triangles: BTreeMap<usize, TriangleNode2d>,
	vertex_lookup: BTreeMap<usize, Vec2>,
}

impl Delaunay2d {
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
		// store vertices with a unique id
		let mut vertex_lookup = BTreeMap::from([
			(0, super_vertices[0]),
			(1, super_vertices[1]),
			(2, super_vertices[2]),
		]);
		// store a node representation of the triangle
		let mut triangles = BTreeSet::from([TriangleNode2d::new(0, 1, 2)]);

		// add each point at a time to the triangulation
		for point in points {
			// store the point with a unique ID
			let new_point_id = vertex_lookup.len();
			vertex_lookup.insert(new_point_id, *point);
			// record triangles that are not delaunay
			let bad_triangles = find_bad_triangles(&point, &triangles, &vertex_lookup);

			//TODO need to check for empty bad triangles?
			//TODO in theory it means a point is duplicate in dataset
			//TODO so we'd want to ignore it anyway...

			if !bad_triangles.is_empty() {
				// remove any bad triangles from the triangle list
				triangles.retain(|t| !bad_triangles.contains(t));

				// we have a polyhedral hole around the point,
				// by using the known bad triangles we can join the point to
				// each unique edge, thereby creating new triangles
				// that can undergo triangulation
				//
				// store the edges of bad triangles
				let mut edges = vec![];
				// store duplicate edges
				let mut duplicate_edges = vec![];

				for bad in bad_triangles.iter() {
					let bad_edges = bad.get_edges();
					for bad_e in bad_edges {
						if !edges.contains(&bad_e) {
							edges.push(bad_e);
						} else {
							duplicate_edges.push(bad_e);
						}
					}
				}
				// strip out duplicates as they must lie across the polyhedral hole
				edges.retain(|e| !duplicate_edges.contains(e));

				// create new triangles from the edges and store them
				for edge in edges {
					let a = new_point_id;
					let b = edge.get_vertex_a_id();
					let c = edge.get_vertex_b_id();
					let mut new_tri = TriangleNode2d::new(a, b, c);
					new_tri.sort_vertices_anti_clockwise(&vertex_lookup);
					triangles.insert(new_tri);
				}
			}
		}

		// remove any triangles that use vertices of the starting super-triangle - these were not real points in the data set, merely an initialisation to kick start triangulation
		let mut count: usize = 0;
		let mut final_triangles = BTreeMap::new();
		for tri in triangles {
			// IDs of the starting vertices
			let super_a = 0;
			let super_b = 1;
			let super_c = 2;
			if !tri.get_vertex_ids().contains(&super_a)
				&& !tri.get_vertex_ids().contains(&super_b)
				&& !tri.get_vertex_ids().contains(&super_c)
			{
				final_triangles.insert(count, tri);
				count += 1;
			}
		}
		// remove the super triangle vertices from the vertex lookup
		vertex_lookup.remove(&0);
		vertex_lookup.remove(&1);
		vertex_lookup.remove(&2);

		if final_triangles.len() > 0 {
			Some(Delaunay2d {
				triangles: final_triangles,
				vertex_lookup,
			})
		} else {
			warn!("No triangulation found");
			None
		}
	}
	/// Get a refernce to the map of unqiuely ID'ed triangles
	pub fn get_triangles(&self) -> &BTreeMap<usize, TriangleNode2d> {
		&self.triangles
	}
	/// Get a refernce to the map of vertex IDs and their position
	pub fn get_vertex_lookup(&self) -> &BTreeMap<usize, Vec2> {
		&self.vertex_lookup
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
/// Triangulation requires a super triangle that encompasses all data points as
/// well as any possible circumcircles bewteen the data points themselves.
///
/// To achieve this we construct a super plane from the minimum and maximum
/// dimension bounds of the data points and use this plane to build a triangle
fn compute_super_triangle(
	minimum_world_dimensions: &Vec2,
	maximum_world_dimensions: &Vec2,
) -> [Vec2; 3] {
	// define vertices of a rectangular plane
	let top_left = Vec2::new(minimum_world_dimensions.x, maximum_world_dimensions.y);
	let top_right = Vec2::new(maximum_world_dimensions.x, maximum_world_dimensions.y);
	let bottom_left = Vec2::new(minimum_world_dimensions.x, minimum_world_dimensions.y);
	let bottom_right = Vec2::new(maximum_world_dimensions.x, minimum_world_dimensions.y);
	// define 4 triangles using each edge of the plane and a new midpoint along the edge that's offset to be slightly inside the area of the plane
	let top_midpoint_offset = ((top_left + top_right) / 2.0) - Vec2::Y;
	let tri_1 = [top_midpoint_offset, top_left, top_right];

	let left_midpoint_offset = ((top_left + bottom_left) / 2.0) + Vec2::X;
	let tri_2 = [top_left, bottom_left, left_midpoint_offset];

	let bottom_midpoint_offset = ((bottom_left + bottom_right) / 2.0) + Vec2::Y;
	let tri_3 = [bottom_left, bottom_midpoint_offset, bottom_right];

	let right_midpoint_offset = ((top_right + bottom_right) / 2.0) - Vec2::X;
	let tri_4 = [top_right, right_midpoint_offset, bottom_right];

	// find the circumcircles of each triangle
	let a = Circumcircle::new(tri_1[0], tri_1[1], tri_1[2]).unwrap();
	let b = Circumcircle::new(tri_2[0], tri_2[1], tri_2[2]).unwrap();
	let c = Circumcircle::new(tri_3[0], tri_3[1], tri_3[2]).unwrap();
	let d = Circumcircle::new(tri_4[0], tri_4[1], tri_4[2]).unwrap();

	// using centres and radii construct the bounds of a super plane that encloses all possible cirumcentres
	let new_min = Vec2::new(
		b.get_centre().x - b.get_radius_sqaured().sqrt(),
		c.get_centre().y - c.get_radius_sqaured().sqrt(),
	);
	let new_max = Vec2::new(
		d.get_centre().x + d.get_radius_sqaured().sqrt(),
		a.get_centre().y + a.get_radius_sqaured().sqrt(),
	);

	compute_super_triangle_from_plane(&new_min, &new_max)
}

/// Compute the vertices of a triangle that encompasses all points, to ensure all points are contained we use the boundaries of a plane
fn compute_super_triangle_from_plane(
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
		- (maximum_world_dimensions.y - minimum_world_dimensions.y);
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
		(maximum_world_dimensions.y - intercept_b) / gradient_b,
		maximum_world_dimensions.y,
	);
	// repeat for the final vertex
	let gradient_c =
		(sup_triangle_vert_a.y - bottom_right.y) / (sup_triangle_vert_a.x - bottom_right.x);
	let intercept_c = bottom_right.y - gradient_c * bottom_right.x;
	let sup_triangle_vert_c = Vec2::new(
		(maximum_world_dimensions.y - intercept_c) / gradient_c,
		maximum_world_dimensions.y,
	);
	// we now have the vertices of a triangle that contains all cell origins
	[
		sup_triangle_vert_a,
		sup_triangle_vert_b,
		sup_triangle_vert_c,
	]
}

/// Search through triangles and identify any that do not qualify as Delaunay with respect to `point`
fn find_bad_triangles(
	point: &Vec2,
	triangles: &BTreeSet<TriangleNode2d>,
	vertex_lookup: &BTreeMap<usize, Vec2>,
) -> BTreeSet<TriangleNode2d> {
	let mut set = BTreeSet::new();
	// check if the point lies within the circumcircle of a triangle
	for tri in triangles.iter() {
		if let Some(circumcircle) = tri.compute_circumcircle(vertex_lookup)
			&& circumcircle.is_point_within_circle(point)
		{
			// if a point is within then it is not a delaunay triangle,
			// record this triangle for removal
			set.insert(tri.clone());
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
		let a = Vec2::new(0.0, -30600.0);
		let b = Vec2::new(-80199.99, 10200.0);
		let c = Vec2::new(80199.99, 10200.0);
		assert_eq!([a, b, c], s);
	}
	#[test]
	fn edge_count() {
		// if the super triangle is too small then the computed
		// triangulation only ends up with triangles joining data
		// points to super triangle vertices.
		// This test uses a small triangle and ensures that one
		// final triangle is computed between the points
		let points = vec![
			Vec2::new(-50.0, 0.0),
			Vec2::new(0.0, 50.0),
			Vec2::new(50.0, 0.0),
		];
		let data = Delaunay2d::compute_triangulation_2d(&points).unwrap();
		// should only be 1 triangle
		assert_eq!(1, data.triangles.len());
	}
	#[test]
	fn delaunay_too_few_points() {
		let points = vec![Vec2::new(50.0, 0.0), Vec2::new(-50.0, 0.0)];
		let result = Delaunay2d::compute_triangulation_2d(&points);
		assert!(result.is_none());
	}
	#[test]
	fn triangulation_is_none() {
		let points = vec![Vec2::ZERO; 3];
		assert!(Delaunay2d::compute_triangulation_2d(&points).is_none());
	}
}
