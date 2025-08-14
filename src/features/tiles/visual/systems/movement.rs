//! Movement overlay systems
//!
//! This module handles movement range visualization

use bevy::prelude::*;
use std::collections::HashSet;
use crate::features::tiles::{
    core::{TileConfig, TileMap},
    selection::SelectionCtx,
    units::Unit,
    visual::{
        components::MovementOverlay,
        resources::{MovementValidation, MovementOverlayState},
    },
};

/// System that cleans up movement overlays when not in Move state
pub fn cleanup_movement_overlays(
    mut overlay_query: Query<&mut Visibility, With<MovementOverlay>>,
    mut movement_validation: ResMut<MovementValidation>,
) {
    for mut visibility in overlay_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
    // Clear the validation cache
    movement_validation.clear();
}

/// System that updates MovementValidation when entering Move mode
/// This runs once per Move mode entry and sets up valid movement positions
pub fn update_movement_validation_on_enter(
    selection_ctx: Res<SelectionCtx>,
    _tile_config: Res<TileConfig>,
    _tile_map: Res<TileMap>,
    unit_query: Query<&Unit>,
    mut movement_validation: ResMut<MovementValidation>,
) {
    
    // Clear any existing validation
    movement_validation.clear();
    
    // If a unit is selected, validate it has movement
    if let Some(unit_entity) = selection_ctx.selected_unit {
        if let Ok(unit) = unit_query.get(unit_entity) {
            if unit.movement_range > 0 {
                // For now, just mark that movement is possible
                // TODO: Implement proper pathfinding validation
                let mut valid_moves = std::collections::HashSet::new();
                valid_moves.insert(unit.tile_pos);
                movement_validation.set_valid_moves(valid_moves);
            }
        }
    }
}

/// System that updates movement overlays for selected unit in Move action state
/// This system only runs when: PlayerTurn + Move + UnitSelected
/// Note: MovementValidation is now updated separately in OnEnter system
pub fn movement_overlay_system(
    _commands: Commands,
    _selection_ctx: Res<SelectionCtx>,
    _tile_config: Res<TileConfig>,
    _tile_map: Res<TileMap>,
    _unit_query: Query<&Unit>,
    mut overlay_query: Query<&mut Visibility, With<MovementOverlay>>,
) {
    // Simple system that just shows movement overlays
    for mut visibility in overlay_query.iter_mut() {
        *visibility = Visibility::Visible;
    }
}