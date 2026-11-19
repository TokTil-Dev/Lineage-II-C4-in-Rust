use killer_assets::client::Client;
use killer_assets::textures::TextureBank;
use killer_ui::chat::ChatEntry;
use killer_ui::color::Argb;
use killer_ui::detail_status::DetailStatusView;
use killer_ui::draw::DrawList;
use killer_ui::font::BitmapFont;
use killer_ui::inventory::{InventoryView, body_part_to_slot, thousands};
use killer_ui::party::PartyView;
use killer_ui::shortcut::{SLOTS, ShortcutView};
use killer_ui::status::StatusView;
use killer_ui::target::TargetView;
use killer_ui::window::{Rect, WindowId};
use killer_ui::xdat_paint_html::{
    self, FrameLayout, NPC_FRAME_HEIGHT, NPC_TITLE_SYSSTRING, NPC_VIEWER_SIZE, NPC_WND_BACK, align_width,
    content_height, from_document, layout_frame, npc_content_rect, npc_viewer_position,
};
use killer_ui::xdat_paint_tabs::{SCROLL_DOWN_BTN, SCROLL_THUMB_BOTTOM, SCROLL_THUMB_CENTER, SCROLL_THUMB_TOP, SCROLL_UP_BTN};
use killer_ui::xdat_view::{
    self, BarPoint, BuffCell, Cell, Data, FRAME_BACK_LEFT, FRAME_BACK_MID, FRAME_BACK_RIGHT, FRAME_CLOSE_BTN, Layout,
    MinimapState, State, TOWN_MAP_GROUP, TownMap,
};
use killer_world::World;
use l2_xdat::{Kind, TextAlign, TextureLayer, Window, Xdat};
use std::collections::HashMap;

pub const XDAT_FILE: &str = "interface.xdat";
pub const SCRIPT_FILE: &str = "interface.u";
pub const OPTION_FILE: &str = "Option.ini";
pub const BUFF_BACK: &str = "L2UI.EtcWndBack.AbnormalBack";
pub const DEBUFF_BACK: &str = "L2UI_CH3.Debuff";
pub const BUFFS_PER_ROW: usize = 12;
pub const BUFF_ICONS: &str = "StatusIcon";
pub const SILENT_SONG_DANCE: [i32; 30] = [
    276, 273, 365, 275, 274, 271, 272, 310, 264, 265, 266, 267, 268, 269, 349, 270, 304, 305, 306, 308, 363, 364, 529, 277,
    307, 309, 311, 366, 530, 915,
];
pub const SILENT_DEBUFFS: [i32; 57] = [
    28, 92, 101, 102, 105, 115, 129, 1069, 1083, 1160, 1164, 1167, 1168, 1184, 1201, 1206, 1222, 1223, 1224, 100, 95, 96, 120,
    223, 1092, 1095, 1096, 1097, 1099, 1100, 1101, 1102, 1107, 1208, 1209, 18, 48, 65, 84, 97, 98, 103, 106, 107, 116, 122, 127,
    260, 279, 281, 1042, 1049, 1064, 1071, 1072, 1074, 1104,
];

pub const SYSSTRING_FILE: &str = "sysstring-e.dat";
pub const NATIVE_HTML: [WindowId; 2] = [WindowId::NPC_HTML, WindowId::ITEM_HTML];
const NATIVE_TEXTURES: [&str; 10] = [
    NPC_WND_BACK,
    FRAME_BACK_LEFT,
    FRAME_BACK_MID,
    FRAME_BACK_RIGHT,
    FRAME_CLOSE_BTN,
    SCROLL_UP_BTN,
    SCROLL_DOWN_BTN,
    SCROLL_THUMB_TOP,
    SCROLL_THUMB_CENTER,
    SCROLL_THUMB_BOTTOM,
];

fn own_sysstrings(client: &Client) -> HashMap<u32, String> {
    let path = client.root().join("system").join(SYSSTRING_FILE);
    let Ok(raw) = std::fs::read(&path) else { return HashMap::new() };
    let Ok(body) = l2_crypto::decrypt(&raw, &path) else { return HashMap::new() };
    l2_dat::sysstring::read(&body).map(|rows| rows.into_iter().map(|row| (row.id, row.text)).collect()).unwrap_or_default()
}

pub const ZONE_FILE: &str = "zonename-e.dat";

pub const FONT_PACKAGE: &str = "L2Font";
pub const FONT_PACKAGE_LOCALIZED: &str = "L2Font-e";

fn font_package(client: &mut Client, bank: &mut TextureBank, probe: &str) -> &'static str {
    if client.catalog().package_status(FONT_PACKAGE_LOCALIZED) == l2_catalog::PackageStatus::Missing {
        return FONT_PACKAGE;
    }
    let name = format!("{FONT_PACKAGE_LOCALIZED}.{probe}");
    bank.texture(client.catalog(), &name);
    if bank.texture_available(&name) == Some(true) { FONT_PACKAGE_LOCALIZED } else { FONT_PACKAGE }
}

fn own_town_maps(client: &Client, font_package: &str) -> HashMap<u32, TownMap> {
    let path = client.root().join("system").join(ZONE_FILE);
    let Ok(raw) = std::fs::read(&path) else { return HashMap::new() };
    let Ok(body) = l2_crypto::decrypt(&raw, &path) else { return HashMap::new() };
    l2_dat::zonename::read_interlude(&body)
        .map(|zones| {
            zones
                .into_iter()
                .filter(|(_, info)| !info.texture.is_empty())
                .map(|(zone, info)| {
                    (
                        zone.id,
                        TownMap {
                            texture: format!("{font_package}.{TOWN_MAP_GROUP}.{}", info.texture),
                            origin: (info.values[2], info.values[3]),
                            size: (info.values[4], info.values[5]),
                            scale: info.scale,
                        },
                    )
                })
                .collect()
        })
        .unwrap_or_default()
}

pub fn buff_size(script: Script, option: i32) -> i32 {
    match (script, option) {
        (Script::Stock, 0) => 24,
        (Script::Stock, 1) => 20,
        (Script::Stock, 2) => 16,
        (Script::Stock, _) => 0,
        (Script::Silent, 0) => 28,
        (Script::Silent, 1) => 26,
        (Script::Silent, 2) => 24,
        (Script::Silent, 3) => 22,
        (Script::Silent, 4) => 20,
        (Script::Silent, 5) => 18,
        (Script::Silent, 6) => 16,
        (Script::Silent, _) => 0,
    }
}
pub const NAME_DEFAULT_COLOR: Argb = Argb(0xFFDCDCDC);
pub const CLOSE_BUTTON: &str = "BtnClose";
pub const TITLE: &str = "Title";
pub const ITEM_COUNT: &str = "ItemCount";
pub const ADENA_TEXT: &str = "AdenaText";
pub const QUEST_ITEMS: &str = "QuestItem";
pub const ITEM_GRIDS: [&str; 5] = ["InventoryItem", "EquipmentItem", "SuppliesItem", "CraftingItem", QUEST_ITEMS];
pub const PAPERDOLL: [(&str, i32); 14] = [
    ("EquipItem_Underwear", 0),
    ("EquipItem_Head", 1),
    ("EquipItem_Hair", 3),
    ("EquipItem_Neck", 4),
    ("EquipItem_RHand", 5),
    ("EquipItem_Chest", 6),
    ("EquipItem_LHand", 7),
    ("EquipItem_REar", 8),
    ("EquipItem_LEar", 9),
    ("EquipItem_Gloves", 10),
    ("EquipItem_Legs", 11),
    ("EquipItem_Feet", 12),
    ("EquipItem_RFinger", 13),
    ("EquipItem_LFinger", 14),
];

pub const WINDOWS: [(WindowId, &str); 11] = [
    (WindowId::ABNORMAL_STATUS, "AbnormalStatusWnd"),
    (WindowId::RADAR, "NPHRN_MiniMapWnd_NEW"),
    (WindowId::RADAR, "RadarContainerWnd"),
    (WindowId::INVENTORY, "InventoryWnd"),
    (WindowId::STATUS, "StatusWnd"),
    (WindowId::TARGET, "TargetStatusWnd"),
    (WindowId::PARTY_STATUS, "PartyWnd"),
    (WindowId::MENU, "MenuWnd"),
    (WindowId::MAIN, "MainWnd"),
    (WindowId::CHAT, "ChatWnd"),
    (WindowId::SYSTEM_MSG, "SystemMsgWnd"),
];

