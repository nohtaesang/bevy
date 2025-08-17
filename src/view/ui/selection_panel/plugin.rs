use bevy::prelude::*;

use crate::app::state::AppState;
use super::systems::{spawn_selection_panel_once, update_selection_panel_ui};

// 공용 리소스가 아직 어딘가에서 초기화되지 않았다면 여기서도 init 해둠(중복 안전)
use crate::view::ui::resources::{UiColors, UiLayout, UiAssets};


pub struct SelectionPanelPlugin;

impl Plugin for SelectionPanelPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiColors>()
            .init_resource::<UiLayout>()
            .init_resource::<UiAssets>()
            // Battle 진입 시 패널 생성
            .add_systems(OnEnter(AppState::Battle), spawn_selection_panel_once)
            // Battle 동안 매 프레임 패널 갱신
            .add_systems(
                Update,
                update_selection_panel_ui
                    .run_if(in_state(AppState::Battle)),
            );
    }
}
