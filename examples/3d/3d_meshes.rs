//! Generates Delaunay Tetrahedrons, uses them to construct unbounded Voronoi Cells and then
//! produces Bevy meshes from the cells
//!
//! The visibility of each layer can be toggled with the buttons
//!

use std::f32::consts::PI;

use bevy::{color::palettes::css::WHITE, prelude::*};
use voronoi_mosaic::prelude::*;

/// Colour of Delaunay edges
const DELAUNAY_EDGE_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);
/// Colour of Delaunay vertices
const DELAUNAY_VERTEX_COLOUR: Color = Color::srgb(0.0, 0.0, 1.0);
/// Colour of Voronoi edges
const VORONOI_EDGE_COLOUR: Color = Color::srgb(1.0, 0.5, 0.0);
/// Colour of the Voronoi vertices
const VORONOI_VERTEX_COLOUR: Color = Color::srgb(0.5, 1.0, 0.0);

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, (setup, visuals, create_ui_buttons))
		.add_systems(Update, (orbit_camera, handle_toggle_buttons))
		.run();
}
/// Requirements
fn setup(
	mut cmds: Commands,
	// mut meshes: ResMut<Assets<Mesh>>,
	// mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// camera
	let mut cam_tform = Transform::from_translation(Vec3::new(40.0, 25.0, 40.0));
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
/// Create simple shapes to visualise the Voronoi data
fn visuals(
	mut cmds: Commands,
	mut mesh_assets: ResMut<Assets<Mesh>>,
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
		//
		// Vec3::new(1.0, -3.0, 4.0),
		// Vec3::new(-5.0, -6.0, -4.0),
		// Vec3::new(12.0, -10.0, -12.0),
		// Vec3::new(-15.0, -2.0, 8.0),
		// Vec3::new(4.0, -2.0, 12.0),
		// Vec3::new(-8.0, -15.0, -8.0),
		// Vec3::new(0.0, -12.0, 3.0),
	];
	// compute data
	if let Some(delaunay) = Delaunay3d::compute_triangulation_3d(&points) {
		create_delaunay_visuals(&mut cmds, &mut mesh_assets, &mut materials, &delaunay);
		if let Some(voronoi) = Voronoi3d::from_delaunay_3d(&delaunay) {
			create_voronoi_cell_visuals(&mut cmds, &mut mesh_assets, &mut materials, &voronoi);
			create_mesh_visuals(&mut cmds, &mut mesh_assets, &mut materials, &voronoi);
		}
	} else {
		warn!("Data computation failed");
	}
}

/// Labels an entity in the Delaunay view for querying
#[derive(Component)]
struct DelaunayLabel;

/// Create simple shapes to illustrate the raw delaunay data
fn create_delaunay_visuals(
	cmds: &mut Commands,
	mesh_assets: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	delaunay: &mosaic_3d::delaunay::Delaunay3d,
) {
	let tetrahedra = delaunay.get_tetrahedra();
	let vertex_lookup = delaunay.get_vertex_lookup();
	for (_, tetra) in tetrahedra.iter() {
		// create markers for vertices
		let mesh = mesh_assets.add(Sphere::new(0.5));
		let material = materials.add(StandardMaterial {
			base_color: DELAUNAY_VERTEX_COLOUR,
			..default()
		});
		// vertices
		let translations = [
			vertex_lookup.get(&tetra.get_vertex_a_id()).unwrap(),
			vertex_lookup.get(&tetra.get_vertex_b_id()).unwrap(),
			vertex_lookup.get(&tetra.get_vertex_c_id()).unwrap(),
			vertex_lookup.get(&tetra.get_vertex_d_id()).unwrap(),
		];
		for translation in translations.iter() {
			cmds.spawn((
				Mesh3d(mesh.clone()),
				MeshMaterial3d(material.clone()),
				Transform::from_translation(**translation),
				Visibility::Hidden,
				DelaunayLabel,
			));
		}
		// create markers for edges
		// let c = Color::hsv(360. * i as f32 / data.get().len() as f32, 0.95, 0.7);
		let mat = materials.add(StandardMaterial {
			base_color: DELAUNAY_EDGE_COLOUR,
			// base_color: c,
			..default()
		});
		for edge in tetra.get_edges().iter() {
			let start = vertex_lookup.get(&edge.get_vertex_a_id()).unwrap();
			let end = vertex_lookup.get(&edge.get_vertex_b_id()).unwrap();
			let len = (end - start).length();
			let mesh = mesh_assets.add(Cuboid::new(0.25, 0.25, len));
			let translation = (end + start) / 2.0;
			let mut tform = Transform::from_translation(translation);
			tform.look_at(*end, Vec3::Y);
			cmds.spawn((
				Mesh3d(mesh),
				MeshMaterial3d(mat.clone()),
				tform,
				Visibility::Hidden,
				DelaunayLabel,
			));
		}
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
				Visibility::Hidden,
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
				Visibility::Hidden,
				VoronoiLabel,
			));
		}
	}
}

