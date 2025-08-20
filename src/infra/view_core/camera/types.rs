// ===============================
// infra/view_core/camera/types.rs
// ===============================
use bevy::prelude::*;

/// 주 카메라 마커
#[derive(Component)]
pub struct MainCamera;

/// 설정 및 한 프레임 의도 버퍼
#[derive(Resource)]
pub struct CameraSettings {
    pub move_speed: f32,   // world units / sec (기본 이동 속도)
    pub rotate_speed: f32, // rad / sec
    pub zoom_min: f32,
    pub zoom_max: f32,
    pub zoom_step_line: f32,      // MouseScrollUnit::Line 감도
    pub zoom_step_pixel: f32,     // MouseScrollUnit::Pixel 감도
    pub pan_drag_multiplier: f32, // 1.0이면 1px → scale world
}
impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            move_speed: 600.0,
            rotate_speed: 2.0,
            zoom_min: 0.2,
            zoom_max: 5.0,
            zoom_step_line: 0.15,
            zoom_step_pixel: 0.0025,
            pan_drag_multiplier: 1.0, // 기본값
        }
    }
}

/// 한 프레임 동안 누적되는 카메라 의도
#[derive(Resource, Default)]
pub struct CameraIntent {
    /// 키보드 기반 월드 팬 델타 (회전 보정은 적용 단계에서 처리)
    pub pan_world: Vec2,
    /// 마우스 드래그 픽셀 델타 누적 (우/중 클릭)
    pub drag_pixels: Vec2,
    /// Z 회전 라디안(+반시계)
    pub rotate: f32,
    /// 휠 스텝 누적(+이면 줌인)
    pub zoom_steps: f32,
}
