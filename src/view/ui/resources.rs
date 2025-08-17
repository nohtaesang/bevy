use bevy::prelude::*;
use crate::view::z_index::layer; // 필요하면 삭제 가능

/// UI 색상 테마
#[derive(Resource, Clone, Copy, Debug)]
pub struct UiColors {
    pub panel_bg: Color,
    pub panel_border: Color,
    pub tooltip_bg: Color,
    pub tooltip_border: Color,
    pub text: Color,
    pub text_dim: Color,
    pub hp: Color,
    pub shield: Color,
    pub ally_accent: Color,
    pub enemy_accent: Color,
}

impl Default for UiColors {
    fn default() -> Self {
        Self {
            panel_bg:       Color::srgb(0.08, 0.08, 0.10).with_alpha(0.90),
            panel_border:   Color::srgb(0.30, 0.30, 0.35).with_alpha(0.80),
            tooltip_bg:     Color::srgb(0.12, 0.12, 0.14).with_alpha(0.92),
            tooltip_border: Color::srgb(0.35, 0.35, 0.40).with_alpha(0.85),
            text:           Color::srgb(0.95, 0.97, 1.00),
            text_dim:       Color::srgb(0.70, 0.76, 0.86),
            hp:             Color::srgb(1.00, 0.30, 0.35),
            shield:         Color::srgb(0.35, 0.55, 1.00),
            ally_accent:    Color::srgb(0.20, 0.80, 1.00),
            enemy_accent:   Color::srgb(1.00, 0.45, 0.20),
        }
    }
}

/// UI 배치/크기 설정
#[derive(Resource, Clone, Copy, Debug)]
pub struct UiLayout {
    /// 선택 패널 가로 폭(px)
    pub panel_width: f32,
    /// 패널 내부 패딩(px)
    pub panel_padding: f32,
    /// 패널 요소 간 간격(px)
    pub panel_gap: f32,
    /// 툴팁 오프셋(px): 커서 기준 (추후 hover tooltip에 사용)
    pub tooltip_offset: Vec2,
    /// 텍스트 크기들
    pub font_size_large: f32,
    pub font_size_medium: f32,
    pub font_size_small: f32,
    /// (선택) UI용 Z 레이어가 필요하면 보관
    pub z_ui_base: f32,
}

impl Default for UiLayout {
    fn default() -> Self {
        Self {
            panel_width: 320.0,
            panel_padding: 12.0,
            panel_gap: 8.0,
            tooltip_offset: Vec2::new(16.0, -16.0),
            font_size_large: 22.0,
            font_size_medium: 16.0,
            font_size_small: 13.0,
            z_ui_base: layer::UI, // 프로젝트의 z_index가 있으면 사용, 아니면 0.0 등으로 바꿔도 됨
        }
    }
}

/// UI에서 사용할 에셋 핸들(폰트 등)
#[derive(Resource, Clone, Debug, Default)]
pub struct UiAssets {
    /// 기본 폰트 (로드되면 Some으로 세팅)
    pub font_main: Option<Handle<Font>>,
    /// 모노/서브 폰트 (선택)
    pub font_mono: Option<Handle<Font>>,
}
