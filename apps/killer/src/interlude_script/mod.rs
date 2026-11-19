#![allow(dead_code)]
use killer_ui::color::Argb;

pub mod abnormal_status_wnd;
pub mod chat_wnd;
pub mod detail_status_wnd;
pub mod events;
pub mod host;
pub mod host_impl;
pub mod natives_table;
pub mod runtime;
pub mod vm_host;
pub mod vm_host_data;
pub mod vm_host_script;
pub mod vm_host_controls;
pub mod inventory_wnd;
pub mod menu_wnd;
pub mod minimap_wnd;
pub mod party_wnd;
pub mod radar_container_wnd;
pub mod radar_wnd;
pub mod shortcut_wnd;
pub mod status_wnd;
pub mod system_msg_wnd;
pub mod target_status_wnd;
pub mod ui_common_api;

pub const PARAM_DELIMITER: char = '\u{7f}';
pub const PARAM_VALUE_MAX: usize = 0x1000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Undefined,
    Left,
    Center,
    Right,
    MacroIcon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NameCtrlType {
    #[default]
    Normal,
    Item,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceSound {
    Click1,
    Click2,
    ClickFailed,
    Pickup,
    TrashBasket,
    WindowOpen,
    WindowClose,
    QuestTutorial,
    MinimapOpenClose,
    CooltimeEnd,
    Petition,
    StatusWndOpen,
    StatusWndClose,
    InvenWndOpen,
    InvenWndClose,
    MapWndOpen,
    MapWndClose,
    SystemWndOpen,
    SystemWndClose,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub nick_name: String,
    pub level: i32,
    pub class_id: i32,
    pub sub_class: i32,
    pub sp: i32,
    pub cur_hp: i32,
    pub max_hp: i32,
    pub cur_mp: i32,
    pub max_mp: i32,
    pub cur_cp: i32,
    pub max_cp: i32,
    pub cur_exp: i64,
    pub user_rank: i32,
    pub clan_id: i32,
    pub alliance_id: i32,
    pub carry_weight: i32,
    pub carring_weight: i32,
    pub physical_attack: i32,
    pub physical_defense: i32,
    pub hit_rate: i32,
    pub critical_rate: i32,
    pub physical_attack_speed: i32,
    pub magical_attack: i32,
    pub magic_defense: i32,
    pub physical_avoid: i32,
    pub water_max_speed: i32,
    pub water_min_speed: i32,
    pub air_max_speed: i32,
    pub air_min_speed: i32,
    pub ground_max_speed: i32,
    pub ground_min_speed: i32,
    pub non_attack_speed_modifier: f32,
    pub magic_casting_speed: i32,
    pub str: i32,
    pub dex: i32,
    pub con: i32,
    pub int: i32,
    pub wit: i32,
    pub men: i32,
    pub criminal_rate: i32,
    pub dual_count: i32,
    pub pk_count: i32,
    pub sociality: i32,
    pub remain_sulffrage: i32,
    pub hero: bool,
    pub nobless: bool,
    pub npc: bool,
    pub pet: bool,
    pub can_be_attacked: bool,
    pub loc: Vector,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct PetInfo {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub sp: i32,
    pub cur_hp: i32,
    pub max_hp: i32,
    pub cur_mp: i32,
    pub max_mp: i32,
    pub cur_exp: i64,
    pub max_exp: i64,
    pub min_exp: i64,
    pub carry_weight: i32,
    pub carring_weight: i32,
    pub physical_attack: i32,
    pub physical_defense: i32,
    pub hit_rate: i32,
    pub critical_rate: i32,
    pub physical_attack_speed: i32,
    pub magical_attack: i32,
    pub magic_defense: i32,
    pub physical_avoid: i32,
    pub moving_speed: i32,
    pub magic_casting_speed: i32,
    pub soul_shot_consume: i32,
    pub spirit_shot_consume: i32,
    pub fatigue: i32,
    pub max_fatigue: i32,
    pub is_pet_or_summoned: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SkillInfo {
    pub skill_name: String,
    pub skill_desc: String,
    pub skill_id: i32,
    pub skill_level: i32,
    pub operate_type: i32,
    pub mp_consume: i32,
    pub hp_consume: i32,
    pub cast_range: i32,
    pub cast_style: i32,
    pub hit_time: f32,
    pub is_used: bool,
    pub is_magic: i32,
    pub anim_name: String,
    pub skill_preset_name: String,
    pub tex_name: String,
    pub is_debuff: bool,
    pub enchant_name: String,
    pub enchant_desc: String,
    pub enchanted: i32,
    pub enchant_skill_level: i32,
    pub rumble_self: i32,
    pub rumble_target: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StatusIconInfo {
    pub name: String,
    pub icon_name: String,
    pub size: i32,
    pub description: String,
    pub back_tex: String,
    pub remain_time: i32,
    pub class_id: i32,
    pub level: i32,
    pub show: bool,
    pub short_item: bool,
    pub etc_item: bool,
    pub debuff: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ItemInfo {
    pub name: String,
    pub additional_name: String,
    pub icon_name: String,
    pub icon_name_ex1: String,
    pub icon_name_ex2: String,
    pub icon_name_ex3: String,
    pub icon_name_ex4: String,
    pub fore_texture: String,
    pub description: String,
    pub drag_src_name: String,
    pub drag_src_reserved: i32,
    pub macro_command: String,
    pub item_type: i32,
    pub item_sub_type: i32,
    pub server_id: i32,
    pub class_id: i32,
    pub item_num: i32,
    pub price: i32,
    pub level: i32,
    pub slot_bit_type: i32,
    pub weight: i32,
    pub material_type: i32,
    pub weapon_type: i32,
    pub physical_damage: i32,
    pub magical_damage: i32,
    pub physical_defense: i32,
    pub magical_defense: i32,
    pub shield_defense: i32,
    pub shield_defense_rate: i32,
    pub durability: i32,
    pub crystal_type: i32,
    pub random_damage: i32,
    pub critical: i32,
    pub hit_modify: i32,
    pub attack_speed: i32,
    pub mp_consume: i32,
    pub armor_type: i32,
    pub avoid_modify: i32,
    pub damaged: i32,
    pub enchanted: i32,
    pub mp_bonus: i32,
    pub soulshot_count: i32,
    pub spiritshot_count: i32,
    pub pop_msg_num: i32,
    pub body_part: i32,
    pub refinery_op1: i32,
    pub refinery_op2: i32,
    pub current_durability: i32,
    pub reserved: i32,
    pub default_price: i32,
    pub consume_type: i32,
    pub blessed: i32,
    pub all_item_count: i32,
    pub icon_index: i32,
    pub equipped: bool,
    pub recipe: bool,
    pub arrow: bool,
    pub show_count: bool,
    pub disabled: bool,
    pub is_lock: bool,
}

pub fn parse_value<'a>(param: &'a str, name: &str) -> Option<&'a str> {
    if name.is_empty() || param.is_empty() {
        return None;
    }
    let mut from = 0;
    loop {
        let found = from + param[from..].find(name)?;
        let after = found + name.len();
        if param[after..].starts_with('=') {
            let rest = &param[after + 1..];
            let (body, delimiter) = match rest.strip_prefix(PARAM_DELIMITER) {
                Some(body) => (body, PARAM_DELIMITER),
                None => (rest, ' '),
            };
            let end = body.char_indices().take(PARAM_VALUE_MAX).find(|(_, c)| *c == delimiter).map_or_else(
                || body.char_indices().nth(PARAM_VALUE_MAX).map_or(body.len(), |(index, _)| index),
                |(index, _)| index,
            );
            return Some(&body[..end]);
        }
        from = after;
    }
}

pub fn app_atoi(text: &str) -> i32 {
    let trimmed = text.trim_start();
    let (negative, digits) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, trimmed.strip_prefix('+').unwrap_or(trimmed)),
    };
    let mut value: i64 = 0;
    for c in digits.chars().take_while(char::is_ascii_digit) {
        value = (value * 10 + i64::from(c as u8 - b'0')).min(i64::from(i32::MAX) + 1);
    }
    let value = if negative { -value } else { value };
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

pub fn parse_int(param: &str, name: &str) -> Option<i32> {
    parse_value(param, name).map(app_atoi)
}

pub fn parse_string(param: &str, name: &str) -> Option<String> {
    parse_value(param, name).map(str::to_string)
}

pub fn parse_float(param: &str, name: &str) -> Option<f32> {
    parse_value(param, name).map(|value| {
        let trimmed = value.trim_start();
        let end = trimmed
            .char_indices()
            .find(|(index, c)| !(c.is_ascii_digit() || *c == '.' || (*index == 0 && (*c == '-' || *c == '+'))))
            .map_or(trimmed.len(), |(index, _)| index);
        trimmed[..end].parse().unwrap_or(0.0)
    })
}

pub fn param_add(param: &mut String, name: &str, value: &str) {
    if name.is_empty() || value.is_empty() {
        return;
    }
    if !param.is_empty() {
        param.push(' ');
    }
    param.push_str(name);
    param.push('=');
    param.push(PARAM_DELIMITER);
    param.push_str(value);
    param.push(PARAM_DELIMITER);
}

pub fn parse_int64(param: &str, name: &str) -> Option<i64> {
    parse_value(param, name).map(|value| {
        let trimmed = value.trim_start();
        let end = trimmed
            .char_indices()
            .find(|(index, c)| !(c.is_ascii_digit() || (*index == 0 && (*c == '-' || *c == '+'))))
            .map_or(trimmed.len(), |(index, _)| index);
        trimmed[..end].parse().unwrap_or(0)
    })
}


#[derive(Debug, Clone, PartialEq, Default)]
pub struct HennaInfo {
    pub henna_id: i32,
    pub class_id: i32,
    pub num: i32,
    pub fee: i32,
    pub can_use: i32,
    pub int_now: i32,
    pub int_change: i32,
    pub str_now: i32,
    pub str_change: i32,
    pub con_now: i32,
    pub con_change: i32,
    pub men_now: i32,
    pub men_change: i32,
    pub dex_now: i32,
    pub dex_change: i32,
    pub wit_now: i32,
    pub wit_change: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawItemType {
    #[default]
    Blank,
    Text,
    Texture,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DrawItemInfo {
    pub kind: DrawItemType,
    pub offset_x: i32,
    pub offset_y: i32,
    pub line_break: bool,
    pub height: i32,
    pub text_id: i32,
    pub text: String,
    pub color: Option<Argb>,
    pub draw_one_line: bool,
    pub texture_width: i32,
    pub texture_height: i32,
    pub texture_u_width: i32,
    pub texture_u_height: i32,
    pub texture: String,
    pub condition: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct CustomTooltip {
    pub minimum_width: i32,
    pub simple_line_count: i32,
    pub draw_list: Vec<DrawItemInfo>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeNodeInfo {
    pub name: String,
    pub offset_x: i32,
    pub offset_y: i32,
    pub show_button: i32,
    pub tooltip: CustomTooltip,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeNodeItemInfo {
    pub kind: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub line_break: bool,
    pub texture_width: i32,
    pub texture_height: i32,
    pub texture_u_width: i32,
    pub texture_u_height: i32,
    pub texture: String,
    pub text: String,
}

pub fn make_tooltip_simple_text(text: &str) -> CustomTooltip {
    CustomTooltip {
        draw_list: vec![DrawItemInfo { kind: DrawItemType::Text, draw_one_line: true, text: text.to_string(), ..Default::default() }],
        ..Default::default()
    }
}

pub fn rgb(r: i32, g: i32, b: i32) -> Argb {
    Argb(0xFF00_0000 | ((r.clamp(0, 255) as u32) << 16) | ((g.clamp(0, 255) as u32) << 8) | (b.clamp(0, 255) as u32))
}

pub fn uc_left(text: &str, count: usize) -> String {
    text.chars().take(count).collect()
}

pub fn uc_clamp(value: i32, min: i32, max: i32) -> i32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn uc_string_float(value: f32) -> String {
    format!("{value:.2}")
}

#[allow(clippy::too_many_arguments)]
pub trait UiApi {
    fn is_show_window(&self, name: &str) -> bool;
    fn show_window(&mut self, name: &str);
    fn hide_window(&mut self, name: &str);
    fn clear(&mut self, name: &str);
    fn get_rect(&self, name: &str) -> Rect;
    fn set_ui_timer(&mut self, name: &str, timer_id: i32, delay_ms: i32);
    fn kill_ui_timer(&mut self, name: &str, timer_id: i32);
    fn set_window_title(&mut self, name: &str, index: i32);
    fn set_window_title_by_text(&mut self, name: &str, text: &str);
    fn enable_window(&mut self, name: &str);
    fn disable_window(&mut self, name: &str);
    fn is_enable_window(&self, name: &str) -> bool;
    fn set_always_on_top(&mut self, name: &str, on: bool);
    fn set_alpha(&mut self, name: &str, alpha: i32, seconds: f32);
    fn move_by(&mut self, name: &str, dx: i32, dy: i32, seconds: f32);
    fn move_to(&mut self, name: &str, x: i32, y: i32);
    fn iconize(&mut self, name: &str, texture: &str, tooltip: i32);
    fn set_focus(&mut self, name: &str);
    fn is_focused(&self, name: &str) -> bool;
    fn set_window_size(&mut self, name: &str, width: i32, height: i32);
    fn set_window_size_rel(&mut self, name: &str, width_rate: f32, height_rate: f32, offset_width: i32, offset_height: i32);
    fn set_frame_size(&mut self, name: &str, width: i32, height: i32);
    fn set_anchor(&mut self, name: &str, anchor_window: &str, relative_point: &str, anchor_point: &str, x: i32, y: i32);
    fn clear_anchor(&mut self, name: &str);
    fn set_tooltip_text(&mut self, name: &str, text: &str);
    fn set_text(&mut self, name: &str, text: &str);
    fn set_text_color(&mut self, name: &str, color: Argb);
    fn set_text_align(&mut self, name: &str, align: TextAlign);
    fn set_int(&mut self, name: &str, value: i32);
    fn get_text(&self, name: &str) -> String;
    fn set_texture(&mut self, name: &str, texture: &str);
    fn set_uv(&mut self, name: &str, u: i32, v: i32);
    fn get_texture_name(&self, name: &str) -> String;
    fn set_name(&mut self, name: &str, text: &str, kind: NameCtrlType, align: TextAlign);
    fn set_name_with_color(&mut self, name: &str, text: &str, kind: NameCtrlType, align: TextAlign, color: Argb);
    fn set_point(&mut self, name: &str, current: i32, max: i32);
    fn set_point_percent(&mut self, name: &str, current: i64, min: i64, max: i64);
    fn set_point_exp(&mut self, name: &str, current: i64, level: i32);
    fn set_regen_info(&mut self, name: &str, duration: i32, ticks: i32, amount: f32);
    fn bar_set_value(&mut self, name: &str, max: i32, current: i32);
    fn bar_get_value(&self, name: &str) -> (i32, i32);
    fn checkbox_set_title(&mut self, name: &str, title: &str);
    fn checkbox_set_check(&mut self, name: &str, checked: bool);
    fn checkbox_is_checked(&self, name: &str) -> bool;
    fn checkbox_set_disable(&mut self, name: &str, disabled: bool);
    fn checkbox_is_disable(&self, name: &str) -> bool;
    fn update_shortcut(&mut self, name: &str, shortcut_id: i32);
    fn status_icon_add_row(&mut self, name: &str);
    fn status_icon_add_col(&mut self, name: &str, row: i32, info: &StatusIconInfo);
    fn status_icon_insert_row(&mut self, name: &str, row: i32);
    fn status_icon_insert_col(&mut self, name: &str, row: i32, col: i32, info: &StatusIconInfo);
    fn status_icon_row_count(&self, name: &str) -> i32;
    fn status_icon_col_count(&self, name: &str, row: i32) -> i32;
    fn status_icon_get_item(&self, name: &str, row: i32, col: i32) -> Option<StatusIconInfo>;
    fn status_icon_del_item(&mut self, name: &str, row: i32, col: i32);
    fn tab_init(&mut self, name: &str);
    fn tab_set_top_order(&mut self, name: &str, index: i32, send_message: bool);
    fn tab_get_top_index(&self, name: &str) -> i32;
    fn tab_set_disable(&mut self, name: &str, index: i32, disabled: bool);
    fn tab_merge(&mut self, name: &str, index: i32);
    fn chat_enable_texture(&mut self, name: &str, on: bool);
    fn chat_set_scroll_bar_position(&mut self, name: &str, x: i32, y: i32, height: i32);
    fn set_chat_message(&mut self, message: &str);
    fn combo_add_string(&mut self, name: &str, text: &str, reserved: i32);
    fn combo_clear(&mut self, name: &str);
    fn combo_get_selected_num(&self, name: &str) -> i32;
    fn combo_set_selected_num(&mut self, name: &str, index: i32);
    fn combo_get_string(&self, name: &str, index: i32) -> String;
    fn combo_get_reserved(&self, name: &str, index: i32) -> i32;
    fn edit_get_string(&self, name: &str) -> String;
    fn edit_set_string(&mut self, name: &str, text: &str);
    fn radar_add_target(&mut self, name: &str, x: i32, y: i32, z: i32);
    fn radar_delete_target(&mut self, name: &str, x: i32, y: i32, z: i32);
    fn radar_delete_all_target(&mut self, name: &str);
    fn radar_set_color(&mut self, name: &str, color: Argb, seconds: f32);
    fn minimap_adjust_map_view(&mut self, name: &str, loc: Vector, zoom_to_town: bool, use_grid_location: bool);
    fn minimap_init_position(&mut self, name: &str);
    fn minimap_add_target(&mut self, name: &str, loc: Vector);
    fn minimap_delete_target(&mut self, name: &str, loc: Vector);
    fn minimap_delete_all_target(&mut self, name: &str);
    fn minimap_set_show_quest(&mut self, name: &str, show: bool);
    fn minimap_set_ssq_status(&mut self, name: &str, status: i32);
    fn minimap_draw_grid_icon(&mut self, name: &str, icon: &str, dup_icon: &str, loc: Vector, refresh: bool, x_offset: i32, y_offset: i32, tooltip: &str);
    fn minimap_request_reduce_btn(&mut self, name: &str);
    fn minimap_is_overlapped(&self, name: &str, first_x: i32, first_y: i32, second_x: i32, second_y: i32) -> bool;
    fn minimap_delete_all_cursed_weapon_icon(&mut self, name: &str);
    fn register_event(&mut self, event_id: i32);
    fn register_state(&mut self, window: &str, state: &str);
    fn set_ui_state(&mut self, state: &str);
    fn get_ui_state(&self) -> String;
    fn execute_event(&mut self, event_id: i32, param: &str);
    fn get_option_bool(&self, section: &str, name: &str) -> bool;
    fn get_option_int(&self, section: &str, name: &str) -> i32;
    fn get_option_string(&self, section: &str, name: &str) -> String;
    fn set_option_bool(&mut self, section: &str, name: &str, value: bool);
    fn set_option_int(&mut self, section: &str, name: &str, value: i32);
    fn get_ini_bool(&self, section: &str, key: &str, file: &str) -> Option<i32>;
    fn get_system_string(&self, id: i32) -> String;
    fn get_system_message(&self, id: i32) -> String;
    fn make_full_system_msg(&self, message: &str, arg1: &str, arg2: &str) -> String;
    fn get_text_size(&self, text: &str) -> (i32, i32);
    fn play_sound(&mut self, sound: &str);
    fn play_console_sound(&mut self, sound: InterfaceSound);
    fn debug(&mut self, message: &str);
    fn get_player_info(&self) -> Option<UserInfo>;
    fn get_target_info(&self) -> Option<UserInfo>;
    fn get_user_info(&self, user_id: i32) -> Option<UserInfo>;
    fn get_pet_info(&self) -> Option<PetInfo>;
    fn get_skill_info(&self, skill_id: i32, level: i32) -> Option<SkillInfo>;
    fn get_player_position(&self) -> Vector;
    fn set_timer(&mut self, window: &str, timer_id: i32, delay_ms: i32);
    fn kill_timer(&mut self, window: &str, timer_id: i32);
    fn edit_add_string(&mut self, name: &str, text: &str);
    fn edit_release_focus(&mut self, name: &str);
    fn chat_add_string(&mut self, name: &str, text: &str, color: Argb);
    fn set_settled_wnd(&mut self, name: &str, settled: bool);
    fn get_window_size(&self, name: &str) -> (i32, i32);
    fn button_set_texture(&mut self, name: &str, fore: &str, back: &str, highlight: &str);
    fn set_tooltip_simple_text(&mut self, name: &str, text: &str);
    fn get_current_ime_lang(&self) -> i32;
    fn process_chat_message(&mut self, message: &str, chat_type: i32);
    fn is_same_chat_prefix(&self, chat_type: i32, prefix: &str) -> bool;
    fn get_chat_prefix(&self, chat_type: i32) -> String;
    fn add_system_message(&mut self, message: &str, color: Argb);
    fn toggle_msn_window(&mut self);
    fn execute_command(&mut self, command: &str);
    fn request_open_party_match(&mut self);
    fn script_on_send_packet_when_hiding(&mut self, window: &str);
    fn set_shortcut_page(&mut self, page: i32);
    fn request_cursed_weapon_list(&mut self);
    fn request_cursed_weapon_location(&mut self);
    fn get_quest_location(&self) -> Option<Vector>;
    fn get_party_member_count(&self) -> i32;
    fn get_party_member_location(&self, index: i32) -> Option<Vector>;
    fn get_current_zone_name(&self) -> String;
    fn get_ini_string(&self, section: &str, key: &str, file: &str) -> Option<String>;
    fn request_self_target(&mut self);
    fn request_target_cancel(&mut self);
    fn request_start_trade(&mut self, target_id: i32);
    fn request_invite_party(&mut self, name: &str);
    fn request_target_user(&mut self, server_id: i32);
    fn request_attack(&mut self, server_id: i32, loc: Vector);
    fn request_action(&mut self, server_id: i32, loc: Vector);
    fn request_assist(&mut self, server_id: i32, loc: Vector);
    fn is_pk_mode(&self) -> bool;
    fn get_class_type(&self, class_id: i32) -> String;
    fn get_class_str(&self, class_id: i32) -> String;
    fn get_class_icon_name(&self, class_id: i32) -> String;
    fn get_user_rank_string(&self, rank: i32) -> String;
    fn get_routing_string(&self, routing_type: i32) -> String;
    fn is_debuff(&self, skill_id: i32, level: i32) -> bool;
    fn dialog_get_string(&self) -> String;
    fn script_call(&mut self, window: &str, function: &str);
    fn script_get_int_array(&self, window: &str, variable: &str, index: i32) -> i32;
    fn target_id(&self) -> i32;
    fn target_name_color(&self, level_diff: i32) -> Argb;
    fn target_is_server_object(&self) -> bool;
    fn target_is_hp_showable_npc(&self) -> bool;
    fn static_object_name_id(&self, server_id: i32) -> i32;
    fn static_object_name(&self, name_id: i32) -> String;
    fn server_object_type(&self, server_id: i32) -> i32;
    fn static_object_show_hp(&self, server_id: i32) -> bool;
    fn server_object_hp(&self, server_id: i32) -> i32;
    fn server_object_max_hp(&self, server_id: i32) -> i32;
    fn party_member_name(&self, server_id: i32) -> String;
    fn player_id(&self) -> i32;
    fn pet_id(&self) -> i32;
    fn npc_property(&self, class_id: i32) -> Option<Vec<i32>>;
    fn clan_name(&self, clan_id: i32) -> String;
    fn clan_type(&self, user_id: i32) -> Option<i32>;
    fn clan_name_value(&self, clan_id: i32) -> Option<i32>;
    fn clan_crest_texture(&self, clan_id: i32) -> Option<String>;
    fn clan_alliance_name(&self, clan_id: i32) -> String;
    fn clan_alliance_crest_texture(&self, clan_id: i32) -> Option<String>;
    fn player_move_type(&self) -> i32;
    fn player_environment(&self) -> i32;
    fn player_exp_rate(&self) -> f32;
    fn skill_icon_name(&self, skill_id: i32, level: i32) -> String;
    fn skill_name(&self, skill_id: i32, level: i32) -> String;
    fn skill_description(&self, skill_id: i32, level: i32) -> String;
    fn set_texture_with_object(&mut self, name: &str, texture: &str);
    fn set_texture_size(&mut self, name: &str, width: i32, height: i32);
    fn set_texture_ctrl_type(&mut self, name: &str, kind: i32);
    fn set_button_texture(&mut self, name: &str, normal: &str, down: &str, over: &str);
    fn set_tooltip_custom(&mut self, name: &str, tooltip: &CustomTooltip);
    fn get_tooltip_custom(&self, name: &str) -> CustomTooltip;
    fn get_name(&self, name: &str) -> String;
    fn set_resize_frame_size(&mut self, name: &str, width: i32, height: i32);
    fn status_icon_set_item(&mut self, name: &str, row: i32, col: i32, info: &StatusIconInfo);
    fn release_focus(&mut self, name: &str);
    fn tree_clear(&mut self, name: &str);
    fn tree_show_scroll_bar(&mut self, name: &str, show: bool);
    fn tree_insert_node(&mut self, name: &str, parent: &str, node: &TreeNodeInfo) -> String;
    fn tree_insert_node_item(&mut self, name: &str, node: &str, item: &TreeNodeItemInfo);
    fn log(&mut self, message: &str);
    fn item_get_selected_num(&self, name: &str) -> i32;
    fn item_get_item_num(&self, name: &str) -> i32;
    fn item_clear_select(&mut self, name: &str);
    fn item_add(&mut self, name: &str, info: &ItemInfo);
    fn item_set(&mut self, name: &str, index: i32, info: &ItemInfo) -> bool;
    fn item_delete(&mut self, name: &str, index: i32);
    fn item_get_selected(&self, name: &str) -> Option<ItemInfo>;
    fn item_get(&self, name: &str, index: i32) -> Option<ItemInfo>;
    fn item_clear(&mut self, name: &str);
    fn item_find_with_server_id(&self, name: &str, server_id: i32) -> i32;
    fn item_swap(&mut self, name: &str, first: i32, second: i32);
    fn item_get_index_at(&self, name: &str, x: i32, y: i32, offset_x: i32, offset_y: i32) -> i32;
    fn item_set_disable_tex(&mut self, name: &str, texture: &str);
    fn item_set_row(&mut self, name: &str, row: i32);
    fn set_tooltip_string(&mut self, name: &str, text: &str);
    fn request_use_item(&mut self, server_id: i32);
    fn request_destroy_item(&mut self, server_id: i32, num: i32);
    fn request_drop_item(&mut self, server_id: i32, num: i32, location: Vector);
    fn request_unequip_item(&mut self, server_id: i32, slot_bit_type: i32);
    fn request_crystallize_item(&mut self, server_id: i32, num: i32);
    fn request_item_list(&mut self);
    fn request_open_minimap(&mut self);
    fn request_exit(&mut self);
    fn request_get_item_from_pet(&mut self, server_id: i32, num: i32, is_equip_item: bool);
    fn get_accessory_server_id(&self) -> Option<(i32, i32, i32, i32)>;
    fn get_class_step(&self, class_id: i32) -> i32;
    fn get_adena(&self) -> i32;
    fn is_key_down(&self, key: i32) -> bool;
    fn make_cost_string(&self, text: &str) -> String;
    fn convert_num_to_text(&self, text: &str) -> String;
    fn check_item_limit(&self, class_id: i32, count: i32) -> bool;
    fn get_click_location(&self) -> Vector;
    fn is_stackable_item(&self, consume_type: i32) -> bool;
    fn save_inventory_order(&mut self, order: &[i32]);
    fn load_inventory_order(&self) -> Option<Vec<i32>>;
    fn player_has_crystallize_ability(&self) -> bool;
    fn player_inventory_limit(&self) -> i32;
    fn item_is_crystallizable(&self, class_id: i32) -> bool;
    fn henna_info_count(&self) -> i32;
    fn henna_info(&self, index: i32) -> Option<(i32, i32)>;
    fn henna_item_name(&self, henna_id: i32) -> Option<String>;
    fn henna_description(&self, henna_id: i32) -> Option<String>;
    fn henna_icon_tex(&self, henna_id: i32) -> Option<String>;
    fn dialog_show(&mut self, kind: i32, message: &str, target: &str);
    fn dialog_hide(&mut self);
    fn dialog_set_id(&mut self, id: i32);
    fn dialog_set_param_int(&mut self, value: i32);
    fn dialog_set_reserved_int(&mut self, value: i32);
    fn dialog_set_reserved_int2(&mut self, value: i32);
    fn dialog_get_target(&self) -> String;
    fn dialog_get_edit_message(&self) -> String;
    fn dialog_get_id(&self) -> i32;
    fn dialog_get_reserved_int(&self) -> i32;
    fn dialog_get_reserved_int2(&self) -> i32;
    fn script_get_bool(&self, window: &str, var: &str) -> bool;
    fn script_get_int(&self, window: &str, var: &str) -> i32;
    fn class_label_texture(&self, class_id: i32) -> String;
}

pub trait WindowScript {
    fn on_load(&mut self, _api: &mut dyn UiApi) {}
    fn on_show(&mut self, _api: &mut dyn UiApi) {}
    fn on_hide(&mut self, _api: &mut dyn UiApi) {}
    fn on_tick(&mut self, _api: &mut dyn UiApi) {}
    fn on_event(&mut self, _api: &mut dyn UiApi, _event_id: i32, _param: &str) {}
    fn on_timer(&mut self, _api: &mut dyn UiApi, _timer_id: i32) {}
    fn on_minimize(&mut self, _api: &mut dyn UiApi) {}
    fn on_enter_state(&mut self, _api: &mut dyn UiApi, _previous: &str) {}
    fn on_exit_state(&mut self, _api: &mut dyn UiApi, _next: &str) {}
    fn on_default_position(&mut self, _api: &mut dyn UiApi) {}
    fn on_click_button(&mut self, _api: &mut dyn UiApi, _id: &str) {}
    fn on_tab_split(&mut self, _api: &mut dyn UiApi, _name: &str) {}
    fn on_tab_merge(&mut self, _api: &mut dyn UiApi, _name: &str) {}
    fn on_complete_edit_box(&mut self, _api: &mut dyn UiApi, _id: &str) {}
    fn on_frame_expand_click(&mut self, _api: &mut dyn UiApi, _expand: bool) {}
    fn on_drop_item(&mut self, _api: &mut dyn UiApi, _target: &str, _info: &ItemInfo, _x: i32, _y: i32) {}
    fn on_drop_item_source(&mut self, _api: &mut dyn UiApi, _target: &str, _info: &ItemInfo) {}
    fn on_select_item_with_handle(&mut self, _api: &mut dyn UiApi, _handle: &str, _index: i32) {}
    fn on_db_click_item_with_handle(&mut self, _api: &mut dyn UiApi, _handle: &str, _index: i32) {}
    fn on_r_click_item_with_handle(&mut self, _api: &mut dyn UiApi, _handle: &str, _index: i32) {}
}

#[cfg(test)]
pub mod mock;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_value_follows_l2paramparser_1003fc10() {
        assert_eq!(parse_value("Name=abc Level=12 ", "Level"), Some("12"));
        assert_eq!(parse_value("Name=abc Level=12", "Level"), Some("12"));
        assert_eq!(parse_value("NameLevel Level=7 ", "Level"), Some("7"));
        assert_eq!(parse_value("Text=\u{7f}two words\u{7f} Next=1", "Text"), Some("two words"));
        assert_eq!(parse_value("Name=abc", "Level"), None);
        assert_eq!(parse_int("HP=12x Max=30", "HP"), Some(12));
        assert_eq!(parse_int("HP=-5 ", "HP"), Some(-5));
        assert_eq!(parse_string("A=1 B=xyz", "B"), Some("xyz".into()));
        assert_eq!(parse_float("X=1.5 Y=2", "X"), Some(1.5));
        assert_eq!(parse_int64("Exp=12345678901 ", "Exp"), Some(12_345_678_901));
        assert_eq!(parse_value("", "A"), None);
        assert_eq!(parse_value("A=1", ""), None);
    }

    #[test]
    fn param_add_follows_l2paramparser_append_1003fb20() {
        let mut param = String::new();
        param_add(&mut param, "MsgType", "1");
        param_add(&mut param, "Msg", "two words");
        param_add(&mut param, "", "x");
        param_add(&mut param, "Empty", "");
        assert_eq!(param, "MsgType=\u{7f}1\u{7f} Msg=\u{7f}two words\u{7f}");
        assert_eq!(parse_string(&param, "Msg"), Some("two words".into()));
        assert_eq!(parse_int(&param, "MsgType"), Some(1));
    }
}
