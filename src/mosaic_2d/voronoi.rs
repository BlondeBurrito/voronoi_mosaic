//! 2d Voronoi is built by identifying the circumcentres of a Delaunay
//! Triangulation and grouping those centres into a cell based on their shared
//! cell-site/generating point
//!
//!

use std::{
	cmp::Ordering,
	collections::{BTreeMap, BTreeSet},
};

use crate::{
	mosaic_2d::{delaunay::*, triangle_node2d::TriangleNode2d},
	prelude::{is_point_within_edge_range_limt, is_vertex_within_polygon, sort_vertices_2d},
};
use bevy::{
	asset::RenderAssetUsages,
	prelude::*,
	render::mesh::{Indices, PrimitiveTopology},
};

/// The vertices of a Voronoi Cell in 2-dimensions
#[derive(PartialEq)]
pub struct VoronoiCell2d {
	/// List of vertex ids that make up the cell
	vertices: Vec<usize>,
	/// The Delaunay vertex ID which is the nearest site to the boundary vertices of the
	/// cell compared to any other cell source
	generating_point: usize,
}

impl VoronoiCell2d {
	/// Get a reference to the list of vertices of this cell
	pub fn get_vertex_ids(&self) -> &Vec<usize> {
		&self.vertices
	}
	/// Get the delaunay vertex ID which is the nearest site to the vertices of the cell
	/// compared to any other cell source
	pub fn get_generating_point(&self) -> &usize {
		&self.generating_point
	}
	/// Get the midpoint between all vertices of the cell in real-space
	pub fn get_centre_position(&self, vertex_lookup: &BTreeMap<usize, Vec2>) -> Vec2 {
		let vertex_ids = self.get_vertex_ids();
		let mut sum = Vec2::ZERO;
		for id in vertex_ids.iter() {
			let point = vertex_lookup.get(id).unwrap();
			sum += point;
		}
		sum / vertex_ids.len() as f32
	}
	// /// Get a list of edges of the cell. Arranged in an anti-clockwise fashion
	// pub fn get_edges(&self) -> Vec<(Vec2, Vec2)> {
	// 	let mut edges = vec![];
	// 	for i in 0..self.get_vertices().len() {
	// 		if i < self.get_vertices().len() - 1 {
	// 			edges.push((self.get_vertices()[i], self.get_vertices()[i + 1]));
	// 		} else {
	// 			edges.push((self.get_vertices()[i], self.get_vertices()[0]));
	// 		}
	// 	}
	// 	edges
	// }
}

/// Describes the Voronoi cells
pub struct Voronoi2d {
	/// Uniquely ID'ed cells
	cells: BTreeMap<usize, VoronoiCell2d>,
	/// Each vertex of a cell is an ID corresponding to a point in space
	vertex_lookup: BTreeMap<usize, Vec2>,
}

impl Voronoi2d {
	/// Get a reference to the map of Voronoi Cells
	pub fn get_cells(&self) -> &BTreeMap<usize, VoronoiCell2d> {
		&self.cells
	}
	/// Get a mutable reference to the map of Voronoi Cells
	pub fn get_cells_mut(&mut self) -> &mut BTreeMap<usize, VoronoiCell2d> {
		&mut self.cells
	}
	/// Get a refernce to the map of vertex IDs and their position
	pub fn get_vertex_lookup(&self) -> &BTreeMap<usize, Vec2> {
		&self.vertex_lookup
	}
	/// Get a mutable refernce to the map of vertex IDs and their position
	pub fn get_vertex_lookup_mut(&mut self) -> &mut BTreeMap<usize, Vec2> {
		&mut self.vertex_lookup
	}
	/// Generate a map of [VoronoiCell2d] from a Delaunay Triangle without any boundary restrictions on the Cells
	pub fn from_delaunay_2d(delaunay: &Delaunay2d) -> Option<Self> {
		let triangle_store = delaunay.get_triangles();
		let delaunay_vertex_lookup = delaunay.get_vertex_lookup();

		// store IDs for all the cirumcentres
		let mut voronoi_vertex_lookup = BTreeMap::new();
		// store the triangle ID and what circumcentre ID is corresponds to
		let mut triangle_to_circumcentre_ids = BTreeMap::new();

		for (tri_id, triangle) in triangle_store.iter() {
			if let Some(circumcircle) = triangle.compute_circumcircle(delaunay_vertex_lookup) {
				let centre = circumcircle.get_centre();
				let voronoi_id = voronoi_vertex_lookup.len();
				voronoi_vertex_lookup.insert(voronoi_id, *centre);
				triangle_to_circumcentre_ids.insert(*tri_id, voronoi_id);
			}
		}

		// loop thorugh triangles and find cases where 3 or more triangles have
		// a vertex id in common, this means that the circumcentres of those
		// triangles are the voronoi vertices of a cell.
		// Keys are sets of triangle IDs, value is the ID of the generating point
		let cell_triangles = find_shared_sets(triangle_store);

		// convert the triangle groupings into voronoi vertices IDs
		let cells = compute_cells_from_triangle_sets(
			&cell_triangles,
			&voronoi_vertex_lookup,
			&triangle_to_circumcentre_ids,
		);

		Some(Voronoi2d {
			cells,
			vertex_lookup: voronoi_vertex_lookup,
		})
	}

