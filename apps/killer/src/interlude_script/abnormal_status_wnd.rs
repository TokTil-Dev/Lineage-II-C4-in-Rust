use super::ui_common_api::{is_debuff_custom, is_noblesse_custom, is_song_dance_custom};
use super::{StatusIconInfo, UiApi, WindowScript, parse_int, parse_string};

pub const WINDOW: &str = "AbnormalStatusWnd";
pub const STATUS_ICON: &str = "AbnormalStatusWnd.StatusIcon";
pub const EVENTS: [i32; 7] = [950, 960, 970, 40, 50, 310, 1900];
pub const DEBUFF_BACK: &str = "L2UI_CH3.Debuff";
pub const ABNORMAL_BACK: &str = "L2UI.EtcWndBack.AbnormalBack";
pub const STATUS_ICON_MAX_COL: i32 = 12;
pub const STATUS_ICON_FRAME_SIZE: i32 = 12;
pub const OPTION_SECTION: &str = "Neophron";
pub const BUFF_SIZES: [i32; 3] = [24, 20, 16];

#[derive(Debug)]
pub struct Script {
    pub option_section: String,
    pub row: i32,
    pub song_row: i32,
    pub debuff_row: i32,
    pub etc_row: i32,
    pub short_row: i32,
    pub on_cur_state: bool,
}

impl Default for Script {
    fn default() -> Self {
        Script { option_section: String::new(), row: -1, song_row: -1, debuff_row: -1, etc_row: -1, short_row: -1, on_cur_state: false }
    }
}

fn read_icon(param: &str, i: i32, info: &mut StatusIconInfo) {
    if let Some(value) = parse_int(param, &format!("SkillID_{i}")) {
        info.class_id = value;
    }
    if let Some(value) = parse_int(param, &format!("SkillLevel_{i}")) {
        info.level = value;
    }
    if let Some(value) = parse_int(param, &format!("RemainTime_{i}")) {
        info.remain_time = value;
    }
    if let Some(value) = parse_string(param, &format!("Name_{i}")) {
        info.name = value;
    }
    if let Some(value) = parse_string(param, &format!("IconName_{i}")) {
        info.icon_name = value;
    }
    if let Some(value) = parse_string(param, &format!("Description_{i}")) {
        info.description = value;
    }
}

impl Script {
    pub fn init(&mut self) {
        self.option_section = OPTION_SECTION.to_string();
    }

    pub fn get_buff_size(index: i32) -> i32 {
        match index {
            0 => BUFF_SIZES[0],
            1 => BUFF_SIZES[1],
            2 => BUFF_SIZES[2],
            _ => 0,
        }
    }

    fn icon_size(&self, api: &dyn UiApi) -> i32 {
        Self::get_buff_size(api.get_option_int(&self.option_section, "AbnormalSize"))
    }

    pub fn handle_restart(&mut self, api: &mut dyn UiApi) {
        self.clear_all(api);
    }

    pub fn handle_die(&mut self, api: &mut dyn UiApi) {
        self.clear_status(api, false, false);
        self.clear_status(api, false, true);
    }

    pub fn handle_show_replay_quit_dialog_box(&mut self, api: &mut dyn UiApi) {
        api.hide_window(WINDOW);
    }

    pub fn clear_status(&mut self, api: &mut dyn UiApi, etc_item: bool, short_item: bool) {
        if !etc_item && !short_item {
            self.row = -1;
            self.song_row = -1;
            self.debuff_row = -1;
        }
        if etc_item && !short_item {
            self.etc_row = -1;
        }
        if !etc_item && short_item {
            self.short_row = -1;
        }
        let mut row_count = api.status_icon_row_count(STATUS_ICON);
        let mut i = 0;
        while i < row_count {
            let mut col_count = api.status_icon_col_count(STATUS_ICON, i);
            let mut j = 0;
            while j < col_count {
                let info = api.status_icon_get_item(STATUS_ICON, i, j).unwrap_or_default();
                if info.class_id > 0 && info.etc_item == etc_item && info.short_item == short_item {
                    api.status_icon_del_item(STATUS_ICON, i, j);
                    j -= 1;
                    col_count -= 1;
                    let new_row_count = api.status_icon_row_count(STATUS_ICON);
                    if new_row_count != row_count {
                        i -= 1;
                        row_count -= 1;
                    }
                }
                j += 1;
            }
            i += 1;
        }
    }

    pub fn clear_all(&mut self, api: &mut dyn UiApi) {
        self.clear_status(api, false, false);
        self.clear_status(api, true, false);
        self.clear_status(api, false, true);
    }

