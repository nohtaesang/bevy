// ===============================
// infra/view_core/camera/plugin.rs
// ===============================
use bevy::prelude::*;

use super::apply::camera_apply_intent;
use super::input::{camera_input_drag, camera_input_keys, camera_input_rotate, camera_input_wheel};
use super::types::{CameraIntent, CameraSettings, MainCamera};
use crate::app::state::ModeState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum CamSet {
    Input,
    Apply,
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .init_resource::<CameraIntent>()
            .configure_sets(Update, (CamSet::Input, CamSet::Apply).chain())
            .add_systems(Startup, setup_camera)
            // Input 모음
            .add_systems(
                Update,
                (
                    camera_input_keys,
                    camera_input_rotate,
                    camera_input_wheel,
                    camera_input_drag,
                )
                    .in_set(CamSet::Input)
                    .run_if(in_state(ModeState::Battle)),
            )
            // Apply 한 번
            .add_systems(
                Update,
                camera_apply_intent
                    .in_set(CamSet::Apply)
                    .run_if(in_state(ModeState::Battle)),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::default(), MainCamera));
}
