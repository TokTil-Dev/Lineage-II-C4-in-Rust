use super::{UiApi, WindowScript, parse_int};

pub const RADAR: &str = "RadarContainterWnd.Radar";
pub const EVENT_RADAR_ADD_TARGET: i32 = 1730;
pub const EVENT_RADAR_DELETE_TARGET: i32 = 1740;
pub const EVENT_RADAR_DELETE_ALL_TARGET: i32 = 1750;

#[derive(Debug, Default)]
pub struct Script;

impl Script {
    pub fn handle_radar_add_target(&mut self, api: &mut dyn UiApi, param: &str) {
        if let (Some(x), Some(y), Some(z)) = (parse_int(param, "X"), parse_int(param, "Y"), parse_int(param, "Z")) {
            api.radar_add_target(RADAR, x, y, z);
        }
    }

    pub fn handle_radar_delete_target(&mut self, api: &mut dyn UiApi, param: &str) {
        if let (Some(x), Some(y), Some(z)) = (parse_int(param, "X"), parse_int(param, "Y"), parse_int(param, "Z")) {
            api.radar_delete_target(RADAR, x, y, z);
        }
    }

    pub fn handle_radar_delete_all_target(&mut self, api: &mut dyn UiApi) {
        api.radar_delete_all_target(RADAR);
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        api.register_event(EVENT_RADAR_ADD_TARGET);
        api.register_event(EVENT_RADAR_DELETE_TARGET);
        api.register_event(EVENT_RADAR_DELETE_ALL_TARGET);
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        match event_id {
            EVENT_RADAR_ADD_TARGET => self.handle_radar_add_target(api, param),
            EVENT_RADAR_DELETE_TARGET => self.handle_radar_delete_target(api, param),
            EVENT_RADAR_DELETE_ALL_TARGET => self.handle_radar_delete_all_target(api),
            _ => {}
        }
    }

    fn on_click_button(&mut self, api: &mut dyn UiApi, id: &str) {
        if id == "BtnSupport" {
            if api.is_show_window("Some") {
                api.hide_window("Some");
            } else {
                api.show_window("Some");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interlude_script::mock::RecordingApi;

    #[test]
    fn on_load_registers_the_three_radar_events() {
        let mut api = RecordingApi::default();
        Script.on_load(&mut api);
        assert_eq!(api.calls, ["RegisterEvent(1730)", "RegisterEvent(1740)", "RegisterEvent(1750)"]);
    }

    #[test]
    fn radar_events_parse_xyz_and_use_the_misspelled_control_name_of_the_script() {
        let mut api = RecordingApi::default();
        let mut script = Script;
        script.on_event(&mut api, 1730, "X=10 Y=-20 Z=30 ");
        script.on_event(&mut api, 1740, "X=1 Y=2 Z=3");
        script.on_event(&mut api, 1740, "X=1 Y=2");
        script.on_event(&mut api, 1750, "");
        assert_eq!(
            api.calls,
            [
                "RadarAddTarget(RadarContainterWnd.Radar,10,-20,30)",
                "RadarDeleteTarget(RadarContainterWnd.Radar,1,2,3)",
                "RadarDeleteAllTarget(RadarContainterWnd.Radar)"
            ]
        );
    }

    #[test]
    fn btn_support_toggles_the_some_window() {
        let mut api = RecordingApi::default();
        let mut script = Script;
        script.on_click_button(&mut api, "BtnSupport");
        script.on_click_button(&mut api, "BtnSupport");
        script.on_click_button(&mut api, "Other");
        assert_eq!(api.calls, ["ShowWindow(Some)", "HideWindow(Some)"]);
    }
}