    pub fn handle_add_normal_status(&mut self, api: &mut dyn UiApi, param: &str) {
        self.clear_status(api, false, false);
        let mut info = StatusIconInfo { size: self.icon_size(api), show: true, etc_item: false, short_item: false, ..Default::default() };
        let mut buff_count = 0;
        let max = parse_int(param, "Max").unwrap_or(0);
        for i in 0..max {
            read_icon(param, i, &mut info);
            if info.class_id > 0 {
                if is_song_dance_custom(info.class_id) {
                    info.back_tex = ABNORMAL_BACK.to_string();
                    if self.song_row == -1 {
                        self.song_row = self.row + 1;
                        api.status_icon_insert_row(STATUS_ICON, self.song_row);
                        if self.debuff_row > -1 {
                            self.debuff_row += 1;
                        }
                    }
                    api.status_icon_add_col(STATUS_ICON, self.song_row, &info);
                } else if is_debuff_custom(info.class_id) {
                    info.back_tex = if is_noblesse_custom(info.class_id) { ABNORMAL_BACK } else { DEBUFF_BACK }.to_string();
                    self.insert_debuff_row(api);
                    api.status_icon_add_col(STATUS_ICON, self.debuff_row, &info);
                } else {
                    info.back_tex = ABNORMAL_BACK.to_string();
                    if buff_count % STATUS_ICON_MAX_COL == 0 {
                        self.row += 1;
                        api.status_icon_insert_row(STATUS_ICON, self.row);
                        if self.song_row > -1 {
                            self.song_row += 1;
                        }
                        if self.debuff_row > -1 {
                            self.debuff_row += 1;
                        }
                    }
                    api.status_icon_add_col(STATUS_ICON, self.row, &info);
                    buff_count += 1;
                }
            }
        }
        if self.etc_row > -1 {
            self.etc_row = self.row + 1;
            if self.song_row > -1 {
                self.etc_row = self.song_row + 1;
            }
            if self.debuff_row > -1 {
                self.etc_row = self.debuff_row + 1;
            }
        }
        if self.short_row > -1 {
            self.short_row = self.row + 1;
            if self.song_row > -1 {
                self.short_row = self.song_row + 1;
            }
            if self.debuff_row > -1 {
                self.short_row = self.debuff_row + 1;
            }
        }
        self.update_window_size(api);
    }

    pub fn insert_debuff_row(&mut self, api: &mut dyn UiApi) {
        if self.debuff_row == -1 && self.song_row == -1 {
            self.debuff_row = self.row + 1;
            api.status_icon_insert_row(STATUS_ICON, self.debuff_row);
        } else if self.debuff_row == -1 && self.song_row > -1 {
            self.debuff_row = self.song_row + 1;
            api.status_icon_insert_row(STATUS_ICON, self.debuff_row);
        }
    }

    fn extra_row_start(&self, existing: i32) -> (bool, i32) {
        if existing > -1 {
            return (false, existing);
        }
        let mut row = self.row;
        if self.song_row > -1 {
            row = self.song_row;
        }
        if self.debuff_row > -1 {
            row = self.debuff_row;
        }
        (true, row)
    }

    pub fn handle_add_etc_status(&mut self, api: &mut dyn UiApi, param: &str) {
        self.clear_status(api, true, false);
        let mut info = StatusIconInfo {
            size: self.icon_size(api),
            back_tex: ABNORMAL_BACK.to_string(),
            show: true,
            etc_item: true,
            short_item: false,
            ..Default::default()
        };
        let (mut new_row, mut row) = self.extra_row_start(self.short_row);
        let mut count = 0;
        let max = parse_int(param, "Max").unwrap_or(0);
        for i in 0..max {
            read_icon(param, i, &mut info);
            if info.class_id > 0 {
                if new_row {
                    new_row = !new_row;
                    row += 1;
                    api.status_icon_insert_row(STATUS_ICON, row);
                }
                api.status_icon_add_col(STATUS_ICON, row, &info);
                self.etc_row = row;
                count += 1;
            }
        }
        let _ = count;
        self.update_window_size(api);
    }

    pub fn handle_add_short_status(&mut self, api: &mut dyn UiApi, param: &str) {
        self.clear_status(api, false, true);
        let mut info = StatusIconInfo {
            size: self.icon_size(api),
            back_tex: ABNORMAL_BACK.to_string(),
            show: true,
            etc_item: false,
            short_item: true,
            ..Default::default()
        };
        let mut col = -1;
        let (mut new_row, mut row) = self.extra_row_start(self.etc_row);
        let mut count = 0;
        let max = parse_int(param, "Max").unwrap_or(0);
        for i in 0..max {
            read_icon(param, i, &mut info);
            if info.class_id > 0 {
                if new_row {
                    new_row = !new_row;
                    row += 1;
                    api.status_icon_insert_row(STATUS_ICON, row);
                }
                col += 1;
                api.status_icon_insert_col(STATUS_ICON, row, col, &info);
                self.short_row = row;
                count += 1;
            }
        }
        let _ = count;
        self.update_window_size(api);
    }

