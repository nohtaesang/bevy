use bevy::prelude::*;

use crate::{
    gameplay::{
        interaction::resources::SelectionCtx,
        units::components::{Unit, TeamId, Stats, MoveBudget, ActionBudget, Attack},
        units::resources::{TeamColors, Teams},
    },
};

use super::components::*;
use crate::view::ui::resources::{UiColors, UiLayout, UiAssets};

/// 패널을 한 번만 생성 (화면 고정 우상단)
pub fn spawn_selection_panel_once(
    mut commands: Commands,
    colors: Res<UiColors>,
    layout: Res<UiLayout>,
    q_existing: Query<Entity, With<SelectionPanelRoot>>,
) {
    if !q_existing.is_empty() {
        return;
    }

    let root = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(12.0),
                right: Val::Px(12.0),
                width: Val::Px(layout.panel_width),
                padding: UiRect::all(Val::Px(layout.panel_padding)),
                row_gap: Val::Px(layout.panel_gap),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                ..default()
            },
            BackgroundColor(colors.panel_bg),
            BorderColor(colors.panel_border),
            SelectionPanelRoot,
            Visibility::Hidden, // 기본은 숨김 (선택 없을 때)
        ))
        .id();

    // 자식 텍스트들 순서대로 생성
    commands.entity(root).with_children(|p| {
        p.spawn((Text::new(""), SelNameText));
        p.spawn((Text::new(""), SelHpText));
        p.spawn((Text::new(""), SelShieldText));
        p.spawn((Text::new(""), SelMoveApText));
        p.spawn((Text::new(""), SelAttackText));
    });
}

/// 선택된 유닛 정보를 패널에 바인딩 / 가시성 토글
pub fn update_selection_panel_ui(
    ctx: Res<SelectionCtx>,
    ui_colors: Res<UiColors>,
    _ui_layout: Res<UiLayout>,
    ui_assets: Res<UiAssets>,
    teams: Res<Teams>,
    team_colors: Res<TeamColors>,

    mut q_panel: Query<(&mut Visibility, &mut BorderColor), With<SelectionPanelRoot>>,

    mut q_name: Query<&mut Text, With<SelNameText>>,
    mut q_hp: Query<&mut Text, (With<SelHpText>, Without<SelNameText>)>,
    mut q_shield: Query<&mut Text, (With<SelShieldText>, Without<SelNameText>, Without<SelHpText>)>,
    mut q_move_ap: Query<&mut Text, (With<SelMoveApText>, Without<SelNameText>, Without<SelHpText>, Without<SelShieldText>)>,
    mut q_attack: Query<&mut Text, (With<SelAttackText>, Without<SelNameText>, Without<SelHpText>, Without<SelShieldText>, Without<SelMoveApText>)>,

    q_unit: Query<(
        Option<&Name>,
        Option<&TeamId>,
        Option<&Stats>,
        Option<&MoveBudget>,
        Option<&ActionBudget>,
        Option<&Attack>,
    ), With<Unit>>,
) {
    // SelectionCtx 또는 폰트/테마가 바뀌지 않았다면 일찍 종료해도 됨 (선택)
    if !(ctx.is_changed() || ui_assets.is_changed() || ui_colors.is_changed() || teams.is_changed() || team_colors.is_changed()) {
        // 계속 업데이트하고 싶다면 이 게이트를 지워도 OK
    
    }

    let Ok((mut vis, mut border)) = q_panel.single_mut() else { return; };

    let Some(selected) = ctx.selected_unit else {
        *vis = Visibility::Hidden;
        return;
    };

    // 유닛 쿼리
    let Ok((
        name_opt,
        team_opt,
        stats_opt,
        move_opt,
        act_opt,
        attack_opt,
    )) = q_unit.get(selected) else {
        *vis = Visibility::Hidden;
        return;
    };

    *vis = Visibility::Visible;

    // 내용 채우기
    // 이름 라인 + 팀 배색(테두리)
    let (team_id, team_col) = if let Some(team) = team_opt {
        let col = if *team == teams.ally { team_colors.ally }
                  else if *team == teams.enemy { team_colors.enemy }
                  else { ui_colors.panel_border }; // fallback: 기존 보더
        (team.0, col)
    } else {
        (255u8, ui_colors.panel_border)
    };

    if let Ok(mut t) = q_name.single_mut() {
        let name_str = name_opt.map(|n| n.as_str()).unwrap_or("Unknown Unit");
        t.0 = format!("{name}  (Team {team})", name = name_str, team = team_id);
        // 패널 보더 색도 팀 색으로 살짝 강조
        border.0 = team_col;
    }

    // HP / Shield
    if let Some(st) = stats_opt {
        if let Ok(mut t) = q_hp.single_mut() {
            t.0 = format!("HP: {}/{}", st.hp, st.max_hp);
        }
        if let Ok(mut t) = q_shield.single_mut() {
            t.0 = format!("Shield: {}/{}", st.shield, st.max_shield);
        }
    } else {
        if let Ok(mut t) = q_hp.single_mut() { t.0.clear(); }
        if let Ok(mut t) = q_shield.single_mut() { t.0.clear(); }
    }

    // Move/AP
    if let (Some(mv), Some(ap)) = (move_opt, act_opt) {
        if let Ok(mut t) = q_move_ap.single_mut() {
            t.0 = format!("Move: {}/{}   AP: {}/{}", mv.current, mv.per_turn, ap.current, ap.per_turn);
        }
    } else if let Ok(mut t) = q_move_ap.single_mut() {
        t.0.clear();
    }

    // Attack
    if let Some(atk) = attack_opt {
        let dirs = match atk.dirs { crate::gameplay::units::components::AimDirs::Four => "4-dir", _ => "8-dir" };
        if let Ok(mut t) = q_attack.single_mut() {
            t.0 = format!(
                "ATK: dmg {}  range {}  crit {}% x{:.1}  ({})",
                atk.damage,
                atk.range,
                (atk.crit_chance * 100.0).round(),
                atk.crit_mult,
                dirs
            );
        }
    } else if let Ok(mut t) = q_attack.single_mut() {
        t.0.clear();
    }
}