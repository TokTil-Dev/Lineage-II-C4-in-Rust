use super::{UiApi, WindowScript, param_add};
use killer_ui::color::Argb;

pub const WINDOW: &str = "MenuWnd";
pub const TIMER_WELCOME: i32 = 1;
pub const TIMER_EXIT: i32 = 2;
pub const TIMER_WELCOME_MS: i32 = 1000;
pub const SOUND_CHARSTAT_OPEN: &str = "InterfaceSound.charstat_open_01";
pub const SOUND_CHARSTAT_CLOSE: &str = "InterfaceSound.charstat_close_01";
pub const SOUND_INVENTORY_OPEN: &str = "InterfaceSound.inventory_open_01";
pub const SOUND_INVENTORY_CLOSE: &str = "InterfaceSound.inventory_close_01";
pub const SOUND_SYSTEM_OPEN: &str = "InterfaceSound.system_open_01";
pub const SOUND_SYSTEM_CLOSE: &str = "InterfaceSound.system_close_01";
pub const WELCOME_INI_SECTION: &str = "url";
pub const WELCOME_INI_KEY: &str = "serveraddr";
pub const WELCOME_INI_FILE: &str = "l2.ini";
pub const WELCOME_SERVER: &str = "auth.la2interlude.com.br";
pub const WELCOME_SERVER_NAME: &str = "Honor";
pub const WELCOME_PREFIX: &str = "Welcome to Lineage 2 ";
pub const SCREEN_MESSAGE_EVENT: i32 = 140;
pub const NEOPHRON_OPTIONS: [(&str, &str); 3] = [("Neophron", "HoldTarget"), ("Neophron", "IgnoreAggr"), ("Neophron", "UseDoping")];
pub const NEOPHRON_CHECKS: [&str; 3] = ["NPHRN_Tab.Cb_HoldTarget", "NPHRN_Tab.Cb_IgnoreAggr", "NPHRN_Tab.Cb_UseDoping"];
pub const NEOPHRON_WINDOWS: [&str; 2] = ["NPHRN_MagicSkillWnd", "NPHRN_DopingWnd"];

pub fn msn_color(name: &str) -> Argb {
    let (r, g, b): (u32, u32, u32) = match name {
        "Yellow" => (255, 255, 0),
        "System" => (176, 155, 121),
        "Amber" => (218, 165, 32),
        "White" => (255, 255, 255),
        "Dim" => (177, 173, 172),
        "Magenta" => (255, 0, 255),
        _ => (0, 0, 0),
    };
    Argb(0xFF00_0000 | (r << 16) | (g << 8) | b)
}

pub fn screen_message_param(msg_type: i32, msg_no: i32, window_type: i32, font_type: i32, color: Argb, lifetime: i32, text: &str) -> String {
    let mut param = String::new();
    param_add(&mut param, "MsgType", &msg_type.to_string());
    param_add(&mut param, "MsgNo", &msg_no.to_string());
    param_add(&mut param, "WindowType", &window_type.to_string());
    param_add(&mut param, "FontSize", "10");
    param_add(&mut param, "FontType", &font_type.to_string());
    param_add(&mut param, "MsgColor", "1");
    param_add(&mut param, "MsgColorR", &((color.0 >> 16) & 0xFF).to_string());
    param_add(&mut param, "MsgColorG", &((color.0 >> 8) & 0xFF).to_string());
    param_add(&mut param, "MsgColorB", &(color.0 & 0xFF).to_string());
    param_add(&mut param, "ShadowType", "0");
    param_add(&mut param, "BackgroundType", "1");
    param_add(&mut param, "LifeTime", &lifetime.to_string());
    param_add(&mut param, "AnimationType", "0");
    param_add(&mut param, "Msg", text);
    param
}

pub fn reset_neophron_options(api: &mut dyn UiApi) {
    for (section, name) in NEOPHRON_OPTIONS {
        api.set_option_bool(section, name, false);
    }
    for check in NEOPHRON_CHECKS {
        api.checkbox_set_check(check, false);
    }
    for check in NEOPHRON_CHECKS {
        api.checkbox_set_disable(check, true);
    }
    for check in NEOPHRON_CHECKS {
        api.hide_window(check);
    }
    for window in NEOPHRON_WINDOWS {
        api.hide_window(window);
    }
}

#[derive(Debug, Default)]
pub struct Script {
    pub owner: String,
}

impl Script {
    pub fn new() -> Script {
        Script { owner: WINDOW.to_string() }
    }

    fn show_window_with_focus(api: &mut dyn UiApi, name: &str) {
        api.show_window(name);
        api.set_focus(name);
    }

    pub fn init_tooltips(&mut self, api: &mut dyn UiApi) {
        api.set_tooltip_simple_text("MenuWnd.BtnAction", "Actions (Alt+C)");
        api.set_tooltip_simple_text("MenuWnd.BtnSkill", "Skills & Magic (Alt+K)");
        api.set_tooltip_simple_text("MenuWnd.BtnQuest", "Quests (Alt+U)");
        api.set_tooltip_simple_text("MenuWnd.BtnClan", "Clan Action (Alt+N)");
        api.set_tooltip_simple_text("MenuWnd.BtnCharInfo", "Character Status (Alt+T)");
    }

