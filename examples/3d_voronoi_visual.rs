//! Visualise the vertices and edges of the Voronoi Cells in 3d
//!
//! Vertices are shown as red spheres and edges as blue lines for illustration
//!

use std::f32::consts::PI;

use bevy::prelude::*;
use voronoi_mosaic::prelude::*;

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
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// camera
	let mut cam_tform = Transform::from_translation(Vec3::new(40.0, 25.0, 40.0));
	cam_tform.look_at(Vec3::ZERO, Vec3::Y);
	cmds.spawn((Camera3d::default(), cam_tform));
	// background plane
	let mesh = meshes.add(Cuboid::new(20.0, 1.0, 20.0));
	let material = materials.add(StandardMaterial {
		base_color: Color::srgb(0.75, 0.75, 0.75),
		..default()
	});
	cmds.spawn((
		Transform::from_translation(Vec3::NEG_Y),
		Mesh3d(mesh),
		MeshMaterial3d(material),
	));
	// lighting
	cmds.spawn((
		DirectionalLight {
			illuminance: light_consts::lux::OVERCAST_DAY,
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
		let x = 40.0 * angle.cos();
		let z = 40.0 * angle.sin();
		tform.translation.x = x;
		tform.translation.z = z;
		tform.look_at(Vec3::ZERO, Vec3::Y);
	}
}

fn visuals(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// points to be used
	let mut points = vec![
		Vec3::new(1.0, 3.0, 4.0),
		Vec3::new(-5.0, 6.0, -4.0),
		Vec3::new(6.0, 0.0, 5.0),
		Vec3::new(12.0, 10.0, -12.0),
		Vec3::new(-15.0, 2.0, 8.0),
		Vec3::new(4.0, 2.0, 12.0),
		Vec3::new(-8.0, 15.0, -8.0),
		Vec3::new(0.0, 12.0, 3.0),
	];
	// compute data
	if let Some(data) = DelaunayData::compute_triangulation_3d(&mut points) {
		if let Some(voronoi) = VoronoiData::cells_from_delaunay_3d(&data) {
			// add simple shapes to showcase what the data looks like
			for cell in voronoi.get_cells().iter() {
				for (i, point) in cell.get_vertices().iter().enumerate() {
					// mark each vertex of every cell
					let mesh = meshes.add(Sphere::new(0.5));
					let material = materials.add(StandardMaterial {
						base_color: Color::srgb(1.0, 0.0, 0.0),
						..default()
					});
					cmds.spawn((
						Mesh3d(mesh.clone()),
						MeshMaterial3d(material.clone()),
						Transform::from_translation(*point),
					));
					// mark the edges
					let (v1, v0) = if i < cell.get_vertices().len() - 1 {
						(cell.get_vertices()[i + 1], *point)
					} else {
						(cell.get_vertices()[0], *point)
					};
					let len = (v1 - v0).length();
					let mesh = meshes.add(Cuboid::new(0.25, 0.25, len));
					let mat = materials.add(StandardMaterial {
						base_color: Color::srgb(0.0, 0.0, 1.0),
						..default()
					});
					let translation = (v1 + v0) / 2.0;
					let mut tform = Transform::from_translation(translation);
					tform.look_at(v1, Vec3::Y);
					cmds.spawn((Mesh3d(mesh), MeshMaterial3d(mat.clone()), tform));
				}
			}
		}
	} else {
		warn!("Data computation failed");
	}
}
