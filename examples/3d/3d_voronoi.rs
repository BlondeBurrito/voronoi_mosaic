//! Visualise the vertices and edges of the Voronoi Cells in 3d
//!
//! Vertices are shown as red spheres and edges as blue lines for illustration
//!

use std::f32::consts::PI;

use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

/// Colour of Voronoi edges
const VORONOI_EDGE_COLOUR: Color = Color::srgb(1.0, 0.5, 0.0);
/// Colour of the Voronoi vertices
const VORONOI_VERTEX_COLOUR: Color = Color::srgb(0.5, 1.0, 0.0);

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, (setup, visuals))
		.add_systems(Update, orbit_camera)
		.run();
}
/// Requirements
fn setup(
	mut cmds: Commands,
	// mut meshes: ResMut<Assets<Mesh>>,
	// mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// camera
	let mut cam_tform = Transform::from_translation(Vec3::new(300.0, 75.0, 300.0));
	cam_tform.look_at(Vec3::ZERO, Vec3::Y);
	cmds.spawn((Camera3d::default(), cam_tform));
	// // background plane
	// let mesh = meshes.add(Cuboid::new(20.0, 1.0, 20.0));
	// let material = materials.add(StandardMaterial {
	// 	base_color: Color::srgb(0.75, 0.75, 0.75),
	// 	..default()
	// });
	// cmds.spawn((
	// 	Transform::from_translation(Vec3::NEG_Y),
	// 	Mesh3d(mesh),
	// 	MeshMaterial3d(material),
	// ));
	// lighting
	cmds.spawn((
		DirectionalLight {
			illuminance: light_consts::lux::FULL_DAYLIGHT,
			shadows_enabled: true,
			..default()
		},
		Transform {
			translation: Vec3::new(0.0, 200.0, 0.0),
			rotation: Quat::from_rotation_x(-PI / 4.),
			..default()
		},
	));
}

/// Orbit the camera around the scene
fn orbit_camera(
	mut camera_q: Query<&mut Transform, With<Camera>>,
	time: Res<Time>,
	mut angle: Local<f32>,
) {
	for mut tform in &mut camera_q {
		let dt = time.delta_secs();
		let speed = 0.5;
		*angle += speed * dt;
		let x = 300.0 * angle.cos();
		let z = 300.0 * angle.sin();
		tform.translation.x = x;
		tform.translation.z = z;
		tform.look_at(Vec3::ZERO, Vec3::Y);
	}
}
/// Create simple shapes to visualise the Voronoi data
fn visuals(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// points to be used
	let points = vec![
		Vec3::new(-50.0, -50.0, -50.0),
		Vec3::new(-25.0, -50.0, -50.0),
		Vec3::new(0.0, -50.0, -50.0),
		Vec3::new(25.0, -50.0, -50.0),
		Vec3::new(50.0, -50.0, -50.0),
		Vec3::new(50.0, -50.0, -25.0),
		Vec3::new(50.0, -50.0, 0.0),
		Vec3::new(50.0, -50.0, 25.0),
		Vec3::new(50.0, -50.0, 50.0),
		Vec3::new(25.0, -50.0, 50.0),
		Vec3::new(0.0, -50.0, 50.0),
		Vec3::new(-25.0, -50.0, 50.0),
		Vec3::new(-50.0, -50.0, 50.0),
		Vec3::new(-50.0, 50.0, -50.0),
		Vec3::new(-25.0, 50.0, -50.0),
		Vec3::new(0.0, 50.0, -50.0),
		Vec3::new(25.0, 50.0, -50.0),
		Vec3::new(50.0, 50.0, -50.0),
		Vec3::new(50.0, 50.0, -25.0),
		Vec3::new(50.0, 50.0, 0.0),
		Vec3::new(50.0, 50.0, 25.0),
		Vec3::new(50.0, 50.0, 50.0),
		Vec3::new(25.0, 50.0, 50.0),
		Vec3::new(0.0, 50.0, 50.0),
		Vec3::new(-25.0, 50.0, 50.0),
		Vec3::new(-50.0, 50.0, 50.0),
		//
		Vec3::new(-50.0, -25.0, 50.0),
		Vec3::new(-50.0, 0.0, 50.0),
		Vec3::new(-50.0, 25.0, 50.0),
		//
		Vec3::new(-50.0, -25.0, -50.0),
		Vec3::new(-50.0, 0.0, -50.0),
		Vec3::new(-50.0, 25.0, -50.0),
		//
		Vec3::new(50.0, -25.0, 50.0),
		Vec3::new(50.0, 0.0, 50.0),
		Vec3::new(50.0, 5.0, 50.0),
		//
		Vec3::new(50.0, -25.0, -50.0),
		Vec3::new(50.0, 0.0, -50.0),
		Vec3::new(50.0, 25.0, -50.0),
		//
		Vec3::new(10.0, 19.0, 3.0),
		Vec3::new(32.0, 43.0, 17.0),
		Vec3::new(15.0, 9.0, 36.0),
		Vec3::new(43.0, 21.0, 41.0),
		//
		Vec3::new(-2.0, 43.0, 7.0),
		Vec3::new(-24.0, 9.0, 22.0),
		Vec3::new(-17.0, 27.0, 45.0),
		Vec3::new(-41.0, 36.0, 35.0),
		//
		Vec3::new(-38.0, 11.0, -12.0),
		Vec3::new(-48.0, 39.0, -26.0),
		Vec3::new(-12.0, 24.0, -31.0),
		Vec3::new(-24.0, 35.0, -44.0),
		//
		Vec3::new(10.0, 6.0, -43.0),
		Vec3::new(23.0, 15.0, -34.0),
		Vec3::new(36.0, 38.0, -26.0),
		Vec3::new(41.0, 45.0, -5.0),
		//
		Vec3::new(13.0, -43.0, 25.0),
		Vec3::new(29.0, -36.0, 19.0),
		Vec3::new(35.0, -24.0, 37.0),
		Vec3::new(46.0, -5.0, 14.0),
		//
		Vec3::new(-11.0, -15.0, 6.0),
		Vec3::new(-26.0, -23.0, 13.0),
		Vec3::new(-41.0, -29.0, 27.0),
		Vec3::new(-46.0, -35.0, 39.0),
		//
		Vec3::new(-45.0, -6.0, -15.0),
		Vec3::new(-36.0, -18.0, -24.0),
		Vec3::new(-28.0, -36.0, -30.0),
		Vec3::new(-10.0, -44.0, -40.0),
		//
		Vec3::new(19.0, -15.0, -25.0),
		Vec3::new(27.0, -30.0, -42.0),
		Vec3::new(39.0, -38.0, -12.0),
		Vec3::new(48.0, -45.0, -30.0),
	];
	// compute data
	if let Some(delaunay) = Delaunay3d::compute_triangulation_3d(&points) {
		if let Some(voronoi) = Voronoi3d::from_delaunay_3d(&delaunay) {
			// add simple shapes to showcase what the data looks like
			create_voronoi_cell_visuals(&mut cmds, &mut meshes, &mut materials, &voronoi);
		}
	} else {
		warn!("Data computation failed");
	}
}

