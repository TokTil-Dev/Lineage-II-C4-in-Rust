use super::{
    CustomTooltip, DrawItemInfo, DrawItemType, InterfaceSound, NameCtrlType, TextAlign, TreeNodeInfo, TreeNodeItemInfo,
    UiApi, WindowScript, parse_int, rgb,
};

pub const WINDOW: &str = "TargetStatusWnd";
pub const EVENTS: [i32; 11] = [980, 990, 190, 210, 200, 220, 300, 290, 50, 110, 2960];
pub const CHAT_EDIT_BOX: &str = "ChatWnd.ChatEditBox";
pub const BUMPER: &str = "TargetStatusWnd.NPHRN_Bumper";
pub const USER_NAME: &str = "TargetStatusWnd.UserName";
pub const RANK_NAME: &str = "TargetStatusWnd.RankName";
pub const BAR_HP: &str = "TargetStatusWnd.barHP";
pub const BAR_MP: &str = "TargetStatusWnd.barMP";
pub const NPC_INFO: &str = "TargetStatusWnd.NpcInfo";
pub const BTN_EXPAND: &str = "TargetStatusWnd.BtnExpand";
pub const PLEDGE_SYSSTRING: i32 = 431;
pub const ALLIANCE_SYSSTRING: i32 = 591;
pub const OPTION_SECTION: &str = "Neophron";
pub const TIMER_STATUS_USER: i32 = 1;
pub const TIMER_STATUS_USER_MS: i32 = 500;
pub const AGGRO_SKILLS: [i32; 3] = [18, 28, 286];
pub const ALL_WHITE_IDS: [i32; 7] = [12778, 13031, 13032, 13033, 13034, 13035, 13036];
pub const NO_BAR_IDS: [i32; 1] = [13036];
pub const EXPAND_TEXTURES: [&str; 3] = ["Interface.Shortcut_Expand", "Interface.Shortcut_Expand_Down", "Interface.Shortcut_Expand_Over"];
pub const MINIMIZE_TEXTURES: [&str; 3] =
    ["Interface.Shortcut_Minimize", "Interface.Shortcut_Minimize_Down", "Interface.Shortcut_Minimize_Over"];

#[derive(Debug, Default)]
pub struct Script {
    pub expand: bool,
    pub level_diff: i32,
    pub target_id: i32,
    pub status_user_id: i32,
    pub hold_target_id: i32,
    pub aggr_id: i32,
    pub dlg_text: String,
    pub show: bool,
}

impl Script {
    pub fn on_timer(&mut self, api: &mut dyn UiApi, timer_id: i32) {
        if timer_id == TIMER_STATUS_USER {
            api.kill_timer(WINDOW, TIMER_STATUS_USER);
            self.status_user_id = api.script_get_int("StatusWnd", "m_UserID");
        }
    }

    pub fn on_close_button(&mut self, api: &mut dyn UiApi) {
        api.request_target_cancel();
        api.play_console_sound(InterfaceSound::WindowClose);
    }

    pub fn handle_update_hpmp(&mut self, api: &mut dyn UiApi, param: &str) {
        if self.show {
            let server_id = parse_int(param, "ServerID").unwrap_or(0);
            if self.target_id == server_id {
                self.handle_target_update(api);
            }
        }
    }

    pub fn handle_receive_target_level_diff(&mut self, param: &str) {
        if let Some(diff) = parse_int(param, "LevelDiff") {
            self.level_diff = diff;
        }
    }

