//! Visualise the vertices and edges of the Delaunay Triangulation in 3d
//!
//! Vertices are shown as red spheres and edges as blue lines for illustration
//!

use std::f32::consts::PI;

use bevy::{color::palettes::css::WHITE, prelude::*};
use voronoi_mosaic::{delaunay::delaunay_3d::{compute_dimension_bounds, compute_super_tetrahedra}, prelude::*};

/// Colour of Delaunay edges
const DELAUNAY_EDGE_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);
/// Colour of Delaunay vertices
const DELAUNAY_VERTEX_COLOUR: Color = Color::srgb(0.0, 0.0, 1.0);

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, (setup, visuals))
		// .add_systems(Update, orbit_camera)
		.add_plugins(bevy::pbr::wireframe::WireframePlugin::default())
			.insert_resource(bevy::pbr::wireframe::WireframeConfig {
				// The global wireframe config enables drawing of wireframes on every mesh,
				// except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
				// regardless of the global configuration.
				global: false,
				// Controls the default color of all wireframes. Used as the default color for global wireframes.
				// Can be changed per mesh using the `WireframeColor` component.
				default_color: WHITE.into(),
			})
		.run();
}
/// Requirements
fn setup(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// camera
	let mut cam_tform = Transform::from_translation(Vec3::new(50.0, 35.0, 35.0));
	let mut cam_tform = Transform::from_translation(Vec3::new(200.0, 35.0, 35.0));
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
		let x = 40.0 * angle.cos();
		let z = 40.0 * angle.sin();
		tform.translation.x = x;
		tform.translation.z = z;
		tform.look_at(Vec3::ZERO, Vec3::Y);
	}
}

/// Create simple shapes to visualise the Delaunay data
fn visuals(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// points to be used
	let points = vec![
		Vec3::new(1.0, 3.0, 4.0),
		Vec3::new(-5.0, 6.0, -4.0),
		Vec3::new(6.0, 0.0, 5.0),
		Vec3::new(12.0, 10.0, -12.0),
		Vec3::new(-15.0, 2.0, 8.0),
		Vec3::new(4.0, 2.0, 12.0),
		Vec3::new(-8.0, 15.0, -8.0),
		Vec3::new(0.0, 12.0, 3.0),
	];
	for (i, point) in points.iter().enumerate() {
		if i != 0 {break}
		let mesh = meshes.add(Sphere::new(0.5));
			let material = materials.add(StandardMaterial {
				base_color: DELAUNAY_VERTEX_COLOUR,
				..default()
			});
		cmds.spawn((
					Mesh3d(mesh.clone()),
					MeshMaterial3d(material.clone()),
					Transform::from_translation(*point),
				));
	}
	let dim_bounds = compute_dimension_bounds(&points);
	let super_tet = compute_super_tetrahedra(&dim_bounds.0, &dim_bounds.1);
	for tetra in super_tet.iter() {
		info!("super tet verts {:?}", tetra);
		for vert in tetra {
			let mesh = meshes.add(Sphere::new(0.5));
			let material = materials.add(StandardMaterial {
				base_color: Color::srgb(1.0, 1.0, 0.0),
				..default()
			});
			cmds.spawn((
				Mesh3d(mesh.clone()),
				MeshMaterial3d(material.clone()),
				Transform::from_translation(*vert),
			));
		}
		// show edges
		let edges = [
			(tetra[0], tetra[1]),
			(tetra[1], tetra[2]),
			(tetra[2], tetra[0]),
			(tetra[3], tetra[0]),
			(tetra[3], tetra[1]),
			(tetra[3], tetra[2])
		];
		for edge in edges {
			let mat = materials.add(StandardMaterial {
				base_color: Color::srgb(1.0, 1.0, 0.0),
				..default()
			});
			let len = (edge.1 - edge.0).length();
			let mesh = meshes.add(Cuboid::new(0.25, 0.25, len));
			let translation = (edge.1 + edge.0) / 2.0;
			let mut tform = Transform::from_translation(translation);
			tform.look_at(edge.1, Vec3::Y);
			// info!("edge {:?}", edge);
			// info!("midpoint {}", translation);
			cmds.spawn((Mesh3d(mesh), MeshMaterial3d(mat.clone()), tform));
		}
	}
	// circumsphere

	let circumcnetre = Vec3::new(-1.5, 43.6, 0.0);
	let radius = 53.1;
	let mat = materials.add(StandardMaterial {
				base_color: Color::srgba(1.0, 1.0, 1.0, 0.0),
				alpha_mode: AlphaMode::Mask(0.0),
				unlit: true,
				..default()
			});
	let mesh = meshes.add(Sphere::new(radius).mesh().ico(2).unwrap());
	let tform = Transform::from_translation(circumcnetre);
	cmds.spawn((Mesh3d(mesh), MeshMaterial3d(mat.clone()), tform, bevy::pbr::wireframe::Wireframe));




	// // compute data
	// if let Some(data) = DelaunayData::compute_triangulation_3d(&points) {
	// 	for tetra in data.get().iter() {
	// 		// create markers for vertices
	// 		let mesh = meshes.add(Sphere::new(0.5));
	// 		let material = materials.add(StandardMaterial {
	// 			base_color: DELAUNAY_VERTEX_COLOUR,
	// 			..default()
	// 		});
	// 		// vertices
	// 		let translations = [
	// 			tetra.get_vertex_a(),
	// 			tetra.get_vertex_b(),
	// 			tetra.get_vertex_c(),
	// 			tetra.get_vertex_d(),
	// 		];
	// 		for translation in translations.iter() {
	// 			cmds.spawn((
	// 				Mesh3d(mesh.clone()),
	// 				MeshMaterial3d(material.clone()),
	// 				Transform::from_translation(**translation),
	// 			));
	// 		}
	// 		// create markers for edges
	// 		let mat = materials.add(StandardMaterial {
	// 			base_color: DELAUNAY_EDGE_COLOUR,
	// 			..default()
	// 		});
	// 		for edge in tetra.get_edges().iter() {
	// 			let len = (edge.1 - edge.0).length();
	// 			let mesh = meshes.add(Cuboid::new(0.25, 0.25, len));
	// 			let translation = (edge.1 + edge.0) / 2.0;
	// 			let mut tform = Transform::from_translation(translation);
	// 			tform.look_at(edge.1, Vec3::Y);
	// 			// info!("edge {:?}", edge);
	// 			// info!("midpoint {}", translation);
	// 			cmds.spawn((Mesh3d(mesh), MeshMaterial3d(mat.clone()), tform));
	// 		}
	// 	}
	// } else {
	// 	warn!("Data computation failed");
	// }
}
