use bevy::prelude::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TerrainKind {
    Ground = 0,
    Forest = 1,
    Water  = 2,
    Wall   = 3,
    Road   = 4,
}

impl TerrainKind {
    /// 보병 기준 기본 통과 규칙(정책에서 덮어쓸 수 있음)
    #[inline]
    pub fn is_passable_base(self) -> bool {
        !matches!(self, TerrainKind::Wall | TerrainKind::Water)
    }
}

/// ===== 그리드 좌표 (컴포넌트) =====
#[derive(Component, Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: u32,
    pub y: u32,
}
impl GridPos {
    pub const fn new(x: u32, y: u32) -> Self { Self { x, y } }
    pub fn in_bounds(self, width: u32, height: u32) -> bool {
        self.x < width && self.y < height
    }
    #[inline]
    pub fn index(self, width: u32) -> usize {
        (self.y as usize) * (width as usize) + (self.x as usize)
    }
    #[inline]
    pub fn manhattan(self, other: GridPos) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

// ===== conversions =====

// 튜플 -> GridPos (OK)
impl From<(u32, u32)> for GridPos {
    fn from(t: (u32, u32)) -> Self {
        GridPos { x: t.0, y: t.1 }
    }
}

// GridPos -> 튜플 (From은 불가하므로 Into로 구현)
impl Into<(u32, u32)> for GridPos {
    fn into(self) -> (u32, u32) {
        (self.x, self.y)
    }
}