    pub fn handle_target_update(&mut self, api: &mut dyn UiApi) {
        let mut target = api.target_id();
        if api.get_option_bool(OPTION_SECTION, "IgnoreAggr") && target == self.aggr_id {
            api.request_target_cancel();
            self.aggr_id = -1;
            target = -1;
        }
        if target == self.status_user_id {
            api.hide_window(BUMPER);
        }
        if target < 1 {
            api.hide_window(WINDOW);
            return;
        }
        let mut is_target_changed = false;
        if self.target_id != target {
            is_target_changed = true;
            self.hold_target_id = target;
        }
        self.target_id = target;
        let info = api.get_target_info().unwrap_or_default();
        let rect = api.get_rect(WINDOW);
        let mut pledge_name = api.get_system_string(PLEDGE_SYSSTRING);
        let mut pledge_alliance_name = api.get_system_string(ALLIANCE_SYSSTRING);
        let mut pledge_name_color = rgb(128, 128, 128);
        let mut pledge_alliance_name_color = rgb(128, 128, 128);
        let target_name_color = api.target_name_color(self.level_diff);
        let mut name = String::new();
        let mut rank_name = String::new();
        let (mut show_hp_bar, mut show_mp_bar, mut show_pledge_info) = (false, false, false);
        let (mut show_pledge_tex, mut show_pledge_alliance_tex, mut show_npc_info) = (false, false, false);
        if api.target_is_server_object() {
            api.hide_window(BUMPER);
            let name_id = api.static_object_name_id(self.target_id);
            if name_id > 0 {
                name = api.static_object_name(name_id);
                rank_name = String::new();
            }
            api.set_name(USER_NAME, &name, NameCtrlType::Normal, TextAlign::Center);
            api.set_name(RANK_NAME, &rank_name, NameCtrlType::Normal, TextAlign::Center);
            if api.server_object_type(self.target_id) == 2 && api.static_object_show_hp(self.target_id) {
                show_hp_bar = true;
                let (hp, max_hp) = (api.server_object_hp(self.target_id), api.server_object_max_hp(self.target_id));
                self.update_hp_bar(api, hp, max_hp);
            }
        } else if info.name.chars().count() < 1 {
            name = api.party_member_name(self.target_id);
            rank_name = String::new();
            api.debug(&format!("m_TargetID{}, info.Name : {}, Name : {name}", self.target_id, info.name));
            api.set_name(USER_NAME, &name, NameCtrlType::Normal, TextAlign::Center);
            api.set_name(RANK_NAME, &rank_name, NameCtrlType::Normal, TextAlign::Center);
        } else {
            let player_id = api.player_id();
            let pet_id = api.pet_id();
            let hp_showable_npc = api.target_is_hp_showable_npc();
            if hp_showable_npc {
                api.hide_window(BUMPER);
            }
            if (info.npc && !info.pet && info.can_be_attacked)
                || (player_id > 0 && self.target_id == player_id)
                || (info.npc && info.pet && self.target_id == pet_id)
                || (info.npc && hp_showable_npc)
            {
                if Self::is_all_white_id(info.class_id) {
                    name = info.name.clone();
                    rank_name = String::new();
                    api.set_name(USER_NAME, &name, NameCtrlType::Normal, TextAlign::Center);
                    api.set_name(RANK_NAME, &rank_name, NameCtrlType::Normal, TextAlign::Center);
                    if !Self::is_no_bar_id(info.class_id) {
                        show_hp_bar = true;
                        self.update_hp_bar(api, info.cur_hp, info.max_hp);
                    }
                } else {
                    name = info.name.clone();
                    rank_name = String::new();
                    api.set_name_with_color(USER_NAME, &name, NameCtrlType::Normal, TextAlign::Center, target_name_color);
                    api.set_name(RANK_NAME, &rank_name, NameCtrlType::Normal, TextAlign::Center);
                    show_hp_bar = true;
                    self.update_hp_bar(api, info.cur_hp, info.max_hp);
                    if !(info.npc && !info.pet && info.can_be_attacked) {
                        show_mp_bar = true;
                        self.update_mp_bar(api, info.cur_mp, info.max_mp);
                    }
                }
            } else {
                name = info.name.clone();
                if info.npc {
                    rank_name = String::new();
                    api.hide_window(BUMPER);
                } else {
                    rank_name = api.get_user_rank_string(info.user_rank);
                    api.show_window(BUMPER);
                }
                api.set_name(USER_NAME, &name, NameCtrlType::Normal, TextAlign::Center);
                api.set_name(RANK_NAME, &rank_name, NameCtrlType::Normal, TextAlign::Center);
            }
            if self.expand {
                if info.npc {
                    if let Some(npc_info) = api.npc_property(info.class_id) {
                        show_npc_info = true;
                        if is_target_changed {
                            self.update_npc_info_tree(api, &npc_info);
                        }
                    }
                } else {
                    show_pledge_info = true;
                    if info.clan_id > 0 {
                        pledge_name = api.clan_name(info.clan_id);
                        pledge_name_color = rgb(176, 152, 121);
                        if !pledge_name.is_empty()
                            && let Some(clan_type) = api.clan_type(self.target_id)
                            && let Some(clan_name_value) = api.clan_name_value(info.clan_id)
                        {
                            if clan_type == -1 {
                                pledge_name_color = rgb(209, 167, 2);
                            } else if clan_name_value > 0 {
                                pledge_name_color = rgb(0, 130, 255);
                            } else if clan_name_value < 0 {
                                pledge_name_color = rgb(255, 0, 0);
                            }
                        }
                        match api.clan_crest_texture(info.clan_id) {
                            Some(texture) => {
                                show_pledge_tex = true;
                                api.set_texture_with_object("TargetStatusWnd.texPledgeCrest", &texture);
                            }
                            None => show_pledge_tex = false,
                        }
                        let alliance = api.clan_alliance_name(info.clan_id);
                        if !alliance.is_empty() {
                            pledge_alliance_name = alliance;
                            pledge_alliance_name_color = rgb(176, 155, 121);
                            match api.clan_alliance_crest_texture(info.clan_id) {
                                Some(texture) => {
                                    show_pledge_alliance_tex = true;
                                    api.set_texture_with_object("TargetStatusWnd.texPledgeAllianceCrest", &texture);
                                }
                                None => show_pledge_alliance_tex = false,
                            }
                        }
                    }
                }
            }
        }
        if !api.is_show_window(WINDOW) {
            api.show_window(WINDOW);
            let expand = self.expand;
            self.set_expand_mode(api, expand);
        }
        if show_hp_bar {
            api.show_window(BAR_HP);
        } else {
            api.hide_window(BAR_HP);
        }
        if show_mp_bar {
            api.show_window(BAR_MP);
        } else {
            api.hide_window(BAR_MP);
        }
        if show_pledge_info {
            api.show_window("TargetStatusWnd.txtPledge");
            api.show_window("TargetStatusWnd.txtAlliance");
            api.show_window("TargetStatusWnd.txtPledgeName");
            api.show_window("TargetStatusWnd.txtPledgeAllianceName");
            api.set_text("TargetStatusWnd.txtPledgeName", &pledge_name);
            api.set_text("TargetStatusWnd.txtPledgeAllianceName", &pledge_alliance_name);
            api.set_text_color("TargetStatusWnd.txtPledgeName", pledge_name_color);
            api.set_text_color("TargetStatusWnd.txtPledgeAllianceName", pledge_alliance_name_color);
            if show_pledge_tex {
                api.show_window("TargetStatusWnd.texPledgeCrest");
                api.move_to("TargetStatusWnd.txtPledgeName", rect.x + 63, rect.y + 43);
            } else {
                api.hide_window("TargetStatusWnd.texPledgeCrest");
                api.move_to("TargetStatusWnd.txtPledgeName", rect.x + 45, rect.y + 43);
            }
            if show_pledge_alliance_tex {
                api.show_window("TargetStatusWnd.texPledgeAllianceCrest");
                api.move_to("TargetStatusWnd.txtPledgeAllianceName", rect.x + 63, rect.y + 59);
            } else {
                api.hide_window("TargetStatusWnd.texPledgeAllianceCrest");
                api.move_to("TargetStatusWnd.txtPledgeAllianceName", rect.x + 45, rect.y + 59);
            }
        } else {
            api.hide_window("TargetStatusWnd.txtPledge");
            api.hide_window("TargetStatusWnd.texPledgeCrest");
            api.hide_window("TargetStatusWnd.txtPledgeName");
            api.hide_window("TargetStatusWnd.txtAlliance");
            api.hide_window("TargetStatusWnd.texPledgeAllianceCrest");
            api.hide_window("TargetStatusWnd.txtPledgeAllianceName");
        }
        if show_npc_info {
            api.show_window(NPC_INFO);
            api.tree_show_scroll_bar(NPC_INFO, false);
        } else {
            api.hide_window(NPC_INFO);
        }
    }