pub const NESTED: [(WindowId, &str, &str, &str); 1] =
    [(WindowId::SHORTCUT, "ShortcutWnd", "ShortcutWndHorizontal", "ShortcutWndVertical")];
pub const SHORTCUT_SLOT: &str = "Shortcut";
pub const PAGE_TEXT: &str = "PageNumTextBox";
pub const CHAT_INPUT: &str = "ChatEditBox";
pub const CHAT_TABS: &str = "ChatTabCtrl";
pub const CHAT_WINDOWS: [&str; 5] = ["NormalChat", "TradeChat", "PartyChat", "ClanChat", "AllyChat"];
pub const SYSTEM_MSG_LIST: &str = "SystemMsgList";
pub const CHAT_SCROLL_BAR_STOCK: (i32, i32, i32) = (5, 10, -25);
pub const CHAT_SCROLL_BAR_SILENT: (i32, i32, i32) = (5, 10, -30);
pub const CHAT_TRANSPARENT_ALPHA: [i32; 5] = [255, 200, 150, 80, 0];
pub const CHAT_TRANSPARENT_CTRLS: [&str; 7] = [
    "ChatWnd.ChatWndHeadTex",
    "ChatWnd.ChatWndBodyTex",
    "ChatWnd.ChatWndBottomTex",
    "ChatWnd.ChatWndBottomTex1",
    "SystemMsgWnd.BackTexture",
    "SystemMsgWnd.ChatWndHeadTex",
    "SystemMsgWnd.ChatWndBottomTex2",
];
pub const MAIN_TABS: &str = "MainTabCtrl";
pub const MAIN_TARGETS: [&str; 5] = ["DetailStatusWnd", "MagicSkillWnd", "ActionWnd", "ClanWnd", "QuestTreeWnd"];
pub const PARTY_MEMBER: &str = "PartyStatusWnd";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Close,
    Tab(String, usize),
    Cell(String, usize),
    Button(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Script {
    Stock,
    Silent,
}

impl Script {
    pub fn detect(client: &Client) -> Script {
        let path = client.root().join("system").join(SCRIPT_FILE);
        match std::fs::read(&path).ok().and_then(|raw| l2_crypto::detect(&raw).ok()) {
            Some(l2_crypto::Scheme::FfEdFaCe) => Script::Silent,
            _ => Script::Stock,
        }
    }
}

pub struct GenericWindow {
    pub name: String,
    pub layout: Layout,
    pub rect: Rect,
    pub grip: Rect,
    pub placed: bool,
}

pub struct InterludeUi {
    script: Script,
    pub generic: Vec<GenericWindow>,
    pub generic_drag: Option<(usize, i32, i32)>,
    buff_size: i32,
    sysstrings: HashMap<u32, String>,
    town_maps: HashMap<u32, TownMap>,
    pub town_zoom: bool,
    song_dance_row: bool,
    system_msg_shown: bool,
    chat_alpha: Option<i32>,
    font_package: &'static str,
    positions: Vec<(WindowId, (i32, i32))>,
    xdat: Xdat,
    nested: Vec<(WindowId, Window)>,
    layouts: Vec<(WindowId, Layout)>,
    extra: Vec<(String, Layout)>,
    missing: Vec<String>,
    pub saved: Vec<(WindowId, Rect, Rect, bool)>,
    pub state: State,
    pub enabled: bool,
    pub runtime: crate::interlude_script::runtime::Runtime,
}

pub fn textures_of(window: &Window) -> Vec<String> {
    let mut out = Vec::new();
    let mut push = |name: &str| {
        if !name.is_empty() && !name.eq_ignore_ascii_case("undefined") && !out.iter().any(|known| known == name) {
            out.push(name.to_string());
        }
    };
    for node in window.nodes() {
        match &node.kind {
            Kind::Texture(texture) => push(&texture.file),
            Kind::Button(button) => {
                push(&button.normal_tex);
                push(&button.pushed_tex);
                push(&button.highlight_tex);
            }
            Kind::EffectButton(button) => {
                push(&button.normal_tex);
                push(&button.pushed_tex);
                push(&button.highlight_tex);
            }
            Kind::Tab(tab) => {
                for element in &tab.tabs {
                    push(&element.normal_tex);
                    push(&element.pushed_tex);
                }
            }
            Kind::StatusBar(bar) => {
                push(&bar.fore_texture);
                push(&bar.back_texture);
            }
            Kind::BarCtrl(bar) => {
                push(&bar.fore_texture);
                push(&bar.back_texture);
            }
            Kind::Progress(progress) => {
                push(&progress.back_texture);
                push(&progress.bar_texture);
            }
            Kind::CheckBox(check) => {
                push(&check.check_texture);
                push(&check.uncheck_texture);
            }
            _ => {}
        }
    }
    out
}

pub fn register_system_packages(client: &mut Client, textures: &[String]) {
    let mut packages: Vec<&str> = textures.iter().filter_map(|name| name.split('.').next()).collect();
    packages.sort_unstable();
    packages.dedup();
    for package in packages {
        if client.catalog().package_status(package) != l2_catalog::PackageStatus::Missing {
            continue;
        }
        let path = client.root().join("system").join(format!("{package}.u"));
        if path.is_file() {
            log::info!("interface.xdat: pacote {package} registrado de system/{package}.u");
            client.catalog().register_package(package, &path);
        }
    }
}

fn grip_of(window: &Window, layout: &Layout) -> Rect {
    if let Some(frame) = xdat_view::frame_grip(window, layout.size) {
        return frame;
    }
    let body = layout.placed.iter().find(|placed| {
        placed.rect.x == 0
            && placed.rect.y > 0
            && matches!(xdat_view::node_at(window, &placed.path).map(|node| &node.kind), Some(Kind::Texture(texture)) if texture.layer == TextureLayer::Background)
    });
    Rect::new(0, 0, layout.size.0, body.map_or(killer_ui::frame::TITLE_HEIGHT, |placed| placed.rect.y))
}

fn entry<'a>(
    xdat: &'a Xdat,
    nested: &'a [(WindowId, Window)],
    layouts: &'a [(WindowId, Layout)],
    id: WindowId,
) -> Option<(&'a Window, &'a Layout)> {
    let (_, layout) = layouts.iter().find(|(known, _)| *known == id)?;
    let window = nested
        .iter()
        .find(|(known, _)| *known == id)
        .map(|(_, window)| window)
        .or_else(|| xdat.window(&layout.window))?;
    Some((window, layout))
}

