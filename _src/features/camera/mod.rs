//! Camera control systems
//! 
//! This module contains all camera-related functionality:
//! - Movement: WASD keys + middle mouse panning
//! - Rotation: Q/E keys for rotating the camera
//! - Zoom: Mouse wheel for zooming in/out

pub mod movement;
pub mod rotation;
pub mod zoom;
pub mod plugin;

pub use movement::camera_movement;
pub use rotation::camera_rotation;
pub use zoom::camera_zoom;
pub use plugin::CameraPlugin;