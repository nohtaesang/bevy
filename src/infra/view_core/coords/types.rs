// src/infra/view_core/coords/types.rs
//! 그리드 좌표계 타입 정의 및 변환 유틸리티

use crate::domain::map::grid_index::GridPos;
use bevy::prelude::*;

/// 전역 그리드 기하(SSOT가 아님 — 뷰 설정의 스냅샷)
/// - `origin`: 그리드 (0,0)의 월드 좌표(좌하단 기준)
/// - `cell`: 한 칸의 월드 크기 (폭, 높이)
/// - `size`: 그리드 가로/세로 칸 수
#[derive(Resource, Clone, Copy, Debug)]
pub struct GridGeometry {
    pub origin: Vec2,
    pub cell: Vec2,
    pub size: UVec2,
}

impl Default for GridGeometry {
    fn default() -> Self {
        Self {
            origin: Vec2::ZERO,
            cell: Vec2::ZERO,
            size: UVec2::ZERO,
        }
    }
}

impl GridGeometry {
    /// 현재 설정이 유효한지(0이 아닌지)
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.cell.x > 0.0 && self.cell.y > 0.0 && self.size.x > 0 && self.size.y > 0
    }

    /// 그리드 좌표가 범위 내인지
    #[inline]
    pub fn in_bounds(&self, gp: GridPos) -> bool {
        gp.x < self.size.x && gp.y < self.size.y
    }

    /// 월드 좌표 → 그리드 좌표. 범위 밖이면 None
    #[inline]
    pub fn world_to_grid(&self, world: Vec2) -> Option<GridPos> {
        if !self.is_ready() {
            return None;
        }
        let rel = world - self.origin;
        if rel.x < 0.0 || rel.y < 0.0 {
            return None;
        }
        let gx = (rel.x / self.cell.x).floor() as i64;
        let gy = (rel.y / self.cell.y).floor() as i64;
        if gx < 0 || gy < 0 {
            return None;
        }
        let gp = GridPos {
            x: gx as u32,
            y: gy as u32,
        };
        if self.in_bounds(gp) {
            Some(gp)
        } else {
            None
        }
    }

    /// 그리드 중심의 월드 좌표
    #[inline]
    pub fn grid_center(&self, gp: GridPos) -> Vec2 {
        self.origin
            + Vec2::new(
                gp.x as f32 * self.cell.x + self.cell.x * 0.5,
                gp.y as f32 * self.cell.y + self.cell.y * 0.5,
            )
    }

    /// 그리드 셀의 월드 AABB(min,max)
    #[inline]
    pub fn grid_cell_aabb(&self, gp: GridPos) -> (Vec2, Vec2) {
        let min = self.origin + Vec2::new(gp.x as f32 * self.cell.x, gp.y as f32 * self.cell.y);
        let max = min + self.cell;
        (min, max)
    }

    /// 전체 그리드의 월드 AABB(min,max)
    #[inline]
    pub fn world_aabb(&self) -> (Vec2, Vec2) {
        let max = self.origin
            + Vec2::new(
                self.size.x as f32 * self.cell.x,
                self.size.y as f32 * self.cell.y,
            );
        (self.origin, max)
    }
}

/// 뷰 설정으로부터 GridGeometry를 생성하는 헬퍼
#[inline]
pub fn make_grid_geometry(origin: Vec2, cell: Vec2, size: UVec2) -> GridGeometry {
    GridGeometry { origin, cell, size }
}
