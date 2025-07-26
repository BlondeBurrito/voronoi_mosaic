//! The various states the simulation can take

use ::bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod seed;

/// Handle simulation state
pub struct StatePlugin;

impl Plugin for StatePlugin {
	fn build(&self, app: &mut App) {
		app.init_state::<AppState>()
			.insert_resource(Difficulty(DifficultyLevel::Medium));
	}
}

/// Game state
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, States, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum AppState {
	#[default]
	SplashScreen,
	MainMenu,
	SettingsMenu,
	WikiMenu,
}
/// Controls the difficulty, exists as a resource and is created at startup
#[derive(Debug, Clone, Eq, PartialEq, Hash, Resource)]
pub struct Difficulty(DifficultyLevel);

impl Difficulty {
	/// Get teh difficulty level
	pub fn get(&self) -> &DifficultyLevel {
		&self.0
	}
}

/// Difficulty levels
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Copy, Serialize, Deserialize)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum DifficultyLevel {
	Easy,
	#[default]
	Medium,
	Hard,
	Insane,
}

use std::{env, fmt};

/// Running environment
#[derive(PartialEq, Clone)]
enum AppEnv {
	/// Set for local dev work
	Dev,
	/// Default
	Prod,
}

impl fmt::Display for AppEnv {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			AppEnv::Dev => write!(f, "dev"),
			AppEnv::Prod => write!(f, "prod"),
		}
	}
}

impl AppEnv {
	/// Is the system a dev environemnt
	pub fn is_dev(&self) -> bool {
		*self == AppEnv::Dev
	}
}
/// Whether the app is running in dev mode or live mode
#[derive(Resource, Clone)]
pub struct LocalEnv {
	/// Environemnt mode
	environment: AppEnv,
}

impl Default for LocalEnv {
	fn default() -> Self {
		if let Ok(path) = dotenvy::dotenv() {
			trace!(".env read successfully from {}", path.display())
		};
		let environment = match env::var("ENV") {
			Ok(v) if v == "dev" => {
				println!("Dev environment detetced");
				AppEnv::Dev
			}
			_ => AppEnv::Prod,
		};
		LocalEnv { environment }
	}
}

impl LocalEnv {
	/// Is this the dev environment
	pub fn is_dev(&self) -> bool {
		self.environment.is_dev()
	}
}
