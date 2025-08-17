use bevy::prelude::*;
use crate::gameplay::tiles::components::GridPos;
use super::state::PlayerIntent;

/// 상호작용 컨텍스트 - 단일 진실(리소스)
#[derive(Resource, Debug)]
pub struct SelectionCtx {
    pub hover_tile: Option<GridPos>,
    pub selected_tile: Option<GridPos>,
    pub selected_unit: Option<Entity>, // 유닛 시스템 붙일 때 채움
    pub intent: PlayerIntent,
}

impl Default for SelectionCtx {
    fn default() -> Self {
        Self {
            hover_tile: None,
            selected_tile: None,
            selected_unit: None,
            intent: PlayerIntent::Idle,
        }
    }
}

/// 변화 요약을 계산해주는 편의 메서드들
impl SelectionCtx {
    /// Hover 타일 갱신 (변화 여부 반환)
    pub fn set_hover(&mut self, p: Option<GridPos>) -> bool {
        let changed = self.hover_tile != p;
        self.hover_tile = p;
        changed
    }

    /// 선택 타일/유닛 갱신 (tile_changed, unit_changed 반환)
    pub fn set_selection(
        &mut self,
        tile: Option<GridPos>,
        unit: Option<Entity>,
    ) -> (bool, bool) {
        let tile_changed = self.selected_tile != tile;
        let unit_changed = self.selected_unit != unit;
        self.selected_tile = tile;
        self.selected_unit = unit;
        (tile_changed, unit_changed)
    }

    /// 선택 해제 (tile_changed, unit_changed 반환)
    pub fn clear_selection(&mut self) -> (bool, bool) {
        self.set_selection(None, None)
    }

    /// 의도 전환 (이전/새 의도 반환)
    pub fn set_intent(&mut self, new_intent: PlayerIntent) -> (PlayerIntent, PlayerIntent) {
        let prev = self.intent;
        self.intent = new_intent;
        (prev, new_intent)
    }
}
