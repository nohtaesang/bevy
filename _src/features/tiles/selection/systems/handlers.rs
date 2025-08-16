//! Selection click handlers
//!
//! State-specific handlers that respond to ClickTargetEvent

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState, TurnState},
    features::tiles::{
        interaction::{ClickTargetEvent, ClickTarget},
        selection::SelectionCtx,
    },
};

/// Handle clicks when in Idle selection state
pub fn handle_click_idle(
    mut ev: EventReader<ClickTargetEvent>,
    mut next_sel: ResMut<NextState<SelectionState>>,
    mut ctx: ResMut<SelectionCtx>,
) {
    for ClickTargetEvent(target) in ev.read() {
        match *target {
            ClickTarget::SelfUnit(pos, ent) | ClickTarget::FriendlyUnit(pos, ent) => {
                ctx.selected_unit = Some(ent);
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::UnitSelected);
                info!("Selected unit {:?} at {:?}", ent, pos);
            }
            ClickTarget::Enemy(pos, ent) => {
                ctx.selected_enemy = Some(ent);
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::EnemySelected);
                info!("Selected enemy {:?} at {:?}", ent, pos);
            }
            ClickTarget::EmptyTile(pos) => {
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::TileSelected);
                info!("Selected tile {:?}", pos);
            }
            // Ignore overlays and outside clicks in Idle state
            _ => {}
        }
    }
}

/// Handle clicks when a unit is selected
pub fn handle_click_unit_selected(
    mut ev: EventReader<ClickTargetEvent>,
    mut next_sel: ResMut<NextState<SelectionState>>,
    mut next_cmd: ResMut<NextState<UnitCommandState>>,
    mut ctx: ResMut<SelectionCtx>,
) {
    for ClickTargetEvent(target) in ev.read() {
        match *target {
            ClickTarget::SelfUnit(pos, ent) => {
                // Clicking selected unit again - keep selected
                ctx.selected_unit = Some(ent);
                ctx.tile = Some(pos);
                info!("Re-selected same unit {:?} at {:?}", ent, pos);
            }
            ClickTarget::FriendlyUnit(pos, ent) => {
                // Switch to different unit
                ctx.selected_unit = Some(ent);
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::UnitSelected);
                info!("Switched to unit {:?} at {:?}", ent, pos);
            }
            ClickTarget::Enemy(pos, ent) => {
                // Switch to enemy
                ctx.clear_unit_selection();
                ctx.selected_enemy = Some(ent);
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::EnemySelected);
                info!("Selected enemy {:?} at {:?}", ent, pos);
            }
            ClickTarget::EmptyTile(pos) => {
                // Switch to tile
                ctx.clear_unit_selection();
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::TileSelected);
                info!("Selected tile {:?}", pos);
            }
            ClickTarget::MovementOverlay(pos) => {
                // Enter movement command state
                ctx.target_tile = Some(pos);
                next_cmd.set(UnitCommandState::Move);
                info!("Entering Move command state, target: {:?}", pos);
            }
            ClickTarget::AttackOverlay(pos) => {
                // Enter attack command state
                ctx.target_tile = Some(pos);
                next_cmd.set(UnitCommandState::Attack);
                info!("Entering Attack command state, target: {:?}", pos);
            }
            ClickTarget::OutsideGrid => {
                // Clear selection
                ctx.clear_all();
                next_sel.set(SelectionState::Idle);
                info!("Cleared selection (clicked outside grid)");
            }
        }
    }
}

/// Handle clicks when an enemy is selected
pub fn handle_click_enemy_selected(
    mut ev: EventReader<ClickTargetEvent>,
    mut next_sel: ResMut<NextState<SelectionState>>,
    mut ctx: ResMut<SelectionCtx>,
) {
    for ClickTargetEvent(target) in ev.read() {
        match *target {
            ClickTarget::SelfUnit(pos, ent) | ClickTarget::FriendlyUnit(pos, ent) => {
                // Switch to unit
                ctx.clear_enemy_selection();
                ctx.selected_unit = Some(ent);
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::UnitSelected);
                info!("Switched to unit {:?} at {:?}", ent, pos);
            }
            ClickTarget::Enemy(pos, ent) => {
                // Switch to different enemy or re-select same
                ctx.selected_enemy = Some(ent);
                ctx.tile = Some(pos);
                info!("Selected enemy {:?} at {:?}", ent, pos);
            }
            ClickTarget::EmptyTile(pos) => {
                // Switch to tile
                ctx.clear_enemy_selection();
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::TileSelected);
                info!("Selected tile {:?}", pos);
            }
            ClickTarget::OutsideGrid => {
                // Clear selection
                ctx.clear_all();
                next_sel.set(SelectionState::Idle);
                info!("Cleared selection (clicked outside grid)");
            }
            // Ignore overlays in enemy selected state
            _ => {}
        }
    }
}

/// Handle clicks when a tile is selected
pub fn handle_click_tile_selected(
    mut ev: EventReader<ClickTargetEvent>,
    mut next_sel: ResMut<NextState<SelectionState>>,
    mut ctx: ResMut<SelectionCtx>,
) {
    for ClickTargetEvent(target) in ev.read() {
        match *target {
            ClickTarget::SelfUnit(pos, ent) | ClickTarget::FriendlyUnit(pos, ent) => {
                // Switch to unit
                ctx.selected_unit = Some(ent);
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::UnitSelected);
                info!("Switched to unit {:?} at {:?}", ent, pos);
            }
            ClickTarget::Enemy(pos, ent) => {
                // Switch to enemy
                ctx.selected_enemy = Some(ent);
                ctx.tile = Some(pos);
                next_sel.set(SelectionState::EnemySelected);
                info!("Selected enemy {:?} at {:?}", ent, pos);
            }
            ClickTarget::EmptyTile(pos) => {
                // Switch to different tile or re-select same
                ctx.tile = Some(pos);
                info!("Selected tile {:?}", pos);
            }
            ClickTarget::OutsideGrid => {
                // Clear selection
                ctx.clear_all();
                next_sel.set(SelectionState::Idle);
                info!("Cleared selection (clicked outside grid)");
            }
            // Ignore overlays in tile selected state
            _ => {}
        }
    }
}