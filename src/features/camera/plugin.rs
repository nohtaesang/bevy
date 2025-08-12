//! Camera plugin

use bevy::prelude::*;
use crate::core::AppState;
use super::{camera_movement, camera_rotation, camera_zoom};

/// Camera plugin that handles all camera controls
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            // Camera control systems that run every frame while in InGame state
            .add_systems(Update, (
                camera_movement,
                camera_rotation,
                camera_zoom,
            ).run_if(in_state(AppState::InGame)));
    }
}