/// Labels an entity in the Voronoi view for querying
#[derive(Component)]
struct VoronoiLabel;

/// Create simple shapes to illustrate the raw voronoi data
fn create_voronoi_cell_visuals(
	cmds: &mut Commands,
	mesh_assets: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	voronoi: &Voronoi3d,
) {
	let cells = voronoi.get_cells();
	let vertex_lookup = voronoi.get_vertex_lookup();
	for (_, cell) in cells.iter() {
		let cell_vertex_ids = cell.get_vertex_ids();
		for vertex_id in cell_vertex_ids.iter() {
			// mark each vertex of every cell
			let mesh = mesh_assets.add(Sphere::new(0.5));
			let material = materials.add(StandardMaterial {
				base_color: VORONOI_VERTEX_COLOUR,
				..default()
			});
			let pos = vertex_lookup.get(vertex_id).unwrap();
			cmds.spawn((
				Mesh3d(mesh.clone()),
				MeshMaterial3d(material.clone()),
				Transform::from_translation(*pos),
				Visibility::Visible,
				VoronoiLabel,
			));
		}
		// mark the edges
		let edges = cell.get_edges();
		for edge in edges.iter() {
			let start = vertex_lookup.get(&edge.get_vertex_a_id()).unwrap();
			let end = vertex_lookup.get(&edge.get_vertex_b_id()).unwrap();
			let len = (end - start).length();
			let mesh = mesh_assets.add(Cuboid::new(0.25, 0.25, len));
			let mat = materials.add(StandardMaterial {
				base_color: VORONOI_EDGE_COLOUR,
				..default()
			});
			let translation = (end + start) / 2.0;
			let mut tform = Transform::from_translation(translation);
			tform.look_at(*end, Vec3::Y);
			cmds.spawn((
				Mesh3d(mesh),
				MeshMaterial3d(mat.clone()),
				tform,
				Visibility::Visible,
				VoronoiLabel,
			));
		}
	}
}
