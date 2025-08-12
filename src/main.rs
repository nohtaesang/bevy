use bevy::prelude::*;

mod core;
mod states;
mod features;
mod ui;
mod resources;

use core::AppState;
use states::in_game::InGamePlugin;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins(InGamePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    
}

