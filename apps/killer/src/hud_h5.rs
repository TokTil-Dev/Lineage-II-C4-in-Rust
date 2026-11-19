use crate::lobby_c4::DOUBLE_CLICK_MS;
use killer_assets::client::DatTables;
use killer_platform::time::Instant;
use killer_ui::chat::{InputKey, TextInput};
use killer_ui::color::{Argb, TEXT};
use killer_ui::draw::{DrawList, H5_STATUS_NUMBER_STRIP, digits_width_of};
use killer_ui::font::BitmapFont;
use killer_ui::abnormal::AbnormalView;
use killer_ui::inventory::{InventoryItemView, InventoryView};
use killer_ui::window::Rect;
use killer_ui::xdat_window::{self, Item, Layout, Paint, TextAlign, TextVAlign};
use killer_world::chat::{CHAT_SYSTEM, ChatLine};
use killer_world::{Entity, EntityKind, World};
use l2_dat::xdat::{Control, Interface, Props, ShortcutKey, WindowDef};
use l2_net::game::ShortcutKind;
use std::collections::{HashMap, HashSet};

pub const STATUS: &str = "StatusWnd";
pub const TARGET: &str = "TargetStatusWnd";
pub const SHORTCUT: &str = "ShortcutWnd";
pub const CHAT: &str = "ChatWnd";
pub const SYSTEM_MENU: &str = "SystemMenuWnd";
pub const INVENTORY: &str = "InventoryWnd";
pub const ABNORMAL: &str = "AbnormalStatusWnd";
pub const PARTY: &str = "PartyWnd";
pub const PARTY_SLOTS: usize = 9;
pub const MINIMAP: &str = "MinimapWnd";
pub const DIALOG: &str = "DialogBox";

pub const SHORTCUT_PER_PAGE: i32 = 12;
pub const SHORTCUT_PAGES: i32 = 10;
pub const TARGET_CONTRACT_HEIGHT: i32 = 46;
pub const TARGET_EXPAND_HEIGHT: i32 = 76;
pub const CHAT_TABS: usize = 5;
pub const CHAT_WINDOWS: [&str; CHAT_TABS] = ["NormalChat", "TradeChat", "PartyChat", "ClanChat", "AllyChat"];
pub const DRAG_THRESHOLD: i32 = 3;
pub const SELF_TARGET_MARGIN: (i32, i32) = (13, 10);
pub const VITALITY_LEVELS: [(i32, i32, u8); 5] = [(17000, 3000, 5), (13000, 4000, 4), (2000, 11000, 3), (240, 1760, 2), (0, 240, 1)];
pub const VITALITY_MAX: i32 = 20000;
pub const VITALITY_TEXTURE: &str = "l2ui_ct1.LifeForce.Icon_df_LifeForce_StatusWnd_0";
pub const SYSMSG_EXIT: u32 = 125;
pub const SYSMSG_RESTART: u32 = 126;
pub const DIALOG_BUTTON_OK: u32 = 1337;
pub const DIALOG_BUTTON_CANCEL: u32 = 1342;
pub const SYSSTRING_BBS: u32 = 387;
pub const SYSSTRING_MACRO: u32 = 711;
pub const SYSSTRING_PLEDGE: u32 = 431;
pub const SYSSTRING_ALLIANCE: u32 = 591;
pub const SHORTCUT_OWNER_PET: i32 = 2;
pub const CRITICAL_ANNOUNCE: i32 = 18;
pub const SCREEN_ANNOUNCE: i32 = 19;
pub const SYSTEM_TYPE_PALETTE: i32 = 5;

pub const INVENTORY_COLS: i32 = 6;
pub const INVENTORY_ROWS: i32 = 8;
pub const INVENTORY_VISIBLE: i32 = INVENTORY_COLS * INVENTORY_ROWS;
pub const INVENTORY_SLOT: i32 = 32;
pub const INVENTORY_GAP: i32 = 4;
pub const INVENTORY_PITCH: i32 = INVENTORY_SLOT + INVENTORY_GAP;
pub const ABNORMAL_COLS: i32 = 12;
pub const ABNORMAL_ICON: i32 = 24;

#[derive(Debug, Clone, PartialEq)]
pub enum HudAction {
    SelfTarget,
    CancelTarget,
    Say { text: String, kind: i32, target: String },
    UseShortcut(i32),
    UseItem(i32),
    UnequipPaperdoll(usize),
    Restart,
    Quit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyOutcome {
    Ignored,
    Consumed,
}

pub struct HudView<'a> {
    pub world: &'a World,
    pub tables: Option<&'a DatTables>,
    pub chat: &'a [ChatLine],
    pub size: (i32, i32),
    pub font: &'a BitmapFont,
    pub sysstring: &'a dyn Fn(u32) -> Option<String>,
    pub inventory: Option<&'a InventoryView>,
    pub abnormal: Option<&'a AbnormalView>,
}

#[derive(Debug, Clone, PartialEq)]
enum RegionKind {
    Button(String),
    Slot(i32),
    Tab(usize),
    Edit,
    Body,
    ItemSlot(i32),
    Paperdoll(usize),
}

#[derive(Debug, Clone)]
struct Region {
    rect: Rect,
    kind: RegionKind,
}

#[derive(Debug, Clone)]
struct HitWindow {
    name: String,
    bounds: Rect,
    drag_zone: Rect,
    regions: Vec<Region>,
}

#[derive(Debug, Clone)]
struct Press {
    window: String,
    region: Option<RegionKind>,
    start: (i32, i32),
    origin: (i32, i32),
    dragging: bool,
}

#[derive(Default, Clone, Copy)]
struct Modifiers {
    alt: bool,
    ctrl: bool,
    shift: bool,
}

#[derive(Default)]
struct ChatState {
    tab: usize,
    scroll: [usize; CHAT_TABS],
    input: TextInput,
    focused: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Confirm {
    Restart,
    Quit,
}

#[derive(Default, Clone)]
struct SlotView {
    icon: Option<String>,
    count: Option<String>,
}

#[derive(Default)]
struct Binding {
    hidden: HashSet<String>,
    hidden_prefix: Vec<&'static str>,
    values: HashMap<String, String>,
    gauges: HashMap<String, f32>,
    colors: HashMap<String, Argb>,
    textures: HashMap<String, String>,
    disabled: HashSet<String>,
    names: HashMap<String, (String, Argb, TextAlign)>,
    chat_lines: Vec<(String, Argb)>,
    chat_active: Option<&'static str>,
    edit: Option<(String, String, bool, usize)>,
    tab_selected: usize,
    height: Option<i32>,
    width: Option<i32>,
    captions: Vec<(String, String)>,
    bounds_child: Option<&'static str>,
}

impl Binding {
    fn hide(&mut self, names: &[&str]) {
        self.hidden.extend(names.iter().map(|name| name.to_string()));
    }

    fn is_hidden(&self, name: &str) -> bool {
        self.hidden.contains(name) || self.hidden_prefix.iter().any(|prefix| name.starts_with(prefix))
    }
}

pub struct H5Hud {
    interface: Interface,
    keys: Vec<ShortcutKey>,
    sizes: HashMap<String, Option<(i32, i32)>>,
    primed: bool,
    stack: Vec<String>,
    shown: HashSet<String>,
    offsets: HashMap<String, (i32, i32)>,
    frame: Vec<HitWindow>,
    press: Option<Press>,
    pub cursor: (i32, i32),
    modifiers: Modifiers,
    target_expanded: bool,
    page: i32,
    page2: i32,
    page3: i32,
    page4: i32,
    expand_level: u8,
    shortcut_views: HashMap<usize, SlotView>,
    shortcut_slot_num: HashMap<usize, i32>,
    shortcut_page_text: HashMap<usize, String>,
    locked: bool,
    vertical: bool,
    tooltip_min: bool,
    chat: ChatState,
    confirm: Option<Confirm>,
    reported: Vec<String>,
    missing: Vec<String>,
    inventory_tab: usize,
    inventory_scroll: i32,
    inventory_selected: Option<i32>,
    last_item_click: Option<(i32, Instant)>,
    last_paperdoll_click: Option<(usize, Instant)>,
    party_names: HashMap<usize, (String, Argb, TextAlign)>,
    party_icons: HashMap<usize, String>,
    party_gauges: HashMap<usize, f32>,
    window_rects: HashMap<String, Rect>,
    social_tab: usize,
    social_page: usize,
    system: crate::hud_h5_system::SystemWindows,
}

fn key(name: &str) -> String {
    name.to_ascii_lowercase()
}

fn collect(controls: &[Control], out: &mut Vec<String>) {
    for control in controls {
        match &control.props {
            Props::Button(button) => out.extend(button.textures.iter().cloned()),
            Props::Texture(texture) => out.push(texture.texture.clone()),
            Props::StatusBar(gauge) | Props::InvenWeight(gauge) => out.extend(gauge.textures.iter().cloned()),
            Props::BarCtrl { textures, .. } => out.extend(textures.iter().cloned()),
            Props::Tab(pages) => out.extend(pages.iter().flat_map(|page| page.textures.iter().cloned())),
            Props::Window(part) => out.push(part.back_tex.clone()),
            _ => {}
        }
        collect(&control.children, out);
    }
}

fn level_color(diff: i32, my_level: i32) -> Argb {
    let rgb = if my_level < 78 {
        match diff {
            i32::MIN..=-9 => (255, 0, 0),
            -8..=-6 => (255, 145, 145),
            -5..=-3 => (250, 254, 145),
            -2..=2 => (255, 255, 255),
            3..=5 => (162, 255, 171),
            6..=8 => (162, 168, 252),
            _ => (0, 0, 255),
        }
    } else {
        match diff {
            i32::MIN..=-8 => (255, 0, 0),
            -7..=-5 => (255, 145, 145),
            -4..=-2 => (250, 254, 145),
            -1..=1 => (255, 255, 255),
            2..=3 => (162, 255, 171),
            4..=5 => (162, 168, 252),
            _ => (0, 0, 255),
        }
    };
    Argb(0xFF00_0000 | (rgb.0 << 16) | (rgb.1 << 8) | rgb.2)
}

fn rgb(r: u32, g: u32, b: u32) -> Argb {
    Argb(0xFF00_0000 | (r << 16) | (g << 8) | b)
}

fn union_rect(a: Rect, b: Rect) -> Rect {
    let x0 = a.x.min(b.x);
    let y0 = a.y.min(b.y);
    let x1 = (a.x + a.w).max(b.x + b.w);
    let y1 = (a.y + a.h).max(b.y + b.h);
    Rect::new(x0, y0, x1 - x0, y1 - y0)
}

fn equip_slot_index(name: &str) -> Option<usize> {
    Some(match name {
        "EquipItem_Underwear" => 0,
        "EquipItem_Head" => 1,
        "EquipItem_Cloak" => 2,
        "EquipItem_Hair" => 3,
        "EquipItem_Neck" => 4,
        "EquipItem_RHand" => 5,
        "EquipItem_Chest" => 6,
        "EquipItem_LHand" => 7,
        "EquipItem_REar" => 8,
        "EquipItem_LEar" => 9,
        "EquipItem_Gloves" => 10,
        "EquipItem_Legs" => 11,
        "EquipItem_Feet" => 12,
        "EquipItem_RFinger" => 13,
        "EquipItem_LFinger" => 14,
        _ => return None,
    })
}

fn inventory_items(inventory: &InventoryView, quest: bool) -> Vec<&InventoryItemView> {
    inventory.items.iter().filter(|item| !item.equipped && item.quest == quest).collect()
}

fn inventory_scroll_clamped(scroll: i32, total: usize) -> i32 {
    let rows = (total as i32 + INVENTORY_COLS - 1) / INVENTORY_COLS;
    scroll.clamp(0, (rows - INVENTORY_ROWS).max(0))
}

fn item_cell_rect(origin: Rect, cell: i32) -> Rect {
    let col = cell % INVENTORY_COLS;
    let row = cell / INVENTORY_COLS;
    Rect::new(origin.x + col * INVENTORY_PITCH, origin.y + row * INVENTORY_PITCH, INVENTORY_SLOT, INVENTORY_SLOT)
}

fn draw_item_grid(rect: Rect, inventory: &InventoryView, quest: bool, scroll: i32, textures: (&str, &str), font: &BitmapFont, list: &mut DrawList) {
    let (slot_back, frame) = textures;
    let items = inventory_items(inventory, quest);
    let scroll = inventory_scroll_clamped(scroll, items.len());
    let start = scroll as usize * INVENTORY_COLS as usize;
    if xdat_window::UNDEFINED != slot_back && !slot_back.is_empty() {
        for cell in 0..INVENTORY_VISIBLE {
            let slot = item_cell_rect(rect, cell);
            list.draw_texture(slot.x, slot.y, slot.w, slot.h, 0, 0, slot.w, slot.h, slot_back, 1.0);
        }
    }
    for (cell, inv_item) in items.iter().enumerate().skip(start).take(INVENTORY_VISIBLE as usize) {
        let slot = item_cell_rect(rect, (cell - start) as i32);
        list.draw_texture(slot.x, slot.y, slot.w, slot.h, 0, 0, slot.w, slot.h, inv_item.icon(0), 1.0);
        if xdat_window::UNDEFINED != frame && !frame.is_empty() {
            list.draw_texture(slot.x, slot.y, slot.w, slot.h, 0, 0, slot.w, slot.h, frame, 1.0);
        }
        if inv_item.count > 1 {
            let text = inv_item.count.to_string();
            let width = font.measure(&text) as i32;
            list.draw_text(slot.x + slot.w - width, slot.y + slot.h - font.line_height as i32, TEXT, &text, font);
        }
    }
}

struct AbnormalEntry<'a> {
    icon: Option<&'a str>,
    remaining: Option<i32>,
}

