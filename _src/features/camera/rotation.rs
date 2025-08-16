use bevy::prelude::*;

/// Camera rotation system: Q/E keys
pub fn camera_rotation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let rotate_speed = 2.0;

    // QE rotation
    if keyboard_input.pressed(KeyCode::KeyQ) {
        camera_transform.rotate_z(rotate_speed * dt);
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        camera_transform.rotate_z(-rotate_speed * dt);
    }
}