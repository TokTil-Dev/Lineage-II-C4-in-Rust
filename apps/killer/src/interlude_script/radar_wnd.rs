use super::{UiApi, WindowScript, parse_int};
use killer_ui::color::Argb;

pub const RADAR: &str = "RadarContainerWnd.Radar";
pub const MOVING_TEXT: &str = "movingtext";
pub const MOVING_TEXT_BOX: &str = "movingtext.textboxmove";
pub const EVENT_ZONE_CHANGED: i32 = 110;
pub const ICONS: [&str; 9] = [
    "RadarWnd.icon1",
    "RadarWnd.icon2",
    "RadarWnd.icon3",
    "RadarWnd.icon4",
    "RadarWnd.icon5",
    "RadarWnd.icon6",
    "RadarWnd.icon7",
    "RadarWnd.icon8",
    "RadarWnd.icon9",
];

pub fn color(r: u32, g: u32, b: u32) -> Argb {
    Argb((r << 16) | (g << 8) | b)
}

pub const RED: Argb = Argb(0x0032_0000);
pub const BLUE: Argb = Argb(0x0000_0032);
pub const GREY: Argb = Argb(0x001E_1E1E);
pub const ORANGE: Argb = Argb(0x003C_1E00);
pub const GREEN: Argb = Argb(0x0000_3200);

#[derive(Debug, Default)]
pub struct Script {
    pub onshowstat1: bool,
    pub onshowstat2: bool,
    pub global_alpha_value1: i32,
    pub global_y_loc: i32,
    pub number_strange: f32,
    pub global_move_val: i32,
}

impl Script {
    pub fn set_radar_color(&mut self, api: &mut dyn UiApi, radar_color: Argb, seconds: f32) {
        api.radar_set_color(RADAR, radar_color, seconds);
    }

    pub fn fade_in(&mut self, api: &mut dyn UiApi) {
        self.global_alpha_value1 += 3;
        if self.global_alpha_value1 < 255 {
            api.set_alpha(MOVING_TEXT, self.global_alpha_value1, 0.0);
        } else {
            api.set_alpha(MOVING_TEXT, 255, 0.0);
            self.onshowstat1 = true;
            self.onshowstat2 = false;
        }
    }

    pub fn fade_out(&mut self, api: &mut dyn UiApi) {
        self.global_alpha_value1 -= 2;
        self.global_y_loc += 1;
        if self.global_alpha_value1 > 1 {
            api.set_alpha(MOVING_TEXT, self.global_alpha_value1, 0.0);
        } else {
            api.set_alpha(MOVING_TEXT, 0, 0.0);
            api.hide_window(MOVING_TEXT);
            self.global_y_loc = 0;
            self.global_move_val = 0;
            api.debug(&format!("thisisrunningright:{}", self.global_y_loc / 2));
            self.global_alpha_value1 = 0;
            self.onshowstat1 = false;
            self.onshowstat2 = false;
        }
    }

    pub fn move_value(&mut self) -> i32 {
        self.number_strange += 0.5;
        if self.number_strange < 1.0 {
            return 0;
        }
        if self.number_strange == 1.0 {
            self.number_strange = 0.0;
            return -1;
        }
        0
    }

    pub fn resetanimloc(&mut self) {
        self.onshowstat1 = false;
        self.onshowstat2 = false;
        self.global_y_loc = 0;
        self.global_move_val = 0;
        self.global_alpha_value1 = 0;
    }

    pub fn hide_all_icons(&mut self, api: &mut dyn UiApi) {
        for icon in ICONS {
            api.hide_window(icon);
        }
        api.hide_window("RadarWnd");
    }

    pub fn init_textboxmove(&mut self, api: &mut dyn UiApi) {
        api.set_text(MOVING_TEXT_BOX, "");
    }

    fn zone_message(&mut self, api: &mut dyn UiApi, sysstring: i32, radar_color: Argb) {
        let text = api.get_system_string(sysstring);
        api.set_text(MOVING_TEXT_BOX, &text);
        self.set_radar_color(api, radar_color, 2.0);
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        api.register_event(EVENT_ZONE_CHANGED);
        self.onshowstat1 = false;
        self.onshowstat2 = false;
        self.global_alpha_value1 = 0;
        self.global_y_loc = 0;
        self.number_strange = 0.0;
        self.global_move_val = 0;
        api.hide_window(MOVING_TEXT);
        self.hide_all_icons(api);
        self.init_textboxmove(api);
    }

    fn on_show(&mut self, api: &mut dyn UiApi) {
        self.hide_all_icons(api);
    }

