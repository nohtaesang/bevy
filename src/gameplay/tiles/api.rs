// src/tiles/api.rs
use bevy::prelude::*;
use super::{
    components::GridPos,
    events::{
        GridPlace, GridRemove, GridMove, GridBatch, GridCommand, MapReinitialized, ReindexMode,
    },
    resources::{BaseTileMap, GridIndex},
};

/// 타일 모듈을 쓰는 쪽(입력/게임플레이/스폰)에서 호출하는 편의 API.
/// 내부적으로 이벤트를 발행하고, 실제 적용은 tiles::systems가 처리합니다.
#[derive(SystemParam)]
pub struct TilesApi<'w, 's> {
    /// 맵/인덱스는 존재하지 않을 수도 있으므로 Option
    pub map:   Option<Res<'w, BaseTileMap>>,
    pub index: Option<Res<'w, GridIndex>>,

    place_w:  EventWriter<'w, 's, GridPlace>,
    remove_w: EventWriter<'w, 's, GridRemove>,
    move_w:   EventWriter<'w, 's, GridMove>,
    batch_w:  EventWriter<'w, 's, GridBatch>,
    reinit_w: EventWriter<'w, 's, MapReinitialized>,
}

impl<'w, 's> TilesApi<'w, 's> {
    // ===== 조회/유틸 =====

    /// 맵 크기 반환 (없으면 None)
    #[inline]
    pub fn map_size(&self) -> Option<(u32, u32)> {
        self.map.as_ref().map(|m| (m.w, m.h))
    }

    /// 경계 체크(맵이 없으면 None)
    #[inline]
    pub fn in_bounds(&self, p: GridPos) -> Option<bool> {
        self.map.as_ref().map(|m| m.in_bounds_pos(p))
    }

    /// 해당 칸이 비었는지(인덱스가 없거나 경계 밖이면 None)
    #[inline]
    pub fn is_empty(&self, p: GridPos) -> Option<bool> {
        match (self.index.as_ref(), self.map.as_ref()) {
            (Some(idx), Some(map)) if map.in_bounds_pos(p) => Some(idx.is_empty_pos(p)),
            _ => None,
        }
    }

    // ===== 단건 이벤트 =====

    /// 단건 배치 (검증은 시스템에서 수행)
    #[inline]
    pub fn place(&mut self, entity: Entity, at: GridPos) {
        self.place_w.send(GridPlace { entity, at });
    }

    /// 단건 제거 (엔티티/좌표가 일치해야 제거되도록 시스템에서 검증)
    #[inline]
    pub fn remove(&mut self, entity: Entity, at: GridPos) {
        self.remove_w.send(GridRemove { entity, at });
    }

    /// 단건 이동
    #[inline]
    pub fn move_to(&mut self, entity: Entity, from: GridPos, to: GridPos) {
        self.move_w.send(GridMove { entity, from, to });
    }

    /// 자리 교환 (시스템이 각 엔티티의 현재 위치를 조회)
    #[inline]
    pub fn swap(&mut self, a: Entity, b: Entity) {
        self.command(GridCommand::swap(a, b));
    }

    /// 단일 셀 비우기
    #[inline]
    pub fn clear(&mut self, at: GridPos) {
        self.command(GridCommand::clear(at));
    }

    // ===== 배치 이벤트 =====

    /// 단일 커맨드 전송
    #[inline]
    pub fn command(&mut self, cmd: GridCommand) {
        self.batch_w.send(GridBatch::single(cmd));
    }

    /// 여러 커맨드 일괄 전송
    #[inline]
    pub fn batch<I: IntoIterator<Item = GridCommand>>(&mut self, ops: I) {
        self.batch_w.send(GridBatch { ops: ops.into_iter().collect() });
    }

    /// 대량 배치 (엔티티, 위치)
    pub fn place_many<I: IntoIterator<Item = (Entity, GridPos)>>(&mut self, items: I) {
        let ops = items
            .into_iter()
            .map(|(entity, at)| GridCommand::place(entity, at))
            .collect();
        self.batch_w.send(GridBatch { ops });
    }

    /// 대량 제거 (엔티티/좌표 일치 기준)
    #[inline]
    pub fn remove_many<I: IntoIterator<Item = (Entity, GridPos)>>(&mut self, items: I) {
        let ops = items
            .into_iter()
            .map(|(entity, at)| GridCommand::remove(entity, at))
            .collect();
        self.batch_w.send(GridBatch { ops });
    }

    /// 대량 이동 (엔티티, from, to)
    #[inline]
    pub fn move_many<I: IntoIterator<Item = (Entity, GridPos, GridPos)>>(&mut self, items: I) {
        let ops = items
            .into_iter()
            .map(|(entity, from, to)| GridCommand::r#move(entity, from, to))
            .collect();
        self.batch_w.send(GridBatch { ops });
    }

    /// 여러 칸 비우기
    #[inline]
    pub fn clear_many<I: IntoIterator<Item = GridPos>>(&mut self, cells: I) {
        let ops = cells.into_iter().map(GridCommand::clear).collect();
        self.batch_w.send(GridBatch { ops });
    }

    // ===== 맵 리사이즈/재초기화 =====

    /// 맵 리사이즈 + 전체 초기화(모두 비움)
    #[inline]
    pub fn resize_clear(&mut self, w: u32, h: u32) {
        self.reinit_w.send(MapReinitialized { w, h, mode: ReindexMode::Clear });
    }

    /// 기존 배치 보존 + 확장(축소는 거부)
    #[inline]
    pub fn resize_preserve_expand(&mut self, w: u32, h: u32) {
        self.reinit_w.send(MapReinitialized { w, h, mode: ReindexMode::PreserveExpand });
    }
}
