use crate::hud_h5::HudView;
use killer_ui::board::BoardPages;
use killer_ui::draw::DrawList;
use killer_ui::font::BitmapFont;
use killer_ui::html::{self, Line, Marker};
use killer_ui::npc_viewer::{HTML_WRAP_MARGIN, paint_html_lines};
use killer_ui::window::Rect;
use killer_ui::xdat_window::Layout;

pub const BOARD: &str = "BoardWnd";
pub const HELP_HTML: &str = "HelpHtmlWnd";
pub const OPTION: &str = "OptionWnd";
pub const GM: &str = "GMWnd";

pub const HTML_VIEWER: &str = "HtmlViewer";
pub const BOARD_TAB_CTRL: &str = "TabCtrl";
pub const BOARD_BOOKMARK: &str = "btnBookmark";

pub const HELP_FILE: &str = "l2text/help.htm";

pub const BOARD_TABS: [&str; 8] = [
    "bypass _bbshome",
    "bypass _bbsgetfav",
    "bypass _bbslink",
    "bypass _bbsloc",
    "bypass _bbsclan",
    "bypass _bbsmemo",
    "bypass _maillist_0_1_0_",
    "bypass _friendlist_0_",
];

pub const BOARD_COMMANDS: usize = 8;
pub const BOARD_BOOKMARK_COMMAND: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptionValueKind {
    Bool,
    Int,
    Float,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptionBinding {
    pub control: &'static str,
    pub section: &'static str,
    pub key: &'static str,
    pub kind: OptionValueKind,
}

const fn option(control: &'static str, section: &'static str, key: &'static str, kind: OptionValueKind) -> OptionBinding {
    OptionBinding { control, section, key, kind }
}

pub const OPTION_BINDINGS: &[OptionBinding] = &[
    option("RefreshRateBox", "Video", "RefreshRate", OptionValueKind::Int),
    option("GammaBox", "Video", "Gamma", OptionValueKind::Float),
    option("CharBox", "Video", "PawnClippingRange", OptionValueKind::Int),
    option("TerrainBox", "Video", "TerrainClippingRange", OptionValueKind::Int),
    option("DecoBox", "Video", "RenderDeco", OptionValueKind::Bool),
    option("HDRBox", "Video", "PostProc", OptionValueKind::Int),
    option("ShadowBox", "Video", "PawnShadow", OptionValueKind::Bool),
    option("TextureBox", "Video", "TextureDetail", OptionValueKind::Int),
    option("ModelBox", "Video", "ModelDetail", OptionValueKind::Int),
    option("AnimBox", "Video", "SkipAnim", OptionValueKind::Int),
    option("TriBox", "Video", "UseTrilinear", OptionValueKind::Bool),
    option("CharacterLimitSliderCtrl", "Video", "RenderActorLimitOpt", OptionValueKind::Int),
    option("AABox", "Video", "AntiAliasing", OptionValueKind::Int),
    option("S30Box", "Video", "L2Shader", OptionValueKind::Bool),
    option("S30DOFBOX", "Video", "S30DOFBOX", OptionValueKind::Bool),
    option("S30WaterEffectBox", "Video", "S30WaterEffectBox", OptionValueKind::Bool),
    option("S30ShadowBox", "Video", "S30ShadowBox", OptionValueKind::Bool),
    option("FrameBox", "Video", "IsKeepMinFrameRate", OptionValueKind::Bool),
    option("WeatherEffectComboBox", "Video", "WeatherEffect", OptionValueKind::Int),
    option("GPUAnimationCheckBox", "Video", "GPUAnimation", OptionValueKind::Bool),
    option("CursorBox", "Video", "UseColorCursor", OptionValueKind::Bool),
    option("CaptureBox", "Game", "ScreenShotQuality", OptionValueKind::Int),
    option("DefaultPositionBox", "Game", "LayoutDF", OptionValueKind::Int),
    option("mutecheckbox", "Audio", "AudioMuteOn", OptionValueKind::Bool),
    option("EffectVolumeSliderCtrl", "Audio", "SoundVolume", OptionValueKind::Float),
    option("MusicVolumeSliderCtrl", "Audio", "MusicVolume", OptionValueKind::Float),
    option("SystemVolumeSliderCtrl", "Audio", "WavVoiceVolume", OptionValueKind::Float),
    option("TutorialVolumeSliderCtrl", "Audio", "OggVoiceVolume", OptionValueKind::Float),
    option("OpacityBox", "Game", "TransparencyMode", OptionValueKind::Bool),
    option("LanguageBox", "Game", "IsNative", OptionValueKind::Bool),
    option("NameBox0", "Game", "MyName", OptionValueKind::Bool),
    option("NameBox1", "Game", "NPCName", OptionValueKind::Bool),
    option("NameBox2", "Game", "GroupName", OptionValueKind::Bool),
    option("NameBox3", "Game", "PledgeMemberName", OptionValueKind::Bool),
    option("NameBox4", "Game", "PartyMemberName", OptionValueKind::Bool),
    option("NameBox5", "Game", "OtherPCName", OptionValueKind::Bool),
    option("OldChatBox", "Game", "OldChatting", OptionValueKind::Bool),
    option("SystemTutorialBox", "Game", "SystemTutorialBox", OptionValueKind::Bool),
    option("UniversalToolTipBox", "Game", "UniversalToolTipBox", OptionValueKind::Bool),
    option("ToolTipPositionBox", "Game", "ToolTipPositionBox", OptionValueKind::Int),
    option("JoypadBox", "Game", "UseJoystick", OptionValueKind::Bool),
    option("RightClickBox", "Game", "RightClickBox", OptionValueKind::Bool),
    option("CameraBox", "Game", "AutoTrackingPawn", OptionValueKind::Bool),
    option("ArrowBox", "Game", "ArrowMode", OptionValueKind::Bool),
    option("ZoneNameBox", "Game", "ShowZoneTitle", OptionValueKind::Bool),
    option("GametipBox", "Game", "ShowGameTipMsg", OptionValueKind::Bool),
    option("DuelBox", "Game", "IsRejectingDuel", OptionValueKind::Bool),
    option("PartyRejectBox", "Game", "IsRejectingParty", OptionValueKind::Bool),
    option("CoupleActionBox", "Game", "IsCoupleAction", OptionValueKind::Bool),
    option("DropItemBox", "Game", "HideDropItem", OptionValueKind::Bool),
    option("LazyModeBox", "Game", "UseLazyMode", OptionValueKind::Bool),
    option("LootingBox", "Game", "PartyLooting", OptionValueKind::Int),
    option("ReflectBox", "L2WaterEffect", "IsUseEffect", OptionValueKind::Bool),
];

pub const OPTION_ENGINE_ONLY: [&str; 2] = ["ResBox", "KeyboardBox"];

pub const GM_BUTTONS: [&str; 19] = [
    "TeleButton",
    "MoveButton",
    "RecallButton",
    "DetailStatusButton",
    "InventoryButton",
    "MagicSkillButton",
    "QuestButton",
    "InfoButton",
    "StoreButton",
    "ClanButton",
    "PetitionButton",
    "SendHomeButton",
    "NPCListButton",
    "ItemListButton",
    "SkillListButton",
    "ForcePetitionButton",
    "ChangeServerButton",
    "btnClassChange",
    "UIButton",
];

pub const GM_EDIT_BOX: &str = "EditBox";

pub fn gamma_index(gamma: f32) -> usize {
    if gamma >= 1.2 {
        0
    } else if gamma >= 1.0 {
        1
    } else if gamma >= 0.8 {
        2
    } else if gamma >= 0.6 {
        3
    } else {
        4
    }
}

pub fn volume_tick(volume: f32) -> usize {
    if volume >= 1.0 {
        return 5;
    }
    ((volume.max(0.0) * 5.0) as usize).min(5)
}

#[derive(Default)]
struct HtmlPane {
    lines: Vec<Line>,
    height: i32,
    scroll: i32,
    source: String,
    width: i32,
}

impl HtmlPane {
    fn refresh(&mut self, text: &str, width: i32, view: &HudView) {
        if self.source == text && self.width == width {
            return;
        }
        let resolve = |marker: Marker, id: i32| match marker {
            Marker::SysString if id > 0 => (view.sysstring)(id as u32),
            _ => None,
        };
        let document = html::parse_with(text, &resolve);
        self.lines = html::layout(&document, width.max(1), view.font);
        self.height = html::height(&self.lines, view.font);
        self.source = text.to_string();
        self.width = width;
        self.scroll = 0;
    }

    fn max_scroll(&self, visible: i32) -> i32 {
        (self.height - visible).max(0)
    }

    fn scroll_by(&mut self, dy: i32, visible: i32) {
        self.scroll = (self.scroll + dy).clamp(0, self.max_scroll(visible));
    }
}

#[derive(Default)]
pub struct SystemWindows {
    board_pages: BoardPages,
    board_raw: String,
    board_text: String,
    board_commands: [String; BOARD_COMMANDS],
    board_tab: usize,
    board: HtmlPane,
    help: HtmlPane,
    help_text: String,
    option_tab: usize,
    reports: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WindowPlan {
    pub hide: Vec<&'static str>,
    pub tab_selected: usize,
}

impl SystemWindows {
    pub fn set_help_html(&mut self, text: &str) {
        self.help_text = text.to_string();
    }

    pub fn has_help_html(&self) -> bool {
        !self.help_text.is_empty()
    }

    pub fn drain_reports(&mut self) -> Vec<&'static str> {
        std::mem::take(&mut self.reports)
    }

    pub fn board_tab(&self) -> usize {
        self.board_tab
    }

    pub fn option_tab(&self) -> usize {
        self.option_tab
    }

    pub fn set_option_tab(&mut self, tab: usize) {
        self.option_tab = tab;
    }

    pub fn board_command(&self, index: usize) -> Option<&str> {
        self.board_commands.get(index).map(String::as_str).filter(|command| !command.is_empty())
    }

    fn note(&mut self, what: &'static str) {
        if !self.reports.contains(&what) {
            self.reports.push(what);
        }
    }

    fn take_board(&mut self, view: &HudView) -> bool {
        let Some(board) = view.world.windows.board.as_ref() else { return false };
        if self.board_raw == board.html {
            return true;
        }
        self.board_raw.clone_from(&board.html);
        for (slot, link) in self.board_commands.iter_mut().zip(board.links.iter()) {
            slot.clone_from(link);
        }
        if let Some(shown) = self.board_pages.receive(&board.html) {
            self.board_text = shown.text;
        }
        true
    }
}

fn content_rect(layout: &Layout, name: &str) -> Option<Rect> {
    layout.find(name).map(|item| item.rect)
}

pub fn plan_board(state: &mut SystemWindows, view: &HudView) -> Option<WindowPlan> {
    let mut plan = WindowPlan { tab_selected: state.board_tab, ..WindowPlan::default() };
    if !state.take_board(view) {
        state.note("BoardWnd: servidor ainda não enviou ShowBoard do H5 (opcode 0x7b não lido); janela vazia");
    }
    if state.board_command(BOARD_BOOKMARK_COMMAND).is_none() {
        plan.hide.push(BOARD_BOOKMARK);
    }
    Some(plan)
}

pub fn plan_help(state: &mut SystemWindows, _view: &HudView) -> Option<WindowPlan> {
    if !state.has_help_html() {
        state.note("HelpHtmlWnd: l2text/help.htm não carregado pelo app; janela vazia");
    }
    Some(WindowPlan::default())
}

pub fn plan_option(state: &mut SystemWindows, _view: &HudView) -> Option<WindowPlan> {
    state.note("OptionWnd: valores do option.ini não chegam ao HUD e CheckBox/ComboBox/SliderCtrl não são pintados pelo xdat_window; só a estrutura da janela aparece");
    Some(WindowPlan { tab_selected: state.option_tab, ..WindowPlan::default() })
}

pub fn plan_gm(state: &mut SystemWindows, _view: &HudView) -> Option<WindowPlan> {
    state.note("GMWnd: comandos de GM não têm canal portado no H5; os botões desenham mas não enviam nada");
    Some(WindowPlan::default())
}

pub fn draw_board(state: &mut SystemWindows, layout: &Layout, view: &HudView, list: &mut DrawList) {
    let Some(rect) = content_rect(layout, HTML_VIEWER) else { return };
    state.board.refresh(&state.board_text, rect.w - HTML_WRAP_MARGIN, view);
    paint_html_lines(list, rect.x, rect.y, (rect.w, rect.h), &state.board.lines, state.board.scroll, view.font, 1.0);
}

pub fn draw_help(state: &mut SystemWindows, layout: &Layout, view: &HudView, list: &mut DrawList) {
    let Some(rect) = content_rect(layout, HTML_VIEWER) else { return };
    state.help.refresh(&state.help_text, rect.w - HTML_WRAP_MARGIN, view);
    paint_html_lines(list, rect.x, rect.y, (rect.w, rect.h), &state.help.lines, state.help.scroll, view.font, 1.0);
}

pub fn scroll_board(state: &mut SystemWindows, layout: &Layout, dy: i32) {
    let visible = content_rect(layout, HTML_VIEWER).map_or(0, |rect| rect.h);
    state.board.scroll_by(dy, visible);
}

pub fn scroll_help(state: &mut SystemWindows, layout: &Layout, dy: i32) {
    let visible = content_rect(layout, HTML_VIEWER).map_or(0, |rect| rect.h);
    state.help.scroll_by(dy, visible);
}

pub fn board_tab_command(index: usize) -> Option<String> {
    BOARD_TABS.get(index).map(|bypass| bypass.to_string())
}

pub fn select_board_tab(state: &mut SystemWindows, index: usize) -> Option<String> {
    if index >= BOARD_TABS.len() {
        return None;
    }
    state.board_tab = index;
    board_tab_command(index)
}

pub fn board_bookmark(state: &SystemWindows) -> Option<String> {
    state.board_command(BOARD_BOOKMARK_COMMAND).map(str::to_string)
}

pub fn board_link_at(state: &SystemWindows, layout: &Layout, x: i32, y: i32, font: &BitmapFont) -> Option<String> {
    let rect = content_rect(layout, HTML_VIEWER)?;
    if !rect.contains(x, y) {
        return None;
    }
    let action = html::link_at(&state.board.lines, x - rect.x, y - rect.y + state.board.scroll, font)?;
    html::bypass_command(action)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn board_tabs_follow_the_high_five_script_with_the_link_tab() {
        assert_eq!(BOARD_TABS[0], "bypass _bbshome");
        assert_eq!(BOARD_TABS[2], "bypass _bbslink");
        assert_eq!(BOARD_TABS[3], "bypass _bbsloc");
        assert_eq!(BOARD_TABS[7], "bypass _friendlist_0_");
        assert_eq!(BOARD_TABS.len(), BOARD_COMMANDS);
    }

    #[test]
    fn gamma_buckets_match_the_option_script() {
        assert_eq!(gamma_index(1.3), 0);
        assert_eq!(gamma_index(1.0), 1);
        assert_eq!(gamma_index(0.9), 2);
        assert_eq!(gamma_index(0.7), 3);
        assert_eq!(gamma_index(0.5), 4);
    }

    #[test]
    fn volume_ticks_match_the_option_script() {
        assert_eq!(volume_tick(0.0), 0);
        assert_eq!(volume_tick(0.3), 1);
        assert_eq!(volume_tick(0.5), 2);
        assert_eq!(volume_tick(0.7), 3);
        assert_eq!(volume_tick(0.9), 4);
        assert_eq!(volume_tick(1.0), 5);
    }

    #[test]
    fn every_option_control_maps_to_a_real_ini_section() {
        for binding in OPTION_BINDINGS {
            assert!(!binding.control.is_empty());
            assert!(matches!(binding.section, "Video" | "Audio" | "Game" | "L2WaterEffect"));
            assert!(!binding.key.is_empty());
        }
    }
}
