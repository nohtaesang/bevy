use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseScrollUnit};
use bevy::render::camera::Projection;

use super::movement::MainCamera;

/// 마우스 휠 줌 (Bevy 0.16)
/// - Orthographic 배율(scale) 변경
/// - Line/Pixel 단위 감도 구분
/// - 커서 고정 줌: 줌 전/후 커서 아래 월드 포인트를 맞춰 카메라 평행이동
pub fn camera_zoom(
    mut scrolls: EventReader<MouseWheel>,
    windows: Query<&Window>,
    mut q: Query<(&Camera, &GlobalTransform, &mut Transform, &mut Projection), With<MainCamera>>,
) {
    // 단일 윈도우 전제 (멀티 윈도우면 구분 로직 추가)
    let window = match windows.single() {
        Ok(w) => w,
        Err(_) => {
            // 윈도우가 없으면 이벤트만 소진하고 리턴
            scrolls.clear();
            return;
        }
    };

    let (camera, cam_gt, mut tf, mut proj) = match q.single_mut() {
        Ok(v) => v,
        Err(_) => {
            scrolls.clear();
            return;
        }
    };

    // Orthographic일 때만 scale 사용
    let scale_ref_opt: Option<&mut f32> = match &mut *proj {
        Projection::Orthographic(o) => Some(&mut o.scale),
        _ => None,
    };
    let Some(scale_ref) = scale_ref_opt else {
        scrolls.clear();
        return;
    };

    // 라인/픽셀 단위 감도 분리 후 누적
    let mut step_accum: f32 = 0.0;
    for s in scrolls.read() {
        let step = match s.unit {
            MouseScrollUnit::Line  => s.y * 0.15,
            MouseScrollUnit::Pixel => s.y * 0.0025,
        };
        step_accum += step;
    }
    if step_accum.abs() < f32::EPSILON {
        return; // 이번 프레임엔 실질적 스크롤 없음
    }

    // 줌 한계
    const ZOOM_MIN: f32 = 0.2;
    const ZOOM_MAX: f32 = 5.0;

    // 커서 고정: 줌 전 커서 아래 월드 좌표 (Result → Option으로 변환)
    let before_world: Option<Vec2> = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(cam_gt, cursor).ok());

    // 배율 변경 (휠 업(y>0) 확대)
    let factor = (1.0 - step_accum).clamp(0.2, 5.0);
    *scale_ref = (*scale_ref * factor).clamp(ZOOM_MIN, ZOOM_MAX);

    // 커서 고정 보정: 줌 후 같은 스크린 좌표 아래 월드 포인트로 되돌리기
    if let (Some(cursor), Some(before)) = (window.cursor_position(), before_world) {
        if let Ok(after) = camera.viewport_to_world_2d(cam_gt, cursor) {
            let delta = before - after;
            let z = tf.translation.z;
            tf.translation.x += delta.x;
            tf.translation.y += delta.y;
            tf.translation.z = z;
        }
    }
}
