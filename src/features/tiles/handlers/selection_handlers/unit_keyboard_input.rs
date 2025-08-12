//! Keyboard input handling when a unit is selected
//!
//! This module handles keyboard shortcuts when a unit is selected

use bevy::prelude::*;
use crate::core::{ActionState};

/// Handle keyboard input for unit actions when a unit is selected
pub fn handle_unit_keyboard_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_action_state: ResMut<NextState<ActionState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Digit1) {
        println!("1 key pressed - entering Move mode");
        next_action_state.set(ActionState::Move);
    }
    
    if keyboard_input.just_pressed(KeyCode::Digit2) {
        println!("2 key pressed - entering Attack mode");
        next_action_state.set(ActionState::Attack);
    }
}