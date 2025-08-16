use bevy::prelude::*;
use super::components::GridPos;

/// --- 단건 배치/제거/이동 (SSOT: GridPos) ---
#[derive(Event, Clone, Copy, Debug)]
pub struct GridPlace { pub entity: Entity, pub at: GridPos }
#[derive(Event, Clone, Copy, Debug)]
pub struct GridRemove { pub entity: Entity, pub at: GridPos }
#[derive(Event, Clone, Copy, Debug)]
pub struct GridMove   { pub entity: Entity, pub from: GridPos, pub to: GridPos }

impl GridPlace  { #[inline] pub fn new(entity: Entity, at: GridPos) -> Self { Self { entity, at } } }
impl GridRemove { #[inline] pub fn new(entity: Entity, at: GridPos) -> Self { Self { entity, at } } }
impl GridMove   { #[inline] pub fn new(entity: Entity, from: GridPos, to: GridPos) -> Self { Self { entity, from, to } } }

/// 그리드 조작 명령(한 프레임 내에서 순차 적용)
#[derive(Clone, Copy, Debug)]
pub enum GridCommand {
    Place  { entity: Entity, at: GridPos },
    Remove { entity: Entity, at: GridPos },
    Move   { entity: Entity, from: GridPos, to: GridPos },
    /// 두 엔티티의 위치를 서로 교환(시스템에서 각 엔티티의 현재 위치를 조회)
    Swap   { a: Entity, b: Entity },
    /// 해당 셀 비우기
    Clear  { at: GridPos },
}

impl GridCommand {
    #[inline] pub fn place(entity: Entity, at: GridPos) -> Self { Self::Place { entity, at } }
    #[inline] pub fn remove(entity: Entity, at: GridPos) -> Self { Self::Remove { entity, at } }
    #[inline] pub fn r#move(entity: Entity, from: GridPos, to: GridPos) -> Self { Self::Move { entity, from, to } }
    #[inline] pub fn swap(a: Entity, b: Entity) -> Self { Self::Swap { a, b } }
    #[inline] pub fn clear(at: GridPos) -> Self { Self::Clear { at } }
}

/// 여러 연산을 순서대로 적용하기 위한 배치 이벤트
#[derive(Event, Clone, Debug)]
pub struct GridBatch {
    pub ops: Vec<GridCommand>,
}

impl GridBatch {
    #[inline] pub fn new(ops: Vec<GridCommand>) -> Self { Self { ops } }
    #[inline] pub fn with_capacity(cap: usize) -> Self { Self { ops: Vec::with_capacity(cap) } }
    #[inline] pub fn single(op: GridCommand) -> Self { Self { ops: vec![op] } }
    #[inline] pub fn push(&mut self, op: GridCommand) { self.ops.push(op); }
    #[inline] pub fn extend<I: IntoIterator<Item = GridCommand>>(&mut self, it: I) { self.ops.extend(it); }
}

/// --- 맵 교체/리사이즈 트리거 ---
/// BaseTileMap 리소스를 갱신한 '이후'에 발행.
/// GridIndex를 어떻게 초기화할지 모드를 함께 전달.
#[derive(Event, Clone, Copy, Debug)]
pub struct MapReinitialized {
    pub w: u32,
    pub h: u32,
    pub mode: ReindexMode,
}

/// GridIndex 초기화 모드
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReindexMode {
   /// 새 크기로 리사이즈하고 전부 비움
   Clear,
   /// 기존 배치를 그대로 유지하고, 그리드만 '확장'(축소는 거부/경고)
   PreserveExpand,
}
