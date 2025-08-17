use bevy::prelude::*;

/// Hover 네모 오버레이 마커
#[derive(Component, Debug)]
pub struct HoverTileOverlay;

/// Selected 네모 오버레이 마커
#[derive(Component, Debug)]
pub struct SelectedTileOverlay;

/// 네모 오버레이용 스프라이트 번들
/// - 기본은 Hidden
/// - 크기/위치는 시스템에서 타일 크기/그리드 좌표에 맞춰 갱신
#[derive(Bundle, Debug)]
pub struct OverlaySpriteBundle {
    pub sprite: Sprite,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl OverlaySpriteBundle {
    /// color만 지정(크기/위치는 시스템에서 설정)
    pub fn new_hidden(color: Color) -> Self {
        Self {
            sprite: Sprite {
                color,
                // 크기는 시스템에서 tile_size 기반으로 채움
                custom_size: None,
                ..default()
            },
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            visibility: Visibility::Hidden,
            inherited_visibility: InheritedVisibility::default(),
            view_visibility: ViewVisibility::default(),
        }
    }
}
