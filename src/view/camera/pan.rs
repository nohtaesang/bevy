use bevy::prelude::*;
use bevy::render::camera::Projection;
use bevy::input::mouse::MouseMotion;

use super::movement::MainCamera;

/// Middle mouse drag panning
/// - 스크린 픽셀 → 월드 좌표로 감도 변환
/// - Orthographic 줌 배율(scale)에 비례해 감도 보정
/// - 카메라 회전을 고려해 방향 자연스럽게
pub fn camera_pan(
    mouse: Res<ButtonInput<MouseButton>>,
    mut motions: EventReader<MouseMotion>,
    mut q: Query<(&mut Transform, &Projection), With<MainCamera>>,
) {
    // 메인 카메라만
    let Ok((mut tf, proj)) = q.single_mut() else { return; };

    // 중클릭이 아닐 땐 아무 것도 하지 않음
    if !mouse.pressed(MouseButton::Middle) {
        // 읽지 않은 이벤트가 남아 있어도 다음 프레임에 다시 읽히므로 굳이 clear 필요 없음
        return;
    }

    // 기본 감도 (픽셀 → 월드 비율의 베이스). 필요하면 리소스로 빼서 튜닝.
    const BASE_SENSITIVITY: f32 = 1.0;

    // Orthographic일 때만 scale 사용 (그 외는 1.0)
    let zoom_scale = match proj {
        Projection::Orthographic(o) => o.scale,
        _ => 1.0,
    };

    // 줌이 커질수록 더 많이 움직여 “세계 좌표 기준” 체감이 일정해지도록 보정
    let sensitivity = BASE_SENSITIVITY * zoom_scale;

    for m in motions.read() {
        // 스크린 기준 delta: 우로 드래그하면 카메라는 좌로 이동해야 하므로 x는 음수
        let delta_screen = Vec2::new(-m.delta.x, m.delta.y) * sensitivity;

        // 회전 고려 후 적용
        let delta_world = tf.rotation * delta_screen.extend(0.0);
        // z 고정
        let z = tf.translation.z;
        tf.translation += delta_world;
        tf.translation.z = z;
    }
}
