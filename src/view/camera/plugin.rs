use bevy::prelude::*;
use crate::app::state::AppState;

use super::movement::{camera_movement, MainCamera};
use super::rotation::camera_rotation;
use super::zoom::camera_zoom;
use super::pan::camera_pan;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (camera_movement, camera_rotation, camera_zoom, camera_pan)
                    .run_if(in_state(AppState::Battle)),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,           // ✅ 0.16 스타일
        Transform::default(),
        MainCamera,
    ));
}