    pub fn hold_target(&mut self, api: &mut dyn UiApi) {
        if api.get_option_bool(OPTION_SECTION, "HoldTarget") && self.hold_target_id != self.aggr_id {
            api.request_target_user(self.hold_target_id);
        }
    }

    pub fn ignore_aggr(&mut self, api: &mut dyn UiApi, param: &str) {
        let skill_id = parse_int(param, "SkillID").unwrap_or(0);
        if Self::is_aggro_skill(skill_id) {
            let attacker_id = parse_int(param, "AttackerID").unwrap_or(0);
            if !self.is_party_member(api, attacker_id) {
                self.aggr_id = attacker_id;
            }
        }
    }

    pub fn is_party_member(&self, api: &dyn UiApi, target_id: i32) -> bool {
        let mut flag = false;
        for index in 0..8 {
            if target_id == api.script_get_int_array("PartyWnd", "m_arrID", index) {
                flag = true;
            }
        }
        flag
    }

    pub fn is_aggro_skill(skill_id: i32) -> bool {
        AGGRO_SKILLS.contains(&skill_id)
    }

    pub fn set_expand_mode(&mut self, api: &mut dyn UiApi, expand: bool) {
        self.expand = expand;
        self.target_id = -1;
        self.handle_target_update(api);
        if expand {
            api.hide_window("TargetStatusWnd.BackTex");
            api.show_window("TargetStatusWnd.BackExpTex");
            api.set_button_texture(BTN_EXPAND, EXPAND_TEXTURES[0], EXPAND_TEXTURES[1], EXPAND_TEXTURES[2]);
        } else {
            api.show_window("TargetStatusWnd.BackTex");
            api.hide_window("TargetStatusWnd.BackExpTex");
            api.set_button_texture(BTN_EXPAND, MINIMIZE_TEXTURES[0], MINIMIZE_TEXTURES[1], MINIMIZE_TEXTURES[2]);
        }
    }

