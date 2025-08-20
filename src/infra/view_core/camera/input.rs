// ===============================
// infra/view_core/camera/input.rs
// ===============================
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::{MouseButton, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::Projection;

use super::types::{CameraIntent, CameraSettings, MainCamera};

/// WASD/Arrow 입력 수집 → pan_world 누적
pub fn camera_input_keys(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<CameraSettings>,
    mut intent: ResMut<CameraIntent>,
    q: Query<&Projection, With<MainCamera>>,
) {
    let Ok(proj) = q.single() else {
        return;
    };

    // 입력 벡터
    let mut v = Vec2::ZERO;
    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        v.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        v.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        v.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        v.x += 1.0;
    }
    if v == Vec2::ZERO {
        return;
    }

    // 줌 배율에 비례한 속도 보정 (Orthographic만)
    let zoom_scale = match proj {
        Projection::Orthographic(o) => o.scale,
        _ => 1.0,
    };
    let speed = settings.move_speed * zoom_scale * time.delta_secs();

    intent.pan_world += v.normalize() * speed;
}

/// Q/E 키 회전 수집 → rotate 누적
pub fn camera_input_rotate(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<CameraSettings>,
    mut intent: ResMut<CameraIntent>,
) {
    let mut rot = 0.0;
    if keys.pressed(KeyCode::KeyQ) {
        rot += settings.rotate_speed * time.delta_secs();
    }
    if keys.pressed(KeyCode::KeyE) {
        rot -= settings.rotate_speed * time.delta_secs();
    }
    intent.rotate += rot;
}

/// 마우스 휠 입력 수집 → zoom_steps 누적
pub fn camera_input_wheel(
    mut ev_wheel: EventReader<MouseWheel>,
    settings: Res<CameraSettings>,
    mut intent: ResMut<CameraIntent>,
) {
    let mut steps = 0.0;
    for e in ev_wheel.read() {
        steps += match e.unit {
            MouseScrollUnit::Line => e.y * settings.zoom_step_line,
            MouseScrollUnit::Pixel => e.y * settings.zoom_step_pixel,
        };
    }
    intent.zoom_steps += steps;
}

pub fn camera_input_drag(
    buttons: Res<ButtonInput<MouseButton>>,
    mut ev_motion: EventReader<MouseMotion>,
    mut intent: ResMut<CameraIntent>,
) {
    // 중클릭 또는 오른쪽 버튼 허용 (확인용)
    let dragging = buttons.pressed(MouseButton::Middle) || buttons.pressed(MouseButton::Right);

    let mut delta = Vec2::ZERO;
    for m in ev_motion.read() {
        if dragging {
            delta += m.delta;
        }
    }

    if delta != Vec2::ZERO {
        intent.drag_pixels += delta;
    }
}
