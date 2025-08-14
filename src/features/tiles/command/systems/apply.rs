//! Command application system - single entry point for command execution

use bevy::{prelude::*, ecs::system::ParamSet};
use crate::features::tiles::{
    core::{TileConfig, TileMap, TileMoved, Team, components::TileCoords},
    interaction::AttackValidation,
    units::bundles::UnitMarker,
};
use super::{
    super::{
        components::{
            CommandQueue, PendingCommand, CommandType, CommandResult, 
            CommandCompletedEvent, ExecutingCommand
        },
        resources::{CommandConfig, CommandStats, CommandSystemState, PathCache},
    },
    movement::execute_movement_command,
    attack::execute_attack_command,
};

/// Main command processing system - single entry point for all command execution
pub fn process_command_queue(
    mut commands: Commands,
    mut command_queue: ResMut<CommandQueue>,
    mut command_stats: ResMut<CommandStats>,
    mut command_state: ResMut<CommandSystemState>,
    command_config: Res<CommandConfig>,
    tile_config: Res<TileConfig>,
    mut tile_map: ResMut<TileMap>,
    mut path_cache: ResMut<PathCache>,
    attack_validation: Res<AttackValidation>,
    mut unit_queries: ParamSet<(
        Query<(&mut TileCoords, &mut Transform, &Team), With<UnitMarker>>,
        Query<&Team, With<UnitMarker>>,
    )>,
    mut command_events: EventWriter<CommandCompletedEvent>,
    mut tile_moved: EventWriter<TileMoved>,
    time: Res<Time>,
) {
    // Skip processing if paused
    if command_state.paused {
        return;
    }

    // Update frame count
    command_state.frame_count += 1;

    // Process commands up to the frame limit
    let mut commands_processed = 0;
    let max_commands = command_config.max_commands_per_frame;
    
    while commands_processed < max_commands && !command_queue.is_empty() {
        let Some(command) = command_queue.pop_command() else { break; };
        
        let start_time = time.elapsed_secs_f64();
        
        // Execute the command
        let result = execute_command(
            &command,
            &command_config,
            &tile_config,
            &mut tile_map,
            &mut path_cache,
            &attack_validation,
            &mut unit_queries,
            &mut command_events,
            &mut tile_moved,
        );
        
        let execution_time = (time.elapsed_secs_f64() - start_time) * 1000.0; // Convert to milliseconds
        
        // Record statistics
        match &result {
            CommandResult::Success => {
                command_stats.record_success(execution_time);
                if command_config.debug_logging {
                    info!("Command executed successfully: {:?}", command);
                }
            }
            CommandResult::Failed { reason } => {
                command_stats.record_failure(execution_time);
                if command_config.debug_logging {
                    warn!("Command failed: {:?} - Reason: {}", command, reason);
                }
            }
            CommandResult::Cancelled => {
                command_stats.record_cancellation();
                if command_config.debug_logging {
                    info!("Command cancelled: {:?}", command);
                }
            }
        }
        
        // Fire completion event
        command_events.write(CommandCompletedEvent {
            entity: command.entity,
            command: command.clone(),
            result,
        });
        
        commands_processed += 1;
    }
    
    // Update peak commands statistic
    command_stats.update_peak_commands(commands_processed);
    
    if command_config.debug_logging && commands_processed > 0 {
        info!(
            "Processed {} commands this frame. Queue size: {}",
            commands_processed,
            command_queue.len()
        );
    }
}

/// Execute a single command - central dispatch point
fn execute_command(
    command: &PendingCommand,
    command_config: &CommandConfig,
    tile_config: &TileConfig,
    tile_map: &mut ResMut<TileMap>,
    path_cache: &mut ResMut<PathCache>,
    attack_validation: &AttackValidation,
    unit_queries: &mut ParamSet<(
        Query<(&mut TileCoords, &mut Transform, &Team), With<UnitMarker>>,
        Query<&Team, With<UnitMarker>>,
    )>,
    command_events: &mut EventWriter<CommandCompletedEvent>,
    tile_moved: &mut EventWriter<TileMoved>,
) -> CommandResult {
    match &command.command_type {
        CommandType::Move { from, to } => {
            execute_movement_command(
                command.entity,
                *from,
                *to,
                tile_config,
                tile_map,
                path_cache,
                &mut unit_queries.p0(),
                command_events,
                tile_moved,
            )
        }
        CommandType::Attack { from, target } => {
            execute_attack_command(
                command.entity,
                *from,
                *target,
                tile_map,
                attack_validation,
                &mut unit_queries.p1(),
                command_events,
            )
        }
    }
}

/// System to clean up timed-out commands
pub fn cleanup_timed_out_commands(
    mut commands: Commands,
    mut command_queue: ResMut<CommandQueue>,
    command_config: Res<CommandConfig>,
    executing_query: Query<(Entity, &ExecutingCommand)>,
    mut command_events: EventWriter<CommandCompletedEvent>,
    time: Res<Time>,
) {
    let current_time = time.elapsed_secs_f64();
    let timeout = command_config.command_timeout;
    
    for (entity, executing) in executing_query.iter() {
        if current_time - executing.start_time > timeout {
            // Remove the executing command component
            commands.entity(entity).remove::<ExecutingCommand>();
            
            // Fire timeout event
            command_events.write(CommandCompletedEvent {
                entity: executing.command.entity,
                command: executing.command.clone(),
                result: CommandResult::Failed { 
                    reason: "Command timed out".to_string() 
                },
            });
            
            warn!("Command timed out for entity {:?}: {:?}", entity, executing.command);
        }
    }
}

/// System to handle command completion events
pub fn handle_command_events(
    mut command_events: EventReader<CommandCompletedEvent>,
    command_config: Res<CommandConfig>,
) {
    for event in command_events.read() {
        if command_config.debug_logging {
            match &event.result {
                CommandResult::Success => {
                    info!("Command completed successfully for entity {:?}", event.entity);
                }
                CommandResult::Failed { reason } => {
                    warn!("Command failed for entity {:?}: {}", event.entity, reason);
                }
                CommandResult::Cancelled => {
                    info!("Command cancelled for entity {:?}", event.entity);
                }
            }
        }
    }
}

/// Debug system to display command queue status
pub fn debug_command_queue(
    command_queue: Res<CommandQueue>,
    command_stats: Res<CommandStats>,
    command_config: Res<CommandConfig>,
) {
    if !command_config.debug_logging {
        return;
    }
    
    // Log every 60 frames (roughly once per second at 60 FPS)
    if command_stats.total_executed % 60 == 0 && command_stats.total_executed > 0 {
        info!(
            "Command Stats - Queue: {}, Total: {}, Success Rate: {:.1}%, Avg Time: {:.2}ms",
            command_queue.len(),
            command_stats.total_executed,
            command_stats.success_rate(),
            command_stats.avg_execution_time_ms
        );
    }
}