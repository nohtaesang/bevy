//! Hotkey system for unit commands
//!
//! Provides keyboard shortcuts for common unit actions

use bevy::prelude::*;
use crate::{
    features::tiles::{
        selection::SelectionCtx,
        core::Team,
        units::bundles::UnitMarker,
    },
    states::in_game::UnitCommandState,
};

/// 1→Move, 2→Attack 단축키
pub fn unit_command_hotkeys(
    keys: Res<ButtonInput<KeyCode>>,
    selection: Res<SelectionCtx>,
    teams: Query<&Team, With<UnitMarker>>,
    mut next: ResMut<NextState<UnitCommandState>>,
) {
    // 플레이어 유닛이 선택되어 있을 때만
    let Some(e) = selection.selected_unit else { return; };
    if teams.get(e).ok() != Some(&Team::Player) { return; }

    if keys.just_pressed(KeyCode::Digit1) || keys.just_pressed(KeyCode::Numpad1) {
        next.set(UnitCommandState::Move);
    } else if keys.just_pressed(KeyCode::Digit2) || keys.just_pressed(KeyCode::Numpad2) {
        next.set(UnitCommandState::Attack);
    }
}