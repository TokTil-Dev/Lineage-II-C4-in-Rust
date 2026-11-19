use super::{UiApi, WindowScript};

pub const EVENT_SYSTEM_MSG_OPTION: i32 = 150;
pub const SYSTEM_MSG_WND: &str = "SystemMsgWnd";
pub const CHAT_WND: &str = "ChatWnd";
pub const CHAT_FILTER_WND: &str = "ChatFilterWnd";
pub const FILTER_ANCHOR_OFFSET: (i32, i32) = (0, -5);
pub const EFFECT_BUTTONS: [(&str, i32); 3] = [("TutorialBtnWnd", 5), ("QuestBtnWnd", 42), ("MailBtnWnd", 79)];
pub const EFFECT_BUTTON_Y: i32 = -5;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Script;

impl Script {
    pub fn change_anchor_effect_button(&mut self, api: &mut dyn UiApi, id: &str) {
        for (window, x) in EFFECT_BUTTONS {
            api.set_anchor(window, id, "TopLeft", "BottomLeft", x, EFFECT_BUTTON_Y);
        }
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        api.register_event(EVENT_SYSTEM_MSG_OPTION);
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, _param: &str) {
        if event_id == EVENT_SYSTEM_MSG_OPTION {
            if api.get_option_bool("Game", "SystemMsgWnd") {
                api.show_window(SYSTEM_MSG_WND);
            } else {
                api.hide_window(SYSTEM_MSG_WND);
            }
        }
    }

    fn on_show(&mut self, api: &mut dyn UiApi) {
        api.set_anchor(CHAT_FILTER_WND, SYSTEM_MSG_WND, "TopLeft", "BottomLeft", FILTER_ANCHOR_OFFSET.0, FILTER_ANCHOR_OFFSET.1);
        self.change_anchor_effect_button(api, SYSTEM_MSG_WND);
    }

    fn on_hide(&mut self, api: &mut dyn UiApi) {
        api.set_anchor(CHAT_FILTER_WND, CHAT_WND, "TopLeft", "BottomLeft", FILTER_ANCHOR_OFFSET.0, FILTER_ANCHOR_OFFSET.1);
        self.change_anchor_effect_button(api, CHAT_WND);
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::RecordingApi;
    use super::*;

    #[test]
    fn the_window_registers_event_150_and_follows_the_system_msg_option() {
        let mut script = Script;
        let mut api = RecordingApi::default();
        script.on_load(&mut api);
        assert_eq!(api.calls, ["RegisterEvent(150)"]);
        script.on_event(&mut api, 150, "");
        assert!(api.calls.contains(&"HideWindow(SystemMsgWnd)".to_string()));
        api.options_bool.push((("Game".into(), "SystemMsgWnd".into()), true));
        script.on_event(&mut api, 150, "");
        assert!(api.calls.contains(&"ShowWindow(SystemMsgWnd)".to_string()));
        let before = api.calls.len();
        script.on_event(&mut api, 151, "");
        assert_eq!(api.calls.len(), before);
    }

    #[test]
    fn show_and_hide_move_the_filter_window_and_the_effect_buttons_between_the_two_windows() {
        let mut script = Script;
        let mut api = RecordingApi::default();
        script.on_show(&mut api);
        assert_eq!(
            api.calls,
            [
                "SetAnchor(ChatFilterWnd,SystemMsgWnd,TopLeft,BottomLeft,0,-5)",
                "SetAnchor(TutorialBtnWnd,SystemMsgWnd,TopLeft,BottomLeft,5,-5)",
                "SetAnchor(QuestBtnWnd,SystemMsgWnd,TopLeft,BottomLeft,42,-5)",
                "SetAnchor(MailBtnWnd,SystemMsgWnd,TopLeft,BottomLeft,79,-5)",
            ]
        );
        api.calls.clear();
        script.on_hide(&mut api);
        assert_eq!(api.calls[0], "SetAnchor(ChatFilterWnd,ChatWnd,TopLeft,BottomLeft,0,-5)");
        assert_eq!(api.calls[3], "SetAnchor(MailBtnWnd,ChatWnd,TopLeft,BottomLeft,79,-5)");
    }
}
