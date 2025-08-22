// =============================================
// src/modes/battle/features/overlays/move_range/plugin.rs
// =============================================
use super::resources::{MoveRangeConfig, MoveRangeOverlay};
use super::systems_apply::recompute_move_range;
use super::systems_view::sync_move_range_overlay;
use crate::app::schedule::Phase;
use crate::app::state::{AppState, ModeState};
use bevy::prelude::*;

pub struct MoveRangeOverlayPlugin;
impl Plugin for MoveRangeOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MoveRangeOverlay>()
            .init_resource::<MoveRangeConfig>()
            .add_systems(
                Update,
                recompute_move_range
                    .in_set(Phase::Apply) // 계산은 Apply에서
                    .run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
            )
            .add_systems(
                Update,
                sync_move_range_overlay
                    .in_set(Phase::ViewSync) // 렌더는 ViewSync
                    .run_if(in_state(AppState::InGame).and(in_state(ModeState::Battle))),
            );
    }
}
