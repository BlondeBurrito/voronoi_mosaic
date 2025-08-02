//! Visualise the vertices and edges of Voronoi Cells in 2d
//!
//! Vertices are shown as red circles while edge are shown as blue lines
//!

use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

/// Z location of Voronoi cell edes
const VORONOI_CELL_EDGE_Z: f32 = 1.0;
/// Colour of Voronoi edges
const VORONOI_EDGE_COLOUR: Color = Color::srgb(1.0, 0.5, 0.0);
/// Z location of the Voronoi vertices
const VORONOI_CELL_VERTEX_Z: f32 = 2.0;
/// Colour of the Voronoi vertices
const VORONOI_VERTEX_COLOUR: Color = Color::srgb(0.5, 1.0, 0.0);

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, (setup, visuals))
		.run();
}
/// Requirements
fn setup(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// camera
	cmds.spawn((Camera2d,));
	// background plane
	let mesh = meshes.add(Rectangle::from_length(400.0));
	let material = materials.add(Color::srgb(0.75, 0.75, 0.75));
	cmds.spawn((Transform::default(), Mesh2d(mesh), MeshMaterial2d(material)));
}
/// Compute voronoi and dispay it with simple shapes
fn visuals(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// points to be used
	let points = vec![
		Vec2::new(-190.0, 90.0),
		Vec2::new(-145.0, 120.0),
		Vec2::new(-120.0, -45.0),
		Vec2::new(-60.0, -120.0),
		Vec2::new(-20.0, 190.0),
		Vec2::new(60.0, -10.0),
		Vec2::new(80.0, -190.0),
		Vec2::new(100.0, 140.0),
		Vec2::new(190.0, -60.0),
	];
	// compute data
	if let Some(data) = DelaunayData::compute_triangulation_2d(&points) {
		if let Some(voronoi) = VoronoiData::from_delaunay_2d(&data) {
			// add simple shapes to showcase what the data looks like
			for cell in voronoi.get_cells().values() {
				for (i, point) in cell.get_vertices().iter().enumerate() {
					// mark each vertex of every cell
					let mesh = meshes.add(Circle::new(10.0));
					let material = materials.add(VORONOI_VERTEX_COLOUR);
					cmds.spawn((
						Mesh2d(mesh.clone()),
						MeshMaterial2d(material.clone()),
						Transform::from_translation(point.extend(VORONOI_CELL_VERTEX_Z)),
					));
					// mark the edges
					let (v1, v0) = if i < cell.get_vertices().len() - 1 {
						(cell.get_vertices()[i + 1], *point)
					} else {
						(cell.get_vertices()[0], *point)
					};
					let y_len = (v1 - v0).length();
					let mesh = meshes.add(Rectangle::from_size(Vec2::new(5.0, y_len)));
					let mat = materials.add(VORONOI_EDGE_COLOUR);
					let translation = (v1 + v0) / 2.0;
					let angle = Vec2::Y.angle_to(v0 - v1);
					let tform = Transform {
						translation: translation.extend(VORONOI_CELL_EDGE_Z),
						rotation: Quat::from_rotation_z(angle),
						..default()
					};
					// info!("edge {:?}", edge);
					// info!("midpoint {}", translation);
					// info!("angle {}", angle.to_degrees());
					cmds.spawn((Mesh2d(mesh), MeshMaterial2d(mat.clone()), tform));
				}
			}
		}
	} else {
		warn!("Data computation failed");
	}
}
