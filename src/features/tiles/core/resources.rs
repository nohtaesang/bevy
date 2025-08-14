//! Core tile resources
//!
//! This module contains TileMap, TileConfig, GridIndex and related core resources

use bevy::prelude::*;
use std::collections::HashMap;
use super::events::Team;

/// Tile configuration resource
#[derive(Resource)]
pub struct TileConfig {
    pub tile_size: f32,
    pub tile_spacing: f32,
    pub grid_size: i32,
}

impl Default for TileConfig {
    fn default() -> Self {
        Self {
            tile_size: 64.0,
            tile_spacing: 2.0,
            grid_size: 30,
        }
    }
}

/// Represents what occupies a tile
#[derive(Clone, Debug, PartialEq)]
pub enum TileContent {
    Empty,
    Unit(Entity),
    Enemy(Entity),
    Obstacle,
}

/// Result of a move operation
#[derive(Clone, Debug, PartialEq)]
pub enum MoveOutcome {
    /// Move succeeded - entity was moved
    Moved { entity: Entity },
    /// Target position is blocked (occupied or obstacle)
    Blocked,
    /// Target position is out of bounds
    OutOfBounds,
    /// Source position is empty (nothing to move)
    EmptyFrom,
}

/// The main tile map that tracks what's on each tile
#[derive(Resource, Default)]
pub struct TileMap {
    /// Maps tile positions to their contents
    tiles: HashMap<IVec2, TileContent>,
    /// Grid size for bounds checking
    grid_size: i32,
}

impl TileMap {
    /// Create a new TileMap with the specified grid size
    pub fn new(grid_size: i32) -> Self {
        Self {
            tiles: HashMap::new(),
            grid_size,
        }
    }
    
    /// Get the content at a specific position
    pub fn get_content(&self, pos: IVec2) -> TileContent {
        self.tiles.get(&pos).cloned().unwrap_or(TileContent::Empty)
    }
    
    /// Check if a position is empty
    pub fn is_empty(&self, pos: IVec2) -> bool {
        matches!(self.get_content(pos), TileContent::Empty)
    }
    
    /// Check if a position has a unit
    pub fn has_unit(&self, pos: IVec2) -> bool {
        matches!(self.get_content(pos), TileContent::Unit(_))
    }
    
    /// Check if a position has an enemy
    pub fn has_enemy(&self, pos: IVec2) -> bool {
        matches!(self.get_content(pos), TileContent::Enemy(_))
    }
    
    /// Get the entity at a position (if any)
    pub fn get_entity(&self, pos: IVec2) -> Option<Entity> {
        match self.get_content(pos) {
            TileContent::Unit(entity) | TileContent::Enemy(entity) => Some(entity),
            _ => None,
        }
    }
    
    /// Place a unit on the map - returns true if successful, false if position is occupied
    pub fn place_unit(&mut self, pos: IVec2, entity: Entity) -> bool {
        if !self.is_valid_position(pos) {
            return false;
        }
        
        if !self.is_empty(pos) {
            return false; // Position already occupied
        }
        
        self.tiles.insert(pos, TileContent::Unit(entity));
        true
    }
    
    /// Place an enemy on the map - returns true if successful, false if position is occupied
    pub fn place_enemy(&mut self, pos: IVec2, entity: Entity) -> bool {
        if !self.is_valid_position(pos) {
            return false;
        }
        
        if !self.is_empty(pos) {
            return false; // Position already occupied
        }
        
        self.tiles.insert(pos, TileContent::Enemy(entity));
        true
    }
    
    /// Move a unit from one position to another
    pub fn move_unit(&mut self, from: IVec2, to: IVec2) -> MoveOutcome {
        // Check if target is out of bounds
        if !self.is_valid_position(to) {
            return MoveOutcome::OutOfBounds;
        }
        
        // Check if target is blocked
        if !self.is_empty(to) {
            return MoveOutcome::Blocked;
        }
        
        // Check if source has something to move
        if let Some(content) = self.tiles.remove(&from) {
            let entity = match &content {
                TileContent::Unit(e) | TileContent::Enemy(e) => *e,
                _ => return MoveOutcome::EmptyFrom, // Shouldn't happen, but safe
            };
            
            self.tiles.insert(to, content);
            MoveOutcome::Moved { entity }
        } else {
            MoveOutcome::EmptyFrom
        }
    }
    
    /// Remove an entity from the map
    pub fn remove_entity(&mut self, pos: IVec2) {
        self.tiles.remove(&pos);
    }
    
    /// Clear a tile
    pub fn clear_tile(&mut self, pos: IVec2) {
        self.tiles.remove(&pos);
    }
    
    /// Check if a position is within the grid bounds
    pub fn is_valid_position(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.x < self.grid_size && pos.y >= 0 && pos.y < self.grid_size
    }
    
