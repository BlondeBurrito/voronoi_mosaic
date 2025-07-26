//! Populate content
//!

use bevy::prelude::*;
use menu_data::materials;

/// Create the welcome page
pub fn create(cmds: &mut Commands, parent: &Entity, asset_server: &Res<AssetServer>) {
	let font_handle: Handle<Font> = asset_server.load(materials::FONT_LIGHT_PATH);
	cmds.entity(*parent).with_children(|p| {
		p.spawn((
			Text::new("Welcome!"),
			TextFont {
				font: font_handle.clone(),
				font_size: materials::FONT_SIZE_TEXT_LARGE,
				..default()
			},
			TextColor(materials::FONT_COLOUR_LIGHT),
		));
	});
}
