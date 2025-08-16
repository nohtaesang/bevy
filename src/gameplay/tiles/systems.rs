use bevy::prelude::*;
use super::{
    components::GridPos,
    resources::{BaseTileMap, GridIndex, GridError, TileConfig},
    events::{GridPlace, GridRemove, GridMove, GridBatch, GridCommand, MapReinitialized, ReindexMode},
};

/// 1) 맵 재초기화 이벤트 처리: GridIndex 크기/내용 조정
pub fn handle_map_reinitialized(
    mut ev: EventReader<MapReinitialized>,
    mut index: ResMut<GridIndex>,
) {
    for MapReinitialized { w, h, mode } in ev.read().copied() {
        match mode {
            ReindexMode::Clear => {
                index.resize_clear(w, h);
            }
            ReindexMode::PreserveExpand => {
                if let Err(err) = index.expand_preserve(w, h) {
                    warn!("PreserveExpand failed: {:?}", err);
                }
            }
        }
    }
}

/// 2) 단일 이벤트 + 배치 커맨드 적용 → GridIndex & GridPos 컴포넌트 동기화
pub fn apply_grid_events(
    mut commands: Commands,
    map: Option<Res<BaseTileMap>>,
    mut index: ResMut<GridIndex>,
    mut place_r: EventReader<GridPlace>,
    mut remove_r: EventReader<GridRemove>,
    mut move_r: EventReader<GridMove>,
    mut batch_r: EventReader<GridBatch>,
    q_pos: Query<&GridPos>,
) {
    // 유틸: 실제 연산을 수행하고, GridPos 컴포넌트까지 갱신
    let mut do_cmd = |cmd: GridCommand,
                      commands: &mut Commands,
                      map: Option<&BaseTileMap>,
                      index: &mut GridIndex|
     {
        match cmd {
            GridCommand::Place { entity, at } => {
                if let Some(m) = map {
                    if !m.in_bounds_pos(at) {
                        warn!("Place OOB: {:?} at {:?}", entity, at);
                        return;
                    }
                }
                match index.place_pos(at, entity) {
                    Ok(_) => { commands.entity(entity).insert(at); }
                    Err(e) => warn!("Place failed at {:?}: {:?}", at, e),
                }
            }
            GridCommand::Remove { entity, at } => {
                if let Some(m) = map {
                    if !m.in_bounds_pos(at) {
                        warn!("Remove OOB: {:?} at {:?}", entity, at);
                        return;
                    }
                }
                // 먼저 현재 점유자 확인
                match index.get_pos(at) {
                    Some(cur) if cur == entity => {
                        // 이제 지움
                        if let Err(e) = index.clear_pos(at) {
                            warn!("Remove failed at {:?}: {:?}", at, e);
                        } else {
                            commands.entity(entity).remove::<GridPos>();
                        }
                    }
                    Some(other) => {
                        warn!("Remove mismatch: expected {:?} but found {:?}", entity, other);
                    }
                    None => {
                        warn!("Remove empty at {:?}", at);
                    }
                }
            }
            GridCommand::Move { entity, from, to } => {
                if let Some(m) = map {
                    if !m.in_bounds_pos(from) || !m.in_bounds_pos(to) {
                        warn!("Move OOB: {:?} {:?}->{:?}", entity, from, to);
                        return;
                    }
                }
                match index.move_to(from, to) {
                    Ok(_) => { commands.entity(entity).insert(to); }
                    Err(e) => warn!("Move failed {:?}->{:?}: {:?}", from, to, e),
                }
            }
            GridCommand::Swap { a, b } => {
                // O(1): 엔티티의 GridPos를 직접 읽음
                if let (Ok(pa), Ok(pb)) = (q_pos.get(a), q_pos.get(b)) {
                    // (선택) 인덱스와 동기화 검증
                    debug_assert_eq!(index.get_pos(*pa), Some(a));
                    debug_assert_eq!(index.get_pos(*pb), Some(b));

                    if let Err(e) = index.swap_cells(*pa, *pb) {
                        warn!("Swap failed {:?}<->{:?}: {:?}", a, b, e);
                    } else {
                        commands.entity(a).insert(*pb);
                        commands.entity(b).insert(*pa);
                    }
                } else {
                    warn!("Swap failed: GridPos missing for {:?} or {:?}", a, b);
                }
            }
            GridCommand::Clear { at } => {
                if let Some(m) = map {
                    if !m.in_bounds_pos(at) {
                        warn!("Clear OOB at {:?}", at);
                        return;
                    }
                }
                match index.clear_pos(at) {
                    Ok(Some(e)) => {
                        // 그 칸에 있던 엔티티 e의 GridPos 제거
                        commands.entity(e).remove::<GridPos>();
                    }
                    Ok(None) => {
                        // 이미 빈 칸 — 필요하면 로그 생략 가능
                        // trace!("Clear no-op at {:?}", at);
                    }
                    Err(err) => warn!("Clear failed at {:?}: {:?}", at, err),
                }
            }
        }
    };

    // 단일 이벤트들
    for GridPlace { entity, at } in place_r.read().copied() {
        do_cmd(GridCommand::Place { entity, at }, &mut commands, map.as_deref(), &mut index);
    }
    for GridRemove { entity, at } in remove_r.read().copied() {
        do_cmd(GridCommand::Remove { entity, at }, &mut commands, map.as_deref(), &mut index);
    }
    for GridMove { entity, from, to } in move_r.read().copied() {
        do_cmd(GridCommand::Move { entity, from, to }, &mut commands, map.as_deref(), &mut index);
    }

    // 배치 커맨드
    for GridBatch { ops } in batch_r.read() {
        for &op in ops {
            do_cmd(op, &mut commands, map.as_deref(), &mut index);
        }
    }
}

/// 3) GridPos → Transform 동기화 (GridPos가 바뀌거나, 타일 설정이 바뀌면 적용)
pub fn sync_gridpos_transforms(
    tile_cfg: Res<TileConfig>,
    mut q: Query<(Ref<GridPos>, &mut Transform)>,
) {
    let cfg_changed = tile_cfg.is_changed();
    for (gp, mut tr) in &mut q {
        if cfg_changed || gp.is_changed() {
            let z = tr.translation.z;
            tr.translation = tile_cfg.grid_to_world_center(*gp, z);
        }
    }
}
