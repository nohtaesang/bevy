//! Movement command systems

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::{
        tiles::{
            core::{TileConfig, TileMap, TileContent, MoveOutcome, tile_to_world_coords, world_to_tile_coords, TileMoved, Team, components::TileCoords},
            selection::{SelectionCtx, select_tile, select_unit, select_enemy, clear_selection},
            interaction::MovementValidation,
            units::bundles::UnitMarker,
        },
    },
};
use super::super::{
    components::{CommandQueue, PendingCommand, CommandType, CommandResult, CommandCompletedEvent},
    resources::{PathCache, CommandConfig},
};

#[derive(Debug)]
enum ClickTarget {
    SelfUnit,
    FriendlyUnit,
    Enemy,
    MovementOverlay,
    EmptyTile,
    OutsideGrid,
}

/// System that handles mouse clicks when in Move action state
pub fn handle_move_state_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    mut next_selection_state: ResMut<NextState<SelectionState>>,
    mut next_action_state: ResMut<NextState<UnitCommandState>>,
    mut selection_ctx: ResMut<SelectionCtx>,
    mut command_queue: ResMut<CommandQueue>,
    movement_validation: Res<MovementValidation>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };
    
    let click_target = determine_move_click_target(
        world_pos,
        &tile_config,
        &tile_map,
        &selection_ctx,
        &movement_validation,
    );

    handle_move_click_target(
        click_target,
        world_pos,
        &tile_config,
        &tile_map,
        &mut next_selection_state,
        &mut next_action_state,
        &mut selection_ctx,
        &mut command_queue,
    );
}

fn determine_move_click_target(
    world_pos: Vec2,
    tile_config: &TileConfig,
    tile_map: &TileMap,
    selection_ctx: &SelectionCtx,
    movement_validation: &MovementValidation,
) -> ClickTarget {
    let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) else {
        return ClickTarget::OutsideGrid;
    };
    
    let tile_pos = tile_coords.into();

    // First check if clicking on valid movement tile (highest priority in Move mode)
    let is_valid_move = movement_validation.is_valid_move(tile_pos);
    
    if is_valid_move {
        return ClickTarget::MovementOverlay;
    }

    // Then check what's at the clicked position
    if let Some(entity) = tile_map.get_entity(tile_pos) {
        if tile_map.has_unit(tile_pos) {
            if Some(entity) == selection_ctx.selected_unit {
                return ClickTarget::SelfUnit;
            } else {
                return ClickTarget::FriendlyUnit;
            }
        } else if tile_map.has_enemy(tile_pos) {
            return ClickTarget::Enemy;
        }
    }

    ClickTarget::EmptyTile
}

fn handle_move_click_target(
    click_target: ClickTarget,
    world_pos: Vec2,
    tile_config: &TileConfig,
    tile_map: &TileMap,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
    command_queue: &mut ResMut<CommandQueue>,
) {
    match click_target {
        ClickTarget::SelfUnit => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                if let Some(selected_unit) = selection_ctx.selected_unit {
                    select_unit(selected_unit, tile_coords.into(), next_selection_state, next_action_state, selection_ctx);
                }
            }
        },
        ClickTarget::FriendlyUnit => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                let tile_pos = tile_coords.into();
                if let TileContent::Unit(unit_entity) = tile_map.get_content(tile_pos) {
                    select_unit(unit_entity, tile_pos, next_selection_state, next_action_state, selection_ctx);
                }
            }
        },
        ClickTarget::Enemy => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                let tile_pos = tile_coords.into();
                if let TileContent::Enemy(enemy_entity) = tile_map.get_content(tile_pos) {
                    select_enemy(enemy_entity, tile_pos, next_selection_state, next_action_state, selection_ctx);
                }
            }
        },
        ClickTarget::MovementOverlay => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                let target_pos = tile_coords.into();
                queue_move_command(target_pos, next_action_state, selection_ctx, command_queue);
            }
        },
        ClickTarget::EmptyTile => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                let tile_pos = tile_coords.into();
                select_tile(tile_pos, next_selection_state, next_action_state, selection_ctx);
            }
        },
        ClickTarget::OutsideGrid => {
            clear_selection(next_selection_state, next_action_state, selection_ctx);
        },
    }
}

