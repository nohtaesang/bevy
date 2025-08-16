use bevy::prelude::*;
use crate::states::in_game::TurnState;

pub fn handle_turn_switch(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<TurnState>>,
    mut next_state: ResMut<NextState<TurnState>>,
) {
    if input.just_pressed(KeyCode::Space) {
        match current_state.get() {
            TurnState::PlayerTurn => {
                next_state.set(TurnState::EnemyTurn);
                info!("Switched to Enemy Turn");
            }
            TurnState::EnemyTurn => {
                next_state.set(TurnState::PlayerTurn);
                info!("Switched to Player Turn");
            }
        }
    }
}