    /// Get all units on the map
    pub fn get_all_units(&self) -> Vec<(IVec2, Entity)> {
        self.tiles
            .iter()
            .filter_map(|(pos, content)| {
                if let TileContent::Unit(entity) = content {
                    Some((*pos, *entity))
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Get all enemies on the map
    pub fn get_all_enemies(&self) -> Vec<(IVec2, Entity)> {
        self.tiles
            .iter()
            .filter_map(|(pos, content)| {
                if let TileContent::Enemy(entity) = content {
                    Some((*pos, *entity))
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Find entity position
    pub fn find_entity_position(&self, entity: Entity) -> Option<IVec2> {
        self.tiles
            .iter()
            .find(|(_, content)| match content {
                TileContent::Unit(e) | TileContent::Enemy(e) => *e == entity,
                _ => false,
            })
            .map(|(pos, _)| *pos)
    }
}

/// High-performance spatial index using row-major Vec storage for O(1) lookups
/// SSOT = components; this is a read-only cache updated via events.
#[derive(Resource)]
pub struct GridIndex {
    /// Grid dimensions
    width: i32,
    height: i32,
    
    /// Entity occupying each tile (None if empty)
    /// Indexed by: row * width + col
    unit_at: Vec<Option<Entity>>,
    
    /// Team of unit at each tile (None if empty)
    team_at: Vec<Option<Team>>,
    
    /// Whether each tile is blocked for movement
    blocked: Vec<bool>,
    
    /// Movement cost for each tile (1 = normal, higher = more expensive)
    cost: Vec<u8>,
    
    /// Version counter for dirty checking - increments on any change
    version: u64,
}

impl GridIndex {
    /// Create new GridIndex with given dimensions
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            unit_at: vec![None; size],
            team_at: vec![None; size],
            blocked: vec![false; size],
            cost: vec![1; size], // Default cost of 1
            version: 0,
        }
    }
    
    /// Convert 2D position to 1D index (row-major order)
    #[inline]
    fn pos_to_index(&self, pos: IVec2) -> Option<usize> {
        if pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height {
            Some((pos.y * self.width + pos.x) as usize)
        } else {
            None
        }
    }
    
    /// Get entity at position (O(1) lookup)
    #[inline]
    pub fn unit_at(&self, pos: IVec2) -> Option<Entity> {
        self.pos_to_index(pos)
            .and_then(|idx| self.unit_at.get(idx))
            .copied()
            .flatten()
    }
    
    /// Get team at position (O(1) lookup)
    #[inline]
    pub fn team_at(&self, pos: IVec2) -> Option<Team> {
        self.pos_to_index(pos)
            .and_then(|idx| self.team_at.get(idx))
            .copied()
            .flatten()
    }
    
    /// Check if position is blocked (O(1) lookup)
    #[inline]
    pub fn is_blocked(&self, pos: IVec2) -> bool {
        self.pos_to_index(pos)
            .and_then(|idx| self.blocked.get(idx))
            .copied()
            .unwrap_or(true) // Out of bounds = blocked
    }
    
    /// Get movement cost at position (O(1) lookup)
    #[inline]
    pub fn cost_at(&self, pos: IVec2) -> u8 {
        self.pos_to_index(pos)
            .and_then(|idx| self.cost.get(idx))
            .copied()
            .unwrap_or(255) // Out of bounds = max cost
    }
    
    /// Check if position is empty (O(1) lookup)
    #[inline]
    pub fn is_empty(&self, pos: IVec2) -> bool {
        self.unit_at(pos).is_none()
    }
    
    /// Check if position is in bounds
    #[inline]
    pub fn is_in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
    
    /// Get current version for dirty checking
    #[inline]
    pub fn version(&self) -> u64 {
        self.version
    }
    
    /// Get grid dimensions
    #[inline]
    pub fn dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }
    
    /// Internal: Set unit at position and increment version
    pub(crate) fn set_unit_at(&mut self, pos: IVec2, entity: Option<Entity>, team: Option<Team>) {
        if let Some(idx) = self.pos_to_index(pos) {
            self.unit_at[idx] = entity;
            self.team_at[idx] = team;
            self.version += 1;
        }
    }
    
    /// Internal: Set blocked status and increment version
    pub(crate) fn set_blocked(&mut self, pos: IVec2, blocked: bool) {
        if let Some(idx) = self.pos_to_index(pos) {
            self.blocked[idx] = blocked;
            self.version += 1;
        }
    }
    
    /// Internal: Set movement cost and increment version
    pub(crate) fn set_cost(&mut self, pos: IVec2, cost: u8) {
        if let Some(idx) = self.pos_to_index(pos) {
            self.cost[idx] = cost;
            self.version += 1;
        }
    }
}