impl InterludeUi {
    pub fn load(
        client: &mut Client,
        bank: &mut TextureBank,
        vertical_shortcut: bool,
        info: &killer_ui::ini::WindowsInfo,
    ) -> Result<InterludeUi, String> {
        let body = client.read_system_file(XDAT_FILE).map_err(|error| error.to_string())?;
        let mut xdat = l2_xdat::read(&body).map_err(|error| error.to_string())?;
        for window in &mut xdat.windows {
            xdat_view::resolve_extends(window);
        }
        let script = Script::detect(client);
        log::info!("interface.u: script {script:?}");
        let option = client.read_system_text(OPTION_FILE).map(|text| killer_ui::ini::parse_ini(&text)).ok();
        let buff_size = buff_size(script, option.as_ref().and_then(|ini| ini.int("Custom", "AbnormalSize")).unwrap_or(0));
        let song_dance_row = option.as_ref().and_then(|ini| ini.bool("Custom", "checkAbSongDance")).unwrap_or(false);
        let system_msg_shown = option.as_ref().and_then(|ini| ini.bool("Game", "SystemMsgWnd")).unwrap_or(false);
        let chat_alpha = match script {
            Script::Silent => option
                .as_ref()
                .and_then(|ini| ini.int("Custom", "ChatTransparent"))
                .and_then(|level| usize::try_from(level).ok())
                .and_then(|level| CHAT_TRANSPARENT_ALPHA.get(level).copied()),
            Script::Stock => None,
        };
        let sysstrings = own_sysstrings(client);
        let mut runtime = match crate::interlude_script::runtime::Runtime::load_packages(&client.root().join("system")) {
            Ok(vm) => {
                let scripted: Vec<(String, String)> = xdat
                    .windows
                    .iter()
                    .filter(|window| !window.script.is_empty() && !window.script.eq_ignore_ascii_case("undefined"))
                    .map(|window| (window.base.name.clone(), window.script.clone()))
                    .collect();
                let runtime = crate::interlude_script::runtime::Runtime::from_vm(vm, &scripted);
                log::info!("interface.u: {} scripts instanciados pelo interpretador ({} erros)", scripted.len() - runtime.errors.len(), runtime.errors.len());
                for error in &runtime.errors {
                    log::warn!("interface.u: {error}");
                }
                runtime
            }
            Err(error) => {
                log::warn!("interface.u: interpretador indisponível ({error}); usando os scripts portados à mão");
                crate::interlude_script::runtime::Runtime::standard()
            }
        };
        runtime.host.sysstrings = sysstrings.clone();
        runtime.host.load_data(client);
        runtime.host.options = client.read_system_text(OPTION_FILE).map(|text| killer_ui::ini::parse_ini(&text)).ok();
        for (_, name) in WINDOWS {
            if let Some(window) = xdat.window(name) {
                runtime.host.initial_shown.insert(name.to_string(), window.hidden == 0);
            }
        }
        for window in &xdat.windows {
            for node in window.nodes() {
                if let l2_xdat::Kind::ListCtrl(ctrl) = &node.kind {
                    let spec = killer_ui::xdat_paint_lists::ListCtrlSpec::from_xdat(ctrl);
                    runtime
                        .host
                        .controls
                        .list_limits
                        .insert(format!("{}.{}", window.base.name, node.base.name), (spec.max_row, spec.show_row));
                }
            }
        }
        let world_map_package = font_package(client, bank, &format!("{}1", xdat_view::WORLD_MAP_PREFIX));
        let town_probe = own_town_maps(client, FONT_PACKAGE).values().next().map(|town| town.texture.trim_start_matches("L2Font.").to_string());
        let town_package = town_probe.map_or(FONT_PACKAGE, |probe| font_package(client, bank, &probe));
        let town_maps = own_town_maps(client, town_package);
        log::info!("interface: mapa-múndi em {world_map_package}, mapas de cidade em {town_package}");
        let font_package = world_map_package;
        log::info!("interface: {} sysstrings do próprio cliente", sysstrings.len());
        let mut positions = Vec::new();
        let mut layouts = Vec::new();
        let mut missing = Vec::new();
        let nested: Vec<(WindowId, &Window)> = NESTED
            .iter()
            .filter_map(|(id, top, horizontal, vertical)| {
                let name = if vertical_shortcut { vertical } else { horizontal };
                let node = xdat.window(top)?.children.iter().find(|node| node.base.name == *name)?;
                match &node.kind {
                    Kind::Window(window) => Some((*id, window.as_ref())),
                    _ => None,
                }
            })
            .collect();
        let windows = WINDOWS
            .iter()
            .map(|(id, name)| (*id, xdat.window(name), *name))
            .chain(nested.iter().map(|(id, window)| (*id, Some(*window), window.base.name.as_str())));
        for (id, window, name) in windows {
            let Some(window) = window else {
                log::warn!("interface.xdat: sem a janela {name}");
                continue;
            };
            if window.base.size.as_ref().is_none_or(|size| size.width <= 0 || size.height <= 0) {
                log::info!("interface.xdat: {name} sem tamanho fixo; fica com a janela C4");
                continue;
            }
            let textures = textures_of(window);
            register_system_packages(client, &textures);
            for file in textures {
                bank.texture(client.catalog(), &file);
                if bank.texture_available(&file) == Some(false) {
                    log::warn!("interface.xdat: {name} usa a textura {file}, que não existe neste cliente");
                    missing.push(file);
                }
            }
            let saved = info.saved(name);
            let xdat_size = window.base.size.as_ref().map_or((0, 0), |size| (size.width, size.height));
            let layout = xdat_view::layout_sized(window, xdat_view::saved_size(window, xdat_size, saved.size));
            if let Some(position) = saved.position.filter(|_| window.save_position != 0) {
                positions.push((id, position));
            }
            if !layout.unsupported.is_empty() {
                log::info!(
                    "interface.xdat: {name} usa {:?}, ainda sem pintor; fica com a janela C4",
                    layout.unsupported
                );
                continue;
            }
            log::info!(
                "interface.xdat: {name} {}x{} com {} controles",
                layout.size.0,
                layout.size.1,
                layout.placed.len()
            );
            layouts.push((id, layout));
        }
        let native: Vec<String> = NATIVE_TEXTURES.iter().map(|name| name.to_string()).collect();
        register_system_packages(client, &native);
        for file in native {
            bank.texture(client.catalog(), &file);
            if bank.texture_available(&file) == Some(false) {
                log::warn!("janela do NPC: textura {file} não existe neste cliente");
                missing.push(file);
            }
        }
        let mut generic = Vec::new();
        let reserved: Vec<&str> = WINDOWS.iter().map(|(_, name)| *name).chain(MAIN_TARGETS).chain(NESTED.iter().map(|(_, top, _, _)| *top)).collect();
        for window in &xdat.windows {
            let name = window.base.name.as_str();
            if reserved.contains(&name) || window.base.size.as_ref().is_none_or(|size| size.width <= 0 || size.height <= 0) {
                continue;
            }
            let textures = textures_of(window);
            register_system_packages(client, &textures);
            for file in textures {
                bank.texture(client.catalog(), &file);
                if bank.texture_available(&file) == Some(false) {
                    missing.push(file);
                }
            }
            let saved = info.saved(name);
            let xdat_size = window.base.size.as_ref().map_or((0, 0), |size| (size.width, size.height));
            let layout = xdat_view::layout_sized(window, xdat_view::saved_size(window, xdat_size, saved.size));
            let (w, h) = layout.size;
            let rect = match saved.position.filter(|_| window.save_position != 0) {
                Some((x, y)) => Rect::new(x, y, w, h),
                None => Rect::new(-1, -1, w, h),
            };
            let placed = rect.x >= 0;
            let grip = grip_of(window, &layout);
            generic.push(GenericWindow { name: name.to_string(), layout, rect, grip, placed });
        }
        log::info!("interface.xdat: {} janelas genéricas prontas para os scripts", generic.len());
        let mut extra = Vec::new();
        for name in MAIN_TARGETS {
            let Some(window) = xdat.window(name) else { continue };
            let textures = textures_of(window);
            register_system_packages(client, &textures);
            for file in textures {
                bank.texture(client.catalog(), &file);
                if bank.texture_available(&file) == Some(false) {
                    missing.push(file);
                }
            }
            let layout = xdat_view::layout(window);
            log::info!(
                "interface.xdat: alvo de aba {name} {}x{} com {} controles{}",
                layout.size.0,
                layout.size.1,
                layout.placed.len(),
                if layout.unsupported.is_empty() {
                    String::new()
                } else {
                    format!(" (sem pintor para {:?})", layout.unsupported)
                }
            );
            extra.push((name.to_string(), layout));
        }
        let nested: Vec<(WindowId, Window)> = nested.into_iter().map(|(id, window)| (id, window.clone())).collect();
        Ok(InterludeUi {
            script,
            generic,
            generic_drag: None,
            buff_size,
            sysstrings,
            town_maps,
            town_zoom: true,
            song_dance_row,
            system_msg_shown,
            chat_alpha,
            font_package,
            positions,
            xdat,
            nested,
            layouts,
            extra,
            missing,
            saved: Vec::new(),
            state: State::default(),
            enabled: false,
            runtime,
        })
    }

