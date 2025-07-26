//! Settings menu
//!

use bevy::prelude::*;
use menu_data::{MenuEntity, materials};
use state::AppState;

/// Plugin for populating the settings menu and system for interacting with the `settings` crate
pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(AppState::SettingsMenu), create)
			.add_systems(
				Update,
				(update, back_btn, window_mode_btn).run_if(in_state(AppState::SettingsMenu)),
			)
			.add_systems(OnExit(AppState::SettingsMenu), delete);
	}
}

/// Labels button to return to main menu
#[derive(Component)]
pub struct BackButtonLabel;

/// Labels buttons to chnage window mode
#[derive(Component, Clone, Copy)]
pub enum WindowModeButton {
	/// Exclusive fullscreen
	Fullscreen,
	/// Fullscreen
	FullscreenBorderless,
	/// Windowed
	Windowed,
}

impl std::fmt::Display for WindowModeButton {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			WindowModeButton::Fullscreen => write!(f, "Fullscreen"),
			WindowModeButton::FullscreenBorderless => write!(f, "Borderless Fullscreen"),
			WindowModeButton::Windowed => write!(f, "Windowed"),
		}
	}
}

/// Spawn the menu
pub fn create(mut cmds: Commands, asset_server: Res<AssetServer>) {
	let font_handle: Handle<Font> = asset_server.load(materials::FONT_LIGHT_PATH);
	let id = cmds
		.spawn((
			Node {
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				..default()
			},
			BackgroundColor(materials::TEST_BACKGROUND_BLUE),
		))
		.with_children(|p| {
			//window mode
			let modes = [
				WindowModeButton::Fullscreen,
				WindowModeButton::FullscreenBorderless,
				WindowModeButton::Windowed,
			];
			for mode in modes {
				p.spawn((
					Button,
					mode,
					BackgroundColor(materials::NORMAL_BUTTON),
					Node {
						width: materials::BUTTON_LARGE.0,
						height: materials::BUTTON_LARGE.1,
						margin: UiRect::all(Val::Px(5.0)),
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						..Default::default()
					},
				))
				.with_children(|p| {
					p.spawn((
						Text::new(mode.to_string()),
						TextFont {
							font: font_handle.clone(),
							font_size: materials::FONT_SIZE_BUTTON_LARGE,
							..default()
						},
						TextColor(materials::FONT_COLOUR_LIGHT),
					));
				});
			}
			//back
			p.spawn((
				BackButtonLabel,
				Button,
				Node {
					width: Val::Px(300.0),
					height: Val::Px(90.0),
					margin: UiRect::all(Val::Px(5.0)),
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					..Default::default()
				},
				BackgroundColor(materials::NORMAL_BUTTON),
			))
			.with_children(|p| {
				p.spawn((
					Text::new("Back"),
					TextFont {
						font: font_handle.clone(),
						font_size: materials::FONT_SIZE_BUTTON_LARGE,
						..default()
					},
					TextColor(materials::FONT_COLOUR_LIGHT),
				));
			});
		})
		.id();
	cmds.insert_resource(MenuEntity(id));
}

/// Process menu logic
pub fn update() {} //TODO

/// Temp buttons to change window mode
#[allow(clippy::type_complexity)]
fn window_mode_btn(
	mut query: Query<
		(&Interaction, &mut BackgroundColor, &WindowModeButton),
		(With<Button>, Changed<Interaction>),
	>,
	mut video_settings: ResMut<settings::video_settings::VideoSettings>,
) {
	for (interaction, mut colour, button) in &mut query {
		match interaction {
			Interaction::Pressed => {
				*colour = materials::PRESSED_BUTTON.into();
				match button {
					WindowModeButton::Fullscreen => {
						info!("Setting fullscreen");
						video_settings.window_mode =
							settings::video_settings::GameWindowMode::Fullscreen;
					}
					WindowModeButton::FullscreenBorderless => {
						info!("Setting borderless");
						video_settings.window_mode =
							settings::video_settings::GameWindowMode::BorderlessFullscreen;
					}
					WindowModeButton::Windowed => {
						info!("Setting windowed");
						video_settings.window_mode =
							settings::video_settings::GameWindowMode::Windowed;
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
/// Handles returning to the main menu
#[allow(clippy::type_complexity)]
pub fn back_btn(
	mut query: Query<
		(&Interaction, &mut BackgroundColor),
		(With<BackButtonLabel>, Changed<Interaction>),
	>,
	mut state: ResMut<NextState<AppState>>,
) {
	for (interaction, mut colour) in &mut query {
		match *interaction {
			Interaction::Pressed => {
				*colour = materials::PRESSED_BUTTON.into();
				state.set(AppState::MainMenu);
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
