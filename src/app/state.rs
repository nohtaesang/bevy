use bevy::prelude::*;

/// 전역 앱 상태
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum AppState {
    Title,
    Loading,
    Lobby,
    #[default]
    Battle,
    Result,
}

/// 일시정지 상태
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}