	/// Convert each Voronoi Cell into a Bevy Mesh. These are for use in 2d with assumed normals of [Vec3::Z]
	pub fn as_bevy2d_meshes(&self) -> BTreeMap<usize, (Mesh, Vec2)> {
		let mut meshes = BTreeMap::new();
		let cells = self.get_cells();
		let vertex_lookup = self.get_vertex_lookup();
		for (id, cell) in cells.iter() {
			let cell_vertex_ids = cell.get_vertex_ids();
			// find the vertices in real-space
			let mut cell_vertices = vec![];
			for id in cell_vertex_ids.iter() {
				let point = vertex_lookup.get(id).unwrap();
				cell_vertices.push(*point);
			}

			// normalise vertices around origin 0,0
			let cell_vertices_normalised: Vec<Vec2> = cell_vertices
				.iter()
				.map(|v| v - cell.get_centre_position(vertex_lookup))
				.collect();

			if let Some(mesh) = triangulate_mesh(&cell_vertices_normalised) {
				let origin = cell.get_centre_position(vertex_lookup);
				meshes.insert(*id, (mesh, origin));
			}
		}
		meshes
	}

	/// Convert each Voronoi Cell into a Bevy Mesh that is clipped to a boundary polygon.
	///
	/// The boundary polygon must contain at least 3 vertices and the vertices
	/// should be expressed in an anti-clockwise order around their centre
	///
	/// *NB: Delaunay and Voronoi are duals - they can precisely be converted from one fomrat to the other back and forth. By applying clipping to the Voronoi, cell vertices may be added/removed which will destroy the duality - i.e if you apply clipping you cannot convert meshes into Delaunay and expect to get your oringal dataset back*
	pub fn as_clipped_bevy2d_meshes(&self, boundary: &[Vec2]) -> BTreeMap<usize, (Mesh, Vec2)> {
		//TODO sort the supplied boundary points or trust user input?
		let mut meshes = BTreeMap::new();
		let cells = self.get_cells();
		let vertex_lookup = self.get_vertex_lookup();
		for (id, cell) in cells.iter() {
			let cell_vertex_ids = cell.get_vertex_ids();
			// find the vertices in real-space
			let mut cell_vertices = vec![];
			for id in cell_vertex_ids.iter() {
				let point = vertex_lookup.get(id).unwrap();
				cell_vertices.push(*point);
			}

			if let Some(clipped_vertices) = clip_vertices_to_boundary(cell_vertices, boundary) {
				// normalise vertices around origin 0,0
				let cell_vertices_normalised: Vec<Vec2> = clipped_vertices
					.iter()
					.map(|v| v - cell.get_centre_position(vertex_lookup))
					.collect();

				if let Some(mesh) = triangulate_mesh(&cell_vertices_normalised) {
					let origin = cell.get_centre_position(vertex_lookup);
					meshes.insert(*id, (mesh, origin));
				}
			}
		}
		meshes
	}
}

