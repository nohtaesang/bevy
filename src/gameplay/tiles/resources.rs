use bevy::prelude::*;
use super::components::{TerrainKind, GridPos};

/// ===== 불변: 보드 전체 데이터 (리소스) =====
#[derive(Resource, Debug)]
pub struct BaseTileMap {
    pub w: u32,                      // 가로 칸 수
    pub h: u32,                      // 세로 칸 수
    pub terrain: Vec<TerrainKind>,   // len = w*h
}

impl BaseTileMap {
    /// 길이 검증 포함 생성자
    pub fn new(w: u32, h: u32, terrain: Vec<TerrainKind>) -> Self {
        assert_eq!(terrain.len(), (w as usize) * (h as usize), "terrain length must equal w*h");
        Self { w, h, terrain }
    }

    /// 단일 지형으로 채우기
    pub fn filled(w: u32, h: u32, kind: TerrainKind) -> Self {
        Self { w, h, terrain: vec![kind; (w as usize) * (h as usize)] }
    }

  
#[inline] pub fn len(&self) -> usize { (self.w as usize) * (self.h as usize) }
    // ===== SSOT: GridPos 기반 =====
    #[inline] pub fn in_bounds_pos(&self, p: GridPos) -> bool {
        p.x < self.w && p.y < self.h
    }
    #[inline] pub fn idx_pos(&self, p: GridPos) -> usize {
        p.index(self.w) // y * w + x
    }
    #[inline] pub fn terrain_at_pos(&self, p: GridPos) -> TerrainKind {
        debug_assert!(self.in_bounds_pos(p));
        self.terrain[self.idx_pos(p)]
    }

    // ===== 래퍼: (x,y) -> GridPos 위임 =====
    #[inline] pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        self.in_bounds_pos(GridPos::new(x, y))
    }
    #[inline] pub fn idx(&self, x: u32, y: u32) -> usize {
        self.idx_pos(GridPos::new(x, y))
    }
    #[inline] pub fn terrain_at(&self, x: u32, y: u32) -> TerrainKind {
        self.terrain_at_pos(GridPos::new(x, y))
    }
}

/// ===== 좌표계 설정 (리소스) =====
#[derive(Resource, Clone, Copy, Debug)]
pub struct TileConfig {
    /// 그리드 '한 칸'의 월드 폭/높이
    pub cell_size: Vec2,
    /// 보드의 월드 원점(좌하단 등 기준점)
    pub origin: Vec2,
}

impl Default for TileConfig {
    fn default() -> Self {
        Self { cell_size: Vec2::new(1.0, 1.0), origin: Vec2::ZERO }
    }
}

impl TileConfig {
    /// 월드 → 그리드 (경계 밖이면 None)
    #[inline]
    pub fn world_to_grid(&self, world: Vec3, map: &BaseTileMap) -> Option<GridPos> {
        debug_assert!(self.cell_size.x > 0.0 && self.cell_size.y > 0.0, "cell_size must be > 0");
        let eps = 1e-6;
        let gx = ((world.x - self.origin.x) / self.cell_size.x + eps).floor() as i32;
        let gy = ((world.y - self.origin.y) / self.cell_size.y + eps).floor() as i32;
        if gx < 0 || gy < 0 { return None; }
        let p = GridPos::new(gx as u32, gy as u32);
        map.in_bounds_pos(p).then_some(p)
    }

    /// 그리드 중심점 → 월드
    #[inline]
    pub fn grid_to_world_center(&self, gp: GridPos, z: f32) -> Vec3 {
        Vec3::new(
            self.origin.x + (gp.x as f32 + 0.5) * self.cell_size.x,
            self.origin.y + (gp.y as f32 + 0.5) * self.cell_size.y,
            z,
        )
    }

    /// 그리드 좌하단 모서리 → 월드
    #[inline]
    pub fn grid_to_world_min(&self, gp: GridPos, z: f32) -> Vec3 {
        Vec3::new(
            self.origin.x + (gp.x as f32) * self.cell_size.x,
            self.origin.y + (gp.y as f32) * self.cell_size.y,
            z,
        )
    }

    /// 그리드 '한 칸'의 월드 AABB
    #[inline]
    pub fn grid_cell_aabb(&self, gp: GridPos, z_min: f32, z_max: f32) -> (Vec3, Vec3) {
        let min = self.grid_to_world_min(gp, z_min);
        let max = Vec3::new(min.x + self.cell_size.x, min.y + self.cell_size.y, z_max);
        (min, max)
    }
}

/// ===== 점유 인덱스 (리소스) =====
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridError {
    OutOfBounds,
    Occupied,
    FromEmpty,
    ToOccupied,
    SameCell,
    ShrinkNotAllowed,
}

#[derive(Resource, Debug)]
pub struct GridIndex {
    pub w: u32,
    pub h: u32,
    /// (x,y) -> Some(Entity) or None
    pub unit: Vec<Option<Entity>>, // len = w*h
    /// 실제 변경이 있을 때만 증가
    pub version: u32,
}

impl GridIndex {
    #[inline] pub fn new(w: u32, h: u32) -> Self {
        debug_assert!((w as u64) * (h as u64) <= (usize::MAX as u64), "w*h too large for usize");
       
Self { w, h, unit: vec![None; (w as usize) * (h as usize)], version: 0 }
    }

  
