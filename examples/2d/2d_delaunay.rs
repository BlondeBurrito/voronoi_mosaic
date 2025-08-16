//! Visualise the vertices and edges of the Delaunay Triangulation in 2d
//!
//! Vertices are shown as circles while edge are shown as lines
//!

use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

/// Z location of Delaunay edges
const DELAUNAY_EDGE_Z: f32 = 1.0;
/// Colour of Delaunay edges
const DELAUNAY_EDGE_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);
/// Z location of Delaunay vertices
const DELAUNAY_VERTEX_Z: f32 = 2.0;
/// Colour of Delaunay vertices
const DELAUNAY_VERTEX_COLOUR: Color = Color::srgb(0.0, 0.0, 1.0);

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
/// Compute triangluation and dispay it with simple shapes
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
	// let points = vec![
	// 		Vec2::new(50.0, 0.0),
	// 		Vec2::new(-50.0, 0.0),
	// 		Vec2::new(0.0, 50.0),
	// 	];
	// compute data
	// if let Some(data) = DelaunayData::compute_triangulation_2d(&points) {
	// 	create_delaunay_visuals(&mut cmds, &mut meshes, &mut materials, &data);
	// } else {
	// 	warn!("Data computation failed");
	// }
	if let Some(delaunay) = Delaunay2d::compute_triangulation_2d(&points) {
		create_delaunay_visuals(&mut cmds, &mut meshes, &mut materials, &delaunay);
	} else {
		warn!("Delaunay computation failed");
	}
}

/// Labels an entity in the Delaunay view for querying
#[derive(Component)]
struct DelaunayLabel;

/// Create simple shapes to illustrate the raw delaunay data
fn create_delaunay_visuals(
	cmds: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	data: &Delaunay2d,
) {
	let vertex_lookup = data.get_vertex_lookup();
	for (_tri_id, triangle) in data.get_triangles().iter() {
		// create markers for vertices
		let mesh = meshes.add(Circle::new(10.0));
		let material = materials.add(DELAUNAY_VERTEX_COLOUR);
		// vertices
		let translations = [
			vertex_lookup.get(&triangle.get_vertex_a_id()).unwrap(),
			vertex_lookup.get(&triangle.get_vertex_b_id()).unwrap(),
			vertex_lookup.get(&triangle.get_vertex_c_id()).unwrap(),
		];
		for translation in translations.iter() {
			cmds.spawn((
				Mesh2d(mesh.clone()),
				MeshMaterial2d(material.clone()),
				Transform::from_translation(translation.extend(DELAUNAY_VERTEX_Z)),
				DelaunayLabel,
				Visibility::Visible,
			));
		}
		// create markers for edges
		let mat = materials.add(DELAUNAY_EDGE_COLOUR);
		for edge in triangle.get_edges().iter() {
			let start = vertex_lookup.get(&edge.get_vertex_a_id()).unwrap();
			let end = vertex_lookup.get(&edge.get_vertex_b_id()).unwrap();
			let y_len = (end - start).length();
			let mesh = meshes.add(Rectangle::from_size(Vec2::new(5.0, y_len)));
			let translation = (end + start) / 2.0;
			let angle = Vec2::Y.angle_to(start - end);
			let tform = Transform {
				translation: translation.extend(DELAUNAY_EDGE_Z),
				rotation: Quat::from_rotation_z(angle),
				..default()
			};
			cmds.spawn((
				Mesh2d(mesh),
				MeshMaterial2d(mat.clone()),
				tform,
				DelaunayLabel,
				Visibility::Visible,
			));
		}
	}
}
