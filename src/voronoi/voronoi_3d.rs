//! 
//!
//!

use std::{
	cmp::Ordering,
	collections::BTreeMap,
};

use bevy::prelude::*;

use crate::{prelude::DelaunayData, tetrahedron, voronoi::VoronoiData};

/// The vertices of a Voronoi Cell in 3-dimensions
pub struct VoronoiCell3d {
	/// List of vertices that make up the cell
	vertices: Vec<Vec3>,
	/// The vertex which is the nearest site to the boundary vertices of the
	/// cell compared to any other cell source
	source_vertex: Vec3,
}

impl VoronoiCell3d {
	/// Get a reference to the list of vertices of this cell
	pub fn get_vertices(&self) -> &Vec<Vec3> {
		&self.vertices
	}
	/// Get the vertex which is the nearest site to the vertices of the cell
	/// compared to any other cell source
	pub fn get_source_vertex(&self) -> &Vec3 {
		&self.source_vertex
	}
	/// Get the midpoint between all vertices of the cell
	pub fn get_centre_position(&self) -> Vec3 {
		self.get_vertices().iter().sum::<Vec3>() / self.get_vertices().len() as f32
	}
	/// Get a list of edges of the cell. Arranged in an anti-clockwise fashion
	pub fn get_edges(&self) -> Vec<(Vec3, Vec3)> {
		// let mut edges = vec![];
		// for i in 0..self.get_vertices().len() {
		// 	if i < self.get_vertices().len() - 1 {
		// 		edges.push((self.get_vertices()[i], self.get_vertices()[i + 1]));
		// 	} else {
		// 		edges.push((self.get_vertices()[i], self.get_vertices()[0]));
		// 	}
		// }
		// edges
		todo!()
	}
}

impl VoronoiData<VoronoiCell3d> {
	/// Get a reference to the list of Voronoi Cells
	pub fn get_cells(&self) -> &BTreeMap<u32, VoronoiCell3d> {
		&self.cells
	}
	/// Get a mutable reference to the map of Voronoi Cells
	pub fn get_cells_mut(&mut self) -> &mut BTreeMap<u32, VoronoiCell3d> {
		&mut self.cells
	}
	/// From a Delaunay Tetrahedralization compute its dual - the Voronoi Cells
	pub fn from_delaunay_3d(
		delaunay: &DelaunayData<tetrahedron::Tetrahedron>,
	) -> Option<Self> {
		// each circumcentre of a Delaunay triangle is a vertex of a Voronoi cell
		let tetras = delaunay.get();

		// uniquely identify each triangle
		let mut tetra_store: BTreeMap<usize, &tetrahedron::Tetrahedron> = BTreeMap::new();
		for (i, tetra) in tetras.iter().enumerate() {
			tetra_store.insert(i, tetra);
		}

		// store each set of tetrahedron IDs that together form a voronoi cell
		// if a vertex is shared 4+ times then all the circumcentres of tetras that use it
		// are voronoi vertices
		let id_sets = find_shared_sets_tetrahedra(&tetra_store);

		// from the set of IDs find each circumsphere as a vertex on a voronoi cell
		let mut cells = BTreeMap::new();
		for (i, (ids, common_vertex)) in id_sets.iter().enumerate() {
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
			cells.insert(i as u32, VoronoiCell3d{
				vertices: cell_vertices,
				source_vertex: *common_vertex
			});
		}

		Some(VoronoiData { cells })
	}
	/// Convert each Voronoi Cell into a Bevy Mesh
	pub fn as_bevy_meshes_3d(&self) -> Vec<(Mesh, Vec3)> {
		//TODO
		vec![]
	}
	/// Clip all the [VoronoiCell3d] so they cannot extend or exist outside of
	/// a boundary polyhedron
	///
	/// The boundary polyhedron must contain at least 4 vertices and the vertices
	/// should be expressed in an anti-clockwise order around their centre
	///
	/// *NB: Delaunay and Voronoi are duals - they can precisely be converted from one fomrat to the other back and forth. By applying clipping to the Voronoi, cell vertices may be added/removed which will destroy the duality - i.e if you apply clipping you cannot convert Voronoi into Delaunay and expect to get your oringal dataset back*
	pub fn clip_cells_to_boundary(&mut self, boundary: &[Vec3]) {}
}

/// Compare the vertices of tetrahedra and identify groupings of IDs whereby 4
/// or more tetrahedra share a vertex.
///
/// The grouping forms the key and the value is the vertex they all have in common
fn find_shared_sets_tetrahedra(map: &BTreeMap<usize, &tetrahedron::Tetrahedron>) -> BTreeMap<Vec<usize>, Vec3> {
	let mut set = BTreeMap::new();
	for (id, tetra) in map {
		// compare each vert with the verts of all the other tetrahedra
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
			if shared.len() >= 3 {
				let mut ids = shared;
				ids.push(*id);
				ids.sort();
				set.insert(ids, *vert);
			}
		}
	}
	set
}