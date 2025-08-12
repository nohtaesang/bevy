use bevy::prelude::*;
use crate::states::AppState;

/// Turn state (SubState of InGame)
#[derive(SubStates, Debug, Clone, Eq, PartialEq, Hash, Default)]
#[source(AppState = AppState::InGame)]
pub enum TurnState {
    #[default]
    PlayerTurn,
    EnemyTurn,
}

/// Player's selection state (SubState of PlayerTurn)
#[derive(SubStates, Debug, Clone, Eq, PartialEq, Hash, Default)]
#[source(TurnState = TurnState::PlayerTurn)]
pub enum SelectionState {
    #[default]
    Idle,
    TileSelected,
    UnitSelected,
    EnemySelected,
}

/// Player's action state (SubState of PlayerTurn)
#[derive(SubStates, Debug, Clone, Eq, PartialEq, Hash, Default)]
#[source(TurnState = TurnState::PlayerTurn)]
pub enum UnitCommandState {
    #[default]
    Idle,
    Move,
    Attack,
}

