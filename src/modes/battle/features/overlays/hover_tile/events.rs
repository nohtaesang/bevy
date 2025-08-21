// =============================================
// src/modes/battle/features/overlays/hover_tile/events.rs
// =============================================
use crate::domain::map::grid_index::GridPos;
use bevy::prelude::*;

/// 커서가 가리키는 그리드 타일이 바뀌었음을 알리는 적용 결과 이벤트
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct HoverTileChangedApplied {
    pub grid: Option<GridPos>,
}

/// (선택) 뷰 동기화 완료 알림
#[derive(Event, Clone, Copy, Debug, Default)]
pub struct HoverTileUiSynced;