    pub fn toggle_skill_wnd(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window("MagicSkillWnd") {
            api.hide_window("MagicSkillWnd");
            api.play_sound(SOUND_CHARSTAT_CLOSE);
        } else {
            Self::show_window_with_focus(api, "MagicSkillWnd");
            api.play_sound(SOUND_CHARSTAT_OPEN);
        }
    }

    pub fn toggle_action_wnd(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window("ActionWnd") {
            api.hide_window("ActionWnd");
            api.play_sound(SOUND_CHARSTAT_CLOSE);
        } else {
            Self::show_window_with_focus(api, "ActionWnd");
            api.execute_event(1300, "");
            api.execute_event(1310, "");
            api.execute_event(1900, "");
            api.play_sound(SOUND_CHARSTAT_OPEN);
        }
    }

    pub fn toggle_clan_wnd(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window("ClanWnd") {
            api.hide_window("ClanWnd");
            api.play_sound(SOUND_CHARSTAT_CLOSE);
        } else {
            Self::show_window_with_focus(api, "ClanWnd");
            api.play_sound(SOUND_CHARSTAT_OPEN);
        }
    }

    pub fn toggle_quest_wnd(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window("QuestTreeWnd") {
            api.hide_window("QuestTreeWnd");
            api.play_sound(SOUND_CHARSTAT_CLOSE);
        } else {
            Self::show_window_with_focus(api, "QuestTreeWnd");
            api.play_sound(SOUND_CHARSTAT_OPEN);
        }
    }

    pub fn toggle_char_info_wnd(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window("DetailStatusWnd") {
            api.hide_window("DetailStatusWnd");
            api.play_sound(SOUND_CHARSTAT_CLOSE);
        } else {
            api.show_window("DetailStatusWnd");
            api.play_sound(SOUND_CHARSTAT_OPEN);
        }
    }

    pub fn toggle_inventory_wnd(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window("InventoryWnd") {
            api.hide_window("InventoryWnd");
            api.play_sound(SOUND_INVENTORY_CLOSE);
        } else {
            Self::show_window_with_focus(api, "InventoryWnd");
            api.play_sound(SOUND_INVENTORY_OPEN);
        }
    }

    pub fn open_map(&mut self, api: &mut dyn UiApi) {
        api.request_open_minimap();
    }

    pub fn toggle_system_menu_wnd(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window("SystemMenuWnd") {
            api.hide_window("SystemMenuWnd");
            api.play_sound(SOUND_SYSTEM_CLOSE);
        } else {
            Self::show_window_with_focus(api, "SystemMenuWnd");
            api.play_sound(SOUND_SYSTEM_OPEN);
        }
    }

    pub fn welcome_message(&mut self, api: &mut dyn UiApi) {
        let address = api.get_ini_string(WELCOME_INI_SECTION, WELCOME_INI_KEY, WELCOME_INI_FILE).unwrap_or_default();
        if !address.eq_ignore_ascii_case(WELCOME_SERVER) {
            return;
        }
        let message = format!("{WELCOME_PREFIX}{WELCOME_SERVER_NAME}");
        api.add_system_message(&message, msn_color("System"));
        let param = screen_message_param(1, 2261, 2, 0, msn_color("White"), 5000, &message);
        api.execute_event(SCREEN_MESSAGE_EVENT, &param);
        reset_neophron_options(api);
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        self.owner = WINDOW.to_string();
        self.init_tooltips(api);
    }

    fn on_show(&mut self, api: &mut dyn UiApi) {
        api.set_timer(&self.owner, TIMER_WELCOME, TIMER_WELCOME_MS);
    }

    fn on_click_button(&mut self, api: &mut dyn UiApi, id: &str) {
        match id {
            "BtnCharInfo" => self.toggle_char_info_wnd(api),
            "BtnInventory" => self.toggle_inventory_wnd(api),
            "BtnMap" => self.open_map(api),
            "BtnSystemMenu" => self.toggle_system_menu_wnd(api),
            "BtnSkill" => self.toggle_skill_wnd(api),
            "BtnAction" => self.toggle_action_wnd(api),
            "BtnClan" => self.toggle_clan_wnd(api),
            "BtnQuest" => self.toggle_quest_wnd(api),
            _ => {}
        }
    }

