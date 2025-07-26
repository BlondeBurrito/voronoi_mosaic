//! Handles video related settings
//!

use bevy::{
	core_pipeline::fxaa::{Fxaa, Sensitivity},
	prelude::*,
	window::{PrimaryWindow, WindowMode},
};
use serde::{Deserialize, Serialize};

/// Resolution of the game
#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum Resolution {
	HD1600x900,
	HD1280x1080,
	HD1440x1080,
	HD1920x1080,
	HD2560x1600,
}

impl std::fmt::Display for Resolution {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Resolution::HD1600x900 => write!(f, "1600x900"),
			Resolution::HD1280x1080 => write!(f, "1280x1080"),
			Resolution::HD1440x1080 => write!(f, "1440x1080"),
			Resolution::HD1920x1080 => write!(f, "1920x1080"),
			Resolution::HD2560x1600 => write!(f, "2560x1600"),
		}
	}
}

/// Window mode of the game
#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
pub enum GameWindowMode {
	//TODO fullscreen breaks alt-tab
	Fullscreen,
	//TODO ui elements out of place
	BorderlessFullscreen,
	Windowed,
}
/// Type and amount of anti-aliasing
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AntiAliasing {
	/// Amount of MSAA sampling, 1 is off, other valid options are 2, 4 and 8
	pub msaa: Option<u32>,
	/// Amount of FXAA, values ranging frm 0..=4 map to [bevy::core_pipeline::fxaa::Sensitivity]
	pub fxaa: Option<u32>,
	//TODO other kinds of AA like SMAA
}

/// Defines the video settings
#[derive(Serialize, Deserialize, Debug, Resource, Clone)]
pub struct VideoSettings {
	/// Window mode
	pub window_mode: GameWindowMode,
	/// Window resolution
	pub resolution: Resolution,
	/// Is vsync enabled
	pub vsync: bool,
	/// Is the cursor restricted to the window
	pub cursor_locked: bool,
	/// UI scale factor
	pub ui_scale: f32,
	/// Sharp edge smoothing
	pub anti_aliasing: AntiAliasing,
}

impl Default for VideoSettings {
	fn default() -> Self {
		VideoSettings {
			window_mode: GameWindowMode::Windowed,
			resolution: Resolution::HD1920x1080, //TODO: get this default from OS, then check if supported by game
			vsync: true,
			cursor_locked: false,
			ui_scale: 1.0,
			anti_aliasing: AntiAliasing {
				msaa: None,
				fxaa: Some(2),
			},
		}
	}
}

/// If video settings are modified then update necessary components/settings
pub fn detect_video_changes(
	settings: Res<VideoSettings>,
	mut windows: Query<&mut Window, With<PrimaryWindow>>,
	mut ui_scale: ResMut<UiScale>,
	mut msaa_q: Query<&mut Msaa>,
	mut fxaa_q: Query<&mut Fxaa>,
) {
	if settings.is_changed() {
		info!("Video settings changed");
		let mut window = windows.single_mut().unwrap();
		match settings.window_mode {
			GameWindowMode::Fullscreen => {
				window.mode =
					WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current);
				window.resolution.set_scale_factor_override(Some(1.0));
			}
			GameWindowMode::BorderlessFullscreen => {
				window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Current);
				//TODO at 1.0 it sitcks static box on monitor
				// window.resolution.set_scale_factor_override(Some(1.0));
				//TODO at None windows leaks outside monitor
				window.resolution.set_scale_factor_override(None);
			}
			GameWindowMode::Windowed => {
				window.mode = WindowMode::Windowed;
				window.resolution.set_scale_factor_override(None);
			}
		}
		match settings.resolution {
			Resolution::HD1600x900 => {
				window.resolution.set(1600.0, 900.0);
			}
			Resolution::HD1280x1080 => {
				window.resolution.set(1280.0, 1080.0);
			}
			Resolution::HD1440x1080 => {
				window.resolution.set(1440.0, 1080.0);
			}
			Resolution::HD1920x1080 => {
				window.resolution.set(1920.0, 1080.0);
			}
			Resolution::HD2560x1600 => {
				window.resolution.set(2560.0, 1600.0);
			}
		}
		info!("Physical size {}", window.resolution.physical_size());
		info!("Logical size {}", window.size());
		match settings.vsync {
			true => window.present_mode = bevy::window::PresentMode::AutoVsync,
			false => window.present_mode = bevy::window::PresentMode::AutoNoVsync,
		}
		match settings.cursor_locked {
			true => window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Confined,
			false => window.cursor_options.grab_mode = bevy::window::CursorGrabMode::None,
		}
		ui_scale.0 = settings.ui_scale;
		match settings.anti_aliasing.msaa {
			Some(i) => {
				let mut msaa = msaa_q.single_mut().unwrap();
				match i {
					1 => *msaa = Msaa::Off,
					2 => *msaa = Msaa::Sample2,
					4 => *msaa = Msaa::Sample4,
					8 => *msaa = Msaa::Sample8,
					_ => panic!("Unsupported Msaa value {i}"),
				}
				//todo iter over fxaa to handle extra cameras?
				let mut fxaa = fxaa_q.single_mut().unwrap();
				fxaa.enabled = false;
			}
			None => {
				let mut msaa = msaa_q.single_mut().unwrap();
				*msaa = Msaa::Off;
				match settings.anti_aliasing.fxaa {
					Some(x) => match x {
						0 => {
							let mut fxaa = fxaa_q.single_mut().unwrap();
							fxaa.enabled = true;
							fxaa.edge_threshold = Sensitivity::Low;
							fxaa.edge_threshold_min = Sensitivity::Low;
						}
						1 => {
							let mut fxaa = fxaa_q.single_mut().unwrap();
							fxaa.enabled = true;
							fxaa.edge_threshold = Sensitivity::Medium;
							fxaa.edge_threshold_min = Sensitivity::Medium;
						}
						2 => {
							let mut fxaa = fxaa_q.single_mut().unwrap();
							fxaa.enabled = true;
							fxaa.edge_threshold = Sensitivity::High;
							fxaa.edge_threshold_min = Sensitivity::High;
						}
						3 => {
							let mut fxaa = fxaa_q.single_mut().unwrap();
							fxaa.enabled = true;
							fxaa.edge_threshold = Sensitivity::Extreme;
							fxaa.edge_threshold_min = Sensitivity::Extreme;
						}
						4 => {
							let mut fxaa = fxaa_q.single_mut().unwrap();
							fxaa.enabled = true;
							fxaa.edge_threshold = Sensitivity::Ultra;
							fxaa.edge_threshold_min = Sensitivity::Ultra;
						}
						_ => panic!("Invalid FXAA value selected"),
					},
					None => {
						let mut fxaa = fxaa_q.single_mut().unwrap();
						fxaa.enabled = true;
						fxaa.edge_threshold = Sensitivity::Low;
						fxaa.edge_threshold_min = Sensitivity::Low;
					}
				}
			}
		}
	}
}
