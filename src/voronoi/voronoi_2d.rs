//! TODO
//!
//!

use std::collections::HashMap;

use crate::{
	prelude::{is_vertex_within_polygon, sort_vertices_2d, DelaunayData},
	triangle_2d,
};
use bevy::{
	asset::RenderAssetUsages,
	prelude::*,
	render::mesh::{Indices, PrimitiveTopology},
};

use super::VoronoiData;

/// The vertices of a Voronoi Cell in 2-dimensions
#[derive(PartialEq)]
pub struct VoronoiCell2d {
	vertices: Vec<Vec2>,
	/// The vertex which is the nearest site to the boundary vertices of the
	/// cell compared to any other cell source
	source_vertex: Vec2,
}

impl VoronoiCell2d {
	/// Get a reference to the list of vertices of this cell
	pub fn get_vertices(&self) -> &Vec<Vec2> {
		&self.vertices
	}
	/// Get the vertex which is the nearest site to the vertices of the cell
	/// compared to any other cell source
	pub fn get_source_vertex(&self) -> &Vec2 {
		&self.source_vertex
	}
	/// Get the midpoint between all vertices of the cell
	pub fn get_centre_position(&self) -> Vec2 {
		self.get_vertices().iter().sum::<Vec2>() / self.get_vertices().len() as f32
	}
	/// Get a list of edges of the cell. Arranged in an anti-clockwise fashion
	pub fn get_edges(&self) -> Vec<(Vec2, Vec2)> {
		let mut edges = vec![];
		for i in 0..self.get_vertices().len() {
			if i < self.get_vertices().len() - 1 {
				edges.push((self.get_vertices()[i], self.get_vertices()[i + 1]));
			} else {
				edges.push((self.get_vertices()[i], self.get_vertices()[0]));
			}
		}
		edges
	}
}

impl VoronoiData<VoronoiCell2d> {
	/// Get a reference to the map of Voronoi Cells
	pub fn get_cells(&self) -> &HashMap<u32, VoronoiCell2d> {
		&self.cells
	}
	/// Generate a set of [VoronoiCell2d] from a Delaunay Triangle without any boundary restrictions on the Cells
	pub fn from_delaunay_2d(
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
		let mut cells = HashMap::new();
		for (i, (ids, common_vertex)) in id_sets.iter().enumerate() {
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
			sort_vertices_2d(&mut cell_vertices, &midpoint);
			cells.insert(i as u32, VoronoiCell2d {
				vertices: cell_vertices,
				source_vertex: *common_vertex,
			});
		}

		// VoronoiCell2d(cells)
		Some(VoronoiData { cells })
	}
	/// Convert each Voronoi Cell into a Bevy Mesh. These are for use in 2d with assumed normals of [Vec3::Z]
	pub fn as_bevy_meshes_2d(&self) -> Vec<(Mesh, Vec2)> {
		let mut meshes = vec![];
		let cells = self.get_cells();
		for (_, cell) in cells.iter() {
			let cell_vertices = cell.get_vertices();
			// normalise vertices around origin 0,0
			let cell_vertices_normalised: Vec<Vec2> = cell_vertices
				.iter()
				.map(|v| v - cell.get_centre_position())
				.collect();

			// to create a mesh we need a series of triangles describing the mesh.
			// by applying Delaunay to the vertices of the cell we can
			// triangulate the triangles that make up the mesh
			if let Some(delaunay) =
				DelaunayData::compute_triangulation_2d(&cell_vertices_normalised)
			{
				let triangles = delaunay.get();

				// store all the vertices of the mesh
				let positions: Vec<Vec3> = cell_vertices_normalised
					.iter()
					.map(|v| v.extend(0.0))
					.collect();
				let normals = vec![Vec3::Z; positions.len()];
				//TODO compute UVs properly
				let uvs = vec![Vec2::Y; positions.len()];

				//TODO tests to ensure right number of indices/postions
				//TODO verify no "hole" in mesh
				// for each triangle lookup the index in `positions` of each vertex
				let mut indices = vec![];
				for tri in triangles.iter() {
					let tri_vertices = tri.get_vertices();
					// indices are in groupings of 3
					for tri_ver in tri_vertices.iter() {
						// find the index in positions of this vertex
						for (i, p) in positions.iter().enumerate() {
							if tri_ver.extend(0.0) == *p {
								indices.push(i as u32);
							}
						}
					}
				}

				let mesh = Mesh::new(
					PrimitiveTopology::TriangleList,
					RenderAssetUsages::default(),
				)
				.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
				.with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
				.with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
				.with_inserted_indices(Indices::U32(indices));

				meshes.push((mesh, cell.get_centre_position()));
			} else {
				warn!("Cannot compute triangulation for cell mesh");
			}
		}

		meshes
	}
	/// Clip all the [VoronoiCell2d] so they cannot extend or exists outside of
	/// a boundary polygon
	/// 
	/// The boundary polygon must contain at least 3 vertices and the vertice
	/// should be expressed in an anti-clockwise order around their centre
	/// 
	/// *NB: Delaunay and Voronoi are duals - they can precisely be converted from one fomrat to the other back and forth. By applying clipping to the Voronoi, cell vertices may be added/removed which will destroy the duality - i.e if you apply clipping you cannot convert Voronoi into Delaunay and expect to get your oringal dataset back*
	pub fn clip_cells_to_boundary(&mut self, boundary: &[Vec2]) {
		//TODO sort the supplied boundary points or trust user input?
		// form the edges of the bounding polygon
		let mut bounding_edges = vec![];
		for i in 0..boundary.len() {
			if i < boundary.len() - 1 {
				bounding_edges.push((boundary[i], boundary[i + 1]));
			} else {
				bounding_edges.push((boundary[i], boundary[0]));
			}
		}
		// store cells that exist entirely outside of the polygon
		let mut outside_cell_ids: Vec<u32> = vec![];
		// store cells are are aprtially inside and partially out
		let mut partial = vec![];
		for (id, cell) in self.get_cells() {
			// count the number of cell vertices that lie outside of the boundary
			let mut outside_point_count = 0;
			for cell_v in cell.get_vertices().iter() {
				// check if cell vertex is within the bounds
				if !is_vertex_within_polygon(cell_v, &bounding_edges) {
					outside_point_count += 1;
				}
			}
			// if all vertices are outside of the boundary then store the cell ID so it can be dropped from the map later
			if outside_point_count == cell.get_vertices().len() {
				outside_cell_ids.push(*id);
			} else if outside_point_count > 0 {
				// some cell vertices are inside the boundary
				// and some are outside
				partial.push(*id);
			}
		}
		// clip partial cells to the boundary
		//TODO
		for id in partial {
			if let Some(cell) = self.get_cells().get(&id) {
				
			}
		}
		// remove cells that exists completely outside of the boundary
		for id in outside_cell_ids.iter() {
			let _ = self.cells.remove(id);
		}
	}
}

/// Compare the vertices of triangles and identify groupings of IDs whereby 3
/// or more triangles share a vertex.
///
/// The grouping forms the key and the value is the vertex they all have in common
fn find_shared_sets(map: &HashMap<usize, &triangle_2d::Triangle2d>) -> HashMap<Vec<usize>, Vec2> {
	let mut set = HashMap::new();
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
				set.insert(ids, *vert);
			}
		}
	}
	set
}
