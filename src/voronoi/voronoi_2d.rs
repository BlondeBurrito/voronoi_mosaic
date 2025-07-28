//! TODO
//!
//!

use std::{
	cmp::Ordering,
	collections::{HashMap, HashSet},
};

use crate::{prelude::DelaunayData, triangle_2d};
use bevy::prelude::*;

use super::VoronoiData;

/// The vertices of a Voronoi Cell in 2-dimensions
pub struct VoronoiCell2d(Vec<Vec2>);

impl VoronoiCell2d {
	/// Get a reference to the list of vertices of this cell
	pub fn get_vertices(&self) -> &Vec<Vec2> {
		&self.0
	}
}

impl VoronoiData<VoronoiCell2d> {
	/// Get a reference to the list of Voronoi Cells
	pub fn get_cells(&self) -> &Vec<VoronoiCell2d> {
		&self.cells
	}
	/// Froma  series of 2d points in space compute the Voronoi Cells
	pub fn cells_from_points_2d(points: &mut Vec<Vec2>) -> Option<Self> {
		if let Some(delaunay) = DelaunayData::compute_triangulation_2d(points) {
			VoronoiData::cells_from_delaunay_2d(&delaunay)
		} else {
			None
		}
	}
	/// From a Delaunay Triangulation compute its dual - the Voronoi Cells
	pub fn cells_from_delaunay_2d(
		delaunay: &DelaunayData<triangle_2d::Triangle2d>,
	) -> Option<Self> {
		// each circumcentre of a Delaunay triangle is a vertex of a Voronoi cell
		let triangles = delaunay.get();

		// uniquely identify each triangle
		let mut triangle_store: HashMap<usize, &triangle_2d::Triangle2d> = HashMap::new();
		for (i, triangle) in triangles.iter().enumerate() {
			triangle_store.insert(i, triangle);
		}

		// store each set of triagnle IDs that together form a voronoi cell
		// if a vertex is shared 3+ times then all the circumcentres of traingles that use it
		// are voronoi vertices
		let id_sets = find_shared_sets(&triangle_store);

		// from the set of IDs find each circumcircle as a vertex on a voronoi cell
		let mut cells = vec![];
		for ids in id_sets.iter() {
			let mut cell_vertices = vec![];
			for id in ids.iter() {
				if let Some(triangle) = triangle_store.get(id) {
					if let Some(circumcircle) = triangle.compute_circumcircle() {
						let centre = circumcircle.get_centre();
						cell_vertices.push(*centre);
					}
				}
			}
			// find the midpoint of the cell vertices
			let mut total = Vec2::ZERO;
			for c in cell_vertices.iter() {
				total += c;
			}
			let midpoint = total / (cell_vertices.len() as f32);
			// sort the vertices in anti-clockwise order
			//TODO both vertices len squared cannot be zero
			cell_vertices.sort_by(|a, b| {
				if let Some(ordering) = (a - midpoint)
					.angle_to(*a)
					.partial_cmp(&(b - midpoint).angle_to(*b))
				{
					ordering
				} else {
					Ordering::Less
				}
			});
			cells.push(VoronoiCell2d(cell_vertices));
		}

		// VoronoiCell2d(cells)
		Some(VoronoiData { cells })
	}
}

/// Compare the vertices of triangles and identify groupings of IDs whereby 3
/// or more triangles share a vertex
fn find_shared_sets(map: &HashMap<usize, &triangle_2d::Triangle2d>) -> HashSet<Vec<usize>> {
	let mut set = HashSet::new();
	for (id, triangle) in map {
		// compare each vert with the verts of all the other triangles
		let tri_verts = triangle.get_vertices();
		for vert in tri_verts {
			// store the ID of each other_tri that shares this vertex
			let mut shared = vec![];
			for (other_id, other_tri) in map {
				if id != other_id {
					let other_abc = other_tri.get_vertices();
					if other_abc.contains(&vert) {
						shared.push(*other_id);
					}
				}
			}
			// including original id there must be 3+ ids sharing a vertex to constitute a cell
			if shared.len() >= 2 {
				let mut ids = shared;
				ids.push(*id);
				ids.sort();
				set.insert(ids);
			}
		}
		}
		set
}