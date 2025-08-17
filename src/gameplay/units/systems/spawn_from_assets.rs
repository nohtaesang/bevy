// src/gameplay/units/systems/spawn_from_assets.rs
use bevy::prelude::*;
use crate::gameplay::tiles::events::GridPlace;
use crate::gameplay::units::assets::{PendingUnitLoads};
use crate::gameplay::units::spec::UnitSpec;

/// 로드 완료된 프리셋을 변환→스폰하고 타일에 올리기
pub fn process_loaded_units(
    mut commands: Commands,
    assets: Res<Assets<UnitSpec>>,
    mut pending: ResMut<PendingUnitLoads>,
    mut ev_place: EventWriter<GridPlace>,
) {
    let mut remaining = Vec::with_capacity(pending.0.len());
    for (handle, pos) in pending.0.drain(..) {
        if let Some(spec) = assets.get(&handle) {
            let bundle = spec.to_bundle();
            let entity = commands.spawn(bundle).id();
            ev_place.write(GridPlace { entity, at: pos }); // ← 필드명이 pos 라면 이렇게
        } else {
            remaining.push((handle, pos)); // 아직 로드 안됨 → 다음 프레임 재시도
        }
    }
    pending.0 = remaining;
}



pub fn has_pending(pending: Option<Res<PendingUnitLoads>>) -> bool {
    pending.map_or(false, |p| !p.0.is_empty())
}