// src/gameplay/units/assets.rs
use bevy::prelude::*;
use crate::gameplay::tiles::components::GridPos;
use crate::gameplay::units::spec::UnitSpec;

/// 로드 완료 후 스폰할 (스펙핸들, 타일위치) 큐
#[derive(Resource, Default)]
pub struct PendingUnitLoads(pub Vec<(Handle<UnitSpec>, GridPos)>);

/// 외부에서 스폰 큐에 등록할 때 쓰는 헬퍼
pub fn enqueue_unit(
    asset_server: &AssetServer,
    pending: &mut PendingUnitLoads,
    path: &str,
    at: GridPos,
) {
    let handle: Handle<UnitSpec> = asset_server.load(path);
    pending.0.push((handle, at));
}
