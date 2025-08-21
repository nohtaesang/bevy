

// =============================================
// src/modes/battle/features/spawner_debug/plugin.rs
// =============================================
use bevy::prelude::*;
use crate::app::state::{AppState, ModeState};
use crate::app::schedule::Phase;
use crate::domain::units::events::{UnitSpawnRequested, UnitSpawnApplied};
use super::systems_input::emit_spawn_requests_from_input;
use super::systems_apply::apply_unit_spawns;
use super::systems_view::spawn_unit_sprites_on_applied;


pub struct SpawnerDebugPlugin;
impl Plugin for SpawnerDebugPlugin {
fn build(&self, app: &mut App) {
app.add_event::<UnitSpawnRequested>()
.add_event::<UnitSpawnApplied>()
// Input: 키 입력 → 요청 이벤트
.add_systems(
Update,
emit_spawn_requests_from_input
.in_set(Phase::Input)
.run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
)
// Apply/Publish: SSOT(Occupancy) 검증→스폰→Applied 발행
.add_systems(
Update,
apply_unit_spawns
.in_set(Phase::Apply)
.run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
)
// ViewSync: 스프라이트 생성(도메인 엔티티와 분리)
.add_systems(
Update,
spawn_unit_sprites_on_applied
.in_set(Phase::ViewSync)
.run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
);
}
}