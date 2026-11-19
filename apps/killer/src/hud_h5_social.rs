use crate::hud_h5::HudView;
use crate::hud_h5_windows::WindowFill;
use killer_ui::color::{Argb, TEXT};
use killer_ui::draw::DrawList;
use killer_ui::window::Rect;
use killer_ui::xdat_window::{self, Layout, TextAlign, TextVAlign};

pub const MENU: &str = "MenuWnd";
pub const CLAN: &str = "ClanWnd";
pub const QUEST_TREE: &str = "QuestTreeWnd";
pub const PARTY_MATCH: &str = "PartyMatchWnd";
pub const RADAR_MAP: &str = "RadarMapWnd";

pub const WINDOWS: [&str; 5] = [MENU, CLAN, QUEST_TREE, PARTY_MATCH, RADAR_MAP];

pub const QUEST_TABS: usize = 5;
pub const PARTY_MATCH_PAGE: usize = 10;

pub const ONLINE_ICON: &str = "L2UI_CH3.BloodHoodWnd.BloodHood_Logon";
pub const OFFLINE_ICON: &str = "L2UI_CH3.BloodHoodWnd.BloodHood_Logoff";

const OFFLINE_COLOR: Argb = Argb(0xFF808080);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SocialClick {
    Toggle(&'static str),
    QuestTab(usize),
    PartyMatchPage(i32),
    Handled,
}

pub fn is_social(name: &str) -> bool {
    WINDOWS.contains(&name)
}

pub fn bind(name: &str, view: &HudView, tab: usize) -> Option<WindowFill> {
    match name {
        MENU => Some(WindowFill::default()),
        CLAN => Some(bind_clan(view)),
        QUEST_TREE => Some(bind_quest_tree(view, tab)),
        PARTY_MATCH => Some(bind_party_match(view)),
        RADAR_MAP => Some(bind_radar_map()),
        _ => None,
    }
}

pub fn draw(name: &str, layout: &Layout, view: &HudView, tab: usize, page: usize, list: &mut DrawList) {
    match name {
        CLAN => draw_clan_members(layout, view, list),
        QUEST_TREE => draw_quest_list(layout, view, tab, list),
        PARTY_MATCH => draw_party_rooms(layout, view, page, list),
        _ => {}
    }
}

pub fn click(window: &str, button: &str) -> Option<SocialClick> {
    match (window, button) {
        (MENU, "BtnCharInfo") => Some(SocialClick::Toggle("DetailStatusWnd")),
        (MENU, "BtnInventory") => Some(SocialClick::Toggle("InventoryWnd")),
        (MENU, "BtnMap") => Some(SocialClick::Toggle("MinimapWnd")),
        (MENU, "BtnSystemMenu") => Some(SocialClick::Toggle("SystemMenuWnd")),
        (MENU, "BtnSkill") => Some(SocialClick::Toggle("MagicSkillWnd")),
        (MENU, "BtnAction") => Some(SocialClick::Toggle("ActionWnd")),
        (MENU, "BtnClan") => Some(SocialClick::Toggle(CLAN)),
        (MENU, "BtnQuest") => Some(SocialClick::Toggle(QUEST_TREE)),
        (MENU, "RotateButton1" | "RotateButton2") => Some(SocialClick::Handled),
        (QUEST_TREE, "QuestTreeTab0") => Some(SocialClick::QuestTab(0)),
        (QUEST_TREE, "QuestTreeTab1") => Some(SocialClick::QuestTab(1)),
        (QUEST_TREE, "QuestTreeTab2") => Some(SocialClick::QuestTab(2)),
        (QUEST_TREE, "QuestTreeTab3") => Some(SocialClick::QuestTab(3)),
        (QUEST_TREE, "QuestTreeTab4") => Some(SocialClick::QuestTab(4)),
        (QUEST_TREE, "btnClose") => Some(SocialClick::Toggle("QuestTreeDrawerWnd")),
        (PARTY_MATCH, "PrevBtn" | "UnionPrevBtn") => Some(SocialClick::PartyMatchPage(-1)),
        (PARTY_MATCH, "NextBtn" | "UnionNextBtn") => Some(SocialClick::PartyMatchPage(1)),
        _ => None,
    }
}

fn bind_clan(view: &HudView) -> WindowFill {
    let mut bind = WindowFill::default();
    let pledge = &view.world.pledge_window;
    match pledge.info.as_ref() {
        Some(info) => {
            bind.values.push(("ClanNameText".into(), info.name.clone()));
            bind.values.push(("ClanMasterNameText".into(), info.leader_name.clone()));
            bind.values.push(("ClanLevelText".into(), info.level.to_string()));
            bind.values.push(("ClanCurrentNum".into(), pledge.members.len().to_string()));
        }
        None => {
            if let Some(summary) = pledge.summary.as_ref() {
                bind.values.push(("ClanNameText".into(), summary.name.clone()));
            }
            bind.reports.push(
                "ClanWnd: lista completa do clã ainda não recebida (PledgeShowMemberListAll); só o nome do PledgeInfo é mostrado".into(),
            );
        }
    }
    bind.hidden.push("ClanAgitText".into());
    bind.hidden.push("ClanStatusText".into());
    bind.reports.push("ClanWnd: sede/castelo (ClanAgitText) e status (ClanStatusText) sem dado lido em l2net".into());
    for name in [
        "ClanDrawerWnd",
        "ClanAskJoinBtn",
        "ClanAuthEditBtn",
        "ClanBoardBtn",
        "ClanMemAuthBtn",
        "ClanPenaltyBtn",
        "ClanQuitBtn",
        "ClanTitleManageBtn",
        "ClanWarCancleBtn",
        "ClanWarDeclareBtn",
        "ClanWarInfoBtn",
        "HeroBtn",
    ] {
        bind.hidden.push(name.into());
    }
    bind.reports.push("ClanWnd: botões de gestão (convidar/expulsar/guerra/brasão/hero) sem comando portado em l2net".into());
    bind
}

fn bind_quest_tree(view: &HudView, tab: usize) -> WindowFill {
    let mut bind = WindowFill::default();
    let tab = tab.min(QUEST_TABS - 1);
    let total = quest_rows(view, tab).len();
    bind.values.push(("txtQuestNum".into(), total.to_string()));
    for index in 0..QUEST_TABS {
        if index != tab {
            bind.hidden.push(quest_tree_control(index));
        }
    }
    bind.hidden.push("QuestTreeDrawerWnd".into());
    bind.reports.push(
        "QuestTreeWnd: detalhe da quest (QuestTreeDrawerWnd: descrição, itens, recompensa) não portado - o texto vem de UIDATA_QUEST nativo e a árvore XML (TreeCtrl) não tem desenho genérico".into(),
    );
    bind
}

fn bind_party_match(view: &HudView) -> WindowFill {
    let mut bind = WindowFill::default();
    if party_rooms(view).is_empty() {
        bind.reports.push("PartyMatchWnd: nenhuma sala recebida do servidor (PartyMatchList) - lista vazia".into());
    }
    for name in ["MakeRoomBtn", "AutoJoinBtn", "WaitListButton", "UnionRefreshBtn", "RefreshBtn", "UnionMatchListCtrl"] {
        bind.hidden.push(name.into());
    }
    bind.reports.push("PartyMatchWnd: criar sala / entrar automático / lista de espera sem comando portado em l2net".into());
    bind.reports.push("PartyMatchWnd: lista de união (UnionMatchListCtrl) sem pacote lido em l2net".into());
    bind
}

fn bind_radar_map() -> WindowFill {
    let mut bind = WindowFill::default();
    for name in ["RadarMapRotationWnd", "FlightStatusGauges", "FlightStatusAltitude", "RadarOptionWnd"] {
        bind.hidden.push(name.into());
    }
    bind.reports.push(
        "RadarMapWnd: os pontos do radar são nativos (RadarMapCtrlHandle.AddObject em NWindow.dll, sem fórmula em UnrealScript), igual ao MinimapWnd - só a moldura e as opções são desenhadas".into(),
    );
    bind.reports.push("RadarMapWnd: painéis de voo (combustível/altitude) sem dado de veículo portado".into());
    bind
}

fn quest_tree_control(tab: usize) -> String {
    format!("MainTree{}", tab.min(QUEST_TABS - 1) + 1)
}

fn quest_rows(view: &HudView, tab: usize) -> Vec<(String, bool)> {
    let Some(tables) = view.tables else { return Vec::new() };
    let mut rows = Vec::new();
    for quest in &view.world.windows.quests {
        let Some(row) = tables.questname.get(&(quest.quest_id, quest.cond)) else { continue };
        if row.quest_type != tab as i32 {
            continue;
        }
        rows.push((row.title.clone(), row.journal_disp != 0));
    }
    rows
}

fn party_rooms(view: &HudView) -> Vec<String> {
    let Some(list) = view.world.windows.party_rooms.as_ref() else { return Vec::new() };
    list.rooms
        .iter()
        .map(|room| format!("{}  {}  {}-{}  {}/{}", room.title, room.owner_name, room.min_level, room.max_level, room.members, room.max_members))
        .collect()
}

fn row_height(view: &HudView) -> i32 {
    (view.font.line_height as i32 + 2).max(1)
}

fn draw_rows(rect: Rect, rows: &[(String, Argb)], view: &HudView, list: &mut DrawList) {
    let height = row_height(view);
    if rect.h <= 0 || rect.w <= 0 {
        return;
    }
    let visible = (rect.h / height).max(0) as usize;
    let start = list.quads.len();
    for (index, (text, color)) in rows.iter().take(visible).enumerate() {
        let y = rect.y + index as i32 * height;
        xdat_window::draw_aligned(
            list,
            Rect::new(rect.x, y, rect.w, height),
            text,
            *color,
            TextAlign::Left,
            TextVAlign::Middle,
            view.font,
        );
    }
    list.clip_from(start, rect);
}

fn draw_clan_members(layout: &Layout, view: &HudView, list: &mut DrawList) {
    let Some(item) = layout.find("ClanMemberList") else { return };
    let height = row_height(view);
    if item.rect.h <= 0 || item.rect.w <= height {
        return;
    }
    let visible = (item.rect.h / height).max(0) as usize;
    let start = list.quads.len();
    for (index, member) in view.world.pledge_window.members.iter().take(visible).enumerate() {
        let y = item.rect.y + index as i32 * height;
        let online = member.online_object_id > 0;
        let icon = if online { ONLINE_ICON } else { OFFLINE_ICON };
        list.draw_texture(item.rect.x, y, height, height, 0, 0, height, height, icon, 1.0);
        let text = format!("{}  {}", member.name, member.level);
        let color = if online { TEXT } else { OFFLINE_COLOR };
        xdat_window::draw_aligned(
            list,
            Rect::new(item.rect.x + height, y, item.rect.w - height, height),
            &text,
            color,
            TextAlign::Left,
            TextVAlign::Middle,
            view.font,
        );
    }
    list.clip_from(start, item.rect);
}

fn draw_quest_list(layout: &Layout, view: &HudView, tab: usize, list: &mut DrawList) {
    let name = quest_tree_control(tab);
    let Some(item) = layout.find(&name) else { return };
    let rows: Vec<(String, Argb)> = quest_rows(view, tab.min(QUEST_TABS - 1))
        .into_iter()
        .map(|(title, journal)| (title, if journal { TEXT } else { OFFLINE_COLOR }))
        .collect();
    draw_rows(item.rect, &rows, view, list);
}

fn draw_party_rooms(layout: &Layout, view: &HudView, page: usize, list: &mut DrawList) {
    let Some(item) = layout.find("PartyMatchListCtrl") else { return };
    let all = party_rooms(view);
    let start = page * PARTY_MATCH_PAGE;
    let rows: Vec<(String, Argb)> = all.into_iter().skip(start).take(PARTY_MATCH_PAGE).map(|text| (text, TEXT)).collect();
    draw_rows(item.rect, &rows, view, list);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_buttons_follow_the_real_menuwnd_script_targets() {
        assert_eq!(click(MENU, "BtnCharInfo"), Some(SocialClick::Toggle("DetailStatusWnd")));
        assert_eq!(click(MENU, "BtnSkill"), Some(SocialClick::Toggle("MagicSkillWnd")));
        assert_eq!(click(MENU, "BtnAction"), Some(SocialClick::Toggle("ActionWnd")));
        assert_eq!(click(MENU, "BtnQuest"), Some(SocialClick::Toggle(QUEST_TREE)));
        assert_eq!(click(MENU, "BtnClan"), Some(SocialClick::Toggle(CLAN)));
        assert_eq!(click(MENU, "BtnInventory"), Some(SocialClick::Toggle("InventoryWnd")));
        assert_eq!(click(MENU, "BtnMap"), Some(SocialClick::Toggle("MinimapWnd")));
        assert_eq!(click(MENU, "BtnSystemMenu"), Some(SocialClick::Toggle("SystemMenuWnd")));
        assert_eq!(click(MENU, "BtnUnknown"), None);
    }

    #[test]
    fn quest_tab_zero_maps_to_the_one_based_maintree1_control_of_the_real_script() {
        assert_eq!(quest_tree_control(0), "MainTree1");
        assert_eq!(quest_tree_control(4), "MainTree5");
        assert_eq!(quest_tree_control(9), "MainTree5");
    }

    #[test]
    fn quest_tabs_select_one_tree_and_hide_the_other_four() {
        assert_eq!(click(QUEST_TREE, "QuestTreeTab3"), Some(SocialClick::QuestTab(3)));
        for tab in 0..QUEST_TABS {
            let hidden: Vec<String> = (0..QUEST_TABS).filter(|index| *index != tab).map(quest_tree_control).collect();
            assert_eq!(hidden.len(), QUEST_TABS - 1);
            assert!(!hidden.contains(&quest_tree_control(tab)));
        }
    }

    #[test]
    fn party_match_pages_step_in_both_directions() {
        assert_eq!(click(PARTY_MATCH, "NextBtn"), Some(SocialClick::PartyMatchPage(1)));
        assert_eq!(click(PARTY_MATCH, "PrevBtn"), Some(SocialClick::PartyMatchPage(-1)));
        assert_eq!(click(PARTY_MATCH, "UnionNextBtn"), Some(SocialClick::PartyMatchPage(1)));
    }

    #[test]
    fn every_social_window_is_recognised() {
        for name in WINDOWS {
            assert!(is_social(name), "{name} deveria ser social");
        }
        assert!(!is_social("InventoryWnd"));
    }
}
