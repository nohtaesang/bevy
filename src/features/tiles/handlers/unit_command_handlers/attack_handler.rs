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
            TileMap, resources::TileContent,
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

    // Check what's at the clicked tile using TileMap
    match tile_map.get_content(tile_pos) {
        TileContent::Unit(entity) => {
            select_unit(entity, tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        }
        TileContent::Enemy(entity) => {
            // TODO: Implement attack action here
            println!("Attack enemy at {:?} - not implemented yet", tile_pos);
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