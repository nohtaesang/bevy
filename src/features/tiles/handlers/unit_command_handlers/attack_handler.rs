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
            actions::{select_tile, select_unit, select_enemy, clear_selection},
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
    mut next_selection_state: ResMut<NextState<SelectionState>>,
    mut next_action_state: ResMut<NextState<UnitCommandState>>,
    mut selection_ctx: ResMut<SelectionCtx>,
    unit_query: Query<(Entity, &Unit)>,
    enemy_query: Query<(Entity, &Enemy)>,
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

    // Check for unit at clicked position
    for (unit_entity, unit) in unit_query.iter() {
        if unit.tile_pos == tile_pos {
            select_unit(unit_entity, tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
            return;
        }
    }

    // Check for enemy at clicked position - this would be attack target
    for (enemy_entity, enemy) in enemy_query.iter() {
        if enemy.tile_pos == tile_pos {
            // TODO: Implement attack action here
            println!("Attack enemy at {:?} - not implemented yet", tile_pos);
            select_enemy(enemy_entity, tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
            return;
        }
    }

    // Empty tile - select it
    select_tile(tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
}