// =============================================
// src/domain/units/events.rs
// =============================================
use super::components::TeamId;
use crate::domain::map::grid_index::GridPos;
use bevy::prelude::*;

/// U/I 입력으로 스폰 요청
#[derive(Event, Clone, Copy, Debug)]
pub struct UnitSpawnRequested {
    pub team: TeamId,
    pub at: GridPos,
}

/// 스폰 성공(도메인 엔티티 확정)
#[derive(Event, Clone, Copy, Debug)]
pub struct UnitSpawnApplied {
    pub entity: Entity,
    pub team: TeamId,
    pub at: GridPos,
}
