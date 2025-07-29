//! Visualise the vertices and edges of the Delaunay Triangulation in 2d
//!
//! Vertices are shown as circles while edge are shown as lines
//!

use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

const VERTEX_Z: f32 = 2.0;
const EDGE_Z: f32 = 1.0;

const DELAUNAY_EDGE_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);
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
	if let Some(data) = DelaunayData::compute_triangulation_2d(&points) {
		for triangle in data.get().iter() {
			// create markers for vertices
			let mesh = meshes.add(Circle::new(10.0));
			let material = materials.add(DELAUNAY_VERTEX_COLOUR);
			// vertices
			let translations = [
				triangle.get_vertex_a(),
				triangle.get_vertex_b(),
				triangle.get_vertex_c(),
			];
			for translation in translations.iter() {
				cmds.spawn((
					Mesh2d(mesh.clone()),
					MeshMaterial2d(material.clone()),
					Transform::from_translation(translation.extend(VERTEX_Z)),
				));
			}
			// create markers for edges
			let mat = materials.add(DELAUNAY_EDGE_COLOUR);
			for edge in triangle.get_edges().iter() {
				let y_len = (edge.1 - edge.0).length();
				let mesh = meshes.add(Rectangle::from_size(Vec2::new(5.0, y_len)));
				let translation = (edge.1 + edge.0) / 2.0;
				let angle = Vec2::Y.angle_to(edge.0 - edge.1);
				let tform = Transform {
					translation: translation.extend(EDGE_Z),
					rotation: Quat::from_rotation_z(angle),
					..default()
				};
				// info!("edge {:?}", edge);
				// info!("midpoint {}", translation);
				// info!("angle {}", angle.to_degrees());
				cmds.spawn((Mesh2d(mesh), MeshMaterial2d(mat.clone()), tform));
			}
		}
	} else {
		warn!("Data computation failed");
	}
}
