//! Core tile components
//!
//! This module contains Tile, TileKind, TileCoords and related core components

use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoords {
    pub x: i32,
    pub y: i32,
}

impl TileCoords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    /// Convert to IVec2
    pub fn to_ivec2(self) -> IVec2 {
        IVec2::new(self.x, self.y)
    }
    
    /// Create from IVec2
    pub fn from_ivec2(pos: IVec2) -> Self {
        Self { x: pos.x, y: pos.y }
    }
}

impl From<IVec2> for TileCoords {
    fn from(pos: IVec2) -> Self {
        Self::from_ivec2(pos)
    }
}

impl From<TileCoords> for IVec2 {
    fn from(coords: TileCoords) -> Self {
        coords.to_ivec2()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    /// Standard walkable ground tile
    Ground,
    /// Impassable obstacle tile
    Obstacle,
    /// Water tile (may have special movement rules)
    Water,
    /// Special terrain tile
    Special,
}

impl Default for TileKind {
    fn default() -> Self {
        Self::Ground
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct TileType {
    pub kind: TileKind,
}

impl TileType {
    pub fn new(kind: TileKind) -> Self {
        Self { kind }
    }
    
    /// Check if this tile is walkable
    pub fn is_walkable(&self) -> bool {
        matches!(self.kind, TileKind::Ground | TileKind::Water)
    }
    
    /// Check if this tile blocks line of sight
    pub fn blocks_sight(&self) -> bool {
        matches!(self.kind, TileKind::Obstacle)
    }
}

impl Default for TileType {
    fn default() -> Self {
        Self::new(TileKind::Ground)
    }
}