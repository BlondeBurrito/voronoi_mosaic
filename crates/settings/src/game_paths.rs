//! Locale of game config files and directories for data

use std::{fs, path::Path};

use bevy::prelude::*;

/// Folder name for game config files
pub const GAME_FOLDER: &str = "bevy_repo_template";

/// Path to the `earlydawngames` directory from the user's local data directory
pub fn get_company_config_path() -> String {
	let data_path = dirs::data_local_dir().unwrap().to_str().unwrap().to_owned();
	[data_path + "/earlydawngames"].concat()
}

/// Create the `earlydawngames` directory under the user's local data path
pub fn create_game_company_path() {
	let path = get_company_config_path();
	if !Path::new(&path).exists() {
		info!("Company directory does not exist, creating at {}", &path);
		match fs::create_dir(path) {
			Ok(c) => c,
			Err(error) => panic!("Unable to create directory for company files: {error}"),
		};
	}
}
/// Path to where game config files live
pub fn get_game_config_path() -> String {
	[get_company_config_path() + "/" + GAME_FOLDER].concat()
}
/// Create the game specific directory for config files
pub fn create_game_config_path() {
	let path = get_game_config_path();
	if !Path::new(&path).exists() {
		info!("Game directory does not exist, creating at {}", &path);
		match fs::create_dir(path) {
			Ok(c) => c,
			Err(error) => panic!("Unable to create directory for game files: {error}"),
		};
	}
}
/// Path to where game save files live
pub fn get_game_config_save_path() -> String {
	[get_game_config_path() + "/saves"].concat()
}
/// Create the game specific directory for save files
pub fn create_game_save_path() {
	let path = get_game_config_save_path();
	if !Path::new(&path).exists() {
		info!("Game save directory does not exist, creating at {}", &path);
		match fs::create_dir(path) {
			Ok(c) => c,
			Err(error) => panic!("Unable to create directory for game files: {error}"),
		};
	}
}

/// Path to where data lives
pub fn get_game_data_path() -> String {
	[get_game_config_path() + "/data"].concat()
}
/// Create the game data dir
pub fn create_game_data_path() {
	let path = get_game_data_path();
	if !Path::new(&path).exists() {
		info!("Game data directory does not exist, creating at {}", &path);
		match fs::create_dir(path) {
			Ok(c) => c,
			Err(error) => panic!("Unable to create directory for game data: {error}"),
		};
	}
}
// /// Path to where game custom map files live
// pub fn get_game_config_custom_map_path() -> String {
// 	[get_game_config_path() + "/custom_maps"].concat()
// }
// /// Create the game specific directory for save files
// pub fn create_game_custom_map_path() {
// 	let path = get_game_config_custom_map_path();
// 	if !Path::new(&path).exists() {
// 		info!(
// 			"Game custom map directory does not exist, creating at {}",
// 			&path
// 		);
// 		let c = fs::create_dir(path);
// 		let _c = match c {
// 			Ok(c) => c,
// 			Err(error) => panic!("Unable to create directory for custom map files: {}", error),
// 		};
// 	}
// }
