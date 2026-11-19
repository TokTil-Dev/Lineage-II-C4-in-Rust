use super::*;

#[derive(Default)]
pub struct RecordingApi {
    pub calls: Vec<String>,
    pub ini_strings: Vec<((String, String, String), String)>,
    pub shown: Vec<String>,
    pub texts: Vec<(String, String)>,
    pub options_bool: Vec<((String, String), bool)>,
    pub options_int: Vec<((String, String), i32)>,
    pub sysstrings: Vec<(i32, String)>,
    pub player: Option<UserInfo>,
    pub target: Option<UserInfo>,
    pub pet: Option<PetInfo>,
    pub rects: Vec<(String, Rect)>,
    pub checks: Vec<(String, bool)>,
    pub tab_top: Vec<(String, i32)>,
    pub items: Vec<(String, Vec<ItemInfo>)>,
    pub item_selected: Vec<(String, i32)>,
    pub item_index_at: Vec<(String, (i32, i32), i32)>,
    pub accessory: Option<(i32, i32, i32, i32)>,
    pub class_step: i32,
    pub adena: i32,
    pub keys_down: Vec<i32>,
    pub click_location: Vector,
    pub inventory_order: Option<Vec<i32>>,
    pub crystallize_ability: bool,
    pub inventory_limit: i32,
    pub hennas: Vec<(i32, i32)>,
    pub dialog_target: String,
    pub dialog_edit: String,
    pub dialog_id: i32,
    pub dialog_reserved: i32,
    pub dialog_reserved2: i32,
    pub script_bools: Vec<(String, String, bool)>,
    pub script_ints: Vec<(String, String, i32)>,
    pub quest_location: Option<Vector>,
    pub party_locations: Vec<Vector>,
    pub zone_name: String,
    pub pk_mode: bool,
    pub debuffs: Vec<i32>,
    pub script_int_arrays: Vec<(String, String, i32, i32)>,
    pub status_grids: Vec<(String, Vec<Vec<StatusIconInfo>>)>,
    pub server_object: bool,
    pub hp_showable_npc: bool,
    pub static_object_name_id: i32,
    pub server_object_type: i32,
    pub server_object_show_hp: bool,
    pub server_object_hp: (i32, i32),
    pub npc_property: Option<Vec<i32>>,
    pub clan_type: Option<i32>,
    pub clan_name_value: Option<i32>,
    pub clan_crest: bool,
    pub alliance_name: String,
    pub alliance_crest: bool,
    pub move_type: i32,
    pub environment: i32,
    pub exp_rate: f32,
}

impl RecordingApi {
    fn record(&mut self, call: String) {
        self.calls.push(call);
    }

    fn grid(&mut self, name: &str) -> &mut Vec<Vec<StatusIconInfo>> {
        if let Some(index) = self.status_grids.iter().position(|(known, _)| known == name) {
            return &mut self.status_grids[index].1;
        }
        self.status_grids.push((name.to_string(), Vec::new()));
        let last = self.status_grids.len() - 1;
        &mut self.status_grids[last].1
    }

    fn item_slots(&self, name: &str) -> &[ItemInfo] {
        self.items.iter().find(|(known, _)| known == name).map_or(&[], |(_, slots)| slots.as_slice())
    }

    fn item_slots_mut(&mut self, name: &str) -> &mut Vec<ItemInfo> {
        if let Some(index) = self.items.iter().position(|(known, _)| known == name) {
            return &mut self.items[index].1;
        }
        self.items.push((name.to_string(), Vec::new()));
        let last = self.items.len() - 1;
        &mut self.items[last].1
    }
}

