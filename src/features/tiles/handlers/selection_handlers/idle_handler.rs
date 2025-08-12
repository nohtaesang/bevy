//! Idle state click handler
//!
//! Handles clicks when in Idle selection state

use bevy::prelude::*;
use crate::{
    core::{SelectionState, ActionState, SelectionCtx},
    features::{
        tiles::{
            utils::world_to_tile_coords,
            actions::{select_tile, select_unit, select_enemy, clear_selection},
        },
        units::{Unit, Enemy},
    },
    resources::{TileConfig, TileMap},
};

/// System that handles mouse clicks when in Idle selection state
pub fn handle_idle_state_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    mut next_selection_state: ResMut<NextState<SelectionState>>,
    mut next_action_state: ResMut<NextState<ActionState>>,
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

    // Use TileMap to check what's at the clicked position
    if let Some(entity) = tile_map.get_entity(tile_pos) {
        if tile_map.has_unit(tile_pos) {
            select_unit(entity, tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
            return;
        } else if tile_map.has_enemy(tile_pos) {
            select_enemy(entity, tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
            return;
        }
    }

    // Empty tile - select it
    select_tile(tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
}