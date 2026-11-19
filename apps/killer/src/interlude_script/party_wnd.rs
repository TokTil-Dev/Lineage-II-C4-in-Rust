use super::ui_common_api::{is_debuff_custom, is_noblesse_custom};
use super::{NameCtrlType, StatusIconInfo, TextAlign, UiApi, WindowScript, make_tooltip_simple_text, parse_int, parse_string};

pub const WINDOW: &str = "PartyWnd";
pub const OPTION_WINDOW: &str = "PartyWndOption";
pub const BTN_BUFF: &str = "PartyWnd.btnBuff";
pub const EVENTS: [i32; 7] = [1000, 1140, 1150, 1160, 1170, 1180, 40];
pub const DEBUFF_BACK: &str = "L2UI_CH3.Debuff";
pub const ABNORMAL_BACK: &str = "L2UI.EtcWndBack.AbnormalBack";
pub const PARTY_STATUS_MAX_COUNT: usize = 8;
pub const PARTY_STATUS_HEIGHT: i32 = 46;
pub const STATUS_ICON_MAX_COL: i32 = 12;
pub const LEADER_ICON: &str = "L2UI_CH3.PartyWnd.party_leadericon";
pub const BUFF_TOOLTIP_SYSSTRINGS: [i32; 3] = [1496, 1497, 1498];
pub const OPTION_SECTION: &str = "Neophron";

pub fn status(index: usize) -> String {
    format!("PartyWnd.PartyStatusWnd{index}")
}

pub fn player_name(index: usize) -> String {
    format!("{}.PlayerName", status(index))
}

pub fn class_icon(index: usize) -> String {
    format!("{}.ClassIcon", status(index))
}

pub fn leader_icon(index: usize) -> String {
    format!("{}.LeaderIcon", status(index))
}

pub fn status_icon_buff(index: usize) -> String {
    format!("{}.StatusIconBuff", status(index))
}

pub fn status_icon_debuff(index: usize) -> String {
    format!("{}.StatusIconDebuff", status(index))
}

pub fn bar_cp(index: usize) -> String {
    format!("{}.barCP", status(index))
}

pub fn bar_hp(index: usize) -> String {
    format!("{}.barHP", status(index))
}

pub fn bar_mp(index: usize) -> String {
    format!("{}.barMP", status(index))
}

#[derive(Debug, Default)]
pub struct Script {
    pub compact: bool,
    pub buff: bool,
    pub ids: [i32; PARTY_STATUS_MAX_COUNT],
    pub cur_count: i32,
    pub cur_bf: i32,
}

fn copy_status_icons(api: &mut dyn UiApi, destination: &str, source: &str) {
    api.clear(destination);
    let max_row = api.status_icon_row_count(source);
    for row in 0..max_row {
        api.status_icon_add_row(destination);
        let max_col = api.status_icon_col_count(source, row);
        for col in 0..max_col {
            if let Some(info) = api.status_icon_get_item(source, row, col) {
                api.status_icon_add_col(destination, row, &info);
            }
        }
    }
}

impl Script {
    pub fn handle_restart(&mut self, api: &mut dyn UiApi) {
        self.clear(api);
    }

    pub fn clear(&mut self, api: &mut dyn UiApi) {
        for index in 0..PARTY_STATUS_MAX_COUNT {
            self.clear_status(api, index);
        }
        self.cur_count = 0;
        self.resize_wnd(api);
    }

    pub fn clear_status(&mut self, api: &mut dyn UiApi, index: usize) {
        api.clear(&status_icon_buff(index));
        api.clear(&status_icon_debuff(index));
        api.set_name(&player_name(index), "", NameCtrlType::Normal, TextAlign::Center);
        api.set_texture(&leader_icon(index), "");
        api.set_texture(&class_icon(index), "");
        self.update_cp_bar(api, index, 0, 0);
        self.update_hp_bar(api, index, 0, 0);
        self.update_mp_bar(api, index, 0, 0);
        if let Some(slot) = self.ids.get_mut(index) {
            *slot = 0;
        }
    }

