//! Command system resources

use bevy::prelude::*;
use std::collections::HashMap;

/// Configuration for command execution
#[derive(Resource)]
pub struct CommandConfig {
    /// Maximum number of commands that can be processed per frame
    pub max_commands_per_frame: usize,
    
    /// Whether to enable command batching
    pub enable_batching: bool,
    
    /// Timeout for command execution in seconds
    pub command_timeout: f64,
    
    /// Whether to log command execution details
    pub debug_logging: bool,
}

impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            max_commands_per_frame: 10,
            enable_batching: true,
            command_timeout: 5.0,
            debug_logging: false,
        }
    }
}

/// Cache for pathfinding results to optimize repeated calculations
#[derive(Resource, Default)]
pub struct PathCache {
    /// Cache mapping from (start, end, movement_range) to path validity
    cache: HashMap<(IVec2, IVec2, i32), bool>,
    
    /// Maximum number of cached entries
    max_entries: usize,
}

impl PathCache {
    /// Create a new path cache with specified capacity
    pub fn new(max_entries: usize) -> Self {
        Self {
            cache: HashMap::new(),
            max_entries,
        }
    }
    
    /// Check if a path is cached
    pub fn get_path_validity(&self, from: IVec2, to: IVec2, movement_range: i32) -> Option<bool> {
        self.cache.get(&(from, to, movement_range)).copied()
    }
    
    /// Cache a path validity result
    pub fn cache_path_validity(&mut self, from: IVec2, to: IVec2, movement_range: i32, is_valid: bool) {
        // If cache is full, remove the oldest entry (simple FIFO)
        if self.cache.len() >= self.max_entries {
            if let Some(key) = self.cache.keys().next().copied() {
                self.cache.remove(&key);
            }
        }
        
        self.cache.insert((from, to, movement_range), is_valid);
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    
    /// Clear cache entries involving a specific position
    pub fn invalidate_position(&mut self, pos: IVec2) {
        self.cache.retain(|(from, to, _), _| *from != pos && *to != pos);
    }
}

/// Statistics for command execution performance
#[derive(Resource, Default)]
pub struct CommandStats {
    /// Total commands executed
    pub total_executed: u64,
    
    /// Commands executed successfully
    pub successful: u64,
    
    /// Commands that failed
    pub failed: u64,
    
    /// Commands that were cancelled
    pub cancelled: u64,
    
    /// Average execution time in milliseconds
    pub avg_execution_time_ms: f64,
    
    /// Peak commands processed in a single frame
    pub peak_commands_per_frame: usize,
}

impl CommandStats {
    /// Record a successful command execution
    pub fn record_success(&mut self, execution_time_ms: f64) {
        self.total_executed += 1;
        self.successful += 1;
        self.update_avg_time(execution_time_ms);
    }
    
    /// Record a failed command execution
    pub fn record_failure(&mut self, execution_time_ms: f64) {
        self.total_executed += 1;
        self.failed += 1;
        self.update_avg_time(execution_time_ms);
    }
    
    /// Record a cancelled command
    pub fn record_cancellation(&mut self) {
        self.total_executed += 1;
        self.cancelled += 1;
    }
    
    /// Update the peak commands per frame
    pub fn update_peak_commands(&mut self, commands_this_frame: usize) {
        if commands_this_frame > self.peak_commands_per_frame {
            self.peak_commands_per_frame = commands_this_frame;
        }
    }
    
    /// Reset all statistics
    pub fn reset(&mut self) {
        *self = CommandStats::default();
    }
    
    /// Get success rate as a percentage
    pub fn success_rate(&self) -> f64 {
        if self.total_executed == 0 {
            0.0
        } else {
            (self.successful as f64 / self.total_executed as f64) * 100.0
        }
    }
    
    fn update_avg_time(&mut self, new_time: f64) {
        if self.total_executed == 1 {
            self.avg_execution_time_ms = new_time;
        } else {
            // Running average calculation
            let count = self.total_executed as f64;
            self.avg_execution_time_ms = ((self.avg_execution_time_ms * (count - 1.0)) + new_time) / count;
        }
    }
}

/// Resource to control command system behavior
#[derive(Resource, Default)]
pub struct CommandSystemState {
    /// Whether the command system is currently paused
    pub paused: bool,
    
    /// Whether to process commands in strict order
    pub strict_ordering: bool,
    
    /// Current frame number for debugging
    pub frame_count: u64,
}