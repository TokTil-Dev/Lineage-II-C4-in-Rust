use super::{UiApi, Vector, WindowScript, parse_float, parse_int, parse_string};

pub const MINIMAP: &str = "MinimapWnd.Minimap";
pub const CURSED_COMBO: &str = "MinimapWnd.Container.CursedComboBox";
pub const EXPAND_WND: &str = "MinimapWnd_Expand";
pub const GUIDE_WND: &str = "GuideWnd";
pub const EVENT_SHOW_MINIMAP: i32 = 1780;
pub const EVENT_PARTY_MEMBER_CHANGED: i32 = 520;
pub const EVENT_ADD_TARGET: i32 = 1790;
pub const EVENT_DELETE_TARGET: i32 = 1800;
pub const EVENT_DELETE_ALL_TARGET: i32 = 1810;
pub const EVENT_SHOW_QUEST: i32 = 1820;
pub const EVENT_HIDE_QUEST: i32 = 1830;
pub const EVENT_MY_LOCATION: i32 = 1840;
pub const EVENT_CURSED_WEAPON_LIST: i32 = 1850;
pub const EVENT_CURSED_WEAPON_LOCATION: i32 = 1860;
pub const EVENT_CURRENT_LOCATION: i32 = 2420;
pub const EVENT_SHOW_REDUCE: i32 = 1870;
pub const EVENT_HIDE_REDUCE: i32 = 1880;
pub const EVENT_GAME_TIME: i32 = 1890;
pub const CURSED_ICON_BOTH: &str = "L2UI_CH3.MiniMap.cursedmapicon00";
pub const CURSED_ICON_ZARICHE: &str = "L2UI_CH3.MiniMap.cursedmapicon01";
pub const CURSED_ICON_AKAMANAH: &str = "L2UI_CH3.MiniMap.cursedmapicon02";
pub const ZARICHE_ID: i32 = 8190;
pub const AKAMANAH_ID: i32 = 8689;

#[derive(Debug, Default)]
pub struct Script {
    pub party_member_count: i32,
    pub party_loc_index: i32,
    pub is_show_guide_wnd: i32,
    pub adjust_cursed_loc: bool,
    pub ssq_status: i32,
    pub show_ssq_type: bool,
    pub show_current_location: bool,
    pub show_game_time: bool,
    pub expand_state: bool,
}

fn show_window_with_focus(api: &mut dyn UiApi, name: &str) {
    api.show_window(name);
    api.set_focus(name);
}

impl Script {
    pub fn set_ssq_type_text(&mut self, api: &mut dyn UiApi) {
        let text = match self.ssq_status {
            0 => api.get_system_string(973),
            1 => api.get_system_string(974),
            2 => api.get_system_string(975),
            3 => api.get_system_string(976),
            _ => String::new(),
        };
        api.set_text("Minimapwnd.txtVarSSQType", &text);
    }

    pub fn set_current_location(&mut self, api: &mut dyn UiApi) {
        let zone_name = api.get_current_zone_name();
        api.set_text("Minimapwnd.txtVarCurLoc", &zone_name);
    }

    pub fn handle_party_member_changed(&mut self, param: &str) {
        if let Some(count) = parse_int(param, "PartyMemberCount") {
            self.party_member_count = count;
        }
    }

    pub fn set_expand_state(&mut self, expand_state: bool) {
        self.expand_state = expand_state;
    }

    pub fn is_expand_state(&self) -> bool {
        self.expand_state
    }

    pub fn handle_show_minimap(&mut self, api: &mut dyn UiApi, param: &str) {
        api.debug("HandleShowMiniap");
        if let Some(ssq_status) = parse_int(param, "SSQStatus") {
            api.minimap_set_ssq_status(MINIMAP, ssq_status);
            self.ssq_status = ssq_status;
        }
        if self.is_expand_state() {
            if api.is_show_window(EXPAND_WND) {
                api.hide_window(EXPAND_WND);
            } else {
                api.hide_window("MinimapWnd");
                show_window_with_focus(api, EXPAND_WND);
            }
        } else if api.is_show_window("MinimapWnd") {
            api.hide_window("MinimapWnd");
        } else {
            api.hide_window(EXPAND_WND);
            show_window_with_focus(api, "MinimapWnd");
        }
        if api.is_show_window("MinimapWnd") || api.is_show_window(EXPAND_WND) {
            api.request_cursed_weapon_list();
            api.request_cursed_weapon_location();
        }
    }

