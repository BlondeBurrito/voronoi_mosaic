//! 3d Voronoi is built from a Delaunay Tetrahedralization by locating the
//! circumcentres of circumspheres and grouping those centres based on
//! tetrahedra boundary sharing
//!
//!

use std::collections::{BTreeMap, BTreeSet};

use bevy::prelude::*;

use crate::mosaic_3d::{
	delaunay::Delaunay3d, edge_node3d::EdgeNode3d, tetrahedron_node::TetrahedronNode,
};

/// The vertices of a Voronoi Cell in 3-dimensions
pub struct VoronoiCell3d {
	/// List of vertex IDs that make up the cell
	vertices: Vec<usize>,
	/// List of edges in node ID form
	edges: BTreeSet<EdgeNode3d>,
	/// The Delaunay vertex ID which is the nearest site to the boundary vertices of the
	/// cell compared to any other cell source
	generating_point: usize,
}

impl VoronoiCell3d {
	/// Get a reference to the list of vertex IDs of this cell
	pub fn get_vertex_ids(&self) -> &Vec<usize> {
		&self.vertices
	}
	/// Get the Delaunay vertex ID which is the nearest site to the vertices of the cell
	/// compared to any other cell source
	pub fn get_generating_point(&self) -> &usize {
		&self.generating_point
	}
	/// Get the midpoint between all vertices of the cell
	pub fn get_centre_position(&self, vertex_lookup: &BTreeMap<usize, Vec3>) -> Vec3 {
		let vertex_ids = self.get_vertex_ids();
		let mut sum = Vec3::ZERO;
		for id in vertex_ids.iter() {
			let point = vertex_lookup.get(id).unwrap();
			sum += point;
		}
		sum / vertex_ids.len() as f32
	}
	/// Get a list of edges of the cell
	pub fn get_edges(&self) -> &BTreeSet<EdgeNode3d> {
		&self.edges
	}
}

/// Describes Voronoi Cells
pub struct Voronoi3d {
	/// Uniquely ID'ed cells
	cells: BTreeMap<usize, VoronoiCell3d>,
	/// Each vertex of a cell is an ID corresponding to a point in space
	vertex_lookup: BTreeMap<usize, Vec3>,
}

impl Voronoi3d {
	/// Get a reference to the list of usize Cells
	pub fn get_cells(&self) -> &BTreeMap<usize, VoronoiCell3d> {
		&self.cells
	}
	/// Get a mutable reference to the map of Voronoi Cells
	pub fn get_cells_mut(&mut self) -> &mut BTreeMap<usize, VoronoiCell3d> {
		&mut self.cells
	}
	/// Get a refernce to the map of vertex IDs and their position
	pub fn get_vertex_lookup(&self) -> &BTreeMap<usize, Vec3> {
		&self.vertex_lookup
	}
	/// Get a mutable refernce to the map of vertex IDs and their position
	pub fn get_vertex_lookup_mut(&mut self) -> &mut BTreeMap<usize, Vec3> {
		&mut self.vertex_lookup
	}
	/// From a Delaunay Tetrahedralization compute its dual - the Voronoi Cells
	pub fn from_delaunay_3d(delaunay: &Delaunay3d) -> Option<Self> {
		// each circumcentre of a Delaunay tetrahedron is a vertex of a Voronoi cell
		let tetras_store = delaunay.get_tetrahedra();
		let delaunay_vertex_lookup = delaunay.get_vertex_lookup();

		// store IDs for all the cirumcentres
		let mut voronoi_vertex_lookup = BTreeMap::new();
		// store the tetrahedron ID and what circumcentre ID is corresponds to
		let mut tetrahedron_to_circumcentre_ids = BTreeMap::new();

		for (tet_id, tet) in tetras_store.iter() {
			if let Some(circumsphere) = tet.compute_circumsphere(delaunay_vertex_lookup) {
				let centre = circumsphere.get_centre();
				let voronoi_id = voronoi_vertex_lookup.len();
				voronoi_vertex_lookup.insert(voronoi_id, *centre);
				tetrahedron_to_circumcentre_ids.insert(*tet_id, voronoi_id);
			} else {
				error!("Tet doesnt have circumsphere");
			}
		}

		// loop thorugh tetrahedra and find cases where 4 or more tetrahedra
		// have a vertex id in common, this means that the circumcentres of
		// those tetrahedra are the voronoi vertices of a cell.
		// Keys are sets of tetrahedra IDs, value is the ID of the generating point
		let cell_tetrahedra = find_shared_sets(tetras_store);

		// convert the tetrahedra groupings into voronoi vertices IDs
		let cells = compute_cells_from_tetrahedra_sets(
			&cell_tetrahedra,
			tetras_store,
			&tetrahedron_to_circumcentre_ids,
		);

		Some(Voronoi3d {
			cells,
			vertex_lookup: voronoi_vertex_lookup,
		})
	}
	/// Convert each Voronoi Cell into a Bevy Mesh
	pub fn as_bevy3d_meshes(&self) -> Vec<(Mesh, Vec3)> {
		warn!("Unimplemented, this currently does nothing");
		vec![]
	}
	/// Clip all the [VoronoiCell3d] so they cannot extend or exist outside of
	/// a boundary polyhedron
	///
	/// The boundary polyhedron must contain at least 4 vertices and the vertices
	/// should be expressed in an anti-clockwise order around their centre
	///
	/// *NB: Delaunay and Voronoi are duals - they can precisely be converted from one fomrat to the other back and forth. By applying clipping to the Voronoi, cell vertices may be added/removed which will destroy the duality - i.e if you apply clipping you cannot convert Voronoi into Delaunay and expect to get your oringal dataset back*
	pub fn as_clipped_bevy3d_meshes(&self, _boundary: &[Vec3]) -> Vec<(Mesh, Vec3)> {
		warn!("Unimplemented, this currently does nothing");
		vec![]
	}
}

