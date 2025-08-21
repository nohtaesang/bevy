

// =============================================
// src/domain/units/components.rs
// =============================================
use bevy::prelude::*;
use crate::domain::map::grid_index::GridPos;


/// 전투 유닛 마커(도메인 엔티티)
#[derive(Component)]
pub struct Unit;


/// 팀 구분 (간단판)
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TeamId { Ally, Enemy }


/// 유닛의 그리드 좌표(도메인 좌표)
/// - 뷰는 이 값을 읽어서 world 좌표로 변환해 렌더
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnitGrid(pub GridPos);



// =============================================
// PATCH: src/domain/units/components.rs — 이동력 컴포넌트 추가
// =============================================
// ... 기존 use들과 Unit/TeamId/UnitGrid 위는 동일
#[derive(Component, Clone, Copy, Debug)]
pub struct UnitMove { pub max_steps: u32 }
impl Default for UnitMove { fn default() -> Self { Self { max_steps: 5 } } }

