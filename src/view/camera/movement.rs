use bevy::prelude::*;

/// Camera movement system: WASD keys + middle mouse panning
pub fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut mouse_motion: EventReader<bevy::input::mouse::MouseMotion>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    let move_speed = 300.0;

    // WASD movement
    let mut movement = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement.y += move_speed * dt;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement.y -= move_speed * dt;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        movement.x -= move_speed * dt;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        movement.x += move_speed * dt;
    }

    // Apply movement relative to camera rotation
    let rotated_movement = camera_transform.rotation * movement;
    camera_transform.translation += rotated_movement;

    // Middle mouse button panning
    if mouse_input.pressed(MouseButton::Middle) {
        for motion in mouse_motion.read() {
            let pan_speed = 2.0;
            let delta = Vec3::new(-motion.delta.x * pan_speed, motion.delta.y * pan_speed, 0.0);
            let rotated_delta = camera_transform.rotation * delta;
            camera_transform.translation += rotated_delta;
        }
    }
}