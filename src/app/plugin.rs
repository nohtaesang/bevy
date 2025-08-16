use bevy::prelude::*;
use super::state::{AppState, PauseState};
use super::systems::{on_enter_battle, on_exit_battle};  

pub struct AppStatesPlugin;
impl Plugin for AppStatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<AppState>()
            .init_state::<PauseState>()
            .add_systems(OnEnter(AppState::Battle), on_enter_battle)
            .add_systems(OnExit(AppState::Battle), on_exit_battle);
    }
}
