use super::{NameCtrlType, TextAlign, UiApi, WindowScript, parse_float, parse_int};

pub const WINDOW: &str = "StatusWnd";
pub const EVENTS: [i32; 8] = [70, 180, 190, 200, 210, 220, 230, 240];
pub const CP_BAR: &str = "StatusWnd.CPBar";
pub const HP_BAR: &str = "StatusWnd.HPBar";
pub const MP_BAR: &str = "StatusWnd.MPBar";
pub const EXP_BAR: &str = "StatusWnd.EXPBar";
pub const USER_NAME: &str = "StatusWnd.UserName";
pub const LEVEL_TEXT: &str = "StatusWnd.StatusWnd_LevelTextBox";
pub const TARGET_BUMPER: &str = "TargetStatusWnd.NPHRN_Bumper";

#[derive(Debug, Default)]
pub struct Script {
    pub user_id: i32,
    pub received_user_info: bool,
}

impl Script {
    pub fn update_user_info(&mut self, api: &mut dyn UiApi) {
        if let Some(info) = api.get_player_info() {
            self.received_user_info = true;
            self.user_id = info.id;
            api.set_point(CP_BAR, info.cur_cp, info.max_cp);
            api.set_point(HP_BAR, info.cur_hp, info.max_hp);
            api.set_point(MP_BAR, info.cur_mp, info.max_mp);
            api.set_point_exp(EXP_BAR, info.cur_exp, info.level);
            api.set_name(USER_NAME, &info.name, NameCtrlType::Normal, TextAlign::Left);
            api.set_int(LEVEL_TEXT, info.level);
        }
    }

    pub fn on_l_button_down(&mut self, api: &mut dyn UiApi, x: i32, _y: i32) {
        let rect = api.get_rect(WINDOW);
        if x > rect.x + 13 && x < (rect.x + rect.width) - 10 {
            api.request_self_target();
            api.hide_window(TARGET_BUMPER);
        }
    }

    pub fn handle_update_info(&mut self, api: &mut dyn UiApi, param: &str) {
        let server_id = parse_int(param, "ServerID").unwrap_or(0);
        if self.user_id == server_id || !self.received_user_info {
            self.update_user_info(api);
        }
    }

    pub fn handle_regen_status(&mut self, api: &mut dyn UiApi, param: &str) {
        let kind = parse_int(param, "Type").unwrap_or(0);
        if kind == 1 {
            let duration = parse_int(param, "Duration").unwrap_or(0);
            let ticks = parse_int(param, "Ticks").unwrap_or(0);
            let amount = parse_float(param, "Amount").unwrap_or(0.0);
            api.set_regen_info(HP_BAR, duration, ticks, amount);
        }
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        for event in EVENTS {
            api.register_event(event);
        }
    }

    fn on_enter_state(&mut self, api: &mut dyn UiApi, _previous: &str) {
        self.received_user_info = false;
        self.update_user_info(api);
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        match event_id {
            180 => self.update_user_info(api),
            200 | 210 | 220 | 230 | 240 => self.handle_update_info(api, param),
            70 => self.handle_regen_status(api, param),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::RecordingApi;
    use super::super::{Rect, UserInfo};
    use super::*;

    fn player() -> UserInfo {
        UserInfo { id: 7, name: "Killer".into(), level: 52, cur_hp: 400, max_hp: 900, cur_mp: 200, max_mp: 300, cur_cp: 100, max_cp: 100, cur_exp: 123, ..Default::default() }
    }

    #[test]
    fn on_load_registers_the_eight_events_and_enter_state_fills_the_bars() {
        let mut api = RecordingApi { player: Some(player()), ..Default::default() };
        let mut script = Script::default();
        script.on_load(&mut api);
        assert_eq!(api.calls, EVENTS.map(|id| format!("RegisterEvent({id})")));
        api.calls.clear();
        script.on_enter_state(&mut api, "LoadingState");
        assert_eq!(
            api.calls,
            [
                "SetPoint(StatusWnd.CPBar,100,100)",
                "SetPoint(StatusWnd.HPBar,400,900)",
                "SetPoint(StatusWnd.MPBar,200,300)",
                "SetPointExp(StatusWnd.EXPBar,123,52)",
                "SetName(StatusWnd.UserName,Killer,Normal,Left)",
                "SetInt(StatusWnd.StatusWnd_LevelTextBox,52)",
            ]
        );
        assert!(script.received_user_info);
        assert_eq!(script.user_id, 7);
    }

    #[test]
    fn status_events_only_refresh_for_the_own_server_id_or_before_the_first_user_info() {
        let mut api = RecordingApi { player: Some(player()), ..Default::default() };
        let mut script = Script::default();
        script.on_event(&mut api, 200, "ServerID=99 ");
        assert_eq!(api.calls.len(), 6);
        api.calls.clear();
        script.on_event(&mut api, 210, "ServerID=99 ");
        assert!(api.calls.is_empty());
        script.on_event(&mut api, 240, "ServerID=7 ");
        assert_eq!(api.calls.len(), 6);
        api.calls.clear();
        script.on_event(&mut api, 190, "ServerID=7 ");
        assert!(api.calls.is_empty());
    }

    #[test]
    fn regen_event_type_one_feeds_the_hp_bar_and_a_click_inside_the_bar_targets_self() {
        let mut api = RecordingApi { rects: vec![("StatusWnd".into(), Rect { x: 0, y: 0, width: 176, height: 84 })], ..Default::default() };
        let mut script = Script::default();
        script.on_event(&mut api, 70, "Type=1 Duration=30 Ticks=3 Amount=2.5 ");
        assert_eq!(api.calls, ["SetRegenInfo(StatusWnd.HPBar,30,3,2.5)"]);
        api.calls.clear();
        script.on_event(&mut api, 70, "Type=2 Duration=30 Ticks=3 Amount=2.5 ");
        assert!(api.calls.is_empty());
        script.on_l_button_down(&mut api, 13, 5);
        assert!(api.calls.is_empty());
        script.on_l_button_down(&mut api, 14, 5);
        assert_eq!(api.calls, ["RequestSelfTarget", "HideWindow(TargetStatusWnd.NPHRN_Bumper)"]);
        api.calls.clear();
        script.on_l_button_down(&mut api, 166, 5);
        assert!(api.calls.is_empty());
    }
}
