use bevy::prelude::*;

/// Tile component representing a single grid tile
#[derive(Component)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
}