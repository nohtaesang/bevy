//! Selection action functions
//!
//! Reusable functions for selecting tiles, units, and enemies

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::tiles::SelectionCtx,
};

/// Select a tile and update game state
pub fn select_tile(
    tile_pos: IVec2,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    selection_ctx.tile = Some(tile_pos);
    selection_ctx.selected_unit = None;
    selection_ctx.selected_enemy = None;
    next_selection_state.set(SelectionState::TileSelected);
    next_action_state.set(UnitCommandState::Idle);
}

/// Select a unit and update game state
pub fn select_unit(
    unit_entity: Entity,
    tile_pos: IVec2,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    selection_ctx.tile = Some(tile_pos);
    selection_ctx.selected_unit = Some(unit_entity);
    selection_ctx.selected_enemy = None;
    next_selection_state.set(SelectionState::UnitSelected);
    next_action_state.set(UnitCommandState::Idle);
}

/// Select an enemy and update game state
pub fn select_enemy(
    enemy_entity: Entity,
    tile_pos: IVec2,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    selection_ctx.tile = Some(tile_pos);
    selection_ctx.selected_unit = None;
    selection_ctx.selected_enemy = Some(enemy_entity);
    next_selection_state.set(SelectionState::EnemySelected);
    next_action_state.set(UnitCommandState::Idle);
}

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