    pub fn copy_status(&mut self, api: &mut dyn UiApi, des_index: usize, src_index: usize) {
        if des_index >= PARTY_STATUS_MAX_COUNT || src_index >= PARTY_STATUS_MAX_COUNT {
            return;
        }
        self.ids[des_index] = self.ids[src_index];
        let name = api.get_name(&player_name(src_index));
        api.set_name(&player_name(des_index), &name, NameCtrlType::Normal, TextAlign::Center);
        let class_texture = api.get_texture_name(&class_icon(src_index));
        api.set_texture(&class_icon(des_index), &class_texture);
        let tooltip = api.get_tooltip_custom(&class_icon(src_index));
        api.set_tooltip_custom(&class_icon(des_index), &tooltip);
        let leader_texture = api.get_texture_name(&leader_icon(src_index));
        api.set_texture(&leader_icon(des_index), &leader_texture);
        if !leader_texture.is_empty() {
            let (width, _) = api.get_text_size(&leader_texture);
            api.set_anchor(&leader_icon(des_index), &status(des_index), "TopCenter", "TopLeft", -(width / 2) - 18, 8);
            api.show_window(&leader_icon(des_index));
        }
        let tooltip = api.get_tooltip_custom(&leader_icon(src_index));
        api.set_tooltip_custom(&leader_icon(des_index), &tooltip);
        for (des, src) in [(bar_cp(des_index), bar_cp(src_index)), (bar_hp(des_index), bar_hp(src_index)), (bar_mp(des_index), bar_mp(src_index))] {
            let (max_value, cur_value) = api.bar_get_value(&src);
            api.bar_set_value(&des, max_value, cur_value);
        }
        copy_status_icons(api, &status_icon_buff(des_index), &status_icon_buff(src_index));
        copy_status_icons(api, &status_icon_debuff(des_index), &status_icon_debuff(src_index));
    }

    pub fn resize_wnd(&mut self, api: &mut dyn UiApi) {
        let small = api.get_option_bool("Game", "SmallPartyWnd");
        let modern = api.get_option_bool(OPTION_SECTION, "ModernParty");
        if self.cur_count > 0 {
            let rect = api.get_rect(WINDOW);
            api.set_window_size(WINDOW, rect.width, PARTY_STATUS_HEIGHT * self.cur_count);
            api.set_resize_frame_size(WINDOW, 10, PARTY_STATUS_HEIGHT * self.cur_count);
            if !small && !modern {
                api.show_window(WINDOW);
            } else {
                api.hide_window(WINDOW);
            }
            for index in 0..PARTY_STATUS_MAX_COUNT {
                if (index as i32) < self.cur_count {
                    api.show_window(&status(index));
                } else {
                    api.hide_window(&status(index));
                }
            }
        } else {
            api.hide_window(WINDOW);
        }
    }

    pub fn find_party_id(&self, id: i32) -> i32 {
        for (index, known) in self.ids.iter().enumerate() {
            if *known == id {
                return index as i32;
            }
        }
        -1
    }

    pub fn handle_party_add_party(&mut self, api: &mut dyn UiApi, param: &str) {
        let id = parse_int(param, "ID").unwrap_or(0);
        if id > 0 {
            self.cur_count += 1;
            self.resize_wnd(api);
            let index = (self.cur_count - 1) as usize;
            if let Some(slot) = self.ids.get_mut(index) {
                *slot = id;
            }
            self.update_status(api, self.cur_count - 1, param);
        }
    }

    pub fn handle_party_update_party(&mut self, api: &mut dyn UiApi, param: &str) {
        let id = parse_int(param, "ID").unwrap_or(0);
        if id > 0 {
            let index = self.find_party_id(id);
            self.update_status(api, index, param);
        }
    }

    pub fn handle_party_delete_party(&mut self, api: &mut dyn UiApi, param: &str) {
        let id = parse_int(param, "ID").unwrap_or(0);
        if id > 0 {
            let index = self.find_party_id(id);
            if index > -1 {
                let mut i = index;
                while i < self.cur_count - 1 {
                    self.copy_status(api, i as usize, i as usize + 1);
                    i += 1;
                }
                let last = self.cur_count as usize;
                self.clear_status(api, last);
                self.cur_count -= 1;
                self.resize_wnd(api);
            }
        }
    }

    pub fn handle_party_delete_all_party(&mut self, api: &mut dyn UiApi) {
        self.clear(api);
    }

