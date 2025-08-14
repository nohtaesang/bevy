//! Raw mouse input collection
//!
//! This module handles primitive mouse input without domain knowledge

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::tiles::selection::{SelectionCtx, clear_selection},
};

/// System that handles right-click to cancel actions when in Move or Attack mode
pub fn handle_right_click_action_cancel(
    mouse_input: Res<ButtonInput<MouseButton>>,
    action_state: Res<State<UnitCommandState>>,
    mut next_action_state: ResMut<NextState<UnitCommandState>>,
) {
    
    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }
    
    // Cancel action states (Move/Attack) back to Idle
    match action_state.get() {
        UnitCommandState::Move => {
            next_action_state.set(UnitCommandState::Idle);
        },
        UnitCommandState::Attack => {
            next_action_state.set(UnitCommandState::Idle);
        },
        UnitCommandState::Idle => {
            // Idle action state - do nothing here, handled by selection cancel
        },
    }
}

/// System that handles right-click to clear selection when in Idle action state
pub fn handle_right_click_selection_clear(
    mouse_input: Res<ButtonInput<MouseButton>>,
    selection_state: Res<State<SelectionState>>,
    action_state: Res<State<UnitCommandState>>,
    mut next_selection_state: ResMut<NextState<SelectionState>>,
    mut next_action_state: ResMut<NextState<UnitCommandState>>,
    mut selection_ctx: ResMut<SelectionCtx>,
) {
    
    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }
    
    // Only clear selection if we're in Idle action state
    if *action_state.get() != UnitCommandState::Idle {
        return;
    }
    
    // Clear selection for these states
    match selection_state.get() {
        SelectionState::TileSelected => {
            clear_selection(&mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        },
        SelectionState::UnitSelected => {
            clear_selection(&mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        },
        SelectionState::EnemySelected => {
            clear_selection(&mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        },
        SelectionState::Idle => {
            // Already idle - do nothing
        },
    }
}