#[inline] pub fn len(&self) -> usize { (self.w as usize) * (self.h as usize) }
    // ===== SSOT: GridPos 기반 =====
    #[inline] pub fn in_bounds_pos(&self, p: GridPos) -> bool { p.x < self.w && p.y < self.h }
    #[inline] fn idx_pos(&self, p: GridPos) -> usize { p.index(self.w) }

    #[inline] pub fn get_pos(&self, p: GridPos) -> Option<Entity> {
        debug_assert!(self.in_bounds_pos(p));
        self.unit[self.idx_pos(p)]
    }
    #[inline] pub fn is_empty_pos(&self, p: GridPos) -> bool { self.get_pos(p).is_none() }

    #[inline] fn set_slot(&mut self, i: usize, v: Option<Entity>) {
        if self.unit[i] != v {
            self.unit[i] = v;
            self.version = self.version.wrapping_add(1);
        }
    }

    pub fn place_pos(&mut self, p: GridPos, e: Entity) -> Result<(), GridError> {
        if !self.in_bounds_pos(p) { return Err(GridError::OutOfBounds); }
        let i = self.idx_pos(p);
        if self.unit[i].is_some() { return Err(GridError::Occupied); }
        self.set_slot(i, Some(e));
        Ok(())
    }

    pub fn set_pos(&mut self, p: GridPos, v: Option<Entity>) -> Result<(), GridError> {
        if !self.in_bounds_pos(p) { return Err(GridError::OutOfBounds); }
        self.set_slot(self.idx_pos(p), v);
        Ok(())
    }

    pub fn clear_pos(&mut self, p: GridPos) -> Result<Option<Entity>, GridError> {
        if !self.in_bounds_pos(p) { return Err(GridError::OutOfBounds); }
        let i = self.idx_pos(p);
        let prev = self.unit[i];
        self.set_slot(i, None);
        Ok(prev)
    }

    pub fn move_to(&mut self, from: GridPos, to: GridPos) -> Result<(), GridError> {
        if from == to { return Err(GridError::SameCell); }
        if !self.in_bounds_pos(from) || !self.in_bounds_pos(to) {
            return Err(GridError::OutOfBounds);
        }
        let fi = self.idx_pos(from);
        let ti = self.idx_pos(to);
        let e = self.unit[fi].ok_or(GridError::FromEmpty)?;
        if self.unit[ti].is_some() { return Err(GridError::ToOccupied); }
        self.unit[fi] = None;
        self.unit[ti] = Some(e);
        self.version = self.version.wrapping_add(1);
        Ok(())
    }

    pub fn swap_cells(&mut self, a: GridPos, b: GridPos) -> Result<(), GridError> {
        if a == b { return Err(GridError::SameCell); }
        if !self.in_bounds_pos(a) || !self.in_bounds_pos(b) {
            return Err(GridError::OutOfBounds);
        }
        let ai = self.idx_pos(a);
        let bi = self.idx_pos(b);
        match (self.unit[ai], self.unit[bi]) {
            (Some(_), Some(_)) => {
                self.unit.swap(ai, bi);
                self.version = self.version.wrapping_add(1);
                Ok(())
            }
            _ => Err(GridError::FromEmpty),
        }
    }

    pub fn resize_clear(&mut self, w: u32, h: u32) {
        self.w = w; self.h = h;
        self.unit.clear();
        self.unit.resize((w as usize) * (h as usize), None);
        self.version = self.version.wrapping_add(1);
    }

    pub fn expand_preserve(&mut self, new_w: u32, new_h: u32) -> Result<(), GridError> {
        if new_w < self.w || new_h < self.h { return Err(GridError::ShrinkNotAllowed); }
        if new_w == self.w && new_h == self.h { return Ok(()); }
        let mut new_vec = vec![None; (new_w as usize) * (new_h as usize)];
        for y in 0..self.h {
            let old_base = (y as usize) * (self.w as usize);
            let new_base = (y as usize) * (new_w as usize);
            let width = self.w as usize;
            new_vec[new_base .. new_base + width]
                .copy_from_slice(&self.unit[old_base .. old_base + width]);
        }
        self.w = new_w; self.h = new_h; self.unit = new_vec;
        self.version = self.version.wrapping_add(1);
        Ok(())
    }

    /// (선택) 느린 편의 함수 — 디버그/드문 사용에만
    pub fn find(&self, e: Entity) -> Option<GridPos> {
        for (i, slot) in self.unit.iter().enumerate() {
            if *slot == Some(e) {
                let y = (i as u32) / self.w;
                let x = (i as u32) % self.w;
                return Some(GridPos { x, y });
            }
        }
        None
    }

    // ===== 래퍼: (x,y) -> GridPos 위임 =====
    #[inline] pub fn in_bounds(&self, x: u32, y: u32) -> bool {
        self.in_bounds_pos(GridPos::new(x, y))
    }
    #[inline] fn idx(&self, x: u32, y: u32) -> usize {
        self.idx_pos(GridPos::new(x, y))
    }
    #[inline] pub fn get(&self, x: u32, y: u32) -> Option<Entity> {
        self.get_pos(GridPos::new(x, y))
    }
    #[inline] pub fn is_empty(&self, x: u32, y: u32) -> bool {
        self.is_empty_pos(GridPos::new(x, y))
    }
    pub fn place(&mut self, x: u32, y: u32, e: Entity) -> Result<(), GridError> {
        self.place_pos(GridPos::new(x, y), e)
    }
    pub fn set(&mut self, x: u32, y: u32, v: Option<Entity>) -> Result<(), GridError> {
        self.set_pos(GridPos::new(x, y), v)
    }
    pub fn clear_cell(&mut self, x: u32, y: u32) -> Result<Option<Entity>, GridError> {
        self.clear_pos(GridPos::new(x, y))
    }
}
