//! Keyboard input handling when a unit is selected
//!
//! This module handles keyboard shortcuts when a unit is selected

use bevy::prelude::*;
use crate::states::in_game::{UnitCommandState};

/// Handle keyboard input for unit actions when a unit is selected
pub fn handle_unit_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_action_state: ResMut<NextState<UnitCommandState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        next_action_state.set(UnitCommandState::Move);
    }
    
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        next_action_state.set(UnitCommandState::Attack);
    }
}