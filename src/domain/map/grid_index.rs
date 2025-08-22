use super::components::MapSize;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct GridPos {
    pub x: u32,
    pub y: u32,
}

/// SSOT: 타일 점유 인덱스 (유닛/장애물 등 한 칸 1개 가정)
#[derive(Resource)]
pub struct OccupancyIndex {
    pub size: MapSize,
    pub slots: Vec<Option<Entity>>, // len = w*h
}
impl OccupancyIndex {
    pub fn new(size: MapSize) -> Self {
        Self {
            size,
            slots: vec![None; size.area()],
        }
    }
    #[inline]
    fn idx(&self, p: GridPos) -> usize {
        (p.x + p.y * self.size.w) as usize
    }
    #[inline]
    pub fn in_bounds(&self, p: GridPos) -> bool {
        p.x < self.size.w && p.y < self.size.h
    }
    pub fn occupant(&self, p: GridPos) -> Option<Entity> {
        let i = (p.x + p.y * self.size.w) as usize;
        self.slots[i]
    }
    pub fn is_free(&self, p: GridPos) -> bool {
        self.in_bounds(p) && self.slots[self.idx(p)].is_none()
    }
    pub fn claim(&mut self, p: GridPos, e: Entity) -> bool {
        if !self.is_free(p) {
            return false;
        }
        let idx = self.idx(p);
        self.slots[idx] = Some(e);
        true
    }
    pub fn release(&mut self, p: GridPos) {
        if self.in_bounds(p) {
            let idx = self.idx(p);
            self.slots[idx] = None;
        }
    }
    pub fn try_move(&mut self, from: GridPos, to: GridPos, e: Entity) -> bool {
        match (self.idx(from), self.idx(to)) {
            (fi, ti) if self.slots[fi] == Some(e) && self.slots[ti].is_none() => {
                self.slots[fi] = None;
                self.slots[ti] = Some(e);
                true
            }
            _ => false,
        }
    }
}
