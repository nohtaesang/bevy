use bevy::prelude::*;

/// 플레이어 의도(모드)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PlayerIntent {
    #[default]
    Idle,
    Move,
    Attack,
}
