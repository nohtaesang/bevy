//! System sets for tiles scheduling
//!
//! Ensures proper ordering: Update(ApplyCommands) → PostUpdate(SyncIndex) → Update(ConsumeIndex)

use bevy::prelude::*;

/// System sets for organizing tile system execution order
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum TilesSet {
    /// Input processing - handle user input, commands
    /// Runs in Update, may emit events
    Input,
    
    /// Apply commands and component changes (원본 수정)
    /// Runs in Update, emits events for GridIndex
    ApplyCommands,
    
    /// Sync spatial index with events (apply_index_updates)
    /// Runs in PostUpdate after all commands are applied
    SyncIndex,
    
    /// Consume spatial index for queries
    /// Runs in Update after index sync (next frame)
    ConsumeIndex,
}

/// Configure system set ordering and scheduling
pub fn configure_system_sets(app: &mut App) {
    app
        // Update: Input → Commands → Consumption
        .configure_sets(Update, (
            TilesSet::Input,
            TilesSet::ApplyCommands,
            TilesSet::ConsumeIndex,
        ).chain())
        
        // PostUpdate: Index synchronization after all commands applied
        .configure_sets(PostUpdate, (
            TilesSet::SyncIndex,
        ));
        
        // Cross-schedule ordering:
        // Update(ApplyCommands) → PostUpdate(SyncIndex) → Next Update(ConsumeIndex)
}