    pub fn update_status(&mut self, api: &mut dyn UiApi, index: i32, param: &str) {
        if index < 0 || index >= PARTY_STATUS_MAX_COUNT as i32 {
            return;
        }
        let index = index as usize;
        let name = parse_string(param, "Name").unwrap_or_default();
        let id = parse_int(param, "ID").unwrap_or(0);
        let cp = parse_int(param, "CurCP").unwrap_or(0);
        let max_cp = parse_int(param, "MaxCP").unwrap_or(0);
        let hp = parse_int(param, "CurHP").unwrap_or(0);
        let max_hp = parse_int(param, "MaxHP").unwrap_or(0);
        let mp = parse_int(param, "CurMP").unwrap_or(0);
        let max_mp = parse_int(param, "MaxMP").unwrap_or(0);
        let class_id = parse_int(param, "ClassID").unwrap_or(0);
        let _level = parse_int(param, "Level").unwrap_or(0);
        api.set_name(&player_name(index), &name, NameCtrlType::Normal, TextAlign::Center);
        if let Some(master_id) = parse_int(param, "MasterID") {
            if master_id > 0 && master_id == id {
                let routing_type = parse_int(param, "RoutingType").unwrap_or(0);
                api.set_texture(&leader_icon(index), LEADER_ICON);
                let routing = api.get_routing_string(routing_type);
                api.set_tooltip_custom(&leader_icon(index), &make_tooltip_simple_text(&routing));
                let (width, _) = api.get_text_size(&name);
                api.set_anchor(&leader_icon(index), &status(index), "TopCenter", "TopLeft", -(width / 2) - 18, 8);
            } else {
                api.set_texture(&leader_icon(index), "");
                api.set_tooltip_custom(&leader_icon(index), &make_tooltip_simple_text(""));
            }
        }
        api.debug(&format!("GetClassIconName(ClassID) = {}", api.get_class_icon_name(class_id)));
        if api.get_option_bool(OPTION_SECTION, "ShowClassLabels") {
            let texture = api.class_label_texture(class_id);
            api.set_texture(&class_icon(index), &texture);
            api.set_texture_size(&class_icon(index), 32, 32);
            api.set_window_size(&class_icon(index), 15, 15);
            api.set_texture_ctrl_type(&class_icon(index), 0);
        } else {
            let texture = api.get_class_icon_name(class_id);
            api.set_texture(&class_icon(index), &texture);
            api.set_texture_size(&class_icon(index), 0, 0);
            api.set_window_size(&class_icon(index), 11, 11);
            api.set_texture_ctrl_type(&class_icon(index), 4);
        }
        let tooltip = format!("{} - {}", api.get_class_str(class_id), api.get_class_type(class_id));
        api.set_tooltip_custom(&class_icon(index), &make_tooltip_simple_text(&tooltip));
        self.update_cp_bar(api, index, cp, max_cp);
        self.update_hp_bar(api, index, hp, max_hp);
        self.update_mp_bar(api, index, mp, max_mp);
    }

    pub fn handle_party_spelled_list(&mut self, api: &mut dyn UiApi, param: &str) {
        let mut debuff_row = -1;
        let mut buff_row = -1;
        let mut debuff_count = 0;
        let mut buff_count = 0;
        let id = parse_int(param, "ID").unwrap_or(0);
        if id < 1 {
            return;
        }
        let index = self.find_party_id(id);
        if index < 0 {
            return;
        }
        let index = index as usize;
        api.clear(&status_icon_buff(index));
        api.clear(&status_icon_debuff(index));
        let mut info = StatusIconInfo { size: 16, show: true, ..Default::default() };
        let max = parse_int(param, "Max").unwrap_or(0);
        for i in 0..max {
            info.class_id = parse_int(param, &format!("SkillID_{i}")).unwrap_or(info.class_id);
            info.level = parse_int(param, &format!("Level_{i}")).unwrap_or(info.level);
            if info.class_id > 0 {
                info.icon_name = api.skill_icon_name(info.class_id, info.level);
                if api.is_debuff(info.class_id, info.level) || is_debuff_custom(info.class_id) {
                    info.back_tex = if is_noblesse_custom(info.class_id) { ABNORMAL_BACK } else { DEBUFF_BACK }.to_string();
                    if debuff_count % STATUS_ICON_MAX_COL == 0 {
                        debuff_row += 1;
                        api.status_icon_add_row(&status_icon_debuff(index));
                    }
                    api.status_icon_add_col(&status_icon_debuff(index), debuff_row, &info);
                    debuff_count += 1;
                } else {
                    if buff_count % STATUS_ICON_MAX_COL == 0 {
                        buff_row += 1;
                        api.status_icon_add_row(&status_icon_buff(index));
                    }
                    api.status_icon_add_col(&status_icon_buff(index), buff_row, &info);
                    buff_count += 1;
                }
            }
        }
        self.update_buff(api);
    }

