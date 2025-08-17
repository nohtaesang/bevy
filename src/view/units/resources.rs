use bevy::prelude::*;

/// 유닛 뷰 설정
#[derive(Resource, Clone, Copy)]
pub struct UnitViewConfig {
    /// 타일 내부에서 유닛 스프라이트가 차지하는 비율(0~1)
    pub scale_in_cell: f32,
    /// 유닛 Z 레이어
    pub z_layer: f32,
}

impl Default for UnitViewConfig {
    fn default() -> Self {
        Self {
            scale_in_cell: 0.75, // 타일보다 살짝 작게
            z_layer: crate::view::z_index::layer::UNITS,
        }
    }
}
