use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackDirection {
    Cardinal,  // 상하좌우만
    EightWay,  // 대각 포함 8방향
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    Direct,    // 직사: 유닛/장애물 넘겨쏘기 불가(시야 필요)
    Indirect,  // 곡사: 넘겨쏘기 가능(시야 불필요)
}

#[derive(Component, Debug, Clone, Copy)]
pub struct AttackProfile {
    pub direction: AttackDirection,
    pub kind: AttackType,
}

impl AttackProfile {
    pub fn new(direction: AttackDirection, kind: AttackType) -> Self {
        Self { direction, kind }
    }
}