    pub fn update_hp_bar(&self, api: &mut dyn UiApi, hp: i32, max_hp: i32) {
        api.bar_set_value(BAR_HP, max_hp, hp);
    }

    pub fn update_mp_bar(&self, api: &mut dyn UiApi, mp: i32, max_mp: i32) {
        api.bar_set_value(BAR_MP, max_mp, mp);
    }

    pub fn update_npc_info_tree(&self, api: &mut dyn UiApi, npc_info: &[i32]) {
        api.tree_clear(NPC_INFO);
        let root = TreeNodeInfo { name: "root".into(), ..Default::default() };
        let node_name = api.tree_insert_node(NPC_INFO, "", &root);
        if node_name.chars().count() < 1 {
            api.debug(&format!("ERROR: Can't insert root node. Name: {}", root.name));
            return;
        }
        let mut i = 0usize;
        while i < npc_info.len() {
            let skill_id = npc_info[i];
            let skill_level = npc_info.get(i + 1).copied().unwrap_or(0);
            let column = (i as i32 / 2) % 8;
            let mut node = TreeNodeInfo { offset_x: column * 18, ..Default::default() };
            node.offset_y = if column == 0 {
                if i > 0 { 3 } else { 0 }
            } else {
                -15
            };
            node.name = format!("{}", i / 2);
            node.show_button = 0;
            node.tooltip = Self::set_npc_info_tooltip(api, skill_id, skill_level);
            let node_name = api.tree_insert_node(NPC_INFO, "root", &node);
            if node_name.chars().count() < 1 {
                api.log(&format!("ERROR: Can't insert node. Name: {}", node.name));
                return;
            }
            node.tooltip.draw_list.clear();
            let item = TreeNodeItemInfo {
                kind: 2,
                texture_width: 15,
                texture_height: 15,
                texture_u_width: 32,
                texture_u_height: 32,
                texture: api.skill_icon_name(skill_id, skill_level),
                ..Default::default()
            };
            api.tree_insert_node_item(NPC_INFO, &node_name, &item);
            i += 2;
        }
    }

