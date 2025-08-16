use bevy::prelude::*;
use crate::gameplay::tiles::components::{GridPos, TerrainKind};

#[derive(Component)]
pub struct TileSprite;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TileVisual { pub kind: TerrainKind }
impl TileVisual { #[inline] pub fn new(kind: TerrainKind) -> Self { Self { kind } } }

/// 0.16 스타일: 렌더링에 필요한 모든 컴포넌트 포함
#[derive(Bundle)]
pub struct TileSpriteBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
    pub grid: GridPos,
    pub marker: TileSprite,
    pub visual: TileVisual,
}

impl TileSpriteBundle {
    pub fn new(grid: GridPos, visual: TileVisual, sprite: Sprite, transform: Transform) -> Self {
        Self {
            sprite,
            transform,
            global_transform: GlobalTransform::default(),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
            grid,
            marker: TileSprite,
            visual,
        }
    }
}