    pub fn sync(&mut self, world: &World) {
        let host = &mut self.runtime.host;
        host.party_members = world.party.members.iter().map(|member| member.object_id).collect();
        host.party_locations = world
            .party
            .members
            .iter()
            .map(|member| {
                let position = world.party.positions.get(&member.object_id).copied().unwrap_or([0, 0, 0]);
                crate::interlude_script::Vector { x: position[0] as f32, y: position[1] as f32, z: position[2] as f32 }
            })
            .collect();
        for member in &world.party.members {
            host.users.insert(
                member.object_id,
                crate::interlude_script::UserInfo {
                    id: member.object_id,
                    name: member.name.clone(),
                    level: member.level,
                    class_id: member.class_id,
                    cur_hp: member.hp,
                    max_hp: member.max_hp,
                    cur_mp: member.mp,
                    max_mp: member.max_mp,
                    cur_cp: member.cp,
                    max_cp: member.max_cp,
                    ..Default::default()
                },
            );
        }
        host.target = world.target_entity().map(|entity| crate::interlude_script::UserInfo {
            id: entity.object_id,
            name: entity.name.clone(),
            cur_hp: entity.hp.map_or(0, |hp| hp.0),
            max_hp: entity.hp.map_or(0, |hp| hp.1),
            cur_mp: entity.mp.map_or(0, |mp| mp.0),
            max_mp: entity.mp.map_or(0, |mp| mp.1),
            clan_id: entity.clan_id,
            npc: !matches!(entity.kind, killer_world::EntityKind::Player { .. }),
            loc: crate::interlude_script::Vector { x: entity.position.x, y: entity.position.y, z: entity.position.z },
            ..Default::default()
        });
        for clan_id in world.target_entity().map(|entity| entity.clan_id).into_iter().chain(world.me.as_ref().map(|me| me.clan_id)) {
            if clan_id > 0
                && let Some(name) = world.pledge_name(clan_id)
            {
                let entry = host.clans.entry(clan_id).or_default();
                entry.name = name.to_string();
                entry.alliance_name = world.ally_name(clan_id).unwrap_or_default().to_string();
            }
        }
        host.player = world.me.as_ref().map(|me| {
            let status = crate::ui2d::status_view(me);
            crate::interlude_script::UserInfo {
                id: me.object_id,
                name: status.name.clone(),
                level: status.level,
                cur_hp: status.hp.0,
                max_hp: status.hp.1,
                cur_mp: status.mp.0,
                max_mp: status.mp.1,
                cur_cp: status.cp.0,
                max_cp: status.cp.1,
                cur_exp: status.exp,
                loc: crate::interlude_script::Vector { x: me.position.x, y: me.position.y, z: me.position.z },
                ..Default::default()
            }
        });
    }

    pub fn script_event(&mut self, event_id: i32, param: &str) -> Vec<crate::interlude_script::host_impl::HostEvent> {
        self.runtime.event(event_id, param);
        self.runtime.drain()
    }

    pub fn script_load(&mut self, visible: &[WindowId]) -> Vec<crate::interlude_script::host_impl::HostEvent> {
        self.runtime.load();
        let names: Vec<String> = WINDOWS
            .iter()
            .filter(|(id, _)| visible.contains(id))
            .map(|(_, name)| name.to_string())
            .collect();
        for name in names {
            self.runtime.show(&name);
        }
        self.runtime.drain()
    }

    pub fn html_window(id: WindowId) -> bool {
        NATIVE_HTML.contains(&id)
    }

    pub fn handles(&self, id: WindowId) -> bool {
        Self::html_window(id) || self.layouts.iter().any(|(known, _)| *known == id)
    }

    pub fn ids(&self) -> Vec<WindowId> {
        self.layouts.iter().map(|(id, _)| *id).chain(NATIVE_HTML).collect()
    }

    pub fn starts_hidden(&self, id: WindowId) -> Option<bool> {
        if Self::html_window(id) {
            return Some(true);
        }
        if id == WindowId::SYSTEM_MSG {
            return Some(!self.system_msg_shown);
        }
        entry(&self.xdat, &self.nested, &self.layouts, id).map(|(window, _)| window.hidden != 0)
    }

    pub fn size(&self, id: WindowId) -> Option<(i32, i32)> {
        if Self::html_window(id) {
            return Some(NPC_VIEWER_SIZE);
        }
        entry(&self.xdat, &self.nested, &self.layouts, id).map(|(_, layout)| layout.size)
    }

    pub fn position(&self, id: WindowId, viewport: (i32, i32), rect_of: &dyn Fn(WindowId) -> Option<Rect>) -> Option<(i32, i32)> {
        if let Some((_, position)) = self.positions.iter().find(|(known, _)| *known == id) {
            return Some(*position);
        }
        if Self::html_window(id) {
            return Some(npc_viewer_position(viewport.1));
        }
        let (window, layout) = entry(&self.xdat, &self.nested, &self.layouts, id)?;
        let screen = Rect::new(0, 0, viewport.0, viewport.1);
        let reference = window
            .base
            .anchor
            .as_ref()
            .filter(|anchor| !anchor.ctrl.is_empty())
            .and_then(|anchor| self.id_of(&anchor.ctrl))
            .and_then(rect_of)
            .unwrap_or(screen);
        Some(xdat_view::place(window.base.anchor.as_ref(), reference, layout.size))
    }

    pub fn id_of(&self, name: &str) -> Option<WindowId> {
        WINDOWS.iter().find(|(_, known)| *known == name).map(|(id, _)| *id)
    }

    fn has_texture(&self, file: &str) -> bool {
        !self.missing.iter().any(|known| known.eq_ignore_ascii_case(file))
    }

    pub fn html_title(&self, id: WindowId, fallback: &str) -> String {
        if id == WindowId::NPC_HTML
            && let Some(title) = self.sysstrings.get(&NPC_TITLE_SYSSTRING)
        {
            return title.clone();
        }
        fallback.to_string()
    }

    pub fn html_frame(body: &[killer_ui::html::Node], font: &BitmapFont) -> FrameLayout {
        let content = npc_content_rect(NPC_VIEWER_SIZE);
        layout_frame(&from_document(body), align_width(content.w), font)
    }

    pub fn html_scroll_limit(body: &[killer_ui::html::Node], font: &BitmapFont) -> i32 {
        let content = npc_content_rect(NPC_VIEWER_SIZE);
        (content_height(&Self::html_frame(body, font)) - content.h).max(0)
    }

    pub fn abnormal_size(&self, abnormal: &killer_ui::abnormal::AbnormalView) -> (i32, i32) {
        let rows = buff_rows(self.script, self.song_dance_row, abnormal);
        xdat_view::status_icon_size(&rows, self.buff_size, &|name| self.has_texture(name))
    }

    pub fn html_close_button() -> Rect {
        xdat_view::frame_close_button_rect(NPC_VIEWER_SIZE)
    }

    pub fn html_link_at(body: &[killer_ui::html::Node], scroll: i32, x: i32, y: i32, font: &BitmapFont) -> Option<String> {
        let content = npc_content_rect(NPC_VIEWER_SIZE);
        if !content.contains(x, y) {
            return None;
        }
        xdat_paint_html::link_at(&Self::html_frame(body, font), font, x - content.x, y - content.y + scroll).map(str::to_string)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn paint_html(
        &self,
        list: &mut DrawList,
        x: i32,
        y: i32,
        title: &str,
        body: &[killer_ui::html::Node],
        scroll: i32,
        font: &BitmapFont,
        alpha: f32,
    ) {
        let has_texture = |name: &str| self.has_texture(name);
        let rect = Rect::new(x, y, NPC_VIEWER_SIZE.0, NPC_VIEWER_SIZE.1);
        let start = list.quads.len();
        if has_texture(NPC_WND_BACK) {
            xdat_paint_html::paint_npc_back(list, rect, NPC_WND_BACK, alpha);
        }
        list.clip_from(start, rect);
        xdat_view::paint_frame_bar(list, &has_texture, Rect::new(x, y, rect.w, NPC_FRAME_HEIGHT), 0, false, Some(title), font, alpha);
        xdat_view::paint_frame_button(list, &has_texture, Self::html_close_button().offset(x, y), FRAME_CLOSE_BTN, alpha);
        let content = npc_content_rect(NPC_VIEWER_SIZE).offset(x, y);
        let frame = Self::html_frame(body, font);
        xdat_paint_html::paint_viewer(list, font, content, None, (0, 0), -scroll, &frame, alpha);
        let total = content_height(&frame);
        if total > content.h {
            xdat_view::paint_scroll_bar(list, &has_texture, content, total, content.h, scroll, alpha);
        }
    }

    pub fn grip(&self, id: WindowId) -> Option<Rect> {
        if Self::html_window(id) {
            return Some(Rect::new(0, 0, NPC_VIEWER_SIZE.0, NPC_FRAME_HEIGHT));
        }
        let (window, layout) = entry(&self.xdat, &self.nested, &self.layouts, id)?;
        Some(grip_of(window, layout))
    }

    pub fn name_of(&self, id: WindowId) -> Option<&'static str> {
        WINDOWS.iter().find(|(known, _)| *known == id).map(|(_, name)| *name)
    }