    pub fn handle_show_buff_icon(&mut self, api: &mut dyn UiApi, param: &str) {
        let _show = parse_int(param, "Show").unwrap_or(0);
        self.cur_bf += 1;
        if self.cur_bf > 2 {
            self.cur_bf = 0;
        }
        self.set_buff_button_tooltip(api);
        match self.cur_bf {
            1 | 2 => self.update_buff(api),
            0 => {
                self.cur_bf = 0;
                self.update_buff(api);
            }
            _ => {}
        }
    }

    pub fn on_open_party_wnd_option(&mut self, api: &mut dyn UiApi) {
        api.script_call(OPTION_WINDOW, "ShowPartyWndOption");
        api.set_anchor(OPTION_WINDOW, "PartyWnd.PartyStatusWnd0", "TopRight", "TopLeft", 5, 5);
    }

    pub fn on_buff_button(&mut self, api: &mut dyn UiApi) {
        self.cur_bf += 1;
        if self.cur_bf > 2 {
            self.cur_bf = 0;
        }
        self.set_buff_button_tooltip(api);
        self.update_buff(api);
    }

    pub fn update_buff(&mut self, api: &mut dyn UiApi) {
        for index in 0..PARTY_STATUS_MAX_COUNT {
            match self.cur_bf {
                1 => {
                    api.show_window(&status_icon_buff(index));
                    api.hide_window(&status_icon_debuff(index));
                }
                2 => {
                    api.hide_window(&status_icon_buff(index));
                    api.show_window(&status_icon_debuff(index));
                }
                _ => {
                    api.hide_window(&status_icon_buff(index));
                    api.hide_window(&status_icon_debuff(index));
                }
            }
        }
    }

    pub fn update_cp_bar(&self, api: &mut dyn UiApi, index: usize, value: i32, max_value: i32) {
        api.bar_set_value(&bar_cp(index), max_value, value);
    }

    pub fn update_hp_bar(&self, api: &mut dyn UiApi, index: usize, value: i32, max_value: i32) {
        api.bar_set_value(&bar_hp(index), max_value, value);
    }

    pub fn update_mp_bar(&self, api: &mut dyn UiApi, index: usize, value: i32, max_value: i32) {
        api.bar_set_value(&bar_mp(index), max_value, value);
    }

    fn member_at(&self, api: &dyn UiApi, x: i32, y: i32) -> Option<i32> {
        let rect = api.get_rect(WINDOW);
        if x > rect.x + 13 && x < (rect.x + rect.width) - 10 {
            let index = (y - rect.y) / PARTY_STATUS_HEIGHT;
            return usize::try_from(index).ok().and_then(|index| self.ids.get(index).copied());
        }
        None
    }

    pub fn on_l_button_down(&mut self, api: &mut dyn UiApi, x: i32, y: i32) {
        if let Some(member) = self.member_at(api, x, y)
            && let Some(info) = api.get_player_info()
        {
            if api.is_pk_mode() {
                api.request_attack(member, info.loc);
            } else {
                api.request_action(member, info.loc);
            }
        }
    }

    pub fn on_r_button_up(&mut self, api: &mut dyn UiApi, x: i32, y: i32) {
        if let Some(member) = self.member_at(api, x, y)
            && let Some(info) = api.get_player_info()
        {
            api.request_assist(member, info.loc);
        }
    }

    pub fn set_buff_button_tooltip(&self, api: &mut dyn UiApi) {
        let id = match self.cur_bf {
            0 => BUFF_TOOLTIP_SYSSTRINGS[0],
            1 => BUFF_TOOLTIP_SYSSTRINGS[1],
            2 => BUFF_TOOLTIP_SYSSTRINGS[2],
            _ => 0,
        };
        let text = api.get_system_string(id);
        api.set_tooltip_custom(BTN_BUFF, &make_tooltip_simple_text(&text));
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        for event in EVENTS {
            api.register_event(event);
        }
        self.compact = false;
        self.buff = false;
        self.cur_bf = 0;
        for index in 0..PARTY_STATUS_MAX_COUNT {
            api.set_anchor(&status_icon_buff(index), &status(index), "TopRight", "TopLeft", -3, 5);
            api.set_anchor(&status_icon_debuff(index), &status(index), "TopRight", "TopLeft", -3, 5);
        }
        api.hide_window(OPTION_WINDOW);
    }

    fn on_show(&mut self, api: &mut dyn UiApi) {
        self.set_buff_button_tooltip(api);
    }

