use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;

use super::movement::MainCamera;

/// Q/E 키로 카메라 회전 (Z축)
/// - 대각선 이동과 상호작용 문제 없도록 Transform만 수정
pub fn camera_rotation(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(mut tf) = q.single_mut() else { return; };

    const ROT_SPEED: f32 = 2.0; // rad/sec (원하면 리소스로 빼서 튜닝)
    let dt = time.delta_secs();

    if keys.pressed(KeyCode::KeyQ) {
        tf.rotate_z( ROT_SPEED * dt);
    }
    if keys.pressed(KeyCode::KeyE) {
        tf.rotate_z(-ROT_SPEED * dt);
    }
}
