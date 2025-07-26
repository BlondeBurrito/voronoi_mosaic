//! Data structures and logic controlling the settings

use bevy::prelude::*;
use std::path::PathBuf;
pub mod audio_settings;
pub mod control_settings;
pub mod game_paths;
pub mod game_settings;
pub mod video_settings;

/// Init game settings and files
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins(game_settings::GameSettingsPlugin);
	}
}
/// Attempts to find the `assets` dir the app requires, if it cannot be found then a panic ensues
pub fn get_system_assets_path() -> PathBuf {
	let try_path = match std::env::var("BEVY_ASSET_ROOT") {
		Ok(p) => PathBuf::from(p),
		Err(_) => match std::env::current_exe() {
			Ok(exe_path) => exe_path,
			Err(e) => panic!("Could not determine path of application, {e}"),
		},
	};
	// use ancestors to 'walk' up the system path checking for ./assets along the way
	let mut ancestors = try_path.ancestors();
	let mut path: Option<PathBuf> = None;
	while path.is_none() {
		let next = ancestors.next();
		if next.is_none() {
			panic!("Cannot determine `assets` location");
		}
		let try_assets = next.unwrap().join("assets");
		match try_assets.try_exists() {
			Ok(b) => {
				if b {
					path = Some(try_assets)
				} else {
					continue;
				}
			}
			Err(e) => {
				error!("Cannot try_exists {:?}", try_assets);
				panic!("Cannot determine `assets` location, failure checking dir tree, {e}");
			}
		}
	}
	path.unwrap()
}
