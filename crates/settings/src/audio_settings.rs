//! Handles audio related settings
//!

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Defines audio settings
#[derive(Serialize, Deserialize, Debug, Resource, Clone)]
pub struct AudioSettings {
	/// Overall volume
	pub master_volume: i32,
}

impl Default for AudioSettings {
	fn default() -> Self {
		AudioSettings { master_volume: 100 }
	}
}

/// When audio settings are changed update components/resources
pub fn detect_audio_changes(settings: Res<AudioSettings>) {
	if settings.is_changed() {
		info!("Audio settings changed");
		warn!("TODO");
	}
}