    pub fn handle_minimap_add_target(&mut self, api: &mut dyn UiApi, param: &str) {
        api.debug(&format!("~~~MinimapWnd - HandleMiniampAddTarget~~~~~~{param}"));
        if let (Some(x), Some(y), Some(z)) = (parse_float(param, "X"), parse_float(param, "Y"), parse_float(param, "Z")) {
            let loc = Vector { x, y, z };
            api.minimap_add_target(MINIMAP, loc);
            api.minimap_adjust_map_view(MINIMAP, loc, false, false);
        }
    }

    pub fn handle_minimap_delete_target(&mut self, api: &mut dyn UiApi, param: &str) {
        if let (Some(x), Some(y), Some(z)) = (parse_int(param, "X"), parse_int(param, "Y"), parse_int(param, "Z")) {
            api.minimap_delete_target(MINIMAP, Vector { x: x as f32, y: y as f32, z: z as f32 });
        }
    }

    pub fn handle_minimap_delete_all_target(&mut self, api: &mut dyn UiApi) {
        api.minimap_delete_all_target(MINIMAP);
    }

    pub fn handle_minimap_show_quest(&mut self, api: &mut dyn UiApi) {
        api.debug("MinimapWnd.HandleMinimapShowQuest");
        api.minimap_set_show_quest(MINIMAP, true);
    }

    pub fn handle_minimap_hide_quest(&mut self, api: &mut dyn UiApi) {
        api.debug("MinimapWnd.HandleMinimapHideQuest");
        api.minimap_set_show_quest(MINIMAP, false);
    }

    pub fn on_combo_box_item_selected(&mut self, api: &mut dyn UiApi, name: &str, index: i32) -> i32 {
        if name == "CursedComboBox" { api.combo_get_reserved(CURSED_COMBO, index) } else { 0 }
    }

    pub fn on_click_target_button(&mut self, api: &mut dyn UiApi) {
        if let Some(quest_location) = api.get_quest_location() {
            api.minimap_adjust_map_view(MINIMAP, quest_location, false, false);
        }
    }

    pub fn on_click_my_loc_button(&mut self, api: &mut dyn UiApi) {
        self.adjust_map_to_player_position(api, true);
    }

    pub fn adjust_map_to_player_position(&mut self, api: &mut dyn UiApi, zoom_to_town_map: bool) {
        let player_position = api.get_player_position();
        api.minimap_adjust_map_view(MINIMAP, player_position, zoom_to_town_map, false);
    }

    pub fn on_click_party_loc_button(&mut self, api: &mut dyn UiApi) {
        self.party_member_count = api.get_party_member_count();
        if self.party_member_count == 0 {
            return;
        }
        self.party_loc_index = ((self.party_loc_index + 1) as f32 % self.party_member_count as f32) as i32;
        if let Some(location) = api.get_party_member_location(self.party_loc_index) {
            api.minimap_adjust_map_view(MINIMAP, location, false, false);
        }
    }

    pub fn handle_cursed_weapon_list(&mut self, api: &mut dyn UiApi, param: &str) {
        let num = parse_int(param, "NUM").unwrap_or(0);
        api.combo_clear(CURSED_COMBO);
        let mut i = 0;
        while i < num + 1 {
            if i == 0 {
                let label = api.get_system_string(1463);
                api.combo_add_string(CURSED_COMBO, &label, 0);
            } else {
                let item_id = parse_int(param, &format!("ID{}", i - 1)).unwrap_or(0);
                let cursed_name = parse_string(param, &format!("NAME{}", i - 1)).unwrap_or_default();
                api.combo_add_string(CURSED_COMBO, &cursed_name, item_id);
                api.combo_set_selected_num(CURSED_COMBO, 0);
            }
            i += 1;
        }
        api.minimap_delete_all_target(MINIMAP);
    }