/// Compare the vertices of triangles and identify groupings of IDs whereby 3
/// or more triangles share a vertex.
///
/// The grouping forms the key and the value is the Delaunay vertex ID they all
/// have in common - i.e the generating point of a Voronoi cell of the
/// triangles
fn find_shared_sets(
	triangle_store: &BTreeMap<usize, TriangleNode2d>,
) -> BTreeMap<BTreeSet<&usize>, &usize> {
	let mut cell_triangles = BTreeMap::new();
	for (this_tri_id, this_tri) in triangle_store.iter() {
		// loop through all vertex IDs
		for this_vert_id in this_tri.get_vertex_ids() {
			let mut shared_tri_ids = BTreeSet::from([this_tri_id]);
			// loop over other triangles
			for (other_tri_id, other_tri) in triangle_store.iter() {
				if this_tri_id != other_tri_id && other_tri.get_vertex_ids().contains(this_vert_id)
				{
					// triangles share a common vertex ID, store other
					shared_tri_ids.insert(other_tri_id);
				}
			}
			if shared_tri_ids.len() >= 3 {
				// we have found a series of triangles with a common vertex,
				// their circumcentres are voronoi vertices
				cell_triangles.insert(shared_tri_ids, this_vert_id);
			}
		}
	}
	cell_triangles
}
/// From triangle groupings calculate each [VoronoiCell2d] from their
/// circumcentres
fn compute_cells_from_triangle_sets(
	cell_triangles: &BTreeMap<BTreeSet<&usize>, &usize>,
	voronoi_vertex_lookup: &BTreeMap<usize, Vec2>,
	triangle_to_circumcentre_ids: &BTreeMap<usize, usize>,
) -> BTreeMap<usize, VoronoiCell2d> {
	let mut cells = BTreeMap::new();
	for (tri_ids, generating_point_id) in cell_triangles.iter() {
		// lookup the circumcentre IDs of each triangle
		let mut vertex_ids = vec![];
		for tri_id in tri_ids.iter() {
			if let Some(circum_id) = triangle_to_circumcentre_ids.get(tri_id) {
				vertex_ids.push(*circum_id);
			}
		}
		// order the vertex ids in an anti-clockwise fashion about their centre
		let midpoint = {
			let mut sum = Vec2::ZERO;
			for id in vertex_ids.iter() {
				sum += voronoi_vertex_lookup.get(id).unwrap();
			}
			sum / vertex_ids.len() as f32
		};
		vertex_ids.sort_by(|a, b| {
			let a_pos = voronoi_vertex_lookup.get(a).unwrap();
			let b_pos = voronoi_vertex_lookup.get(b).unwrap();
			if let Some(ordering) = Vec2::Y
				.angle_to(*a_pos - midpoint)
				.partial_cmp(&Vec2::Y.angle_to(*b_pos - midpoint))
			{
				ordering
			} else {
				warn!("Unable to find Ordering between {} and {}", a, b);
				Ordering::Less
			}
		});
		let key = cells.len();
		let cell = VoronoiCell2d {
			vertices: vertex_ids,
			generating_point: **generating_point_id,
		};
		cells.insert(key, cell);
	}
	cells
}

