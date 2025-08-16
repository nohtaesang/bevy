use bevy::prelude::*;

/// Main application states
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
}