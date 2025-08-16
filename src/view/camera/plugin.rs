use bevy::prelude::*;
use crate::app::state::AppState;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (
                super::movement::camera_movement,
                super::rotation::camera_rotation,
                super::zoom::camera_zoom,
            )
            .run_if(in_state(AppState::Battle)) // 전투 상태에서만
        );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}