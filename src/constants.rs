//! This module defines constants that are used by the rest of the project regarding window size.

/// Height of the window
pub const WINDOW_HEIGHT: u32 = 1000;

/// Width of the window
pub const WINDOW_WIDTH: u32 = 1000;

/// Size of the scene (which is a square)
pub const SCENE_SIZE: u32 = 1000;

/// Point polling spacing
pub const POINT_SPACING: f32 = 2.0;

/// Bezier polyline approximation for then finding equidistant points
pub const POLYLINE_N: u32 = 1000;

pub const BACKGROUND_COLOR: &str = "#77a8c9";

pub const ZOOM_AMOUNT: f32 = 0.3;
pub const PAN_PERCENT: f32 = 0.1;
