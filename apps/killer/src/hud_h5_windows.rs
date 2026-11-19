use crate::hud_h5::HudView;
use killer_ui::draw::DrawList;
use killer_ui::window::Rect;
use killer_ui::xdat_window::{self, Item};
use l2_dat::actionname::{CATEGORY_BASIC, CATEGORY_PARTY, CATEGORY_SOCIAL};
use l2_dat::xdat::{ItemWindowProps, Props};

pub const ACTION: &str = "ActionWnd";
pub const MAGIC_SKILL: &str = "MagicSkillWnd";
pub const DETAIL_STATUS: &str = "DetailStatusWnd";

pub const SKILL_GROUPS_ACTIVE: usize = 8;
pub const SKILL_GROUPS_PASSIVE: usize = 6;

pub const ACTION_LISTS: [(&str, u32); 3] =
    [("ActionBasicItem", CATEGORY_BASIC), ("ActionPartyItem", CATEGORY_PARTY), ("ActionSocialItem", CATEGORY_SOCIAL)];

#[derive(Default)]
pub struct WindowFill {
    pub values: Vec<(String, String)>,
    pub hidden: Vec<String>,
    pub reports: Vec<String>,
}

impl WindowFill {
    fn value(&mut self, control: &str, text: impl Into<String>) {
        self.values.push((control.to_string(), text.into()));
    }

    fn hide_one(&mut self, control: String) {
        self.hidden.push(control);
    }

    fn hide(&mut self, controls: &[&str]) {
        self.hidden.extend(controls.iter().map(|name| (*name).to_string()));
    }

    fn report(&mut self, what: impl Into<String>) {
        self.reports.push(what.into());
    }
}

pub struct Grid {
    pub cols: i32,
    pub rows: i32,
    pub icon_w: i32,
    pub icon_h: i32,
    pub gap_x: i32,
    pub gap_y: i32,
}

impl Grid {
    pub fn visible(&self) -> i32 {
        self.cols * self.rows
    }

    pub fn cell(&self, origin: Rect, index: i32) -> Rect {
        let col = index % self.cols;
        let row = index / self.cols;
        let x = origin.x + col * (self.icon_w + self.gap_x);
        let y = origin.y + row * (self.icon_h + self.gap_y);
        Rect::new(x, y, self.icon_w, self.icon_h)
    }
}

pub fn grid_of(props: &ItemWindowProps) -> Grid {
    Grid {
        cols: props.head[0].max(1),
        rows: props.head[1].max(1),
        icon_w: props.head[3].max(1),
        icon_h: props.head[4].max(1),
        gap_x: props.head[5].max(0),
        gap_y: props.head[6].max(0),
    }
}

fn item_window_props<'a>(item: &'a Item<'a>) -> Option<&'a ItemWindowProps> {
    match &item.control.props {
        Props::ItemWindow(props) => Some(&**props),
        _ => None,
    }
}

fn blit_icon(list: &mut DrawList, rect: Rect, icon: &str, frame: &str) {
    if icon.is_empty() || icon == xdat_window::UNDEFINED {
        return;
    }
    list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, icon, 1.0);
    if !frame.is_empty() && frame != xdat_window::UNDEFINED {
        list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, frame, 1.0);
    }
}

pub fn action_icons(view: &HudView, category: u32) -> Vec<String> {
    let Some(tables) = view.tables else { return Vec::new() };
    tables.actions.iter().filter(|action| action.category == category).map(|action| action.icon.clone()).collect()
}

pub fn bind_action(view: &HudView) -> WindowFill {
    let mut fill = WindowFill::default();
    if view.tables.is_none() {
        fill.hide(&["ActionBasicItem", "ActionPartyItem", "ActionSocialItem"]);
        fill.report("ActionWnd: tabelas .dat ausentes; listas de ação não desenhadas");
    }
    fill
}

pub fn draw_action_list(item: &Item, view: &HudView, list: &mut DrawList) -> bool {
    let name = item.control.head.name.as_str();
    let Some((_, category)) = ACTION_LISTS.iter().find(|(control, _)| *control == name) else {
        return false;
    };
    let Some(props) = item_window_props(item) else { return false };
    let grid = grid_of(props);
    let icons = action_icons(view, *category);
    let start = list.quads.len();
    for (index, icon) in icons.iter().enumerate().take(grid.visible() as usize) {
        let cell = grid.cell(item.rect, index as i32);
        blit_icon(list, cell, icon, &props.frame);
    }
    list.clip_from(start, item.rect);
    true
}

pub fn active_skills(view: &HudView) -> Vec<(i32, i32)> {
    view.world.skills.iter().filter(|skill| !skill.passive).map(|skill| (skill.id, skill.level)).collect()
}

pub fn passive_skills(view: &HudView) -> Vec<(i32, i32)> {
    view.world.skills.iter().filter(|skill| skill.passive).map(|skill| (skill.id, skill.level)).collect()
}

pub fn bind_magic_skill(view: &HudView) -> WindowFill {
    let mut fill = WindowFill::default();
    for index in 0..SKILL_GROUPS_ACTIVE {
        fill.hide_one(format!("ASkillName{index}"));
        fill.hide_one(format!("ASkill{index}"));
    }
    for index in 0..SKILL_GROUPS_PASSIVE {
        fill.hide_one(format!("PSkillName{index}"));
        fill.hide_one(format!("PSkill{index}"));
    }
    if !active_skills(view).is_empty() || !passive_skills(view).is_empty() {
        fill.report(
            "MagicSkillWnd: agrupamento por IconType do skillgrp (0-7 ativas, 11-16 passivas) indisponível; record_high_five descarta essa coluna, então nenhum grupo é desenhado",
        );
    }
    fill
}