    pub fn update_window_size(&mut self, api: &mut dyn UiApi) {
        let row_count = api.status_icon_row_count(STATUS_ICON);
        if row_count > 0 {
            if self.on_cur_state {
                api.show_window(WINDOW);
            } else {
                api.hide_window(WINDOW);
            }
            let rect = api.get_rect(STATUS_ICON);
            api.set_window_size(WINDOW, rect.width + STATUS_ICON_FRAME_SIZE, rect.height);
            api.set_frame_size(WINDOW, STATUS_ICON_FRAME_SIZE, rect.height);
        } else {
            api.hide_window(WINDOW);
        }
    }

    pub fn handle_language_changed(&mut self, api: &mut dyn UiApi) {
        let row_count = api.status_icon_row_count(STATUS_ICON);
        for i in 0..row_count {
            let col_count = api.status_icon_col_count(STATUS_ICON, i);
            for j in 0..col_count {
                let mut info = api.status_icon_get_item(STATUS_ICON, i, j).unwrap_or_default();
                if info.class_id > 0 {
                    info.name = api.skill_name(info.class_id, info.level);
                    info.description = api.skill_description(info.class_id, info.level);
                    api.status_icon_set_item(STATUS_ICON, i, j, &info);
                }
            }
        }
    }

    pub fn rearrange_noblesse(&mut self, api: &mut dyn UiApi) {
        let mut row_count = api.status_icon_row_count(STATUS_ICON);
        let mut i = 0;
        while i < row_count {
            let mut col_count = api.status_icon_col_count(STATUS_ICON, i);
            let mut j = 0;
            while j < col_count {
                let info = api.status_icon_get_item(STATUS_ICON, i, j).unwrap_or_default();
                if is_noblesse_custom(info.class_id) {
                    api.status_icon_del_item(STATUS_ICON, i, j);
                    j -= 1;
                    col_count -= 1;
                    let new_row_count = api.status_icon_row_count(STATUS_ICON);
                    if new_row_count != row_count {
                        i -= 1;
                        row_count -= 1;
                    }
                    if api.get_option_bool(&self.option_section, "ShowNoble") {
                        self.debuff_row = -1;
                        self.insert_debuff_row(api);
                        api.status_icon_insert_col(STATUS_ICON, self.debuff_row, 0, &info);
                    } else {
                        api.status_icon_add_col(STATUS_ICON, self.row, &info);
                    }
                }
                j += 1;
            }
            i += 1;
        }
        self.update_window_size(api);
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        for event in EVENTS {
            api.register_event(event);
        }
        self.row = -1;
        self.song_row = -1;
        self.debuff_row = -1;
        self.etc_row = -1;
        self.short_row = -1;
        self.on_cur_state = false;
        self.init();
    }

    fn on_enter_state(&mut self, api: &mut dyn UiApi, _previous: &str) {
        self.on_cur_state = true;
        self.update_window_size(api);
    }

    fn on_exit_state(&mut self, _api: &mut dyn UiApi, _next: &str) {
        self.on_cur_state = false;
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        match event_id {
            950 => self.handle_add_normal_status(api, param),
            960 => self.handle_add_etc_status(api, param),
            970 => self.handle_add_short_status(api, param),
            40 => self.handle_restart(api),
            50 => self.handle_die(api),
            310 => self.handle_show_replay_quit_dialog_box(api),
            1900 => self.handle_language_changed(api),
            _ => {}
        }
    }

