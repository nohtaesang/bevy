use bevy::prelude::*;
use bevy::input::keyboard::KeyCode;
use bevy::render::camera::Projection; // ⬅️ OrthographicProjection은 Projection 안에 들어있음

// 카메라 스폰 시 이 마커를 붙여주세요.
// 예) commands.spawn((Camera2dBundle::default(), MainCamera));
#[derive(Component)]
pub struct MainCamera;

/// WASD 이동:
/// - 대각선 속도 보정 (normalize)
/// - 줌 비율(Projection::Orthographic.scale)에 비례한 이동 속도 보정
/// - 카메라 회전 고려(월드 기준 자연스러움)
pub fn camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut q: Query<(&mut Transform, &Projection), With<MainCamera>>,
) {
    let Ok((mut tf, proj)) = q.single_mut() else { return; };
    // 입력 벡터 수집
    let mut v = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) { v.y += 1.0; }
    if keys.pressed(KeyCode::KeyS) { v.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA) { v.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD) { v.x += 1.0; }
    if v == Vec2::ZERO { return; }

    // 기본 속도(월드 유닛/초). 나중에 리소스로 빼도 좋음.
    const BASE_SPEED: f32 = 600.0;

    // 현재 카메라가 Orthographic일 때만 scale 사용 (아니면 1.0)
    let zoom_scale = match proj {
        Projection::Orthographic(o) => o.scale,
        _ => 1.0,
    };

    // 줌 보정: 확대(큰 scale)일수록 더 빨리 이동
    let speed = BASE_SPEED * zoom_scale;

    let dt = time.delta_secs();
    let move_vec_world = v.normalize() * speed * dt;

    // 회전 고려 이동
    let delta = tf.rotation * move_vec_world.extend(0.0);
    let z = tf.translation.z; // z 고정
    tf.translation += delta;
    tf.translation.z = z;
}
