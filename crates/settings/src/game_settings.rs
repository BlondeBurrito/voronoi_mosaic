//! Reads game settings from a file, creates it on first boot. Ensures relevant directories are created

use bevy::prelude::*;
use ron::de::from_reader;
use ron::ser::{PrettyConfig, to_string_pretty};
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::audio_settings::{AudioSettings, detect_audio_changes};
use crate::control_settings::{ControlSettings, detect_control_changes};
use crate::video_settings::{VideoSettings, detect_video_changes};

use super::game_paths::*;

/// Adds settings resources and inits required files and settings
pub struct GameSettingsPlugin;

impl Plugin for GameSettingsPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource(UiScale(1.0));
		app.add_systems(
			Update,
			(
				(write_updated_settings).run_if(
					resource_exists::<VideoSettings>
						.and(resource_exists::<ControlSettings>)
						.and(resource_exists::<AudioSettings>),
				),
				(detect_video_changes).run_if(resource_exists::<VideoSettings>),
				(detect_control_changes).run_if(resource_exists::<ControlSettings>),
				(detect_audio_changes).run_if(resource_exists::<AudioSettings>),
			),
		);
	}
}

/// Resource of settings that can be written to disk and read & updated in game
#[derive(Debug, Deserialize, Serialize, Resource)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub struct GameSettings {
	/// Window related settings
	pub video_settings: VideoSettings,
	/// User input related settings
	pub control_settings: ControlSettings,
	/// Audio settings
	pub audio_settings: AudioSettings,
}

impl GameSettings {
	/// Reads a config ron file containing game settings
	fn read_settings(path: String) -> Self {
		info!("Reading settings file");
		if !Path::new(&path).exists() {
			info!("Settings file does not exist, creating");
			let s = GameSettings::create_settings();
			s.write_to_disk(path.clone());
		}
		// file contents is converted into data struct
		// if it fails new settings are created and written to disk
		let f = File::open(&path).expect("Failed opening file");
		let mut game_settings: GameSettings = match from_reader(f) {
			Ok(x) => x,
			Err(_) => {
				let s = GameSettings::create_settings();
				s.write_to_disk(path);
				s
			}
		};
		game_settings.control_settings.update_keybind_integrity();
		game_settings
	}
	/// Generates a ron config file of default game settings
	fn create_settings() -> Self {
		// create a config ron file based on default struct
		GameSettings {
			video_settings: VideoSettings::default(),
			control_settings: ControlSettings::default(),
			audio_settings: AudioSettings::default(),
		}
	}
	/// Write the current settings in resource [GameSettings] to disk
	fn write_to_disk(&self, path: String) {
		let pretty = PrettyConfig::new();
		let s = to_string_pretty(&self, pretty).expect("Serialization failed");
		match fs::write(path, s) {
			Ok(file) => file,
			Err(e) => panic!("Unable to create config file {e}"),
		};
	}
}

/// Create/read game settings and create the settings resources
pub fn init_settings(mut cmds: Commands) {
	info!("Checking user data directory for config files");
	let company_config_path = get_company_config_path();
	let game_config_path = get_game_config_path();
	let save_path = get_game_config_save_path();
	let settings_file = game_config_path.to_owned() + "/settings.ron";
	let data_path = get_game_data_path();
	// create required paths for game data if they don't exist
	if !Path::new(&company_config_path).exists() {
		info!("Company directory does not exist, creating");
		create_game_company_path();
	}
	if !Path::new(&game_config_path).exists() {
		info!("Game directory does not exist, creating");
		create_game_config_path()
	}
	if !Path::new(&save_path).exists() {
		info!("Save directory does not exist, creating");
		create_game_save_path()
	}
	if !Path::new(&data_path).exists() {
		info!("Data directory does not exist, creating");
		create_game_data_path()
	}
	let settings = GameSettings::read_settings(settings_file);
	cmds.insert_resource(settings.video_settings);
	cmds.insert_resource(settings.control_settings);
	cmds.insert_resource(settings.audio_settings);
}

/// If any settings resources are changed then write them to disk
fn write_updated_settings(
	video: Res<VideoSettings>,
	control: Res<ControlSettings>,
	audio: Res<AudioSettings>,
) {
	if video.is_changed() || control.is_changed() || audio.is_changed() {
		debug!("Writing settings to disk");
		let settings = GameSettings {
			video_settings: video.clone(),
			control_settings: control.clone(),
			audio_settings: audio.clone(),
		};
		let game_config_path = get_game_config_path();
		let settings_file = game_config_path.to_owned() + "/settings.ron";
		settings.write_to_disk(settings_file);
	}
}
