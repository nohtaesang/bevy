use bevy::prelude::*;

#[derive(Resource)]
pub struct CameraSettings {
    pub move_speed: f32,      // base world units / sec
    pub rotate_speed: f32,    // rad / sec
    pub zoom_min: f32,
    pub zoom_max: f32,
}