impl UiApi for RecordingApi {
    fn is_show_window(&self, name: &str) -> bool {
        self.shown.iter().any(|known| known == name)
    }
    fn show_window(&mut self, name: &str) {
        if !self.is_show_window(name) {
            self.shown.push(name.to_string());
        }
        self.record(format!("ShowWindow({name})"));
    }
    fn hide_window(&mut self, name: &str) {
        self.shown.retain(|known| known != name);
        self.record(format!("HideWindow({name})"));
    }
    fn clear(&mut self, name: &str) {
        self.record(format!("Clear({name})"));
    }
    fn get_rect(&self, name: &str) -> Rect {
        self.rects.iter().find(|(known, _)| known == name).map_or(Rect::default(), |(_, rect)| *rect)
    }
    fn set_ui_timer(&mut self, name: &str, timer_id: i32, delay_ms: i32) {
        self.record(format!("SetUITimer({name},{timer_id},{delay_ms})"));
    }
    fn kill_ui_timer(&mut self, name: &str, timer_id: i32) {
        self.record(format!("KillUITimer({name},{timer_id})"));
    }
    fn set_window_title(&mut self, name: &str, index: i32) {
        self.record(format!("SetWindowTitle({name},{index})"));
    }
    fn set_window_title_by_text(&mut self, name: &str, text: &str) {
        self.record(format!("SetWindowTitleByText({name},{text})"));
    }
    fn enable_window(&mut self, name: &str) {
        self.record(format!("EnableWindow({name})"));
    }
    fn disable_window(&mut self, name: &str) {
        self.record(format!("DisableWindow({name})"));
    }
    fn is_enable_window(&self, _name: &str) -> bool {
        true
    }
    fn set_always_on_top(&mut self, name: &str, on: bool) {
        self.record(format!("SetAlwaysOnTop({name},{on})"));
    }
    fn set_alpha(&mut self, name: &str, alpha: i32, seconds: f32) {
        self.record(format!("SetAlpha({name},{alpha},{seconds})"));
    }
    fn move_by(&mut self, name: &str, dx: i32, dy: i32, seconds: f32) {
        self.record(format!("Move({name},{dx},{dy},{seconds})"));
    }
    fn move_to(&mut self, name: &str, x: i32, y: i32) {
        self.record(format!("MoveTo({name},{x},{y})"));
    }
    fn iconize(&mut self, name: &str, texture: &str, tooltip: i32) {
        self.record(format!("Iconize({name},{texture},{tooltip})"));
    }
    fn set_focus(&mut self, name: &str) {
        self.record(format!("SetFocus({name})"));
    }
    fn is_focused(&self, _name: &str) -> bool {
        false
    }
    fn set_window_size(&mut self, name: &str, width: i32, height: i32) {
        self.record(format!("SetWindowSize({name},{width},{height})"));
    }
    fn set_window_size_rel(&mut self, name: &str, width_rate: f32, height_rate: f32, offset_width: i32, offset_height: i32) {
        self.record(format!("SetWindowSizeRel({name},{width_rate},{height_rate},{offset_width},{offset_height})"));
    }
    fn set_frame_size(&mut self, name: &str, width: i32, height: i32) {
        self.record(format!("SetFrameSize({name},{width},{height})"));
    }
    fn set_anchor(&mut self, name: &str, anchor_window: &str, relative_point: &str, anchor_point: &str, x: i32, y: i32) {
        self.record(format!("SetAnchor({name},{anchor_window},{relative_point},{anchor_point},{x},{y})"));
    }
    fn clear_anchor(&mut self, name: &str) {
        self.record(format!("ClearAnchor({name})"));
    }
    fn set_tooltip_text(&mut self, name: &str, text: &str) {
        self.record(format!("SetTooltipText({name},{text})"));
    }
    fn set_text(&mut self, name: &str, text: &str) {
        self.texts.push((name.to_string(), text.to_string()));
        self.record(format!("SetText({name},{text})"));
    }
    fn set_text_color(&mut self, name: &str, color: Argb) {
        self.record(format!("SetTextColor({name},{:08X})", color.0));
    }
    fn set_text_align(&mut self, name: &str, align: TextAlign) {
        self.record(format!("SetAlign({name},{align:?})"));
    }
    fn set_int(&mut self, name: &str, value: i32) {
        self.record(format!("SetInt({name},{value})"));
    }
    fn get_text(&self, name: &str) -> String {
        self.texts.iter().rev().find(|(known, _)| known == name).map(|(_, text)| text.clone()).unwrap_or_default()
    }
    fn set_texture(&mut self, name: &str, texture: &str) {
        self.record(format!("SetTexture({name},{texture})"));
    }
    fn set_uv(&mut self, name: &str, u: i32, v: i32) {
        self.record(format!("SetUV({name},{u},{v})"));
    }
    fn get_texture_name(&self, _name: &str) -> String {
        String::new()
    }
    fn set_name(&mut self, name: &str, text: &str, kind: NameCtrlType, align: TextAlign) {
        self.record(format!("SetName({name},{text},{kind:?},{align:?})"));
    }
    fn set_name_with_color(&mut self, name: &str, text: &str, kind: NameCtrlType, align: TextAlign, color: Argb) {
        self.record(format!("SetNameWithColor({name},{text},{kind:?},{align:?},{:08X})", color.0));
    }
    fn set_point(&mut self, name: &str, current: i32, max: i32) {
        self.record(format!("SetPoint({name},{current},{max})"));
    }
    fn set_point_percent(&mut self, name: &str, current: i64, min: i64, max: i64) {
        self.record(format!("SetPointPercent({name},{current},{min},{max})"));
    }
    fn set_point_exp(&mut self, name: &str, current: i64, level: i32) {
        self.record(format!("SetPointExp({name},{current},{level})"));
    }
    fn set_regen_info(&mut self, name: &str, duration: i32, ticks: i32, amount: f32) {
        self.record(format!("SetRegenInfo({name},{duration},{ticks},{amount})"));
    }
    fn bar_set_value(&mut self, name: &str, max: i32, current: i32) {
        self.record(format!("BarSetValue({name},{max},{current})"));
    }
    fn bar_get_value(&self, _name: &str) -> (i32, i32) {
        (0, 0)
    }
    fn checkbox_set_title(&mut self, name: &str, title: &str) {
        self.record(format!("CheckSetTitle({name},{title})"));
    }
    fn checkbox_set_check(&mut self, name: &str, checked: bool) {
        self.checks.retain(|(known, _)| known != name);
        self.checks.push((name.to_string(), checked));
        self.record(format!("SetCheck({name},{checked})"));
    }
    fn checkbox_is_checked(&self, name: &str) -> bool {
        self.checks.iter().find(|(known, _)| known == name).is_some_and(|(_, checked)| *checked)
    }
    fn checkbox_set_disable(&mut self, name: &str, disabled: bool) {
        self.record(format!("CheckSetDisable({name},{disabled})"));
    }
    fn checkbox_is_disable(&self, _name: &str) -> bool {
        false
    }
    fn update_shortcut(&mut self, name: &str, shortcut_id: i32) {
        self.record(format!("UpdateShortcut({name},{shortcut_id})"));
    }
    fn status_icon_add_row(&mut self, name: &str) {
        self.grid(name).push(Vec::new());
        self.record(format!("AddRow({name})"));
    }
    fn status_icon_add_col(&mut self, name: &str, row: i32, info: &StatusIconInfo) {
        if let Some(cells) = usize::try_from(row).ok().and_then(|row| self.grid(name).get_mut(row)) {
            cells.push(info.clone());
        }
        self.record(format!("AddCol({name},{row},{})", info.icon_name));
    }
    fn status_icon_insert_row(&mut self, name: &str, row: i32) {
        let grid = self.grid(name);
        let at = usize::try_from(row).unwrap_or(0).min(grid.len());
        grid.insert(at, Vec::new());
        self.record(format!("InsertRow({name},{row})"));
    }
    fn status_icon_insert_col(&mut self, name: &str, row: i32, col: i32, info: &StatusIconInfo) {
        if let Some(cells) = usize::try_from(row).ok().and_then(|row| self.grid(name).get_mut(row)) {
            let at = usize::try_from(col).unwrap_or(0).min(cells.len());
            cells.insert(at, info.clone());
        }
        self.record(format!("InsertCol({name},{row},{col},{})", info.icon_name));
    }
    fn status_icon_row_count(&self, name: &str) -> i32 {
        self.status_grids.iter().find(|(known, _)| known == name).map_or(0, |(_, rows)| rows.len() as i32)
    }
    fn status_icon_col_count(&self, name: &str, row: i32) -> i32 {
        self.status_grids
            .iter()
            .find(|(known, _)| known == name)
            .and_then(|(_, rows)| rows.get(usize::try_from(row).ok()?))
            .map_or(0, |cells| cells.len() as i32)
    }
    fn status_icon_get_item(&self, name: &str, row: i32, col: i32) -> Option<StatusIconInfo> {
        let rows = &self.status_grids.iter().find(|(known, _)| known == name)?.1;
        rows.get(usize::try_from(row).ok()?)?.get(usize::try_from(col).ok()?).cloned()
    }
    fn status_icon_del_item(&mut self, name: &str, row: i32, col: i32) {
        let grid = self.grid(name);
        if let Some(index) = usize::try_from(row).ok().filter(|row| *row < grid.len()) {
            let cells = &mut grid[index];
            if let Some(col) = usize::try_from(col).ok().filter(|col| *col < cells.len()) {
                cells.remove(col);
            }
            if cells.is_empty() {
                grid.remove(index);
            }
        }
        self.record(format!("DelItem({name},{row},{col})"));
    }
    fn tab_init(&mut self, name: &str) {
        self.record(format!("InitTabCtrl({name})"));
    }
    fn tab_set_top_order(&mut self, name: &str, index: i32, send_message: bool) {
        self.tab_top.retain(|(known, _)| known != name);
        self.tab_top.push((name.to_string(), index));
        self.record(format!("SetTopOrder({name},{index},{send_message})"));
    }
    fn tab_get_top_index(&self, name: &str) -> i32 {
        self.tab_top.iter().find(|(known, _)| known == name).map_or(0, |(_, index)| *index)
    }
    fn tab_set_disable(&mut self, name: &str, index: i32, disabled: bool) {
        self.record(format!("TabSetDisable({name},{index},{disabled})"));
    }
    fn tab_merge(&mut self, name: &str, index: i32) {
        self.record(format!("MergeTab({name},{index})"));
    }
    fn chat_enable_texture(&mut self, name: &str, on: bool) {
        self.record(format!("EnableTexture({name},{on})"));
    }
    fn chat_set_scroll_bar_position(&mut self, name: &str, x: i32, y: i32, height: i32) {
        self.record(format!("SetScrollBarPosition({name},{x},{y},{height})"));
    }
    fn set_chat_message(&mut self, message: &str) {
        self.record(format!("SetChatMessage({message})"));
    }
    fn combo_add_string(&mut self, name: &str, text: &str, reserved: i32) {
        self.record(format!("ComboAddString({name},{text},{reserved})"));
    }
    fn combo_clear(&mut self, name: &str) {
        self.record(format!("ComboClear({name})"));
    }
    fn combo_get_selected_num(&self, _name: &str) -> i32 {
        0
    }
    fn combo_set_selected_num(&mut self, name: &str, index: i32) {
        self.record(format!("ComboSetSelectedNum({name},{index})"));
    }
    fn combo_get_string(&self, _name: &str, _index: i32) -> String {
        String::new()
    }
    fn combo_get_reserved(&self, _name: &str, _index: i32) -> i32 {
        0
    }
    fn edit_get_string(&self, name: &str) -> String {
        self.get_text(name)
    }
    fn edit_set_string(&mut self, name: &str, text: &str) {
        self.set_text(name, text);
    }
    fn radar_add_target(&mut self, name: &str, x: i32, y: i32, z: i32) {
        self.record(format!("RadarAddTarget({name},{x},{y},{z})"));
    }
    fn radar_delete_target(&mut self, name: &str, x: i32, y: i32, z: i32) {
        self.record(format!("RadarDeleteTarget({name},{x},{y},{z})"));
    }
    fn radar_delete_all_target(&mut self, name: &str) {
        self.record(format!("RadarDeleteAllTarget({name})"));
    }
    fn radar_set_color(&mut self, name: &str, color: Argb, seconds: f32) {
        self.record(format!("SetRadarColor({name},{:08X},{seconds})", color.0));
    }
    fn minimap_adjust_map_view(&mut self, name: &str, loc: Vector, zoom_to_town: bool, use_grid_location: bool) {
        self.record(format!("AdjustMapView({name},{},{},{},{zoom_to_town},{use_grid_location})", loc.x, loc.y, loc.z));
    }
    fn minimap_init_position(&mut self, name: &str) {
        self.record(format!("InitPosition({name})"));
    }
    fn minimap_add_target(&mut self, name: &str, loc: Vector) {
        self.record(format!("MinimapAddTarget({name},{},{},{})", loc.x, loc.y, loc.z));
    }
    fn minimap_delete_target(&mut self, name: &str, loc: Vector) {
        self.record(format!("MinimapDeleteTarget({name},{},{},{})", loc.x, loc.y, loc.z));
    }
    fn minimap_delete_all_target(&mut self, name: &str) {
        self.record(format!("MinimapDeleteAllTarget({name})"));
    }
    fn minimap_set_show_quest(&mut self, name: &str, show: bool) {
        self.record(format!("SetShowQuest({name},{show})"));
    }
    fn minimap_set_ssq_status(&mut self, name: &str, status: i32) {
        self.record(format!("SetSSQStatus({name},{status})"));
    }
    fn minimap_draw_grid_icon(&mut self, name: &str, icon: &str, dup_icon: &str, loc: Vector, refresh: bool, x_offset: i32, y_offset: i32, tooltip: &str) {
        self.record(format!("DrawGridIcon({name},{icon},{dup_icon},{},{},{},{refresh},{x_offset},{y_offset},{tooltip})", loc.x, loc.y, loc.z));
    }
    fn minimap_request_reduce_btn(&mut self, name: &str) {
        self.record(format!("RequestReduceBtn({name})"));
    }
    fn minimap_is_overlapped(&self, _name: &str, _first_x: i32, _first_y: i32, _second_x: i32, _second_y: i32) -> bool {
        false
    }
    fn minimap_delete_all_cursed_weapon_icon(&mut self, name: &str) {
        self.record(format!("DeleteAllCursedWeaponIcon({name})"));
    }
    fn register_event(&mut self, event_id: i32) {
        self.record(format!("RegisterEvent({event_id})"));
    }
    fn register_state(&mut self, window: &str, state: &str) {
        self.record(format!("RegisterState({window},{state})"));
    }
    fn set_ui_state(&mut self, state: &str) {
        self.record(format!("SetUIState({state})"));
    }
    fn get_ui_state(&self) -> String {
        String::new()
    }
    fn execute_event(&mut self, event_id: i32, param: &str) {
        self.record(format!("ExecuteEvent({event_id},{param})"));
    }
    fn get_option_bool(&self, section: &str, name: &str) -> bool {
        self.options_bool.iter().find(|((s, n), _)| s == section && n == name).is_some_and(|(_, value)| *value)
    }
    fn get_option_int(&self, section: &str, name: &str) -> i32 {
        self.options_int.iter().find(|((s, n), _)| s == section && n == name).map_or(0, |(_, value)| *value)
    }
    fn get_option_string(&self, _section: &str, _name: &str) -> String {
        String::new()
    }
    fn set_option_bool(&mut self, section: &str, name: &str, value: bool) {
        self.options_bool.retain(|((s, n), _)| !(s == section && n == name));
        self.options_bool.push(((section.to_string(), name.to_string()), value));
        self.record(format!("SetOptionBool({section},{name},{value})"));
    }
    fn set_option_int(&mut self, section: &str, name: &str, value: i32) {
        self.options_int.retain(|((s, n), _)| !(s == section && n == name));
        self.options_int.push(((section.to_string(), name.to_string()), value));
        self.record(format!("SetOptionInt({section},{name},{value})"));
    }
    fn get_ini_bool(&self, _section: &str, _key: &str, _file: &str) -> Option<i32> {
        None
    }
    fn get_system_string(&self, id: i32) -> String {
        self.sysstrings.iter().find(|(known, _)| *known == id).map(|(_, text)| text.clone()).unwrap_or_default()
    }
    fn get_system_message(&self, id: i32) -> String {
        format!("sysmsg{id}")
    }
    fn make_full_system_msg(&self, message: &str, arg1: &str, arg2: &str) -> String {
        format!("{message}|{arg1}|{arg2}")
    }
    fn get_text_size(&self, text: &str) -> (i32, i32) {
        (text.chars().count() as i32 * 6, 10)
    }
    fn play_sound(&mut self, sound: &str) {
        self.record(format!("PlaySound({sound})"));
    }
    fn play_console_sound(&mut self, sound: InterfaceSound) {
        self.record(format!("PlayConsoleSound({sound:?})"));
    }
    fn debug(&mut self, _message: &str) {}
    fn get_player_info(&self) -> Option<UserInfo> {
        self.player.clone()
    }
    fn get_target_info(&self) -> Option<UserInfo> {
        self.target.clone()
    }
    fn get_user_info(&self, _user_id: i32) -> Option<UserInfo> {
        None
    }
    fn get_pet_info(&self) -> Option<PetInfo> {
        self.pet.clone()
    }
    fn get_skill_info(&self, _skill_id: i32, _level: i32) -> Option<SkillInfo> {
        None
    }
    fn get_player_position(&self) -> Vector {
        self.player.as_ref().map_or(Vector::default(), |player| player.loc)
    }
    fn set_timer(&mut self, window: &str, timer_id: i32, delay_ms: i32) {
        self.record(format!("SetTimer({window},{timer_id},{delay_ms})"));
    }
    fn kill_timer(&mut self, window: &str, timer_id: i32) {
        self.record(format!("KillTimer({window},{timer_id})"));
    }
    fn edit_add_string(&mut self, name: &str, text: &str) {
        let joined = format!("{}{text}", self.get_text(name));
        self.texts.push((name.to_string(), joined));
        self.record(format!("EditAddString({name},{text})"));
    }
    fn edit_release_focus(&mut self, name: &str) {
        self.record(format!("ReleaseFocus({name})"));
    }
    fn chat_add_string(&mut self, name: &str, text: &str, color: Argb) {
        self.record(format!("AddString({name},{text},{:08X})", color.0));
    }
    fn set_settled_wnd(&mut self, name: &str, settled: bool) {
        self.record(format!("SetSettledWnd({name},{settled})"));
    }
    fn get_window_size(&self, name: &str) -> (i32, i32) {
        let rect = self.get_rect(name);
        (rect.width, rect.height)
    }
    fn button_set_texture(&mut self, name: &str, fore: &str, back: &str, highlight: &str) {
        self.record(format!("ButtonSetTexture({name},{fore},{back},{highlight})"));
    }
    fn set_tooltip_simple_text(&mut self, name: &str, text: &str) {
        self.record(format!("SetTooltipCustomType({name},{text})"));
    }
    fn get_current_ime_lang(&self) -> i32 {
        2
    }
    fn process_chat_message(&mut self, message: &str, chat_type: i32) {
        self.record(format!("ProcessChatMessage({message},{chat_type})"));
    }
    fn is_same_chat_prefix(&self, chat_type: i32, prefix: &str) -> bool {
        self.get_chat_prefix(chat_type) == prefix
    }
    fn get_chat_prefix(&self, chat_type: i32) -> String {
        match chat_type {
            1 => "!".into(),
            2 => "\"".into(),
            3 => "#".into(),
            4 => "@".into(),
            8 => "+".into(),
            9 => "$".into(),
            _ => String::new(),
        }
    }
    fn add_system_message(&mut self, message: &str, color: Argb) {
        self.record(format!("AddSystemMessage({message},{:08X})", color.0));
    }
    fn toggle_msn_window(&mut self) {
        self.record("ToggleMsnWindow()".into());
    }
    fn execute_command(&mut self, command: &str) {
        self.record(format!("ExecuteCommand({command})"));
    }
    fn request_open_party_match(&mut self) {
        self.record("RequestOpenPartyMatch()".into());
    }
    fn script_on_send_packet_when_hiding(&mut self, window: &str) {
        self.record(format!("OnSendPacketWhenHiding({window})"));
    }
    fn set_shortcut_page(&mut self, page: i32) {
        self.record(format!("SetShortcutPage({page})"));
    }
    fn request_cursed_weapon_list(&mut self) {
        self.record("RequestCursedWeaponList()".to_string());
    }
    fn request_cursed_weapon_location(&mut self) {
        self.record("RequestCursedWeaponLocation()".to_string());
    }
    fn get_quest_location(&self) -> Option<Vector> {
        self.quest_location
    }
    fn get_party_member_count(&self) -> i32 {
        self.party_locations.len() as i32
    }
    fn get_party_member_location(&self, index: i32) -> Option<Vector> {
        usize::try_from(index).ok().and_then(|index| self.party_locations.get(index).copied())
    }
    fn get_current_zone_name(&self) -> String {
        self.zone_name.clone()
    }
    fn get_ini_string(&self, section: &str, key: &str, file: &str) -> Option<String> {
        self.ini_strings.iter().find(|((s, k, f), _)| s == section && k == key && f == file).map(|(_, value)| value.clone())
    }
    fn item_get_selected_num(&self, name: &str) -> i32 {
        self.item_selected.iter().find(|(known, _)| known == name).map_or(-1, |(_, index)| *index)
    }
    fn item_get_item_num(&self, name: &str) -> i32 {
        self.item_slots(name).len() as i32
    }
    fn item_clear_select(&mut self, name: &str) {
        self.item_selected.retain(|(known, _)| known != name);
        self.record(format!("ClearSelect({name})"));
    }
    fn item_add(&mut self, name: &str, info: &ItemInfo) {
        self.item_slots_mut(name).push(info.clone());
        self.record(format!("AddItem({name},{})", info.server_id));
    }
    fn item_set(&mut self, name: &str, index: i32, info: &ItemInfo) -> bool {
        self.record(format!("SetItem({name},{index},{})", info.server_id));
        let slots = self.item_slots_mut(name);
        match usize::try_from(index).ok().filter(|index| *index < slots.len()) {
            Some(index) => {
                slots[index] = info.clone();
                true
            }
            None => false,
        }
    }
    fn item_delete(&mut self, name: &str, index: i32) {
        self.record(format!("DeleteItem({name},{index})"));
        let slots = self.item_slots_mut(name);
        if let Some(index) = usize::try_from(index).ok().filter(|index| *index < slots.len()) {
            slots.remove(index);
        }
    }
    fn item_get_selected(&self, name: &str) -> Option<ItemInfo> {
        let index = self.item_get_selected_num(name);
        self.item_get(name, index)
    }
    fn item_get(&self, name: &str, index: i32) -> Option<ItemInfo> {
        usize::try_from(index).ok().and_then(|index| self.item_slots(name).get(index).cloned())
    }
    fn item_clear(&mut self, name: &str) {
        self.item_slots_mut(name).clear();
        self.record(format!("ItemClear({name})"));
    }
    fn item_find_with_server_id(&self, name: &str, server_id: i32) -> i32 {
        self.item_slots(name).iter().position(|item| item.server_id == server_id).map_or(-1, |index| index as i32)
    }
    fn item_swap(&mut self, name: &str, first: i32, second: i32) {
        self.record(format!("SwapItems({name},{first},{second})"));
        let slots = self.item_slots_mut(name);
        if let (Ok(first), Ok(second)) = (usize::try_from(first), usize::try_from(second))
            && first < slots.len()
            && second < slots.len()
        {
            slots.swap(first, second);
        }
    }
    fn item_get_index_at(&self, name: &str, x: i32, y: i32, _offset_x: i32, _offset_y: i32) -> i32 {
        self.item_index_at.iter().find(|(known, at, _)| known == name && *at == (x, y)).map_or(-1, |(_, _, index)| *index)
    }
    fn item_set_disable_tex(&mut self, name: &str, texture: &str) {
        self.record(format!("SetDisableTex({name},{texture})"));
    }
    fn item_set_row(&mut self, name: &str, row: i32) {
        self.record(format!("SetRow({name},{row})"));
    }
    fn set_tooltip_string(&mut self, name: &str, text: &str) {
        self.record(format!("SetTooltipString({name},{text})"));
    }
    fn request_use_item(&mut self, server_id: i32) {
        self.record(format!("RequestUseItem({server_id})"));
    }
    fn request_destroy_item(&mut self, server_id: i32, num: i32) {
        self.record(format!("RequestDestroyItem({server_id},{num})"));
    }
    fn request_drop_item(&mut self, server_id: i32, num: i32, location: Vector) {
        self.record(format!("RequestDropItem({server_id},{num},{},{},{})", location.x, location.y, location.z));
    }
    fn request_unequip_item(&mut self, server_id: i32, slot_bit_type: i32) {
        self.record(format!("RequestUnequipItem({server_id},{slot_bit_type})"));
    }
    fn request_crystallize_item(&mut self, server_id: i32, num: i32) {
        self.record(format!("RequestCrystallizeItem({server_id},{num})"));
    }
    fn request_item_list(&mut self) {
        self.record("RequestItemList()".to_string());
    }
    fn request_open_minimap(&mut self) {
        self.record("RequestOpenMinimap()".to_string());
    }
    fn request_exit(&mut self) {
        self.record("RequestExit()".to_string());
    }
    fn request_get_item_from_pet(&mut self, server_id: i32, num: i32, is_equip_item: bool) {
        self.record(format!("RequestGetItemFromPet({server_id},{num},{is_equip_item})"));
    }
    fn get_accessory_server_id(&self) -> Option<(i32, i32, i32, i32)> {
        self.accessory
    }
    fn get_class_step(&self, _class_id: i32) -> i32 {
        self.class_step
    }
    fn get_adena(&self) -> i32 {
        self.adena
    }
    fn is_key_down(&self, key: i32) -> bool {
        self.keys_down.contains(&key)
    }
    fn make_cost_string(&self, text: &str) -> String {
        format!("cost:{text}")
    }
    fn convert_num_to_text(&self, text: &str) -> String {
        format!("text:{text}")
    }
    fn check_item_limit(&self, _class_id: i32, _count: i32) -> bool {
        true
    }
    fn get_click_location(&self) -> Vector {
        self.click_location
    }
    fn is_stackable_item(&self, consume_type: i32) -> bool {
        consume_type != 0
    }
    fn save_inventory_order(&mut self, order: &[i32]) {
        self.inventory_order = Some(order.to_vec());
        self.record(format!("SaveInventoryOrder({order:?})"));
    }
    fn load_inventory_order(&self) -> Option<Vec<i32>> {
        self.inventory_order.clone()
    }
    fn player_has_crystallize_ability(&self) -> bool {
        self.crystallize_ability
    }
    fn player_inventory_limit(&self) -> i32 {
        self.inventory_limit
    }
    fn item_is_crystallizable(&self, _class_id: i32) -> bool {
        true
    }
    fn henna_info_count(&self) -> i32 {
        self.hennas.len() as i32
    }
    fn henna_info(&self, index: i32) -> Option<(i32, i32)> {
        usize::try_from(index).ok().and_then(|index| self.hennas.get(index).copied())
    }
    fn henna_item_name(&self, henna_id: i32) -> Option<String> {
        Some(format!("henna{henna_id}"))
    }
    fn henna_description(&self, henna_id: i32) -> Option<String> {
        Some(format!("desc{henna_id}"))
    }
    fn henna_icon_tex(&self, henna_id: i32) -> Option<String> {
        Some(format!("icon{henna_id}"))
    }
    fn dialog_show(&mut self, kind: i32, message: &str, target: &str) {
        self.dialog_target = target.to_string();
        self.record(format!("DialogShow({kind},{message},{target})"));
    }
    fn dialog_hide(&mut self) {
        self.record("DialogHide()".to_string());
    }
    fn dialog_set_id(&mut self, id: i32) {
        self.dialog_id = id;
        self.record(format!("DialogSetID({id})"));
    }
    fn dialog_set_param_int(&mut self, value: i32) {
        self.record(format!("DialogSetParamInt({value})"));
    }
    fn dialog_set_reserved_int(&mut self, value: i32) {
        self.dialog_reserved = value;
        self.record(format!("DialogSetReservedInt({value})"));
    }
    fn dialog_set_reserved_int2(&mut self, value: i32) {
        self.dialog_reserved2 = value;
        self.record(format!("DialogSetReservedInt2({value})"));
    }
    fn dialog_get_target(&self) -> String {
        self.dialog_target.clone()
    }
    fn dialog_get_edit_message(&self) -> String {
        self.dialog_edit.clone()
    }
    fn dialog_get_id(&self) -> i32 {
        self.dialog_id
    }
    fn dialog_get_reserved_int(&self) -> i32 {
        self.dialog_reserved
    }
    fn dialog_get_reserved_int2(&self) -> i32 {
        self.dialog_reserved2
    }
    fn script_get_bool(&self, window: &str, var: &str) -> bool {
        self.script_bools.iter().any(|(w, v, value)| w == window && v == var && *value)
    }
    fn script_get_int(&self, window: &str, var: &str) -> i32 {
        self.script_ints.iter().find(|(w, v, _)| w == window && v == var).map_or(0, |(_, _, value)| *value)
    }
    fn request_self_target(&mut self) {
        self.record("RequestSelfTarget".into());
    }
    fn request_target_cancel(&mut self) {
        self.record("RequestTargetCancel".into());
    }
    fn request_start_trade(&mut self, target_id: i32) {
        self.record(format!("RequestStartTrade({target_id})"));
    }
    fn request_invite_party(&mut self, name: &str) {
        self.record(format!("RequestInviteParty({name})"));
    }
    fn request_target_user(&mut self, server_id: i32) {
        self.record(format!("RequestTargetUser({server_id})"));
    }
    fn request_attack(&mut self, server_id: i32, loc: Vector) {
        self.record(format!("RequestAttack({server_id},{},{},{})", loc.x, loc.y, loc.z));
    }
    fn request_action(&mut self, server_id: i32, loc: Vector) {
        self.record(format!("RequestAction({server_id},{},{},{})", loc.x, loc.y, loc.z));
    }
    fn request_assist(&mut self, server_id: i32, loc: Vector) {
        self.record(format!("RequestAssist({server_id},{},{},{})", loc.x, loc.y, loc.z));
    }
    fn is_pk_mode(&self) -> bool {
        self.pk_mode
    }
    fn get_class_type(&self, class_id: i32) -> String {
        format!("type{class_id}")
    }
    fn get_class_str(&self, class_id: i32) -> String {
        format!("class{class_id}")
    }
    fn get_class_icon_name(&self, class_id: i32) -> String {
        format!("icon{class_id}")
    }
    fn get_user_rank_string(&self, rank: i32) -> String {
        format!("rank{rank}")
    }
    fn get_routing_string(&self, routing_type: i32) -> String {
        format!("routing{routing_type}")
    }
    fn is_debuff(&self, skill_id: i32, _level: i32) -> bool {
        self.debuffs.contains(&skill_id)
    }
    fn dialog_get_string(&self) -> String {
        String::new()
    }
    fn script_call(&mut self, window: &str, function: &str) {
        self.record(format!("ScriptCall({window},{function})"));
    }
    fn script_get_int_array(&self, window: &str, variable: &str, index: i32) -> i32 {
        self.script_int_arrays.iter().find(|(w, v, i, _)| w == window && v == variable && *i == index).map_or(0, |(_, _, _, value)| *value)
    }
    fn target_id(&self) -> i32 {
        self.target.as_ref().map_or(-1, |target| target.id)
    }
    fn target_name_color(&self, _level_diff: i32) -> Argb {
        Argb(0xFFFFFFFF)
    }
    fn target_is_server_object(&self) -> bool {
        self.server_object
    }
    fn target_is_hp_showable_npc(&self) -> bool {
        self.hp_showable_npc
    }
    fn static_object_name_id(&self, _server_id: i32) -> i32 {
        self.static_object_name_id
    }
    fn static_object_name(&self, name_id: i32) -> String {
        format!("static{name_id}")
    }
    fn server_object_type(&self, _server_id: i32) -> i32 {
        self.server_object_type
    }
    fn static_object_show_hp(&self, _server_id: i32) -> bool {
        self.server_object_show_hp
    }
    fn server_object_hp(&self, _server_id: i32) -> i32 {
        self.server_object_hp.0
    }
    fn server_object_max_hp(&self, _server_id: i32) -> i32 {
        self.server_object_hp.1
    }
    fn party_member_name(&self, server_id: i32) -> String {
        format!("member{server_id}")
    }
    fn player_id(&self) -> i32 {
        self.player.as_ref().map_or(0, |player| player.id)
    }
    fn pet_id(&self) -> i32 {
        self.pet.as_ref().map_or(0, |pet| pet.id)
    }
    fn npc_property(&self, _class_id: i32) -> Option<Vec<i32>> {
        self.npc_property.clone()
    }
    fn clan_name(&self, clan_id: i32) -> String {
        format!("clan{clan_id}")
    }
    fn clan_type(&self, _user_id: i32) -> Option<i32> {
        self.clan_type
    }
    fn clan_name_value(&self, _clan_id: i32) -> Option<i32> {
        self.clan_name_value
    }
    fn clan_crest_texture(&self, clan_id: i32) -> Option<String> {
        self.clan_crest.then(|| format!("crest{clan_id}"))
    }
    fn clan_alliance_name(&self, _clan_id: i32) -> String {
        self.alliance_name.clone()
    }
    fn clan_alliance_crest_texture(&self, clan_id: i32) -> Option<String> {
        self.alliance_crest.then(|| format!("allycrest{clan_id}"))
    }
    fn player_move_type(&self) -> i32 {
        self.move_type
    }
    fn player_environment(&self) -> i32 {
        self.environment
    }
    fn player_exp_rate(&self) -> f32 {
        self.exp_rate
    }
    fn skill_icon_name(&self, skill_id: i32, level: i32) -> String {
        format!("skillicon{skill_id}_{level}")
    }
    fn skill_name(&self, skill_id: i32, level: i32) -> String {
        format!("skill{skill_id}_{level}")
    }
    fn skill_description(&self, skill_id: i32, level: i32) -> String {
        format!("desc{skill_id}_{level}")
    }
    fn set_texture_with_object(&mut self, name: &str, texture: &str) {
        self.record(format!("SetTextureWithObject({name},{texture})"));
    }
    fn set_texture_size(&mut self, name: &str, width: i32, height: i32) {
        self.record(format!("SetTextureSize({name},{width},{height})"));
    }
    fn set_texture_ctrl_type(&mut self, name: &str, kind: i32) {
        self.record(format!("SetTextureCtrlType({name},{kind})"));
    }
    fn set_button_texture(&mut self, name: &str, normal: &str, down: &str, over: &str) {
        self.record(format!("SetButtonTexture({name},{normal},{down},{over})"));
    }
    fn set_tooltip_custom(&mut self, name: &str, tooltip: &CustomTooltip) {
        let text = tooltip.draw_list.first().map(|item| item.text.clone()).unwrap_or_default();
        self.record(format!("SetTooltipCustomType({name},{text})"));
    }
    fn get_tooltip_custom(&self, _name: &str) -> CustomTooltip {
        CustomTooltip::default()
    }
    fn get_name(&self, name: &str) -> String {
        self.get_text(name)
    }
    fn set_resize_frame_size(&mut self, name: &str, width: i32, height: i32) {
        self.record(format!("SetResizeFrameSize({name},{width},{height})"));
    }
    fn status_icon_set_item(&mut self, name: &str, row: i32, col: i32, info: &StatusIconInfo) {
        self.record(format!("SetItem({name},{row},{col},{})", info.name));
    }
    fn release_focus(&mut self, name: &str) {
        self.record(format!("ReleaseFocus({name})"));
    }
    fn tree_clear(&mut self, name: &str) {
        self.record(format!("TreeClear({name})"));
    }
    fn tree_show_scroll_bar(&mut self, name: &str, show: bool) {
        self.record(format!("TreeShowScrollBar({name},{show})"));
    }
    fn tree_insert_node(&mut self, name: &str, parent: &str, node: &TreeNodeInfo) -> String {
        self.record(format!("TreeInsertNode({name},{parent},{})", node.name));
        node.name.clone()
    }
    fn tree_insert_node_item(&mut self, name: &str, node: &str, item: &TreeNodeItemInfo) {
        self.record(format!("TreeInsertNodeItem({name},{node},{})", item.texture));
    }
    fn log(&mut self, message: &str) {
        self.record(format!("Log({message})"));
    }
    fn class_label_texture(&self, class_id: i32) -> String {
        format!("label{class_id}")
    }
}
