use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainKind {
    Ground,
    Wall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapSize {
    pub w: u32,
    pub h: u32,
}
impl MapSize {
    #[inline]
    pub const fn area(&self) -> usize {
        (self.w as usize) * (self.h as usize)
    }
}

/// SSOT: 지형 타일만 보관 (뷰/엔티티는 여기 없음)
#[derive(Resource)]
pub struct Map {
    pub size: MapSize,
    pub tiles: Vec<TerrainKind>, // len = w*h
}
impl Map {
    pub fn new(size: MapSize, fill: TerrainKind) -> Self {
        Self {
            size,
            tiles: vec![fill; size.area()],
        }
    }
    #[inline]
    pub fn index(&self, x: u32, y: u32) -> usize {
        (x + y * self.size.w) as usize
    }
}
