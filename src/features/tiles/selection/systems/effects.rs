//! Selection visual effects and event systems
//!
//! This module handles visual triggers and events when selections occur

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::tiles::selection::SelectionCtx,
};

/// Clear all selections
pub fn clear_selection(
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    selection_ctx.tile = None;
    selection_ctx.selected_unit = None;
    selection_ctx.selected_enemy = None;
    next_selection_state.set(SelectionState::Idle);
    next_action_state.set(UnitCommandState::Idle);
}