    pub fn set_npc_info_tooltip(api: &dyn UiApi, id: i32, level: i32) -> CustomTooltip {
        let name = api.skill_name(id, level);
        let description = api.skill_description(id, level);
        let mut tooltip = CustomTooltip {
            draw_list: vec![DrawItemInfo { kind: DrawItemType::Text, draw_one_line: true, text: name, ..Default::default() }],
            ..Default::default()
        };
        if !description.is_empty() {
            tooltip.minimum_width = 144;
            tooltip.draw_list.push(DrawItemInfo {
                kind: DrawItemType::Text,
                offset_y: 6,
                line_break: true,
                color: Some(rgb(178, 190, 207)),
                text: description,
                ..Default::default()
            });
        }
        tooltip
    }

    pub fn is_all_white_id(class_id: i32) -> bool {
        ALL_WHITE_IDS.contains(&class_id)
    }

    pub fn is_no_bar_id(class_id: i32) -> bool {
        NO_BAR_IDS.contains(&class_id)
    }

    pub fn on_r_button_up(&mut self, api: &mut dyn UiApi, x: i32, _y: i32) {
        self.dlg_text = api.dialog_get_string();
        let rect = api.get_rect(WINDOW);
        if x > rect.x {
            let info = api.get_target_info().unwrap_or_default();
            api.set_chat_message(&format!(" {} ", info.name));
            api.release_focus(CHAT_EDIT_BOX);
        }
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        for event in EVENTS {
            api.register_event(event);
        }
        self.set_expand_mode(api, false);
        self.show = false;
        self.target_id = -1;
        api.hide_window(BUMPER);
    }

    fn on_enter_state(&mut self, api: &mut dyn UiApi, _previous: &str) {
        let expand = self.expand;
        self.set_expand_mode(api, expand);
        api.set_timer(WINDOW, TIMER_STATUS_USER, TIMER_STATUS_USER_MS);
    }

    fn on_timer(&mut self, api: &mut dyn UiApi, timer_id: i32) {
        Script::on_timer(self, api, timer_id);
    }

    fn on_show(&mut self, _api: &mut dyn UiApi) {
        self.show = true;
    }

    fn on_hide(&mut self, _api: &mut dyn UiApi) {
        self.show = false;
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        match event_id {
            980 => {
                self.handle_target_update(api);
                if !api.is_show_window(WINDOW) {
                    api.execute_event(2960, "");
                }
            }
            990 => api.hide_window(WINDOW),
            300 => self.handle_receive_target_level_diff(param),
            190 | 210 | 200 | 220 => self.handle_update_hpmp(api, param),
            290 if api.get_option_bool(OPTION_SECTION, "IgnoreAggr") => self.ignore_aggr(api, param),
            50 | 110 => self.hold_target_id = -1,
            _ => {}
        }
    }

    fn on_click_button(&mut self, api: &mut dyn UiApi, id: &str) {
        let info = api.get_target_info().unwrap_or_default();
        match id {
            "BtnExpand" => {
                let expand = self.expand;
                self.set_expand_mode(api, expand);
                self.on_close_button(api);
            }
            "BtnClose" => self.on_close_button(api),
            "BtnExchange" => api.request_start_trade(info.id),
            "BtnInvite" => api.request_invite_party(&info.name),
            _ => {}
        }
    }

