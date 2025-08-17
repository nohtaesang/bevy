// src/view/z_index.rs
use crate::gameplay::tiles::components::GridPos;

/// Z 레이어 간 기본 간격(충분히 크게)
// (타일 -10, 유닛 0, 효과 10, UI 100… 이런 느낌)
pub const LAYER_GAP: f32 = 10.0;

/// 같은 레이어 내에서 미세 정렬할 때 쓸 스텝(아주 작게)
pub const SUB_STEP: f32 = 0.1;

/// 그리드 Y기반 정렬에 쓰는 스텝(더 작게)
// y가 커질수록 "뒤로" 가게 하려면 음수 계수 사용
pub const Y_SORT_STEP: f32 = 0.001;

/// 최상위 레이어들(의미별 베이스 Z)
pub mod layer {
    pub const BACKGROUND: f32 = -100.0;
    pub const TILES: f32      = -10.0;
    pub const HOVER: f32      = -5.0;
    pub const SELECT: f32     = -4.0;
    pub const UNITS: f32      =   0.0;
    pub const EFFECTS: f32    =  10.0;
    pub const UI_WORLD: f32   = 100.0; // 월드 공간 UI(범위 표시 등)
    pub const DEBUG: f32      = 900.0;
}

/// y가 클수록 뒤로 가게 하는 정렬 오프셋
#[inline]
pub fn y_sort_desc_by_front(gp: GridPos) -> f32 {
    // y가 작을수록(아래쪽일수록) 더 앞 (=Z 큼)으로 보이게 음수 계수 적용
    -(gp.y as f32) * Y_SORT_STEP
}

/// 타일 Z: 타일 레이어 베이스
#[inline]
pub fn z_tile(gp: GridPos) -> f32 {
    layer::TILES + y_sort_desc_by_front(gp)
}

/// 유닛 Z: 유닛 레이어 베이스 + y정렬
#[inline]
pub fn z_unit(gp: GridPos) -> f32 {
    layer::UNITS + y_sort_desc_by_front(gp)
}

/// 유닛 위 이펙트(하이라이트/버프 링 등)
#[inline]
pub fn z_effect_above_unit(gp: GridPos) -> f32 {
    layer::EFFECTS + y_sort_desc_by_front(gp)
}

/// 같은 셀에서 조금만 위로 올리고 싶을 때(미세 조정)
#[inline]
pub fn add_sub_layer(z: f32, steps: f32) -> f32 {
    z + steps * SUB_STEP
}
