// src/domain/map/events.rs
use super::components::MapSize;
use bevy::prelude::*;

/// 맵 초기화 요청 (Requested)
#[derive(Event, Clone, Copy, Debug)]
pub struct MapInitRequested {
    pub size: MapSize,
}

/// 맵 초기화 완료(SSOT 생성 확정) (Applied)
#[derive(Event, Clone, Copy, Debug)]
pub struct MapInitializedApplied {
    pub size: MapSize,
}

/// 뷰 동기화까지 끝났음을 알림 (UiSynced)
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct MapUiSynced;