fn abnormal_entries(abnormal: &AbnormalView) -> Vec<AbnormalEntry<'_>> {
    let mut out = Vec::new();
    for buff in abnormal.buffs.iter().filter(|buff| !buff.debuff) {
        out.push(AbnormalEntry { icon: buff.icon.as_deref(), remaining: buff.remaining });
    }
    for buff in abnormal.buffs.iter().filter(|buff| buff.debuff) {
        out.push(AbnormalEntry { icon: buff.icon.as_deref(), remaining: buff.remaining });
    }
    for icon in abnormal.etc.iter().flatten() {
        out.push(AbnormalEntry { icon: Some(icon.as_str()), remaining: None });
    }
    if let Some(short) = &abnormal.short_buff {
        out.push(AbnormalEntry { icon: short.icon.as_deref(), remaining: short.remaining });
    }
    out
}

fn draw_abnormal_icons(rect: Rect, abnormal: &AbnormalView, view: &HudView, list: &mut DrawList) {
    let hour = (view.sysstring)(killer_ui::abnormal::HOUR_SYSSTRING).unwrap_or_default();
    let minute = (view.sysstring)(killer_ui::abnormal::MINUTE_SYSSTRING).unwrap_or_default();
    for (index, entry) in abnormal_entries(abnormal).into_iter().enumerate() {
        let col = index as i32 % ABNORMAL_COLS;
        let row = index as i32 / ABNORMAL_COLS;
        let (x, y) = (rect.x + col * ABNORMAL_ICON, rect.y + row * ABNORMAL_ICON);
        if let Some(icon) = entry.icon {
            let texels = killer_ui::abnormal::SKILL_ICON_TEXELS;
            list.draw_texture(x, y, ABNORMAL_ICON, ABNORMAL_ICON, 0, 0, texels, texels, icon, 1.0);
        }
        if let Some(remaining) = entry.remaining {
            let text = killer_ui::abnormal::short_time_text(remaining, &hour, &minute);
            let width = view.font.measure(&text) as i32;
            list.draw_text(x + (ABNORMAL_ICON - width) / 2, y + ABNORMAL_ICON - view.font.line_height as i32, TEXT, &text, view.font);
        }
    }
}

fn status_number_row(list: &mut DrawList, bar_rect: Rect, label: &str, value: (i32, i32)) {
    let strip = H5_STATUS_NUMBER_STRIP;
    let y = bar_rect.y + (bar_rect.h - strip.cell_height) / 2;
    let center = bar_rect.x + bar_rect.w / 2;
    let slash_w = digits_width_of(strip, "/");
    list.draw_digits_with(strip, bar_rect.x, y, label, TEXT);
    list.draw_digits_with(strip, center - slash_w / 2, y, "/", TEXT);
    let current = value.0.to_string();
    let current_w = digits_width_of(strip, &current);
    list.draw_digits_with(strip, center - slash_w / 2 - current_w, y, &current, TEXT);
    list.draw_digits_with(strip, center + slash_w / 2, y, &value.1.to_string(), TEXT);
}

fn status_percent_row(list: &mut DrawList, bar_rect: Rect, percent: f32) {
    let strip = H5_STATUS_NUMBER_STRIP;
    let y = bar_rect.y + (bar_rect.h - strip.cell_height) / 2;
    let text = format!("{percent:.2}%");
    let width = digits_width_of(strip, &text);
    list.draw_digits_with(strip, bar_rect.x + bar_rect.w / 2 - width / 2, y, &text, TEXT);
}

fn draw_status_numbers(layout: &Layout, view: &HudView, list: &mut DrawList) {
    let Some(me) = view.world.me.as_ref() else { return };
    if let Some(cp) = me.cp
        && let Some(item) = layout.find("CPBar")
    {
        status_number_row(list, item.rect, "CP", cp);
    }
    if let Some(hp) = me.hp
        && let Some(item) = layout.find("HPBar")
    {
        status_number_row(list, item.rect, "HP", hp);
    }
    if let Some(mp) = me.mp
        && let Some(item) = layout.find("MPBar")
    {
        status_number_row(list, item.rect, "MP", mp);
    }
    if let Some(extras) = view.world.user_extras
        && let Some(item) = layout.find("EXPBar")
    {
        status_percent_row(list, item.rect, extras.exp_percent);
    }
}

fn draw_weight_number(layout: &Layout, inventory: &InventoryView, list: &mut DrawList) {
    let Some(item) = layout.find("InvenWeight") else { return };
    status_number_row(list, item.rect, "", (inventory.load, inventory.max_load));
}

const SHORTCUT_VERTICAL_ROWS: [&str; 4] = ["ShortcutWndVertical", "ShortcutWndVertical_1", "ShortcutWndVertical_2", "ShortcutWndVertical_3"];
const SHORTCUT_HORIZONTAL_ROWS: [&str; 4] = ["ShortcutWndHorizontal", "ShortcutWndHorizontal_1", "ShortcutWndHorizontal_2", "ShortcutWndHorizontal_3"];

pub fn vitality_level(vitality: i32) -> (u8, f32) {
    if vitality > VITALITY_MAX {
        return (1, 0.0);
    }
    for (floor, range, texture) in VITALITY_LEVELS {
        if vitality >= floor {
            return (texture, (vitality - floor) as f32 / range as f32);
        }
    }
    (1, 0.0)
}

#[derive(Clone, Copy)]
pub struct ChatFilter {
    system: bool,
    normal: bool,
    shout: bool,
    clan: bool,
    party: bool,
    trade: bool,
    whisper: bool,
    damage: bool,
    ally: bool,
    use_item: bool,
    hero: bool,
    union: bool,
    battle: bool,
}

const fn filter(bits: [bool; 13]) -> ChatFilter {
    ChatFilter {
        system: bits[0],
        normal: bits[1],
        shout: bits[2],
        clan: bits[3],
        party: bits[4],
        trade: bits[5],
        whisper: bits[6],
        damage: bits[7],
        ally: bits[8],
        use_item: bits[9],
        hero: bits[10],
        union: bits[11],
        battle: bits[12],
    }
}

pub const CHAT_FILTERS: [ChatFilter; CHAT_TABS] = [
    filter([true, true, true, true, true, false, true, true, false, false, false, true, true]),
    filter([true, false, true, false, false, true, true, true, false, false, false, false, false]),
    filter([true, false, true, false, true, false, true, true, false, false, false, false, false]),
    filter([true, false, true, true, false, false, true, true, false, false, false, false, false]),
    filter([true, false, true, false, false, false, true, true, true, false, false, false, false]),
];

const SYSTEM_SERVER: u32 = 2;
const SYSTEM_DAMAGE: u32 = 3;
const SYSTEM_PETITION: u32 = 6;
const SYSTEM_USE_ITEMS: u32 = 7;

pub fn chat_passes(tab: usize, line: &ChatLine) -> bool {
    let Some(filter) = CHAT_FILTERS.get(tab) else { return false };
    match line.kind {
        8 => filter.trade,
        0 => filter.normal,
        4 => filter.clan,
        3 => filter.party,
        1 => filter.shout,
        2 => filter.whisper,
        9 => filter.ally,
        17 => filter.hero,
        20 => filter.battle,
        10 | 18 | 7 => true,
        15 | 16 => filter.union,
        22 | 23 => true,
        SCREEN_ANNOUNCE => tab == 0,
        CHAT_SYSTEM => {
            let kind = line.group;
            if kind == SYSTEM_SERVER || kind == SYSTEM_PETITION {
                true
            } else if filter.system {
                match kind {
                    SYSTEM_DAMAGE => filter.damage,
                    SYSTEM_USE_ITEMS => filter.use_item,
                    _ => true,
                }
            } else {
                false
            }
        }
        _ => false,
    }
}

fn chat_text(line: &ChatLine) -> String {
    if line.kind == CRITICAL_ANNOUNCE || line.sender.is_empty() {
        line.text.clone()
    } else {
        format!("{}: {}", line.sender, line.text)
    }
}

fn wrap(text: &str, width: i32, font: &BitmapFont) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    for ch in text.chars() {
        let mut probe = current.clone();
        probe.push(ch);
        if !current.is_empty() && font.measure(&probe) as i32 > width {
            out.push(std::mem::take(&mut current));
        }
        current.push(ch);
    }
    out.push(current);
    out
}

impl H5Hud {
    pub fn new(interface: Interface) -> H5Hud {
        let keys = interface.shortcut_sets.iter().filter(|set| set.state == "GamingState").flat_map(|set| set.keys.iter().cloned()).collect();
        let shown: HashSet<String> = [STATUS, SHORTCUT, CHAT].iter().map(|name| name.to_string()).collect();
        let stack = vec![SHORTCUT.to_string(), STATUS.to_string(), TARGET.to_string(), CHAT.to_string(), ABNORMAL.to_string(), PARTY.to_string()];
        H5Hud {
            interface,
            keys,
            sizes: HashMap::new(),
            primed: false,
            stack,
            shown,
            offsets: HashMap::new(),
            frame: Vec::new(),
            press: None,
            cursor: (0, 0),
            modifiers: Modifiers::default(),
            target_expanded: false,
            page: 0,
            page2: 1,
            page3: 2,
            page4: 3,
            expand_level: 0,
            shortcut_views: HashMap::new(),
            shortcut_slot_num: HashMap::new(),
            shortcut_page_text: HashMap::new(),
            locked: false,
            vertical: false,
            tooltip_min: false,
            chat: ChatState::default(),
            confirm: None,
            reported: Vec::new(),
            missing: Vec::new(),
            inventory_tab: 0,
            inventory_scroll: 0,
            inventory_selected: None,
            last_item_click: None,
            last_paperdoll_click: None,
            party_names: HashMap::new(),
            party_icons: HashMap::new(),
            party_gauges: HashMap::new(),
            window_rects: HashMap::new(),
            social_tab: 0,
            social_page: 0,
            system: crate::hud_h5_system::SystemWindows::default(),
        }
    }

