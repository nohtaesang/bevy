// =============================================
// src/modes/battle/features/spawner_debug/systems_apply.rs
// =============================================
use crate::domain::map::grid_index::{GridPos, OccupancyIndex};
use crate::domain::units::components::{TeamId, Unit, UnitGrid, UnitMove};
use crate::domain::units::events::{UnitSpawnApplied, UnitSpawnRequested};
use bevy::prelude::*;

/// 요청을 처리하여 도메인 유닛 엔티티를 생성하고 점유를 갱신
pub fn apply_unit_spawns(
    mut commands: Commands,
    occ: Option<ResMut<OccupancyIndex>>,
    mut ev_req: EventReader<UnitSpawnRequested>,
    mut ev_applied: EventWriter<UnitSpawnApplied>,
) {
    let Some(mut occ) = occ else { 
        ev_req.clear(); // 이벤트 제거하여 누적 방지
        return; 
    };

    for ev in ev_req.read().copied() {
        let GridPos { x, y } = ev.at;
        // 경계/중복 체크
        if !occ.in_bounds(ev.at) {
            warn!("spawn out of bounds: ({},{})", x, y);
            continue;
        }
        if !occ.is_free(ev.at) {
            warn!("spawn blocked at: ({},{})", x, y);
            continue;
        }

        // 1) 우선 엔티티를 만들어 식별자를 얻는다
        let _mv: UnitMove = match ev.team {
            TeamId::Ally => UnitMove { max_steps: 6 },
            TeamId::Enemy => UnitMove { max_steps: 5 },
        };

        let e = commands.spawn((Unit, ev.team, UnitGrid(ev.at))).id();

        // 2) 점유 시도 (경쟁 상황 대비)
        if occ.claim(ev.at, e) {
            ev_applied.write(UnitSpawnApplied {
                entity: e,
                team: ev.team,
                at: ev.at,
            });
        } else {
            // 실패 시 엔티티 정리
            warn!("race: claim failed at ({},{})", x, y);
            commands.entity(e).despawn();
        }
    }
}