    fn on_tick(&mut self, api: &mut dyn UiApi) {
        if self.onshowstat2 {
            self.fade_in(api);
        }
        if self.onshowstat1 {
            self.fade_out(api);
        }
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        if event_id != EVENT_ZONE_CHANGED {
            return;
        }
        let zone_code = parse_int(param, "ZoneCode").unwrap_or(0);
        self.resetanimloc();
        self.onshowstat2 = true;
        api.set_alpha(MOVING_TEXT, 0, 0.0);
        api.show_window(MOVING_TEXT);
        match zone_code {
            15 => {
                self.hide_all_icons(api);
                self.zone_message(api, 1284, GREY);
            }
            12 => {
                self.hide_all_icons(api);
                self.zone_message(api, 1285, BLUE);
            }
            11 => {
                self.hide_all_icons(api);
                self.zone_message(api, 1286, ORANGE);
            }
            9 => {
                self.hide_all_icons(api);
                self.zone_message(api, 1287, RED);
            }
            8 => {
                self.hide_all_icons(api);
                self.zone_message(api, 1288, RED);
            }
            13 => {
                self.hide_all_icons(api);
                api.show_window("RadarWnd");
                api.show_window("RadarWnd.icon6");
                self.zone_message(api, 1289, GREY);
            }
            14 => {
                self.hide_all_icons(api);
                self.zone_message(api, 1290, GREEN);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interlude_script::mock::RecordingApi;

    #[test]
    fn on_load_registers_110_hides_the_moving_text_and_every_icon() {
        let mut api = RecordingApi::default();
        let mut script = Script::default();
        script.on_load(&mut api);
        assert_eq!(api.calls[0], "RegisterEvent(110)");
        assert_eq!(api.calls[1], "HideWindow(movingtext)");
        assert_eq!(api.calls[2], "HideWindow(RadarWnd.icon1)");
        assert_eq!(api.calls[11], "HideWindow(RadarWnd)");
        assert_eq!(api.calls[12], "SetText(movingtext.textboxmove,)");
        assert_eq!(api.calls.len(), 13);
    }

    #[test]
    fn zone_code_13_shows_the_radar_icon_6_with_sysstring_1289_and_grey_radar() {
        let mut api = RecordingApi { sysstrings: vec![(1289, "Siege".into())], ..Default::default() };
        let mut script = Script::default();
        script.on_event(&mut api, 110, "ZoneCode=13 ");
        assert!(script.onshowstat2);
        assert_eq!(api.calls[0], "SetAlpha(movingtext,0,0)");
        assert_eq!(api.calls[1], "ShowWindow(movingtext)");
        assert!(api.calls.contains(&"ShowWindow(RadarWnd.icon6)".to_string()));
        assert!(api.calls.contains(&"SetText(movingtext.textboxmove,Siege)".to_string()));
        assert_eq!(api.calls.last().unwrap(), "SetRadarColor(RadarContainerWnd.Radar,001E1E1E,2)");
    }

    #[test]
    fn zone_colors_follow_the_rgb_of_the_script() {
        assert_eq!(color(50, 0, 0), RED);
        assert_eq!(color(0, 0, 50), BLUE);
        assert_eq!(color(30, 30, 30), GREY);
        assert_eq!(color(60, 30, 0), ORANGE);
        assert_eq!(color(0, 50, 0), GREEN);
        let mut api = RecordingApi::default();
        let mut script = Script::default();
        script.on_event(&mut api, 110, "ZoneCode=9");
        assert_eq!(api.calls.last().unwrap(), "SetRadarColor(RadarContainerWnd.Radar,00320000,2)");
        script.on_event(&mut api, 110, "ZoneCode=7");
        assert_eq!(api.calls.last().unwrap(), "ShowWindow(movingtext)");
    }

    #[test]
    fn tick_fades_in_by_3_until_255_then_fades_out_by_2_and_hides() {
        let mut api = RecordingApi::default();
        let mut script = Script::default();
        script.on_event(&mut api, 110, "ZoneCode=15");
        api.calls.clear();
        script.on_tick(&mut api);
        assert_eq!(api.calls, ["SetAlpha(movingtext,3,0)"]);
        for _ in 0..84 {
            script.on_tick(&mut api);
        }
        assert_eq!(api.calls[api.calls.len() - 2], "SetAlpha(movingtext,255,0)");
        assert_eq!(api.calls.last().unwrap(), "SetAlpha(movingtext,253,0)");
        assert!(script.onshowstat1 && !script.onshowstat2);
        api.calls.clear();
        for _ in 0..126 {
            script.on_tick(&mut api);
        }
        assert_eq!(api.calls[0], "SetAlpha(movingtext,251,0)");
        assert_eq!(api.calls[api.calls.len() - 2], "SetAlpha(movingtext,0,0)");
        assert_eq!(api.calls.last().unwrap(), "HideWindow(movingtext)");
        assert!(!script.onshowstat1 && !script.onshowstat2 && script.global_alpha_value1 == 0);
    }

    #[test]
    fn move_value_steps_by_half_and_returns_minus_one_when_it_reaches_one() {
        let mut script = Script::default();
        assert_eq!(script.move_value(), 0);
        assert_eq!(script.move_value(), -1);
        assert_eq!(script.number_strange, 0.0);
    }
}