    fn on_show(&mut self, api: &mut dyn UiApi) {
        if api.status_icon_row_count(STATUS_ICON) < 1 {
            api.hide_window(WINDOW);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::RecordingApi;
    use super::super::Rect;
    use super::*;

    fn api() -> RecordingApi {
        RecordingApi {
            rects: vec![("AbnormalStatusWnd.StatusIcon".into(), Rect { x: 0, y: 0, width: 288, height: 52 })],
            options_int: vec![(("Neophron".into(), "AbnormalSize".into()), 1)],
            ..Default::default()
        }
    }

    fn calls(api: &RecordingApi, prefix: &str) -> Vec<String> {
        api.calls.iter().filter(|call| call.starts_with(prefix)).cloned().collect()
    }

    fn param(ids: &[i32]) -> String {
        let mut param = format!("Max={} ", ids.len());
        for (i, id) in ids.iter().enumerate() {
            param.push_str(&format!("SkillID_{i}={id} SkillLevel_{i}=1 RemainTime_{i}=10 Name_{i}=n{id} IconName_{i}=icon{id} Description_{i}=d{id} "));
        }
        param
    }

    #[test]
    fn on_load_registers_the_events_and_resets_the_rows() {
        let mut api = api();
        let mut script = Script { row: 5, ..Default::default() };
        script.on_load(&mut api);
        assert_eq!(calls(&api, "RegisterEvent(").len(), 7);
        assert_eq!((script.row, script.song_row, script.debuff_row, script.etc_row, script.short_row), (-1, -1, -1, -1, -1));
        assert_eq!(script.option_section, "Neophron");
        assert_eq!(Script::get_buff_size(0), 24);
        assert_eq!(Script::get_buff_size(2), 16);
        assert_eq!(Script::get_buff_size(3), 0);
    }

    #[test]
    fn normal_status_puts_buffs_in_rows_of_twelve_songs_and_debuffs_in_their_own_rows() {
        let mut api = api();
        let mut script = Script::default();
        script.init();
        script.on_enter_state(&mut api, "LoadingState");
        api.calls.clear();
        let mut ids: Vec<i32> = (2000..2013).collect();
        ids.push(264);
        ids.push(1069);
        ids.push(1323);
        script.on_event(&mut api, 950, &param(&ids));
        assert_eq!(calls(&api, "InsertRow("), ["InsertRow(AbnormalStatusWnd.StatusIcon,0)", "InsertRow(AbnormalStatusWnd.StatusIcon,1)", "InsertRow(AbnormalStatusWnd.StatusIcon,2)", "InsertRow(AbnormalStatusWnd.StatusIcon,3)"]);
        assert_eq!(calls(&api, "AddCol(").len(), 16);
        assert!(api.calls.contains(&"AddCol(AbnormalStatusWnd.StatusIcon,2,icon264)".to_string()));
        assert!(api.calls.contains(&"AddCol(AbnormalStatusWnd.StatusIcon,3,icon1069)".to_string()));
        assert!(api.calls.contains(&"AddCol(AbnormalStatusWnd.StatusIcon,1,icon1323)".to_string()));
        assert_eq!((script.row, script.song_row, script.debuff_row), (1, 2, 3));
        assert!(api.calls.contains(&"ShowWindow(AbnormalStatusWnd)".to_string()));
        assert_eq!(api.calls.last().unwrap(), "SetFrameSize(AbnormalStatusWnd,12,52)");
    }

    #[test]
    fn etc_and_short_items_open_rows_after_the_existing_ones_and_the_window_follows_the_grid() {
        let mut api = api();
        let mut script = Script::default();
        script.init();
        script.on_cur_state = true;
        script.on_event(&mut api, 950, &param(&[2000, 2001]));
        api.calls.clear();
        script.on_event(&mut api, 960, &param(&[3000]));
        assert_eq!(calls(&api, "InsertRow("), ["InsertRow(AbnormalStatusWnd.StatusIcon,1)"]);
        assert_eq!(script.etc_row, 1);
        api.calls.clear();
        script.on_event(&mut api, 970, &param(&[4000, 4001]));
        assert!(calls(&api, "InsertRow(").is_empty());
        assert_eq!(calls(&api, "InsertCol("), ["InsertCol(AbnormalStatusWnd.StatusIcon,1,0,icon4000)", "InsertCol(AbnormalStatusWnd.StatusIcon,1,1,icon4001)"]);
        assert_eq!(script.short_row, 1);
        assert_eq!(api.status_icon_col_count(STATUS_ICON, 1), 3);
        script.on_event(&mut api, 310, "");
        assert_eq!(api.calls.last().unwrap(), "HideWindow(AbnormalStatusWnd)");
    }

    #[test]
    fn die_and_restart_clear_the_rows_and_the_window_size_follows_the_icon_rect() {
        let mut api = api();
        let mut script = Script::default();
        script.init();
        script.on_enter_state(&mut api, "LoadingState");
        assert_eq!(api.calls, ["HideWindow(AbnormalStatusWnd)"]);
        script.on_event(&mut api, 950, &param(&[2000, 264]));
        api.calls.clear();
        script.update_window_size(&mut api);
        assert_eq!(api.calls, ["ShowWindow(AbnormalStatusWnd)", "SetWindowSize(AbnormalStatusWnd,300,52)", "SetFrameSize(AbnormalStatusWnd,12,52)"]);
        assert_eq!((script.row, script.song_row), (0, 1));
        api.calls.clear();
        script.on_event(&mut api, 50, "");
        assert_eq!((script.row, script.song_row, script.debuff_row, script.short_row), (-1, -1, -1, -1));
        assert_eq!(calls(&api, "DelItem(").len(), 2);
        assert_eq!(api.status_icon_row_count(STATUS_ICON), 0);
        script.etc_row = 3;
        script.on_event(&mut api, 40, "");
        assert_eq!(script.etc_row, -1);
    }
}
