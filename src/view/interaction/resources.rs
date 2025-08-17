use bevy::prelude::*;
use crate::view::z_index::layer;

/// 타일 네모 오버레이 뷰 설정
#[derive(Resource, Clone, Copy, Debug)]
pub struct TileOverlayConfig {
    /// Hover 네모 색(알파 포함)
    pub hover_color: Color,
    /// Selected 네모 색(알파 포함)
    pub selected_color: Color,
    /// 타일 내부에서 차지하는 비율(1.0 = 타일 가득)
    pub scale_in_cell: f32,
    /// Hover 오버레이 Z (타일 위, 유닛 아래 권장)
    pub z_hover: f32,
    /// Selected 오버레이 Z (hover보다 약간 위)
    pub z_selected: f32,
}

impl Default for TileOverlayConfig {
    fn default() -> Self {
        Self {
            // 살짝 반투명
            hover_color: Color::srgb(1.0, 1.0, 1.0).with_alpha(0.25),     // 흰색
            selected_color: Color::srgb(1.0, 0.2, 0.2).with_alpha(0.35),  // 빨간색
            scale_in_cell: 1.0,
            z_hover: layer::HOVER,
            z_selected: layer::SELECT,
        }
    }
}