fn pair(value: Option<(i32, i32)>) -> String {
    match value {
        Some((current, max)) => format!("{current}/{max}"),
        None => String::new(),
    }
}

pub fn bind_detail_status(view: &HudView) -> Option<WindowFill> {
    let me = view.world.me.as_ref()?;
    let stats = &view.world.stats;
    let social = &view.world.social;
    let mut fill = WindowFill::default();

    fill.value("txtName1", me.name.clone());
    fill.value("txtName2", me.title.clone());
    fill.value("txtLvName", me.level.to_string());
    fill.report("DetailStatusWnd: nome da classe (txtLvName) sem tabela de classes no projeto; só o nível é mostrado");

    fill.value("txtSTR", stats.str_.to_string());
    fill.value("txtDEX", stats.dex.to_string());
    fill.value("txtCON", stats.con.to_string());
    fill.value("txtINT", stats.int_.to_string());
    fill.value("txtWIT", stats.wit.to_string());
    fill.value("txtMEN", stats.men.to_string());

    fill.value("txtPhysicalAttack", stats.p_atk.to_string());
    fill.value("txtPhysicalDefense", stats.p_def.to_string());
    fill.value("txtPhysicalAttackSpeed", stats.atk_spd.to_string());
    fill.value("txtPhysicalAvoid", stats.evasion.to_string());
    fill.value("txtHitRate", stats.accuracy.to_string());
    fill.value("txtCriticalRate", stats.critical.to_string());
    fill.value("txtMagicalAttack", stats.m_atk.to_string());
    fill.value("txtMagicDefense", stats.m_def.to_string());
    fill.value("txtMagicCastingSpeed", stats.cast_spd.to_string());
    fill.value("txtMovingSpeed", me.run_speed.to_string());
    fill.value("txtSP", stats.sp.to_string());

    fill.value("txtPVP", social.pvp_kills.to_string());
    fill.value("txtSociality", format!("{} / {}", social.recom_have, social.recom_left));
    fill.value("txtCriminalRate", me.karma.to_string());

    match view.world.pledge_name(social.clan_id) {
        Some(name) => fill.value("txtPledge", name.to_string()),
        None => fill.hide(&["txtPledge"]),
    }

    fill.value("texHP", pair(me.hp));
    fill.value("texMP", pair(me.mp));
    fill.value("texCP", pair(me.cp));

    match view.world.user_extras {
        Some(extras) => fill.value("texExp", format!("{:.2}%", extras.exp_percent)),
        None => fill.hide(&["texExp"]),
    }

    fill.hide(&[
        "txtAttrAttackType",
        "txtAttrAttackValue",
        "txtAttrDefenseValFire",
        "txtAttrDefenseValWater",
        "txtAttrDefenseValWind",
        "txtAttrDefenseValEarth",
        "txtAttrDefenseValHoly",
        "txtAttrDefenseValUnholy",
    ]);
    fill.report(
        "DetailStatusWnd: atributos elementais chegam no pacote H5 (UserInfoExtrasHighFive.elements) mas UserExtras do killer-world só guarda exp_percent e vitality; campos escondidos",
    );

    fill.hide(&["txtRank", "txtHeadRank", "txtRemainSulffrage", "txtGmMoving", "texPledgeCrest", "texHero", "LifeForceTex"]);
    fill.report(
        "DetailStatusWnd: rank de olimpíada, sufrágio restante, brasão de clã e marca de herói sem dado no estado de mundo; campos escondidos",
    );

    Some(fill)
}

#[cfg(test)]
mod tests {
    use super::*;
    use l2_dat::xdat::ItemGroup;

    fn props(head: [i32; 11]) -> ItemWindowProps {
        ItemWindowProps {
            scroll_type: String::new(),
            head,
            groups: [
                ItemGroup { texture: String::new(), values: [0, 0] },
                ItemGroup { texture: String::new(), values: [0, 0] },
                ItemGroup { texture: String::new(), values: [0, 0] },
            ],
            slot: String::new(),
            values: [0; 6],
            frame: String::new(),
            frame_down: String::new(),
            fill: [0; 4],
            extras: Vec::new(),
        }
    }

    #[test]
    fn grid_comes_from_the_item_window_head_and_not_from_hardcoded_numbers() {
        let grid = grid_of(&props([6, 8, 250, 32, 32, 4, 4, 0, 0, 0, 0]));
        assert_eq!((grid.cols, grid.rows), (6, 8));
        assert_eq!((grid.icon_w, grid.icon_h), (32, 32));
        assert_eq!((grid.gap_x, grid.gap_y), (4, 4));
        assert_eq!(grid.visible(), 48);
    }

    #[test]
    fn cells_advance_by_icon_plus_gap_and_wrap_at_the_column_count() {
        let grid = grid_of(&props([6, 8, 250, 32, 32, 4, 4, 0, 0, 0, 0]));
        let origin = Rect::new(210, 58, 231, 288);
        assert_eq!(grid.cell(origin, 0), Rect::new(210, 58, 32, 32));
        assert_eq!(grid.cell(origin, 5), Rect::new(210 + 5 * 36, 58, 32, 32));
        assert_eq!(grid.cell(origin, 6), Rect::new(210, 58 + 36, 32, 32));
    }

    #[test]
    fn the_three_action_lists_map_to_the_real_actionname_categories() {
        assert_eq!(ACTION_LISTS[0], ("ActionBasicItem", CATEGORY_BASIC));
        assert_eq!(ACTION_LISTS[1], ("ActionPartyItem", CATEGORY_PARTY));
        assert_eq!(ACTION_LISTS[2], ("ActionSocialItem", CATEGORY_SOCIAL));
    }
}
