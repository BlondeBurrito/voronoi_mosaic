//! Generates Delaunay Triangles, uses them to construct Voronoi Cells, clips
//! them to a polygon and then produces Bevy meshes from the cells
//!
//! The visibility of each layer can be toggled with the buttons
//!

use bevy::{color::palettes::css::WHITE, prelude::*};
use voronoi_mosaic::prelude::*;

/// Z location of the generated meshes
const MESH_Z: f32 = 1.0;
/// Z location of Voronoi cell edes
const VORONOI_CELL_EDGE_Z: f32 = 2.0;
/// Colour of Voronoi edges
const VORONOI_EDGE_COLOUR: Color = Color::srgb(1.0, 0.5, 0.0);
/// Z location of the Voronoi vertices
const VORONOI_CELL_VERTEX_Z: f32 = 3.0;
/// Colour of the Voronoi vertices
const VORONOI_VERTEX_COLOUR: Color = Color::srgb(0.5, 1.0, 0.0);
/// Z location of Delaunay edges
const DELAUNAY_EDGE_Z: f32 = 4.0;
/// Colour of Delaunay edges
const DELAUNAY_EDGE_COLOUR: Color = Color::srgb(1.0, 0.0, 0.0);
/// Z location of Delaunay vertices
const DELAUNAY_VERTEX_Z: f32 = 5.0;
/// Colour of Delaunay vertices
const DELAUNAY_VERTEX_COLOUR: Color = Color::srgb(0.0, 0.0, 1.0);

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_systems(Startup, (setup, visuals, create_ui_buttons))
		.add_systems(Update, handle_toggle_buttons)
		// .add_plugins(bevy::sprite::Wireframe2dPlugin::default())
		// 	.insert_resource(bevy::sprite::Wireframe2dConfig {
		// 		// The global wireframe config enables drawing of wireframes on every mesh,
		// 		// except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
		// 		// regardless of the global configuration.
		// 		global: true,
		// 		// Controls the default color of all wireframes. Used as the default color for global wireframes.
		// 		// Can be changed per mesh using the `WireframeColor` component.
		// 		default_color: WHITE.into(),
		// 	})
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
	let mesh = meshes.add(Rectangle::from_length(450.0));
	let material = materials.add(Color::srgb(0.75, 0.75, 0.75));
	cmds.spawn((Transform::default(), Mesh2d(mesh), MeshMaterial2d(material)));
	// background plane matching clip boundary
	let mesh = meshes.add(Rectangle::from_length(400.0));
	let material = materials.add(Color::srgb(0.0, 0.0, 0.0));
	cmds.spawn((
		Transform::from_xyz(0.0, 0.0, 0.1),
		Mesh2d(mesh),
		MeshMaterial2d(material),
	));
}
/// Compute and display data
fn visuals(
	mut cmds: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// points to be used
	let points = vec![
		Vec2::new(-380.0, -380.0),
		Vec2::new(-355.0, -375.0),
		Vec2::new(-350.0, -233.0),
		Vec2::new(-241.0, -296.0),
		Vec2::new(-169.0, -201.0),
		Vec2::new(-124.0, -86.0),
		Vec2::new(-53.0, -124.0),
		Vec2::new(-94.0, -75.0),
		Vec2::new(-22.0, -35.0),
		//
		Vec2::new(366.0, -24.0),
		Vec2::new(340.0, -284.0),
		Vec2::new(285.0, -165.0),
		Vec2::new(236.0, -94.0),
		Vec2::new(156.0, -156.0),
		Vec2::new(120.0, -85.0),
		Vec2::new(99.0, -33.0),
		Vec2::new(72.0, -199.0),
		Vec2::new(16.0, -350.0),
		//
		Vec2::new(352.0, 42.0),
		Vec2::new(326.0, 107.0),
		Vec2::new(256.0, 251.0),
		Vec2::new(175.0, 365.0),
		Vec2::new(142.0, 168.0),
		Vec2::new(102.0, 72.0),
		Vec2::new(84.0, 192.0),
		Vec2::new(58.0, 247.0),
		Vec2::new(19.0, 27.0),
		//
		Vec2::new(-385.0, 36.0),
		Vec2::new(-321.0, 354.0),
		Vec2::new(-276.0, 68.0),
		Vec2::new(-244.0, 302.0),
		Vec2::new(-153.0, 168.0),
		Vec2::new(-122.0, 272.0),
		Vec2::new(-84.0, 196.0),
		Vec2::new(-63.0, 241.0),
		Vec2::new(-24.0, 202.0),
		//
		Vec2::new(399.0, 399.0),
		Vec2::new(-399.0, 399.0),
		Vec2::new(-399.0, -399.0),
		Vec2::new(399.0, -399.0),
		//
		Vec2::new(0.0, 399.0),
		Vec2::new(-399.0, 0.0),
		Vec2::new(0.0, -399.0),
		Vec2::new(399.0, 0.0),
	];
	// compute data
	let mosaic = Mosaic2d::new(&points);
	if let Some(delaunay) = mosaic.get_delaunay() {
		create_delaunay_visuals(&mut cmds, &mut meshes, &mut materials, delaunay);
		if let Some(voronoi) = mosaic.get_voronoi() {
			// create the voronoi markers before mutation so it can be
			// seen with the actual meshes how they have been clipped
			create_voronoi_cell_visuals(&mut cmds, &mut meshes, &mut materials, voronoi);
			// let boundary = vec![Vec2::new(400.0, 400.0), Vec2::new(-400.0, 400.0), Vec2::new(-400.0, -400.0), Vec2::new(400.0, -400.0)];
			let boundary = vec![
				Vec2::new(200.0, 200.0),
				Vec2::new(-200.0, 200.0),
				Vec2::new(-200.0, -200.0),
				Vec2::new(200.0, -200.0),
			];
			create_clipped_mesh_visuals(&mut cmds, &mut meshes, &mut materials, voronoi, &boundary);
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
				Visibility::Hidden,
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
				Visibility::Hidden,
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
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	voronoi: &Voronoi2d,
) {
	let cells = voronoi.get_cells();
	let vertex_lookup = voronoi.get_vertex_lookup();
	for cell in cells.values() {
		for (i, vertex_id) in cell.get_vertex_ids().iter().enumerate() {
			// mark each vertex of every cell
			let mesh = meshes.add(Circle::new(10.0));
			let material = materials.add(VORONOI_VERTEX_COLOUR);
			let position = vertex_lookup.get(vertex_id).unwrap();
			cmds.spawn((
				Mesh2d(mesh.clone()),
				MeshMaterial2d(material.clone()),
				Transform::from_translation(position.extend(VORONOI_CELL_VERTEX_Z)),
				VoronoiLabel,
				Visibility::Hidden,
			));
			// mark the edges
			let (v1, v0) = if i < cell.get_vertex_ids().len() - 1 {
				(cell.get_vertex_ids()[i + 1], *vertex_id)
			} else {
				(cell.get_vertex_ids()[0], *vertex_id)
			};
			let v1_pos = vertex_lookup.get(&v1).unwrap();
			let v0_pos = vertex_lookup.get(&v0).unwrap();
			let y_len = (v1_pos - v0_pos).length();
			let mesh = meshes.add(Rectangle::from_size(Vec2::new(5.0, y_len)));
			let mat = materials.add(VORONOI_EDGE_COLOUR);
			let translation = (v1_pos + v0_pos) / 2.0;
			let angle = Vec2::Y.angle_to(v0_pos - v1_pos);
			let tform = Transform {
				translation: translation.extend(VORONOI_CELL_EDGE_Z),
				rotation: Quat::from_rotation_z(angle),
				..default()
			};
			cmds.spawn((
				Mesh2d(mesh),
				MeshMaterial2d(mat.clone()),
				tform,
				VoronoiLabel,
				Visibility::Hidden,
			));
		}
	}
}

/// Labels an entity in the bevy mesh view for querying
#[derive(Component)]
struct MeshClippedLabel;

/// Create the meshes
fn create_clipped_mesh_visuals(
	cmds: &mut Commands,
	meshe_assets: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	voronoi: &Voronoi2d,
	boundary: &[Vec2],
) {
	let meshes = voronoi.as_clipped_bevy2d_meshes(boundary);
	for (i, (mesh, position)) in meshes.values().enumerate() {
		// randomise mesh colour
		let colour = Color::hsl(360. * i as f32 / meshes.len() as f32, 0.95, 0.7);
		let tform = Transform::from_translation(position.extend(MESH_Z));
		cmds.spawn((
			Mesh2d(meshe_assets.add(mesh.clone())),
			MeshMaterial2d(materials.add(colour)),
			tform,
			MeshClippedLabel,
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
			Without<MeshClippedLabel>,
		),
	>,
	mut voronoi_q: Query<
		&mut Visibility,
		(
			Without<DelaunayLabel>,
			With<VoronoiLabel>,
			Without<MeshClippedLabel>,
		),
	>,
	mut mesh_q: Query<
		&mut Visibility,
		(
			Without<DelaunayLabel>,
			Without<VoronoiLabel>,
			With<MeshClippedLabel>,
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