    pub fn handle_cursed_weapon_location(&mut self, api: &mut dyn UiApi, param: &str) {
        let num = parse_int(param, "NUM").unwrap_or(0);
        if num == 0 {
            if self.adjust_cursed_loc {
                let player = api.get_player_position();
                api.minimap_adjust_map_view(MINIMAP, player, false, false);
            }
            api.minimap_delete_all_cursed_weapon_icon(MINIMAP);
            return;
        }
        let mut item_id1 = 0;
        let mut item_id2 = 0;
        let mut is_owned1 = 0;
        let mut is_owned2 = 0;
        let mut cursed_name1 = String::new();
        let mut cursed_name2 = String::new();
        let mut loc1 = Vector::default();
        let mut loc2 = Vector::default();
        let mut i = 0;
        while i < num {
            let item_id = parse_int(param, &format!("ID{i}")).unwrap_or(0);
            let cursed_name = parse_string(param, &format!("NAME{i}")).unwrap_or_default();
            let is_owned = parse_int(param, &format!("ISOWNED{i}")).unwrap_or(0);
            let x = parse_int(param, &format!("X{i}")).unwrap_or(0);
            let y = parse_int(param, &format!("Y{i}")).unwrap_or(0);
            let z = parse_int(param, &format!("Z{i}")).unwrap_or(0);
            let location = Vector { x: x as f32, y: y as f32, z: z as f32 };
            match i {
                0 => {
                    item_id1 = item_id;
                    cursed_name1 = cursed_name;
                    is_owned1 = is_owned;
                    loc1 = location;
                    api.debug(&format!("무기1:{cursed_name1}, 위치: {}", vector_text(loc1)));
                }
                1 => {
                    item_id2 = item_id;
                    cursed_name2 = cursed_name;
                    is_owned2 = is_owned;
                    loc2 = location;
                    api.debug(&format!("무기2:{cursed_name2}, 위치: {}", vector_text(loc2)));
                }
                _ => {}
            }
            i += 1;
        }
        if self.adjust_cursed_loc {
            self.adjust_cursed_loc = false;
            let current = api.combo_get_selected_num(CURSED_COMBO);
            let combo_cursed_name = api.combo_get_string(CURSED_COMBO, current);
            if combo_cursed_name == cursed_name1 {
                api.minimap_adjust_map_view(MINIMAP, loc1, false, false);
            } else if combo_cursed_name == cursed_name2 {
                api.minimap_adjust_map_view(MINIMAP, loc2, false, false);
            } else {
                self.adjust_map_to_player_position(api, true);
            }
        } else if num == 1 {
            self.draw_cursed_weapon(api, MINIMAP, item_id1, &cursed_name1, loc1, is_owned1 == 0, true);
        } else if num == 2 {
            let combined = api.minimap_is_overlapped(MINIMAP, loc1.x as i32, loc1.y as i32, loc2.x as i32, loc2.y as i32);
            api.debug(&format!("컴바인 {combined}"));
            if combined {
                api.minimap_draw_grid_icon(
                    MINIMAP,
                    CURSED_ICON_BOTH,
                    CURSED_ICON_BOTH,
                    loc1,
                    true,
                    0,
                    -12,
                    &format!("{cursed_name1}\n{cursed_name2}"),
                );
            } else {
                api.debug(&format!("ownded: {is_owned1} {is_owned2}"));
                self.draw_cursed_weapon(api, MINIMAP, item_id1, &cursed_name1, loc1, is_owned1 == 0, true);
                self.draw_cursed_weapon(api, MINIMAP, item_id2, &cursed_name2, loc2, is_owned2 == 0, false);
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn draw_cursed_weapon(
        &mut self,
        api: &mut dyn UiApi,
        window_name: &str,
        item_id: i32,
        cursed_name: &str,
        loc: Vector,
        dropped: bool,
        refresh: bool,
    ) {
        let mut icon = if item_id == ZARICHE_ID {
            CURSED_ICON_ZARICHE.to_string()
        } else if item_id == AKAMANAH_ID {
            CURSED_ICON_AKAMANAH.to_string()
        } else {
            String::new()
        };
        if dropped {
            icon.push_str("_drop");
        }
        api.minimap_draw_grid_icon(window_name, &icon, CURSED_ICON_BOTH, loc, refresh, 0, -12, cursed_name);
    }

    pub fn handle_update_game_time(&mut self, api: &mut dyn UiApi, param: &str) {
        let mut game_hour = parse_int(param, "GameHour").unwrap_or(0);
        let game_minute = parse_int(param, "GameMinute").unwrap_or(0);
        self.select_sun_or_moon(api, game_hour);
        let mut text = if game_hour >= 12 {
            game_hour -= 12;
            "PM ".to_string()
        } else {
            "AM ".to_string()
        };
        if game_hour == 0 {
            game_hour = 12;
        }
        if game_hour < 10 {
            text = format!("{text}0{game_hour} : ");
        } else {
            text = format!("{text}{game_hour} : ");
        }
        if game_minute < 10 {
            text = format!("{text}0{game_minute}");
        } else {
            text = format!("{text}{game_minute}");
        }
        api.set_text("MinimapWnd.txtGameTime", &text);
    }

    pub fn select_sun_or_moon(&mut self, api: &mut dyn UiApi, game_hour: i32) {
        if (6..=24).contains(&game_hour) {
            api.show_window("MinimapWnd.texSun");
            api.hide_window("MinimapWnd.texMoon");
        } else {
            api.show_window("MinimapWnd.texMoon");
            api.hide_window("MinimapWnd.texSun");
        }
    }

    pub fn set_focus(&mut self, api: &mut dyn UiApi) {
        for name in [
            "MinimapWnd.PartyLocButton",
            "MinimapWnd.MyLocButton",
            "MinimapWnd.TargetButton",
            "MinimapWnd.ExpandButton",
            "MinimapWnd.Pursuit",
            "MinimapWnd.OpenGuideWnd",
            "MinimapWnd.CursedComboBox",
        ] {
            api.set_focus(name);
        }
    }
}

fn vector_text(v: Vector) -> String {
    format!("{:.2},{:.2},{:.2}", v.x, v.y, v.z)
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        self.party_loc_index = -1;
        self.party_member_count = api.get_party_member_count();
        for id in [
            EVENT_SHOW_MINIMAP,
            EVENT_PARTY_MEMBER_CHANGED,
            EVENT_ADD_TARGET,
            EVENT_DELETE_TARGET,
            EVENT_DELETE_ALL_TARGET,
            EVENT_SHOW_QUEST,
            EVENT_HIDE_QUEST,
            EVENT_MY_LOCATION,
            EVENT_CURSED_WEAPON_LIST,
            EVENT_CURSED_WEAPON_LOCATION,
            EVENT_CURRENT_LOCATION,
            EVENT_SHOW_REDUCE,
            EVENT_HIDE_REDUCE,
            EVENT_GAME_TIME,
        ] {
            api.register_event(id);
        }
        self.adjust_cursed_loc = false;
        self.show_ssq_type = true;
        self.show_current_location = true;
        self.show_game_time = true;
        self.expand_state = false;
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        match event_id {
            EVENT_SHOW_MINIMAP => self.handle_show_minimap(api, param),
            EVENT_PARTY_MEMBER_CHANGED => self.handle_party_member_changed(param),
            EVENT_ADD_TARGET => self.handle_minimap_add_target(api, param),
            EVENT_DELETE_TARGET => self.handle_minimap_delete_target(api, param),
            EVENT_DELETE_ALL_TARGET => self.handle_minimap_delete_all_target(api),
            EVENT_SHOW_QUEST => self.handle_minimap_show_quest(api),
            EVENT_HIDE_QUEST => self.handle_minimap_hide_quest(api),
            EVENT_MY_LOCATION => self.adjust_map_to_player_position(api, true),
            EVENT_CURSED_WEAPON_LIST => self.handle_cursed_weapon_list(api, param),
            EVENT_CURSED_WEAPON_LOCATION => self.handle_cursed_weapon_location(api, param),
            EVENT_CURRENT_LOCATION => self.set_current_location(api),
            EVENT_SHOW_REDUCE => api.show_window("MinimapWnd.Container.btnReduce"),
            EVENT_HIDE_REDUCE => api.hide_window("MinimapWnd.Container.btnReduce"),
            EVENT_GAME_TIME if self.show_game_time => self.handle_update_game_time(api, param),
            _ => {}
        }
    }

    fn on_show(&mut self, api: &mut dyn UiApi) {
        api.debug("MinimapWnd - OnShow");
        self.adjust_map_to_player_position(api, true);
        api.play_sound("interfacesound.Interface.map_open_01");
        if self.is_show_guide_wnd == 1 {
            api.show_window(GUIDE_WND);
        }
        self.is_show_guide_wnd = 0;
        self.set_ssq_type_text(api);
        self.set_current_location(api);
        self.set_focus(api);
    }

    fn on_hide(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window(GUIDE_WND) {
            self.is_show_guide_wnd = 1;
        }
        api.play_sound("interfacesound.Interface.map_close_01");
    }

    fn on_click_button(&mut self, api: &mut dyn UiApi, id: &str) {
        api.debug(id);
        match id {
            "TargetButton" => self.on_click_target_button(api),
            "MyLocButton" => self.on_click_my_loc_button(api),
            "PartyLocButton" => self.on_click_party_loc_button(api),
            "OpenGuideWnd" => {
                if api.is_show_window(GUIDE_WND) {
                    api.hide_window(GUIDE_WND);
                } else {
                    api.show_window(GUIDE_WND);
                }
            }
            "Pursuit" => {
                self.adjust_cursed_loc = true;
                api.request_cursed_weapon_location();
            }
            "ExpandButton" => {
                self.set_expand_state(true);
                show_window_with_focus(api, EXPAND_WND);
                api.hide_window("MinimapWnd");
            }
            "btnReduce" => {
                api.minimap_request_reduce_btn(MINIMAP);
                api.hide_window("MinimapWnd.Container.btnReduce");
            }
            "BtnClose" => api.hide_window("MinimapWnd"),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interlude_script::mock::RecordingApi;

    fn api() -> RecordingApi {
        RecordingApi {
            player: Some(super::super::UserInfo { loc: Vector { x: 100.0, y: 200.0, z: 30.0 }, ..Default::default() }),
            zone_name: "Giran".into(),
            ..Default::default()
        }
    }

    #[test]
    fn on_load_registers_the_fourteen_events_and_resets_the_flags() {
        let mut api = api();
        let mut script = Script::default();
        script.on_load(&mut api);
        let ids: Vec<String> = [1780, 520, 1790, 1800, 1810, 1820, 1830, 1840, 1850, 1860, 2420, 1870, 1880, 1890]
            .iter()
            .map(|id| format!("RegisterEvent({id})"))
            .collect();
        assert_eq!(api.calls, ids);
        assert_eq!(script.party_loc_index, -1);
        assert!(script.show_ssq_type && script.show_current_location && script.show_game_time && !script.expand_state);
    }

    #[test]
    fn on_show_adjusts_to_the_player_with_zoom_to_town_and_fills_ssq_and_zone_texts() {
        let mut api = RecordingApi { sysstrings: vec![(975, "Dusk".into())], ..api() };
        let mut script = Script { ssq_status: 2, is_show_guide_wnd: 1, ..Default::default() };
        script.on_show(&mut api);
        assert_eq!(api.calls[0], "AdjustMapView(MinimapWnd.Minimap,100,200,30,true,false)");
        assert_eq!(api.calls[1], "PlaySound(interfacesound.Interface.map_open_01)");
        assert_eq!(api.calls[2], "ShowWindow(GuideWnd)");
        assert_eq!(api.calls[3], "SetText(Minimapwnd.txtVarSSQType,Dusk)");
        assert_eq!(api.calls[4], "SetText(Minimapwnd.txtVarCurLoc,Giran)");
        assert_eq!(api.calls[5], "SetFocus(MinimapWnd.PartyLocButton)");
        assert_eq!(script.is_show_guide_wnd, 0);
    }

    #[test]
    fn show_minimap_event_toggles_the_window_and_requests_cursed_weapons_when_open() {
        let mut api = api();
        let mut script = Script::default();
        script.on_event(&mut api, 1780, "SSQStatus=1 ");
        assert_eq!(script.ssq_status, 1);
        assert!(api.calls.contains(&"SetSSQStatus(MinimapWnd.Minimap,1)".to_string()));
        assert!(api.calls.contains(&"ShowWindow(MinimapWnd)".to_string()));
        assert!(api.calls.contains(&"SetFocus(MinimapWnd)".to_string()));
        assert_eq!(api.calls.last().unwrap(), "RequestCursedWeaponLocation()");
        api.calls.clear();
        script.on_event(&mut api, 1780, "");
        assert_eq!(api.calls, ["HideWindow(MinimapWnd)"]);
    }

    #[test]
    fn targets_parse_floats_on_add_and_ints_on_delete() {
        let mut api = api();
        let mut script = Script::default();
        script.on_event(&mut api, 1790, "X=1.5 Y=2 Z=3");
        assert_eq!(api.calls[0], "MinimapAddTarget(MinimapWnd.Minimap,1.5,2,3)");
        assert_eq!(api.calls[1], "AdjustMapView(MinimapWnd.Minimap,1.5,2,3,false,false)");
        script.on_event(&mut api, 1800, "X=4 Y=5 Z=6");
        assert_eq!(api.calls[2], "MinimapDeleteTarget(MinimapWnd.Minimap,4,5,6)");
        script.on_event(&mut api, 1810, "");
        assert_eq!(api.calls[3], "MinimapDeleteAllTarget(MinimapWnd.Minimap)");
        script.on_event(&mut api, 1820, "");
        script.on_event(&mut api, 1830, "");
        assert_eq!(api.calls[4], "SetShowQuest(MinimapWnd.Minimap,true)");
        assert_eq!(api.calls[5], "SetShowQuest(MinimapWnd.Minimap,false)");
    }

    #[test]
    fn party_loc_button_cycles_members_and_target_button_uses_the_quest_location() {
        let mut api = RecordingApi {
            party_locations: vec![Vector { x: 1.0, y: 1.0, z: 1.0 }, Vector { x: 2.0, y: 2.0, z: 2.0 }],
            quest_location: Some(Vector { x: 9.0, y: 9.0, z: 9.0 }),
            ..api()
        };
        let mut script = Script { party_loc_index: -1, ..Default::default() };
        script.on_click_button(&mut api, "PartyLocButton");
        script.on_click_button(&mut api, "PartyLocButton");
        script.on_click_button(&mut api, "PartyLocButton");
        let adjusts: Vec<&String> = api.calls.iter().filter(|call| call.starts_with("AdjustMapView")).collect();
        assert_eq!(adjusts.len(), 3, "{:?}", api.calls);
        assert_eq!(adjusts[0], "AdjustMapView(MinimapWnd.Minimap,1,1,1,false,false)");
        assert_eq!(adjusts[1], "AdjustMapView(MinimapWnd.Minimap,2,2,2,false,false)");
        assert_eq!(adjusts[2], "AdjustMapView(MinimapWnd.Minimap,1,1,1,false,false)");
        api.calls.clear();
        script.on_click_button(&mut api, "TargetButton");
        assert_eq!(api.calls, ["AdjustMapView(MinimapWnd.Minimap,9,9,9,false,false)"]);
        api.party_locations.clear();
        api.calls.clear();
        script.on_click_button(&mut api, "PartyLocButton");
        assert!(api.calls.is_empty());
    }

    #[test]
    fn cursed_weapon_list_fills_the_combo_with_the_all_entry_first() {
        let mut api = RecordingApi { sysstrings: vec![(1463, "All".into())], ..api() };
        let mut script = Script::default();
        script.on_event(&mut api, 1850, "NUM=2 ID0=8190 NAME0=Zariche ID1=8689 NAME1=Akamanah ");
        assert_eq!(
            api.calls,
            [
                "ComboClear(MinimapWnd.Container.CursedComboBox)",
                "ComboAddString(MinimapWnd.Container.CursedComboBox,All,0)",
                "ComboAddString(MinimapWnd.Container.CursedComboBox,Zariche,8190)",
                "ComboSetSelectedNum(MinimapWnd.Container.CursedComboBox,0)",
                "ComboAddString(MinimapWnd.Container.CursedComboBox,Akamanah,8689)",
                "ComboSetSelectedNum(MinimapWnd.Container.CursedComboBox,0)",
                "MinimapDeleteAllTarget(MinimapWnd.Minimap)"
            ]
        );
    }

    #[test]
    fn cursed_weapon_location_draws_icons_with_drop_suffix_and_combines_overlapping_ones() {
        let mut api = api();
        let mut script = Script::default();
        script.on_event(&mut api, 1860, "NUM=1 ID0=8190 NAME0=Zariche ISOWNED0=0 X0=10 Y0=20 Z0=30");
        assert_eq!(
            api.calls.last().unwrap(),
            "DrawGridIcon(MinimapWnd.Minimap,L2UI_CH3.MiniMap.cursedmapicon01_drop,L2UI_CH3.MiniMap.cursedmapicon00,10,20,30,true,0,-12,Zariche)"
        );
        api.calls.clear();
        script.on_event(&mut api, 1860, "NUM=2 ID0=8190 NAME0=Z ISOWNED0=1 X0=1 Y0=2 Z0=3 ID1=8689 NAME1=A ISOWNED1=0 X1=4 Y1=5 Z1=6");
        let draws: Vec<&String> = api.calls.iter().filter(|call| call.starts_with("DrawGridIcon")).collect();
        assert_eq!(draws[0], "DrawGridIcon(MinimapWnd.Minimap,L2UI_CH3.MiniMap.cursedmapicon01,L2UI_CH3.MiniMap.cursedmapicon00,1,2,3,true,0,-12,Z)");
        assert_eq!(draws[1], "DrawGridIcon(MinimapWnd.Minimap,L2UI_CH3.MiniMap.cursedmapicon02_drop,L2UI_CH3.MiniMap.cursedmapicon00,4,5,6,false,0,-12,A)");
        api.calls.clear();
        script.adjust_cursed_loc = true;
        script.on_event(&mut api, 1860, "NUM=0");
        assert_eq!(api.calls, ["AdjustMapView(MinimapWnd.Minimap,100,200,30,false,false)", "DeleteAllCursedWeaponIcon(MinimapWnd.Minimap)"]);
    }

    #[test]
    fn pursuit_then_location_adjusts_to_the_combo_choice_or_falls_back_to_the_player() {
        let mut api = api();
        let mut script = Script::default();
        script.on_click_button(&mut api, "Pursuit");
        assert!(script.adjust_cursed_loc);
        assert_eq!(api.calls.last().unwrap(), "RequestCursedWeaponLocation()");
        api.calls.clear();
        script.on_event(&mut api, 1860, "NUM=1 ID0=8190 NAME0=Zariche ISOWNED0=0 X0=10 Y0=20 Z0=30");
        assert!(!script.adjust_cursed_loc);
        assert_eq!(api.calls.last().unwrap(), "AdjustMapView(MinimapWnd.Minimap,0,0,0,false,false)");
        api.calls.clear();
        script.adjust_cursed_loc = true;
        script.on_event(&mut api, 1860, "NUM=2 ID0=8190 NAME0=Z ISOWNED0=1 X0=1 Y0=2 Z0=3 ID1=8689 NAME1=A ISOWNED1=0 X1=4 Y1=5 Z1=6");
        assert_eq!(api.calls.last().unwrap(), "AdjustMapView(MinimapWnd.Minimap,100,200,30,true,false)");
    }

    #[test]
    fn game_time_formats_am_pm_with_two_digits_and_picks_sun_or_moon() {
        let mut api = api();
        let mut script = Script { show_game_time: true, ..Default::default() };
        script.on_event(&mut api, 1890, "GameHour=13 GameMinute=5");
        assert_eq!(api.calls, ["ShowWindow(MinimapWnd.texSun)", "HideWindow(MinimapWnd.texMoon)", "SetText(MinimapWnd.txtGameTime,PM 01 : 05)"]);
        api.calls.clear();
        script.on_event(&mut api, 1890, "GameHour=0 GameMinute=30");
        assert_eq!(api.calls, ["ShowWindow(MinimapWnd.texMoon)", "HideWindow(MinimapWnd.texSun)", "SetText(MinimapWnd.txtGameTime,AM 12 : 30)"]);
        api.calls.clear();
        script.show_game_time = false;
        script.on_event(&mut api, 1890, "GameHour=10 GameMinute=0");
        assert!(api.calls.is_empty());
    }

    #[test]
    fn expand_reduce_guide_and_close_buttons_follow_the_script() {
        let mut api = api();
        let mut script = Script::default();
        script.on_click_button(&mut api, "ExpandButton");
        assert!(script.expand_state);
        assert_eq!(api.calls, ["ShowWindow(MinimapWnd_Expand)", "SetFocus(MinimapWnd_Expand)", "HideWindow(MinimapWnd)"]);
        api.calls.clear();
        script.on_event(&mut api, 1780, "");
        assert_eq!(api.calls, ["HideWindow(MinimapWnd_Expand)"]);
        api.calls.clear();
        script.on_click_button(&mut api, "btnReduce");
        assert_eq!(api.calls, ["RequestReduceBtn(MinimapWnd.Minimap)", "HideWindow(MinimapWnd.Container.btnReduce)"]);
        api.calls.clear();
        script.on_click_button(&mut api, "OpenGuideWnd");
        script.on_click_button(&mut api, "OpenGuideWnd");
        script.on_click_button(&mut api, "BtnClose");
        assert_eq!(api.calls[0], "ShowWindow(GuideWnd)");
        assert_eq!(api.calls[1], "HideWindow(GuideWnd)");
        assert_eq!(api.calls[2], "HideWindow(MinimapWnd)");
        script.on_event(&mut api, 1870, "");
        script.on_event(&mut api, 1880, "");
        assert_eq!(api.calls[3], "ShowWindow(MinimapWnd.Container.btnReduce)");
        assert_eq!(api.calls[4], "HideWindow(MinimapWnd.Container.btnReduce)");
    }

    #[test]
    fn hide_remembers_the_guide_window_and_party_count_comes_from_event_520() {
        let mut api = api();
        api.shown.push(GUIDE_WND.into());
        let mut script = Script::default();
        script.on_hide(&mut api);
        assert_eq!(script.is_show_guide_wnd, 1);
        assert_eq!(api.calls, ["PlaySound(interfacesound.Interface.map_close_01)"]);
        script.on_event(&mut api, 520, "PartyMemberCount=5");
        assert_eq!(script.party_member_count, 5);
        script.on_event(&mut api, 2420, "");
        assert_eq!(api.calls.last().unwrap(), "SetText(Minimapwnd.txtVarCurLoc,Giran)");
    }
}
