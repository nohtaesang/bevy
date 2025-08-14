//! Right-click system for state transitions
//!
//! Handles right-click to go back to previous states

use bevy::prelude::*;
use crate::states::in_game::{UnitCommandState, SelectionState, TurnState};

/// Handle right-click to transition between states
pub fn handle_right_click_state_transition(
    mouse_input: Res<ButtonInput<MouseButton>>,
    current_unit_command: Res<State<UnitCommandState>>,
    current_selection: Res<State<SelectionState>>,
    mut unit_command_next: ResMut<NextState<UnitCommandState>>,
    mut selection_next: ResMut<NextState<SelectionState>>,
    mut turn_next: ResMut<NextState<TurnState>>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        match current_unit_command.get() {
            // If in Move or Attack state, go back to Idle (SelectionState)
            UnitCommandState::Move | UnitCommandState::Attack => {
                unit_command_next.set(UnitCommandState::Idle);
                info!("Right-click: UnitCommand → Idle");
            }
            // If in Idle state, check SelectionState
            UnitCommandState::Idle => {
                match current_selection.get() {
                    // If something is selected, go back to Idle
                    SelectionState::UnitSelected | 
                    SelectionState::TileSelected | 
                    SelectionState::EnemySelected => {
                        selection_next.set(SelectionState::Idle);
                        info!("Right-click: Selection → Idle");
                    }
                    // If idle, end turn (go to next turn state)
                    SelectionState::Idle => {
                        turn_next.set(TurnState::EnemyTurn);
                        info!("Right-click: Turn → Enemy");
                    }
                }
            }
        }
    }
}