    fn on_frame_expand_click(&mut self, api: &mut dyn UiApi, expand: bool) {
        self.set_expand_mode(api, expand);
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::RecordingApi;
    use super::super::{Rect, UserInfo};
    use super::*;

    fn api() -> RecordingApi {
        RecordingApi {
            rects: vec![("TargetStatusWnd".into(), Rect { x: 100, y: 0, width: 176, height: 46 })],
            sysstrings: vec![(431, "Clan".into()), (591, "Alliance".into())],
            ..Default::default()
        }
    }

    fn calls(api: &RecordingApi, prefix: &str) -> Vec<String> {
        api.calls.iter().filter(|call| call.starts_with(prefix)).cloned().collect()
    }

    #[test]
    fn on_load_registers_events_collapses_and_hides_the_window_without_a_target() {
        let mut api = api();
        let mut script = Script::default();
        script.on_load(&mut api);
        assert_eq!(calls(&api, "RegisterEvent(").len(), 11);
        assert!(api.calls.contains(&"HideWindow(TargetStatusWnd)".to_string()));
        assert!(api.calls.contains(&"ShowWindow(TargetStatusWnd.BackTex)".to_string()));
        assert!(api.calls.contains(&"SetButtonTexture(TargetStatusWnd.BtnExpand,Interface.Shortcut_Minimize,Interface.Shortcut_Minimize_Down,Interface.Shortcut_Minimize_Over)".to_string()));
        assert_eq!(script.target_id, -1);
        assert!(!script.expand);
    }

    #[test]
    fn a_player_target_shows_name_rank_bumper_hp_and_mp_bars() {
        let mut api = api();
        api.target = Some(UserInfo { id: 55, name: "Bob".into(), user_rank: 3, cur_hp: 10, max_hp: 20, cur_mp: 5, max_mp: 9, ..Default::default() });
        api.player = Some(UserInfo { id: 1, ..Default::default() });
        let mut script = Script { show: true, ..Default::default() };
        script.on_event(&mut api, 980, "");
        assert_eq!(script.target_id, 55);
        assert_eq!(script.hold_target_id, 55);
        assert!(api.calls.contains(&"SetName(TargetStatusWnd.UserName,Bob,Normal,Center)".to_string()));
        assert!(api.calls.contains(&"SetName(TargetStatusWnd.RankName,rank3,Normal,Center)".to_string()));
        assert!(api.calls.contains(&"ShowWindow(TargetStatusWnd.NPHRN_Bumper)".to_string()));
        assert!(api.calls.contains(&"ShowWindow(TargetStatusWnd)".to_string()));
        assert!(api.calls.contains(&"HideWindow(TargetStatusWnd.barHP)".to_string()));
        assert!(api.calls.contains(&"HideWindow(TargetStatusWnd.txtPledge)".to_string()));
        api.calls.clear();
        script.on_event(&mut api, 210, "ServerID=55 ");
        assert!(!api.calls.is_empty());
        api.calls.clear();
        script.on_event(&mut api, 210, "ServerID=56 ");
        assert!(api.calls.is_empty());
        script.on_event(&mut api, 990, "");
        assert_eq!(api.calls, ["HideWindow(TargetStatusWnd)"]);
    }

    #[test]
    fn an_attackable_npc_shows_the_colored_name_and_hp_bar_and_the_expanded_tree() {
        let mut api = api();
        api.target = Some(UserInfo { id: 70, name: "Wolf".into(), npc: true, can_be_attacked: true, cur_hp: 30, max_hp: 60, class_id: 20000, ..Default::default() });
        api.npc_property = Some(vec![4, 1, 7, 2]);
        let mut script = Script { show: true, ..Default::default() };
        script.on_frame_expand_click(&mut api, true);
        assert!(api.calls.contains(&"SetNameWithColor(TargetStatusWnd.UserName,Wolf,Normal,Center,FFFFFFFF)".to_string()));
        assert!(api.calls.contains(&"BarSetValue(TargetStatusWnd.barHP,60,30)".to_string()));
        assert!(api.calls.contains(&"ShowWindow(TargetStatusWnd.barHP)".to_string()));
        assert!(api.calls.contains(&"HideWindow(TargetStatusWnd.barMP)".to_string()));
        assert!(api.calls.contains(&"ShowWindow(TargetStatusWnd.NpcInfo)".to_string()));
        let tree = ["TreeInsertNode(TargetStatusWnd.NpcInfo,,root)", "TreeInsertNode(TargetStatusWnd.NpcInfo,root,0)", "TreeInsertNode(TargetStatusWnd.NpcInfo,root,1)"];
        assert_eq!(calls(&api, "TreeInsertNode("), [tree, tree].concat());
        let items = ["TreeInsertNodeItem(TargetStatusWnd.NpcInfo,0,skillicon4_1)", "TreeInsertNodeItem(TargetStatusWnd.NpcInfo,1,skillicon7_2)"];
        assert_eq!(calls(&api, "TreeInsertNodeItem("), [items, items].concat());
        assert!(api.calls.contains(&"HideWindow(TargetStatusWnd.BackTex)".to_string()));
    }

    #[test]
    fn a_pledge_target_in_expand_mode_colors_the_clan_line_and_places_it_after_the_crest() {
        let mut api = api();
        api.target = Some(UserInfo { id: 55, name: "Bob".into(), clan_id: 9, ..Default::default() });
        api.clan_type = Some(-1);
        api.clan_name_value = Some(0);
        api.clan_crest = true;
        api.alliance_name = "Ally".into();
        let mut script = Script { show: true, ..Default::default() };
        script.set_expand_mode(&mut api, true);
        assert!(api.calls.contains(&"SetText(TargetStatusWnd.txtPledgeName,clan9)".to_string()));
        assert!(api.calls.contains(&"SetTextColor(TargetStatusWnd.txtPledgeName,FFD1A702)".to_string()));
        assert!(api.calls.contains(&"SetTextureWithObject(TargetStatusWnd.texPledgeCrest,crest9)".to_string()));
        assert!(api.calls.contains(&"MoveTo(TargetStatusWnd.txtPledgeName,163,43)".to_string()));
        assert!(api.calls.contains(&"SetText(TargetStatusWnd.txtPledgeAllianceName,Ally)".to_string()));
        assert!(api.calls.contains(&"MoveTo(TargetStatusWnd.txtPledgeAllianceName,145,59)".to_string()));
    }

    #[test]
    fn buttons_timer_aggro_and_right_click_follow_the_script() {
        let mut api = api();
        api.target = Some(UserInfo { id: 55, name: "Bob".into(), ..Default::default() });
        api.script_ints = vec![("StatusWnd".into(), "m_UserID".into(), 42)];
        api.script_int_arrays = vec![("PartyWnd".into(), "m_arrID".into(), 2, 77)];
        api.options_bool = vec![(("Neophron".into(), "IgnoreAggr".into()), true)];
        let mut script = Script::default();
        script.on_enter_state(&mut api, "LoadingState");
        assert!(api.calls.contains(&"SetTimer(TargetStatusWnd,1,500)".to_string()));
        script.on_timer(&mut api, 1);
        assert_eq!(script.status_user_id, 42);
        api.calls.clear();
        script.on_click_button(&mut api, "BtnExchange");
        script.on_click_button(&mut api, "BtnInvite");
        script.on_click_button(&mut api, "BtnClose");
        assert_eq!(api.calls, ["RequestStartTrade(55)", "RequestInviteParty(Bob)", "RequestTargetCancel", "PlayConsoleSound(WindowClose)"]);
        script.on_event(&mut api, 290, "SkillID=28 AttackerID=77 ");
        assert_eq!(script.aggr_id, 0);
        script.on_event(&mut api, 290, "SkillID=28 AttackerID=78 ");
        assert_eq!(script.aggr_id, 78);
        script.on_event(&mut api, 290, "SkillID=29 AttackerID=79 ");
        assert_eq!(script.aggr_id, 78);
        script.on_event(&mut api, 50, "");
        assert_eq!(script.hold_target_id, -1);
        api.calls.clear();
        script.on_r_button_up(&mut api, 150, 10);
        assert_eq!(api.calls, ["SetChatMessage( Bob )", "ReleaseFocus(ChatWnd.ChatEditBox)"]);
        api.calls.clear();
        script.on_r_button_up(&mut api, 100, 10);
        assert!(api.calls.is_empty());
    }
}
