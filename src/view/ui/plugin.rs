// view/ui/plugin.rs
use bevy::prelude::*;
use super::selection_panel::plugin::SelectionPanelPlugin;

/// 인게임 UI(선택 패널 등)를 묶는 상위 플러그인
pub struct UiViewPlugin;

impl Plugin for UiViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SelectionPanelPlugin);
    }
}
