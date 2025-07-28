//! TODO
//!
//!

use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use bevy::prelude::*;

use crate::{prelude::DelaunayData, tetrahedron, voronoi::VoronoiData};

/// The vertices of a Voronoi Cell in 3-dimensions
pub struct VoronoiCell3d(Vec<Vec3>);

impl VoronoiCell3d {
	/// Get a reference to the list of vertices of this cell
	pub fn get_vertices(&self) -> &Vec<Vec3> {
		&self.0
	}
}

impl VoronoiData<VoronoiCell3d> {
	/// Get a reference to the list of Voronoi Cells
	pub fn get_cells(&self) -> &Vec<VoronoiCell3d> {
		&self.cells
	}
	/// Froma  series of 2d points in space compute the Voronoi Cells
	pub fn cells_from_points_2d(points: &mut Vec<Vec3>) -> Option<Self> {
		if let Some(delaunay) = DelaunayData::compute_triangulation_3d(points) {
			VoronoiData::cells_from_delaunay_3d(&delaunay)
		} else {
			None
		}
	}
	/// From a Delaunay Triangulation compute its dual - the Voronoi Cells
	pub fn cells_from_delaunay_3d(
		delaunay: &DelaunayData<tetrahedron::Tetrahedron>,
	) -> Option<Self> {
		// each circumcentre of a Delaunay triangle is a vertex of a Voronoi cell
		let tetras = delaunay.get();

		// uniquely identify each triangle
		let mut tetra_store: HashMap<usize, &tetrahedron::Tetrahedron> = HashMap::new();
		for (i, tetra) in tetras.iter().enumerate() {
			tetra_store.insert(i, tetra);
		}

		// store each set of tetrahedron IDs that together form a voronoi cell
		// if a vertex is shared 4+ times then all the circumcentres of tetras that use it
		// are voronoi vertices
		let id_sets = find_shared_sets(&tetra_store);

		// from the set of IDs find each circumsphere as a vertex on a voronoi cell
		let mut cells = vec![];
		for ids in id_sets.iter() {
			let mut cell_vertices = vec![];
			for id in ids.iter() {
				if let Some(tetra) = tetra_store.get(id) {
					if let Some(circumsphere) = tetra.compute_circumsphere() {
						let centre = circumsphere.get_centre();
						cell_vertices.push(*centre);
					}
				}
			}
			// find the midpoint of the cell vertices
			let mut total = Vec3::ZERO;
			for c in cell_vertices.iter() {
				total += c;
			}
			let midpoint = total / (cell_vertices.len() as f32);
			// sort the vertices in anti-clockwise order
			//TODO both vertices len squared cannot be zero
			//TODO test explcitly it works?
			cell_vertices.sort_by(|a, b| {
				if let Some(ordering) = (a - midpoint)
					.angle_between(*a)
					.partial_cmp(&(b - midpoint).angle_between(*b))
				{
					ordering
				} else {
					Ordering::Less
				}
			});
			cells.push(VoronoiCell3d(cell_vertices));
		}

		Some(VoronoiData { cells })
	}
}

/// Compare the vertices of tetrahedrons and identify groupings of IDs whereby 4
/// or more tetrahedrons share a vertex
fn find_shared_sets(map: &HashMap<usize, &tetrahedron::Tetrahedron>) -> HashSet<Vec<usize>> {
	let mut set = HashSet::new();
	for (id, tetra) in map {
		// compare each vert with the verts of all the other triangles
		let tetra_verts = tetra.get_vertices();
		for vert in tetra_verts {
			// store the ID of each other_tetra that shares this vertex
			let mut shared = vec![];
			for (other_id, other_tetra) in map {
				if id != other_id {
					let other_abc = other_tetra.get_vertices();
					if other_abc.contains(&vert) {
						shared.push(*other_id);
					}
				}
			}
			// including original id there must be 4+ ids sharing a vertex to constitute a cell
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