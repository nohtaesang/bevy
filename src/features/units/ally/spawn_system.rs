//! Spawn system for ally units

use bevy::prelude::*;
use crate::features::{
    tiles::{tile_to_world_coords, TileConfig, TileMap},
    units::shared::{AttackDirection, AttackType, AttackRange},
};
use super::Unit;

pub fn spawn_units(
    mut commands: Commands,
    tile_config: Res<TileConfig>,
    mut tile_map: ResMut<TileMap>,
) {
    // Spawn 4 units with different attack configurations
    
    // Unit 1: 4-way direct, range 1-3
    let pos1 = IVec2::new(2, 2);
    let world_pos = tile_to_world_coords(pos1.x, pos1.y, &tile_config);
    let unit1 = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 1.0), // Blue
            custom_size: Some(Vec2::new(tile_config.tile_size * 0.6, tile_config.tile_size * 0.6)),
            ..default()
        },
        Transform::from_xyz(world_pos.x, world_pos.y, 1.0),
        Unit::new(
            pos1,
            AttackDirection::Cardinal,
            AttackType::Direct,
            AttackRange::new(1, 3),
        ),
        Name::new("Unit 1"),
    )).id();
    tile_map.place_unit(pos1, unit1);
    
    // Unit 2: 8-way direct, range 1-3
    let pos2 = IVec2::new(3, 2);
    let world_pos = tile_to_world_coords(pos2.x, pos2.y, &tile_config);
    let unit2 = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 1.0), // Blue
            custom_size: Some(Vec2::new(tile_config.tile_size * 0.6, tile_config.tile_size * 0.6)),
            ..default()
        },
        Transform::from_xyz(world_pos.x, world_pos.y, 1.0),
        Unit::new(
            pos2,
            AttackDirection::EightWay,
            AttackType::Direct,
            AttackRange::new(1, 3),
        ),
        Name::new("Unit 2"),
    )).id();
    tile_map.place_unit(pos2, unit2);
    
    // Unit 3: 4-way indirect, range 1-3
    let pos3 = IVec2::new(2, 3);
    let world_pos = tile_to_world_coords(pos3.x, pos3.y, &tile_config);
    let unit3 = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 1.0), // Blue
            custom_size: Some(Vec2::new(tile_config.tile_size * 0.6, tile_config.tile_size * 0.6)),
            ..default()
        },
        Transform::from_xyz(world_pos.x, world_pos.y, 1.0),
        Unit::new(
            pos3,
            AttackDirection::Cardinal,
            AttackType::Indirect,
            AttackRange::new(1, 3),
        ),
        Name::new("Unit 3"),
    )).id();
    tile_map.place_unit(pos3, unit3);
    
    // Unit 4: 4-way indirect, range 1-3
    let pos4 = IVec2::new(3, 3);
    let world_pos = tile_to_world_coords(pos4.x, pos4.y, &tile_config);
    let unit4 = commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.6, 1.0), // Blue
            custom_size: Some(Vec2::new(tile_config.tile_size * 0.6, tile_config.tile_size * 0.6)),
            ..default()
        },
        Transform::from_xyz(world_pos.x, world_pos.y, 1.0),
        Unit::new(
            pos4,
            AttackDirection::Cardinal,
            AttackType::Indirect,
            AttackRange::new(1, 3),
        ),
        Name::new("Unit 4"),
    )).id();
    tile_map.place_unit(pos4, unit4);
    
}