/// Labels an entity in the bevy mesh view for querying
#[derive(Component)]
struct MeshLabel;

/// Create the meshes
fn create_mesh_visuals(
	cmds: &mut Commands,
	mesh_assets: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<StandardMaterial>>,
	voronoi: &Voronoi3d,
) {
	let meshes = voronoi.as_bevy3d_meshes();
	for (i, (mesh, position)) in meshes.iter().enumerate() {
		// randomise mesh colour
		let colour = Color::hsl(360. * i as f32 / meshes.len() as f32, 0.95, 0.7);
		let tform = Transform::from_translation(*position);
		let mat = StandardMaterial {
			base_color: colour,
			..default()
		};
		cmds.spawn((
			Mesh3d(mesh_assets.add(mesh.clone())),
			MeshMaterial3d(materials.add(mat)),
			tform,
			MeshLabel,
			Visibility::Visible,
		));
	}
}

/// Labels the toggle buttons for easy querying
#[derive(Component, Copy, Clone)]
enum ButtonLabel {
	/// Button label to toggle Delaunay visibility
	Delaunay,
	/// Button label to toggle Voronoi visibility
	Voronoi,
	/// Button label to toggle mesh visibility
	Mesh,
}

impl std::fmt::Display for ButtonLabel {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ButtonLabel::Delaunay => write!(f, "Toggle Delaunay"),
			ButtonLabel::Voronoi => write!(f, "Toggle Voronoi"),
			ButtonLabel::Mesh => write!(f, "Toggle Meshes"),
		}
	}
}

/// Create the UI
fn create_ui_buttons(mut cmds: Commands) {
	let btns = [
		ButtonLabel::Delaunay,
		ButtonLabel::Voronoi,
		ButtonLabel::Mesh,
	];
	cmds.spawn(Node {
		flex_direction: FlexDirection::Column,
		..default()
	})
	.with_children(|p| {
		for btn in btns.iter() {
			p.spawn((
				*btn,
				Button,
				Node {
					width: Val::Px(100.0),
					height: Val::Px(50.0),
					margin: UiRect::all(Val::Px(5.0)),
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					..Default::default()
				},
				BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
			))
			.with_children(|p| {
				p.spawn((
					Text::new(btn.to_string()),
					TextFont {
						font_size: 22.0,
						..default()
					},
					TextColor(WHITE.into()),
				));
			});
		}
	});
}

/// Handle pressing buttons to change visibility
#[allow(clippy::type_complexity)]
fn handle_toggle_buttons(
	mut btn_q: Query<(&Interaction, &ButtonLabel, &mut BackgroundColor), Changed<Interaction>>,
	mut delaunay_q: Query<
		&mut Visibility,
		(
			With<DelaunayLabel>,
			Without<VoronoiLabel>,
			Without<MeshLabel>,
		),
	>,
	mut voronoi_q: Query<
		&mut Visibility,
		(
			Without<DelaunayLabel>,
			With<VoronoiLabel>,
			Without<MeshLabel>,
		),
	>,
	mut mesh_q: Query<
		&mut Visibility,
		(
			Without<DelaunayLabel>,
			Without<VoronoiLabel>,
			With<MeshLabel>,
		),
	>,
) {
	for (interaction, label, mut colour) in &mut btn_q {
		match interaction {
			Interaction::Pressed => {
				*colour = Color::srgb(0.35, 0.75, 0.35).into();
				match label {
					ButtonLabel::Delaunay => {
						for mut vis in &mut delaunay_q {
							vis.toggle_visible_hidden();
						}
					}
					ButtonLabel::Voronoi => {
						for mut vis in &mut voronoi_q {
							vis.toggle_visible_hidden();
						}
					}
					ButtonLabel::Mesh => {
						for mut vis in &mut mesh_q {
							vis.toggle_visible_hidden();
						}
					}
				}
			}
			Interaction::Hovered => {
				*colour = Color::srgb(0.25, 0.25, 0.25).into();
			}
			Interaction::None => {
				*colour = Color::srgb(0.15, 0.15, 0.15).into();
			}
		}
	}
}