/// Compare the vertices of tetrahedra and identify groupings of IDs whereby 4
/// or more tetrahedra share a vertex.
///
/// The grouping forms the key and the value is the vertex they all have in common
fn find_shared_sets(
	tetras_store: &BTreeMap<usize, TetrahedronNode>,
) -> BTreeMap<BTreeSet<&usize>, &usize> {
	let mut cell_tetrahedra = BTreeMap::new();
	for (this_tet_id, this_tet) in tetras_store.iter() {
		// loop through all vertex IDs
		for this_vert_id in this_tet.get_vertex_ids() {
			let mut shared_tet_ids = BTreeSet::from([this_tet_id]);
			// loop over other triangles
			for (other_tet_id, other_tet) in tetras_store.iter() {
				if this_tet_id != other_tet_id {
					if other_tet.get_vertex_ids().contains(this_vert_id) {
						// triangles share a common vertex ID, store other
						shared_tet_ids.insert(other_tet_id);
					}
				}
			}
			if shared_tet_ids.len() >= 4 {
				// we have found a series of tetrahedra with a common vertex,
				// their circumcentres are voronoi vertices
				cell_tetrahedra.insert(shared_tet_ids, this_vert_id);
			}
		}
	}
	cell_tetrahedra
}
/// From tetrahedra groupings calculate each [VoronoiCell3d] from their
/// circumcentres
fn compute_cells_from_tetrahedra_sets(
	cell_tetrahedra: &BTreeMap<BTreeSet<&usize>, &usize>,
	tetras_store: &BTreeMap<usize, TetrahedronNode>,
	tetrahedron_to_circumcentre_ids: &BTreeMap<usize, usize>,
) -> BTreeMap<usize, VoronoiCell3d> {
	let mut cells = BTreeMap::new();
	for (tet_ids, generating_point_id) in cell_tetrahedra.iter() {
		// lookup the circumcentre IDs of each tetrahedron
		let mut vertex_ids = vec![];
		for tet_id in tet_ids.iter() {
			if let Some(circum_id) = tetrahedron_to_circumcentre_ids.get(tet_id) {
				vertex_ids.push(*circum_id);
			}
		}
		//TODO need to find a way of linking the vertices

		let mut edges = BTreeSet::new();
		for this_tet_id in tet_ids.iter() {
			for other_tet_id in tet_ids.iter() {
				if this_tet_id != other_tet_id {
					// if two tetrahedra share a face then their circumcentres
					// form an edge
					let this_tet = tetras_store.get(this_tet_id).unwrap();
					let this_faces = this_tet.get_triangle_node_3d_faces();
					let other_tet = tetras_store.get(other_tet_id).unwrap();
					let other_faces = other_tet.get_triangle_node_3d_faces();
					for this_face in this_faces {
						if other_faces.contains(&this_face) {
							// shared face so store an edge of circumcentres
							let start = tetrahedron_to_circumcentre_ids.get(this_tet_id).unwrap();
							let end = tetrahedron_to_circumcentre_ids.get(other_tet_id).unwrap();
							let edge = EdgeNode3d::new(*start, *end);
							edges.insert(edge);
						}
					}
				}
			}
		}

		let cell = VoronoiCell3d {
			vertices: vertex_ids,
			edges: edges,
			generating_point: **generating_point_id,
		};
		let key = cells.len();
		cells.insert(key, cell);
	}
	cells
}