    pub fn click(&mut self, window: &str, button: &str) -> Vec<crate::interlude_script::host_impl::HostEvent> {
        self.runtime.click(window, button);
        self.runtime.drain()
    }

    pub fn place_generic(&mut self, viewport: (i32, i32)) {
        let screen = Rect::new(0, 0, viewport.0, viewport.1);
        for index in 0..self.generic.len() {
            if self.generic[index].placed {
                continue;
            }
            let Some(window) = self.xdat.window(&self.generic[index].name) else { continue };
            let size = self.generic[index].layout.size;
            let (x, y) = xdat_view::place(window.base.anchor.as_ref(), screen, size);
            let entry = &mut self.generic[index];
            entry.rect.x = x;
            entry.rect.y = y;
            entry.placed = true;
        }
    }

    pub fn generic_shown(&self, index: usize) -> bool {
        self.generic.get(index).is_some_and(|entry| entry.placed && self.runtime.host.overlay.shown.get(&entry.name).copied().unwrap_or(false))
    }

    pub fn generic_at(&self, x: i32, y: i32) -> Option<usize> {
        (0..self.generic.len()).rev().find(|index| self.generic_shown(*index) && self.generic[*index].rect.contains(x, y))
    }

    pub fn mouse_down_generic(&mut self, index: usize, x: i32, y: i32) {
        let Some(entry) = self.generic.get(index) else { return };
        let name = entry.name.clone();
        let (lx, ly) = (x - entry.rect.x, y - entry.rect.y);
        let Some(window) = self.xdat.window(&name) else { return };
        if xdat_view::frame_close_button(window, entry.layout.size).is_some_and(|button| button.contains(lx, ly)) {
            self.runtime.host.overlay.shown.insert(name.clone(), false);
            self.runtime.hide(&name);
            return;
        }
        let hit = entry.layout.hit(window, &self.state, lx, ly).cloned();
        if let Some(hit) = hit
            && let Some(node) = xdat_view::node_at(window, &hit.path)
        {
            match &node.kind {
                Kind::Button(_) | Kind::EffectButton(_) | Kind::CheckBox(_) => {
                    self.state.pressed = Some(hit.name.clone());
                    self.runtime.click(&name, &hit.name);
                    return;
                }
                Kind::Tab(tab) => {
                    if let Some(tab_index) = xdat_view::tab_at(tab, hit.rect, lx, ly) {
                        self.state.select_tab(&hit.name, tab_index);
                    }
                    return;
                }
                _ => {}
            }
        }
        if entry.grip.contains(lx, ly) {
            self.generic_drag = Some((index, lx, ly));
        }
    }

    pub fn drag_generic(&mut self, x: i32, y: i32) {
        if let Some((index, dx, dy)) = self.generic_drag
            && let Some(entry) = self.generic.get_mut(index)
        {
            entry.rect.x = x - dx;
            entry.rect.y = y - dy;
        }
    }

