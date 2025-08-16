mod app;
mod tiles;
mod gameplay;
mod view;

use bevy::prelude::*;
use crate::app::plugin::AppStatesPlugin; // 너가 만든 plugin.rs
use crate::view::camera::plugin::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)   // Bevy 기본 플러그인
        .add_plugins(AppStatesPlugin)  // 네가 만든 AppStatesPlugin
        .add_plugins(CameraPlugin)
        .run();
}
