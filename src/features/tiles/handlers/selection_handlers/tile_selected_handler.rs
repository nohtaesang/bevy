use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::{
        tiles::{SelectionCtx, TileConfig, TileMap, utils::world_to_tile_coords, resources::TileContent},
        units::{Unit, Enemy},
    },
};
use crate::features::tiles::actions::{clear_selection, select_tile, select_unit, select_enemy};


/// Handle clicking different unit when tile is selected
fn handle_unit_click_when_tile_selected(
    entity: Entity,
    tile_pos: IVec2,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    select_unit(entity, tile_pos, next_selection_state, next_action_state, selection_ctx);
}

/// Handle clicking enemy when tile is selected
fn handle_enemy_click_when_tile_selected(
    entity: Entity,
    tile_pos: IVec2,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    select_enemy(entity, tile_pos, next_selection_state, next_action_state, selection_ctx);
}

/// Handle clicking different empty tile when tile is selected
fn handle_empty_tile_click_when_tile_selected(
    tile_pos: IVec2,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    select_tile(tile_pos, next_selection_state, next_action_state, selection_ctx);
}

/// Handle clicking outside grid when tile is selected
fn handle_outside_grid_click_when_tile_selected(
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    clear_selection(next_selection_state, next_action_state, selection_ctx);
}

/// System that handles tile clicks when a tile is selected
pub fn handle_tile_selected_click(
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
        handle_outside_grid_click_when_tile_selected(
            &mut next_selection_state,
            &mut next_action_state,
            &mut selection_ctx,
        );
        return;
    };
    
    let tile_pos = IVec2::new(tile_coords.0, tile_coords.1);
    
    // Note: Removed self-deselection behavior - clicking same tile now does nothing
    
    // Check what's at the clicked tile using TileMap
    match tile_map.get_content(tile_pos) {
        TileContent::Unit(entity) => {
            handle_unit_click_when_tile_selected(
                entity,
                tile_pos,
                &mut next_selection_state,
                &mut next_action_state,
                &mut selection_ctx,
            );
        }
        TileContent::Enemy(entity) => {
            handle_enemy_click_when_tile_selected(
                entity,
                tile_pos,
                &mut next_selection_state,
                &mut next_action_state,
                &mut selection_ctx,
            );
        }
        TileContent::Empty => {
            handle_empty_tile_click_when_tile_selected(
                tile_pos,
                &mut next_selection_state,
                &mut next_action_state,
                &mut selection_ctx,
            );
        }
        TileContent::Obstacle => {
            // Do nothing for obstacles
        }
    }
}