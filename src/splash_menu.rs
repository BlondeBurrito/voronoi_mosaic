//! Startup spalash screen displaying logos of tools and and the EarlyDawnGames logo.
//!
//! The screen exists based on a timer after which the main menu loads.
//!

use bevy::prelude::*;
use menu_data::{MenuEntity, materials};
use state::AppState;

/// Logic for loading and progressing the startup splash screen
pub struct SplashPlugin;

impl Plugin for SplashPlugin {
	fn build(&self, app: &mut App) {
		// app.add_systems(
		// 	OnEnter(AppState::SplashScreen),
		// 	settings::game_settings::init_settings,
		// );
		app.add_systems(OnEnter(AppState::SplashScreen), create)
			.add_systems(Update, (update).run_if(in_state(AppState::SplashScreen)))
			.add_systems(
				OnExit(AppState::SplashScreen),
				(delete, settings::game_settings::init_settings),
			);
	}
}
/// Length of time splash screen should be displayed
const SPLASH_SCREEN_DURATION_SECS: f32 = 3.0;

/// Handles how long the splash screen should be displayed
#[derive(Component, Deref, DerefMut)]
pub struct SplashTimer(Timer);

/// Spawn the menu
pub fn create(mut cmds: Commands, asset_server: Res<AssetServer>) {
	cmds.spawn(SplashTimer(Timer::from_seconds(
		SPLASH_SCREEN_DURATION_SECS,
		TimerMode::Once,
	)));
	let ui_id = cmds
		.spawn((
			Node {
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				..default()
			},
			BackgroundColor(materials::TEST_BACKGROUND_BLUE),
		))
		.with_children(|p| {
			p.spawn((
				ImageNode {
					image: asset_server.load("images/logo/logo.png"),
					..default()
				},
				Node {
					width: Val::Percent(40.0),
					height: Val::Percent(40.0),
					..default()
				},
			));
		})
		.id();
	cmds.insert_resource(MenuEntity(ui_id));
}

/// Process menu logic
pub fn update(
	time: Res<Time>,
	mut timer: Query<&mut SplashTimer>,
	mut state: ResMut<NextState<AppState>>,
) {
	for mut splash in &mut timer {
		if splash.tick(time.delta()).just_finished() {
			state.set(AppState::MainMenu);
		}
	}
}

/// Despawn the menu
pub fn delete(
	mut cmds: Commands,
	menu_entity: Res<MenuEntity>,
	timer: Query<Entity, With<SplashTimer>>,
) {
	trace!("Despawning splash timer");
	let splash_t = timer.single().unwrap();
	cmds.entity(splash_t).despawn();
	trace!("Despawning menu");
	cmds.entity(menu_entity.0).despawn();
}
