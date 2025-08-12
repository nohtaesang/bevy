//! Movement overlay system

use bevy::prelude::*;
use crate::{
    core::SelectionCtx,
    features::units::{Unit, Enemy},
    resources::TileConfig,
};
use super::{MovementOverlay, create_movement_overlay_sprites};

#[derive(Default)]
pub struct MovementOverlayState {
    current_overlays: Vec<Entity>,
    last_unit_pos: Option<IVec2>,
    last_movement_range: Option<i32>,
}

/// System that cleans up movement overlays when not in Move state
pub fn cleanup_movement_overlays(
    mut overlay_query: Query<&mut Visibility, With<MovementOverlay>>,
) {
    for mut visibility in overlay_query.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

/// System that updates movement overlays for selected unit in Move action state
/// This system only runs when: PlayerTurn + Move + UnitSelected
pub fn movement_overlay_system(
    mut commands: Commands,
    selection_ctx: Res<SelectionCtx>,
    tile_config: Res<TileConfig>,
    unit_query: Query<&Unit>,
    enemy_query: Query<&Enemy>,
    mut overlay_query: Query<(Entity, &mut Visibility, &MovementOverlay)>,
    mut overlay_state: Local<MovementOverlayState>,
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
                    let new_overlays = create_movement_overlay_sprites(
                        &mut commands,
                        &tile_config,
                        unit.tile_pos,
                        unit.movement_range,
                        &unit_query,
                        &enemy_query,
                    );
                    overlay_state.current_overlays = new_overlays;
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