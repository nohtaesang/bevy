//! Input handling systems for selection
//!
//! This module handles mouse and keyboard input specifically for selection functionality

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::tiles::{
        core::{TileConfig, TileMap, TileContent, world_to_tile_coords},
        units::{Unit, Enemy},
        selection::{SelectionCtx, systems::{state::*, effects::clear_selection}},
    },
};

/// System that handles mouse clicks when in Idle selection state
pub fn handle_idle_state_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
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
        clear_selection(&mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        return;
    };
    
    let tile_pos = IVec2::new(tile_coords.0, tile_coords.1);
    
    // Check what's at the clicked tile using TileMap
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

/// System that handles tile clicks when a unit is selected
pub fn handle_unit_selected_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    action_state: Res<State<UnitCommandState>>,
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
    
    let tile_pos = IVec2::new(tile_coords.0, tile_coords.1);
    
    let Some(selected_tile) = selection_ctx.tile else { return; };
    
    if tile_pos == selected_tile {
        // Clicking same tile - do nothing
    } else {
        // Check what's at the clicked tile using TileMap
        match tile_map.get_content(tile_pos) {
            TileContent::Unit(entity) => {
                select_unit(entity, tile_pos, &mut next_selection_state, &mut next_action_state, &mut selection_ctx);
                return;
            }
            _ => {
                // Continue to action handling for non-unit tiles
            }
        }
        
        // Clicking different tile - execute action based on current action state
        match action_state.get() {
            UnitCommandState::Move => {
                // Update unit position and return to idle
                selection_ctx.tile = Some(tile_pos);
                next_action_state.set(UnitCommandState::Idle);
            },
            UnitCommandState::Attack => {
                // Keep unit position and return to idle action
                next_action_state.set(UnitCommandState::Idle);
            },
            UnitCommandState::Idle => {
                selection_ctx.tile = Some(tile_pos);
                selection_ctx.selected_unit = None;
                selection_ctx.selected_enemy = None;
                next_selection_state.set(SelectionState::TileSelected);
            },
        }
    }
}

/// System that handles tile clicks when an enemy is selected
pub fn handle_enemy_selected_click(
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
    
    let tile_pos = IVec2::new(tile_coords.0, tile_coords.1);
    
    // Check what's at the clicked tile using TileMap
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

/// Handle keyboard input for unit actions when a unit is selected
pub fn handle_unit_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_action_state: ResMut<NextState<UnitCommandState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        next_action_state.set(UnitCommandState::Move);
    }
    
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        next_action_state.set(UnitCommandState::Attack);
    }
}