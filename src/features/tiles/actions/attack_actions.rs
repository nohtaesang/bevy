//! Attack action functions
//!
//! This module contains action functions for handling attack behavior

use bevy::prelude::*;
use crate::{
    states::in_game::{SelectionState, UnitCommandState},
    features::{
        tiles::SelectionCtx,
        tiles::overlay::attack::AttackValidation,
        units::{Unit, Enemy},
    },
};

/// Perform an attack action if the target position is valid
/// 
/// This function:
/// - Checks if the clicked position is a valid attack target
/// - Finds the enemy at that position
/// - Consumes an attack count from the attacking unit
/// - Deals damage equal to the attacker's attack power to the enemy's health
pub fn perform_attack_action(
    target_pos: IVec2,
    attacker_entity: Entity,
    target_enemy_entity: Entity,
    attack_validation: &AttackValidation,
    unit_query: &mut Query<&mut Unit>,
    enemy_query: &mut Query<&mut Enemy>,
    _next_selection_state: &mut ResMut<NextState<SelectionState>>,
    next_action_state: &mut ResMut<NextState<UnitCommandState>>,
    _selection_ctx: &mut ResMut<SelectionCtx>,
) -> bool {
    // Check if the position is a valid attack target
    if !attack_validation.is_valid_attack(target_pos) {
        return false;
    }

    // Get the attacking unit
    let mut attacker = match unit_query.get_mut(attacker_entity) {
        Ok(unit) => unit,
        Err(_) => {
            return false;
        }
    };

    // Check if unit has attacks left
    
    if !attacker.can_attack() {
        return false;
    }

    // Get the target enemy
    let mut target_enemy = match enemy_query.get_mut(target_enemy_entity) {
        Ok(enemy) => enemy,
        Err(_) => return false,
    };

    // Consume attack count
    attacker.use_attack();
    
    // Deal damage
    target_enemy.health -= attacker.attack_power;
    
    // Ensure health doesn't go below 0
    if target_enemy.health < 0 {
        target_enemy.health = 0;
    }

    info!(
        "Unit at {:?} attacked enemy at {:?} for {} damage. Enemy health: {}/{}",
        attacker.tile_pos,
        target_pos,
        attacker.attack_power,
        target_enemy.health,
        target_enemy.max_health
    );

    // If unit has no more attacks, return to idle state
    if !attacker.can_attack() {
        next_action_state.set(UnitCommandState::Idle);
    }

    true
}