/// Queue a movement command instead of executing it immediately
fn queue_move_command(
    target_pos: IVec2,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
    command_queue: &mut ResMut<CommandQueue>,
) {
    if let (Some(selected_unit), Some(current_pos)) = (selection_ctx.selected_unit, selection_ctx.tile) {
        // Create movement command
        let command = PendingCommand::move_command(selected_unit, current_pos, target_pos);
        
        // Add to queue
        command_queue.add_command(command);
        
        // Update selection context to target position (optimistic update)
        selection_ctx.tile = Some(target_pos);
        
        // Return to idle action state
        next_action_state.set(UnitCommandState::Idle);
        
        info!("Queued movement command from {:?} to {:?}", current_pos, target_pos);
    }
}

/// Execute a movement command
pub fn execute_movement_command(
    entity: Entity,
    from: IVec2,
    to: IVec2,
    tile_config: &TileConfig,
    tile_map: &mut ResMut<TileMap>,
    path_cache: &mut ResMut<PathCache>,
    unit_query: &mut Query<(&mut TileCoords, &mut Transform, &Team), With<UnitMarker>>,
    command_events: &mut EventWriter<CommandCompletedEvent>,
    tile_moved: &mut EventWriter<TileMoved>,
) -> CommandResult {
    let Ok((mut tile_coords, mut transform, team)) = unit_query.get_mut(entity) else {
        return CommandResult::Failed { 
            reason: "Unit entity not found".to_string() 
        };
    };

    // For now, use a default movement range of 3
    // TODO: Get movement range from BaseStats or CurrentStats component
    let movement_range = 3;
    
    // Check cache first
    if let Some(cached_validity) = path_cache.get_path_validity(from, to, movement_range) {
        if !cached_validity {
            return CommandResult::Failed { 
                reason: "Cached path invalid".to_string() 
            };
        }
    } else {
        // Calculate and cache path validity
        // Simple distance check for now
        // TODO: Implement proper pathfinding
        let distance = (from.x - to.x).abs() + (from.y - to.y).abs();
        let is_valid = distance <= movement_range;
        path_cache.cache_path_validity(from, to, movement_range, is_valid);
        
        if !is_valid {
            return CommandResult::Failed { 
                reason: "Target position not reachable".to_string() 
            };
        }
    }

    // Calculate movement cost
    let distance = (from.x - to.x).abs() + (from.y - to.y).abs();
    
    if distance > movement_range {
        return CommandResult::Failed { 
            reason: "Insufficient movement range".to_string() 
        };
    }

    // Try to execute movement in TileMap first (SSOT for tile occupancy)
    match tile_map.move_unit(from, to) {
        MoveOutcome::Moved { entity: moved_entity } => {
            // TileMap update succeeded - now update other systems
            
            // Update tile coordinates
            tile_coords.x = to.x;
            tile_coords.y = to.y;
            // TODO: Update movement_range in CurrentStats component
            
            // Update visual position
            let world_pos = tile_to_world_coords(to.x, to.y, tile_config);
            transform.translation.x = world_pos.x;
            transform.translation.y = world_pos.y;
            
            // Emit event only after TileMap update succeeded (use actual team from component)
            tile_moved.send(TileMoved {
                entity: moved_entity,
                from,
                to,
                team: *team,
            });
            
            // Invalidate cache for affected positions
            path_cache.invalidate_position(from);
            path_cache.invalidate_position(to);
            
            info!("Executed movement from {:?} to {:?}", from, to);
            CommandResult::Success
        },
        MoveOutcome::Blocked => {
            CommandResult::Failed { 
                reason: "Target position is blocked".to_string() 
            }
        },
        MoveOutcome::OutOfBounds => {
            CommandResult::Failed { 
                reason: "Target position is out of bounds".to_string() 
            }
        },
        MoveOutcome::EmptyFrom => {
            CommandResult::Failed { 
                reason: "Source position is empty in tile map".to_string() 
            }
        },
    }
}