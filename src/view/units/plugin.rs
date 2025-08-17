use bevy::prelude::*;

use crate::gameplay::units::resources::{Teams, TeamColors};
use super::{
    resources::UnitViewConfig,
    systems::{spawn_unit_sprites_for_new_units, sync_unit_sprites},
};

pub struct UnitViewPlugin;

impl Plugin for UnitViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UnitViewConfig>()
            .init_resource::<Teams>()
            .init_resource::<TeamColors>();

        // 타일이 먼저 스폰된 다음에 위치를 잡는 편이 시각적으로 자연스럽습니다.
        // 필요하면 .after(타일 스폰 시스템) 으로 순서 지정 가능.
        app.add_systems(
            Update,
            (
                spawn_unit_sprites_for_new_units,
                sync_unit_sprites,
            ),
        );
    }
}
