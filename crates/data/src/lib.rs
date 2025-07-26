//! Data structures and some logic for various map elements

use bevy::prelude::*;

pub mod common_components;

/// Component used for generating unique IDs, generally exists as an entity in itself
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(GameLabel)]
pub struct IDManager(u32);

impl IDManager {
	/// Creates a new unique ID
	pub fn generate(&mut self) -> u32 {
		self.0 = self.0.overflowing_add(1).0;
		info!("Unique ID generated: {}", self.0);
		self.0.to_owned()
	}
}

/// Label component for every in-game entity. Allows all of them to be serialised and despawned to tidy
/// game state
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct GameLabel;
