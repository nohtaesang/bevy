// src/gameplay/units/resources.rs
use bevy::prelude::*;
use super::components::TeamId;

/// 플레이어의 진영 ID (기본: 0)
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerTeamId(pub u8);
impl Default for PlayerTeamId {
    fn default() -> Self { Self(0) }
}

/// 두 팀(아군/적군)만 사용하는 간단한 구성
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Teams {
    pub ally: TeamId,   // 보통 0
    pub enemy: TeamId,  // 보통 1
}
impl Default for Teams {
    fn default() -> Self {
        Self { ally: TeamId(0), enemy: TeamId(1) }
    }
}

/// 팀 색상(디버그/뷰용)
#[derive(Resource, Debug, Clone, Copy)]
pub struct TeamColors {
    pub ally: Color,
    pub enemy: Color,
}
impl Default for TeamColors {
    fn default() -> Self {
        Self {
            ally: Color::srgb(0.2, 0.8, 1.0),
            enemy: Color::srgb(1.0, 0.6, 0.2),
        }
    }
}