/// To create a mesh we need a series of triangles describing the mesh.
/// By applying Delaunay to the vertices of a cell we can
/// triangulate the triangles that make up the mesh
fn triangulate_mesh(offset_cell_vertices: &Vec<Vec2>) -> Option<Mesh> {
	if let Some(delaunay) = Delaunay2d::compute_triangulation_2d(offset_cell_vertices) {
		let delaunay_triangles = delaunay.get_triangles();
		let delaunay_vertex_lookup = delaunay.get_vertex_lookup();

		// store all the vertices of the mesh
		let positions: Vec<Vec3> = offset_cell_vertices.iter().map(|v| v.extend(0.0)).collect();
		let normals = vec![Vec3::Z; positions.len()];
		let uvs = compute_mesh_uvs(&positions);

		//TODO tests to ensure right number of indices/postions
		//TODO verify no "hole" in mesh
		// for each triangle lookup the index in `positions` of each vertex
		let mut indices = vec![];
		for (_, tri) in delaunay_triangles.iter() {
			let ids = tri.get_vertex_ids();
			// indices are in groupings of 3
			for id in ids.iter() {
				let tri_ver = delaunay_vertex_lookup.get(id).unwrap();
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

		Some(mesh)
	} else {
		warn!("Cannot compute triangulation for cell mesh");
		None
	}
}

/// Each vertex of a mesh requires a UV coordinate. A UV coordinate describes
/// the texture mapping of a surface. UVs range from `[0, 0]` to `[1, 1]` with
/// the origin being located in the bottom left corner of the surface and the
/// maximum a the top right
fn compute_mesh_uvs(vertices: &[Vec3]) -> Vec<Vec2> {
	// find min-max x-y of vertices to allow them to be normalised in range of [0,0] [1, 1]
	let mut min = Vec2::ZERO;
	let mut max = Vec2::ZERO;
	for v in vertices {
		if v.x < min.x {
			min.x = v.x;
		}
		if v.y < min.y {
			min.y = v.y;
		}
		if v.x > max.x {
			max.x = v.x;
		}
		if v.y > max.y {
			max.y = v.y;
		}
	}
	let mut uvs = vec![];
	for v in vertices {
		// transpose v coordinate to uv coordinate space
		let transposed = v.truncate() - min;
		if max - min != Vec2::ZERO {
			// normalise transposed based on coord range to bring
			// it in between 0 and 1
			let uv = transposed / (max - min);
			uvs.push(uv);
		} else {
			// safety
			uvs.push(Vec2::ONE);
		}
	}
	uvs
}

/// Restrain a series of vertices to a boundary if possible and required.
///
/// If the vertices all exist outside of the boundary then `None` is returned.
/// If all vertices sit within the boundary then they are returned unaltered
/// If some vertices sit inside the boundary and some outside then they will be
/// modified to ensure that they all sit within the boundary
///
/// The supplied `cell_vertices` must be ordered in an anti-clockwise fashion
fn clip_vertices_to_boundary(cell_vertices: Vec<Vec2>, boundary: &[Vec2]) -> Option<Vec<Vec2>> {
	// form the edges of the bounding polygon
	let mut bounding_edges = vec![];
	for i in 0..boundary.len() {
		if i < boundary.len() - 1 {
			bounding_edges.push((boundary[i], boundary[i + 1]));
		} else {
			bounding_edges.push((boundary[i], boundary[0]));
		}
	}

	// count the number of cell vertices that lie outside of the boundary
	let mut outside_point_count = 0;
	for vertex in cell_vertices.iter() {
		if !is_vertex_within_polygon(vertex, &bounding_edges) {
			outside_point_count += 1;
		}
	}

	if outside_point_count == cell_vertices.len() {
		// if all vertices are outside of the boundary then we ignore them
		None
	} else if outside_point_count == 0 {
		// all vertices are inside the boundary we don't need to do anything
		Some(cell_vertices)
	} else {
		// create edges of the cell vertices
		let mut existing_edges = vec![];
		for i in 0..cell_vertices.len() {
			if i < cell_vertices.len() - 1 {
				existing_edges.push((cell_vertices[i], cell_vertices[i + 1]));
			} else {
				existing_edges.push((cell_vertices[i], cell_vertices[0]));
			}
		}

		// store new vertices from intersections with boundary
		let mut new_vertices = vec![];

		// if any boundary vert lies within the cell then they
		// will be new verts the cell gets clipped to
		for bounding_edge in bounding_edges.iter() {
			if is_vertex_within_polygon(&bounding_edge.0, &existing_edges)
				&& !new_vertices.contains(&bounding_edge.0)
			{
				new_vertices.push(bounding_edge.0);
			}
			if is_vertex_within_polygon(&bounding_edge.1, &existing_edges)
				&& !new_vertices.contains(&bounding_edge.1)
			{
				new_vertices.push(bounding_edge.1);
			}
		}

		// walk around the cell finding which boundary edges it passes through
		// to find clipping intersections
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
						let intersect_y = bounding_gradient * intersect_x + bounding_intercept;
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
					let cell_intercept = cell_edge_start.y - (cell_gradient * cell_edge_start.x);
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
				} else if (cell_edge_dy / cell_edge_dx) == (bounding_edge_dy / bounding_edge_dx) {
					// handle case of both edges being parallel
					//TODO cell edge verts should already be treated
					//TODO as being within the polygon then?
					None
				} else {
					let bounding_gradient = bounding_edge_dy / bounding_edge_dx;
					let bounding_intercept =
						bounding_edge.0.y - (bounding_gradient * bounding_edge.0.x);
					let cell_gradient = cell_edge_dy / cell_edge_dx;
					let cell_intercept = cell_edge_start.y - (cell_gradient * cell_edge_start.x);
					let intersect_x =
						(bounding_intercept - cell_intercept) / (cell_gradient - bounding_gradient);
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

				if let Some(point) = intersection
					&& !new_vertices.contains(&point)
				{
					new_vertices.push(point);
				}
			}
		}
		// add any original vertices that are inside the boundary to new_vertices
		for vertex in cell_vertices.iter() {
			if is_vertex_within_polygon(vertex, &bounding_edges) && !new_vertices.contains(vertex) {
				new_vertices.push(*vertex);
			}
		}
		// sort the vertices anti-clockwise
		let midpoint = {
			let mut sum = Vec2::ZERO;
			for nv in new_vertices.iter() {
				sum += nv;
			}
			sum / new_vertices.len() as f32
		};
		sort_vertices_2d(&mut new_vertices, &midpoint);
		Some(new_vertices)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	// #[test]
	// fn cell_edges() {
	// 	let cell = VoronoiCell2d {
	// 		vertices: vec![
	// 			Vec2::new(1.0, 0.0),
	// 			Vec2::new(0.0, 1.0),
	// 			Vec2::new(-1.0, 0.5),
	// 			Vec2::new(-1.0, 0.0),
	// 		],
	// 		generating_point: Vec2::new(0.0, 0.5),
	// 	};
	// 	let actual: Vec<(Vec2, Vec2)> = vec![
	// 		(Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)),
	// 		(Vec2::new(0.0, 1.0), Vec2::new(-1.0, 0.5)),
	// 		(Vec2::new(-1.0, 0.5), Vec2::new(-1.0, 0.0)),
	// 		(Vec2::new(-1.0, 0.0), Vec2::new(1.0, 0.0)),
	// 	];
	// 	assert_eq!(actual, cell.get_edges());
	// }
	// #[test]
	// fn shared_generating_point() {
	// 	let t1 = triangle_2d::Triangle2d::new(vec2(0.0, 0.0), vec2(5.0, 0.0), vec2(0.0, 5.0));
	// 	let t2 = triangle_2d::Triangle2d::new(vec2(0.0, 0.0), vec2(0.0, 5.0), vec2(-5.0, 0.0));
	// 	let t3 = triangle_2d::Triangle2d::new(vec2(0.0, 0.0), vec2(-5.0, 0.0), vec2(0.0, -5.0));
	// 	let t4 = triangle_2d::Triangle2d::new(vec2(0.0, 0.0), vec2(0.0, -5.0), vec2(5.0, 0.0));
	// 	let map: BTreeMap<usize, &triangle_2d::Triangle2d> =
	// 		BTreeMap::from([(0, &t1), (1, &t2), (2, &t3), (3, &t4)]);
	// 	let shared_sets = find_shared_sets(&map);
	// 	assert!(shared_sets.len() == 1);
	// 	let (shared_ids, generating_point) = shared_sets.first_key_value().unwrap();
	// 	let actual_shared_ids = vec![0, 1, 2, 3];
	// 	assert!(actual_shared_ids == *shared_ids);
	// 	assert!(Vec2::ZERO == *generating_point);
	// }
	#[test]
	fn mesh_uvs() {
		let vertices = vec![
			Vec3::new(-5.0, -5.0, 0.0),
			Vec3::new(5.0, -5.0, 0.0),
			Vec3::new(5.0, 5.0, 0.0),
			Vec3::new(-5.0, 5.0, 0.0),
		];
		let actual = vec![
			Vec2::new(0.0, 0.0),
			Vec2::new(1.0, 0.0),
			Vec2::new(1.0, 1.0),
			Vec2::new(0.0, 1.0),
		];
		assert_eq!(actual, compute_mesh_uvs(&vertices));
	}
}
