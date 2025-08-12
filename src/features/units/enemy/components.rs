//! Enemy (AI-controlled) unit components

use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Enemy {
    pub tile_pos: IVec2,
    pub health: i32,
    pub max_health: i32,
    pub attack_power: i32,
}

impl Enemy {
    pub fn new(tile_pos: IVec2) -> Self {
        Self {
            tile_pos,
            health: 50,
            max_health: 50,
            attack_power: 5,
        }
    }
}