//! Movement overlay system

use bevy::prelude::*;
use std::collections::HashSet;
use crate::{
    features::{tiles::{SelectionCtx, TileConfig, TileMap}, units::Unit},
};
use super::{MovementOverlay, create_movement_overlay_sprites};

#[derive(Default)]
pub struct MovementOverlayState {
    current_overlays: Vec<Entity>,
    valid_moves: HashSet<IVec2>,  // Cache valid move positions
    last_unit_pos: Option<IVec2>,
    last_movement_range: Option<i32>,
}

/// System that cleans up movement overlays when not in Move state
pub fn cleanup_movement_overlays(
    mut overlay_query: Query<&mut Visibility, With<MovementOverlay>>,
    mut movement_validation: ResMut<super::MovementValidation>,
) {
    for mut visibility in overlay_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
    // Clear the validation cache
    movement_validation.clear();
}

/// System that updates movement overlays for selected unit in Move action state
/// This system only runs when: PlayerTurn + Move + UnitSelected
pub fn movement_overlay_system(
    mut commands: Commands,
    selection_ctx: Res<SelectionCtx>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    unit_query: Query<&Unit>,
    mut overlay_query: Query<(Entity, &mut Visibility, &MovementOverlay)>,
    mut overlay_state: Local<MovementOverlayState>,
    mut movement_validation: ResMut<super::MovementValidation>,
) {
    if let Some(unit_entity) = selection_ctx.selected_unit {
        if let Ok(unit) = unit_query.get(unit_entity) {
            // Check if we need to update overlays (position or movement range changed)
            let needs_update = overlay_state.last_unit_pos != Some(unit.tile_pos) ||
                               overlay_state.last_movement_range != Some(unit.movement_range);
            
            if needs_update {
                // Clear existing overlays
                for entity in overlay_state.current_overlays.drain(..) {
                    if let Ok((entity, _, _)) = overlay_query.get(entity) {
                        commands.entity(entity).despawn();
                    }
                }
                
                // Create new overlays if unit has movement range left
                if unit.movement_range > 0 {
                    let (new_overlays, valid_positions) = create_movement_overlay_sprites(
                        &mut commands,
                        &tile_config,
                        &tile_map,
                        unit.tile_pos,
                        unit.movement_range,
                    );
                    overlay_state.current_overlays = new_overlays;
                    let valid_moves_set: HashSet<IVec2> = valid_positions.into_iter().collect();
                    overlay_state.valid_moves = valid_moves_set.clone();
                    movement_validation.set_valid_moves(valid_moves_set);
                } else {
                    overlay_state.valid_moves.clear();
                    movement_validation.clear();
                }
                
                // Update tracking state
                overlay_state.last_unit_pos = Some(unit.tile_pos);
                overlay_state.last_movement_range = Some(unit.movement_range);
            }
            
            // Show all current overlays
            for &entity in overlay_state.current_overlays.iter() {
                if let Ok((_, mut visibility, _)) = overlay_query.get_mut(entity) {
                    *visibility = Visibility::Visible;
                }
            }
        }
    }
}