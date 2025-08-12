//! Right-click handling for action and selection cancellation

use bevy::prelude::*;
use crate::core::{SelectionState, ActionState, SelectionCtx};
use crate::features::tiles::actions::clear_selection;

/// System that handles right-click to cancel actions when in Move or Attack mode
pub fn handle_right_click_action_cancel(
    mouse_input: Res<ButtonInput<MouseButton>>,
    action_state: Res<State<ActionState>>,
    mut next_action_state: ResMut<NextState<ActionState>>,
) {
    
    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }
    
    // Cancel action states (Move/Attack) back to Idle
    match action_state.get() {
        ActionState::Move => {
            println!("Right-clicked - canceling Move mode, returning to Idle");
            next_action_state.set(ActionState::Idle);
        },
        ActionState::Attack => {
            println!("Right-clicked - canceling Attack mode, returning to Idle");
            next_action_state.set(ActionState::Idle);
        },
        ActionState::Idle => {
            // Idle action state - do nothing here, handled by selection cancel
        },
    }
}

/// System that handles right-click to clear selection when in Idle action state
pub fn handle_right_click_selection_clear(
    mouse_input: Res<ButtonInput<MouseButton>>,
    selection_state: Res<State<SelectionState>>,
    action_state: Res<State<ActionState>>,
    mut next_selection_state: ResMut<NextState<SelectionState>>,
    mut next_action_state: ResMut<NextState<ActionState>>,
    mut selection_ctx: ResMut<SelectionCtx>,
) {
    
    if !mouse_input.just_pressed(MouseButton::Right) {
        return;
    }
    
    // Only clear selection if we're in Idle action state
    if *action_state.get() != ActionState::Idle {
        return;
    }
    
    // Clear selection for these states
    match selection_state.get() {
        SelectionState::TileSelected => {
            println!("Right-clicked - clearing tile selection");
            clear_selection(&mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        },
        SelectionState::UnitSelected => {
            println!("Right-clicked - clearing unit selection");
            clear_selection(&mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        },
        SelectionState::EnemySelected => {
            println!("Right-clicked - clearing enemy selection");
            clear_selection(&mut next_selection_state, &mut next_action_state, &mut selection_ctx);
        },
        SelectionState::Idle => {
            // Already idle - do nothing
        },
    }
}