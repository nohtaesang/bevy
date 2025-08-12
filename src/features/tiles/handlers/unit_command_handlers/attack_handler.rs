//! Attack state click handler
//!
//! Handles clicks when in Attack action state

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::{
        tiles::SelectionCtx,
        tiles::{
            utils::world_to_tile_coords,
            actions::{select_tile, select_unit, select_enemy, clear_selection, perform_attack_action},
            TileMap, resources::TileContent,
            overlay::attack::AttackValidation,
        },
        units::{Unit, Enemy},
    },
    features::tiles::TileConfig,
};

/// System that handles mouse clicks when in Attack action state
pub fn handle_attack_state_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    attack_validation: Res<AttackValidation>,
    mut unit_query: Query<&mut Unit>,
    mut enemy_query: Query<&mut Enemy>,
    mut next_selection_state: ResMut<NextState<SelectionState>>,
    mut next_action_state: ResMut<NextState<UnitCommandState>>,
    mut selection_ctx: ResMut<SelectionCtx>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };
    
    let Some(tile_coords) = world_to_tile_coords(world_pos, &tile_config) else {
        clear_selection(&mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        return;
    };

    let tile_pos = tile_coords.into();

    // First check if this is a valid attack position
    if let Some(attacker_entity) = selection_ctx.selected_unit {
        let is_valid_attack = attack_validation.is_valid_attack(tile_pos);
        
        if is_valid_attack {
            // This is a valid attack position, check what's at the tile
            match tile_map.get_content(tile_pos) {
                TileContent::Enemy(enemy_entity) => {
                    // Attack the enemy
                    let attack_successful = perform_attack_action(
                        tile_pos,
                        attacker_entity,
                        enemy_entity,
                        &attack_validation,
                        &mut unit_query,
                        &mut enemy_query,
                        &mut next_selection_state,
                        &mut next_action_state,
                        &mut selection_ctx,
                    );
                    
                    if !attack_successful {
                        // If attack failed for some reason, select the enemy instead
                        select_enemy(enemy_entity, tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
                    }
                }
                _ => {
                    // Valid attack position but no enemy there - attack fails silently
                    info!("Attack attempted at {:?} but no enemy present", tile_pos);
                }
            }
            return; // Don't process normal tile selection logic
        } else {
            // Selected unit exists but tile is not a valid attack position
        }
    } else {
        // No unit selected
    }
    
    // Normal tile selection logic for non-attack scenarios
    match tile_map.get_content(tile_pos) {
        TileContent::Unit(entity) => {
            select_unit(entity, tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        }
        TileContent::Enemy(entity) => {
            select_enemy(entity, tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        }
        TileContent::Empty => {
            select_tile(tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        }
        TileContent::Obstacle => {
            // Do nothing for obstacles
        }
    }
}