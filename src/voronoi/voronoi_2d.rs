//! TODO
//!
//!

use std::collections::BTreeMap;

use crate::{
	prelude::{
		DelaunayData, is_point_within_edge_range_limt, is_vertex_within_polygon, sort_vertices_2d,
	},
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
	/// List of vertices that make up the cell
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
	pub fn get_cells(&self) -> &BTreeMap<u32, VoronoiCell2d> {
		&self.cells
	}
	/// Get a mutable reference to the map of Voronoi Cells
	pub fn get_cells_mut(&mut self) -> &mut BTreeMap<u32, VoronoiCell2d> {
		&mut self.cells
	}
	/// Generate a set of [VoronoiCell2d] from a Delaunay Triangle without any boundary restrictions on the Cells
	pub fn from_delaunay_2d(delaunay: &DelaunayData<triangle_2d::Triangle2d>) -> Option<Self> {
		// each circumcentre of a Delaunay triangle is a vertex of a Voronoi cell
		let triangles = delaunay.get();

		// uniquely identify each triangle
		let mut triangle_store: BTreeMap<usize, &triangle_2d::Triangle2d> = BTreeMap::new();
		for (i, triangle) in triangles.iter().enumerate() {
			triangle_store.insert(i, triangle);
		}

		// store each set of triagnle IDs that together form a voronoi cell
		// if a vertex is shared 3+ times then all the circumcentres of traingles that use it
		// are voronoi vertices
		let id_sets = find_shared_sets(&triangle_store);

		// from the set of IDs find each circumcircle as a vertex on a voronoi cell
		let mut cells = BTreeMap::new();
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
			cells.insert(
				i as u32,
				VoronoiCell2d {
					vertices: cell_vertices,
					source_vertex: *common_vertex,
				},
			);
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
	/// Clip all the [VoronoiCell2d] so they cannot extend or exist outside of
	/// a boundary polygon
	///
	/// The boundary polygon must contain at least 3 vertices and the vertices
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
		for id in partial {
			if let Some(cell) = self.get_cells_mut().get_mut(&id) {
				// store new vertices from intersections with boundary
				let mut new_vertices = vec![];

				// if any boundary vert lies within the cell then they
				// will be new verts the cell gets clipped to
				for bounding_edge in bounding_edges.iter() {
					if is_vertex_within_polygon(&bounding_edge.0, &cell.get_edges())
						&& !new_vertices.contains(&bounding_edge.0)
					{
						new_vertices.push(bounding_edge.0);
					}
					if is_vertex_within_polygon(&bounding_edge.1, &cell.get_edges())
						&& !new_vertices.contains(&bounding_edge.1)
					{
						new_vertices.push(bounding_edge.1);
					}
				}

				// walk around the cell finding which boundary edges it passes through
				// to find clipping intersections
				let cell_vertices = cell.get_vertices();
				for i in 0..cell_vertices.len() {
					// store cell edge start and end
					let (cell_edge_start, cell_edge_end) = {
						if i < cell_vertices.len() - 1 {
							(cell_vertices[i], cell_vertices[i + 1])
						} else {
							(cell_vertices[0], cell_vertices[i])
						}
					};
					// only process it if one of the cell edge vertices is in the boundary and one outside
					if (is_vertex_within_polygon(&cell_edge_start, &bounding_edges)
						&& is_vertex_within_polygon(&cell_edge_end, &bounding_edges))
						|| (!is_vertex_within_polygon(&cell_edge_start, &bounding_edges)
							&& !is_vertex_within_polygon(&cell_edge_end, &bounding_edges))
					{
						continue;
					}
					let cell_edge_dy = cell_edge_end.y - cell_edge_start.y;
					let cell_edge_dx = cell_edge_end.x - cell_edge_start.x;
					// see if the cell edge crosses a boundary
					// when a crossing occurs store an index into the bounding_edges
					for bounding_edge in bounding_edges.iter() {
						let bounding_edge_dy = bounding_edge.1.y - bounding_edge.0.y;
						let bounding_edge_dx = bounding_edge.1.x - bounding_edge.0.x;

						let intersection = if cell_edge_dx == 0.0 {
							// handle vertical line
							if bounding_edge_dx == 0.0 {
								// boundary vertical too,
								// if edge x's are different then they are
								// parallel but don't overlap
								if bounding_edge.0.x == cell_edge_start.x {
									None
								} else {
									// they overlap but the cell edge vertices
									// are already inside the polygon then
									//TODO dont need to push then?
									None
								}
							} else {
								// cell edge is vert, boundary at an angle
								let bounding_gradient = bounding_edge_dy / bounding_edge_dx;
								let bounding_intercept =
									bounding_edge.0.y - (bounding_gradient * bounding_edge.0.x);
								// plug cell x to find y
								let intersect_x = cell_edge_start.x;
								let intersect_y =
									bounding_gradient * intersect_x + bounding_intercept;
								// ensure intersection is on the line and not beyond it
								if is_point_within_edge_range_limt(
									&Vec2::new(intersect_x, intersect_y),
									&cell_edge_start,
									&cell_edge_end,
								) {
									// store the intersection
									Some(Vec2::new(intersect_x, intersect_y))
								} else {
									None
								}
							}
						} else if bounding_edge_dx == 0.0 {
							// handle vertical boundary
							// bounding x const so find new y with cell edge
							// y = mx + c with subbing in boundary x
							let cell_gradient = cell_edge_dy / cell_edge_dx;
							let cell_intercept =
								cell_edge_start.y - (cell_gradient * cell_edge_start.x);
							let intersect_x = bounding_edge.1.x;
							let intersect_y = cell_gradient * intersect_x + cell_intercept;
							// ensure intersection is on the line and not beyond it
							if is_point_within_edge_range_limt(
								&Vec2::new(intersect_x, intersect_y),
								&cell_edge_start,
								&cell_edge_end,
							) {
								// store the intersection
								Some(Vec2::new(intersect_x, intersect_y))
							} else {
								None
							}
						} else if (cell_edge_dy / cell_edge_dx)
							== (bounding_edge_dy / bounding_edge_dx)
						{
							// handle case of both edges being parallel
							//TODO cell edge verts should already be treated
							//TODO as being within the polygon then?
							None
						} else {
							let bounding_gradient = bounding_edge_dy / bounding_edge_dx;
							let bounding_intercept =
								bounding_edge.0.y - (bounding_gradient * bounding_edge.0.x);
							let cell_gradient = cell_edge_dy / cell_edge_dx;
							let cell_intercept =
								cell_edge_start.y - (cell_gradient * cell_edge_start.x);
							let intersect_x = (bounding_intercept - cell_intercept)
								/ (cell_gradient - bounding_gradient);
							let intersect_y = (cell_gradient * intersect_x) + cell_intercept;
							// ensure intersection if on the line and not beyond it
							if is_point_within_edge_range_limt(
								&Vec2::new(intersect_x, intersect_y),
								&cell_edge_start,
								&cell_edge_end,
							) {
								// store the intersection
								Some(Vec2::new(intersect_x, intersect_y))
							} else {
								None
							}
						};

						if let Some(point) = intersection {
							if !new_vertices.contains(&point) {
								new_vertices.push(point);
							}
						}
					}
				}
				// add any original vertices that are inside the boundary to new_vertices
				for vertex in cell.get_vertices().iter() {
					if is_vertex_within_polygon(vertex, &bounding_edges)
						&& !new_vertices.contains(vertex)
					{
						new_vertices.push(*vertex);
					}
				}
				// replace the cell vertices with the new ones
				cell.vertices = new_vertices;
				// sort the vertices anti-clockwise
				let midpoint = cell.get_centre_position();
				sort_vertices_2d(&mut cell.vertices, &midpoint);
				// replace the cell source so it cannot possibly be outside the boundary
				//TODO don't think this is actually needed?
				cell.source_vertex = midpoint;
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
fn find_shared_sets(map: &BTreeMap<usize, &triangle_2d::Triangle2d>) -> BTreeMap<Vec<usize>, Vec2> {
	let mut set = BTreeMap::new();
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
