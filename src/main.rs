mod app;
mod gameplay;
mod view;
mod input;

use bevy::prelude::*;
use crate::app::plugin::AppStatesPlugin;
use crate::view::plugin::ViewPlugin;
use crate::input::InputPlugin;
use crate::gameplay::plugin::GameplayPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(AppStatesPlugin)
        // 화면/입력은 상위에서
        .add_plugins(ViewPlugin)
        .add_plugins(InputPlugin)
        // 게임플레이 한 방에 묶기
        .add_plugins(GameplayPlugin)
        .run();
}
