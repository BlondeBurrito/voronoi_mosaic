//! Generic values and resources used in menu states

use bevy::prelude::*;

pub mod materials;
pub mod prelude;

/// Root resource for storing a menus enity ID, despawning the contained entity will clear the entire menu
#[derive(Resource)]
pub struct MenuEntity(pub Entity);
