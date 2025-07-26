//! A plugin used for registering components which derive [Reflect] implementations. This allows
//! those components to be written to and read from disk
//!

use bevy::prelude::*;
use data::{GameLabel, IDManager};

/// Plugin used to register components for reflection
pub struct DataPluginRegister;

impl Plugin for DataPluginRegister {
	fn build(&self, app: &mut App) {
		app.register_type::<GameLabel>()
			.register_type::<IDManager>();
	}
}
