//! TODO
//!
//!

use std::collections::HashMap;

use crate::{
	prelude::{DelaunayData, sort_vertices_2d},
	triangle_2d,
};
use bevy::{
	asset::RenderAssetUsages,
	prelude::*,
	render::mesh::{Indices, PrimitiveTopology},
};

use super::VoronoiData;

/// The vertices of a Voronoi Cell in 2-dimensions
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
}

impl VoronoiData<VoronoiCell2d> {
	/// Get a reference to the list of Voronoi Cells
	pub fn get_cells(&self) -> &Vec<VoronoiCell2d> {
		&self.cells
	}
	/// From a Delaunay Triangulation compute its dual - the Voronoi Cells
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
		let mut cells = vec![];
		for (ids, common_vertex) in id_sets.iter() {
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
			cells.push(VoronoiCell2d {
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
		for cell in cells.iter() {
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
	// /// Froma series of 2d points in space compute the Voronoi Cells directly
	// pub fn compute_cells_2d(points: &Vec<Vec2>) -> Option<Self> {
	// 	for point in points.iter() {

	// 	}


	// 	Some(VoronoiData {
	// 		cells
	// 	})
	// }
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
