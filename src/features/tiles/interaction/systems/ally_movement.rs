//! Movement range calculation for ally units
//!
//! Calculates valid movement positions when an ally unit is selected

use bevy::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use crate::features::tiles::{
    core::{TileConfig, TileMap, Team, components::TileCoords, TileContent},
    selection::SelectionCtx,
    units::{
        bundles::UnitMarker,
        components::stats::CurrentStats,
    },
    interaction::MovementValidation,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileOcc {
    Empty,
    Ally,
    Other, // 적 유닛, 벽/장애물 등 "아군 이외"
}

#[derive(Clone, Copy, Debug)]
pub struct MovementOptions {
    pub max_range: u32,
    pub allow_diagonal: bool,

    // 통과 정책
    pub pass_through_allies: bool,
    pub pass_through_others: bool,

    pub include_start: bool,
}

impl Default for MovementOptions {
    fn default() -> Self {
        Self {
            max_range: 0,
            allow_diagonal: false,
            pass_through_allies: true,
            pass_through_others: false,
            include_start: false,
        }
    }
}

/// in_bounds: 좌표 유효 여부
/// classify : 좌표 → (Empty | Ally | Other)
pub fn movement_range_bfs<FIn, FCf>(
    start: IVec2,
    opts: MovementOptions,
    in_bounds: FIn,
    classify: FCf,
) -> HashMap<IVec2, u32>
where
    FIn: Fn(IVec2) -> bool,
    FCf: Fn(IVec2) -> TileOcc,
{
    let dirs4 = [IVec2::X, -IVec2::X, IVec2::Y, -IVec2::Y];
    let dirs8 = [
        IVec2::X, -IVec2::X, IVec2::Y, -IVec2::Y,
        IVec2::new( 1,  1), IVec2::new( 1, -1),
        IVec2::new(-1,  1), IVec2::new(-1, -1),
    ];
    let dirs = if opts.allow_diagonal { &dirs8[..] } else { &dirs4[..] };

    let mut dist: HashMap<IVec2, u32> = HashMap::new();
    let mut q = VecDeque::new();

    dist.insert(start, 0);
    q.push_back(start);

    while let Some(pos) = q.pop_front() {
        let d = dist[&pos];
        if d == opts.max_range { continue; }

        for dir in dirs {
            let nx = pos + *dir;
            if !in_bounds(nx) { continue; }

            // 통과 가능?
            let occ = classify(nx);
            let can_traverse = match occ {
                TileOcc::Empty => true,
                TileOcc::Ally  => opts.pass_through_allies,
                TileOcc::Other => opts.pass_through_others,
            };
            if !can_traverse { continue; }

            if !dist.contains_key(&nx) {
                dist.insert(nx, d + 1);
                q.push_back(nx);
            }
        }
    }

    // 착지 가능한 타일만 남기기 + 시작 포함 옵션
    dist.into_iter()
        .filter(|(p, _)| {
            if *p == start && !opts.include_start { return false; }
            match classify(*p) {
                TileOcc::Empty => true,
                TileOcc::Ally  => false, // 아군 위에는 착지 불가
                TileOcc::Other => false, // 적/장애물 위에는 착지 불가
            }
        })
        .collect()
}

/// System that updates movement validation when an ally is selected
pub fn update_ally_movement_range(
    selection_ctx: Res<SelectionCtx>,
    tile_config: Res<TileConfig>,
    tile_map: Res<TileMap>,
    unit_query: Query<(&Team, &TileCoords, &CurrentStats), With<UnitMarker>>,
    mut movement_validation: ResMut<MovementValidation>,
) {
    // Clear existing validation
    movement_validation.clear();
    
    // Check if an ally unit is selected
    let Some(selected_entity) = selection_ctx.selected_unit else {
        return;
    };
    
    // Get unit information
    let Ok((team, tile_coords, current_stats)) = unit_query.get(selected_entity) else {
        return;
    };
    
    // Only calculate for player units
    if *team != Team::Player {
        return;
    }
    
    // Get movement range
    let movement_range = current_stats.move_range;
    if movement_range <= 0 {
        return;
    }
    
    // Set up movement options for basic movement
    let opts = MovementOptions {
        max_range: movement_range as u32,
        allow_diagonal: false, // Can be made configurable later
        pass_through_allies: true,
        pass_through_others: false,
        include_start: false,
    };
    
    let unit_pos = IVec2::new(tile_coords.x, tile_coords.y);
    
    // Create closures for bounds checking and tile classification
    let in_bounds = |pos: IVec2| {
        pos.x >= 0 && pos.x < tile_config.grid_size && 
        pos.y >= 0 && pos.y < tile_config.grid_size
    };
    
    let classify = |pos: IVec2| {
        match tile_map.get_content(pos) {
            TileContent::Empty => TileOcc::Empty,
            TileContent::Unit(entity) => {
                // Check if it's an ally or other unit
                if let Ok((unit_team, _, _)) = unit_query.get(entity) {
                    match unit_team {
                        Team::Player => TileOcc::Ally,
                        _ => TileOcc::Other,
                    }
                } else {
                    TileOcc::Other
                }
            },
            _ => TileOcc::Other,
        }
    };
    
    // Calculate reachable tiles using the new BFS function
    let result = movement_range_bfs(unit_pos, opts, in_bounds, classify);
    
    // Convert to HashSet for the validation resource
    let valid_moves: HashSet<IVec2> = result.keys().copied().collect();
    
    // Update the validation resource
    movement_validation.set_valid_moves(valid_moves);
}
