//! Materials used by menu elements

use bevy::prelude::*;

/// Asset location of thinner font
pub const FONT_LIGHT_PATH: &str = "fonts/Napoleon-Light.ttf";
/// Asset location of bold font
pub const FONT_BOLD_PATH: &str = "fonts/Napoleon-Bold.ttf";

/// Default button colour
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
/// Colour when hovering over a button
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
/// Colour when button pressed
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
/// Transparent button
pub const CLEAR_BUTTON: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);
/// Colour for inactive button
pub const DISABLED_BUTTON: Color = Color::srgb(1.0, 1.0, 1.0);

/// Test background
pub const TEST_BACKGROUND_RED: Color = Color::Srgba(Srgba::RED);
/// Test background
pub const TEST_BACKGROUND_GREEN: Color = Color::Srgba(Srgba::GREEN);
/// Test background
pub const TEST_BACKGROUND_BLUE: Color = Color::Srgba(Srgba::BLUE);
/// Test background
pub const TEST_BACKGROUND_BLACK: Color = Color::Srgba(Srgba::BLACK);

/// Transparent background
pub const BACKGROUND_NONE: Color = Color::srgba(0.0, 0.0, 0.0, 0.0);

/// Button text size
pub const FONT_SIZE_BUTTON_LARGE: f32 = 40.0;
/// Button text size
pub const FONT_SIZE_BUTTON_MEDIUM: f32 = 30.0;
/// Button text size
pub const FONT_SIZE_BUTTON_SMALL: f32 = 15.0;

/// General text size
pub const FONT_SIZE_TEXT_LARGE: f32 = 40.0;
/// General text size
pub const FONT_SIZE_TEXT_MEDIUM: f32 = 30.0;
/// General text size
pub const FONT_SIZE_TEXT_SMALL: f32 = 15.0;

/// Font colour
pub const FONT_COLOUR_LIGHT: Color = Color::srgb(0.8, 0.8, 0.8);
/// Font colour
pub const FONT_COLOUR_DARK: Color = Color::srgb(0.2, 0.2, 0.2);

/// Button size
pub const BUTTON_LARGE: (Val, Val) = (Val::Px(300.0), Val::Px(90.0));
/// Button size
pub const BUTTON_MEDIUM: (Val, Val) = (Val::Px(200.0), Val::Px(65.0));
/// Button size
pub const BUTTON_SMALL: (Val, Val) = (Val::Px(100.0), Val::Px(30.0));
