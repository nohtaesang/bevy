//! Command system components

use bevy::prelude::*;

/// Types of commands that can be executed
#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Move { 
        from: IVec2, 
        to: IVec2 
    },
    Attack { 
        from: IVec2, 
        target: IVec2 
    },
}

/// A pending command to be executed
#[derive(Debug, Clone, Component)]
pub struct PendingCommand {
    pub entity: Entity,
    pub command_type: CommandType,
    pub priority: u32,
}

impl PendingCommand {
    /// Create a new movement command
    pub fn move_command(entity: Entity, from: IVec2, to: IVec2) -> Self {
        Self {
            entity,
            command_type: CommandType::Move { from, to },
            priority: 0,
        }
    }
    
    /// Create a new attack command
    pub fn attack_command(entity: Entity, from: IVec2, target: IVec2) -> Self {
        Self {
            entity,
            command_type: CommandType::Attack { from, target },
            priority: 10, // Attacks have higher priority than movement
        }
    }
    
    /// Create a command with custom priority
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
}

/// Global command queue resource
#[derive(Resource, Default)]
pub struct CommandQueue {
    commands: Vec<PendingCommand>,
}

impl CommandQueue {
    /// Add a command to the queue
    pub fn add_command(&mut self, command: PendingCommand) {
        self.commands.push(command);
        // Sort by priority (higher priority first)
        self.commands.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    
    /// Get the next command from the queue
    pub fn pop_command(&mut self) -> Option<PendingCommand> {
        self.commands.pop()
    }
    
    /// Get all commands for a specific entity
    pub fn get_commands_for_entity(&self, entity: Entity) -> Vec<&PendingCommand> {
        self.commands.iter().filter(|cmd| cmd.entity == entity).collect()
    }
    
    /// Remove all commands for a specific entity
    pub fn remove_commands_for_entity(&mut self, entity: Entity) {
        self.commands.retain(|cmd| cmd.entity != entity);
    }
    
    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }
    
    /// Get the number of commands in the queue
    pub fn len(&self) -> usize {
        self.commands.len()
    }
    
    /// Clear all commands
    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

/// Component marking an entity as currently executing a command
#[derive(Component)]
pub struct ExecutingCommand {
    pub command: PendingCommand,
    pub start_time: f64,
}

/// Component for command execution results
#[derive(Component, Debug)]
pub enum CommandResult {
    Success,
    Failed { reason: String },
    Cancelled,
}

/// Event fired when a command is completed
#[derive(Event, Debug)]
pub struct CommandCompletedEvent {
    pub entity: Entity,
    pub command: PendingCommand,
    pub result: CommandResult,
}