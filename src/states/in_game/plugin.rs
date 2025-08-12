use bevy::prelude::*;
use crate::{
    states::in_game::TurnState, 
    features::{
        camera::CameraPlugin,
        tiles::TilesPlugin,
        units::UnitsPlugin,
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
                UnitsPlugin,
                UIPlugin,
                TempPlugin,
            ));
    }
}