    fn on_enter_state(&mut self, api: &mut dyn UiApi, _previous: &str) {
        self.compact = false;
        self.buff = false;
        self.cur_bf = 0;
        self.resize_wnd(api);
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        match event_id {
            1140 => self.handle_party_add_party(api, param),
            1150 => self.handle_party_update_party(api, param),
            1160 => self.handle_party_delete_party(api, param),
            1170 => self.handle_party_delete_all_party(api),
            1180 => self.handle_party_spelled_list(api, param),
            1000 => self.handle_show_buff_icon(api, param),
            40 => self.handle_restart(api),
            _ => {}
        }
    }

    fn on_click_button(&mut self, api: &mut dyn UiApi, id: &str) {
        match id {
            "btnBuff" => {
                self.on_buff_button(api);
                api.script_call("PartyWndCompact", "OnBuffButton");
            }
            "btnCompact" => self.on_open_party_wnd_option(api),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::RecordingApi;
    use super::super::{Rect, UserInfo, Vector};
    use super::*;

    fn api() -> RecordingApi {
        RecordingApi {
            rects: vec![("PartyWnd".into(), Rect { x: 0, y: 160, width: 176, height: 368 })],
            sysstrings: vec![(1496, "buffs".into()), (1497, "debuffs".into()), (1498, "none".into())],
            ..Default::default()
        }
    }

    fn calls(api: &RecordingApi, prefix: &str) -> Vec<String> {
        api.calls.iter().filter(|call| call.starts_with(prefix)).cloned().collect()
    }

    #[test]
    fn on_load_registers_events_anchors_the_icon_grids_and_hides_the_option_window() {
        let mut api = api();
        let mut script = Script::default();
        script.on_load(&mut api);
        assert_eq!(calls(&api, "RegisterEvent(").len(), 7);
        assert_eq!(calls(&api, "SetAnchor(").len(), 16);
        assert_eq!(calls(&api, "SetAnchor(")[0], "SetAnchor(PartyWnd.PartyStatusWnd0.StatusIconBuff,PartyWnd.PartyStatusWnd0,TopRight,TopLeft,-3,5)");
        assert_eq!(api.calls.last().unwrap(), "HideWindow(PartyWndOption)");
        api.calls.clear();
        script.on_enter_state(&mut api, "LoadingState");
        assert_eq!(api.calls, ["HideWindow(PartyWnd)"]);
    }

    #[test]
    fn adding_members_resizes_and_fills_the_rows_and_deleting_shifts_them() {
        let mut api = api();
        let mut script = Script::default();
        script.on_event(&mut api, 1140, "ID=11 Name=Ana CurCP=1 MaxCP=2 CurHP=3 MaxHP=4 CurMP=5 MaxMP=6 ClassID=10 Level=50 MasterID=11 RoutingType=1 ");
        assert_eq!(script.cur_count, 1);
        assert_eq!(script.ids[0], 11);
        assert!(api.calls.contains(&"SetWindowSize(PartyWnd,176,46)".to_string()));
        assert!(api.calls.contains(&"SetResizeFrameSize(PartyWnd,10,46)".to_string()));
        assert!(api.calls.contains(&"ShowWindow(PartyWnd)".to_string()));
        assert!(api.calls.contains(&"ShowWindow(PartyWnd.PartyStatusWnd0)".to_string()));
        assert!(api.calls.contains(&"HideWindow(PartyWnd.PartyStatusWnd1)".to_string()));
        assert!(api.calls.contains(&"SetName(PartyWnd.PartyStatusWnd0.PlayerName,Ana,Normal,Center)".to_string()));
        assert!(api.calls.contains(&"SetTexture(PartyWnd.PartyStatusWnd0.LeaderIcon,L2UI_CH3.PartyWnd.party_leadericon)".to_string()));
        assert!(api.calls.contains(&"SetTooltipCustomType(PartyWnd.PartyStatusWnd0.LeaderIcon,routing1)".to_string()));
        assert!(api.calls.contains(&"SetAnchor(PartyWnd.PartyStatusWnd0.LeaderIcon,PartyWnd.PartyStatusWnd0,TopCenter,TopLeft,-27,8)".to_string()));
        assert!(api.calls.contains(&"SetTexture(PartyWnd.PartyStatusWnd0.ClassIcon,icon10)".to_string()));
        assert!(api.calls.contains(&"SetTextureCtrlType(PartyWnd.PartyStatusWnd0.ClassIcon,4)".to_string()));
        assert!(api.calls.contains(&"BarSetValue(PartyWnd.PartyStatusWnd0.barHP,4,3)".to_string()));
        script.on_event(&mut api, 1140, "ID=12 Name=Bia ");
        assert_eq!((script.cur_count, script.ids[1]), (2, 12));
        api.calls.clear();
        script.on_event(&mut api, 1160, "ID=11 ");
        assert_eq!((script.cur_count, script.ids[0], script.ids[1], script.ids[2]), (1, 12, 12, 0));
        assert!(api.calls.contains(&"SetName(PartyWnd.PartyStatusWnd2.PlayerName,,Normal,Center)".to_string()));
        assert!(api.calls.contains(&"HideWindow(PartyWnd.PartyStatusWnd1)".to_string()));
        script.on_event(&mut api, 1170, "");
        assert_eq!(script.cur_count, 0);
        assert_eq!(api.calls.last().unwrap(), "HideWindow(PartyWnd)");
    }

    #[test]
    fn spelled_list_splits_buffs_and_debuffs_into_rows_of_twelve_and_the_buff_button_cycles() {
        let mut api = api();
        api.debuffs = vec![1069];
        let mut script = Script::default();
        script.on_event(&mut api, 1140, "ID=11 Name=Ana ");
        api.calls.clear();
        let mut param = String::from("ID=11 Max=14 ");
        for i in 0..13 {
            param.push_str(&format!("SkillID_{i}={} Level_{i}=1 ", 1000 + i));
        }
        param.push_str("SkillID_13=1069 Level_13=1 ");
        script.on_event(&mut api, 1180, &param);
        assert_eq!(calls(&api, "AddRow(PartyWnd.PartyStatusWnd0.StatusIconBuff").len(), 2);
        assert_eq!(calls(&api, "AddCol(PartyWnd.PartyStatusWnd0.StatusIconBuff").len(), 13);
        assert_eq!(calls(&api, "AddCol(PartyWnd.PartyStatusWnd0.StatusIconDebuff"), ["AddCol(PartyWnd.PartyStatusWnd0.StatusIconDebuff,0,skillicon1069_1)"]);
        assert!(api.calls.contains(&"HideWindow(PartyWnd.PartyStatusWnd0.StatusIconBuff)".to_string()));
        api.calls.clear();
        script.on_click_button(&mut api, "btnBuff");
        assert_eq!(script.cur_bf, 1);
        assert!(api.calls.contains(&"SetTooltipCustomType(PartyWnd.btnBuff,debuffs)".to_string()));
        assert!(api.calls.contains(&"ShowWindow(PartyWnd.PartyStatusWnd3.StatusIconBuff)".to_string()));
        assert_eq!(api.calls.last().unwrap(), "ScriptCall(PartyWndCompact,OnBuffButton)");
        script.on_event(&mut api, 1000, "Show=1 ");
        assert_eq!(script.cur_bf, 2);
        script.on_event(&mut api, 1000, "Show=1 ");
        assert_eq!(script.cur_bf, 0);
    }

    #[test]
    fn clicks_on_a_row_act_on_that_member_and_the_option_button_anchors_the_option_window() {
        let mut api = api();
        api.player = Some(UserInfo { id: 1, loc: Vector { x: 1.0, y: 2.0, z: 3.0 }, ..Default::default() });
        let mut script = Script::default();
        script.on_event(&mut api, 1140, "ID=11 Name=Ana ");
        script.on_event(&mut api, 1140, "ID=12 Name=Bia ");
        api.calls.clear();
        script.on_l_button_down(&mut api, 50, 160 + 46 + 3);
        assert_eq!(api.calls, ["RequestAction(12,1,2,3)"]);
        api.pk_mode = true;
        api.calls.clear();
        script.on_l_button_down(&mut api, 50, 161);
        assert_eq!(api.calls, ["RequestAttack(11,1,2,3)"]);
        api.calls.clear();
        script.on_r_button_up(&mut api, 50, 161);
        assert_eq!(api.calls, ["RequestAssist(11,1,2,3)"]);
        api.calls.clear();
        script.on_l_button_down(&mut api, 13, 161);
        assert!(api.calls.is_empty());
        script.on_click_button(&mut api, "btnCompact");
        assert_eq!(api.calls, ["ScriptCall(PartyWndOption,ShowPartyWndOption)", "SetAnchor(PartyWndOption,PartyWnd.PartyStatusWnd0,TopRight,TopLeft,5,5)"]);
    }
}