    pub fn end_drag_generic(&mut self) {
        self.generic_drag = None;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn paint_generic(
        &self,
        list: &mut DrawList,
        cursor: (i32, i32),
        world: &World,
        party: &PartyView,
        inventory: &InventoryView,
        shortcut: &ShortcutView,
        abnormal: &killer_ui::abnormal::AbnormalView,
        minimap: MinimapState,
        chat_lists: &[Vec<ChatEntry>; 5],
        system: &[ChatEntry],
        chat_input: &str,
        sysstrings: Option<&HashMap<u32, String>>,
        font: &BitmapFont,
        alpha: f32,
    ) {
        let status = world.me.as_ref().map(|me| StatusView { regen: world.regen_fill.amount, ..crate::ui2d::status_view(me) });
        let data = WindowData {
            script: self.script,
            buff_size: self.buff_size,
            song_dance_row: self.song_dance_row,
            abnormal,
            minimap,
            inventory,
            sysstrings: if self.sysstrings.is_empty() { sysstrings } else { Some(&self.sysstrings) },
            missing: &self.missing,
            status,
            target: None,
            party,
            shortcut,
            chat_lists,
            system,
            chat_tab: self.state.tab(CHAT_TABS),
            chat_alpha: self.chat_alpha,
            overlay: &self.runtime.host.overlay,
            chat_lines: &self.runtime.host.chat_lines,
            controls: &self.runtime.host.controls,
            font_package: self.font_package,
            chat_scroll: 0,
            chat_input,
            detail: None,
        };
        for index in 0..self.generic.len() {
            if !self.generic_shown(index) {
                continue;
            }
            let entry = &self.generic[index];
            let Some(window) = self.xdat.window(&entry.name) else { continue };
            if std::env::var_os("KILLER_UI_TRACE").is_some() {
                log::info!("janela genérica {} visível em {:?}", entry.name, entry.rect);
            }
            let state = State { cursor: Some((cursor.0 - entry.rect.x, cursor.1 - entry.rect.y)), ..self.state.clone() };
            xdat_view::paint(list, window, &entry.layout, entry.rect.x, entry.rect.y, &state, &data, font, alpha);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn paint(
        &self,
        id: WindowId,
        list: &mut DrawList,
        x: i32,
        y: i32,
        cursor: (i32, i32),
        viewport: (i32, i32),
        world: &World,
        target: Option<&TargetView>,
        party: &PartyView,
        inventory: &InventoryView,
        shortcut: &ShortcutView,
        abnormal: &killer_ui::abnormal::AbnormalView,
        mut minimap: MinimapState,
        zone_id: Option<u32>,
        chat_lists: &[Vec<ChatEntry>; 5],
        system: &[ChatEntry],
        chat_scroll: usize,
        chat_input: &str,
        sysstrings: Option<&HashMap<u32, String>>,
        font: &BitmapFont,
        alpha: f32,
    ) -> bool {
        let Some((window, layout)) = entry(&self.xdat, &self.nested, &self.layouts, id) else { return false };
        let state = State { cursor: Some(cursor), ..self.state.clone() };
        let status = world.me.as_ref().map(|me| StatusView { regen: world.regen_fill.amount, ..crate::ui2d::status_view(me) });
        let target = target.filter(|_| id == WindowId::TARGET);
        let detail = (id == WindowId::MAIN).then(|| crate::ui_keys::detail_status_view(world));
        minimap.town = zone_id.and_then(|id| self.town_maps.get(&id).cloned()).filter(|_| self.town_zoom);
        let data = WindowData {
            script: self.script,
            buff_size: self.buff_size,
            song_dance_row: self.song_dance_row,
            abnormal,
            minimap,
            inventory,
            sysstrings: if self.sysstrings.is_empty() { sysstrings } else { Some(&self.sysstrings) },
            missing: &self.missing,
            status,
            target,
            party,
            shortcut,
            chat_lists,
            system,
            chat_tab: self.state.tab(CHAT_TABS),
            chat_alpha: self.chat_alpha,
            overlay: &self.runtime.host.overlay,
            chat_lines: &self.runtime.host.chat_lines,
            controls: &self.runtime.host.controls,
            font_package: self.font_package,
            chat_scroll,
            chat_input,
            detail,
        };
        xdat_view::paint(list, window, layout, x, y, &state, &data, font, alpha);
        if id == WindowId::MAIN
            && let Some(name) = MAIN_TARGETS.get(self.state.tab(MAIN_TABS))
            && let Some((_, extra)) = self.extra.iter().find(|(known, _)| known == name)
            && let Some(target_window) = self.xdat.window(name)
        {
            let (tx, ty) = xdat_view::place(
                target_window.base.anchor.as_ref(),
                Rect::new(0, 0, viewport.0, viewport.1),
                extra.size,
            );
            xdat_view::paint(list, target_window, extra, tx, ty, &state, &data, font, alpha);
        }
        true
    }

    pub fn mouse_down(&mut self, id: WindowId, x: i32, y: i32) -> Option<Action> {
        let (window, layout) = entry(&self.xdat, &self.nested, &self.layouts, id)?;
        if xdat_view::frame_close_button(window, layout.size).is_some_and(|button| button.contains(x, y)) {
            return Some(Action::Close);
        }
        let hit = layout.hit(window, &self.state, x, y)?.clone();
        let node = xdat_view::node_at(window, &hit.path)?;
        match &node.kind {
            Kind::Button(_) | Kind::EffectButton(_) | Kind::CheckBox(_) => {
                self.state.pressed = Some(hit.name.clone());
                Some(if hit.name == CLOSE_BUTTON { Action::Close } else { Action::Button(hit.name) })
            }
            Kind::Tab(tab) => {
                let index = xdat_view::tab_at(tab, hit.rect, x, y)?;
                self.state.select_tab(&hit.name, index);
                Some(Action::Tab(hit.name, index))
            }
            Kind::ItemWindow(item) => {
                let index = xdat_view::cell_at(item, hit.rect, x, y)?;
                Some(Action::Cell(hit.name, index))
            }
            _ => None,
        }
    }

    pub fn mouse_up(&mut self) {
        self.state.pressed = None;
    }
}

const STATUS_NAME_ORANGE: Argb = Argb(0xFFE6994D);

pub fn buff_rows(script: Script, song_dance_row: bool, abnormal: &killer_ui::abnormal::AbnormalView) -> Vec<Vec<BuffCell>> {
    let cell = |buff: &killer_ui::abnormal::BuffIcon, back: &str| BuffCell {
        icon: buff.icon.clone().unwrap_or_default(),
        back: back.to_string(),
        remaining: buff.remaining,
        short_item: false,
        etc_item: false,
    };
    let silent = script == Script::Silent;
    let is_debuff = |buff: &killer_ui::abnormal::BuffIcon| buff.debuff || (silent && SILENT_DEBUFFS.contains(&buff.skill_id));
    let is_song = |buff: &killer_ui::abnormal::BuffIcon| silent && song_dance_row && SILENT_SONG_DANCE.contains(&buff.skill_id);
    let buffs: Vec<BuffCell> =
        abnormal.buffs.iter().filter(|buff| !is_debuff(buff) && !is_song(buff)).map(|buff| cell(buff, BUFF_BACK)).collect();
    let mut rows: Vec<Vec<BuffCell>> = buffs.chunks(BUFFS_PER_ROW).map(<[BuffCell]>::to_vec).collect();
    let songs: Vec<BuffCell> = abnormal.buffs.iter().filter(|buff| is_song(buff)).map(|buff| cell(buff, BUFF_BACK)).collect();
    if !songs.is_empty() {
        rows.push(songs);
    }
    let debuffs: Vec<BuffCell> = abnormal.buffs.iter().filter(|buff| is_debuff(buff)).map(|buff| cell(buff, DEBUFF_BACK)).collect();
    if !debuffs.is_empty() {
        rows.push(debuffs);
    }
    rows
}

struct WindowData<'a> {
    script: Script,
    buff_size: i32,
    song_dance_row: bool,
    minimap: MinimapState,
    abnormal: &'a killer_ui::abnormal::AbnormalView,
    inventory: &'a InventoryView,
    sysstrings: Option<&'a HashMap<u32, String>>,
    missing: &'a [String],
    status: Option<StatusView>,
    target: Option<&'a TargetView>,
    party: &'a PartyView,
    shortcut: &'a ShortcutView,
    detail: Option<DetailStatusView>,
    chat_lists: &'a [Vec<ChatEntry>; 5],
    system: &'a [ChatEntry],
    chat_tab: usize,
    chat_alpha: Option<i32>,
    overlay: &'a crate::interlude_script::host::Overlay,
    chat_lines: &'a HashMap<String, Vec<(Argb, String)>>,
    controls: &'a crate::interlude_script::vm_host_controls::ControlState,
    font_package: &'a str,
    chat_scroll: usize,
    chat_input: &'a str,
}

fn party_member(key: &str) -> Option<(usize, &str)> {
    let (scope, ctrl) = key.split_once('.')?;
    let index = scope.strip_prefix(PARTY_MEMBER)?.parse::<usize>().ok()?;
    Some((index, ctrl))
}

fn leaf(key: &str) -> &str {
    key.rsplit('.').next().unwrap_or(key)
}

fn detail_text(detail: &DetailStatusView, ctrl: &str) -> Option<String> {
    let pair = |value: (i32, i32)| format!("{}/{}", value.0, value.1);
    Some(match ctrl {
        "txtName1" => detail.name.clone(),
        "txtName2" => detail.nick.clone(),
        "txtPledge" => detail.clan_name.clone().unwrap_or_default(),
        "txtLvl" => detail.level.to_string(),
        "txtHP" => pair(detail.hp),
        "txtMP" => pair(detail.mp),
        "txtCP" => pair(detail.cp),
        "txtExp" => format!("{:.2}%", detail.exp_percent()),
        "txtWeight" => pair((detail.load, detail.max_load)),
        "txtSP" => detail.sp.to_string(),
        "txtPhysicalAttack" => detail.p_atk.to_string(),
        "txtPhysicalDefense" => detail.p_def.to_string(),
        "txtHitRate" => detail.accuracy.to_string(),
        "txtCriticalRate" => detail.critical.to_string(),
        "txtPhysicalAttackSpeed" => detail.atk_spd.to_string(),
        "txtMagicalAttack" => detail.m_atk.to_string(),
        "txtMagicDefense" => detail.m_def.to_string(),
        "txtPhysicalAvoid" => detail.evasion.to_string(),
        "txtMovingSpeed" => detail.speed.to_string(),
        "txtMagicCastingSpeed" => detail.cast_spd.to_string(),
        "txtSTR" => detail.stats[0].to_string(),
        "txtDEX" => detail.stats[1].to_string(),
        "txtCON" => detail.stats[2].to_string(),
        "txtINT" => detail.stats[3].to_string(),
        "txtWIT" => detail.stats[4].to_string(),
        "txtMEN" => detail.stats[5].to_string(),
        "txtCriminalRate" => detail.karma.to_string(),
        "txtPVP" => format!("{}/{}", detail.pvp_kills, detail.pk_kills),
        "txtSociality" => detail.recom_have.to_string(),
        "txtRemainSulffrage" => detail.recom_left.to_string(),
        _ => return None,
    })
}

fn shortcut_index(ctrl: &str) -> Option<usize> {
    ctrl.strip_prefix(SHORTCUT_SLOT)?.parse::<usize>().ok().filter(|n| (1..=SLOTS).contains(n)).map(|n| n - 1)
}

fn pair_text(value: Option<(i32, i32)>) -> Option<String> {
    value.map(|(current, max)| format!("{current} / {max}"))
}

impl Data for WindowData<'_> {
    fn hidden(&self, key: &str) -> bool {
        if let Some(hidden) = self.overlay.hidden(key) {
            return hidden;
        }
        if let Some(index) = CHAT_WINDOWS.iter().position(|name| *name == leaf(key)) {
            return index != self.chat_tab;
        }
        key.strip_prefix(PARTY_MEMBER)
            .and_then(|rest| rest.parse::<usize>().ok())
            .is_some_and(|index| index >= self.party.members.len())
    }

    fn font_package(&self) -> &str {
        self.font_package
    }

    fn alpha_override(&self, key: &str) -> Option<i32> {
        self.overlay.alpha(key).or_else(|| self.chat_alpha.filter(|_| CHAT_TRANSPARENT_CTRLS.contains(&key)))
    }

    fn texture_override(&self, key: &str) -> Option<String> {
        self.overlay.texture(key).map(str::to_string)
    }

    fn checked(&self, key: &str) -> bool {
        self.overlay.checked(key).unwrap_or(false)
    }

    fn chat_scroll_bar(&self, key: &str) -> (i32, i32, i32) {
        if CHAT_WINDOWS.contains(&leaf(key)) {
            match self.script {
                Script::Silent => CHAT_SCROLL_BAR_SILENT,
                Script::Stock => CHAT_SCROLL_BAR_STOCK,
            }
        } else {
            xdat_view::CHAT_SCROLL_BAR_DEFAULT
        }
    }

    fn scroll(&self, key: &str) -> i32 {
        if CHAT_WINDOWS.get(self.chat_tab) == Some(&leaf(key)) { self.chat_scroll as i32 } else { 0 }
    }

    fn text(&self, key: &str) -> Option<String> {
        if let Some(text) = self.overlay.text(key) {
            return Some(text.to_string());
        }
        if let Some((index, ctrl)) = party_member(key) {
            return match ctrl {
                "PlayerName" => self.party.members.get(index).map(|member| member.name.clone()),
                _ => None,
            };
        }
        if let Some(detail) = self.detail.as_ref()
            && let Some(text) = detail_text(detail, leaf(key))
        {
            return Some(text);
        }
        let ctrl = leaf(key);
        if ctrl == TITLE && !key.starts_with("InventoryWnd.") && key != TITLE {
            return None;
        }
        match ctrl {
            TITLE if key.starts_with("InventoryWnd.") || key == TITLE => Some(self.inventory.title.clone()),
            ITEM_COUNT => Some(format!("({}/{})", self.inventory.counted(), self.inventory.limit)),
            ADENA_TEXT => Some(thousands(self.inventory.adena)),
            "UserName" => self
                .target
                .map(|target| target.name.clone())
                .or_else(|| self.status.as_ref().map(|status| status.name.clone())),
            "StatusWnd_LevelTextBox" => self.status.as_ref().map(|status| status.level.to_string()),
            "txtPledgeName" => self.target.and_then(|target| target.clan.clone()),
            "txtPledgeAllianceName" => self.target.and_then(|target| target.ally.clone()),
            "valHP" | "valHP1" => self.target.and_then(|target| pair_text(target.hp)),
            PAGE_TEXT => Some((self.shortcut.page + 1).to_string()),
            CHAT_INPUT => Some(self.chat_input.to_string()),
            _ => None,
        }
    }

    fn lines(&self, key: &str) -> Vec<(Argb, String)> {
        if let Some(lines) = self.chat_lines.get(key) {
            return lines.clone();
        }
        let name = leaf(key);
        let entries: &[ChatEntry] = match CHAT_WINDOWS.iter().position(|known| *known == name) {
            Some(index) => &self.chat_lists[index],
            None if name == SYSTEM_MSG_LIST => self.system,
            None => &[],
        };
        entries.iter().map(|entry| (entry.color(), entry.line())).collect()
    }

    fn sysstring(&self, id: i32) -> Option<String> {
        let id = u32::try_from(id).ok()?;
        self.sysstrings?.get(&id).cloned()
    }

    fn has_texture(&self, file: &str) -> bool {
        !self.missing.iter().any(|known| known.eq_ignore_ascii_case(file))
    }

    fn buff_size(&self) -> i32 {
        self.buff_size
    }

    fn minimap(&self) -> Option<MinimapState> {
        Some(self.minimap.clone())
    }

    fn regen(&self, key: &str) -> f64 {
        if leaf(key) == "HPBar" { self.status.as_ref().map_or(0.0, |status| status.regen) } else { 0.0 }
    }

    fn buff_rows(&self, key: &str) -> Vec<Vec<BuffCell>> {
        if key != BUFF_ICONS {
            return Vec::new();
        }
        buff_rows(self.script, self.song_dance_row, self.abnormal)
    }

    fn name(&self, key: &str) -> Option<(String, TextAlign, Argb)> {
        if let Some((text, _, align, color)) = crate::interlude_script::host::Overlay::lookup(&self.overlay.names, key) {
            let align = match align {
                crate::interlude_script::TextAlign::Center => TextAlign::Center,
                crate::interlude_script::TextAlign::Right => TextAlign::Right,
                _ => TextAlign::Left,
            };
            return Some((text.clone(), align, color.unwrap_or(NAME_DEFAULT_COLOR)));
        }
        match (leaf(key), self.target) {
            ("UserName", Some(target)) => Some((target.name.clone(), TextAlign::Center, NAME_DEFAULT_COLOR)),
            ("RankName", Some(_)) => Some((String::new(), TextAlign::Center, NAME_DEFAULT_COLOR)),
            ("UserName", None) => self.status.as_ref().map(|status| match self.script {
                Script::Silent => (status.name.clone(), TextAlign::Center, STATUS_NAME_ORANGE),
                Script::Stock => (status.name.clone(), TextAlign::Left, NAME_DEFAULT_COLOR),
            }),
            _ => None,
        }
    }

    fn weight(&self) -> Option<(i32, i32)> {
        Some((self.inventory.load, self.inventory.max_load))
    }

    fn bar(&self, key: &str) -> Option<BarPoint> {
        if let Some((current, max)) = self.overlay.point(key) {
            return Some(BarPoint::point(current, max));
        }
        let point = |value: (i32, i32)| BarPoint::point(value.0, value.1);
        if let Some((index, ctrl)) = party_member(key) {
            let member = self.party.members.get(index)?;
            return match ctrl {
                "barCP" => Some(point(member.cp)),
                "barHP" => Some(point(member.hp)),
                "barMP" => Some(point(member.mp)),
                _ => None,
            };
        }
        match leaf(key) {
            "CPBar" => self.status.as_ref().map(|status| point(status.cp)),
            "HPBar" => self.status.as_ref().map(|status| point(status.hp)),
            "MPBar" => self.status.as_ref().map(|status| point(status.mp)),
            "EXPBars" | "EXPBar" => self.status.as_ref().and_then(|status| BarPoint::exp(status.exp, status.level)),
            "barHP" | "barHP1" => self.target.and_then(|target| target.hp).map(point),
            "barMP" => self.target.and_then(|target| target.mp).map(point),
            _ => None,
        }
    }

    fn list_records(&self, key: &str) -> Vec<Vec<killer_ui::xdat_paint_lists::ListCell>> {
        self.controls.list_ctrl(key).map(|list| list.painter_records()).unwrap_or_default()
    }

    fn list_selected(&self, key: &str) -> Option<usize> {
        self.controls.list_ctrl(key).and_then(|list| list.selected)
    }

    fn list_top(&self, key: &str) -> usize {
        self.controls.list_ctrl(key).map_or(0, |list| list.top)
    }

    fn combo_items(&self, key: &str) -> Option<(Vec<String>, Option<usize>)> {
        self.controls.combo_items(key, &|id| self.sysstring(id))
    }

    fn items(&self, key: &str) -> Vec<Cell> {
        if let Some(window) = self.controls.item_window(key) {
            return window.cells();
        }
        let ctrl = leaf(key);
        let inventory = self.inventory;
        let cell = |item: &killer_ui::inventory::InventoryItemView| Cell {
            icon: item.icon(0).to_string(),
            selected: inventory.selected == Some(item.object_id),
            count: (item.count > 1).then_some(item.count),
            equipped: item.equipped,
        };
        if let Some(index) = shortcut_index(ctrl) {
            let slot = self.shortcut.slot(self.shortcut.page as usize * SLOTS + index);
            return slot.map(|slot| Cell { icon: slot.icon().to_string(), selected: false, count: None, equipped: false }).into_iter().collect();
        }
        if let Some((_, slot)) = PAPERDOLL.iter().find(|(name, _)| *name == ctrl) {
            return inventory
                .items
                .iter()
                .filter(|item| {
                    item.equipped
                        && body_part_to_slot(item.body_part, item.object_id, &inventory.paperdoll_ids) == *slot
                })
                .map(cell)
                .collect();
        }
        if !ITEM_GRIDS.contains(&ctrl) {
            return Vec::new();
        }
        let quest = ctrl == QUEST_ITEMS;
        inventory.items.iter().filter(|item| !item.equipped && item.quest == quest).map(cell).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use killer_ui::inventory::{InventoryItemView, InventoryTab, PaperdollIds};

    fn item(object_id: i32, body_part: i32, equipped: bool, quest: bool) -> InventoryItemView {
        InventoryItemView {
            object_id,
            item_id: object_id,
            count: 1,
            enchant: 0,
            body_part,
            equipped,
            quest,
            icons: vec![format!("icon.{object_id}")],
            name: String::new(),
            popup: 0,
            recipe: false,
            crystallizable: false,
            add_name: String::new(),
            description: String::new(),
            weight: 0,
            tip: None,
        }
    }

    fn font() -> BitmapFont {
        use killer_ui::font::{GlyphRect, Glyphs};
        let glyphs = "abcdefghijklmnopqrstuvwxyz -"
            .chars()
            .map(|c| (c, GlyphRect { x: 0, width: if c == ' ' { 3 } else { 6 }, y: 0, height: 10 }))
            .collect();
        BitmapFont::new(String::new(), Glyphs { tex_width: 64, tex_height: 16, glyphs })
    }

    #[test]
    fn the_npc_dialog_follows_the_native_viewer_geometry_and_finds_links_under_the_cursor() {
        let font = font();
        let doc = killer_ui::html::parse("<html><body>ab<br><a action=\"bypass -h x\">cd</a></body></html>");
        assert_eq!(InterludeUi::html_close_button(), Rect::new(310 - 8 - 15, 3, 15, 15));
        assert_eq!(InterludeUi::html_link_at(&doc.body, 0, 7 + 1, 30 + 32 + 1, &font), Some("bypass -h x".to_string()));
        assert_eq!(InterludeUi::html_link_at(&doc.body, 10, 7 + 1, 30 + 22 + 1, &font), Some("bypass -h x".to_string()));
        assert_eq!(InterludeUi::html_link_at(&doc.body, 0, 7 + 1, 30 + 1, &font), None);
        assert_eq!(InterludeUi::html_link_at(&doc.body, 0, 1, 1, &font), None);
        assert_eq!(InterludeUi::html_scroll_limit(&doc.body, &font), 0);
        let long: String = (0..40).map(|_| "ab<br>").collect();
        let doc = killer_ui::html::parse(&format!("<html><body>{long}</body></html>"));
        assert_eq!(InterludeUi::html_scroll_limit(&doc.body, &font), 39 * 32 + 16 - 364);
    }

    #[test]
    fn the_window_data_feeds_the_paperdoll_slots_grids_texts_and_bars_from_the_c4_views() {
        let inventory = InventoryView {
            title: "Inventory".into(),
            tab: InventoryTab::Item,
            items: vec![
                item(1, 0x0040, true, false),
                item(2, 0, false, false),
                item(3, 0, false, true),
                item(4, 0x0002, true, false),
            ],
            adena: 1234567,
            limit: 80,
            paperdoll_ids: PaperdollIds { r_ear: 4, l_ear: 0, r_finger: 0, l_finger: 0 },
            selected: Some(2),
            ..Default::default()
        };
        let mut sysstrings = HashMap::new();
        sysstrings.insert(144u32, "Inventory".to_string());
        let status = StatusView {
            name: "Killer".into(),
            level: 52,
            hp: (400, 900),
            mp: (200, 300),
            cp: (100, 100),
            exp: 0,
            regen: 0.0,
        };
        let shortcut = ShortcutView::new(false);
        let party = PartyView {
            members: vec![killer_ui::party::PartyMemberView {
                object_id: 7,
                name: "Luzal".into(),
                cp: (1, 2),
                hp: (3, 4),
                mp: (5, 6),
                leader: true,
                buffs: Vec::new(),
            }],
            ..Default::default()
        };
        let abnormal = killer_ui::abnormal::AbnormalView::default();
        let controls = crate::interlude_script::vm_host_controls::ControlState::default();
        let data = WindowData {
            song_dance_row: false,
            minimap: MinimapState::default(),
            buff_size: 24,
            abnormal: &abnormal,
            script: Script::Silent,
            inventory: &inventory,
            sysstrings: Some(&sysstrings),
            missing: &["Was.Gone".to_string()],
            status: Some(status),
            target: None,
            shortcut: &shortcut,
            chat_lists: &[vec![ChatEntry::new(0, "Killer", "oi")], Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            system: &[],
            chat_tab: 0,
            chat_alpha: None,
            overlay: &crate::interlude_script::host::Overlay::default(),
            chat_lines: &HashMap::new(),
            controls: &controls,
            font_package: FONT_PACKAGE,
            chat_scroll: 0,
            chat_input: "ol",
            party: &party,
            detail: None,
        };
        assert_eq!(data.items("EquipItem_Head"), vec![Cell { icon: "icon.1".into(), selected: false, count: None, equipped: true }]);
        assert_eq!(data.items("EquipItem_REar"), vec![Cell { icon: "icon.4".into(), selected: false, count: None, equipped: true }]);
        assert_eq!(data.items("EquipItem_LEar"), vec![]);
        assert_eq!(data.items("InventoryItem"), vec![Cell { icon: "icon.2".into(), selected: true, count: None, equipped: false }]);
        assert_eq!(data.items("QuestItem"), vec![Cell { icon: "icon.3".into(), selected: false, count: None, equipped: false }]);
        assert_eq!(data.items("HennaItem"), vec![]);
        assert_eq!(data.text(ITEM_COUNT).as_deref(), Some("(4/80)"));
        assert_eq!(data.text(ADENA_TEXT).as_deref(), Some("1,234,567"));
        assert_eq!(data.text(TITLE).as_deref(), Some("Inventory"));
        assert_eq!(data.text("UserName").as_deref(), Some("Killer"));
        assert_eq!(data.text("StatusWnd_LevelTextBox").as_deref(), Some("52"));
        assert_eq!(data.bar("HPBar"), Some(BarPoint::point(400, 900)));
        assert_eq!(data.bar("CPBar"), Some(BarPoint::point(100, 100)));
        assert_eq!(data.bar("barHP"), None);
        assert_eq!(data.name("UserName"), Some(("Killer".to_string(), TextAlign::Center, STATUS_NAME_ORANGE)));
        let stock = WindowData { script: Script::Stock, status: data.status.clone(), ..data };
        assert_eq!(buff_size(Script::Silent, 3), 22);
        assert_eq!(buff_size(Script::Stock, 0), 24);
        assert_eq!(buff_size(Script::Stock, 3), 0);
        assert_eq!(stock.name("UserName"), Some(("Killer".to_string(), TextAlign::Left, NAME_DEFAULT_COLOR)));
        assert_eq!(stock.name("RankName"), None);
        let data = stock;
        assert_eq!(data.sysstring(144).as_deref(), Some("Inventory"));
        assert_eq!(data.sysstring(-9999), None);
        assert!(!data.has_texture("was.gone"));
        assert!(data.has_texture("Was.Inventory_Back"));
        assert_eq!(data.text(PAGE_TEXT).as_deref(), Some("1"));
        assert_eq!(data.text(CHAT_INPUT).as_deref(), Some("ol"));
        assert_eq!(data.lines("NormalChat")[0].1, "Killer: oi");
        assert_eq!(shortcut_index("Shortcut12"), Some(11));
        assert_eq!(shortcut_index("Shortcut13"), None);
        assert!(data.items("Shortcut1").is_empty());
        assert_eq!(data.text("PartyStatusWnd0.PlayerName").as_deref(), Some("Luzal"));
        assert_eq!(data.bar("PartyStatusWnd0.barHP"), Some(BarPoint::point(3, 4)));
        assert!(!data.hidden("PartyStatusWnd0"));
        assert!(data.hidden("PartyStatusWnd1"));
        assert_eq!(data.text("Container.Title").as_deref(), None);
        assert_eq!(data.text("InventoryWnd.Title").as_deref(), Some("Inventory"));
    }
}
