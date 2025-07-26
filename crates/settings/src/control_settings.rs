//! Handles input settings
//!

use bevy::input::keyboard::KeyCode;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum::{EnumIter, IntoEnumIterator};

/// Defines the control input settings
#[derive(Serialize, Deserialize, Debug, Resource, Clone)]
pub struct ControlSettings {
	/// Speed of camera movement
	pub camera_move_speed: f32,
	/// Camera zoom scale factor
	pub camera_zoom_sensitivity: f32,
	/// Camera rotation scale factor, radians
	pub camera_rotation_sensitivty: f32,
	/// Key bindings
	pub keybinds: KeyBinds,
}

impl Default for ControlSettings {
	fn default() -> Self {
		ControlSettings {
			camera_move_speed: 1.0,
			camera_zoom_sensitivity: 1.0,
			camera_rotation_sensitivty: 0.02,
			keybinds: KeyBinds::default(),
		}
	}
}

impl ControlSettings {
	/// Get the set keybinds
	pub fn get_keybinds(&self) -> &KeyBinds {
		&self.keybinds
	}
	/// When settings are read perform a check over the stored keybinds and insert any new defaults that have been declared - this allows for new [BindAction] to be intriduced without having to delete and recreate the settings file
	pub fn update_keybind_integrity(&mut self) {
		for action in BindAction::iter() {
			if !self.keybinds.0.contains_key(&action) {
				self.keybinds.0.insert(
					action.clone(),
					ActionKeys::new(action.get_default_key(), None),
				);
			}
		}
	}
}

/// When audio settings are changed update components/resources
pub fn detect_control_changes(settings: Res<ControlSettings>) {
	if settings.is_changed() {
		info!("Control settings changed");
		warn!("TODO");
	}
}

/// KeyCodes for driving logic
#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone, Copy)]
pub struct ActionKeys {
	/// Optional primary key of an action
	primary: Option<KeyCode>,
	/// Optional secondary key of an action
	secondary: Option<KeyCode>,
}

impl ActionKeys {
	/// Create a new [`ActionKeys`] with optionally assigned primary and secondary keys
	fn new(primary: Option<KeyCode>, secondary: Option<KeyCode>) -> Self {
		ActionKeys { primary, secondary }
	}
	/// Get the primary assigned key
	pub fn get_primary(&self) -> Option<KeyCode> {
		self.primary
	}
	/// Get the secondary assigned key
	pub fn get_secondary(&self) -> Option<KeyCode> {
		self.secondary
	}
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone, EnumIter, PartialOrd, Ord)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum BindAction {
	CameraForwards,
	CameraBackwards,
	CameraLeft,
	CameraRight,
}

impl BindAction {
	/// Get the default key for a bind action
	fn get_default_key(&self) -> Option<KeyCode> {
		match self {
			BindAction::CameraForwards => Some(KeyCode::KeyW),
			BindAction::CameraBackwards => Some(KeyCode::KeyS),
			BindAction::CameraLeft => Some(KeyCode::KeyA),
			BindAction::CameraRight => Some(KeyCode::KeyD),
		}
	}
}

/// Maps what action corresponds to what key
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyBinds(pub BTreeMap<BindAction, ActionKeys>);

impl Default for KeyBinds {
	fn default() -> Self {
		let mut map = BTreeMap::new();
		for action in BindAction::iter() {
			map.insert(
				action.clone(),
				ActionKeys::new(action.get_default_key(), None),
			);
		}
		KeyBinds(map)
	}
}

impl KeyBinds {
	/// Get the [ActionKeys] from a given [BindAction]
	pub fn get(&self, bind_action: BindAction) -> ActionKeys {
		if let Some(action_keys) = self.0.get(&bind_action) {
			*action_keys
		} else {
			ActionKeys::new(bind_action.get_default_key(), None)
		}
	}
	/// Set the keys for a [BindAction]
	pub fn set(&mut self, action: BindAction, new_action_keys: ActionKeys) {
		for (_, action_keys) in self.0.iter_mut() {
			if action_keys.get_primary() == new_action_keys.get_primary() {
				action_keys.primary = None;
			}
			if action_keys.get_secondary() == new_action_keys.get_secondary() {
				action_keys.secondary = None;
			}
		}
		if let Some(action_keys) = self.0.get_mut(&action) {
			action_keys.primary = new_action_keys.primary;
			action_keys.secondary = new_action_keys.secondary;
		}
	}
	/// Checks if a key corresponding to the `bind_action` is pressed
	pub fn is_bind_action_pressed(
		&self,
		bind_action: &BindAction,
		input: &Res<ButtonInput<KeyCode>>,
	) -> bool {
		if let Some(keys) = self.0.get(bind_action) {
			if let Some(p) = keys.get_primary() {
				input.pressed(p)
			} else if let Some(s) = keys.get_secondary() {
				input.pressed(s)
			} else {
				false
			}
		} else {
			false
		}
	}
	/// Checks if a key corresponding to the `bind_action` has just been released
	pub fn is_bind_action_just_released(
		&self,
		bind_action: &BindAction,
		input: &Res<ButtonInput<KeyCode>>,
	) -> bool {
		if let Some(keys) = self.0.get(bind_action) {
			if let Some(p) = keys.get_primary() {
				input.just_released(p)
			} else if let Some(s) = keys.get_secondary() {
				input.just_released(s)
			} else {
				false
			}
		} else {
			false
		}
	}
}