    pub fn texture_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        let ported = [STATUS, TARGET, SHORTCUT, CHAT, SYSTEM_MENU, DIALOG, INVENTORY, PARTY, MINIMAP, ABNORMAL]
            .into_iter()
            .chain([crate::hud_h5_windows::DETAIL_STATUS, crate::hud_h5_windows::MAGIC_SKILL, crate::hud_h5_windows::ACTION])
            .chain(crate::hud_h5_social::WINDOWS)
            .chain([crate::hud_h5_system::BOARD, crate::hud_h5_system::HELP_HTML, crate::hud_h5_system::OPTION, crate::hud_h5_system::GM]);
        for window in ported {
            if let Some(window) = self.interface.window(window) {
                names.push(window.part.back_tex.clone());
                collect(&window.children, &mut names);
            }
        }
        for index in 1..=5 {
            names.push(format!("{VITALITY_TEXTURE}{index}"));
        }
        names.retain(|name| xdat_window::UNDEFINED != name && !name.is_empty());
        names
    }

    pub fn prime(&mut self, size_of: &mut dyn FnMut(&str) -> Option<(i32, i32)>) {
        if self.primed {
            return;
        }
        self.primed = true;
        for name in self.texture_names() {
            self.sizes.entry(key(&name)).or_insert_with(|| size_of(&name));
        }
    }

    fn size(&self, name: &str) -> Option<(i32, i32)> {
        self.sizes.get(&key(name)).copied().flatten()
    }

    fn report(&mut self, what: &str) {
        if !self.reported.iter().any(|item| item == what) {
            log::warn!("HUD H5: {what}");
            self.reported.push(what.to_string());
        }
    }

    pub fn chat_focused(&self) -> bool {
        self.chat.focused
    }

    pub fn dialog_open(&self) -> bool {
        self.confirm.is_some()
    }

    pub fn set_modifiers(&mut self, alt: bool, ctrl: bool, shift: bool) {
        self.modifiers = Modifiers { alt, ctrl, shift };
    }

    pub fn modifiers(&self) -> (bool, bool) {
        (self.modifiers.ctrl, self.modifiers.shift)
    }

    fn defaults(&self) -> impl Fn(&str) -> Option<[i32; 3]> + '_ {
        |name| self.interface.layouts.iter().find(|layout| layout.window == name).map(|layout| [layout.values[0], layout.values[1], layout.values[2]])
    }

    fn is_open(&self, name: &str, view: &HudView) -> bool {
        match name {
            TARGET => view.world.targeted().is_some(),
            STATUS => view.world.me.is_some() && self.shown.contains(name),
            ABNORMAL => true,
            PARTY => !view.world.party.members.is_empty(),
            _ => self.shown.contains(name),
        }
    }

    fn raise(&mut self, name: &str) {
        if let Some(index) = self.stack.iter().position(|other| other == name) {
            let window = self.stack.remove(index);
            self.stack.push(window);
        }
    }

    fn toggle(&mut self, name: &str) {
        if self.shown.remove(name) {
            return;
        }
        self.shown.insert(name.to_string());
        if !self.stack.iter().any(|other| other == name) {
            self.stack.push(name.to_string());
        }
        self.raise(name);
    }

    fn layout_of<'a>(&self, window: &'a WindowDef, view: &HudView, size: (Option<i32>, Option<i32>)) -> Layout<'a> {
        let viewport = Rect::new(0, 0, view.size.0, view.size.1);
        let defaults = self.defaults();
        let target = |name: &str| self.window_rects.get(name).copied();
        let mut layout = xdat_window::layout_with(window, viewport, &target, &defaults);
        if let Some(width) = size.0 {
            layout.rect.w = width;
        }
        if let Some(height) = size.1 {
            layout.rect.h = height;
        }
        let (dx, dy) = self.offsets.get(&window.head.name).copied().unwrap_or((0, 0));
        layout.shifted(dx, dy)
    }

    fn arrange_shortcut_drawers(&self, layout: &mut Layout) {
        for chain in [SHORTCUT_VERTICAL_ROWS, SHORTCUT_HORIZONTAL_ROWS] {
            let mut previous: Option<Rect> = None;
            for name in chain {
                let Some(index) = layout.items.iter().position(|item| item.control.head.name == name) else { continue };
                let depth = layout.items[index].depth;
                let rect = layout.items[index].rect;
                let delta = match (&layout.items[index].control.props, previous) {
                    (Props::Window(part), Some(anchor)) if part.drawer_direction > 0 => Some(match part.drawer_direction {
                        1 => (anchor.x - part.offset_x - rect.w - rect.x, anchor.y - rect.y),
                        2 => (anchor.x + anchor.w + part.offset_x - rect.x, anchor.y - rect.y),
                        3 => (anchor.x - rect.x, anchor.y - part.offset_y - rect.h - rect.y),
                        4 => (anchor.x - rect.x, anchor.y + anchor.h + part.offset_y - rect.y),
                        _ => (0, 0),
                    }),
                    _ => None,
                };
                if let Some((dx, dy)) = delta {
                    let end = layout.items.iter().skip(index + 1).position(|item| item.depth <= depth).map_or(layout.items.len(), |n| index + 1 + n);
                    for item in &mut layout.items[index..end] {
                        item.rect = item.rect.offset(dx, dy);
                    }
                }
                previous = Some(layout.items[index].rect);
            }
        }
    }

    fn compute_slot_view(&mut self, view: &HudView, slot: i32) -> SlotView {
        let mut entry = SlotView::default();
        let Some(shortcut) = view.world.shortcut(slot) else { return entry };
        match shortcut.kind {
            ShortcutKind::Item { object_id } => {
                if let Some(item) = view.world.inventory.items.iter().find(|item| item.object_id == object_id) {
                    entry.icon = view.tables.and_then(|tables| u32::try_from(item.item_id).ok().and_then(|id| tables.item_icons.texture(id)));
                    if item.count > 1 {
                        entry.count = Some(item.count.to_string());
                    }
                }
            }
            ShortcutKind::Skill { skill_id, level } => entry.icon = crate::ui2d::skill_icon(view.tables, skill_id, level),
            ShortcutKind::Action { .. } => self.report("ShortcutWnd: ícone dos atalhos de ação (actionname) ainda não ligado"),
            ShortcutKind::Macro { .. } => self.report("ShortcutWnd: ícone dos atalhos de macro ainda não ligado"),
            ShortcutKind::Recipe { .. } => self.report("ShortcutWnd: ícone dos atalhos de receita ainda não ligado"),
            ShortcutKind::Other { .. } => {}
        }
        entry
    }

    fn prepare_shortcut_rows(&mut self, layout: &Layout, view: &HudView) {
        self.shortcut_views.clear();
        self.shortcut_slot_num.clear();
        self.shortcut_page_text.clear();
        let rows: [(&str, i32); 8] = [
            (SHORTCUT_VERTICAL_ROWS[0], self.page),
            (SHORTCUT_VERTICAL_ROWS[1], self.page2),
            (SHORTCUT_VERTICAL_ROWS[2], self.page3),
            (SHORTCUT_VERTICAL_ROWS[3], self.page4),
            (SHORTCUT_HORIZONTAL_ROWS[0], self.page),
            (SHORTCUT_HORIZONTAL_ROWS[1], self.page2),
            (SHORTCUT_HORIZONTAL_ROWS[2], self.page3),
            (SHORTCUT_HORIZONTAL_ROWS[3], self.page4),
        ];
        for (row_name, page) in rows {
            let Some(start) = layout.items.iter().position(|item| item.control.head.name == row_name) else { continue };
            let depth = layout.items[start].depth;
            let end = layout.items.iter().skip(start + 1).position(|item| item.depth <= depth).map_or(layout.items.len(), |n| start + 1 + n);
            for offset in start..end {
                let control = layout.items[offset].control;
                let key = control as *const Control as usize;
                if let Some(index) = control.head.name.strip_prefix("Shortcut").and_then(|n| n.parse::<i32>().ok()) {
                    let slot = page * SHORTCUT_PER_PAGE + (index - 1);
                    let slot_view = self.compute_slot_view(view, slot);
                    self.shortcut_slot_num.insert(key, slot);
                    self.shortcut_views.insert(key, slot_view);
                } else if control.head.name == "PageNumTextBox" {
                    self.shortcut_page_text.insert(key, (page + 1).to_string());
                }
            }
        }
    }

    fn prepare_party_rows(&mut self, layout: &Layout, view: &HudView) {
        self.party_names.clear();
        self.party_icons.clear();
        self.party_gauges.clear();
        let members = &view.world.party.members;
        let leader_id = view.world.party.leader_id;
        for slot in 0..PARTY_SLOTS {
            let Some(member) = members.get(slot) else { continue };
            let row_name = if slot == 0 { "PartyStatusWnd0".to_string() } else { format!("PartyStatusWnd{slot}") };
            let Some(start) = layout.items.iter().position(|item| item.control.head.name == row_name) else { continue };
            let depth = layout.items[start].depth;
            let end = layout.items.iter().skip(start + 1).position(|item| item.depth <= depth).map_or(layout.items.len(), |n| start + 1 + n);
            for offset in start..end {
                let control = layout.items[offset].control;
                let key = control as *const Control as usize;
                match control.head.name.as_str() {
                    "PlayerName" => {
                        self.party_names.insert(key, (member.name.clone(), TEXT, TextAlign::Center));
                    }
                    "LeaderIcon" if member.object_id == leader_id => {
                        self.party_icons.insert(key, killer_ui::party::LEADER_ICON.to_string());
                    }
                    "ClassIcon" => {
                        if let Some(icon) = killer_ui::class_icon::texture(member.class_id) {
                            self.party_icons.insert(key, icon.to_string());
                        }
                    }
                    "barCP" => {
                        self.party_gauges.insert(key, Self::me_gauge(Some((member.cp, member.max_cp))));
                    }
                    "barHP" => {
                        self.party_gauges.insert(key, Self::me_gauge(Some((member.hp, member.max_hp))));
                    }
                    "barMP" => {
                        self.party_gauges.insert(key, Self::me_gauge(Some((member.mp, member.max_mp))));
                    }
                    _ => {}
                }
            }
        }
    }

    fn bind_party(&mut self, view: &HudView) -> Option<Binding> {
        if view.world.party.members.is_empty() {
            return None;
        }
        let mut binding = Binding::default();
        for slot in 0..PARTY_SLOTS {
            let row_name = if slot == 0 { "PartyStatusWnd0".to_string() } else { format!("PartyStatusWnd{slot}") };
            if slot >= view.world.party.members.len() {
                binding.hidden.insert(row_name);
            }
            let summon_name = if slot == 0 { "PartyStatusSummonWnd0".to_string() } else { format!("PartyStatusSummonWnd{slot}") };
            binding.hidden.insert(summon_name);
        }
        Some(binding)
    }

    fn bind_minimap(&mut self, _view: &HudView) -> Option<Binding> {
        self.report("MinimapWnd: pontos do radar (jogador/NPCs/grupo) não desenhados - a conversão posição de mundo -> pixel (MinimapCtrlHandle::AdjustMapView/AddTarget) é nativa em nwindow.dll, sem fórmula em UnrealScript (ver docs/evidence/h5-hud-ui.md)");
        Some(Binding::default())
    }

    pub fn paint(&mut self, view: &HudView) -> DrawList {
        let mut list = DrawList::default();
        self.frame.clear();
        self.window_rects.clear();
        let stack = self.stack.clone();
        for name in stack {
            if !self.is_open(&name, view) {
                continue;
            }
            self.paint_window(&name, view, &mut list);
        }
        if let Some(confirm) = self.confirm {
            self.paint_dialog(confirm, view, &mut list);
        }
        list
    }

    fn bind_plan(&mut self, plan: Option<crate::hud_h5_system::WindowPlan>) -> Option<Binding> {
        let plan = plan?;
        for what in self.system.drain_reports() {
            self.report(what);
        }
        let mut binding = Binding { tab_selected: plan.tab_selected, ..Binding::default() };
        for control in plan.hide {
            binding.hidden.insert(control.to_string());
        }
        Some(binding)
    }

    fn bind_fill(&mut self, fill: Option<crate::hud_h5_windows::WindowFill>) -> Option<Binding> {
        let fill = fill?;
        for what in &fill.reports {
            self.report(what);
        }
        let mut binding = Binding::default();
        for control in fill.hidden {
            binding.hidden.insert(control);
        }
        for (control, text) in fill.values {
            binding.values.insert(control, text);
        }
        Some(binding)
    }

    fn paint_window(&mut self, name: &str, view: &HudView, list: &mut DrawList) {
        let Some(window) = self.interface.window(name).cloned() else {
            self.report(&format!("janela {name} sem definição no xdat; não desenhada"));
            return;
        };
        let binding = match name {
            STATUS => self.bind_status(view),
            TARGET => self.bind_target(view),
            SHORTCUT => self.bind_shortcut(view),
            CHAT => self.bind_chat(view),
            SYSTEM_MENU => self.bind_system_menu(view),
            INVENTORY => self.bind_inventory(view),
            ABNORMAL => self.bind_abnormal(view),
            PARTY => self.bind_party(view),
            MINIMAP => self.bind_minimap(view),
            crate::hud_h5_windows::DETAIL_STATUS => self.bind_fill(crate::hud_h5_windows::bind_detail_status(view)),
            crate::hud_h5_windows::MAGIC_SKILL => self.bind_fill(Some(crate::hud_h5_windows::bind_magic_skill(view))),
            crate::hud_h5_windows::ACTION => self.bind_fill(Some(crate::hud_h5_windows::bind_action(view))),
            name if crate::hud_h5_social::is_social(name) => self.bind_fill(crate::hud_h5_social::bind(name, view, self.social_tab)),
            crate::hud_h5_system::BOARD => {
                let plan = crate::hud_h5_system::plan_board(&mut self.system, view);
                self.bind_plan(plan)
            }
            crate::hud_h5_system::HELP_HTML => {
                let plan = crate::hud_h5_system::plan_help(&mut self.system, view);
                self.bind_plan(plan)
            }
            crate::hud_h5_system::OPTION => {
                let plan = crate::hud_h5_system::plan_option(&mut self.system, view);
                self.bind_plan(plan)
            }
            crate::hud_h5_system::GM => {
                let plan = crate::hud_h5_system::plan_gm(&mut self.system, view);
                self.bind_plan(plan)
            }
            _ => {
                self.report(&format!("janela {name} do xdat ainda sem lógica portada; não desenhada"));
                return;
            }
        };
        let Some(binding) = binding else {
            return;
        };
        let mut layout = self.layout_of(&window, view, (binding.width, binding.height));
        self.window_rects.insert(name.to_string(), layout.rect);
        if name == SHORTCUT {
            self.arrange_shortcut_drawers(&mut layout);
            self.prepare_shortcut_rows(&layout, view);
        }
        if name == PARTY {
            self.prepare_party_rows(&layout, view);
        }
        let missing = self.draw_layout(&layout, &binding, view, list);
        if name == ABNORMAL
            && let Some(abnormal) = view.abnormal
        {
            draw_abnormal_icons(layout.rect, abnormal, view, list);
        }
        if name == STATUS {
            draw_status_numbers(&layout, view, list);
        }
        if name == INVENTORY
            && let Some(inventory) = view.inventory
        {
            draw_weight_number(&layout, inventory, list);
        }
        crate::hud_h5_social::draw(name, &layout, view, self.social_tab, self.social_page, list);
        if name == crate::hud_h5_system::BOARD {
            crate::hud_h5_system::draw_board(&mut self.system, &layout, view, list);
        }
        if name == crate::hud_h5_system::HELP_HTML {
            crate::hud_h5_system::draw_help(&mut self.system, &layout, view, list);
        }
        self.register(name, &layout, &binding, view);
        drop(layout);
        for texture in missing {
            if !self.missing.contains(&texture) {
                log::warn!("HUD H5: textura do xdat ausente no catálogo: {texture}");
                self.missing.push(texture);
            }
        }
    }

    fn draw_layout(&self, layout: &Layout, binding: &Binding, view: &HudView, list: &mut DrawList) -> Vec<String> {
        let texture_size = |name: &str| self.size(name);
        let hidden = |name: &str| binding.is_hidden(name);
        let gauge = |name: &str| binding.gauges.get(name).copied();
        let value = |name: &str| binding.values.get(name).cloned();
        let color = |name: &str| binding.colors.get(name).copied();
        let texture = |name: &str| binding.textures.get(name).cloned();
        let disabled = |name: &str| binding.disabled.contains(name);
        let pressed = self.press.as_ref().and_then(|press| match &press.region {
            Some(RegionKind::Button(name)) => Some(name.as_str()),
            _ => None,
        });
        let custom = |item: &Item, list: &mut DrawList| self.draw_custom(item, binding, view, list);
        let paint = Paint {
            font: view.font,
            texture_size: &texture_size,
            sysstring: view.sysstring,
            cursor: Some(self.cursor),
            pressed,
            disabled: &disabled,
            hidden: &hidden,
            gauge: &gauge,
            value: &value,
            color: &color,
            texture: &texture,
            custom: &custom,
        };
        let missing = xdat_window::paint_layout(layout, &paint, list);
        for (button, text) in &binding.captions {
            if let Some(item) = layout.find(button)
                && !binding.is_hidden(button)
            {
                xdat_window::draw_aligned(list, item.rect, text, TEXT, TextAlign::Center, TextVAlign::Middle, view.font);
            }
        }
        missing
    }

    fn draw_custom(&self, item: &Item, binding: &Binding, view: &HudView, list: &mut DrawList) -> bool {
        let control = item.control;
        let name = control.head.name.as_str();
        match control.kind.as_str() {
            "NameCtrl" => {
                let key = control as *const Control as usize;
                let found = self.party_names.get(&key).or_else(|| binding.names.get(name));
                if let Some((text, color, align)) = found {
                    let rect = Rect::new(item.rect.x, item.rect.y, item.rect.w.max(1), view.font.line_height as i32);
                    xdat_window::draw_aligned(list, rect, text, *color, *align, TextVAlign::Middle, view.font);
                }
                true
            }
            "Texture" => {
                let key = control as *const Control as usize;
                if let Some(icon) = self.party_icons.get(&key) {
                    list.draw_texture(item.rect.x, item.rect.y, item.rect.w, item.rect.h, 0, 0, item.rect.w, item.rect.h, icon, 1.0);
                    true
                } else {
                    false
                }
            }
            "BarCtrl" => {
                let key = control as *const Control as usize;
                match (self.party_gauges.get(&key), &control.props) {
                    (Some(&ratio), Props::BarCtrl { textures, .. }) => {
                        let parts = [&textures[0], &textures[1], &textures[2], &textures[3], &textures[4], &textures[5]].map(String::as_str);
                        xdat_window::bar(list, item.rect, parts, Some(ratio), &|name| self.size(name), &mut Vec::new());
                        true
                    }
                    _ => false,
                }
            }
            "ShortcutItemWindow" => {
                if let Some(slot) = self.shortcut_views.get(&(control as *const Control as usize)) {
                    if let Some(icon) = &slot.icon {
                        let (x, y) = (item.rect.x + (item.rect.w - 32) / 2, item.rect.y + (item.rect.h - 32) / 2);
                        list.draw_texture(x, y, 32, 32, 0, 0, 32, 32, icon, 1.0);
                    }
                    if let Some(count) = &slot.count {
                        let width = view.font.measure(count) as i32;
                        list.draw_text(item.rect.x + item.rect.w - width - 2, item.rect.y + item.rect.h - view.font.line_height as i32 - 2, TEXT, count, view.font);
                    }
                }
                true
            }
            "TextBox" if name == "PageNumTextBox" => match self.shortcut_page_text.get(&(control as *const Control as usize)) {
                Some(text) => {
                    let rect = Rect::new(item.rect.x, item.rect.y, item.rect.w.max(1), view.font.line_height as i32);
                    xdat_window::draw_aligned(list, rect, text, TEXT, TextAlign::Center, TextVAlign::Middle, view.font);
                    true
                }
                None => false,
            },
            "ChatWindow" => {
                if binding.chat_active == Some(name) {
                    self.draw_chat_lines(item.rect, binding, view, list);
                }
                true
            }
            "EditBox" => {
                if let Some((edit, text, focused, cursor)) = &binding.edit
                    && edit == name
                {
                    let line = view.font.line_height as i32;
                    let y = item.rect.y + (item.rect.h - line) / 2;
                    let start = list.quads.len();
                    list.draw_text(item.rect.x, y, TEXT, text, view.font);
                    if *focused {
                        let before: String = text.chars().take(*cursor).collect();
                        let x = item.rect.x + view.font.measure(&before) as i32;
                        list.draw_fill(x, y, 1, line, TEXT);
                    }
                    list.clip_from(start, item.rect);
                }
                true
            }
            "Tab" => {
                self.draw_tabs(item, binding, view, list);
                true
            }
            "ItemWindow" => {
                self.draw_item_window(item, view, list);
                true
            }
            _ => false,
        }
    }

    fn draw_item_window(&self, item: &Item, view: &HudView, list: &mut DrawList) {
        if crate::hud_h5_windows::draw_action_list(item, view, list) {
            return;
        }
        let Some(inventory) = view.inventory else { return };
        let name = item.control.head.name.as_str();
        let (slot_back, frame) = match &item.control.props {
            Props::ItemWindow(props) => (props.slot.as_str(), props.frame.as_str()),
            _ => (xdat_window::UNDEFINED, xdat_window::UNDEFINED),
        };
        if name == "InventoryItem" || name == "QuestItem" {
            draw_item_grid(item.rect, inventory, name == "QuestItem", self.inventory_scroll, (slot_back, frame), view.font, list);
        } else if let Some(slot) = equip_slot_index(name)
            && let Some(equipped) = inventory.paperdoll_item(slot)
        {
            let icon = equipped.icon(0);
            list.draw_texture(item.rect.x, item.rect.y, item.rect.w, item.rect.h, 0, 0, item.rect.w, item.rect.h, icon, 1.0);
            if xdat_window::UNDEFINED != frame && !frame.is_empty() {
                list.draw_texture(item.rect.x, item.rect.y, item.rect.w, item.rect.h, 0, 0, item.rect.w, item.rect.h, frame, 1.0);
            }
        }
    }

    fn tab_rects(&self, item: &Item) -> Vec<(Rect, u32, [String; 2])> {
        let Props::Tab(pages) = &item.control.props else { return Vec::new() };
        let mut x = item.rect.x;
        let even = if pages.is_empty() { 0 } else { item.rect.w / pages.len() as i32 };
        pages
            .iter()
            .map(|page| {
                let width = if page.extent[0] > 0 { page.extent[0] } else { even };
                let rect = Rect::new(x, item.rect.y, width, item.rect.h);
                x += width;
                (rect, page.id.max(0) as u32, page.textures.clone())
            })
            .collect()
    }

    fn draw_tabs(&self, item: &Item, binding: &Binding, view: &HudView, list: &mut DrawList) {
        for (index, (rect, id, textures)) in self.tab_rects(item).into_iter().enumerate() {
            let selected = index == binding.tab_selected;
            let name = &textures[usize::from(selected)];
            if let Some((w, h)) = self.size(name) {
                list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, w, h, name, 1.0);
            }
            if let Some(label) = (view.sysstring)(id) {
                xdat_window::draw_aligned(list, rect, &label, TEXT, TextAlign::Center, TextVAlign::Middle, view.font);
            }
        }
    }

    fn draw_chat_lines(&self, rect: Rect, binding: &Binding, view: &HudView, list: &mut DrawList) {
        let line_height = view.font.line_height as i32;
        if line_height <= 0 || rect.h <= 0 {
            return;
        }
        let rows = (rect.h / line_height).max(1) as usize;
        let mut visual: Vec<(String, Argb)> = Vec::new();
        for (text, color) in &binding.chat_lines {
            for part in wrap(text, rect.w, view.font) {
                visual.push((part, *color));
            }
        }
        let scroll = self.chat.scroll[self.chat.tab.min(CHAT_TABS - 1)].min(visual.len().saturating_sub(rows));
        let end = visual.len().saturating_sub(scroll);
        let begin = end.saturating_sub(rows);
        let start = list.quads.len();
        for (row, (text, color)) in visual[begin..end].iter().enumerate() {
            let y = rect.y + rect.h - (end - begin - row) as i32 * line_height;
            list.draw_text(rect.x, y, *color, text, view.font);
        }
        list.clip_from(start, rect);
    }

    fn register(&mut self, name: &str, layout: &Layout, binding: &Binding, view: &HudView) {
        let hidden = |item_name: &str| binding.is_hidden(item_name);
        let items = xdat_window::visible_items(layout, &hidden);
        let mut regions = Vec::new();
        let mut bounds = layout.rect;
        let mut drag_zone = layout.rect;
        if let Some(child) = binding.bounds_child
            && let Some(item) = items.iter().find(|item| item.control.head.name == child)
        {
            bounds = item.rect;
            drag_zone = item.rect;
        }
        if name == CHAT {
            drag_zone = Rect::new(layout.rect.x, layout.rect.y, layout.rect.w, 18);
        }
        if name == SHORTCUT {
            for item in &items {
                if SHORTCUT_VERTICAL_ROWS.contains(&item.control.head.name.as_str()) || SHORTCUT_HORIZONTAL_ROWS.contains(&item.control.head.name.as_str()) {
                    bounds = union_rect(bounds, item.rect);
                }
            }
            drag_zone = bounds;
        }
        regions.push(Region { rect: bounds, kind: RegionKind::Body });
        for item in &items {
            let control_name = item.control.head.name.as_str();
            match &item.control.props {
                Props::Button(_) if !binding.disabled.contains(control_name) => {
                    regions.push(Region { rect: item.rect, kind: RegionKind::Button(control_name.to_string()) });
                }
                Props::ShortcutItemWindow(_) => {
                    if let Some(&slot) = self.shortcut_slot_num.get(&(item.control as *const Control as usize)) {
                        regions.push(Region { rect: item.rect, kind: RegionKind::Slot(slot) });
                    }
                }
                Props::Tab(_) => {
                    for (index, (rect, _, _)) in self.tab_rects(item).into_iter().enumerate() {
                        regions.push(Region { rect, kind: RegionKind::Tab(index) });
                    }
                }
                Props::EditBox(_) if control_name == "ChatEditBox" => {
                    regions.push(Region { rect: item.rect, kind: RegionKind::Edit });
                }
                Props::ItemWindow(_) if control_name == "InventoryItem" || control_name == "QuestItem" => {
                    if let Some(inventory) = view.inventory {
                        let quest = control_name == "QuestItem";
                        let items = inventory_items(inventory, quest);
                        let scroll = inventory_scroll_clamped(self.inventory_scroll, items.len());
                        let start = scroll as usize * INVENTORY_COLS as usize;
                        for (cell, inv_item) in items.iter().enumerate().skip(start).take(INVENTORY_VISIBLE as usize) {
                            let rect = item_cell_rect(item.rect, (cell - start) as i32);
                            regions.push(Region { rect, kind: RegionKind::ItemSlot(inv_item.object_id) });
                        }
                    }
                }
                Props::ItemWindow(_) => {
                    if let Some(slot) = equip_slot_index(control_name) {
                        regions.push(Region { rect: item.rect, kind: RegionKind::Paperdoll(slot) });
                    }
                }
                _ => {}
            }
        }
        self.frame.push(HitWindow { name: name.to_string(), bounds, drag_zone, regions });
    }

    fn me_gauge(pair: Option<(i32, i32)>) -> f32 {
        match pair {
            Some((cur, max)) if max > 0 => (cur as f32 / max as f32).clamp(0.0, 1.0),
            _ => 0.0,
        }
    }

    fn bind_status(&mut self, view: &HudView) -> Option<Binding> {
        let me = view.world.me.as_ref()?;
        let mut binding = Binding::default();
        binding.hide(&["LifeForceAnimTex_Left", "LifeForceAnimTex_Center", "LifeForceAnimTex_Right", "NavitGaugeLeft", "NavitGaugeMid", "NavitGaugeRight", "WndLevelBackPremium", "Statustooltipwnd"]);
        binding.gauges.insert("HPBar".into(), Self::me_gauge(me.hp));
        binding.gauges.insert("MPBar".into(), Self::me_gauge(me.mp));
        binding.gauges.insert("CPBar".into(), Self::me_gauge(me.cp));
        binding.values.insert("StatusWnd_LevelTextBox".into(), me.level.to_string());
        binding.names.insert("UserName".into(), (me.name.clone(), TEXT, TextAlign::Left));
        match view.world.user_extras {
            Some(extras) => {
                binding.gauges.insert("EXPBar".into(), (extras.exp_percent / 100.0).clamp(0.0, 1.0));
                let (texture, ratio) = vitality_level(extras.vitality);
                binding.textures.insert("LifeForceTex".into(), format!("{VITALITY_TEXTURE}{texture}"));
                binding.gauges.insert("VpDetailBar".into(), ratio);
            }
            None => {
                binding.hide(&["EXPBar", "LifeForceTex", "VpDetailBar"]);
                self.report("StatusWnd: EXP e vitalidade sem dado (UserInfo H5 ainda não trouxe os extras); barras não desenhadas");
            }
        }
        Some(binding)
    }

    fn target_flags(&self, view: &HudView, entity: &Entity) -> (bool, bool) {
        let me = view.world.me.as_ref().is_some_and(|me| me.object_id == entity.object_id);
        let own_pet = view.world.pet.as_ref().is_some_and(|pet| pet.info.object_id == entity.object_id);
        match &entity.kind {
            EntityKind::Npc { .. } if own_pet => (true, true),
            EntityKind::Npc { .. } => (entity.hp.is_some(), false),
            _ if me => (true, true),
            _ => (false, false),
        }
    }

    fn bind_target(&mut self, view: &HudView) -> Option<Binding> {
        let entity = view.world.targeted()?;
        let mut binding = Binding::default();
        let (hp_bar, mp_bar) = self.target_flags(view, entity);
        let name = view.tables.map_or_else(|| entity.name.clone(), |tables| crate::characters::display_name(entity, tables));
        let my_level = view.world.me.as_ref().map_or(0, |me| me.level);
        let colored = hp_bar && !matches!(entity.kind, EntityKind::Player { .. });
        let color = if colored { level_color(entity.level - my_level, my_level) } else { TEXT };
        binding.names.insert("UserName".into(), (name, color, TextAlign::Center));
        binding.names.insert("RankName".into(), (String::new(), TEXT, TextAlign::Center));
        binding.hide(&["texAltitude", "texDist", "NpcInfo", "texPledgeCrest", "texPledgeAllianceCrest"]);
        if hp_bar {
            binding.gauges.insert("barHP".into(), Self::me_gauge(entity.hp));
        } else {
            binding.hide(&["barHP"]);
        }
        if mp_bar {
            binding.gauges.insert("barMP".into(), Self::me_gauge(entity.mp));
        } else {
            binding.hide(&["barMP"]);
        }
        let npc = matches!(entity.kind, EntityKind::Npc { .. }) && view.world.pet.as_ref().is_none_or(|pet| pet.info.object_id != entity.object_id);
        if self.target_expanded {
            binding.hide(&["btnContract"]);
            binding.height = Some(TARGET_EXPAND_HEIGHT);
        } else {
            binding.hide(&["btnExpand"]);
            binding.height = Some(TARGET_CONTRACT_HEIGHT);
        }
        let pledge_visible = self.target_expanded && !npc && entity.clan_id >= 0;
        if pledge_visible {
            let clan = if entity.clan_id > 0 { view.world.pledge_name(entity.clan_id) } else { None };
            let ally = if entity.clan_id > 0 { view.world.ally_name(entity.clan_id) } else { None };
            let known = rgb(176, 152, 121);
            let known_ally = rgb(176, 155, 121);
            let grey = rgb(128, 128, 128);
            binding.values.insert("txtPledgeName".into(), clan.map(str::to_string).or_else(|| (view.sysstring)(SYSSTRING_PLEDGE)).unwrap_or_default());
            binding.values.insert("txtPledgeAllianceName".into(), ally.map(str::to_string).or_else(|| (view.sysstring)(SYSSTRING_ALLIANCE)).unwrap_or_default());
            binding.colors.insert("txtPledgeName".into(), if clan.is_some() { known } else { grey });
            binding.colors.insert("txtPledgeAllianceName".into(), if ally.is_some() { known_ally } else { grey });
            self.report("TargetStatusWnd: brasões de clã/aliança (textura dinâmica) ainda não ligados");
        } else {
            binding.hide(&["txtPledge", "txtAlliance", "txtPledgeName", "txtPledgeAllianceName"]);
        }
        if self.target_expanded && npc {
            self.report("TargetStatusWnd: árvore NpcInfo (UIDATA_NPC.GetNpcProperty) ainda não ligada");
        }
        Some(binding)
    }

    fn bind_shortcut(&mut self, _view: &HudView) -> Option<Binding> {
        let mut binding = Binding::default();
        for row in SHORTCUT_VERTICAL_ROWS.into_iter().chain(SHORTCUT_HORIZONTAL_ROWS) {
            let needed = match row {
                row if row.ends_with("_1") => 1,
                row if row.ends_with("_2") => 2,
                row if row.ends_with("_3") => 3,
                _ => 0,
            };
            let is_vertical_row = row.starts_with(SHORTCUT_VERTICAL_ROWS[0]);
            if is_vertical_row != self.vertical || self.expand_level < needed {
                binding.hidden.insert(row.to_string());
            }
        }
        binding.hide(&["ShortcutWndJoypad", "ShortcutWndJoypadExpand"]);
        binding.hide(&["ReduceButton", "JoypadBtn"]);
        if self.tooltip_min {
            binding.hide(&["TooltipMaxBtn"]);
        } else {
            binding.hide(&["TooltipMinBtn"]);
        }
        if self.locked {
            binding.hide(&["UnlockBtn"]);
        } else {
            binding.hide(&["LockBtn"]);
        }
        binding.bounds_child = Some(if self.vertical { "ShortcutWndVertical" } else { "ShortcutWndHorizontal" });
        Some(binding)
    }

    fn bind_chat(&mut self, view: &HudView) -> Option<Binding> {
        let tab = self.chat.tab.min(CHAT_TABS - 1);
        let mut binding = Binding::default();
        for (index, window) in CHAT_WINDOWS.iter().enumerate() {
            if index != tab {
                binding.hidden.insert(window.to_string());
            }
        }
        binding.chat_active = Some(CHAT_WINDOWS[tab]);
        binding.tab_selected = tab;
        binding.hide(&["LanguageTexture"]);
        let palette = |kind: i32| {
            let index = if kind == CHAT_SYSTEM { SYSTEM_TYPE_PALETTE } else { kind };
            self.interface.palette.iter().find(|entry| entry.index == index).map(|entry| Argb(entry.color)).unwrap_or(Argb(self.interface.fonts.default_color))
        };
        for line in view.chat {
            if !chat_passes(tab, line) {
                continue;
            }
            let color = line.color.map(Argb).unwrap_or_else(|| palette(line.kind));
            binding.chat_lines.push((chat_text(line), color));
        }
        binding.edit = Some(("ChatEditBox".into(), self.chat.input.text.clone(), self.chat.focused, self.chat.input.cursor));
        Some(binding)
    }

    fn bind_system_menu(&mut self, view: &HudView) -> Option<Binding> {
        let mut binding = Binding::default();
        binding.hide(&["btnProductInventory"]);
        let bbs = format!("{}(Alt+B)", (view.sysstring)(SYSSTRING_BBS).unwrap_or_default());
        let macro_text = format!("{}(Alt+R)", (view.sysstring)(SYSSTRING_MACRO).unwrap_or_default());
        binding.values.insert("txtBBS".into(), bbs);
        binding.values.insert("txtMacro".into(), macro_text);
        Some(binding)
    }

    fn bind_inventory(&mut self, view: &HudView) -> Option<Binding> {
        let inventory = view.inventory?;
        let mut binding = Binding { tab_selected: self.inventory_tab, ..Binding::default() };
        binding.hide(&[if self.inventory_tab == 0 { "QuestItem" } else { "InventoryItem" }]);
        binding.values.insert("AdenaText".into(), killer_ui::inventory::thousands(inventory.adena));
        if inventory.max_load > 0 {
            binding.gauges.insert("InvenWeight".into(), (inventory.load as f32 / inventory.max_load as f32).clamp(0.0, 1.0));
        }
        Some(binding)
    }

    fn bind_abnormal(&mut self, view: &HudView) -> Option<Binding> {
        let abnormal = view.abnormal?;
        let count = abnormal_entries(abnormal).len() as i32;
        if count == 0 {
            return None;
        }
        let rows = (count + ABNORMAL_COLS - 1) / ABNORMAL_COLS;
        let cols = if rows <= 1 { count } else { ABNORMAL_COLS };
        let mut binding = Binding { width: Some(cols * ABNORMAL_ICON), height: Some(rows * ABNORMAL_ICON), ..Binding::default() };
        binding.hide(&["StatusIcon"]);
        Some(binding)
    }

    fn paint_dialog(&mut self, confirm: Confirm, view: &HudView, list: &mut DrawList) {
        let Some(window) = self.interface.window(DIALOG).cloned() else {
            self.report("DialogBox sem definição no xdat; confirmação não desenhada");
            return;
        };
        let message_id = if confirm == Confirm::Restart { SYSMSG_RESTART } else { SYSMSG_EXIT };
        let text = view.tables.and_then(|tables| tables.systemmsg.get(&message_id)).map(|message| message.text.clone()).unwrap_or_default();
        let mut binding = Binding::default();
        binding.hide(&["DialogReadingText", "DialogBoxEdit", "CenterOKButton", "DialogProgress", "NumberPad"]);
        binding.values.insert("DialogText".into(), text);
        binding.captions.push(("OKButton".into(), (view.sysstring)(DIALOG_BUTTON_OK).unwrap_or_default()));
        binding.captions.push(("CancelButton".into(), (view.sysstring)(DIALOG_BUTTON_CANCEL).unwrap_or_default()));
        let layout = self.layout_of(&window, view, (None, None));
        let missing = self.draw_layout(&layout, &binding, view, list);
        self.register(DIALOG, &layout, &binding, view);
        drop(layout);
        for texture in missing {
            if !self.missing.contains(&texture) {
                log::warn!("HUD H5: textura do xdat ausente no catálogo: {texture}");
                self.missing.push(texture);
            }
        }
    }

    fn window_at(&self, x: i32, y: i32) -> Option<usize> {
        if self.confirm.is_some() {
            return self.frame.iter().rposition(|window| window.name == DIALOG);
        }
        self.frame.iter().rposition(|window| window.bounds.contains(x, y))
    }

    pub fn covers(&self, x: i32, y: i32) -> bool {
        self.confirm.is_some() || self.window_at(x, y).is_some()
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) -> bool {
        self.cursor = (x, y);
        let Some(press) = self.press.as_mut() else {
            return self.covers(x, y);
        };
        let (dx, dy) = (x - press.start.0, y - press.start.1);
        if !press.dragging && (dx.abs() >= DRAG_THRESHOLD || dy.abs() >= DRAG_THRESHOLD) && press.region == Some(RegionKind::Body) {
            press.dragging = true;
        }
        if press.dragging {
            let origin = press.origin;
            let window = press.window.clone();
            self.offsets.insert(window, (origin.0 + dx, origin.1 + dy));
        }
        true
    }

    pub fn mouse_down(&mut self, x: i32, y: i32) -> bool {
        self.cursor = (x, y);
        let Some(index) = self.window_at(x, y) else {
            return false;
        };
        let hit = &self.frame[index];
        let name = hit.name.clone();
        let region = hit.regions.iter().rev().find(|region| region.rect.contains(x, y)).map(|region| region.kind.clone());
        let region = match region {
            Some(RegionKind::Body) if !hit.drag_zone.contains(x, y) => None,
            other => other,
        };
        if name != DIALOG {
            self.raise(&name);
        }
        let origin = self.offsets.get(&name).copied().unwrap_or((0, 0));
        self.press = Some(Press { window: name, region, start: (x, y), origin, dragging: false });
        true
    }

    pub fn mouse_up(&mut self, x: i32, y: i32) -> (bool, Option<HudAction>) {
        self.cursor = (x, y);
        let Some(press) = self.press.take() else {
            return (self.covers(x, y), None);
        };
        if press.dragging {
            return (true, None);
        }
        let Some(hit) = self.frame.iter().find(|window| window.name == press.window) else {
            return (true, None);
        };
        let still = hit.regions.iter().rev().find(|region| region.rect.contains(x, y)).map(|region| region.kind.clone());
        let bounds = hit.bounds;
        match (&press.region, still) {
            (Some(RegionKind::Button(pressed)), Some(RegionKind::Button(released))) if *pressed == released => {
                let action = self.click_button(&press.window, pressed);
                (true, action)
            }
            (Some(RegionKind::Slot(slot)), Some(RegionKind::Slot(released))) if *slot == released => (true, Some(HudAction::UseShortcut(*slot))),
            (Some(RegionKind::Tab(index)), Some(RegionKind::Tab(released))) if *index == released => {
                match press.window.as_str() {
                    CHAT => self.chat.tab = *index,
                    INVENTORY => {
                        self.inventory_tab = *index;
                        self.inventory_scroll = 0;
                    }
                    _ => {}
                }
                (true, None)
            }
            (Some(RegionKind::Edit), Some(RegionKind::Edit)) => {
                self.chat.focused = true;
                (true, None)
            }
            (Some(RegionKind::ItemSlot(object_id)), Some(RegionKind::ItemSlot(released))) if *object_id == released => {
                let now = Instant::now();
                let double = self.last_item_click.is_some_and(|(id, at)| id == *object_id && now.saturating_duration_since(at).as_millis() <= DOUBLE_CLICK_MS);
                self.last_item_click = Some((*object_id, now));
                self.inventory_selected = Some(*object_id);
                (true, double.then_some(HudAction::UseItem(*object_id)))
            }
            (Some(RegionKind::Paperdoll(slot)), Some(RegionKind::Paperdoll(released))) if *slot == released => {
                let now = Instant::now();
                let double = self.last_paperdoll_click.is_some_and(|(s, at)| s == *slot && now.saturating_duration_since(at).as_millis() <= DOUBLE_CLICK_MS);
                self.last_paperdoll_click = Some((*slot, now));
                (true, double.then_some(HudAction::UnequipPaperdoll(*slot)))
            }
            (Some(RegionKind::Body), _) if press.window == STATUS && bounds.contains(x, y) => {
                let inside = x > bounds.x + SELF_TARGET_MARGIN.0 && x < bounds.x + bounds.w - SELF_TARGET_MARGIN.1;
                (true, inside.then_some(HudAction::SelfTarget))
            }
            _ => (true, None),
        }
    }

    fn click_button(&mut self, window: &str, button: &str) -> Option<HudAction> {
        match (window, button) {
            (DIALOG, "OKButton") => {
                let confirm = self.confirm.take()?;
                Some(if confirm == Confirm::Restart { HudAction::Restart } else { HudAction::Quit })
            }
            (DIALOG, "CancelButton") => {
                self.confirm = None;
                None
            }
            (window, button) if crate::hud_h5_social::is_social(window) => {
                match crate::hud_h5_social::click(window, button)? {
                    crate::hud_h5_social::SocialClick::Toggle(target) => self.toggle(target),
                    crate::hud_h5_social::SocialClick::QuestTab(tab) => self.social_tab = tab,
                    crate::hud_h5_social::SocialClick::PartyMatchPage(step) => {
                        self.social_page = (self.social_page as i32 + step).max(0) as usize;
                    }
                    crate::hud_h5_social::SocialClick::Handled => {}
                }
                None
            }
            (TARGET, "btnClose") => Some(HudAction::CancelTarget),
            (TARGET, "btnExpand") => {
                self.target_expanded = false;
                None
            }
            (TARGET, "btnContract") => {
                self.target_expanded = true;
                None
            }
            (SHORTCUT, "PrevBtn") => {
                self.page = (self.page - 1).rem_euclid(SHORTCUT_PAGES);
                None
            }
            (SHORTCUT, "NextBtn") => {
                self.page = (self.page + 1).rem_euclid(SHORTCUT_PAGES);
                None
            }
            (SHORTCUT, "PrevBtn2") => {
                self.page2 = (self.page2 - 1).rem_euclid(SHORTCUT_PAGES);
                None
            }
            (SHORTCUT, "NextBtn2") => {
                self.page2 = (self.page2 + 1).rem_euclid(SHORTCUT_PAGES);
                None
            }
            (SHORTCUT, "PrevBtn3") => {
                self.page3 = (self.page3 - 1).rem_euclid(SHORTCUT_PAGES);
                None
            }
            (SHORTCUT, "NextBtn3") => {
                self.page3 = (self.page3 + 1).rem_euclid(SHORTCUT_PAGES);
                None
            }
            (SHORTCUT, "PrevBtn4") => {
                self.page4 = (self.page4 - 1).rem_euclid(SHORTCUT_PAGES);
                None
            }
            (SHORTCUT, "NextBtn4") => {
                self.page4 = (self.page4 + 1).rem_euclid(SHORTCUT_PAGES);
                None
            }
            (SHORTCUT, "LockBtn" | "UnlockBtn") => {
                self.locked = !self.locked;
                None
            }
            (SHORTCUT, "RotateBtn") => {
                self.vertical = !self.vertical;
                None
            }
            (SHORTCUT, "TooltipMinBtn" | "TooltipMaxBtn") => {
                self.tooltip_min = !self.tooltip_min;
                None
            }
            (SHORTCUT, "ExpandButton" | "ReduceButton") => {
                self.expand_level = if self.expand_level >= 3 { 0 } else { self.expand_level + 1 };
                None
            }
            (SYSTEM_MENU, "btnRestart") => {
                self.confirm = Some(Confirm::Restart);
                None
            }
            (SYSTEM_MENU, "btnQuit") => {
                self.confirm = Some(Confirm::Quit);
                None
            }
            (SYSTEM_MENU, target) => {
                let window = match target {
                    "btnPost" => "PostBoxWnd",
                    "btnBBS" => "BoardWnd",
                    "btnMacro" => "MacroListWnd",
                    "btnHelpHtml" => "HelpHtmlWnd",
                    "btnPetition" => "UserPetitionWnd",
                    "btnOption" => "OptionWnd",
                    _ => {
                        self.report(&format!("SystemMenuWnd: {target} (diálogo 3208 / link externo) ainda não ligado"));
                        return None;
                    }
                };
                self.toggle(window);
                None
            }
            (CHAT, other) => {
                self.report(&format!("ChatWnd: botão {other} (ChatFilterWnd/Messenger/PartyMatch) ainda não ligado"));
                None
            }
            (window, button) => {
                self.report(&format!("{window}: botão {button} sem lógica portada"));
                None
            }
        }
    }

    pub fn wheel(&mut self, x: i32, y: i32, ticks: f32) -> bool {
        let Some(index) = self.window_at(x, y) else { return false };
        match self.frame[index].name.as_str() {
            CHAT => {
                let tab = self.chat.tab.min(CHAT_TABS - 1);
                if ticks > 0.0 {
                    self.chat.scroll[tab] = self.chat.scroll[tab].saturating_add(ticks.ceil() as usize);
                } else if ticks < 0.0 {
                    self.chat.scroll[tab] = self.chat.scroll[tab].saturating_sub((-ticks).ceil() as usize);
                }
            }
            INVENTORY => {
                self.inventory_scroll = (self.inventory_scroll - ticks.round() as i32).max(0);
            }
            _ => {}
        }
        true
    }

    pub fn input(&mut self, input: InputKey) -> Option<HudAction> {
        if !self.chat.focused {
            return None;
        }
        match input {
            InputKey::Escape => {
                self.chat.focused = false;
                None
            }
            InputKey::Enter => {
                let typed = self.chat.input.key(InputKey::Enter)?;
                self.chat.focused = false;
                let message = killer_ui::chat::chat_message(&typed, self.chat.tab)?;
                Some(HudAction::Say { text: message.text, kind: message.kind, target: message.target })
            }
            other => {
                self.chat.input.key(other);
                None
            }
        }
    }

    pub fn hotkey(&mut self, vk: u32, pressed: bool, repeat: bool) -> (KeyOutcome, Option<HudAction>) {
        if self.confirm.is_some() {
            return (KeyOutcome::Consumed, None);
        }
        if self.chat.focused {
            return (KeyOutcome::Consumed, None);
        }
        if !pressed || repeat {
            return (KeyOutcome::Ignored, None);
        }
        let alt = if self.modifiers.alt { 18 } else { 0 };
        let shift = if self.modifiers.shift { 16 } else { 0 };
        if self.modifiers.ctrl {
            return (KeyOutcome::Ignored, None);
        }
        let Some(command) = self.keys.iter().find(|entry| entry.key == vk && entry.edge == 1 && entry.modifier_a == alt && entry.modifier_b == shift).map(|entry| entry.command.clone()) else {
            return (KeyOutcome::Ignored, None);
        };
        self.run_command(&command)
    }

    fn run_command(&mut self, command: &str) -> (KeyOutcome, Option<HudAction>) {
        let mut parts = command.split_whitespace();
        let Some(verb) = parts.next() else { return (KeyOutcome::Ignored, None) };
        let argument = |parts: &mut std::str::SplitWhitespace, name: &str| parts.next().and_then(|part| part.strip_prefix(name)).map(str::to_string);
        match verb {
            "ShowWindow" => {
                let Some(name) = argument(&mut parts, "Name=") else { return (KeyOutcome::Ignored, None) };
                if name.starts_with("FN_") {
                    self.report(&format!("atalho {name} (função do engine) sem equivalente portado"));
                    return (KeyOutcome::Consumed, None);
                }
                self.toggle(&name);
                (KeyOutcome::Consumed, None)
            }
            "SetShortcutPage" => {
                if let Some(page) = argument(&mut parts, "Num=").and_then(|n| n.parse::<i32>().ok())
                    && (0..SHORTCUT_PAGES).contains(&page)
                {
                    self.page = page;
                }
                (KeyOutcome::Consumed, None)
            }
            "UseShortcutItem" => {
                let Some(number) = argument(&mut parts, "Num=").and_then(|n| n.parse::<i32>().ok()) else { return (KeyOutcome::Ignored, None) };
                let slot = match number {
                    0..=11 => self.page * SHORTCUT_PER_PAGE + number,
                    12..=23 => self.page2 * SHORTCUT_PER_PAGE + (number - 12),
                    24..=35 => self.page3 * SHORTCUT_PER_PAGE + (number - 24),
                    _ => return (KeyOutcome::Consumed, None),
                };
                (KeyOutcome::Consumed, Some(HudAction::UseShortcut(slot)))
            }
            "TargetCancel" => (KeyOutcome::Consumed, Some(HudAction::CancelTarget)),
            "FocusChatWindow" => {
                self.chat.focused = true;
                (KeyOutcome::Consumed, None)
            }
            "TabShowInventoryWindow" => {
                self.toggle(INVENTORY);
                (KeyOutcome::Consumed, None)
            }
            "CloseAllWindow" => {
                self.shown.retain(|name| [STATUS, SHORTCUT, CHAT].contains(&name.as_str()));
                (KeyOutcome::Consumed, None)
            }
            _ => (KeyOutcome::Ignored, None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use killer_ui::abnormal::BuffIcon;

    #[test]
    fn equip_slot_index_covers_the_fifteen_c4_slots_and_leaves_h5_only_slots_open() {
        assert_eq!(equip_slot_index("EquipItem_Underwear"), Some(0));
        assert_eq!(equip_slot_index("EquipItem_Head"), Some(1));
        assert_eq!(equip_slot_index("EquipItem_Cloak"), Some(2));
        assert_eq!(equip_slot_index("EquipItem_Hair"), Some(3));
        assert_eq!(equip_slot_index("EquipItem_Neck"), Some(4));
        assert_eq!(equip_slot_index("EquipItem_RHand"), Some(5));
        assert_eq!(equip_slot_index("EquipItem_Chest"), Some(6));
        assert_eq!(equip_slot_index("EquipItem_LHand"), Some(7));
        assert_eq!(equip_slot_index("EquipItem_REar"), Some(8));
        assert_eq!(equip_slot_index("EquipItem_LEar"), Some(9));
        assert_eq!(equip_slot_index("EquipItem_Gloves"), Some(10));
        assert_eq!(equip_slot_index("EquipItem_Legs"), Some(11));
        assert_eq!(equip_slot_index("EquipItem_Feet"), Some(12));
        assert_eq!(equip_slot_index("EquipItem_RFinger"), Some(13));
        assert_eq!(equip_slot_index("EquipItem_LFinger"), Some(14));
        assert_eq!(equip_slot_index("EquipItem_Waist"), None);
        assert_eq!(equip_slot_index("EquipItem_Hair2"), None);
        assert_eq!(equip_slot_index("EquipItem_LBracelet"), None);
        assert_eq!(equip_slot_index("EquipItem_Talisman1"), None);
    }

    #[test]
    fn item_cell_rect_lays_out_the_six_by_eight_grid_with_the_32_plus_4_pitch_from_the_real_xdat() {
        let origin = Rect::new(210, 58, 231, 288);
        assert_eq!(item_cell_rect(origin, 0), Rect::new(210, 58, INVENTORY_SLOT, INVENTORY_SLOT));
        assert_eq!(item_cell_rect(origin, 5), Rect::new(210 + 5 * INVENTORY_PITCH, 58, INVENTORY_SLOT, INVENTORY_SLOT));
        assert_eq!(item_cell_rect(origin, 6), Rect::new(210, 58 + INVENTORY_PITCH, INVENTORY_SLOT, INVENTORY_SLOT));
    }

    #[test]
    fn inventory_scroll_clamps_to_the_last_full_page_of_six_columns_and_eight_rows() {
        assert_eq!(inventory_scroll_clamped(0, 10), 0);
        assert_eq!(inventory_scroll_clamped(5, 48), 0);
        assert_eq!(inventory_scroll_clamped(5, 60), 2);
        assert_eq!(inventory_scroll_clamped(-3, 60), 0);
    }

    #[test]
    fn abnormal_entries_order_buffs_before_debuffs_before_etc_icons_before_the_short_buff() {
        let view = AbnormalView {
            buffs: vec![
                BuffIcon { icon: Some("d".into()), debuff: true, ..Default::default() },
                BuffIcon { icon: Some("b".into()), debuff: false, ..Default::default() },
            ],
            short_buff: Some(BuffIcon { icon: Some("s".into()), ..Default::default() }),
            etc: vec![Some("e".into())],
            etc_skills: Vec::new(),
        };
        let order: Vec<&str> = abnormal_entries(&view).iter().map(|entry| entry.icon.unwrap()).collect();
        assert_eq!(order, ["b", "d", "e", "s"]);
    }

    fn glyph_font() -> BitmapFont {
        BitmapFont::new("L2Font-e.SmallFont-e".into(), killer_ui::font::Glyphs::parse(include_bytes!("../../../crates/killer-ui/tests/fixtures/smallfont-e.gly")).unwrap())
    }

    fn open_real_client() -> Option<(killer_assets::client::Client, Interface)> {
        let root = std::env::var_os("KILLER_CLIENT_DIR").map(std::path::PathBuf::from)?;
        let mut client = killer_assets::client::Client::open(&root).ok()?;
        let body = client.read_system_file("interface.xdat").ok()?;
        let interface = l2_dat::xdat::read_interface(&body).ok()?;
        Some((client, interface))
    }

    #[test]
    #[ignore]
    fn status_and_target_backgrounds_resolve_from_the_real_h5_catalog() {
        let (mut client, interface) = open_real_client().expect("KILLER_CLIENT_DIR");
        let status_back = interface.window(STATUS).unwrap().part.back_tex.clone();
        let target_back = interface.window(TARGET).unwrap().part.back_tex.clone();
        let mut hud = H5Hud::new(interface);
        {
            let catalog = client.catalog();
            hud.prime(&mut |name| catalog.texture_size(name).map(|size| (size[0] as i32, size[1] as i32)));
        }
        let mut world = World { me: Some(crate::ui2d::tests::me()), ..World::default() };
        let mut target = crate::ui2d::tests::me();
        target.object_id = 2;
        world.target = Some(target.object_id);
        world.entities.insert(target.object_id, target);
        let font = glyph_font();
        let sysstring = |_: u32| None;
        let view = HudView { world: &world, tables: None, chat: &[], size: (1920, 1080), font: &font, sysstring: &sysstring, inventory: None, abnormal: None };
        let list = hud.paint(&view);
        let names: Vec<&str> = list.quads.iter().map(|q| q.texture.as_str()).collect();
        assert!(names.iter().any(|name| name.eq_ignore_ascii_case(&status_back)), "fundo do StatusWnd ({status_back}) não desenhado: {names:?}");
        assert!(names.iter().any(|name| name.eq_ignore_ascii_case(&target_back)), "fundo do TargetStatusWnd ({target_back}) não desenhado: {names:?}");
    }

    #[test]
    #[ignore]
    fn status_bars_fill_proportionally_to_hp_mp_cp_from_the_real_h5_catalog() {
        let (mut client, interface) = open_real_client().expect("KILLER_CLIENT_DIR");
        let gauge = |bar: &str| {
            let control = interface.window(STATUS).unwrap().children.iter().find(|control| control.head.name == bar).unwrap();
            match &control.props {
                Props::StatusBar(gauge) => gauge.textures.clone(),
                _ => panic!("{bar} sem GaugeProps"),
            }
        };
        let hp = gauge("HPBar");
        let mp = gauge("MPBar");
        let cp = gauge("CPBar");
        let mut hud = H5Hud::new(interface);
        {
            let catalog = client.catalog();
            hud.prime(&mut |name| catalog.texture_size(name).map(|size| (size[0] as i32, size[1] as i32)));
        }
        let world = World { me: Some(crate::ui2d::tests::me()), ..World::default() };
        let font = glyph_font();
        let sysstring = |_: u32| None;
        let view = HudView { world: &world, tables: None, chat: &[], size: (1280, 956), font: &font, sysstring: &sysstring, inventory: None, abnormal: None };
        let list = hud.paint(&view);
        let fill_width = |textures: &[String; 13]| -> f32 {
            list.quads
                .iter()
                .filter(|q| [0usize, 1, 2].iter().any(|&i| q.texture.eq_ignore_ascii_case(&textures[i])))
                .map(|q| q.w)
                .sum()
        };
        let bg_width = |textures: &[String; 13]| -> f32 {
            list.quads
                .iter()
                .filter(|q| [3usize, 4, 5].iter().any(|&i| q.texture.eq_ignore_ascii_case(&textures[i])))
                .map(|q| q.w)
                .sum()
        };
        let ratio = |textures: &[String; 13]| -> f32 { fill_width(textures) / bg_width(textures) };
        let hp_ratio = ratio(&hp);
        let mp_ratio = ratio(&mp);
        let cp_ratio = ratio(&cp);
        assert!((hp_ratio - 0.5).abs() < 0.08, "HPBar não preenche ~50% (450/900): preenchido {} de fundo {} = {hp_ratio}", fill_width(&hp), bg_width(&hp));
        assert!((mp_ratio - (200.0 / 300.0)).abs() < 0.08, "MPBar não preenche ~66% (200/300): preenchido {} de fundo {} = {mp_ratio}", fill_width(&mp), bg_width(&mp));
        assert!((cp_ratio - 1.0).abs() < 0.08, "CPBar não preenche 100% (100/100): preenchido {} de fundo {} = {cp_ratio}", fill_width(&cp), bg_width(&cp));
    }

    #[test]
    #[ignore]
    fn status_wnd_draws_hp_mp_cp_numbers_with_the_real_h5_statusnumber_strip() {
        let (mut client, interface) = open_real_client().expect("KILLER_CLIENT_DIR");
        let mut hud = H5Hud::new(interface);
        {
            let catalog = client.catalog();
            hud.prime(&mut |name| catalog.texture_size(name).map(|size| (size[0] as i32, size[1] as i32)));
        }
        let world = World { me: Some(crate::ui2d::tests::me()), ..World::default() };
        let font = glyph_font();
        let sysstring = |_: u32| None;
        let view = HudView { world: &world, tables: None, chat: &[], size: (1280, 956), font: &font, sysstring: &sysstring, inventory: None, abnormal: None };
        let list = hud.paint(&view);
        let digit_quads: Vec<_> = list.quads.iter().filter(|q| q.texture.eq_ignore_ascii_case(killer_ui::draw::H5_STATUS_NUMBER_STRIP.texture)).collect();
        assert_eq!(digit_quads.len(), 27, "esperava 27 dígitos (CP 100/100, HP 450/900, MP 200/300): {digit_quads:?}");
        let cell = killer_ui::draw::DIGIT_CELLS.find('9').unwrap() as f32 * killer_ui::draw::H5_STATUS_NUMBER_STRIP.cell_width as f32;
        assert!(digit_quads.iter().any(|q| q.u == cell), "dígito '9' (de 900) não encontrado entre os quads: {digit_quads:?}");
    }

    #[test]
    #[ignore]
    fn inventory_wnd_draws_the_backpack_grid_and_the_equipped_weapon_from_the_real_h5_catalog() {
        let (mut client, interface) = open_real_client().expect("KILLER_CLIENT_DIR");
        let item_window = interface.window(INVENTORY).unwrap().children.iter().find(|control| control.head.name == "InventoryItem").unwrap();
        let (slot_back, item_frame) = match &item_window.props {
            Props::ItemWindow(props) => (props.slot.clone(), props.frame.clone()),
            _ => panic!("InventoryItem sem ItemWindowProps"),
        };
        let equip_frame = interface
            .window(INVENTORY)
            .unwrap()
            .children
            .iter()
            .find(|control| control.head.name == "EquipItem_RHand")
            .and_then(|control| match &control.props {
                Props::ItemWindow(props) => Some(props.frame.clone()),
                _ => None,
            })
            .unwrap();
        let mut hud = H5Hud::new(interface);
        {
            let catalog = client.catalog();
            hud.prime(&mut |name| catalog.texture_size(name).map(|size| (size[0] as i32, size[1] as i32)));
        }
        hud.toggle(INVENTORY);
        let world = World::default();
        let mut inventory = InventoryView::with_sysstrings(|_| None);
        inventory.items = vec![
            InventoryItemView { object_id: 1, item_id: 57, count: 2500, icons: vec!["icon.etc_adena_i00".into()], ..Default::default() },
            InventoryItemView {
                object_id: 2,
                item_id: 2369,
                count: 1,
                equipped: true,
                body_part: killer_ui::inventory::BODY_PART_R_HAND,
                icons: vec!["icon.weapon_squires_sword_i00".into()],
                ..Default::default()
            },
        ];
        inventory.adena = 2500;
        let font = glyph_font();
        let sysstring = |_: u32| None;
        let view = HudView { world: &world, tables: None, chat: &[], size: (1280, 956), font: &font, sysstring: &sysstring, inventory: Some(&inventory), abnormal: None };
        let list = hud.paint(&view);
        let names: Vec<&str> = list.quads.iter().map(|q| q.texture.as_str()).collect();
        assert!(names.contains(&"icon.etc_adena_i00"), "ícone do adena na grade não desenhado: {names:?}");
        assert!(names.contains(&"icon.weapon_squires_sword_i00"), "ícone da arma equipada não desenhado: {names:?}");
        let empty_slots = names.iter().filter(|name| name.eq_ignore_ascii_case(&slot_back)).count();
        assert_eq!(empty_slots, INVENTORY_VISIBLE as usize, "fundo de slot vazio ({slot_back}) não desenhado para as 48 células da grade: {names:?}");
        assert!(
            names.iter().any(|name| name.eq_ignore_ascii_case(&item_frame)),
            "moldura do item na grade ({item_frame}) não desenhada: {names:?}"
        );
        assert!(
            names.iter().any(|name| name.eq_ignore_ascii_case(&equip_frame)),
            "moldura do item equipado ({equip_frame}) não desenhada: {names:?}"
        );
    }

    #[test]
    #[ignore]
    fn abnormal_status_wnd_grows_its_window_with_the_buff_count_from_the_real_h5_catalog() {
        let (mut client, interface) = open_real_client().expect("KILLER_CLIENT_DIR");
        let mut hud = H5Hud::new(interface);
        {
            let catalog = client.catalog();
            hud.prime(&mut |name| catalog.texture_size(name).map(|size| (size[0] as i32, size[1] as i32)));
        }
        let world = World::default();
        let font = glyph_font();
        let sysstring = |_: u32| None;
        let abnormal = AbnormalView {
            buffs: (0..14).map(|i| BuffIcon { icon: Some(format!("Icon.skill{i}")), ..Default::default() }).collect(),
            ..Default::default()
        };
        let view = HudView { world: &world, tables: None, chat: &[], size: (1280, 956), font: &font, sysstring: &sysstring, inventory: None, abnormal: Some(&abnormal) };
        let list = hud.paint(&view);
        assert_eq!(list.quads.iter().filter(|q| q.texture.starts_with("Icon.skill")).count(), 14);
    }

    #[test]
    #[ignore]
    fn party_wnd_draws_member_rows_and_hides_the_rest_from_the_real_h5_catalog() {
        let (mut client, interface) = open_real_client().expect("KILLER_CLIENT_DIR");
        let mut hud = H5Hud::new(interface);
        {
            let catalog = client.catalog();
            hud.prime(&mut |name| catalog.texture_size(name).map(|size| (size[0] as i32, size[1] as i32)));
        }
        let mut world = World::default();
        world.party.leader_id = 10;
        world.party.members = vec![
            l2_net::game::PartyMember { object_id: 10, name: "Lider".into(), cp: 50, max_cp: 100, hp: 300, max_hp: 600, mp: 40, max_mp: 90, level: 40, class_id: 0 },
            l2_net::game::PartyMember { object_id: 11, name: "Membro".into(), cp: 0, max_cp: 0, hp: 200, max_hp: 400, mp: 60, max_mp: 90, level: 38, class_id: 8 },
        ];
        let font = glyph_font();
        let sysstring = |_: u32| None;
        let view = HudView { world: &world, tables: None, chat: &[], size: (1280, 956), font: &font, sysstring: &sysstring, inventory: None, abnormal: None };
        let list = hud.paint(&view);
        let names: Vec<&str> = list.quads.iter().map(|q| q.texture.as_str()).collect();
        assert!(names.contains(&killer_ui::party::LEADER_ICON), "coroa do líder não desenhada: {names:?}");
    }

    #[test]
    fn vitality_levels_follow_the_script_thresholds() {
        assert_eq!(vitality_level(0), (1, 0.0));
        assert_eq!(vitality_level(240).0, 2);
        assert_eq!(vitality_level(2000).0, 3);
        assert_eq!(vitality_level(13000).0, 4);
        assert_eq!(vitality_level(17000).0, 5);
        assert_eq!(vitality_level(20001), (1, 0.0));
        assert_eq!(vitality_level(120), (1, 0.5));
        assert_eq!(vitality_level(18500), (5, 0.5));
    }

    #[test]
    fn level_colors_switch_the_table_at_level_78() {
        assert_eq!(level_color(-9, 70), rgb(255, 0, 0));
        assert_eq!(level_color(-8, 70), rgb(255, 145, 145));
        assert_eq!(level_color(-8, 80), rgb(255, 0, 0));
        assert_eq!(level_color(0, 70), rgb(255, 255, 255));
        assert_eq!(level_color(9, 70), rgb(0, 0, 255));
        assert_eq!(level_color(6, 80), rgb(0, 0, 255));
    }

    #[test]
    fn chat_filters_follow_the_tab_defaults() {
        let tell = ChatLine::new(2, "a", "b");
        let trade = ChatLine::new(8, "a", "b");
        let normal = ChatLine::new(0, "a", "b");
        assert!(chat_passes(0, &normal) && chat_passes(0, &tell) && !chat_passes(0, &trade));
        assert!(chat_passes(1, &trade) && !chat_passes(1, &normal));
        assert!(chat_passes(2, &tell));
        assert!(!chat_passes(9, &normal));
    }
}
