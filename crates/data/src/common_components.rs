//! Collection of various components which are commonl used across entities

use bevy::prelude::*;

/// Labels the players camera
#[derive(Component)]
pub struct PlayerCamera;

/// Component inserted into an entity to indicate that it has been selected
#[derive(Component, Default)]
pub struct Selected;

/// Attaches a unique ID as a component, typically the ID is derived from the [super::IDManager]
#[derive(Component, Reflect, Default, Copy, Clone)]
#[reflect(Component)]
pub struct UniqueID(pub u32);

/// Freindly name of an entity
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Name(String);
