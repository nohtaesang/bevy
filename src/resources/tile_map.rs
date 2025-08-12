//! TileMap resource - Single source of truth for tile contents
//!
//! This resource maintains the authoritative state of what exists on each tile

use bevy::prelude::*;
use std::collections::HashMap;

/// Represents what occupies a tile
#[derive(Clone, Debug, PartialEq)]
pub enum TileContent {
    Empty,
    Unit(Entity),
    Enemy(Entity),
    Obstacle,
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
    
    /// Place a unit on the map
    pub fn place_unit(&mut self, pos: IVec2, entity: Entity) {
        if self.is_valid_position(pos) {
            self.tiles.insert(pos, TileContent::Unit(entity));
        }
    }
    
    /// Place an enemy on the map
    pub fn place_enemy(&mut self, pos: IVec2, entity: Entity) {
        if self.is_valid_position(pos) {
            self.tiles.insert(pos, TileContent::Enemy(entity));
        }
    }
    
    /// Move a unit from one position to another
    pub fn move_unit(&mut self, from: IVec2, to: IVec2) -> bool {
        if !self.is_valid_position(to) || !self.is_empty(to) {
            return false;
        }
        
        if let Some(content) = self.tiles.remove(&from) {
            self.tiles.insert(to, content);
            true
        } else {
            false
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