    fn on_timer(&mut self, api: &mut dyn UiApi, timer_id: i32) {
        if timer_id == TIMER_WELCOME {
            api.kill_timer(&self.owner, TIMER_WELCOME);
            self.welcome_message(api);
        }
        if timer_id == TIMER_EXIT {
            api.request_exit();
            reset_neophron_options(api);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::RecordingApi;
    use super::*;

    #[test]
    fn on_load_sets_the_five_tooltips_and_on_show_arms_the_welcome_timer() {
        let mut api = RecordingApi::default();
        let mut script = Script::new();
        script.on_load(&mut api);
        assert_eq!(api.calls.len(), 5);
        assert_eq!(api.calls[0], "SetTooltipCustomType(MenuWnd.BtnAction,Actions (Alt+C))");
        assert_eq!(api.calls[4], "SetTooltipCustomType(MenuWnd.BtnCharInfo,Character Status (Alt+T))");
        api.calls.clear();
        script.on_show(&mut api);
        assert_eq!(api.calls, ["SetTimer(MenuWnd,1,1000)"]);
    }

    #[test]
    fn buttons_toggle_their_windows_with_focus_sounds_and_action_events() {
        let mut api = RecordingApi::default();
        let mut script = Script::new();
        script.on_click_button(&mut api, "BtnAction");
        assert_eq!(
            api.calls,
            [
                "ShowWindow(ActionWnd)",
                "SetFocus(ActionWnd)",
                "ExecuteEvent(1300,)",
                "ExecuteEvent(1310,)",
                "ExecuteEvent(1900,)",
                "PlaySound(InterfaceSound.charstat_open_01)"
            ]
        );
        api.calls.clear();
        script.on_click_button(&mut api, "BtnAction");
        assert_eq!(api.calls, ["HideWindow(ActionWnd)", "PlaySound(InterfaceSound.charstat_close_01)"]);
        api.calls.clear();
        script.on_click_button(&mut api, "BtnCharInfo");
        assert_eq!(api.calls, ["ShowWindow(DetailStatusWnd)", "PlaySound(InterfaceSound.charstat_open_01)"]);
        api.calls.clear();
        script.on_click_button(&mut api, "BtnInventory");
        assert_eq!(api.calls, ["ShowWindow(InventoryWnd)", "SetFocus(InventoryWnd)", "PlaySound(InterfaceSound.inventory_open_01)"]);
        api.calls.clear();
        script.on_click_button(&mut api, "BtnSystemMenu");
        assert_eq!(api.calls, ["ShowWindow(SystemMenuWnd)", "SetFocus(SystemMenuWnd)", "PlaySound(InterfaceSound.system_open_01)"]);
        api.calls.clear();
        script.on_click_button(&mut api, "BtnMap");
        assert_eq!(api.calls, ["RequestOpenMinimap()"]);
        api.calls.clear();
        script.on_click_button(&mut api, "BtnSkill");
        script.on_click_button(&mut api, "BtnClan");
        script.on_click_button(&mut api, "BtnQuest");
        assert_eq!(api.calls.len(), 9);
        assert_eq!(api.shown, ["DetailStatusWnd", "InventoryWnd", "SystemMenuWnd", "MagicSkillWnd", "ClanWnd", "QuestTreeWnd"]);
        api.calls.clear();
        script.on_click_button(&mut api, "Unknown");
        assert!(api.calls.is_empty());
    }

    #[test]
    fn the_welcome_timer_only_greets_the_hard_coded_server_and_resets_the_neophron_options() {
        let mut api = RecordingApi::default();
        let mut script = Script::new();
        script.on_timer(&mut api, TIMER_WELCOME);
        assert_eq!(api.calls, ["KillTimer(MenuWnd,1)"]);
        api.calls.clear();
        api.ini_strings.push((("url".into(), "serveraddr".into(), "l2.ini".into()), "AUTH.la2interlude.com.br".into()));
        script.on_timer(&mut api, TIMER_WELCOME);
        assert_eq!(api.calls[0], "KillTimer(MenuWnd,1)");
        assert_eq!(api.calls[1], "AddSystemMessage(Welcome to Lineage 2 Honor,FFB09B79)");
        let expected = screen_message_param(1, 2261, 2, 0, Argb(0xFFFFFFFF), 5000, "Welcome to Lineage 2 Honor");
        assert_eq!(api.calls[2], format!("ExecuteEvent(140,{expected})"));
        assert!(expected.starts_with("MsgType=\u{7f}1\u{7f} MsgNo=\u{7f}2261\u{7f} "));
        assert!(expected.contains(" MsgColorR=\u{7f}255\u{7f} MsgColorG=\u{7f}255\u{7f} MsgColorB=\u{7f}255\u{7f} "));
        assert_eq!(api.calls[3], "SetOptionBool(Neophron,HoldTarget,false)");
        assert_eq!(api.calls.len(), 3 + 3 + 3 + 3 + 3 + 2);
        assert_eq!(api.calls.last().map(String::as_str), Some("HideWindow(NPHRN_DopingWnd)"));
        api.calls.clear();
        script.on_timer(&mut api, TIMER_EXIT);
        assert_eq!(api.calls[0], "RequestExit()");
        assert_eq!(api.calls.len(), 1 + 14);
    }

    #[test]
    fn msn_colors_follow_the_table() {
        assert_eq!(msn_color("Yellow"), Argb(0xFFFFFF00));
        assert_eq!(msn_color("System"), Argb(0xFFB09B79));
        assert_eq!(msn_color("Amber"), Argb(0xFFDAA520));
        assert_eq!(msn_color("Dim"), Argb(0xFFB1ADAC));
        assert_eq!(msn_color("Magenta"), Argb(0xFFFF00FF));
        assert_eq!(msn_color("other"), Argb(0xFF000000));
    }
}
