//! Game
//!
//!

use bevy::asset::UnapprovedPathMode;
use bevy::color::palettes::css::WHITE;
use bevy::core_pipeline::fxaa::Fxaa;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::window::WindowMode;
use bevy::winit::{UpdateMode, WinitSettings};
use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use data::common_components::PlayerCamera;
use std::hash::{Hash, Hasher};

mod main_menu;
mod settings_menu;
mod splash_menu;
mod wiki;

/// Get thegame version based on cargo version at compile time
const VERSION: &str = env!("CARGO_PKG_VERSION");
/// Number of times per second the FixedUpdate should run
const TIMESTEP: f64 = 1.0 / 64.0;

fn main() {
	let mut app = App::new();
	let local_env = state::LocalEnv::default();
	let log_level = if local_env.is_dev() {
		bevy::log::Level::INFO
	} else {
		bevy::log::Level::WARN
	};

	app.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						title: settings::game_paths::GAME_FOLDER.to_string(),
						present_mode: bevy::window::PresentMode::AutoVsync,
						mode: WindowMode::Windowed,
						// resolution: WindowResolution::default().with_scale_factor_override(1.0),
						resizable: true,
						..default()
					}),
					..default()
				})
				.set(bevy::log::LogPlugin {
					level: log_level,
					filter: "wgpu=warn,naga=warn".to_string(),
					..default()
				})
				.set(bevy::asset::AssetPlugin {
					mode: AssetMode::Unprocessed,
					unapproved_path_mode: UnapprovedPathMode::Allow,//TODO change this, moves settings under assets?
					..default()
				}),
		)
		.add_plugins(LogDiagnosticsPlugin::default())
		.insert_resource(Time::<Fixed>::from_seconds(TIMESTEP))
		.insert_resource(WinitSettings {
			focused_mode: UpdateMode::Continuous,
			unfocused_mode: UpdateMode::Continuous,
		})
		.add_plugins(state::StatePlugin)
		.add_systems(
			Startup,
			init_system,
		)
		.add_plugins(settings::game_settings::GameSettingsPlugin)
		.add_plugins(register::DataPluginRegister)
		//todo what other registars?
		.add_plugins(splash_menu::SplashPlugin)
		.add_plugins(main_menu::MainMenuPlugin)
		// .add_plugins(campaign::CampaignPlugin)
		// .add_plugins(skirmish::SkirmishPlugin)
		// .add_plugins(multiplayer::MultiplayerPlugin)
		.add_plugins(settings_menu::SettingsMenuPlugin)
		.add_plugins(wiki::WikiPlugin)
		// .add_plugins(load::LoadPlugin)
		// .add_plugins(gameplay::GameplayPlugin)
		;
	// use dotenvy to toggle parts of plugins
	if local_env.is_dev() {
		warn!("Setting wireframe plugin");
		app.add_plugins(WireframePlugin::default())
			.insert_resource(WireframeConfig {
				// The global wireframe config enables drawing of wireframes on every mesh,
				// except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
				// regardless of the global configuration.
				global: true,
				// Controls the default color of all wireframes. Used as the default color for global wireframes.
				// Can be changed per mesh using the `WireframeColor` component.
				default_color: WHITE.into(),
			});
	}
	app.run();
}

/// Holds the hashed game version
#[derive(Resource)]
pub struct GameVersion(u64);

impl GameVersion {
	/// Get the hashed version of the application
	pub fn get(&self) -> u64 {
		self.0
	}
}

/// Spawns any entities immediately required
fn init_system(
	world: &mut World,
	// mut cmds: Commands
) {
	world
		.spawn(Camera2d)
		.insert(Fxaa::default())
		.insert(PlayerCamera);
	let mut hasher = std::hash::DefaultHasher::new();
	VERSION.hash(&mut hasher);
	let hash_version = hasher.finish();
	world.insert_resource(GameVersion(hash_version));
}
