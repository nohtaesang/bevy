

// =============================================
// src/modes/battle/features/overlays/move_range/utils.rs
// =============================================
use bevy::prelude::*;
use crate::domain::map::components::{Map, TerrainKind};
use crate::domain::map::grid_index::GridPos;
use crate::domain::map::grid_index::OccupancyIndex;
use crate::domain::units::components::TeamId;
use bevy::prelude::Entity;


#[inline]
fn passable(kind: TerrainKind) -> bool { matches!(kind, TerrainKind::Ground) }


#[inline]
fn neighbors(p: GridPos, size: UVec2) -> impl Iterator<Item = GridPos> {
let mut v = Vec::with_capacity(4);
if p.x > 0 { v.push(GridPos { x: p.x - 1, y: p.y }); }
if p.y > 0 { v.push(GridPos { x: p.x, y: p.y - 1 }); }
if p.x + 1 < size.x { v.push(GridPos { x: p.x + 1, y: p.y }); }
if p.y + 1 < size.y { v.push(GridPos { x: p.x, y: p.y + 1 }); }
v.into_iter()
}


/// BFS로 이동 가능 타일을 계산 (4방향)
/// - 벽(TerrainKind::Wall) 통과 불가
/// - 점유된 칸 통과/도착 불가(시작 칸은 예외)
/// - 최대 `max_steps` 칸

pub fn compute_reachable_teamaware<F>(
    map: &Map,
    occ: &OccupancyIndex,
    start: GridPos,
    max_steps: u32,
    mover_team: TeamId,
    mut team_of: F,                // 점유 엔티티의 TeamId를 얻는 함수/클로저
) -> Vec<GridPos>
where
    F: FnMut(Entity) -> Option<TeamId>,
{
    let size = UVec2::new(map.size.w, map.size.h);
    let mut out = Vec::new();
    let mut dist = vec![u32::MAX; (map.size.w * map.size.h) as usize];
    let mut q = std::collections::VecDeque::new();
    let idx = |p: GridPos| -> usize { (p.x + p.y * map.size.w) as usize };

    dist[idx(start)] = 0;
    q.push_back(start);

    while let Some(p) = q.pop_front() {
        let d = dist[idx(p)];
        if d >= max_steps { continue; }

        for n in neighbors(p, size) {
            // 지형 통과 여부
            if !passable(map.tiles[idx(n)]) { continue; }

            // 점유 상태 확인
            let occ_entity = occ.occupant(n);
            let occ_team = occ_entity.and_then(|e| team_of(e));

            match occ_team {
                // 적: 완전 차단 (통과/도착 불가)
                Some(t) if t != mover_team => continue,

                // 아군: 통과만 가능, 도착 불가
                Some(_same_team) => {
                    if dist[idx(n)] <= d + 1 { continue; }
                    dist[idx(n)] = d + 1;
                    q.push_back(n);      // 큐에는 넣어서 그 너머로 확장
                    // out.push(n) 는 안 함 (도착지로 인정하지 않음)
                }

                // 빈칸: 도착/통과 모두 가능
                None => {
                    if dist[idx(n)] <= d + 1 { continue; }
                    dist[idx(n)] = d + 1;
                    out.push(n);         // 도착 가능
                    q.push_back(n);
                }
            }
        }
    }
    out
}

