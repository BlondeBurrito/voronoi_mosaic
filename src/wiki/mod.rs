//! Encyclopedia of mechanics
//!
//! ```txt
//! _______________________
//! | topic |
//! | topic |
//! | topic |
//! | topic |
//! | topic |  content
//! | topic |
//! | topic |
//! | topic |
//! |_______|
//! |  back |
//! |_______|_______________
//! ```

use bevy::prelude::*;
use menu_data::{MenuEntity, materials};
use state::AppState;

pub mod welcome;

/// Handles the wiki
pub struct WikiPlugin;

impl Plugin for WikiPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(AppState::WikiMenu), create);
		app.add_systems(OnExit(AppState::WikiMenu), delete);
		app.add_systems(
			Update,
			(back_button, handle_page_buttons).run_if(in_state(AppState::WikiMenu)),
		);
	}
}
/// Labels the parent entity of the content area
#[derive(Component)]
struct ContentParent;

/// Create the main wiki structure
fn create(mut cmds: Commands, asset_server: Res<AssetServer>) {
	let font_handle: Handle<Font> = asset_server.load(materials::FONT_LIGHT_PATH);
	let ui_id = cmds
		.spawn_empty()
		.insert((
			Node {
				// background canvas
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				flex_direction: FlexDirection::Row,
				justify_content: JustifyContent::Center,
				// vertically center child text
				align_items: AlignItems::Center,
				..Default::default()
			},
			BackgroundColor(materials::TEST_BACKGROUND_RED),
		))
		.with_children(|p| {
			// left column with back at bottom
			p.spawn((
				Node {
					width: Val::Percent(10.0),
					height: Val::Percent(100.0),
					flex_direction: FlexDirection::Column,
					..default()
				},
				BackgroundColor(materials::TEST_BACKGROUND_GREEN),
			))
			.with_children(|p| {
				// buttons for pages
				p.spawn((
					Node {
						width: Val::Percent(100.0),
						height: Val::Percent(90.0),
						flex_direction: FlexDirection::Column,
						..default()
					},
					BackgroundColor(materials::TEST_BACKGROUND_GREEN),
				))
				.with_children(|p| {
					// populate pages
					let pages = get_pages();
					for page in pages.iter() {
						// create heading page
						p.spawn((
							Node {
								width: Val::Percent(100.0),
								height: Val::Percent(5.0),
								..default()
							},
							Button,
							page.0,
						))
						.with_children(|p| {
							p.spawn((
								Text::new(page.0.to_string()),
								TextFont {
									font: font_handle.clone(),
									font_size: materials::FONT_SIZE_BUTTON_MEDIUM,
									..default()
								},
								TextColor(materials::FONT_COLOUR_LIGHT),
							));
						});
						// create sub pages if applicable
						if !page.1.is_empty() {
							for subpage in page.1.iter() {
								//TODO use a box to nest it in slightly? use margin prop?
								p.spawn((
									Node {
										width: Val::Percent(100.0),
										height: Val::Percent(5.0),
										..default()
									},
									Button,
									*subpage,
								))
								.with_children(|p| {
									p.spawn((
										Text::new(subpage.to_string()),
										TextFont {
											font: font_handle.clone(),
											font_size: materials::FONT_SIZE_BUTTON_MEDIUM,
											..default()
										},
										TextColor(materials::FONT_COLOUR_LIGHT),
									));
								});
							}
						}
					}
				});
				// box for back button
				p.spawn((
					Node {
						width: Val::Percent(100.0),
						height: Val::Percent(10.0),
						..default()
					},
					BackgroundColor(materials::TEST_BACKGROUND_GREEN),
				))
				.with_children(|p| {
					p.spawn((
						Button,
						Node {
							width: Val::Percent(95.0),
							height: Val::Percent(95.0),
							justify_content: JustifyContent::Center,
							align_items: AlignItems::Center,
							..Default::default()
						},
						BackgroundColor(materials::NORMAL_BUTTON),
						BackButtonLabel,
					))
					.with_children(|p| {
						p.spawn((
							Text::new("Back"),
							TextFont {
								font: font_handle.clone(),
								font_size: materials::FONT_SIZE_BUTTON_MEDIUM,
								..default()
							},
							TextColor(materials::FONT_COLOUR_LIGHT),
						));
					});
				});
			});
			// right large column for content area
			p.spawn((
				Node {
					width: Val::Percent(90.0),
					height: Val::Percent(100.0),
					flex_direction: FlexDirection::Column,
					..default()
				},
				BackgroundColor(materials::TEST_BACKGROUND_BLUE),
				ContentParent,
			));
		})
		.id();
	cmds.insert_resource(MenuEntity(ui_id));
}

/// Labels the back button
#[derive(Component)]
struct BackButtonLabel;

/// Handle the back button
#[allow(clippy::type_complexity)]
fn back_button(
	mut query: Query<(&Interaction, &mut BackgroundColor), (With<Button>, With<BackButtonLabel>)>,
	mut state: ResMut<NextState<AppState>>,
) {
	for (interaction, mut colour) in &mut query {
		match interaction {
			Interaction::Pressed => {
				*colour = materials::PRESSED_BUTTON.into();
				state.set(AppState::MainMenu);
			}
			Interaction::Hovered => *colour = materials::HOVERED_BUTTON.into(),
			Interaction::None => *colour = materials::NORMAL_BUTTON.into(),
		}
	}
}

/// Despawn the menu
pub fn delete(mut cmds: Commands, menu_entity: Res<MenuEntity>) {
	trace!("Despawning menu");
	cmds.entity(menu_entity.0).despawn();
}

/// Label each page and sub-page
#[derive(Component, Clone, Copy)]
#[allow(clippy::missing_docs_in_private_items)]
enum Pages {
	Welcome,
	Other,
	OtherSubpage,
}

impl std::fmt::Display for Pages {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			Pages::Welcome => "Welcome",
			Pages::Other => "Other",
			Pages::OtherSubpage => "OtherSubpage",
		};
		write!(f, "{str}")
	}
}
/// Get all the pages
fn get_pages() -> Vec<(Pages, Vec<Pages>)> {
	vec![
		(Pages::Welcome, vec![]),
		(Pages::Other, vec![Pages::OtherSubpage]),
	]
}
/// Logic to handle selecting a page to view
#[allow(clippy::type_complexity)]
fn handle_page_buttons(
	mut cmds: Commands,
	asset_server: Res<AssetServer>,
	mut btn_q: Query<
		(&Interaction, &mut BackgroundColor, &Pages),
		(With<Button>, Changed<Interaction>),
	>,
	content_q: Query<(Entity, Option<&Children>), With<ContentParent>>,
) {
	for (interaction, mut colour, page) in &mut btn_q {
		match interaction {
			Interaction::Pressed => {
				*colour = materials::PRESSED_BUTTON.into();
				//TODO
				info!("Clciked page {}", page.to_string());
				for (entity, op_children) in &content_q {
					if let Some(children) = op_children {
						for e in children.iter() {
							cmds.entity(e).despawn();
						}
					}
					match page {
						Pages::Welcome => welcome::create(&mut cmds, &entity, &asset_server),
						Pages::Other => panic!("TODO"),
						Pages::OtherSubpage => panic!("TODO"),
					}
				}
			}
			Interaction::Hovered => *colour = materials::HOVERED_BUTTON.into(),
			Interaction::None => *colour = materials::NORMAL_BUTTON.into(),
		}
	}
}
