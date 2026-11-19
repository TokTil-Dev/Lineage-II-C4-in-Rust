use super::{UiApi, WindowScript, parse_int};

pub const MAX_SHORTCUT_PER_PAGE: i32 = 12;
pub const MAX_PAGE: i32 = 10;
pub const EVENT_SHORTCUT_UPDATE: i32 = 630;
pub const EVENT_SHORTCUT_PAGE_UPDATE: i32 = 640;
pub const EVENT_SHORTCUT_CLEAR: i32 = 650;
pub const EVENT_SHORTCUT_JOYPAD: i32 = 660;
pub const EVENT_JOYPAD_LBUTTON_DOWN: i32 = 590;
pub const EVENT_JOYPAD_LBUTTON_UP: i32 = 600;
pub const EVENT_JOYPAD_RBUTTON_DOWN: i32 = 610;
pub const EVENT_JOYPAD_RBUTTON_UP: i32 = 620;
pub const VERTICAL: &str = "ShortcutWndVertical";
pub const HORIZONTAL: &str = "ShortcutWndHorizontal";
pub const JOYPAD: &str = "ShortcutWndJoypad";
pub const JOYPAD_EXPAND: &str = "ShortcutWndJoypadExpand";
pub const TIMER_INI_CHECK: i32 = 1;
pub const TIMER_INI_TICK: i32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JoyShortcut {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct Script {
    pub current_shortcut_page: i32,
    pub current_shortcut_page2: i32,
    pub current_shortcut_page3: i32,
    pub current_shortcut_page4: i32,
    pub current_shortcut_page5: i32,
    pub is_locked: bool,
    pub is_vertical: bool,
    pub is_joypad: bool,
    pub is_joypad_expand: bool,
    pub is_joypad_on: bool,
    pub is_expand1: bool,
    pub is_expand2: bool,
    pub is_expand3: bool,
    pub is_expand4: bool,
    pub is_shortcut_expand: bool,
    pub shortcut_wnd_name: String,
}

impl Default for Script {
    fn default() -> Self {
        Script {
            current_shortcut_page: 0,
            current_shortcut_page2: 0,
            current_shortcut_page3: 0,
            current_shortcut_page4: 0,
            current_shortcut_page5: 0,
            is_locked: false,
            is_vertical: true,
            is_joypad: false,
            is_joypad_expand: false,
            is_joypad_on: false,
            is_expand1: false,
            is_expand2: false,
            is_expand3: false,
            is_expand4: false,
            is_shortcut_expand: false,
            shortcut_wnd_name: String::new(),
        }
    }
}

fn page_out_of_range(page: i32) -> bool {
    !(0..MAX_PAGE).contains(&page)
}

impl Script {
    fn wnd(&self, suffix: &str) -> String {
        format!("ShortcutWnd.{}{suffix}", self.shortcut_wnd_name)
    }

    fn nested(&self, depth: usize, suffix: &str) -> String {
        let name = &self.shortcut_wnd_name;
        match depth {
            2 => format!("ShortcutWnd.{name}.{name}_1{suffix}"),
            3 => format!("ShortcutWnd.{name}.{name}_1.{name}_2{suffix}"),
            4 => format!("ShortcutWnd.{name}.{name}_1.{name}_2.{name}_3{suffix}"),
            _ => format!("ShortcutWnd.{name}.{name}_1.{name}_4{suffix}"),
        }
    }

    pub fn init_short_page_num(&mut self) {
        self.current_shortcut_page = 0;
        self.current_shortcut_page2 = 1;
        self.current_shortcut_page3 = 2;
        self.current_shortcut_page4 = 3;
        self.current_shortcut_page5 = 4;
    }

    pub fn handle_shortcut_page_update(&mut self, api: &mut dyn UiApi, param: &str) {
        let Some(shortcut_page) = parse_int(param, "ShortcutPage") else { return };
        if page_out_of_range(shortcut_page) {
            return;
        }
        self.current_shortcut_page = shortcut_page;
        api.set_text(&self.wnd(".PageNumTextBox"), &(self.current_shortcut_page + 1).to_string());
        for (i, shortcut_id) in (0..MAX_SHORTCUT_PER_PAGE).zip(self.current_shortcut_page * MAX_SHORTCUT_PER_PAGE..) {
            api.update_shortcut(&self.wnd(&format!(".Shortcut{}", i + 1)), shortcut_id);
        }
    }

    pub fn handle_shortcut_update(&mut self, api: &mut dyn UiApi, param: &str) {
        let shortcut_id = parse_int(param, "ShortcutID").unwrap_or(0);
        let shortcut_num = (shortcut_id as f32 % MAX_SHORTCUT_PER_PAGE as f32) as i32 + 1;
        if self.is_shortcut_id_in_cur_page(self.current_shortcut_page, shortcut_id) {
            api.update_shortcut(&self.wnd(&format!(".Shortcut{shortcut_num}")), shortcut_id);
        }
        if self.is_shortcut_id_in_cur_page(self.current_shortcut_page2, shortcut_id) {
            api.update_shortcut(&self.wnd(&format!("_1.Shortcut{shortcut_num}")), shortcut_id);
        }
        if self.is_shortcut_id_in_cur_page(self.current_shortcut_page3, shortcut_id) {
            api.update_shortcut(&self.wnd(&format!("_2.Shortcut{shortcut_num}")), shortcut_id);
        }
        if self.is_shortcut_id_in_cur_page(self.current_shortcut_page4, shortcut_id) {
            api.update_shortcut(&self.wnd(&format!("_3.Shortcut{shortcut_num}")), shortcut_id);
        }
        if self.is_shortcut_id_in_cur_page(self.current_shortcut_page5, shortcut_id) {
            api.update_shortcut(&self.wnd(&format!("_4.Shortcut{shortcut_num}")), shortcut_id);
        }
    }

    pub fn handle_shortcut_clear(&mut self, api: &mut dyn UiApi) {
        for i in 0..MAX_SHORTCUT_PER_PAGE {
            for window in [
                "ShortcutWndVertical",
                "ShortcutWndVertical_1",
                "ShortcutWndVertical_2",
                "ShortcutWndVertical_3",
                "ShortcutWndHorizontal",
                "ShortcutWndHorizontal_1",
                "ShortcutWndHorizontal_2",
                "ShortcutWndHorizontal_3",
                "ShortcutWndHorizontal_4",
                "ShortcutWndJoypadExpand",
            ] {
                api.clear(&format!("ShortcutWnd.{window}.Shortcut{}", i + 1));
            }
        }
        for i in 0..4 {
            api.clear(&format!("ShortcutWnd.ShortcutWndJoypad.Shortcut{}", i + 1));
        }
    }

    pub fn handle_shortcut_joypad(&mut self, api: &mut dyn UiApi, param: &str) {
        let Some(on_off) = parse_int(param, "OnOff") else { return };
        if on_off == 1 {
            self.is_joypad_on = true;
            api.show_window(&self.wnd(".JoypadBtn"));
        } else if on_off == 0 {
            self.is_joypad_on = false;
            api.hide_window(&self.wnd(".JoypadBtn"));
        }
    }

    pub fn handle_joypad_lbutton_up(&mut self, api: &mut dyn UiApi) {
        self.set_joypad_shortcut(api, JoyShortcut::Center);
    }

    pub fn handle_joypad_lbutton_down(&mut self, api: &mut dyn UiApi) {
        self.set_joypad_shortcut(api, JoyShortcut::Left);
    }

    pub fn handle_joypad_rbutton_up(&mut self, api: &mut dyn UiApi) {
        self.set_joypad_shortcut(api, JoyShortcut::Center);
    }

    pub fn handle_joypad_rbutton_down(&mut self, api: &mut dyn UiApi) {
        self.set_joypad_shortcut(api, JoyShortcut::Right);
    }

    pub fn set_joypad_shortcut(&mut self, api: &mut dyn UiApi, joy_shortcut: JoyShortcut) {
        let back = "ShortcutWnd.ShortcutWndJoypadExpand.JoypadButtonBackTex";
        let left = "ShortcutWnd.ShortcutWndJoypad.JoypadLButtonTex";
        let right = "ShortcutWnd.ShortcutWndJoypad.JoypadRButtonTex";
        let (back_tex, back_x, left_tex, right_tex, offset) = match joy_shortcut {
            JoyShortcut::Left => ("L2UI_CH3.ShortcutWnd.joypad2_back_over1", 28, "L2UI_ch3.Joypad.joypad_L_HOLD", "L2UI_ch3.Joypad.joypad_R", 4),
            JoyShortcut::Center => ("L2UI_CH3.ShortcutWnd.joypad2_back_over2", 158, "L2UI_ch3.Joypad.joypad_L", "L2UI_ch3.Joypad.joypad_R", 0),
            JoyShortcut::Right => ("L2UI_CH3.ShortcutWnd.joypad2_back_over3", 288, "L2UI_ch3.Joypad.joypad_L", "L2UI_ch3.Joypad.joypad_R_HOLD", 8),
        };
        api.set_texture(back, back_tex);
        api.set_anchor(back, "ShortcutWnd.ShortcutWndJoypadExpand", "TopLeft", "TopLeft", back_x, 0);
        api.set_texture(left, left_tex);
        api.set_texture(right, right_tex);
        for (i, shortcut_id) in (0..4).zip(self.current_shortcut_page * MAX_SHORTCUT_PER_PAGE + offset..) {
            api.update_shortcut(&format!("ShortcutWnd.ShortcutWndJoypad.Shortcut{}", i + 1), shortcut_id);
        }
    }

    pub fn on_btn_key(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window("NPHRN_KeyWnd") {
            api.hide_window("NPHRN_KeyWnd");
            api.play_sound("InterfaceSound.Charstat_Close_01");
        } else {
            api.show_window("NPHRN_KeyWnd");
            api.play_sound("InterfaceSound.Charstat_Open_01");
        }
    }

    fn prev_page(current: i32) -> i32 {
        let new_page = current - 1;
        if 0 > new_page { MAX_PAGE - 1 } else { new_page }
    }

    fn next_page(current: i32) -> i32 {
        let new_page = current + 1;
        if MAX_PAGE <= new_page { 0 } else { new_page }
    }

    pub fn on_prev_btn(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page(api, Self::prev_page(self.current_shortcut_page));
    }

    pub fn on_prev_btn2(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page2(api, Self::prev_page(self.current_shortcut_page2));
    }

    pub fn on_prev_btn3(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page3(api, Self::prev_page(self.current_shortcut_page3));
    }

    pub fn on_prev_btn4(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page4(api, Self::prev_page(self.current_shortcut_page4));
    }

    pub fn on_prev_btn5(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page5(api, Self::prev_page(self.current_shortcut_page5));
    }

    pub fn on_next_btn(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page(api, Self::next_page(self.current_shortcut_page));
    }

    pub fn on_next_btn2(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page2(api, Self::next_page(self.current_shortcut_page2));
    }

    pub fn on_next_btn3(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page3(api, Self::next_page(self.current_shortcut_page3));
    }

    pub fn on_next_btn4(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page4(api, Self::next_page(self.current_shortcut_page4));
    }

    pub fn on_next_btn5(&mut self, api: &mut dyn UiApi) {
        self.set_cur_page5(api, Self::next_page(self.current_shortcut_page5));
    }

    pub fn on_click_lock_btn(&mut self, api: &mut dyn UiApi) {
        self.un_lock(api);
    }

    pub fn on_click_unlock_btn(&mut self, api: &mut dyn UiApi) {
        self.lock(api);
    }

    pub fn on_rotate_btn(&mut self, api: &mut dyn UiApi) {
        self.set_vertical(api, !self.is_vertical);
        if self.is_vertical {
            api.set_anchor("ShortcutWnd.ShortcutWndVertical", "ShortcutWnd.ShortcutWndHorizontal", "BottomRight", "BottomRight", 0, 0);
            api.clear_anchor("ShortcutWnd.ShortcutWndVertical");
            api.set_anchor("ShortcutWnd.ShortcutWndHorizontal", "ShortcutWnd.ShortcutWndVertical", "BottomRight", "BottomRight", 0, 0);
        } else {
            api.set_anchor("ShortcutWnd.ShortcutWndHorizontal", "ShortcutWnd.ShortcutWndVertical", "BottomRight", "BottomRight", 0, 0);
            api.clear_anchor("ShortcutWnd.ShortcutWndHorizontal");
            api.set_anchor("ShortcutWnd.ShortcutWndVertical", "ShortcutWnd.ShortcutWndHorizontal", "BottomRight", "BottomRight", 0, 0);
        }
        if self.is_expand4 {
            self.expand1(api);
            self.expand2(api);
            self.expand3(api);
            self.expand4(api);
        } else if self.is_expand3 {
            self.expand1(api);
            self.expand2(api);
            self.expand3(api);
        } else if self.is_expand2 {
            self.expand1(api);
            self.expand2(api);
        } else if self.is_expand1 {
            self.expand1(api);
        }
        api.set_focus(&self.wnd(""));
        self.auto_potions_wnd_position(api);
    }

    pub fn on_joypad_btn(&mut self, api: &mut dyn UiApi) {
        self.set_joypad(api, !self.is_joypad);
        api.set_focus(&self.wnd(""));
    }

    pub fn on_expand_btn(&mut self, api: &mut dyn UiApi) {
        self.set_joypad_expand(api, !self.is_joypad_expand);
        api.set_focus(&self.wnd(""));
    }

    pub fn set_cur_page(&mut self, api: &mut dyn UiApi, cur_page: i32) {
        if page_out_of_range(cur_page) {
            return;
        }
        api.set_shortcut_page(cur_page);
    }

    fn set_nested_page(&mut self, api: &mut dyn UiApi, depth: usize, cur_page: i32) {
        api.set_text(&self.nested(depth, ".PageNumTextBox"), &(cur_page + 1).to_string());
        for (i, shortcut_id) in (0..MAX_SHORTCUT_PER_PAGE).zip(cur_page * MAX_SHORTCUT_PER_PAGE..) {
            api.update_shortcut(&self.nested(depth, &format!(".Shortcut{}", i + 1)), shortcut_id);
        }
    }

    pub fn set_cur_page2(&mut self, api: &mut dyn UiApi, cur_page: i32) {
        if page_out_of_range(cur_page) {
            return;
        }
        self.current_shortcut_page2 = cur_page;
        self.set_nested_page(api, 2, cur_page);
    }

    pub fn set_cur_page3(&mut self, api: &mut dyn UiApi, cur_page: i32) {
        if page_out_of_range(cur_page) {
            return;
        }
        self.current_shortcut_page3 = cur_page;
        self.set_nested_page(api, 3, cur_page);
    }

    pub fn set_cur_page4(&mut self, api: &mut dyn UiApi, cur_page: i32) {
        if page_out_of_range(cur_page) {
            return;
        }
        self.current_shortcut_page4 = cur_page;
        self.set_nested_page(api, 4, cur_page);
    }

    pub fn set_cur_page5(&mut self, api: &mut dyn UiApi, cur_page: i32) {
        if page_out_of_range(cur_page) {
            return;
        }
        self.current_shortcut_page5 = cur_page;
        self.set_nested_page(api, 5, cur_page);
    }

    pub fn is_shortcut_id_in_cur_page(&self, page_num: i32, shortcut_id: i32) -> bool {
        if page_num * MAX_SHORTCUT_PER_PAGE > shortcut_id {
            return false;
        }
        if (page_num + 1) * MAX_SHORTCUT_PER_PAGE <= shortcut_id {
            return false;
        }
        true
    }

    pub fn lock(&mut self, api: &mut dyn UiApi) {
        self.is_locked = true;
        api.set_option_bool("Game", "IsLockShortcutWnd", true);
        if api.is_show_window("ShortcutWnd") {
            api.show_window(&self.wnd(".LockBtn"));
            api.hide_window(&self.wnd(".UnlockBtn"));
        }
    }

    pub fn un_lock(&mut self, api: &mut dyn UiApi) {
        self.is_locked = false;
        api.set_option_bool("Game", "IsLockShortcutWnd", false);
        if api.is_show_window("ShortcutWnd") {
            api.show_window(&self.wnd(".UnlockBtn"));
            api.hide_window(&self.wnd(".LockBtn"));
        }
    }

    pub fn set_vertical(&mut self, api: &mut dyn UiApi, is_vertical: bool) {
        self.is_vertical = is_vertical;
        api.set_option_bool("Game", "IsShortcutWndVertical", self.is_vertical);
        self.arrange_wnd(api);
        self.expand_wnd(api);
    }

    pub fn set_joypad(&mut self, api: &mut dyn UiApi, is_joypad: bool) {
        self.is_joypad = is_joypad;
        self.arrange_wnd(api);
    }

    pub fn set_joypad_expand(&mut self, api: &mut dyn UiApi, is_joypad_expand: bool) {
        self.is_joypad_expand = is_joypad_expand;
        if self.is_joypad_expand {
            api.set_anchor("ShortcutWnd.ShortcutWndJoypadExpand", "ShortcutWnd.ShortcutWndJoypad", "TopLeft", "TopLeft", 0, 0);
            api.clear_anchor("ShortcutWnd.ShortcutWndJoypadExpand");
        } else {
            api.set_anchor("ShortcutWnd.ShortcutWndJoypad", "ShortcutWnd.ShortcutWndJoypadExpand", "TopLeft", "TopLeft", 0, 0);
            api.clear_anchor("ShortcutWnd.ShortcutWndJoypad");
        }
        self.arrange_wnd(api);
    }

    pub fn arrange_wnd(&mut self, api: &mut dyn UiApi) {
        if self.is_joypad {
            api.hide_window("ShortcutWnd.ShortcutWndVertical");
            api.hide_window("ShortcutWnd.ShortcutWndHorizontal");
            if self.is_joypad_expand {
                api.hide_window("ShortcutWnd.ShortcutWndJoypad");
                api.show_window("ShortcutWnd.ShortcutWndJoypadExpand");
                self.shortcut_wnd_name = JOYPAD_EXPAND.to_string();
            } else {
                api.hide_window("ShortcutWnd.ShortcutWndJoypadExpand");
                api.show_window("ShortcutWnd.ShortcutWndJoypad");
                self.shortcut_wnd_name = JOYPAD.to_string();
            }
        } else {
            api.hide_window("ShortcutWnd.ShortcutWndJoypadExpand");
            api.hide_window("ShortcutWnd.ShortcutWndJoypad");
            if self.is_vertical {
                self.shortcut_wnd_name = VERTICAL.to_string();
                let rect = api.get_rect("ShortcutWnd.ShortcutWndVertical");
                if rect.y < 0 {
                    api.move_to("ShortcutWnd.ShortcutWndVertical", rect.x, 0);
                }
                api.hide_window("ShortcutWnd.ShortcutWndHorizontal");
                api.show_window("ShortcutWnd.ShortcutWndVertical");
            } else {
                self.shortcut_wnd_name = HORIZONTAL.to_string();
                let rect = api.get_rect("ShortcutWnd.ShortcutWndHorizontal");
                if rect.x < 0 {
                    api.move_to("ShortcutWnd.ShortcutWndHorizontal", 0, rect.y);
                }
                api.hide_window("ShortcutWnd.ShortcutWndVertical");
                api.show_window("ShortcutWnd.ShortcutWndHorizontal");
            }
            if self.is_joypad_on {
                api.show_window(&self.wnd(".JoypadBtn"));
            } else {
                api.hide_window(&self.wnd(".JoypadBtn"));
            }
        }
        if self.is_locked {
            self.lock(api);
        } else {
            self.un_lock(api);
        }
        self.set_cur_page(api, self.current_shortcut_page);
        self.set_cur_page2(api, self.current_shortcut_page2);
        self.set_cur_page3(api, self.current_shortcut_page3);
        self.set_cur_page4(api, self.current_shortcut_page4);
        self.set_cur_page5(api, self.current_shortcut_page5);
        self.is_shortcut_expand = self.is_expand1 || self.is_expand2 || !(self.is_expand3 || self.is_expand4);
        self.handle_expand_button(api);
    }

    pub fn expand_wnd(&mut self, api: &mut dyn UiApi) {
        if self.is_expand1 || self.is_expand2 || self.is_expand3 || self.is_expand4 {
            if self.is_expand4 {
                self.is_shortcut_expand = false;
                self.expand4(api);
            }
            if self.is_expand3 {
                self.is_shortcut_expand = false;
                self.expand3(api);
            }
            if self.is_expand2 {
                self.is_shortcut_expand = false;
                self.expand2(api);
            }
            if self.is_expand1 {
                self.is_shortcut_expand = false;
                self.expand1(api);
            }
        } else {
            self.is_shortcut_expand = true;
            self.reduce(api);
        }
    }

    fn expand(&mut self, api: &mut dyn UiApi, level: i32) {
        self.is_shortcut_expand = true;
        api.set_option_bool("Game", &format!("Is{level}ExpandShortcutWnd"), true);
        api.show_window(&format!("ShortcutWnd.ShortcutWndVertical_{level}"));
        api.show_window(&format!("ShortcutWnd.ShortcutWndHorizontal_{level}"));
        self.handle_expand_button(api);
    }

    pub fn expand1(&mut self, api: &mut dyn UiApi) {
        self.is_expand1 = true;
        self.expand(api, 1);
    }

    pub fn expand2(&mut self, api: &mut dyn UiApi) {
        self.is_expand2 = true;
        self.expand(api, 2);
    }

    pub fn expand3(&mut self, api: &mut dyn UiApi) {
        self.is_expand3 = true;
        self.expand(api, 3);
    }

    pub fn expand4(&mut self, api: &mut dyn UiApi) {
        self.is_expand4 = true;
        self.expand(api, 4);
    }

    pub fn reduce(&mut self, api: &mut dyn UiApi) {
        self.is_shortcut_expand = true;
        self.is_expand1 = false;
        self.is_expand2 = false;
        self.is_expand3 = false;
        self.is_expand4 = false;
        api.set_option_bool("Game", "Is1ExpandShortcutWnd", false);
        api.set_option_bool("Game", "Is2ExpandShortcutWnd", false);
        api.set_option_bool("Game", "Is3ExpandShortcutWnd", false);
        api.set_option_bool("Game", "Is4ExpandShortcutWnd", false);
        for level in 1..=4 {
            api.hide_window(&format!("ShortcutWnd.ShortcutWndVertical_{level}"));
        }
        for level in 1..=4 {
            api.hide_window(&format!("ShortcutWnd.ShortcutWndHorizontal_{level}"));
        }
        self.handle_expand_button(api);
    }

    pub fn on_click_expand_shortcut_button(&mut self, api: &mut dyn UiApi) {
        if self.is_expand4 {
            self.reduce(api);
        } else if self.is_expand3 {
            self.expand4(api);
        } else if self.is_expand2 {
            self.expand3(api);
        } else if self.is_expand1 {
            self.expand2(api);
        } else {
            self.expand1(api);
        }
        self.auto_potions_wnd_position(api);
    }

    pub fn handle_expand_button(&mut self, api: &mut dyn UiApi) {
        if self.is_shortcut_expand {
            api.show_window(&self.wnd(".ExpandButton"));
            api.hide_window(&self.wnd(".ReduceButton"));
        } else {
            api.hide_window(&self.wnd(".ExpandButton"));
            api.show_window(&self.wnd(".ReduceButton"));
        }
    }

    pub fn auto_potions_wnd_position(&mut self, api: &mut dyn UiApi) {
        if !self.is_vertical {
            for suffix in ["_4", "_3", "_2", "_1", ""] {
                if api.is_show_window(&format!("ShortcutWndHorizontal{suffix}")) {
                    api.set_anchor("NPHRN_PotionsWnd", &format!("ShortcutWnd.ShortcutWndHorizontal{suffix}"), "TopRight", "TopRight", -1, -29);
                    return;
                }
            }
        } else {
            api.set_anchor("NPHRN_PotionsWnd", "MenuWnd", "TopCenter", "TopCenter", 8, -29);
        }
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        api.set_tooltip_simple_text("ShortcutWnd.BtnKey", "Bind Setting");
        for id in [
            EVENT_SHORTCUT_UPDATE,
            EVENT_SHORTCUT_PAGE_UPDATE,
            EVENT_SHORTCUT_JOYPAD,
            EVENT_SHORTCUT_CLEAR,
            EVENT_JOYPAD_LBUTTON_DOWN,
            EVENT_JOYPAD_LBUTTON_UP,
            EVENT_JOYPAD_RBUTTON_DOWN,
            EVENT_JOYPAD_RBUTTON_UP,
        ] {
            api.register_event(id);
        }
        self.is_locked = api.get_option_bool("Game", "IsLockShortcutWnd");
        self.is_expand1 = api.get_option_bool("Game", "Is1ExpandShortcutWnd");
        self.is_expand2 = api.get_option_bool("Game", "Is2ExpandShortcutWnd");
        self.is_expand3 = api.get_option_bool("Game", "Is3ExpandShortcutWnd");
        self.is_expand4 = api.get_option_bool("Game", "Is4ExpandShortcutWnd");
        self.is_vertical = api.get_option_bool("Game", "IsShortcutWndVertical");
        self.init_short_page_num();
    }

    fn on_default_position(&mut self, api: &mut dyn UiApi) {
        self.is_expand1 = false;
        self.is_expand2 = false;
        self.is_expand3 = false;
        self.is_expand4 = false;
        self.set_vertical(api, true);
        self.init_short_page_num();
        self.arrange_wnd(api);
        self.expand_wnd(api);
    }

    fn on_enter_state(&mut self, api: &mut dyn UiApi, previous: &str) {
        self.arrange_wnd(api);
        self.expand_wnd(api);
        self.auto_potions_wnd_position(api);
        if previous == "LoadingState" {
            self.init_short_page_num();
        }
        api.set_timer("ShortcutWnd", TIMER_INI_CHECK, 1_200_000);
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        match event_id {
            EVENT_SHORTCUT_PAGE_UPDATE => self.handle_shortcut_page_update(api, param),
            EVENT_SHORTCUT_JOYPAD => self.handle_shortcut_joypad(api, param),
            EVENT_JOYPAD_LBUTTON_DOWN => self.handle_joypad_lbutton_down(api),
            EVENT_JOYPAD_LBUTTON_UP => self.handle_joypad_lbutton_up(api),
            EVENT_JOYPAD_RBUTTON_DOWN => self.handle_joypad_rbutton_down(api),
            EVENT_JOYPAD_RBUTTON_UP => self.handle_joypad_rbutton_up(api),
            EVENT_SHORTCUT_UPDATE => self.handle_shortcut_update(api, param),
            EVENT_SHORTCUT_CLEAR => {
                self.handle_shortcut_clear(api);
                self.arrange_wnd(api);
                self.expand_wnd(api);
            }
            _ => {}
        }
    }

    fn on_timer(&mut self, api: &mut dyn UiApi, timer_id: i32) {
        match timer_id {
            TIMER_INI_CHECK => api.set_timer("ShortcutWnd", TIMER_INI_TICK, 1000),
            TIMER_INI_TICK => api.set_timer("ShortcutWnd", TIMER_INI_TICK, 1000),
            _ => {}
        }
    }

    fn on_click_button(&mut self, api: &mut dyn UiApi, id: &str) {
        match id {
            "PrevBtn" => self.on_prev_btn(api),
            "NextBtn" => self.on_next_btn(api),
            "PrevBtn2" => self.on_prev_btn2(api),
            "NextBtn2" => self.on_next_btn2(api),
            "PrevBtn3" => self.on_prev_btn3(api),
            "NextBtn3" => self.on_next_btn3(api),
            "PrevBtn4" => self.on_prev_btn4(api),
            "NextBtn4" => self.on_next_btn4(api),
            "LockBtn" => self.on_click_lock_btn(api),
            "UnlockBtn" => self.on_click_unlock_btn(api),
            "RotateBtn" => self.on_rotate_btn(api),
            "JoypadBtn" => self.on_joypad_btn(api),
            "ExpandBtn" => self.on_expand_btn(api),
            "ExpandButton" | "ReduceButton" => self.on_click_expand_shortcut_button(api),
            "BtnKey" => self.on_btn_key(api),
            "PrevBtn5" => self.on_prev_btn5(api),
            "NextBtn5" => self.on_next_btn5(api),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interlude_script::mock::RecordingApi;

    fn loaded(vertical: bool) -> (RecordingApi, Script) {
        let mut api = RecordingApi::default();
        api.options_bool.push((("Game".into(), "IsShortcutWndVertical".into()), vertical));
        api.shown.push("ShortcutWnd".into());
        let mut script = Script::default();
        script.on_load(&mut api);
        api.calls.clear();
        (api, script)
    }

    #[test]
    fn on_load_registers_the_eight_events_reads_the_options_and_resets_the_pages() {
        let mut api = RecordingApi::default();
        api.options_bool.push((("Game".into(), "IsLockShortcutWnd".into()), true));
        api.options_bool.push((("Game".into(), "Is2ExpandShortcutWnd".into()), true));
        let mut script = Script::default();
        script.on_load(&mut api);
        assert!(api.calls[0].contains("ShortcutWnd.BtnKey,Bind Setting"), "{}", api.calls[0]);
        let ids: Vec<String> = [630, 640, 660, 650, 590, 600, 610, 620].iter().map(|id| format!("RegisterEvent({id})")).collect();
        assert_eq!(&api.calls[1..], ids);
        assert!(script.is_locked && script.is_expand2 && !script.is_expand1 && !script.is_vertical);
        assert_eq!(
            (script.current_shortcut_page, script.current_shortcut_page2, script.current_shortcut_page3, script.current_shortcut_page4, script.current_shortcut_page5),
            (0, 1, 2, 3, 4)
        );
    }

    #[test]
    fn page_update_sets_the_page_text_and_the_twelve_slots_of_the_current_window() {
        let (mut api, mut script) = loaded(true);
        script.shortcut_wnd_name = VERTICAL.into();
        script.on_event(&mut api, 640, "ShortcutPage=3 ");
        assert_eq!(script.current_shortcut_page, 3);
        assert_eq!(api.calls[0], "SetText(ShortcutWnd.ShortcutWndVertical.PageNumTextBox,4)");
        assert_eq!(api.calls[1], "UpdateShortcut(ShortcutWnd.ShortcutWndVertical.Shortcut1,36)");
        assert_eq!(api.calls[12], "UpdateShortcut(ShortcutWnd.ShortcutWndVertical.Shortcut12,47)");
        assert_eq!(api.calls.len(), 13);
        api.calls.clear();
        script.on_event(&mut api, 640, "ShortcutPage=10");
        assert!(api.calls.is_empty());
    }

    #[test]
    fn shortcut_update_refreshes_every_page_window_that_contains_the_id() {
        let (mut api, mut script) = loaded(true);
        script.shortcut_wnd_name = VERTICAL.into();
        script.current_shortcut_page = 1;
        script.current_shortcut_page2 = 1;
        script.current_shortcut_page3 = 0;
        script.on_event(&mut api, 630, "ShortcutID=14");
        assert_eq!(
            api.calls,
            ["UpdateShortcut(ShortcutWnd.ShortcutWndVertical.Shortcut3,14)", "UpdateShortcut(ShortcutWnd.ShortcutWndVertical_1.Shortcut3,14)"]
        );
        assert!(script.is_shortcut_id_in_cur_page(1, 12) && script.is_shortcut_id_in_cur_page(1, 23));
        assert!(!script.is_shortcut_id_in_cur_page(1, 11) && !script.is_shortcut_id_in_cur_page(1, 24));
    }

    #[test]
    fn clear_wipes_ten_windows_of_twelve_slots_and_four_joypad_slots() {
        let (mut api, mut script) = loaded(true);
        script.handle_shortcut_clear(&mut api);
        assert_eq!(api.calls.len(), 10 * 12 + 4);
        assert_eq!(api.calls[0], "Clear(ShortcutWnd.ShortcutWndVertical.Shortcut1)");
        assert_eq!(api.calls[9], "Clear(ShortcutWnd.ShortcutWndJoypadExpand.Shortcut1)");
        assert_eq!(api.calls[123], "Clear(ShortcutWnd.ShortcutWndJoypad.Shortcut4)");
    }

    #[test]
    fn joypad_buttons_select_the_four_slot_group_and_textures() {
        let (mut api, mut script) = loaded(true);
        script.current_shortcut_page = 2;
        script.on_event(&mut api, 610, "");
        assert_eq!(api.calls[0], "SetTexture(ShortcutWnd.ShortcutWndJoypadExpand.JoypadButtonBackTex,L2UI_CH3.ShortcutWnd.joypad2_back_over3)");
        assert_eq!(api.calls[1], "SetAnchor(ShortcutWnd.ShortcutWndJoypadExpand.JoypadButtonBackTex,ShortcutWnd.ShortcutWndJoypadExpand,TopLeft,TopLeft,288,0)");
        assert_eq!(api.calls[3], "SetTexture(ShortcutWnd.ShortcutWndJoypad.JoypadRButtonTex,L2UI_ch3.Joypad.joypad_R_HOLD)");
        assert_eq!(api.calls[4], "UpdateShortcut(ShortcutWnd.ShortcutWndJoypad.Shortcut1,32)");
        assert_eq!(api.calls[7], "UpdateShortcut(ShortcutWnd.ShortcutWndJoypad.Shortcut4,35)");
        api.calls.clear();
        script.on_event(&mut api, 590, "");
        assert_eq!(api.calls[4], "UpdateShortcut(ShortcutWnd.ShortcutWndJoypad.Shortcut1,28)");
        api.calls.clear();
        script.on_event(&mut api, 600, "");
        assert_eq!(api.calls[4], "UpdateShortcut(ShortcutWnd.ShortcutWndJoypad.Shortcut1,24)");
        api.calls.clear();
        script.shortcut_wnd_name = VERTICAL.into();
        script.on_event(&mut api, 660, "OnOff=1");
        assert!(script.is_joypad_on);
        assert_eq!(api.calls, ["ShowWindow(ShortcutWnd.ShortcutWndVertical.JoypadBtn)"]);
        script.on_event(&mut api, 660, "OnOff=0");
        assert!(!script.is_joypad_on);
    }

    #[test]
    fn prev_and_next_wrap_around_ten_pages() {
        let (mut api, mut script) = loaded(true);
        script.on_click_button(&mut api, "PrevBtn");
        assert_eq!(api.calls, ["SetShortcutPage(9)"]);
        script.current_shortcut_page = 9;
        api.calls.clear();
        script.on_click_button(&mut api, "NextBtn");
        assert_eq!(api.calls, ["SetShortcutPage(0)"]);
        script.shortcut_wnd_name = HORIZONTAL.into();
        api.calls.clear();
        script.on_click_button(&mut api, "NextBtn2");
        assert_eq!(script.current_shortcut_page2, 2);
        assert_eq!(api.calls[0], "SetText(ShortcutWnd.ShortcutWndHorizontal.ShortcutWndHorizontal_1.PageNumTextBox,3)");
        assert_eq!(api.calls[1], "UpdateShortcut(ShortcutWnd.ShortcutWndHorizontal.ShortcutWndHorizontal_1.Shortcut1,24)");
        api.calls.clear();
        script.on_click_button(&mut api, "PrevBtn5");
        assert_eq!(script.current_shortcut_page5, 3);
        assert_eq!(api.calls[0], "SetText(ShortcutWnd.ShortcutWndHorizontal.ShortcutWndHorizontal_1.ShortcutWndHorizontal_4.PageNumTextBox,4)");
        api.calls.clear();
        script.on_click_button(&mut api, "NextBtn3");
        assert_eq!(api.calls[0], "SetText(ShortcutWnd.ShortcutWndHorizontal.ShortcutWndHorizontal_1.ShortcutWndHorizontal_2.PageNumTextBox,4)");
        api.calls.clear();
        script.on_click_button(&mut api, "NextBtn4");
        assert_eq!(api.calls[0], "SetText(ShortcutWnd.ShortcutWndHorizontal.ShortcutWndHorizontal_1.ShortcutWndHorizontal_2.ShortcutWndHorizontal_3.PageNumTextBox,5)");
    }

    #[test]
    fn arrange_wnd_vertical_moves_the_bar_up_shows_lock_state_and_pages() {
        let (mut api, mut script) = loaded(true);
        api.rects.push(("ShortcutWnd.ShortcutWndVertical".into(), super::super::Rect { x: 5, y: -3, width: 46, height: 504 }));
        script.arrange_wnd(&mut api);
        assert_eq!(script.shortcut_wnd_name, VERTICAL);
        assert_eq!(&api.calls[..7], [
            "HideWindow(ShortcutWnd.ShortcutWndJoypadExpand)",
            "HideWindow(ShortcutWnd.ShortcutWndJoypad)",
            "MoveTo(ShortcutWnd.ShortcutWndVertical,5,0)",
            "HideWindow(ShortcutWnd.ShortcutWndHorizontal)",
            "ShowWindow(ShortcutWnd.ShortcutWndVertical)",
            "HideWindow(ShortcutWnd.ShortcutWndVertical.JoypadBtn)",
            "SetOptionBool(Game,IsLockShortcutWnd,false)"
        ]);
        assert!(api.calls.contains(&"ShowWindow(ShortcutWnd.ShortcutWndVertical.UnlockBtn)".to_string()));
        assert!(api.calls.contains(&"SetShortcutPage(0)".to_string()));
        assert!(script.is_shortcut_expand);
        assert_eq!(api.calls[api.calls.len() - 2], "ShowWindow(ShortcutWnd.ShortcutWndVertical.ExpandButton)");
        assert_eq!(api.calls.last().unwrap(), "HideWindow(ShortcutWnd.ShortcutWndVertical.ReduceButton)");
    }

    #[test]
    fn joypad_mode_hides_both_bars_and_picks_the_expand_variant() {
        let (mut api, mut script) = loaded(false);
        script.on_click_button(&mut api, "JoypadBtn");
        assert!(script.is_joypad);
        assert_eq!(script.shortcut_wnd_name, JOYPAD);
        assert_eq!(&api.calls[..4], [
            "HideWindow(ShortcutWnd.ShortcutWndVertical)",
            "HideWindow(ShortcutWnd.ShortcutWndHorizontal)",
            "HideWindow(ShortcutWnd.ShortcutWndJoypadExpand)",
            "ShowWindow(ShortcutWnd.ShortcutWndJoypad)"
        ]);
        assert_eq!(api.calls.last().unwrap(), "SetFocus(ShortcutWnd.ShortcutWndJoypad)");
        api.calls.clear();
        script.on_click_button(&mut api, "ExpandBtn");
        assert!(script.is_joypad_expand);
        assert_eq!(api.calls[0], "SetAnchor(ShortcutWnd.ShortcutWndJoypadExpand,ShortcutWnd.ShortcutWndJoypad,TopLeft,TopLeft,0,0)");
        assert_eq!(api.calls[1], "ClearAnchor(ShortcutWnd.ShortcutWndJoypadExpand)");
        assert_eq!(script.shortcut_wnd_name, JOYPAD_EXPAND);
    }

    #[test]
    fn expand_button_steps_through_four_levels_then_reduces() {
        let (mut api, mut script) = loaded(false);
        script.shortcut_wnd_name = HORIZONTAL.into();
        api.shown.push("ShortcutWndHorizontal_1".into());
        script.on_click_button(&mut api, "ExpandButton");
        assert!(script.is_expand1 && script.is_shortcut_expand);
        assert_eq!(&api.calls[..3], [
            "SetOptionBool(Game,Is1ExpandShortcutWnd,true)",
            "ShowWindow(ShortcutWnd.ShortcutWndVertical_1)",
            "ShowWindow(ShortcutWnd.ShortcutWndHorizontal_1)"
        ]);
        assert_eq!(api.calls.last().unwrap(), "SetAnchor(NPHRN_PotionsWnd,ShortcutWnd.ShortcutWndHorizontal_1,TopRight,TopRight,-1,-29)");
        script.on_click_button(&mut api, "ExpandButton");
        script.on_click_button(&mut api, "ExpandButton");
        script.on_click_button(&mut api, "ReduceButton");
        assert!(script.is_expand4);
        api.calls.clear();
        script.on_click_button(&mut api, "ExpandButton");
        assert!(!script.is_expand1 && !script.is_expand4 && script.is_shortcut_expand);
        assert_eq!(api.calls[0], "SetOptionBool(Game,Is1ExpandShortcutWnd,false)");
        assert_eq!(api.calls[4], "HideWindow(ShortcutWnd.ShortcutWndVertical_1)");
        assert_eq!(api.calls[11], "HideWindow(ShortcutWnd.ShortcutWndHorizontal_4)");
    }

    #[test]
    fn rotate_swaps_the_anchor_between_bars_and_re_expands_the_saved_levels() {
        let (mut api, mut script) = loaded(true);
        script.is_expand2 = true;
        script.on_click_button(&mut api, "RotateBtn");
        assert!(!script.is_vertical);
        assert_eq!(api.calls[0], "SetOptionBool(Game,IsShortcutWndVertical,false)");
        let anchors: Vec<&String> = api.calls.iter().filter(|c| c.starts_with("SetAnchor(ShortcutWnd") || c.starts_with("ClearAnchor")).collect();
        assert_eq!(anchors[0], "SetAnchor(ShortcutWnd.ShortcutWndHorizontal,ShortcutWnd.ShortcutWndVertical,BottomRight,BottomRight,0,0)");
        assert_eq!(anchors[1], "ClearAnchor(ShortcutWnd.ShortcutWndHorizontal)");
        assert_eq!(anchors[2], "SetAnchor(ShortcutWnd.ShortcutWndVertical,ShortcutWnd.ShortcutWndHorizontal,BottomRight,BottomRight,0,0)");
        assert!(script.is_expand1 && script.is_expand2 && !script.is_expand3);
        api.calls.clear();
        script.on_click_button(&mut api, "RotateBtn");
        assert!(script.is_vertical);
        assert_eq!(api.calls.last().unwrap(), "SetAnchor(NPHRN_PotionsWnd,MenuWnd,TopCenter,TopCenter,8,-29)");
    }

    #[test]
    fn lock_buttons_swap_the_option_and_the_visible_button() {
        let (mut api, mut script) = loaded(true);
        script.shortcut_wnd_name = VERTICAL.into();
        script.on_click_button(&mut api, "UnlockBtn");
        assert!(script.is_locked);
        assert_eq!(api.calls, [
            "SetOptionBool(Game,IsLockShortcutWnd,true)",
            "ShowWindow(ShortcutWnd.ShortcutWndVertical.LockBtn)",
            "HideWindow(ShortcutWnd.ShortcutWndVertical.UnlockBtn)"
        ]);
        api.calls.clear();
        script.on_click_button(&mut api, "LockBtn");
        assert!(!script.is_locked);
        assert_eq!(api.calls[1], "ShowWindow(ShortcutWnd.ShortcutWndVertical.UnlockBtn)");
    }

    #[test]
    fn btn_key_toggles_the_key_window_with_sounds_and_timers_follow_the_script() {
        let (mut api, mut script) = loaded(true);
        script.on_click_button(&mut api, "BtnKey");
        assert_eq!(api.calls, ["ShowWindow(NPHRN_KeyWnd)", "PlaySound(InterfaceSound.Charstat_Open_01)"]);
        api.calls.clear();
        script.on_click_button(&mut api, "BtnKey");
        assert_eq!(api.calls, ["HideWindow(NPHRN_KeyWnd)", "PlaySound(InterfaceSound.Charstat_Close_01)"]);
        api.calls.clear();
        script.on_enter_state(&mut api, "LoadingState");
        assert_eq!(api.calls.last().unwrap(), "SetTimer(ShortcutWnd,1,1200000)");
        api.calls.clear();
        script.on_timer(&mut api, 1);
        assert_eq!(api.calls, ["SetTimer(ShortcutWnd,2,1000)"]);
    }

    #[test]
    fn default_position_resets_the_expands_and_goes_vertical() {
        let (mut api, mut script) = loaded(false);
        script.is_expand3 = true;
        script.on_default_position(&mut api);
        assert!(script.is_vertical && !script.is_expand3 && !script.is_expand1);
        assert_eq!(api.calls[0], "SetOptionBool(Game,IsShortcutWndVertical,true)");
        assert!(api.calls.contains(&"SetOptionBool(Game,Is3ExpandShortcutWnd,false)".to_string()));
    }
}
