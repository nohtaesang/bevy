use bevy::prelude::*;
use crate::core::AppState;

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
pub enum ActionState {
    #[default]
    Idle,
    Move,
    Attack,
}

/// Selection context resource to hold coordinates and entity references
#[derive(Resource, Default, Debug)]
pub struct SelectionCtx {
    pub tile: Option<IVec2>,
    pub selected_unit: Option<Entity>,
    pub selected_enemy: Option<Entity>,
}