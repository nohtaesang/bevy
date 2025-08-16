use bevy::prelude::*;
use crate::{
    states::in_game::TurnState, 
    features::{
        camera::CameraPlugin,
        tiles::TilesPlugin,
        scenario::Level1Plugin,
    },
    ui::UIPlugin,
    temp::TempPlugin,
};

/// Plugin that handles all InGame state functionality
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize turn state
            .add_sub_state::<TurnState>()
            
            // Add all feature plugins
            .add_plugins((
                CameraPlugin,
                TilesPlugin,
                Level1Plugin,  // Level 1 scenario with 11x11 grid and center player
                UIPlugin,
                TempPlugin,
            ));
    }
}

