use std::path::Path;
use thiserror::Error;
use ue2_core::{ReadError, Reader};

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum Error {
    #[error("dados truncados em {0:#x}")]
    Truncated(usize),
    #[error("índice compacto inválido em {0:#x}")]
    BadCompactIndex(usize),
    #[error("texto inválido em {0:#x}")]
    InvalidString(usize),
    #[error("contagem negativa em {0:#x}")]
    NegativeCount(usize),
    #[error("booleano {1} em {0:#x}")]
    BadBool(usize, i32),
    #[error("índice {1} fora da tabela de {2} em {0:#x}")]
    BadIndex(usize, i32, &'static str),
    #[error("tipo de controle desconhecido {1:?} em {0:#x}")]
    UnknownControl(usize, String),
    #[error("{0} bytes sobraram depois do fim em {1:#x}")]
    Trailing(usize, usize),
    #[error("cifra: {0}")]
    Crypto(String),
}

impl From<ReadError> for Error {
    fn from(error: ReadError) -> Self {
        match error {
            ReadError::UnexpectedEof(at) => Self::Truncated(at),
            ReadError::BadCompactIndex(at) => Self::BadCompactIndex(at),
            ReadError::InvalidString(at) => Self::InvalidString(at),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    None,
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Alignment {
    const ALL: [Self; 10] = [
        Self::None,
        Self::TopLeft,
        Self::TopCenter,
        Self::TopRight,
        Self::CenterLeft,
        Self::Center,
        Self::CenterRight,
        Self::BottomLeft,
        Self::BottomCenter,
        Self::BottomRight,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureKind {
    Stretch,
    Normal,
    Tile,
    Draggable,
    Control,
    Mask,
}

impl TextureKind {
    const ALL: [Self; 6] = [Self::Stretch, Self::Normal, Self::Tile, Self::Draggable, Self::Control, Self::Mask];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextureLayer {
    None,
    Normal,
    Background,
}

impl TextureLayer {
    const ALL: [Self; 3] = [Self::None, Self::Normal, Self::Background];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextAlign {
    Undefined,
    Left,
    Center,
    Right,
    MacroIcon,
}

impl TextAlign {
    const ALL: [Self; 5] = [Self::Undefined, Self::Left, Self::Center, Self::Right, Self::MacroIcon];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EditKind {
    Normal,
    Chat,
    Password,
    Count,
    Number,
    Unk5,
    Type6,
}

impl EditKind {
    const ALL: [Self; 7] =
        [Self::Normal, Self::Chat, Self::Password, Self::Count, Self::Number, Self::Unk5, Self::Type6];
}

#[derive(Debug, Clone, PartialEq)]
pub struct Percent {
    pub relative_to: String,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Size {
    pub percent: Option<Percent>,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Anchor {
    pub parent: Alignment,
    pub this: Alignment,
    pub ctrl: String,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Base {
    pub name: String,
    pub extends: String,
    pub always_on_top: i32,
    pub always_on_back: i32,
    pub parent_name: String,
    pub tab_pre_order: String,
    pub tab_post_order: String,
    pub anchored: i32,
    pub size: Option<Size>,
    pub anchor: Option<Anchor>,
    pub virtual_: i32,
    pub pass_through: i32,
    pub unk68: i32,
    pub tooltip_type: String,
    pub tooltip_text: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Window {
    pub base: Base,
    pub parent: String,
    pub back_tex: String,
    pub script: String,
    pub state: String,
    pub frame: i32,
    pub iconable: i32,
    pub stuckable: i32,
    pub hidden: i32,
    pub always_full_alpha: i32,
    pub save_position: i32,
    pub save_size: i32,
    pub title: i32,
    pub resize_frame: i32,
    pub frame_size: i32,
    pub frame_direction: i32,
    pub frame_exit_button: i32,
    pub draggable: i32,
    pub resize_frame_direction: i32,
    pub resize_frame_rect: [i32; 4],
    pub resize_max_increase: i32,
    pub expand_button: i32,
    pub expand_org_height: i32,
    pub expand_exp_height: i32,
    pub drawer_direction: i32,
    pub drawer_offset: i32,
    pub drawer_direction_fixed: i32,
    pub drawer_owner_window: String,
    pub show_anim: i32,
    pub hide_anim: i32,
    pub show_anim_info: [i32; 2],
    pub hide_anim_info: [i32; 2],
    pub icon_name: String,
    pub tooltip_idx: i32,
    pub hook_key_input: i32,
    pub working_configuration: String,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    pub file: String,
    pub kind: TextureKind,
    pub layer: TextureLayer,
    pub u: i32,
    pub v: i32,
    pub u_size: f32,
    pub v_size: f32,
    pub alpha: i32,
    pub anim_tex: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    pub normal_tex: String,
    pub pushed_tex: String,
    pub highlight_tex: String,
    pub drop_tex: String,
    pub button_name: i32,
    pub no_highlight: i32,
    pub default_sound_on: i32,
    pub disable_time: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextBox {
    pub text: String,
    pub align: TextAlign,
    pub offset_y: i32,
    pub back_tex: String,
    pub font_type: i32,
    pub sysstring: i32,
    pub system_msg: i32,
    pub text_color: u32,
    pub emoticon: i32,
    pub autosize: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckBox {
    pub title_index: i32,
    pub checked: i32,
    pub left_aligned: i32,
    pub max_width: i32,
    pub check_texture: String,
    pub uncheck_texture: String,
    pub disable_texture: String,
    pub disable_check_texture: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadioButton {
    pub sysstring: i32,
    pub radio_group_id: i32,
    pub is_checked: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EditBox {
    pub kind: EditKind,
    pub max_length: i32,
    pub show_cursor: i32,
    pub chat_mark_on: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiEdit {
    pub unk100: i32,
    pub unk101: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComboBoxElement {
    pub sys_string: i32,
    pub system_msg: i32,
    pub text: String,
    pub reserved: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComboBox {
    pub values: Vec<ComboBoxElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListElement {
    pub text_string_id: i32,
    pub width: i32,
    pub unk108: bool,
    pub unk109: bool,
    pub unk110: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListCtrl {
    pub unk100_104: [i32; 5],
    pub values: Vec<ListElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextListBox {
    pub max_row: i32,
    pub show_row: i32,
    pub line_gap: i32,
    pub is_show_scroll: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TreeCtrl {
    pub save_expanded_node: bool,
    pub multi_expand: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TabElement {
    pub button_name: i32,
    pub target: String,
    pub width: i32,
    pub height: i32,
    pub normal_tex: String,
    pub pushed_tex: String,
    pub movable: bool,
    pub gap: i32,
    pub tooltip: i32,
    pub no_highlight: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tab {
    pub tabs: Vec<TabElement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScrollArea {
    pub area_height: i32,
    pub children: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemWindow {
    pub col: i32,
    pub row: i32,
    pub max_item_num: i32,
    pub icon_width: i32,
    pub icon_height: i32,
    pub gap_x: i32,
    pub gap_y: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub background_item_width: i32,
    pub background_item_height: i32,
    pub background_item_tex: String,
    pub selected_item_width: i32,
    pub selected_item_height: i32,
    pub selected_item_tex: String,
    pub unselected_item_width: i32,
    pub unselected_item_height: i32,
    pub unselected_item_tex: String,
    pub no_select_item: i32,
    pub no_item_drag: i32,
    pub button_click: i32,
    pub use_cool_time: i32,
    pub no_scroll: i32,
    pub out_line_up: String,
    pub out_line_down: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SliderCtrl {
    pub num_of_tick: i32,
    pub curr_tick: i32,
    pub thumb_btn_width: i32,
    pub thumb_btn_height: i32,
    pub back_texture: String,
    pub disable_back_texture: String,
    pub thumb_btn_normal_texture: String,
    pub thumb_btn_down_texture: String,
    pub push_btn_width: i32,
    pub push_btn_height: i32,
    pub push_btn_auto_hit_time: i32,
    pub unk111: i32,
    pub tick_texture: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatusBar {
    pub title: String,
    pub texture_width: i32,
    pub texture_height: i32,
    pub fore_texture: String,
    pub back_texture: String,
    pub warn_texture: String,
    pub regen_texture: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectButton {
    pub kind: i32,
    pub normal_tex: String,
    pub pushed_tex: String,
    pub highlight_tex: String,
    pub effect_tex1: String,
    pub effect_tex2: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MinimapCtrl {
    pub show_time: bool,
    pub show_town: bool,
    pub show_grid: bool,
    pub show_my_loc_mark: bool,
    pub show_my_loc_text: bool,
    pub show_ssq_text: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BarCtrl {
    pub fore_texture: String,
    pub back_texture: String,
    pub u_size: i32,
    pub v_size: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Progress {
    pub back_texture: String,
    pub bar_texture: String,
    pub gap: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FishViewportWindow {
    pub tex_back: String,
    pub tex_clock: String,
    pub tex_fish_hp_bar: String,
    pub tex_fish_hp_bar_back: String,
    pub tex_fish_fake_hp_bar_warning: String,
    pub tex_fishing_effect: String,
    pub tex_icon_pumping: String,
    pub tex_icon_reeling: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatusIconCtrl {
    pub no_clip: bool,
    pub no_tooltip: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlCtrl {
    pub view_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChatWindow {
    pub line_gap: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShortcutItemWindow {
    pub always_show_outline: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Window(Box<Window>),
    BarCtrl(BarCtrl),
    Button(Button),
    ChatWindow(ChatWindow),
    CheckBox(CheckBox),
    ComboBox(ComboBox),
    EditBox(EditBox),
    EffectButton(EffectButton),
    FishViewportWindow(FishViewportWindow),
    HtmlCtrl(HtmlCtrl),
    InvenWeight,
    ItemWindow(Box<ItemWindow>),
    ListCtrl(ListCtrl),
    MinimapCtrl(MinimapCtrl),
    MultiEdit(MultiEdit),
    MultiSellItemInfo,
    MultiSellNeededItem,
    NameCtrl,
    Progress(Progress),
    Radar,
    RadioButton(RadioButton),
    ScrollArea(ScrollArea),
    ShortcutItemWindow(ShortcutItemWindow),
    SliderCtrl(SliderCtrl),
    StatusBar(StatusBar),
    StatusIconCtrl(StatusIconCtrl),
    Tab(Tab),
    TextBox(TextBox),
    TextListBox(TextListBox),
    Texture(Texture),
    TreeCtrl(TreeCtrl),
}

impl Kind {
    pub const CLASSES: [&'static str; 31] = [
        "Window",
        "BarCtrl",
        "Button",
        "ChatWindow",
        "CheckBox",
        "ComboBox",
        "EditBox",
        "EffectButton",
        "FishViewportWindow",
        "HtmlCtrl",
        "InvenWeight",
        "ItemWindow",
        "ListCtrl",
        "MinimapCtrl",
        "MultiEdit",
        "MultiSellItemInfo",
        "MultiSellNeededItem",
        "NameCtrl",
        "Progress",
        "Radar",
        "RadioButton",
        "ScrollArea",
        "ShortcutItemWindow",
        "SliderCtrl",
        "StatusBar",
        "StatusIconCtrl",
        "Tab",
        "TextBox",
        "TextListBox",
        "Texture",
        "TreeCtrl",
    ];

    pub fn class_name(&self) -> &'static str {
        match self {
            Self::Window(_) => "Window",
            Self::BarCtrl(_) => "BarCtrl",
            Self::Button(_) => "Button",
            Self::ChatWindow(_) => "ChatWindow",
            Self::CheckBox(_) => "CheckBox",
            Self::ComboBox(_) => "ComboBox",
            Self::EditBox(_) => "EditBox",
            Self::EffectButton(_) => "EffectButton",
            Self::FishViewportWindow(_) => "FishViewportWindow",
            Self::HtmlCtrl(_) => "HtmlCtrl",
            Self::InvenWeight => "InvenWeight",
            Self::ItemWindow(_) => "ItemWindow",
            Self::ListCtrl(_) => "ListCtrl",
            Self::MinimapCtrl(_) => "MinimapCtrl",
            Self::MultiEdit(_) => "MultiEdit",
            Self::MultiSellItemInfo => "MultiSellItemInfo",
            Self::MultiSellNeededItem => "MultiSellNeededItem",
            Self::NameCtrl => "NameCtrl",
            Self::Progress(_) => "Progress",
            Self::Radar => "Radar",
            Self::RadioButton(_) => "RadioButton",
            Self::ScrollArea(_) => "ScrollArea",
            Self::ShortcutItemWindow(_) => "ShortcutItemWindow",
            Self::SliderCtrl(_) => "SliderCtrl",
            Self::StatusBar(_) => "StatusBar",
            Self::StatusIconCtrl(_) => "StatusIconCtrl",
            Self::Tab(_) => "Tab",
            Self::TextBox(_) => "TextBox",
            Self::TextListBox(_) => "TextListBox",
            Self::Texture(_) => "Texture",
            Self::TreeCtrl(_) => "TreeCtrl",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub base: Base,
    pub kind: Kind,
}

impl Node {
    pub fn children(&self) -> &[Node] {
        match &self.kind {
            Kind::Window(window) => &window.children,
            Kind::ScrollArea(area) => &area.children,
            _ => &[],
        }
    }

    pub fn walk<'a>(&'a self, out: &mut Vec<&'a Node>) {
        out.push(self);
        for child in self.children() {
            child.walk(out);
        }
    }
}

impl Window {
    pub fn nodes(&self) -> Vec<&Node> {
        let mut out = Vec::new();
        for child in &self.children {
            child.walk(&mut out);
        }
        out
    }

    pub fn find(&self, name: &str) -> Option<&Node> {
        self.nodes().into_iter().find(|node| node.base.name == name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutAction {
    pub keys: [i32; 3],
    pub action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shortcut {
    pub unk0: String,
    pub unk1: String,
    pub actions: Vec<ShortcutAction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WndDefPos {
    pub wnd: String,
    pub alignment: Alignment,
    pub x: i32,
    pub y: i32,
    pub move_parent: bool,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Xdat {
    pub windows: Vec<Window>,
    pub shortcuts: Vec<Shortcut>,
    pub default_positions: Vec<Vec<WndDefPos>>,
    pub tail: Vec<u8>,
}

impl Xdat {
    pub fn window(&self, name: &str) -> Option<&Window> {
        self.windows.iter().find(|window| window.base.name == name)
    }

    pub fn wnd_def_pos(&self) -> impl Iterator<Item = &WndDefPos> {
        self.default_positions.iter().flatten()
    }
}

pub fn read_file(data: &[u8], path: &Path) -> Result<Xdat, Error> {
    let body = l2_crypto::decrypt(data, path).map_err(|error| Error::Crypto(error.to_string()))?;
    read(&body)
}

pub fn read(body: &[u8]) -> Result<Xdat, Error> {
    let mut r = Reader::at(body, 0);
    let windows = list(&mut r, window)?;
    let shortcuts = list(&mut r, shortcut)?;
    let default_positions = list(&mut r, |r| list(r, wnd_def_pos))?;
    let left = r.remaining().len();
    let tail = match left {
        0 | 20 | 40 => r.remaining().to_vec(),
        _ => return Err(Error::Trailing(left, r.pos())),
    };
    Ok(Xdat { windows, shortcuts, default_positions, tail })
}

fn list<'a, T>(r: &mut Reader<'a>, item: fn(&mut Reader<'a>) -> Result<T, Error>) -> Result<Vec<T>, Error> {
    let at = r.pos();
    let count = usize::try_from(r.i32()?).map_err(|_| Error::NegativeCount(at))?;
    (0..count).map(|_| item(r)).collect()
}

fn boolean(r: &mut Reader<'_>) -> Result<bool, Error> {
    let at = r.pos();
    match r.i32()? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(Error::BadBool(at, other)),
    }
}

fn indexed<T: Copy>(r: &mut Reader<'_>, table: &[T], name: &'static str) -> Result<T, Error> {
    let at = r.pos();
    let index = r.i32()?;
    usize::try_from(index).ok().and_then(|i| table.get(i).copied()).ok_or(Error::BadIndex(at, index, name))
}

fn ints<const N: usize>(r: &mut Reader<'_>) -> Result<[i32; N], Error> {
    let mut out = [0; N];
    for slot in &mut out {
        *slot = r.i32()?;
    }
    Ok(out)
}

fn base(r: &mut Reader<'_>) -> Result<Base, Error> {
    let name = r.string()?;
    let extends = r.string()?;
    let always_on_top = r.i32()?;
    let always_on_back = r.i32()?;
    let parent_name = r.string()?;
    let tab_pre_order = r.string()?;
    let tab_post_order = r.string()?;
    let anchored = r.i32()?;
    let size = if boolean(r)? {
        let percent = if boolean(r)? {
            None
        } else {
            let relative_to = r.string()?;
            let width = r.f32()?;
            let height = r.f32()?;
            Some(Percent { relative_to, width, height })
        };
        let width = r.i32()?;
        let height = r.i32()?;
        Some(Size { percent, width, height })
    } else {
        None
    };
    let anchor = if boolean(r)? {
        let parent = indexed(r, &Alignment::ALL, "Alignment")?;
        let this = indexed(r, &Alignment::ALL, "Alignment")?;
        let ctrl = r.string()?;
        let x = r.i32()?;
        let y = r.i32()?;
        Some(Anchor { parent, this, ctrl, x, y })
    } else {
        None
    };
    let virtual_ = r.i32()?;
    let pass_through = r.i32()?;
    let unk68 = r.i32()?;
    let tooltip_type = r.string()?;
    let tooltip_text = r.i32()?;
    Ok(Base {
        name,
        extends,
        always_on_top,
        always_on_back,
        parent_name,
        tab_pre_order,
        tab_post_order,
        anchored,
        size,
        anchor,
        virtual_,
        pass_through,
        unk68,
        tooltip_type,
        tooltip_text,
    })
}

fn window(r: &mut Reader<'_>) -> Result<Window, Error> {
    let base = base(r)?;
    window_after_base(r, base)
}

fn window_after_base(r: &mut Reader<'_>, base: Base) -> Result<Window, Error> {
    let parent = r.string()?;
    let back_tex = r.string()?;
    let script = r.string()?;
    let state = r.string()?;
    let frame = r.i32()?;
    let iconable = r.i32()?;
    let stuckable = r.i32()?;
    let hidden = r.i32()?;
    let always_full_alpha = r.i32()?;
    let save_position = r.i32()?;
    let save_size = r.i32()?;
    let title = r.i32()?;
    let resize_frame = r.i32()?;
    let frame_size = r.i32()?;
    let frame_direction = r.i32()?;
    let frame_exit_button = r.i32()?;
    let draggable = r.i32()?;
    let resize_frame_direction = r.i32()?;
    let resize_frame_rect = ints(r)?;
    let resize_max_increase = r.i32()?;
    let expand_button = r.i32()?;
    let expand_org_height = r.i32()?;
    let expand_exp_height = r.i32()?;
    let drawer_direction = r.i32()?;
    let drawer_offset = r.i32()?;
    let drawer_direction_fixed = r.i32()?;
    let drawer_owner_window = r.string()?;
    let show_anim = r.i32()?;
    let hide_anim = r.i32()?;
    let show_anim_info = ints(r)?;
    let hide_anim_info = ints(r)?;
    let icon_name = r.string()?;
    let tooltip_idx = r.i32()?;
    let hook_key_input = r.i32()?;
    let working_configuration = r.string()?;
    let children = list(r, node)?;
    Ok(Window {
        base,
        parent,
        back_tex,
        script,
        state,
        frame,
        iconable,
        stuckable,
        hidden,
        always_full_alpha,
        save_position,
        save_size,
        title,
        resize_frame,
        frame_size,
        frame_direction,
        frame_exit_button,
        draggable,
        resize_frame_direction,
        resize_frame_rect,
        resize_max_increase,
        expand_button,
        expand_org_height,
        expand_exp_height,
        drawer_direction,
        drawer_offset,
        drawer_direction_fixed,
        drawer_owner_window,
        show_anim,
        hide_anim,
        show_anim_info,
        hide_anim_info,
        icon_name,
        tooltip_idx,
        hook_key_input,
        working_configuration,
        children,
    })
}

fn node(r: &mut Reader<'_>) -> Result<Node, Error> {
    let at = r.pos();
    let class = r.string()?;
    if !Kind::CLASSES.contains(&class.as_str()) {
        return Err(Error::UnknownControl(at, class));
    }
    let base = base(r)?;
    let kind = match class.as_str() {
        "Window" => Kind::Window(Box::new(window_after_base(r, base.clone())?)),
        "BarCtrl" => Kind::BarCtrl(BarCtrl {
            fore_texture: r.string()?,
            back_texture: r.string()?,
            u_size: r.i32()?,
            v_size: r.i32()?,
        }),
        "Button" => Kind::Button(Button {
            normal_tex: r.string()?,
            pushed_tex: r.string()?,
            highlight_tex: r.string()?,
            drop_tex: r.string()?,
            button_name: r.i32()?,
            no_highlight: r.i32()?,
            default_sound_on: r.i32()?,
            disable_time: r.i32()?,
        }),
        "ChatWindow" => Kind::ChatWindow(ChatWindow { line_gap: r.i32()? }),
        "CheckBox" => Kind::CheckBox(CheckBox {
            title_index: r.i32()?,
            checked: r.i32()?,
            left_aligned: r.i32()?,
            max_width: r.i32()?,
            check_texture: r.string()?,
            uncheck_texture: r.string()?,
            disable_texture: r.string()?,
            disable_check_texture: r.string()?,
        }),
        "ComboBox" => Kind::ComboBox(ComboBox { values: list(r, combo_element)? }),
        "EditBox" => Kind::EditBox(EditBox {
            kind: indexed(r, &EditKind::ALL, "EditBox.Type")?,
            max_length: r.i32()?,
            show_cursor: r.i32()?,
            chat_mark_on: boolean(r)?,
        }),
        "EffectButton" => Kind::EffectButton(EffectButton {
            kind: r.i32()?,
            normal_tex: r.string()?,
            pushed_tex: r.string()?,
            highlight_tex: r.string()?,
            effect_tex1: r.string()?,
            effect_tex2: r.string()?,
        }),
        "FishViewportWindow" => Kind::FishViewportWindow(FishViewportWindow {
            tex_back: r.string()?,
            tex_clock: r.string()?,
            tex_fish_hp_bar: r.string()?,
            tex_fish_hp_bar_back: r.string()?,
            tex_fish_fake_hp_bar_warning: r.string()?,
            tex_fishing_effect: r.string()?,
            tex_icon_pumping: r.string()?,
            tex_icon_reeling: r.string()?,
        }),
        "HtmlCtrl" => Kind::HtmlCtrl(HtmlCtrl { view_type: r.string()? }),
        "InvenWeight" => Kind::InvenWeight,
        "ItemWindow" => Kind::ItemWindow(Box::new(ItemWindow {
            col: r.i32()?,
            row: r.i32()?,
            max_item_num: r.i32()?,
            icon_width: r.i32()?,
            icon_height: r.i32()?,
            gap_x: r.i32()?,
            gap_y: r.i32()?,
            offset_x: r.i32()?,
            offset_y: r.i32()?,
            background_item_width: r.i32()?,
            background_item_height: r.i32()?,
            background_item_tex: r.string()?,
            selected_item_width: r.i32()?,
            selected_item_height: r.i32()?,
            selected_item_tex: r.string()?,
            unselected_item_width: r.i32()?,
            unselected_item_height: r.i32()?,
            unselected_item_tex: r.string()?,
            no_select_item: r.i32()?,
            no_item_drag: r.i32()?,
            button_click: r.i32()?,
            use_cool_time: r.i32()?,
            no_scroll: r.i32()?,
            out_line_up: r.string()?,
            out_line_down: r.string()?,
        })),
        "ListCtrl" => Kind::ListCtrl(ListCtrl { unk100_104: ints(r)?, values: list(r, list_element)? }),
        "MinimapCtrl" => Kind::MinimapCtrl(MinimapCtrl {
            show_time: boolean(r)?,
            show_town: boolean(r)?,
            show_grid: boolean(r)?,
            show_my_loc_mark: boolean(r)?,
            show_my_loc_text: boolean(r)?,
            show_ssq_text: boolean(r)?,
        }),
        "MultiEdit" => Kind::MultiEdit(MultiEdit { unk100: r.i32()?, unk101: r.i32()? }),
        "MultiSellItemInfo" => Kind::MultiSellItemInfo,
        "MultiSellNeededItem" => Kind::MultiSellNeededItem,
        "NameCtrl" => Kind::NameCtrl,
        "Progress" => Kind::Progress(Progress { back_texture: r.string()?, bar_texture: r.string()?, gap: r.i32()? }),
        "Radar" => Kind::Radar,
        "RadioButton" => {
            Kind::RadioButton(RadioButton { sysstring: r.i32()?, radio_group_id: r.i32()?, is_checked: r.i32()? })
        }
        "ScrollArea" => Kind::ScrollArea(ScrollArea { area_height: r.i32()?, children: list(r, node)? }),
        "ShortcutItemWindow" => Kind::ShortcutItemWindow(ShortcutItemWindow { always_show_outline: boolean(r)? }),
        "SliderCtrl" => Kind::SliderCtrl(SliderCtrl {
            num_of_tick: r.i32()?,
            curr_tick: r.i32()?,
            thumb_btn_width: r.i32()?,
            thumb_btn_height: r.i32()?,
            back_texture: r.string()?,
            disable_back_texture: r.string()?,
            thumb_btn_normal_texture: r.string()?,
            thumb_btn_down_texture: r.string()?,
            push_btn_width: r.i32()?,
            push_btn_height: r.i32()?,
            push_btn_auto_hit_time: r.i32()?,
            unk111: r.i32()?,
            tick_texture: r.string()?,
        }),
        "StatusBar" => Kind::StatusBar(StatusBar {
            title: r.string()?,
            texture_width: r.i32()?,
            texture_height: r.i32()?,
            fore_texture: r.string()?,
            back_texture: r.string()?,
            warn_texture: r.string()?,
            regen_texture: r.string()?,
        }),
        "StatusIconCtrl" => Kind::StatusIconCtrl(StatusIconCtrl { no_clip: boolean(r)?, no_tooltip: boolean(r)? }),
        "Tab" => Kind::Tab(Tab { tabs: list(r, tab_element)? }),
        "TextBox" => Kind::TextBox(TextBox {
            text: r.string()?,
            align: indexed(r, &TextAlign::ALL, "TextAlign")?,
            offset_y: r.i32()?,
            back_tex: r.string()?,
            font_type: r.i32()?,
            sysstring: r.i32()?,
            system_msg: r.i32()?,
            text_color: r.u32()?,
            emoticon: r.i32()?,
            autosize: r.i32()?,
        }),
        "TextListBox" => Kind::TextListBox(TextListBox {
            max_row: r.i32()?,
            show_row: r.i32()?,
            line_gap: r.i32()?,
            is_show_scroll: r.i32()?,
        }),
        "Texture" => Kind::Texture(Texture {
            file: r.string()?,
            kind: indexed(r, &TextureKind::ALL, "TextureCtrlType")?,
            layer: indexed(r, &TextureLayer::ALL, "TextureLayer")?,
            u: r.i32()?,
            v: r.i32()?,
            u_size: r.f32()?,
            v_size: r.f32()?,
            alpha: r.i32()?,
            anim_tex: r.i32()?,
        }),
        "TreeCtrl" => Kind::TreeCtrl(TreeCtrl { save_expanded_node: boolean(r)?, multi_expand: r.i32()? }),
        _ => return Err(Error::UnknownControl(at, class)),
    };
    Ok(Node { base, kind })
}

fn combo_element(r: &mut Reader<'_>) -> Result<ComboBoxElement, Error> {
    Ok(ComboBoxElement { sys_string: r.i32()?, system_msg: r.i32()?, text: r.string()?, reserved: r.i32()? })
}

fn list_element(r: &mut Reader<'_>) -> Result<ListElement, Error> {
    Ok(ListElement {
        text_string_id: r.i32()?,
        width: r.i32()?,
        unk108: boolean(r)?,
        unk109: boolean(r)?,
        unk110: boolean(r)?,
    })
}

fn tab_element(r: &mut Reader<'_>) -> Result<TabElement, Error> {
    Ok(TabElement {
        button_name: r.i32()?,
        target: r.string()?,
        width: r.i32()?,
        height: r.i32()?,
        normal_tex: r.string()?,
        pushed_tex: r.string()?,
        movable: boolean(r)?,
        gap: r.i32()?,
        tooltip: r.i32()?,
        no_highlight: r.i32()?,
    })
}

fn shortcut(r: &mut Reader<'_>) -> Result<Shortcut, Error> {
    let unk0 = r.string()?;
    let unk1 = r.string()?;
    let actions = list(r, |r| Ok(ShortcutAction { keys: ints(r)?, action: r.string()? }))?;
    Ok(Shortcut { unk0, unk1, actions })
}

fn wnd_def_pos(r: &mut Reader<'_>) -> Result<WndDefPos, Error> {
    Ok(WndDefPos {
        wnd: r.string()?,
        alignment: indexed(r, &Alignment::ALL, "Alignment")?,
        x: r.i32()?,
        y: r.i32()?,
        move_parent: boolean(r)?,
        width: r.i32()?,
        height: r.i32()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(out: &mut Vec<u8>, text: &str) {
        if text.is_empty() {
            out.push(0);
            return;
        }
        out.push((text.len() + 1) as u8);
        out.extend(text.as_bytes());
        out.push(0);
    }

    fn i(out: &mut Vec<u8>, value: i32) {
        out.extend(value.to_le_bytes());
    }

    fn base_bytes(out: &mut Vec<u8>, name: &str, size: Option<(i32, i32)>, anchor: Option<(i32, i32)>) {
        s(out, name);
        s(out, "undefined");
        i(out, -1);
        i(out, -1);
        s(out, "");
        s(out, "undefined");
        s(out, "undefined");
        i(out, 0);
        match size {
            Some((w, h)) => {
                i(out, 1);
                i(out, 1);
                i(out, w);
                i(out, h);
            }
            None => i(out, 0),
        }
        match anchor {
            Some((x, y)) => {
                i(out, 1);
                i(out, 1);
                i(out, 1);
                s(out, "");
                i(out, x);
                i(out, y);
            }
            None => i(out, 0),
        }
        i(out, -1);
        i(out, -1);
        i(out, 0);
        s(out, "undefined");
        i(out, -9999);
    }

    fn window_tail(out: &mut Vec<u8>, children: usize) {
        for _ in 0..4 {
            s(out, "undefined");
        }
        for v in 0..16 {
            i(out, v);
        }
        out.extend(1.5f32.to_le_bytes());
        out.extend((-9999f32).to_le_bytes());
        for v in 0..7 {
            i(out, v);
        }
        s(out, "undefined");
        for v in 0..6 {
            i(out, v);
        }
        s(out, "");
        i(out, 7);
        i(out, 8);
        s(out, "Game");
        i(out, children as i32);
    }

    fn sample() -> Vec<u8> {
        let mut out = Vec::new();
        i(&mut out, 1);
        base_bytes(&mut out, "RecipeBookWnd", Some((256, 320)), Some((176, 0)));
        window_tail(&mut out, 2);
        s(&mut out, "Button");
        base_bytes(&mut out, "btnTrash", Some((24, 24)), None);
        s(&mut out, "Inventory_trash");
        s(&mut out, "Inventory_trash_Drag");
        s(&mut out, "undefined");
        s(&mut out, "undefined");
        i(&mut out, -9999);
        i(&mut out, -1);
        i(&mut out, -1);
        i(&mut out, -9999);
        s(&mut out, "ScrollArea");
        base_bytes(&mut out, "area", None, None);
        i(&mut out, 100);
        i(&mut out, 1);
        s(&mut out, "TextBox");
        base_bytes(&mut out, "txtCount", None, None);
        s(&mut out, "(0/0)");
        i(&mut out, 2);
        i(&mut out, 0);
        s(&mut out, "");
        i(&mut out, 0);
        i(&mut out, 0);
        i(&mut out, 0);
        i(&mut out, 0x11223344);
        i(&mut out, 0);
        i(&mut out, 0);
        i(&mut out, 1);
        s(&mut out, "DefaultShortcut");
        s(&mut out, "");
        i(&mut out, 1);
        i(&mut out, 0x12);
        i(&mut out, 0);
        i(&mut out, 0);
        s(&mut out, "ToggleRun");
        i(&mut out, 1);
        i(&mut out, 1);
        s(&mut out, "RecipeBookWnd");
        i(&mut out, 5);
        i(&mut out, 10);
        i(&mut out, 20);
        i(&mut out, 0);
        i(&mut out, 256);
        i(&mut out, 320);
        out
    }

    #[test]
    fn reads_a_window_with_nested_children_shortcuts_and_default_positions() {
        let xdat = read(&sample()).unwrap();
        assert_eq!(xdat.windows.len(), 1);
        let window = &xdat.windows[0];
        assert_eq!(window.base.name, "RecipeBookWnd");
        assert_eq!(window.base.size.as_ref().map(|size| (size.width, size.height)), Some((256, 320)));
        let anchor = window.base.anchor.as_ref().unwrap();
        assert_eq!((anchor.parent, anchor.this, anchor.x, anchor.y), (Alignment::TopLeft, Alignment::TopLeft, 176, 0));
        assert_eq!(window.resize_frame_rect, [14, 15, 1.5f32.to_bits() as i32, (-9999f32).to_bits() as i32]);
        assert_eq!((window.title, window.draggable, window.frame_size), (7, 12, 9));
        assert_eq!(window.working_configuration, "Game");
        assert_eq!(window.icon_name, "");
        assert_eq!(window.children.len(), 2);
        let Kind::Button(button) = &window.children[0].kind else { panic!("{:?}", window.children[0].kind) };
        assert_eq!(button.pushed_tex, "Inventory_trash_Drag");
        let text = window.find("txtCount").unwrap();
        let Kind::TextBox(text) = &text.kind else { panic!("{:?}", text.kind) };
        assert_eq!((text.text.as_str(), text.align, text.text_color), ("(0/0)", TextAlign::Center, 0x11223344));
        assert_eq!(window.nodes().len(), 3);
        assert_eq!(xdat.shortcuts[0].actions[0], ShortcutAction { keys: [0x12, 0, 0], action: "ToggleRun".into() });
        assert_eq!(xdat.default_positions.len(), 1);
        assert_eq!(xdat.default_positions[0][0].alignment, Alignment::Center);
        assert_eq!(xdat.wnd_def_pos().next().unwrap().width, 256);
        assert!(xdat.tail.is_empty());
    }

    #[test]
    fn keeps_a_tail_of_twenty_or_forty_bytes_and_refuses_any_other_leftover() {
        let mut data = sample();
        data.extend([0x90; 20]);
        assert_eq!(read(&data).unwrap().tail, [0x90; 20]);
        data.extend([7; 20]);
        assert_eq!(read(&data).unwrap().tail.len(), 40);
        data.push(0);
        assert!(matches!(read(&data), Err(Error::Trailing(41, _))));
    }

    #[test]
    fn unknown_control_class_and_bad_enum_index_are_errors() {
        let mut out = Vec::new();
        i(&mut out, 1);
        base_bytes(&mut out, "W", None, None);
        window_tail(&mut out, 1);
        s(&mut out, "AutoPotion");
        assert!(matches!(read(&out), Err(Error::UnknownControl(_, class)) if class == "AutoPotion"));
        let mut out = Vec::new();
        i(&mut out, 1);
        base_bytes(&mut out, "W", None, None);
        window_tail(&mut out, 1);
        s(&mut out, "Texture");
        base_bytes(&mut out, "t", None, None);
        s(&mut out, "L2UI.x");
        i(&mut out, 6);
        assert!(matches!(read(&out), Err(Error::BadIndex(_, 6, "TextureCtrlType"))));
    }

    #[test]
    fn compact_string_length_counts_the_terminator() {
        let mut out = Vec::new();
        s(&mut out, "ab");
        assert_eq!(out, [3, b'a', b'b', 0]);
        assert_eq!(Reader::at(&out, 0).string().unwrap(), "ab");
    }
}
