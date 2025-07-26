//! Main menu
//!
//! _________________________________________
//! |                 title                   |
//! |                                         |
//! |                  btn1                   |
//! |                  btn2                   |
//! |                  btn3                   |
//! |                  btn4                   |
//! |                  btn5                   |
//! |                                         |
//! |_________________________________________|
//!

use bevy::app::AppExit;
use bevy::prelude::*;
use menu_data::prelude::*;
use state::AppState;

use crate::VERSION;

/// Defines the various other menus a user can navigate to
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(AppState::MainMenu), create)
			.add_systems(Update, (update).run_if(in_state(AppState::MainMenu)))
			.add_systems(OnExit(AppState::MainMenu), delete);
	}
}

/// Labels the menu buttons
#[derive(Component, Clone, Copy)]
pub enum ButtonLabels {
	/// Settings menu
	Settings,
	/// In built documentation
	Wiki,
	/// Exit
	Quit,
}
impl std::fmt::Display for ButtonLabels {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ButtonLabels::Settings => write!(f, "Settings"),
			ButtonLabels::Wiki => write!(f, "Wiki"),
			ButtonLabels::Quit => write!(f, "Quit"),
		}
	}
}

/// Spawn the menu
pub fn create(mut cmds: Commands, asset_server: Res<AssetServer>) {
	let buttons = [
		ButtonLabels::Wiki,
		ButtonLabels::Settings,
		ButtonLabels::Quit,
	];
	let font_handle: Handle<Font> = asset_server.load(materials::FONT_LIGHT_PATH);
	let ui_id = cmds
		.spawn_empty()
		.insert((
			Node {
				// background canvas
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				flex_direction: FlexDirection::Column,
				justify_content: JustifyContent::Center,
				// vertically center child text
				align_items: AlignItems::Center,
				..Default::default()
			},
			BackgroundColor(materials::TEST_BACKGROUND_RED), //TODO replace with image
		))
		.with_children(|p| {
			// setup menu buttons
			for element in buttons.iter() {
				p.spawn((
					Button,
					*element,
					Node {
						width: materials::BUTTON_LARGE.0,
						height: materials::BUTTON_LARGE.1,
						// center button
						margin: UiRect::all(Val::Px(5.0)),
						// horizontally center child text
						justify_content: JustifyContent::Center,
						// vertically center child text
						align_items: AlignItems::Center,
						..Default::default()
					},
					BackgroundColor(materials::NORMAL_BUTTON),
				))
				.with_children(|p| {
					p.spawn((
						Text::new(element.to_string()),
						TextFont {
							font: font_handle.clone(),
							font_size: materials::FONT_SIZE_BUTTON_LARGE,
							..default()
						},
						TextColor(materials::FONT_COLOUR_LIGHT),
					));
				});
			}
			p.spawn((
				Text::new(format!("Version: {VERSION}")),
				TextFont {
					font: font_handle.clone(),
					font_size: materials::FONT_SIZE_TEXT_SMALL,
					..default()
				},
				TextColor(materials::FONT_COLOUR_LIGHT),
			));
		})
		.id();
	cmds.insert_resource(MenuEntity(ui_id));
}

/// Process menu logic
#[allow(clippy::type_complexity)]
pub fn update(
	mut interaction_query: Query<
		(&Interaction, &mut BackgroundColor, &ButtonLabels),
		(Changed<Interaction>, With<Button>),
	>,
	mut app_exit_event: EventWriter<AppExit>,
	mut state: ResMut<NextState<AppState>>,
) {
	for (interaction, mut colour, label) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Pressed => {
				*colour = materials::PRESSED_BUTTON.into();
				match label {
					ButtonLabels::Settings => state.set(AppState::SettingsMenu),
					ButtonLabels::Wiki => state.set(AppState::WikiMenu),
					ButtonLabels::Quit => {
						app_exit_event.write(AppExit::Success);
					}
				}
			}
			Interaction::Hovered => {
				*colour = materials::HOVERED_BUTTON.into();
			}
			Interaction::None => {
				*colour = materials::NORMAL_BUTTON.into();
			}
		}
	}
}

/// Despawn the menu
pub fn delete(mut cmds: Commands, menu_entity: Res<MenuEntity>) {
	trace!("Despawning menu");
	cmds.entity(menu_entity.0).despawn();
}
