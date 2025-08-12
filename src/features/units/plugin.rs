//! Main plugin for all unit systems

use bevy::prelude::*;
use super::{ally::AllyPlugin, enemy::EnemyPlugin};

/// Plugin that combines all unit-related systems
pub struct UnitsPlugin;

impl Plugin for UnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AllyPlugin,
            EnemyPlugin,
        ));
    }
}