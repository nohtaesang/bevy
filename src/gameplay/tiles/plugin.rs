use bevy::prelude::*;
use super::{
    events::{GridPlace, GridRemove, GridMove, GridBatch, MapReinitialized},
    systems::{handle_map_reinitialized, apply_grid_events, sync_gridpos_transforms},
};

/// 시스템 실행 순서 정의
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TilesSet {
    /// 맵 리사이즈/재인덱싱 처리
    Reindex,
    /// 배치/제거/이동 커맨드 적용
    ApplyCommands,
}

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        
        app
  
            // 이벤트 등록
            .add_event::<GridPlace>()
            .add_event::<GridRemove>()
            .add_event::<GridMove>()
            .add_event::<GridBatch>()
            .add_event::<MapReinitialized>()
            // 순서: Reindex → ApplyCommands
            .configure_sets(PreUpdate, (TilesSet::Reindex, TilesSet::ApplyCommands).chain())
            // PreUpdate: 먼저 맵 재인덱싱
            .add_systems(PreUpdate, handle_map_reinitialized.in_set(TilesSet::Reindex))
            // PreUpdate: 그 다음 커맨드 적용(인덱스/컴포넌트 반영)
            .add_systems(PreUpdate, apply_grid_events.in_set(TilesSet::ApplyCommands))
            // Update: Transform 동기화(렌더 전 위치 갱신)
            .add_systems(Update, sync_gridpos_transforms);
    }
}
