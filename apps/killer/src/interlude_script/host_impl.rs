use std::collections::HashMap;

use killer_ui::color::Argb;

use super::host::{EventRegistry, Overlay};
use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ScriptRequest {
    SelfTarget,
    TargetCancel,
    TargetUser { server_id: i32 },
    Attack { server_id: i32, loc: Vector },
    Action { server_id: i32, loc: Vector },
    Assist { server_id: i32, loc: Vector },
    UseItem { server_id: i32 },
    DestroyItem { server_id: i32, num: i32 },
    DropItem { server_id: i32, num: i32, loc: Vector },
    UnequipItem { server_id: i32, slot_bit_type: i32 },
    CrystallizeItem { server_id: i32, num: i32 },
    ItemList,
    SkillList,
    QuestList,
    DestroyQuest { quest_id: i32 },
    RestartPoint { kind: i32 },
    Exit,
    StartTrade { target_id: i32 },
    AddTradeItem { server_id: i32, num: i32 },
    TradeDone { done: bool },
    AnswerTrade { ok: bool },
    SellItems { merchant: i32, items: Vec<(i32, i32, i32)> },
    BuyItems { merchant: i32, items: Vec<(i32, i32)> },
    BuySeed { merchant: i32, items: Vec<(i32, i32)> },
    ProcureCrop { merchant: i32, items: Vec<(i32, i32, i32)> },
    PreviewItem { merchant: i32, npc: i32, items: Vec<i32> },
    WarehouseDeposit { items: Vec<(i32, i32)> },
    WarehouseWithdraw { items: Vec<(i32, i32)> },
    PackageSend { target_id: i32, items: Vec<(i32, i32)> },
    PackageSendableItemList { target_id: i32 },
    MultiSellChoose { shop_id: i32, item_id: i32, item_count: i32, refinery_op1: i32 },
    QuitPrivateShop { kind: String },
    InviteParty { name: String },
    PartyLeave,
    PartyDismiss { name: String },
    AcquireSkillInfo { id: i32, level: i32, kind: i32 },
    AcquireSkill { id: i32, level: i32, kind: i32 },
    ExEnchantSkillInfo { id: i32, level: i32 },
    ExEnchantSkill { id: i32, level: i32 },
    EnchantItem { server_id: i32 },
    Bypass { command: String },
    LinkHtml { link: String },
    BbsBoard,
    OpenMinimap,
    HennaItemList,
    HennaItemInfo { henna_id: i32 },
    HennaEquip { henna_id: i32 },
    HennaUnequipList,
    HennaUnequipInfo { henna_id: i32 },
    HennaUnequip { henna_id: i32 },
    RaidRecord,
    ObserverModeEnd,
    FishRanking,
    PetUseItem { server_id: i32 },
    GiveItemToPet { server_id: i32, num: i32 },
    GetItemFromPet { server_id: i32, num: i32, is_equip_item: bool },
    PetInventoryItemList,
    ChangePetName { name: String },
    MacroList,
    MakeMacro { macro_id: i32, name: String, icon_name: String, icon_num: i32, description: String },
    DeleteMacro { macro_id: i32 },
    UseMacro { macro_id: i32 },
    Petition { message: String, kind: i32 },
    PetitionCancel,
    PetitionFeedBack { rate: i32, message: String },
    GmCommand { param: String },
    SnoopEnd { snoop_id: i32 },
    OlympiadMatchList,
    OlympiadObserverEnd,
    WriteHeroWords { words: String },
    CursedWeaponList,
    CursedWeaponLocation,
    OpenPartyMatch,
    PartyRoomList { page: i32, location_filter: i32, level_filter: i32 },
    PartyMatchWaitList { page: i32, min_level: i32, max_level: i32, room_type: i32 },
    JoinPartyRoom { room_number: i32 },
    JoinPartyRoomAuto { page: i32, location_filter: i32, level_filter: i32 },
    ManagePartyRoom { room_number: i32, max_members: i32, min_level: i32, max_level: i32, title: String },
    DismissPartyRoom { room_number: i32 },
    WithdrawPartyRoom { room_number: i32 },
    BanFromPartyRoom { member_id: i32 },
    AskJoinPartyRoom { name: String },
    ExitPartyMatchingWaitingRoom,
    CommandChannelInfo,
    CommandChannelWithdraw,
    CommandChannelBanParty { party_master_name: String },
    CommandChannelPartyMembersInfo { master_id: i32 },
    RecipeBookOpen { kind: i32 },
    RecipeItemDelete { server_id: i32 },
    RecipeItemMakeInfo { server_id: i32 },
    RecipeItemMakeSelf { recipe_id: i32 },
    RecipeShopMakeInfo { server_id: i32, recipe_id: i32 },
    RecipeShopMakeDo { merchant_id: i32, recipe_id: i32, adena: i32 },
    RecipeShopSellList { server_id: i32 },
    RecipeShopMessageSet { message: String },
    RecipeShopListSet { items: Vec<(i32, i32)> },
    RecipeShopManageQuit,
    Refine { target_item: i32, refiner_item: i32, gem_stone: i32, gem_stone_count: i32 },
    RefineCancel { cancel_item: i32 },
    SsqStatus { status_id: i32 },
    CastleSiegeAttackerList { castle_id: i32 },
    CastleSiegeDefenderList { castle_id: i32 },
    JoinCastleSiege { castle_id: i32, is_attacker: i32, is_register: i32 },
    ConfirmCastleSiegeWaitingList { castle_id: i32, clan_id: i32, is_register: i32 },
    SetCastleSiegeTime { castle_id: i32, time_id: i32 },
    DuelStart { target_name: String, duel_type: i32 },
    DuelAnswerStart { duel_type: i32, option: i32, answer: i32 },
    DuelSurrender,
    ClanMemberInfo { kind: i32, name: String },
    ClanGradeList,
    ClanChangeGrade { name: String, grade: i32 },
    ClanAssignPupil { master: String, pupil: String },
    ClanDeletePupil { master: String, pupil: String },
    ClanLeave { clan_name: String, clan_type: i32 },
    ClanExpelMember { clan_type: i32, name: String },
    ClanAskJoin { id: i32, clan_type: i32 },
    ClanDeclareWar,
    ClanWithdrawWar,
    ClanWarList { page: i32, state: i32 },
    ClanReorganizeMember { kind: i32, member_name: String, clan_type: i32, target_member_name: String },
    ClanChangeNickName { name: String, nick_name: String },
    ClanRegisterCrest,
    ClanUnregisterCrest,
    ClanRegisterEmblem,
    ClanUnregisterEmblem,
    SetSeed { manor_id: i32, seeds: Vec<(i32, i32)> },
    SetCrop { manor_id: i32, crops: Vec<(i32, i32)> },
    ProcureCropList { crops: Vec<(i32, i32, i32)> },
    LoadAllItem,
    ActionList { kind: i32 },
    ChatLine { message: String, kind: i32 },
    Command { command: String },
    Unmapped { class: String, function: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum HostEvent {
    Request(ScriptRequest),
    ShowWindow(String),
    HideWindow(String),
    Clear(String),
    SetTimer(String, i32, i32),
    KillTimer(String, i32),
    TabTop(String, i32, bool),
    ExecuteEvent(i32, String),
    Sound(String),
    ConsoleSound(InterfaceSound),
    ScriptCall(String, String),
}

#[derive(Default)]
pub struct ScriptHost {
    pub current: String,
    pub overlay: Overlay,
    pub events: EventRegistry,
    pub queue: Vec<HostEvent>,
    pub initial_shown: HashMap<String, bool>,
    pub focus: Option<String>,
    pub options: Option<killer_ui::ini::IniFile>,
    pub option_writes: Vec<(String, String, String)>,
    pub ini_values: HashMap<(String, String, String), String>,
    pub sysstrings: HashMap<u32, String>,
    pub player: Option<UserInfo>,
    pub target: Option<UserInfo>,
    pub pet: Option<PetInfo>,
    pub percent_points: HashMap<String, (i64, i64, i64)>,
    pub exp_points: HashMap<String, (i64, i32)>,
    pub regen: HashMap<String, (i32, i32, f32)>,
    pub status_grids: HashMap<String, Vec<Vec<StatusIconInfo>>>,
    pub script_bools: HashMap<(String, String), bool>,
    pub script_ints: HashMap<(String, String), i32>,
    pub chat_scroll_bars: HashMap<String, (i32, i32, i32)>,
    pub chat_lines: HashMap<String, Vec<(Argb, String)>>,
    pub unhandled: Vec<&'static str>,
    pub unhandled_natives: Vec<String>,
    pub script_objects: HashMap<String, usize>,
    pub language: String,
    pub tables: Option<std::sync::Arc<killer_assets::client::DatTables>>,
    pub own_systemmsg: HashMap<u32, String>,
    pub own_quests: HashMap<(i32, i32), l2_dat::questname::QuestName>,
    pub quest_zones: HashMap<(i32, i32), i32>,
    pub users: HashMap<i32, UserInfo>,
    pub party_members: Vec<i32>,
    pub party_locations: Vec<Vector>,
    pub quest_location: Option<Vector>,
    pub zone_name: String,
    pub pk_mode: bool,
    pub clans: HashMap<i32, super::vm_host_data::ClanData>,
    pub static_objects: HashMap<i32, super::vm_host_data::StaticObjectData>,
    pub hennas: Vec<HennaInfo>,
    pub game_tips: Vec<super::vm_host_data::GameTipData>,
    pub macros: Vec<super::vm_host_data::MacroData>,
    pub inventory_limit: i32,
    pub crystallize_ability: bool,
    pub recipe_shop_msg: String,
    pub player_move_type: i32,
    pub player_environment: i32,
    pub target_static_object: Option<i32>,
    pub target_pet: bool,
    pub target_can_be_attacked: bool,
    pub target_hp_showable: bool,
    pub item_cursor: usize,
    pub skill_cursor: usize,
    pub npc_cursor: usize,
    pub quest_cursor: usize,
    pub controls: super::vm_host_controls::ControlState,
}

impl ScriptHost {
    fn skip(&mut self, native: &'static str) {
        if !self.unhandled.contains(&native) {
            self.unhandled.push(native);
        }
    }
}

impl UiApi for ScriptHost {
    fn is_show_window(&self, name: &str) -> bool {
        self.overlay.shown.get(name).copied().or_else(|| self.initial_shown.get(name).copied()).unwrap_or(false)
    }
    fn show_window(&mut self, name: &str) {
        log::debug!("script {}: ShowWindow({name})", self.current);
        self.overlay.shown.insert(name.to_string(), true);
        self.queue.push(HostEvent::ShowWindow(name.to_string()));
    }
    fn hide_window(&mut self, name: &str) {
        log::debug!("script {}: HideWindow({name})", self.current);
        self.overlay.shown.insert(name.to_string(), false);
        self.queue.push(HostEvent::HideWindow(name.to_string()));
    }
    fn clear(&mut self, name: &str) {
        self.queue.push(HostEvent::Clear(name.to_string()));
    }
    fn get_rect(&self, name: &str) -> Rect {
        self.overlay.rects.get(name).copied().unwrap_or_default()
    }
    fn set_ui_timer(&mut self, name: &str, timer_id: i32, delay_ms: i32) {
        self.queue.push(HostEvent::SetTimer(name.to_string(), timer_id, delay_ms));
    }
    fn kill_ui_timer(&mut self, name: &str, timer_id: i32) {
        self.queue.push(HostEvent::KillTimer(name.to_string(), timer_id));
    }
    fn set_window_title(&mut self, name: &str, index: i32) {
        let title = self.sysstrings.get(&(index as u32)).cloned().unwrap_or_default();
        self.control_set_window_title(name, &title);
    }
    fn set_window_title_by_text(&mut self, name: &str, text: &str) {
        self.control_set_window_title(name, text);
    }
    fn enable_window(&mut self, name: &str) {
        self.overlay.disabled.insert(name.to_string(), false);
    }
    fn disable_window(&mut self, name: &str) {
        self.overlay.disabled.insert(name.to_string(), true);
    }
    fn is_enable_window(&self, name: &str) -> bool {
        !self.overlay.disabled.get(name).copied().unwrap_or(false)
    }
    fn set_always_on_top(&mut self, name: &str, on: bool) {
        self.control_set_style(name, super::vm_host_controls::STYLE_ALWAYS_ON_TOP, on);
    }
    fn set_alpha(&mut self, name: &str, alpha: i32, _seconds: f32) {
        self.overlay.alphas.insert(name.to_string(), alpha);
    }
    fn move_by(&mut self, _name: &str, _dx: i32, _dy: i32, _seconds: f32) {
        self.skip("move_by");
    }
    fn move_to(&mut self, name: &str, x: i32, y: i32) {
        self.overlay.moves.insert(name.to_string(), (x, y));
    }
    fn iconize(&mut self, name: &str, texture: &str, tooltip: i32) {
        self.controls.icons.insert(name.to_string(), (texture.to_string(), tooltip));
    }
    fn set_focus(&mut self, name: &str) {
        self.focus = Some(name.to_string());
    }
    fn is_focused(&self, name: &str) -> bool {
        self.focus.as_deref() == Some(name)
    }
    fn set_window_size(&mut self, name: &str, width: i32, height: i32) {
        self.overlay.sizes.insert(name.to_string(), (width, height));
    }
    fn set_window_size_rel(&mut self, name: &str, width_rate: f32, height_rate: f32, offset_width: i32, offset_height: i32) {
        self.controls.size_rels.insert(name.to_string(), (width_rate, height_rate, offset_width, offset_height));
    }
    fn set_frame_size(&mut self, _name: &str, _width: i32, _height: i32) {
        self.skip("set_frame_size");
    }
    fn set_anchor(&mut self, name: &str, anchor_window: &str, relative_point: &str, anchor_point: &str, x: i32, y: i32) {
        self.overlay.anchors.insert(name.to_string(), (anchor_window.to_string(), relative_point.to_string(), anchor_point.to_string(), x, y));
    }
    fn clear_anchor(&mut self, name: &str) {
        self.overlay.anchors.remove(name);
        self.overlay.cleared_anchors.push(name.to_string());
    }
    fn set_tooltip_text(&mut self, _name: &str, _text: &str) {
        self.skip("set_tooltip_text");
    }
    fn set_text(&mut self, name: &str, text: &str) {
        self.overlay.texts.insert(name.to_string(), text.to_string());
    }
    fn set_text_color(&mut self, name: &str, color: Argb) {
        self.overlay.text_colors.insert(name.to_string(), color);
    }
    fn set_text_align(&mut self, _name: &str, _align: TextAlign) {
        self.skip("set_text_align");
    }
    fn set_int(&mut self, name: &str, value: i32) {
        self.overlay.texts.insert(name.to_string(), value.to_string());
    }
    fn get_text(&self, name: &str) -> String {
        self.overlay.texts.get(name).cloned().unwrap_or_default()
    }
    fn set_texture(&mut self, name: &str, texture: &str) {
        self.overlay.textures.insert(name.to_string(), texture.to_string());
    }
    fn set_uv(&mut self, name: &str, u: i32, v: i32) {
        self.controls.texture_uv.insert(name.to_string(), (u, v));
    }
    fn get_texture_name(&self, name: &str) -> String {
        self.overlay.textures.get(name).cloned().unwrap_or_default()
    }
    fn set_name(&mut self, name: &str, text: &str, kind: NameCtrlType, align: TextAlign) {
        self.overlay.names.insert(name.to_string(), (text.to_string(), kind, align, None));
    }
    fn set_name_with_color(&mut self, name: &str, text: &str, kind: NameCtrlType, align: TextAlign, color: Argb) {
        self.overlay.names.insert(name.to_string(), (text.to_string(), kind, align, Some(color)));
    }
    fn set_point(&mut self, name: &str, current: i32, max: i32) {
        self.overlay.points.insert(name.to_string(), (current, max));
    }
    fn set_point_percent(&mut self, name: &str, current: i64, min: i64, max: i64) {
        self.percent_points.insert(name.to_string(), (current, min, max));
    }
    fn set_point_exp(&mut self, name: &str, current: i64, level: i32) {
        self.exp_points.insert(name.to_string(), (current, level));
    }
    fn set_regen_info(&mut self, name: &str, duration: i32, ticks: i32, amount: f32) {
        self.regen.insert(name.to_string(), (duration, ticks, amount));
    }
    fn bar_set_value(&mut self, name: &str, max: i32, current: i32) {
        self.overlay.points.insert(name.to_string(), (current, max));
    }
    fn bar_get_value(&self, name: &str) -> (i32, i32) {
        self.overlay.points.get(name).map_or((0, 0), |(current, max)| (*max, *current))
    }
    fn checkbox_set_title(&mut self, _name: &str, _title: &str) {
        self.skip("checkbox_set_title");
    }
    fn checkbox_set_check(&mut self, name: &str, checked: bool) {
        self.overlay.checks.insert(name.to_string(), checked);
    }
    fn checkbox_is_checked(&self, name: &str) -> bool {
        self.overlay.checks.get(name).copied().unwrap_or(false)
    }
    fn checkbox_set_disable(&mut self, name: &str, disabled: bool) {
        self.overlay.disabled.insert(name.to_string(), disabled);
    }
    fn checkbox_is_disable(&self, name: &str) -> bool {
        self.overlay.disabled.get(name).copied().unwrap_or(false)
    }
    fn update_shortcut(&mut self, _name: &str, _shortcut_id: i32) {
        self.skip("update_shortcut");
    }
    fn status_icon_add_row(&mut self, name: &str) {
        self.status_grids.entry(name.to_string()).or_default().push(Vec::new());
    }
    fn status_icon_add_col(&mut self, name: &str, row: i32, info: &StatusIconInfo) {
        if let Some(cells) = self.status_grids.entry(name.to_string()).or_default().get_mut(row.max(0) as usize) {
            cells.push(info.clone());
        }
    }
    fn status_icon_insert_row(&mut self, name: &str, row: i32) {
        let grid = self.status_grids.entry(name.to_string()).or_default();
        let at = (row.max(0) as usize).min(grid.len());
        grid.insert(at, Vec::new());
    }
    fn status_icon_insert_col(&mut self, name: &str, row: i32, col: i32, info: &StatusIconInfo) {
        if let Some(cells) = self.status_grids.entry(name.to_string()).or_default().get_mut(row.max(0) as usize) {
            let at = (col.max(0) as usize).min(cells.len());
            cells.insert(at, info.clone());
        }
    }
    fn status_icon_row_count(&self, name: &str) -> i32 {
        self.status_grids.get(name).map_or(0, |grid| grid.len() as i32)
    }
    fn status_icon_col_count(&self, name: &str, row: i32) -> i32 {
        self.status_grids.get(name).and_then(|grid| grid.get(row.max(0) as usize)).map_or(0, |cells| cells.len() as i32)
    }
    fn status_icon_get_item(&self, name: &str, row: i32, col: i32) -> Option<StatusIconInfo> {
        self.status_grids.get(name).and_then(|grid| grid.get(row.max(0) as usize)).and_then(|cells| cells.get(col.max(0) as usize)).cloned()
    }
    fn status_icon_del_item(&mut self, name: &str, row: i32, col: i32) {
        if let Some(cells) = self.status_grids.get_mut(name).and_then(|grid| grid.get_mut(row.max(0) as usize))
            && (col.max(0) as usize) < cells.len()
        {
            cells.remove(col.max(0) as usize);
        }
    }
    fn tab_init(&mut self, _name: &str) {
        self.skip("tab_init");
    }
    fn tab_set_top_order(&mut self, name: &str, index: i32, send_message: bool) {
        self.overlay.tabs.insert(name.to_string(), index);
        self.queue.push(HostEvent::TabTop(name.to_string(), index, send_message));
    }
    fn tab_get_top_index(&self, name: &str) -> i32 {
        self.overlay.tabs.get(name).copied().unwrap_or(0)
    }
    fn tab_set_disable(&mut self, _name: &str, _index: i32, _disabled: bool) {
        self.skip("tab_set_disable");
    }
    fn tab_merge(&mut self, name: &str, index: i32) {
        self.controls.tab_merges.push((name.to_string(), index));
    }
    fn chat_enable_texture(&mut self, _name: &str, _on: bool) {
        self.skip("chat_enable_texture");
    }
    fn chat_set_scroll_bar_position(&mut self, name: &str, x: i32, y: i32, height: i32) {
        self.chat_scroll_bars.insert(name.to_string(), (x, y, height));
    }
    fn set_chat_message(&mut self, message: &str) {
        let mut param = String::new();
        param_add(&mut param, "String", message);
        self.queue.push(HostEvent::ExecuteEvent(560, param));
    }
    fn combo_add_string(&mut self, name: &str, text: &str, reserved: i32) {
        if !text.is_empty() {
            self.control_combo_add(
                name,
                super::vm_host_controls::ComboEntry { sysstring: -1, sysmsg: -1, text: text.to_string(), reserved },
            );
        }
    }
    fn combo_clear(&mut self, name: &str) {
        self.control_combo_clear(name);
    }
    fn combo_get_selected_num(&self, name: &str) -> i32 {
        self.control_combo_selected_num(name)
    }
    fn combo_set_selected_num(&mut self, name: &str, index: i32) {
        self.control_combo_select(name, index);
    }
    fn combo_get_string(&self, name: &str, index: i32) -> String {
        self.control_combo_string(name, index)
    }
    fn combo_get_reserved(&self, name: &str, index: i32) -> i32 {
        self.control_combo_reserved(name, index)
    }
    fn edit_get_string(&self, name: &str) -> String {
        self.control_edit_string(name)
    }
    fn edit_set_string(&mut self, name: &str, text: &str) {
        self.control_edit_set(name, text);
    }
    fn radar_add_target(&mut self, _name: &str, _x: i32, _y: i32, _z: i32) {
        self.skip("radar_add_target");
    }
    fn radar_delete_target(&mut self, _name: &str, _x: i32, _y: i32, _z: i32) {
        self.skip("radar_delete_target");
    }
    fn radar_delete_all_target(&mut self, _name: &str) {
        self.skip("radar_delete_all_target");
    }
    fn radar_set_color(&mut self, _name: &str, _color: Argb, _seconds: f32) {
        self.skip("radar_set_color");
    }
    fn minimap_adjust_map_view(&mut self, _name: &str, _loc: Vector, _zoom_to_town: bool, _use_grid_location: bool) {
        self.skip("minimap_adjust_map_view");
    }
    fn minimap_init_position(&mut self, _name: &str) {
        self.skip("minimap_init_position");
    }
    fn minimap_add_target(&mut self, _name: &str, _loc: Vector) {
        self.skip("minimap_add_target");
    }
    fn minimap_delete_target(&mut self, _name: &str, _loc: Vector) {
        self.skip("minimap_delete_target");
    }
    fn minimap_delete_all_target(&mut self, _name: &str) {
        self.skip("minimap_delete_all_target");
    }
    fn minimap_set_show_quest(&mut self, _name: &str, _show: bool) {
        self.skip("minimap_set_show_quest");
    }
    fn minimap_set_ssq_status(&mut self, _name: &str, _status: i32) {
        self.skip("minimap_set_ssq_status");
    }
    fn minimap_draw_grid_icon(&mut self, _name: &str, _icon: &str, _dup_icon: &str, _loc: Vector, _refresh: bool, _x_offset: i32, _y_offset: i32, _tooltip: &str) {
        self.skip("minimap_draw_grid_icon");
    }
    fn minimap_request_reduce_btn(&mut self, _name: &str) {
        self.skip("minimap_request_reduce_btn");
    }
    fn minimap_is_overlapped(&self, _name: &str, _first_x: i32, _first_y: i32, _second_x: i32, _second_y: i32) -> bool {
        false
    }
    fn minimap_delete_all_cursed_weapon_icon(&mut self, _name: &str) {
        self.skip("minimap_delete_all_cursed_weapon_icon");
    }
    fn register_event(&mut self, event_id: i32) {
        self.events.register(&self.current, event_id);
    }
    fn register_state(&mut self, _window: &str, _state: &str) {
        self.skip("register_state");
    }
    fn set_ui_state(&mut self, _state: &str) {
        self.skip("set_ui_state");
    }
    fn get_ui_state(&self) -> String {
        String::new()
    }
    fn execute_event(&mut self, event_id: i32, param: &str) {
        self.queue.push(HostEvent::ExecuteEvent(event_id, param.to_string()));
    }
    fn get_option_bool(&self, section: &str, name: &str) -> bool {
        self.options.as_ref().and_then(|ini| ini.bool(section, name)).unwrap_or(false)
    }
    fn get_option_int(&self, section: &str, name: &str) -> i32 {
        self.options.as_ref().and_then(|ini| ini.int(section, name)).unwrap_or(0)
    }
    fn get_option_string(&self, section: &str, name: &str) -> String {
        self.options.as_ref().and_then(|ini| ini.get(section, name)).unwrap_or_default().to_string()
    }
    fn set_option_bool(&mut self, section: &str, name: &str, value: bool) {
        self.option_writes.push((section.to_string(), name.to_string(), value.to_string()));
    }
    fn set_option_int(&mut self, section: &str, name: &str, value: i32) {
        self.option_writes.push((section.to_string(), name.to_string(), value.to_string()));
    }
    fn get_ini_bool(&self, section: &str, key: &str, file: &str) -> Option<i32> {
        self.ini_values.get(&(file.to_string(), section.to_string(), key.to_string())).map(|value| i32::from(value.eq_ignore_ascii_case("true") || value == "1"))
    }
    fn get_system_string(&self, id: i32) -> String {
        self.sysstrings.get(&u32::try_from(id).unwrap_or(0)).cloned().unwrap_or_default()
    }
    fn get_system_message(&self, _id: i32) -> String {
        String::new()
    }
    fn make_full_system_msg(&self, _message: &str, _arg1: &str, _arg2: &str) -> String {
        String::new()
    }
    fn get_text_size(&self, _text: &str) -> (i32, i32) {
        Default::default()
    }
    fn play_sound(&mut self, sound: &str) {
        self.queue.push(HostEvent::Sound(sound.to_string()));
    }
    fn play_console_sound(&mut self, sound: InterfaceSound) {
        self.queue.push(HostEvent::ConsoleSound(sound));
    }
    fn debug(&mut self, message: &str) {
        log::debug!("script {}: {message}", self.current);
    }
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
        self.queue.push(HostEvent::SetTimer(window.to_string(), timer_id, delay_ms));
    }
    fn kill_timer(&mut self, window: &str, timer_id: i32) {
        self.queue.push(HostEvent::KillTimer(window.to_string(), timer_id));
    }
    fn edit_add_string(&mut self, name: &str, text: &str) {
        self.control_edit_add(name, text);
    }
    fn edit_release_focus(&mut self, _name: &str) {
        self.skip("edit_release_focus");
    }
    fn chat_add_string(&mut self, name: &str, text: &str, color: Argb) {
        self.chat_lines.entry(name.to_string()).or_default().push((color, text.to_string()));
    }
    fn set_settled_wnd(&mut self, name: &str, settled: bool) {
        self.control_set_style(name, super::vm_host_controls::STYLE_SETTLED, settled);
    }
    fn get_window_size(&self, name: &str) -> (i32, i32) {
        self.overlay.sizes.get(name).copied().or_else(|| self.overlay.rects.get(name).map(|rect| (rect.width, rect.height))).unwrap_or((0, 0))
    }
    fn button_set_texture(&mut self, name: &str, fore: &str, back: &str, highlight: &str) {
        let textures = super::vm_host_controls::ButtonTextures {
            fore: fore.to_string(),
            back: back.to_string(),
            highlight: highlight.to_string(),
        };
        self.controls.button_textures.insert(name.to_string(), textures);
    }
    fn set_tooltip_simple_text(&mut self, _name: &str, _text: &str) {
        self.skip("set_tooltip_simple_text");
    }
    fn get_current_ime_lang(&self) -> i32 {
        0
    }
    fn process_chat_message(&mut self, message: &str, chat_type: i32) {
        self.queue.push(HostEvent::Request(ScriptRequest::ChatLine { message: message.to_string(), kind: chat_type }));
    }
    fn is_same_chat_prefix(&self, _chat_type: i32, _prefix: &str) -> bool {
        false
    }
    fn get_chat_prefix(&self, _chat_type: i32) -> String {
        String::new()
    }
    fn add_system_message(&mut self, _message: &str, _color: Argb) {
        self.skip("add_system_message");
    }
    fn toggle_msn_window(&mut self) {
        self.skip("toggle_msn_window");
    }
    fn execute_command(&mut self, command: &str) {
        self.queue.push(HostEvent::Request(ScriptRequest::Command { command: command.to_string() }));
    }
    fn request_open_party_match(&mut self) {
        self.skip("request_open_party_match");
    }
    fn script_on_send_packet_when_hiding(&mut self, _window: &str) {
        self.skip("script_on_send_packet_when_hiding");
    }
    fn set_shortcut_page(&mut self, _page: i32) {
        self.skip("set_shortcut_page");
    }
    fn request_cursed_weapon_list(&mut self) {
        self.skip("request_cursed_weapon_list");
    }
    fn request_cursed_weapon_location(&mut self) {
        self.skip("request_cursed_weapon_location");
    }
    fn get_quest_location(&self) -> Option<Vector> {
        None
    }
    fn get_party_member_count(&self) -> i32 {
        0
    }
    fn get_party_member_location(&self, _index: i32) -> Option<Vector> {
        None
    }
    fn get_current_zone_name(&self) -> String {
        String::new()
    }
    fn get_ini_string(&self, section: &str, key: &str, file: &str) -> Option<String> {
        self.ini_values.get(&(file.to_string(), section.to_string(), key.to_string())).cloned()
    }
    fn request_self_target(&mut self) {
        self.skip("request_self_target");
    }
    fn request_target_cancel(&mut self) {
        self.skip("request_target_cancel");
    }
    fn request_start_trade(&mut self, _target_id: i32) {
        self.skip("request_start_trade");
    }
    fn request_invite_party(&mut self, _name: &str) {
        self.skip("request_invite_party");
    }
    fn request_target_user(&mut self, _server_id: i32) {
        self.skip("request_target_user");
    }
    fn request_attack(&mut self, _server_id: i32, _loc: Vector) {
        self.skip("request_attack");
    }
    fn request_action(&mut self, _server_id: i32, _loc: Vector) {
        self.skip("request_action");
    }
    fn request_assist(&mut self, _server_id: i32, _loc: Vector) {
        self.skip("request_assist");
    }
    fn is_pk_mode(&self) -> bool {
        false
    }
    fn get_class_type(&self, _class_id: i32) -> String {
        String::new()
    }
    fn get_class_str(&self, _class_id: i32) -> String {
        String::new()
    }
    fn get_class_icon_name(&self, _class_id: i32) -> String {
        String::new()
    }
    fn get_user_rank_string(&self, _rank: i32) -> String {
        String::new()
    }
    fn get_routing_string(&self, _routing_type: i32) -> String {
        String::new()
    }
    fn is_debuff(&self, _skill_id: i32, _level: i32) -> bool {
        false
    }
    fn dialog_get_string(&self) -> String {
        String::new()
    }
    fn script_call(&mut self, window: &str, function: &str) {
        self.queue.push(HostEvent::ScriptCall(window.to_string(), function.to_string()));
    }
    fn script_get_int_array(&self, _window: &str, _variable: &str, _index: i32) -> i32 {
        0
    }
    fn target_id(&self) -> i32 {
        0
    }
    fn target_name_color(&self, _level_diff: i32) -> Argb {
        Argb(0xFFDC_DCDC)
    }
    fn target_is_server_object(&self) -> bool {
        false
    }
    fn target_is_hp_showable_npc(&self) -> bool {
        false
    }
    fn static_object_name_id(&self, _server_id: i32) -> i32 {
        0
    }
    fn static_object_name(&self, _name_id: i32) -> String {
        String::new()
    }
    fn server_object_type(&self, _server_id: i32) -> i32 {
        0
    }
    fn static_object_show_hp(&self, _server_id: i32) -> bool {
        false
    }
    fn server_object_hp(&self, _server_id: i32) -> i32 {
        0
    }
    fn server_object_max_hp(&self, _server_id: i32) -> i32 {
        0
    }
    fn party_member_name(&self, _server_id: i32) -> String {
        String::new()
    }
    fn player_id(&self) -> i32 {
        0
    }
    fn pet_id(&self) -> i32 {
        0
    }
    fn npc_property(&self, _class_id: i32) -> Option<Vec<i32>> {
        None
    }
    fn clan_name(&self, _clan_id: i32) -> String {
        String::new()
    }
    fn clan_type(&self, _user_id: i32) -> Option<i32> {
        None
    }
    fn clan_name_value(&self, _clan_id: i32) -> Option<i32> {
        None
    }
    fn clan_crest_texture(&self, _clan_id: i32) -> Option<String> {
        None
    }
    fn clan_alliance_name(&self, _clan_id: i32) -> String {
        String::new()
    }
    fn clan_alliance_crest_texture(&self, _clan_id: i32) -> Option<String> {
        None
    }
    fn player_move_type(&self) -> i32 {
        0
    }
    fn player_environment(&self) -> i32 {
        0
    }
    fn player_exp_rate(&self) -> f32 {
        0.0
    }
    fn skill_icon_name(&self, _skill_id: i32, _level: i32) -> String {
        String::new()
    }
    fn skill_name(&self, _skill_id: i32, _level: i32) -> String {
        String::new()
    }
    fn skill_description(&self, _skill_id: i32, _level: i32) -> String {
        String::new()
    }
    fn set_texture_with_object(&mut self, _name: &str, _texture: &str) {
        self.skip("set_texture_with_object");
    }
    fn set_texture_size(&mut self, _name: &str, _width: i32, _height: i32) {
        self.skip("set_texture_size");
    }
    fn set_texture_ctrl_type(&mut self, _name: &str, _kind: i32) {
        self.skip("set_texture_ctrl_type");
    }
    fn set_button_texture(&mut self, _name: &str, _normal: &str, _down: &str, _over: &str) {
        self.skip("set_button_texture");
    }
    fn set_tooltip_custom(&mut self, _name: &str, _tooltip: &CustomTooltip) {
        self.skip("set_tooltip_custom");
    }
    fn get_tooltip_custom(&self, _name: &str) -> CustomTooltip {
        Default::default()
    }
    fn get_name(&self, _name: &str) -> String {
        String::new()
    }
    fn set_resize_frame_size(&mut self, name: &str, width: i32, height: i32) {
        self.controls.resize_frames.insert(name.to_string(), (width, height));
    }
    fn status_icon_set_item(&mut self, name: &str, row: i32, col: i32, info: &StatusIconInfo) {
        if let Some(cell) = self.status_grids.get_mut(name).and_then(|grid| grid.get_mut(row.max(0) as usize)).and_then(|cells| cells.get_mut(col.max(0) as usize)) {
            *cell = info.clone();
        }
    }
    fn release_focus(&mut self, _name: &str) {
        self.skip("release_focus");
    }
    fn tree_clear(&mut self, name: &str) {
        self.controls.trees.entry(name.to_string()).or_default().nodes.clear();
    }
    fn tree_show_scroll_bar(&mut self, name: &str, show: bool) {
        self.controls.trees.entry(name.to_string()).or_default().scroll_bar = Some(show);
        self.controls.scroll_bars.insert(name.to_string(), show);
    }
    fn tree_insert_node(&mut self, name: &str, parent: &str, node: &TreeNodeInfo) -> String {
        self.controls.trees.entry(name.to_string()).or_default().insert_node(parent, node.clone())
    }
    fn tree_insert_node_item(&mut self, name: &str, node: &str, item: &TreeNodeItemInfo) {
        let item = super::vm_host_controls::TreeItemState { info: item.clone(), ..Default::default() };
        self.controls.trees.entry(name.to_string()).or_default().insert_node_item(node, item);
    }
    fn log(&mut self, _message: &str) {
        self.skip("log");
    }
    fn item_get_selected_num(&self, name: &str) -> i32 {
        self.control_item_selected_num(name)
    }
    fn item_get_item_num(&self, name: &str) -> i32 {
        self.control_item_num(name)
    }
    fn item_clear_select(&mut self, name: &str) {
        self.control_item_clear_select(name);
    }
    fn item_add(&mut self, name: &str, info: &ItemInfo) {
        self.control_item_add(name, info.clone());
    }
    fn item_set(&mut self, name: &str, index: i32, info: &ItemInfo) -> bool {
        self.control_item_set(name, index, info.clone())
    }
    fn item_delete(&mut self, name: &str, index: i32) {
        self.control_item_delete(name, index);
    }
    fn item_get_selected(&self, name: &str) -> Option<ItemInfo> {
        self.control_item_get_selected(name)
    }
    fn item_get(&self, name: &str, index: i32) -> Option<ItemInfo> {
        self.control_item_get(name, index)
    }
    fn item_clear(&mut self, name: &str) {
        self.control_item_clear(name);
    }
    fn item_find_with_server_id(&self, name: &str, server_id: i32) -> i32 {
        self.control_item_find_server_id(name, server_id)
    }
    fn item_swap(&mut self, name: &str, first: i32, second: i32) {
        self.control_item_swap(name, first, second);
    }
    fn item_get_index_at(&self, _name: &str, _x: i32, _y: i32, _offset_x: i32, _offset_y: i32) -> i32 {
        -1
    }
    fn item_set_disable_tex(&mut self, name: &str, texture: &str) {
        self.controls.item_windows.entry(name.to_string()).or_default().disable_tex = texture.to_string();
    }
    fn item_set_row(&mut self, name: &str, row: i32) {
        self.controls.item_windows.entry(name.to_string()).or_default().rows = Some(row);
    }
    fn set_tooltip_string(&mut self, _name: &str, _text: &str) {
        self.skip("set_tooltip_string");
    }
    fn request_use_item(&mut self, _server_id: i32) {
        self.skip("request_use_item");
    }
    fn request_destroy_item(&mut self, _server_id: i32, _num: i32) {
        self.skip("request_destroy_item");
    }
    fn request_drop_item(&mut self, _server_id: i32, _num: i32, _location: Vector) {
        self.skip("request_drop_item");
    }
    fn request_unequip_item(&mut self, _server_id: i32, _slot_bit_type: i32) {
        self.skip("request_unequip_item");
    }
    fn request_crystallize_item(&mut self, _server_id: i32, _num: i32) {
        self.skip("request_crystallize_item");
    }
    fn request_item_list(&mut self) {
        self.skip("request_item_list");
    }
    fn request_open_minimap(&mut self) {
        self.skip("request_open_minimap");
    }
    fn request_exit(&mut self) {
        self.skip("request_exit");
    }
    fn request_get_item_from_pet(&mut self, _server_id: i32, _num: i32, _is_equip_item: bool) {
        self.skip("request_get_item_from_pet");
    }
    fn get_accessory_server_id(&self) -> Option<(i32, i32, i32, i32)> {
        None
    }
    fn get_class_step(&self, _class_id: i32) -> i32 {
        0
    }
    fn get_adena(&self) -> i32 {
        0
    }
    fn is_key_down(&self, _key: i32) -> bool {
        false
    }
    fn make_cost_string(&self, _text: &str) -> String {
        String::new()
    }
    fn convert_num_to_text(&self, _text: &str) -> String {
        String::new()
    }
    fn check_item_limit(&self, _class_id: i32, _count: i32) -> bool {
        false
    }
    fn get_click_location(&self) -> Vector {
        Default::default()
    }
    fn is_stackable_item(&self, _consume_type: i32) -> bool {
        false
    }
    fn save_inventory_order(&mut self, _order: &[i32]) {
        self.skip("save_inventory_order");
    }
    fn load_inventory_order(&self) -> Option<Vec<i32>> {
        None
    }
    fn player_has_crystallize_ability(&self) -> bool {
        false
    }
    fn player_inventory_limit(&self) -> i32 {
        0
    }
    fn item_is_crystallizable(&self, _class_id: i32) -> bool {
        false
    }
    fn henna_info_count(&self) -> i32 {
        0
    }
    fn henna_info(&self, _index: i32) -> Option<(i32, i32)> {
        None
    }
    fn henna_item_name(&self, _henna_id: i32) -> Option<String> {
        None
    }
    fn henna_description(&self, _henna_id: i32) -> Option<String> {
        None
    }
    fn henna_icon_tex(&self, _henna_id: i32) -> Option<String> {
        None
    }
    fn dialog_show(&mut self, _kind: i32, _message: &str, _target: &str) {
        self.skip("dialog_show");
    }
    fn dialog_hide(&mut self) {
        self.skip("dialog_hide");
    }
    fn dialog_set_id(&mut self, _id: i32) {
        self.skip("dialog_set_id");
    }
    fn dialog_set_param_int(&mut self, _value: i32) {
        self.skip("dialog_set_param_int");
    }
    fn dialog_set_reserved_int(&mut self, _value: i32) {
        self.skip("dialog_set_reserved_int");
    }
    fn dialog_set_reserved_int2(&mut self, _value: i32) {
        self.skip("dialog_set_reserved_int2");
    }
    fn dialog_get_target(&self) -> String {
        String::new()
    }
    fn dialog_get_edit_message(&self) -> String {
        String::new()
    }
    fn dialog_get_id(&self) -> i32 {
        0
    }
    fn dialog_get_reserved_int(&self) -> i32 {
        0
    }
    fn dialog_get_reserved_int2(&self) -> i32 {
        0
    }
    fn script_get_bool(&self, window: &str, var: &str) -> bool {
        self.script_bools.get(&(window.to_string(), var.to_string())).copied().unwrap_or(false)
    }
    fn script_get_int(&self, window: &str, var: &str) -> i32 {
        self.script_ints.get(&(window.to_string(), var.to_string())).copied().unwrap_or(0)
    }
    fn class_label_texture(&self, _class_id: i32) -> String {
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_host_keeps_script_changes_in_the_overlay_and_queues_what_the_app_must_do() {
        let mut host = ScriptHost { current: "StatusWnd".into(), ..Default::default() };
        host.register_event(70);
        host.set_text("StatusWnd.LevelTextBox", "52");
        host.set_point("StatusWnd.HPBar", 400, 900);
        host.show_window("SystemMsgWnd");
        host.hide_window("ChatWnd");
        host.status_icon_add_row("AbnormalStatusWnd.StatusIcon");
        host.status_icon_add_col("AbnormalStatusWnd.StatusIcon", 0, &StatusIconInfo { icon_name: "icon.a".into(), ..Default::default() });
        assert_eq!(host.events.listeners(70), ["StatusWnd"]);
        assert_eq!(host.overlay.text("StatusWnd.LevelTextBox"), Some("52"));
        assert_eq!(host.overlay.point("StatusWnd.HPBar"), Some((400, 900)));
        assert!(host.is_show_window("SystemMsgWnd"));
        assert!(!host.is_show_window("ChatWnd"));
        assert_eq!(host.status_icon_col_count("AbnormalStatusWnd.StatusIcon", 0), 1);
        assert_eq!(host.queue, [HostEvent::ShowWindow("SystemMsgWnd".into()), HostEvent::HideWindow("ChatWnd".into())]);
    }
}
