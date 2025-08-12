use bevy::prelude::*;
use crate::{
    core::{SelectionState, ActionState, SelectionCtx},
    features::{
        tiles::utils::world_to_tile_coords,
        units::{Unit, Enemy},
    },
    resources::TileConfig,
};
use crate::features::tiles::actions::{clear_selection, select_unit};

/// Handle move action when different tile clicked
fn handle_move_action_when_unit_selected(
    from_tile: IVec2,
    to_tile: IVec2,
    next_action_state: &mut ResMut<NextState<ActionState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    println!("Unit moving from ({}, {}) to ({}, {})", from_tile.x, from_tile.y, to_tile.x, to_tile.y);
    // TODO: Check if target tile is valid for movement
    // TODO: Execute actual move command
    
    // Update unit position and return to idle
    selection_ctx.tile = Some(to_tile);
    next_action_state.set(ActionState::Idle);
}

/// Handle attack action when different tile clicked  
fn handle_attack_action_when_unit_selected(
    from_tile: IVec2,
    to_tile: IVec2,
    next_action_state: &mut ResMut<NextState<ActionState>>,
) {
    println!("Unit attacking from ({}, {}) to ({}, {})", from_tile.x, from_tile.y, to_tile.x, to_tile.y);
    // TODO: Check if target tile has enemy for attack
    // TODO: Execute actual attack command
    
    // Keep unit position and return to idle action
    next_action_state.set(ActionState::Idle);
}

/// Handle clicking different unit when unit is selected
fn handle_different_unit_click_when_unit_selected(
    entity: Entity,
    tile_pos: IVec2,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<ActionState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    select_unit(entity, tile_pos, next_selection_state, next_action_state, selection_ctx);
}

/// Handle idle action when different tile clicked - change selection
fn handle_idle_action_when_unit_selected(
    tile_pos: IVec2,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    println!("Unit selected: clicked different tile ({}, {}) - changing selection", tile_pos.x, tile_pos.y);
    selection_ctx.tile = Some(tile_pos);
    selection_ctx.selected_unit = None;
    selection_ctx.selected_enemy = None;
    next_selection_state.set(SelectionState::TileSelected);
}

/// Handle clicking outside grid when unit selected
fn handle_outside_grid_click_when_unit_selected(
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<ActionState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
) {
    clear_selection(next_selection_state, next_action_state, selection_ctx);
}

/// System that handles tile clicks when a unit is selected
pub fn handle_unit_selected_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    action_state: Res<State<ActionState>>,
    mut next_selection_state: ResMut<NextState<SelectionState>>,
    mut next_action_state: ResMut<NextState<ActionState>>,
    mut selection_ctx: ResMut<SelectionCtx>,
    unit_query: Query<(Entity, &Unit)>,
    _enemy_query: Query<(Entity, &Enemy)>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };
    
    let Some(tile_coords) = world_to_tile_coords(world_pos, &tile_config) else {
        handle_outside_grid_click_when_unit_selected(
            &mut next_selection_state,
            &mut next_action_state,
            &mut selection_ctx,
        );
        return;
    };
    
    let tile_pos = IVec2::new(tile_coords.0, tile_coords.1);
    
    let Some(selected_tile) = selection_ctx.tile else { return; };
    
    if tile_pos == selected_tile {
       
    } else {
        // Check for unit at tile
        for (entity, unit) in unit_query.iter() {
            if unit.tile_pos == tile_pos {
                handle_different_unit_click_when_unit_selected(
                    entity,
                    tile_pos,
                    &mut next_selection_state,
                    &mut next_action_state,
                    &mut selection_ctx,
                );
                return;
            }
        }
        
        // Clicking different tile - execute action based on current action state
        match action_state.get() {
            ActionState::Move => {
                handle_move_action_when_unit_selected(
                    selected_tile,
                    tile_pos,
                    &mut next_action_state,
                    &mut selection_ctx,
                );
            },
            ActionState::Attack => {
                handle_attack_action_when_unit_selected(
                    selected_tile,
                    tile_pos,
                    &mut next_action_state,
                );
            },
            ActionState::Idle => {
                handle_idle_action_when_unit_selected(
                    tile_pos,
                    &mut next_selection_state,
                    &mut selection_ctx,
                );
            },
        }
    }
}