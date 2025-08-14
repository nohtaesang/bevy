//! Attack command systems

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::{
        tiles::{
            core::{TileConfig, TileMap, TileContent, world_to_tile_coords},
            selection::{SelectionCtx, select_tile, select_unit, select_enemy, clear_selection},
            visual::AttackValidation,
        },
        tiles::units::{Unit, Enemy},
    },
};
use super::super::{
    components::{CommandQueue, PendingCommand, CommandType, CommandResult, CommandCompletedEvent},
    resources::CommandConfig,
};

/// System that handles mouse clicks when in Attack action state
pub fn handle_attack_state_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    attack_validation: Res<AttackValidation>,
    mut next_selection_state: ResMut<NextState<SelectionState>>,
    mut next_action_state: ResMut<NextState<UnitCommandState>>,
    mut selection_ctx: ResMut<SelectionCtx>,
    mut command_queue: ResMut<CommandQueue>,
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
                    // Queue attack command
                    queue_attack_command(
                        attacker_entity,
                        enemy_entity,
                        tile_pos,
                        &mut next_action_state,
                        &mut selection_ctx,
                        &mut command_queue,
                    );
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

/// Queue an attack command instead of executing it immediately
fn queue_attack_command(
    attacker_entity: Entity,
    target_entity: Entity,
    target_pos: IVec2,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
    command_queue: &mut ResMut<CommandQueue>,
) {
    if let Some(attacker_pos) = selection_ctx.tile {
        // Create attack command
        let command = PendingCommand::attack_command(attacker_entity, attacker_pos, target_pos);
        
        // Add to queue
        command_queue.add_command(command);
        
        // Return to idle action state
        next_action_state.set(UnitCommandState::Idle);
        
        info!("Queued attack command from {:?} to {:?}", attacker_pos, target_pos);
    }
}

/// Execute an attack command
pub fn execute_attack_command(
    attacker_entity: Entity,
    from: IVec2,
    target_pos: IVec2,
    tile_map: &TileMap,
    attack_validation: &AttackValidation,
    unit_query: &mut Query<&mut Unit>,
    enemy_query: &mut Query<&mut Enemy>,
    command_events: &mut EventWriter<CommandCompletedEvent>,
) -> CommandResult {
    // Validate attack position
    if !attack_validation.is_valid_attack(target_pos) {
        return CommandResult::Failed { 
            reason: "Invalid attack position".to_string() 
        };
    }

    // Get the attacking unit
    let mut attacker = match unit_query.get_mut(attacker_entity) {
        Ok(unit) => unit,
        Err(_) => {
            return CommandResult::Failed { 
                reason: "Attacker entity not found".to_string() 
            };
        }
    };

    // Check if unit has attacks left
    if !attacker.can_attack() {
        return CommandResult::Failed { 
            reason: "No attacks remaining".to_string() 
        };
    }

    // Get the target enemy entity from the tile map
    let target_entity = match tile_map.get_content(target_pos) {
        TileContent::Enemy(entity) => entity,
        _ => {
            return CommandResult::Failed { 
                reason: "No enemy at target position".to_string() 
            };
        }
    };

    // Get the target enemy
    let mut target_enemy = match enemy_query.get_mut(target_entity) {
        Ok(enemy) => enemy,
        Err(_) => {
            return CommandResult::Failed { 
                reason: "Target enemy not found".to_string() 
            };
        }
    };

    // Consume attack count
    attacker.use_attack();
    
    // Deal damage
    let damage = attacker.attack_power;
    target_enemy.health -= damage;
    
    // Ensure health doesn't go below 0
    if target_enemy.health < 0 {
        target_enemy.health = 0;
    }

    info!(
        "Unit at {:?} attacked enemy at {:?} for {} damage. Enemy health: {}/{}",
        from,
        target_pos,
        damage,
        target_enemy.health,
        target_enemy.max_health
    );

    CommandResult::Success
}