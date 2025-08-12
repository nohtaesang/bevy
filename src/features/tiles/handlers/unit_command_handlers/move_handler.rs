//! Move state click handler
//!
//! Handles clicks when in Move action state and routes to appropriate actions

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::{
        tiles::SelectionCtx,
        tiles::{
            utils::world_to_tile_coords,
            overlay::{MovementOverlay, MovementValidation},
            actions::{select_tile, select_unit, select_enemy, clear_selection, execute_move},
        },
        units::{Unit, Enemy},
    },
    features::tiles::{TileConfig, TileMap},
};

#[derive(Debug)]
enum ClickTarget {
    SelfUnit,
    FriendlyUnit,
    Enemy,
    MovementOverlay,
    EmptyTile,
    OutsideGrid,
}

/// System that handles mouse clicks when in Move action state
pub fn handle_move_state_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    tile_config: Res<TileConfig>,
    mut tile_map: ResMut<TileMap>,
    mut next_selection_state: ResMut<NextState<SelectionState>>,
    mut next_action_state: ResMut<NextState<UnitCommandState>>,
    mut selection_ctx: ResMut<SelectionCtx>,
    mut unit_queries: ParamSet<(
        Query<(Entity, &Unit)>,
        Query<(&mut Unit, &mut Transform)>,
    )>,
    enemy_query: Query<(Entity, &Enemy)>,
    movement_validation: Res<MovementValidation>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    let Ok(window) = windows.single() else { return; };
    let Some(cursor_pos) = window.cursor_position() else { return; };
    let Ok((camera, camera_transform)) = camera_q.single() else { return; };
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) else { return; };
    
    let click_target = determine_click_target(
        world_pos,
        &tile_config,
        &tile_map,
        &selection_ctx,
        &movement_validation,
    );

    handle_click_target(
        click_target,
        world_pos,
        &tile_config,
        &mut tile_map,
        &mut next_selection_state,
        &mut next_action_state,
        &mut selection_ctx,
        &mut unit_queries,
        &enemy_query,
    );
}

fn determine_click_target(
    world_pos: Vec2,
    tile_config: &TileConfig,
    tile_map: &TileMap,
    selection_ctx: &SelectionCtx,
    movement_validation: &MovementValidation,
) -> ClickTarget {
    let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) else {
        return ClickTarget::OutsideGrid;
    };
    
    let tile_pos = tile_coords.into();

    // Use TileMap to check what's at the clicked position
    if let Some(entity) = tile_map.get_entity(tile_pos) {
        if tile_map.has_unit(tile_pos) {
            if Some(entity) == selection_ctx.selected_unit {
                return ClickTarget::SelfUnit;
            } else {
                return ClickTarget::FriendlyUnit;
            }
        } else if tile_map.has_enemy(tile_pos) {
            return ClickTarget::Enemy;
        }
    }

    // Check if clicking on valid movement tile using HashSet (O(1) lookup)
    if movement_validation.is_valid_move(tile_pos) {
        ClickTarget::MovementOverlay
    } else {
        ClickTarget::EmptyTile
    }
}

fn handle_click_target(
    click_target: ClickTarget,
    world_pos: Vec2,
    tile_config: &TileConfig,
    tile_map: &mut ResMut<TileMap>,
    next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    selection_ctx: &mut ResMut<SelectionCtx>,
    unit_queries: &mut ParamSet<(
        Query<(Entity, &Unit)>,
        Query<(&mut Unit, &mut Transform)>,
    )>,
    enemy_query: &Query<(Entity, &Enemy)>,
) {
    match click_target {
        ClickTarget::SelfUnit => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                if let Some(selected_unit) = selection_ctx.selected_unit {
                    println!("Clicked on self while in Move state - reselecting same unit");
                    select_unit(selected_unit, tile_coords.into(), next_selection_state, next_action_state, selection_ctx);
                }
            }
        },
        ClickTarget::FriendlyUnit => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                let tile_pos = tile_coords.into();
                for (unit_entity, unit) in unit_queries.p0().iter() {
                    if unit.tile_pos == tile_pos {
                        println!("Clicked friendly unit while in Move state - selecting different unit");
                        select_unit(unit_entity, tile_pos, next_selection_state, next_action_state, selection_ctx);
                        break;
                    }
                }
            }
        },
        ClickTarget::Enemy => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                let tile_pos = tile_coords.into();
                for (enemy_entity, enemy) in enemy_query.iter() {
                    if enemy.tile_pos == tile_pos {
                        println!("Clicked enemy while in Move state - selecting enemy");
                        select_enemy(enemy_entity, tile_pos, next_selection_state, next_action_state, selection_ctx);
                        break;
                    }
                }
            }
        },
        ClickTarget::MovementOverlay => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                let target_pos = tile_coords.into();
                println!("Clicked movement overlay at {:?} - attempting to move", target_pos);
                execute_move(target_pos, next_action_state, selection_ctx, tile_config, tile_map, unit_queries);
            }
        },
        ClickTarget::EmptyTile => {
            if let Some(tile_coords) = world_to_tile_coords(world_pos, tile_config) {
                let tile_pos = tile_coords.into();
                println!("Clicked empty tile at {:?} while in Move state - selecting tile", tile_pos);
                select_tile(tile_pos, next_selection_state, next_action_state, selection_ctx);
            }
        },
        ClickTarget::OutsideGrid => {
            println!("Clicked outside grid while in Move state - clearing selection");
            clear_selection(next_selection_state, next_action_state, selection_ctx);
        },
    }
}