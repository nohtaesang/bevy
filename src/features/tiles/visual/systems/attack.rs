//! Attack overlay systems
//!
//! This module handles attack range visualization

use bevy::prelude::*;
use std::collections::HashSet;
use crate::features::tiles::{
    core::{TileConfig, TileMap},
    selection::SelectionCtx,
    units::Unit,
    visual::{
        components::AttackOverlay,
        resources::{AttackValidation, AttackOverlayState},
    },
};

/// System that cleans up attack overlays when not in Attack state
pub fn cleanup_attack_overlays(
    mut overlay_query: Query<&mut Visibility, With<AttackOverlay>>,
    mut attack_validation: ResMut<AttackValidation>,
) {
    for mut visibility in overlay_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
    // Clear the validation cache
    attack_validation.clear();
}

/// System that updates AttackValidation when entering Attack mode
/// This runs once per Attack mode entry and sets up valid attack positions
pub fn update_attack_validation_on_enter(
    selection_ctx: Res<SelectionCtx>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    unit_query: Query<&Unit>,
    mut attack_validation: ResMut<AttackValidation>,
) {
    
    // Clear any existing validation
    attack_validation.clear();
    
    // If a unit is selected, calculate its valid attacks
    if let Some(unit_entity) = selection_ctx.selected_unit {
        if let Ok(unit) = unit_query.get(unit_entity) {
            
            if unit.attack_count > 0 {
                // Calculate valid attack positions
                // For now, just mark that attack is possible
                // TODO: Implement proper attack range calculation
                let valid_positions = vec![unit.tile_pos];
                
                let valid_attacks_set: HashSet<IVec2> = valid_positions.into_iter().collect();
                attack_validation.set_valid_attacks(valid_attacks_set.clone());
            }
        }
    }
}

/// System that updates attack overlays for selected unit in Attack action state
/// This system only runs when: PlayerTurn + Attack + UnitSelected
/// Note: AttackValidation is now updated separately in OnEnter system
pub fn attack_overlay_system(
    mut commands: Commands,
    selection_ctx: Res<SelectionCtx>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    unit_query: Query<&Unit>,
    mut overlay_query: Query<(Entity, &mut Visibility, &AttackOverlay)>,
    mut overlay_state: Local<AttackOverlayState>,
) {
    if let Some(unit_entity) = selection_ctx.selected_unit {
        if let Ok(unit) = unit_query.get(unit_entity) {
            // Check if we need to update overlays (position or attack count changed)
            let needs_update = overlay_state.last_unit_pos != Some(unit.tile_pos) ||
                               overlay_state.last_attack_count != Some(unit.attack_count);
            
            
            if needs_update {
                // Clear existing overlays
                for entity in overlay_state.current_overlays.drain(..) {
                    if let Ok((entity, _, _)) = overlay_query.get(entity) {
                        commands.entity(entity).despawn();
                    }
                }
                
                // Create new overlays if unit has attacks left
                if unit.attack_count > 0 {
                    // For now, just create empty overlay list
                    // TODO: Implement proper overlay creation
                    let new_overlays = Vec::new();
                    let valid_positions = vec![unit.tile_pos];
                    overlay_state.current_overlays = new_overlays;
                    overlay_state.valid_attacks = valid_positions.into_iter().collect();
                } else {
                    overlay_state.valid_attacks.clear();
                }
                
                // Update tracking state
                overlay_state.last_unit_pos = Some(unit.tile_pos);
                overlay_state.last_attack_count = Some(unit.attack_count);
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