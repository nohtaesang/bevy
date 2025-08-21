use bevy::prelude::*;
use crate::domain::map::grid_index::GridPos;

/// 지금 프레임 커서가 가리키는 그리드(맵 밖이면 None)
#[derive(Resource, Default, Debug, Clone, Copy)]
pub struct HoverTile {
    pub grid: Option<GridPos>,
}

// (선택) 하이라이트 색/스케일만 남기고 싶으면 이 정도만 유지
#[derive(Resource)]
pub struct HoverHighlightConfig {
    pub color: Color,
    pub size_scale: f32,
}
impl Default for HoverHighlightConfig {
    fn default() -> Self {
        Self { color: Color::srgb(1.0, 1.0, 1.0, ).with_alpha(0.2), size_scale: 1.0 }
    }
}
