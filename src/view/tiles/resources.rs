// src/view/tiles/resources.rs
use bevy::prelude::*;
use std::collections::HashMap;
use crate::gameplay::tiles::components::TerrainKind;

/// 타일 렌더 설정(뷰 전용)
#[derive(Resource, Clone)]
pub struct TileViewConfig {
    /// 셀 내부 여백(픽셀). 실제 스프라이트 크기 = cell_size - gap
    pub gap: f32,
    /// 단색 렌더용 팔레트
    pub palette: HashMap<TerrainKind, Color>,
    /// 렌더 모드(초기엔 Solid 사용)
    pub mode: TileRenderMode,
}

/// 타일 렌더 모드
#[derive(Clone)]
pub enum TileRenderMode {
    /// 단색 사각형
    Solid,
    /// 스프라이트 아틀라스(필요 시 사용)
    Atlas {
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
        /// 지형 → 아틀라스 인덱스
        map: HashMap<TerrainKind, usize>,
    },
}

impl Default for TileViewConfig {
    fn default() -> Self {
        Self {
            gap: 1.0,
            palette: default_palette(),
            mode: TileRenderMode::Solid,
        }
    }
}

impl TileViewConfig {
    /// 지형에 해당하는 색상
    #[inline]
    pub fn color(&self, kind: TerrainKind) -> Color {
        *self
            .palette
            .get(&kind)
            .unwrap_or(&Color::srgb(0.5, 0.5, 0.5))
    }

    /// 셀 크기에서 gap을 뺀 실제 타일 스프라이트 크기
    #[inline]
    pub fn tile_size(&self, cell_size: Vec2) -> Vec2 {
        Vec2::new(
            (cell_size.x - self.gap).max(0.0),
            (cell_size.y - self.gap).max(0.0),
        )
    }

    /// 팔레트 편집 헬퍼
    #[inline]
    pub fn set_color(&mut self, kind: TerrainKind, color: Color) {
        self.palette.insert(kind, color);
    }
}

fn default_palette() -> HashMap<TerrainKind, Color> {
    use TerrainKind::*;
    let mut m = HashMap::with_capacity(5);
    m.insert(Ground, Color::srgb(0.75, 0.75, 0.75));
    m.insert(Forest, Color::srgb(0.20, 0.55, 0.25));
    m.insert(Water,  Color::srgb(0.25, 0.45, 0.95));
    m.insert(Wall,   Color::srgb(0.25, 0.25, 0.25));
    m.insert(Road,   Color::srgb(0.70, 0.70, 0.35));
    m
}
