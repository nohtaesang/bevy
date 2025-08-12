//! Attack overlay system

use bevy::prelude::*;
use std::collections::HashSet;
use crate::{
    core::SelectionCtx,
    features::units::Unit,
    resources::{TileConfig, TileMap},
};
use super::{AttackOverlay, create_attack_overlay_sprites};

#[derive(Default)]
pub struct AttackOverlayState {
    current_overlays: Vec<Entity>,
    valid_attacks: HashSet<IVec2>,  // Cache valid attack positions
    last_unit_pos: Option<IVec2>,
    last_attack_count: Option<i32>,
}

/// System that cleans up attack overlays when not in Attack state
pub fn cleanup_attack_overlays(
    mut overlay_query: Query<&mut Visibility, With<AttackOverlay>>,
    mut attack_validation: ResMut<super::AttackValidation>,
) {
    for mut visibility in overlay_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
    // Clear the validation cache
    attack_validation.clear();
}

/// System that updates attack overlays for selected unit in Attack action state
/// This system only runs when: PlayerTurn + Attack + UnitSelected
pub fn attack_overlay_system(
    mut commands: Commands,
    selection_ctx: Res<SelectionCtx>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    unit_query: Query<&Unit>,
    mut overlay_query: Query<(Entity, &mut Visibility, &AttackOverlay)>,
    mut overlay_state: Local<AttackOverlayState>,
    mut attack_validation: ResMut<super::AttackValidation>,
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
                    let (new_overlays, valid_positions) = create_attack_overlay_sprites(
                        &mut commands,
                        &tile_config,
                        &tile_map,
                        unit,
                    );
                    overlay_state.current_overlays = new_overlays;
                    let valid_attacks_set: HashSet<IVec2> = valid_positions.into_iter().collect();
                    overlay_state.valid_attacks = valid_attacks_set.clone();
                    attack_validation.set_valid_attacks(valid_attacks_set);
                } else {
                    overlay_state.valid_attacks.clear();
                    attack_validation.clear();
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