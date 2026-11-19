use crate::{DatReader, Error};

pub const SENTINEL: i32 = -9999;
pub const SENTINEL_BITS: u32 = 0xC61C_3C00;

#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
    pub shortcut_sets: Vec<ShortcutSet>,
    pub windows: Vec<WindowDef>,
    pub layouts: Vec<DefaultLayout>,
    pub fonts: FontTable,
    pub palette: Vec<PaletteEntry>,
    pub trailer: [i32; 5],
}

impl Interface {
    pub fn window(&self, name: &str) -> Option<&WindowDef> {
        self.windows.iter().find(|window| window.head.name == name)
    }

    pub fn control_count(&self) -> usize {
        self.windows.iter().map(WindowDef::control_count).sum()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutSet {
    pub name: String,
    pub state: String,
    pub mode: u32,
    pub keys: Vec<ShortcutKey>,
    pub trailer: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutKey {
    pub key: u32,
    pub modifier_a: u32,
    pub modifier_b: u32,
    pub command: String,
    pub edge: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Head {
    pub name: String,
    pub template: String,
    pub always_on_top: i32,
    pub always_on_back: i32,
    pub parent: String,
    pub pre_order: String,
    pub post_order: String,
    pub anchored: i32,
    pub super_child: i32,
    pub size: Option<Size>,
    pub anchor: Option<Anchor>,
    pub styles: Vec<Style>,
    pub virtual_flag: i32,
    pub unk24: i32,
    pub tooltip_type: String,
    pub tooltip_text: i32,
    pub tooltip_appearance_time: i32,
    pub scroll_bar_offset_x: i32,
    pub scroll_bar_offset_y: i32,
    pub scroll_bar_offset_height: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Fixed { width: i32, height: i32 },
    Scaled { tag: String, scale: [f32; 2], extent: [i32; 2] },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Anchor {
    pub kind: i32,
    pub mode: [i32; 2],
    pub target: String,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Style {
    pub style_name: String,
    pub font_name: String,
    pub font_color: u32,
    pub font_line: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowDef {
    pub head: Head,
    pub part: WindowPart,
    pub children: Vec<Control>,
}

impl WindowDef {
    pub fn control_count(&self) -> usize {
        self.children.iter().map(Control::count).sum()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WindowPart {
    pub window_parent: String,
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
    pub exitbutton: i32,
    pub draggable: i32,
    pub resize_frame_direction: i32,
    pub resize_frame_x: f32,
    pub resize_frame_y: f32,
    pub resize_frame_width: f32,
    pub resize_frame_height: f32,
    pub resize_max_increase: i32,
    pub drawer_direction: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub direction_fixed: i32,
    pub owner_window: String,
    pub show_anim_type: i32,
    pub hide_anim_type: i32,
    pub show_anim_direction: i32,
    pub show_anim_seconds: i32,
    pub hide_anim_direction: i32,
    pub hide_anim_seconds: i32,
    pub icon_name: String,
    pub tooltip_idx: i32,
    pub hook_key_input: i32,
    pub working_configuration: String,
    pub left_texture_name: String,
    pub mid_texture_name: String,
    pub right_texture_name: String,
    pub minimize_btn_texture_normal: String,
    pub minimize_btn_texture_pushed: String,
    pub close_btn_texture_normal: String,
    pub close_btn_texture_pushed: String,
    pub left_back_texture_width: i32,
    pub mid_back_texture_width: i32,
    pub additional_state: Vec<String>,
    pub use_parent_clip_rect: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Control {
    pub kind: String,
    pub head: Head,
    pub props: Props,
    pub children: Vec<Control>,
}

impl Control {
    pub fn count(&self) -> usize {
        1 + self.children.iter().map(Control::count).sum::<usize>()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Props {
    Window(Box<WindowPart>),
    Plain,
    Button(ButtonProps),
    StatusIcon([i32; 2]),
    Texture(TextureProps),
    TextBox(TextBoxProps),
    EditBox([i32; 7]),
    CheckBox(CheckBoxProps),
    ScrollArea { content: i32, child_count: i32 },
    TreeCtrl([i32; 2]),
    MultiEdit([i32; 2]),
    ShortcutItemWindow([i32; 2]),
    TextListBox([i32; 4]),
    RadioButton([i32; 3]),
    ListBox([i32; 3]),
    MinimapCtrl([i32; 6]),
    HtmlCtrl { source: String },
    BarCtrl { textures: [String; 6], tail: [i32; 2] },
    Tab(Vec<TabPage>),
    ComboBox(Vec<ComboItem>),
    ListCtrl { head: [i32; 5], columns: Vec<[i32; 5]> },
    StatusBar(Box<GaugeProps>),
    InvenWeight(Box<GaugeProps>),
    ChatWindow([i32; 2]),
    ItemWindow(Box<ItemWindowProps>),
    EffectButton { count: i32, textures: [String; 5] },
    FishViewport { textures: [String; 8] },
    CharacterViewport { scale: f32, values: [i32; 8], mask: String, tail: i32 },
    MoviePlayer { path: String, value: i32 },
    SliderCtrl(SliderProps),
    Progress(ProgressProps),
    RadarMap(RadarMapProps),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ButtonProps {
    pub textures: [String; 4],
    pub button_name_id: i32,
    pub caption_text: String,
    pub no_highlight: i32,
    pub default_sound_on: i32,
    pub disable_time: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureProps {
    pub texture: String,
    pub alpha_mask: String,
    pub values: [i32; 8],
    pub auto_rotate: String,
    pub tail: [i32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextBoxProps {
    pub text: String,
    pub text_align: i32,
    pub text_valign: i32,
    pub font_type: i32,
    pub back_tex: String,
    pub sysstring: i32,
    pub system_msg: i32,
    pub text_color: i32,
    pub emoticon: i32,
    pub auto_size: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CheckBoxProps {
    pub text_id: i32,
    pub caption: String,
    pub values: [i32; 3],
    pub textures: [String; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct TabPage {
    pub id: i32,
    pub label: String,
    pub window: String,
    pub extent: [i32; 2],
    pub textures: [String; 2],
    pub values: [i32; 4],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComboItem {
    pub text_id: i32,
    pub sentinel: i32,
    pub text: String,
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GaugeProps {
    pub name: String,
    pub values: [i32; 2],
    pub textures: [String; 13],
    pub tail: [i32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemWindowProps {
    pub scroll_type: String,
    pub head: [i32; 11],
    pub groups: [ItemGroup; 3],
    pub slot: String,
    pub values: [i32; 6],
    pub frame: String,
    pub frame_down: String,
    pub fill: [i32; 4],
    pub extras: Vec<ItemExtra>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemGroup {
    pub texture: String,
    pub values: [i32; 2],
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemExtra {
    pub values: [i32; 3],
    pub texture: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SliderProps {
    pub head: [i32; 4],
    pub textures: [String; 4],
    pub values: [i32; 4],
    pub mark: String,
    pub tail: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressProps {
    pub name: String,
    pub textures: [String; 6],
    pub tail: [i32; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadarMapProps {
    pub map: String,
    pub scale: f32,
    pub icons: Vec<RadarIcon>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RadarIcon {
    pub name: String,
    pub textures: [String; 3],
    pub values: [i32; 3],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefaultLayout {
    pub window: String,
    pub values: [i32; 6],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontTable {
    pub head: i32,
    pub default_name: String,
    pub default_color: u32,
    pub fonts: Vec<FontDef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontDef {
    pub name: String,
    pub file: String,
    pub charset: String,
    pub values: [i32; 10],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaletteEntry {
    pub index: i32,
    pub color: u32,
}

pub fn read(body: &[u8]) -> Result<Vec<WindowDef>, Error> {
    Ok(read_interface(body)?.windows)
}

pub fn read_interface(body: &[u8]) -> Result<Interface, Error> {
    let mut cursor = Cursor::new(body);
    let shortcut_sets = cursor.shortcut_sets()?;
    let window_count = cursor.u32()? as usize;
    let mut windows = Vec::new();
    for _ in 0..window_count {
        windows.push(cursor.window()?);
    }
    let layout_head = cursor.i32()?;
    if layout_head != 1 {
        return Err(cursor.invalid());
    }
    let layout_count = cursor.u32()? as usize;
    let mut layouts = Vec::new();
    for _ in 0..layout_count {
        layouts.push(DefaultLayout { window: cursor.s()?, values: cursor.ints()? });
    }
    let fonts = cursor.fonts()?;
    let zero = cursor.i32()?;
    if zero != 0 {
        return Err(cursor.invalid());
    }
    let palette_count = cursor.u32()? as usize;
    let mut palette = Vec::new();
    for _ in 0..palette_count {
        palette.push(PaletteEntry { index: cursor.i32()?, color: cursor.u32()? });
    }
    let trailer = cursor.ints()?;
    if cursor.reader.remaining() != 0 {
        return Err(cursor.invalid());
    }
    Ok(Interface { shortcut_sets, windows, layouts, fonts, palette, trailer })
}

struct Cursor<'a> {
    reader: DatReader<'a>,
    len: usize,
}

impl<'a> Cursor<'a> {
    fn new(body: &'a [u8]) -> Self {
        Self { reader: DatReader::new(body), len: body.len() }
    }

    fn at(&self) -> usize {
        self.len - self.reader.remaining()
    }

    fn invalid(&self) -> Error {
        Error::InvalidString(self.at())
    }

    fn u32(&mut self) -> Result<u32, Error> {
        self.reader.u32()
    }

    fn i32(&mut self) -> Result<i32, Error> {
        self.reader.i32()
    }

    fn f32(&mut self) -> Result<f32, Error> {
        self.reader.f32()
    }

    fn s(&mut self) -> Result<String, Error> {
        self.reader.ascf()
    }

    fn ints<const N: usize>(&mut self) -> Result<[i32; N], Error> {
        let mut out = [0; N];
        for slot in &mut out {
            *slot = self.i32()?;
        }
        Ok(out)
    }

    fn strings<const N: usize>(&mut self) -> Result<[String; N], Error> {
        let mut out = std::array::from_fn(|_| String::new());
        for slot in &mut out {
            *slot = self.s()?;
        }
        Ok(out)
    }

    fn shortcut_sets(&mut self) -> Result<Vec<ShortcutSet>, Error> {
        let count = self.u32()? as usize;
        let mut sets = Vec::new();
        for _ in 0..count {
            let name = self.s()?;
            let state = self.s()?;
            let mode = self.u32()?;
            let key_count = self.u32()? as usize;
            let mut keys = Vec::new();
            for _ in 0..key_count {
                keys.push(ShortcutKey {
                    key: self.u32()?,
                    modifier_a: self.u32()?,
                    modifier_b: self.u32()?,
                    command: self.s()?,
                    edge: self.u32()?,
                });
            }
            let trailer = self.u32()?;
            if trailer != 0 {
                return Err(self.invalid());
            }
            sets.push(ShortcutSet { name, state, mode, keys, trailer });
        }
        Ok(sets)
    }

    fn head(&mut self, name: String) -> Result<Head, Error> {
        let template = self.s()?;
        let always_on_top = self.i32()?;
        let always_on_back = self.i32()?;
        let parent = self.s()?;
        let pre_order = self.s()?;
        let post_order = self.s()?;
        let anchored = self.i32()?;
        let super_child = self.i32()?;
        let size = if self.i32()? == 0 {
            None
        } else if self.i32()? == 1 {
            Some(Size::Fixed { width: self.i32()?, height: self.i32()? })
        } else {
            let tag = self.s()?;
            let scale = [self.f32()?, self.f32()?];
            Some(Size::Scaled { tag, scale, extent: self.ints()? })
        };
        let kind = self.i32()?;
        let anchor = if kind == 0 {
            None
        } else {
            let mode = self.ints()?;
            let target = self.s()?;
            Some(Anchor { kind, mode, target, x: self.f32()?, y: self.f32()? })
        };
        let style_count = self.i32()?;
        let mut styles = Vec::new();
        for _ in 0..style_count {
            styles.push(Style {
                style_name: self.s()?,
                font_name: self.s()?,
                font_color: self.u32()?,
                font_line: self.i32()?,
            });
        }
        let virtual_flag = self.i32()?;
        let unk24 = self.i32()?;
        let tooltip_type = self.s()?;
        let tooltip_text = self.i32()?;
        let tooltip_appearance_time = self.i32()?;
        let scroll_bar_offset_x = self.i32()?;
        let scroll_bar_offset_y = self.i32()?;
        let scroll_bar_offset_height = self.i32()?;
        Ok(Head {
            name,
            template,
            always_on_top,
            always_on_back,
            parent,
            pre_order,
            post_order,
            anchored,
            super_child,
            size,
            anchor,
            styles,
            virtual_flag,
            unk24,
            tooltip_type,
            tooltip_text,
            tooltip_appearance_time,
            scroll_bar_offset_x,
            scroll_bar_offset_y,
            scroll_bar_offset_height,
        })
    }

    fn window_part(&mut self) -> Result<WindowPart, Error> {
        let window_parent = self.s()?;
        let back_tex = self.s()?;
        let script = self.s()?;
        let state = self.s()?;
        let frame = self.i32()?;
        let iconable = self.i32()?;
        let stuckable = self.i32()?;
        let hidden = self.i32()?;
        let always_full_alpha = self.i32()?;
        let save_position = self.i32()?;
        let save_size = self.i32()?;
        let title = self.i32()?;
        let resize_frame = self.i32()?;
        let frame_size = self.i32()?;
        let frame_direction = self.i32()?;
        let exitbutton = self.i32()?;
        let draggable = self.i32()?;
        let resize_frame_direction = self.i32()?;
        let resize_frame_x = self.f32()?;
        let resize_frame_y = self.f32()?;
        let resize_frame_width = self.f32()?;
        let resize_frame_height = self.f32()?;
        let resize_max_increase = self.i32()?;
        let drawer_direction = self.i32()?;
        let offset_x = self.i32()?;
        let offset_y = self.i32()?;
        let direction_fixed = self.i32()?;
        let owner_window = self.s()?;
        let show_anim_type = self.i32()?;
        let hide_anim_type = self.i32()?;
        let show_anim_direction = self.i32()?;
        let show_anim_seconds = self.i32()?;
        let hide_anim_direction = self.i32()?;
        let hide_anim_seconds = self.i32()?;
        let icon_name = self.s()?;
        let tooltip_idx = self.i32()?;
        let hook_key_input = self.i32()?;
        let working_configuration = self.s()?;
        let left_texture_name = self.s()?;
        let mid_texture_name = self.s()?;
        let right_texture_name = self.s()?;
        let minimize_btn_texture_normal = self.s()?;
        let minimize_btn_texture_pushed = self.s()?;
        let close_btn_texture_normal = self.s()?;
        let close_btn_texture_pushed = self.s()?;
        let left_back_texture_width = self.i32()?;
        let mid_back_texture_width = self.i32()?;
        let additional_state_count = self.reader.count()?;
        let mut additional_state = Vec::new();
        for _ in 0..additional_state_count {
            additional_state.push(self.s()?);
        }
        let use_parent_clip_rect = self.i32()?;
        Ok(WindowPart {
            window_parent,
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
            exitbutton,
            draggable,
            resize_frame_direction,
            resize_frame_x,
            resize_frame_y,
            resize_frame_width,
            resize_frame_height,
            resize_max_increase,
            drawer_direction,
            offset_x,
            offset_y,
            direction_fixed,
            owner_window,
            show_anim_type,
            hide_anim_type,
            show_anim_direction,
            show_anim_seconds,
            hide_anim_direction,
            hide_anim_seconds,
            icon_name,
            tooltip_idx,
            hook_key_input,
            working_configuration,
            left_texture_name,
            mid_texture_name,
            right_texture_name,
            minimize_btn_texture_normal,
            minimize_btn_texture_pushed,
            close_btn_texture_normal,
            close_btn_texture_pushed,
            left_back_texture_width,
            mid_back_texture_width,
            additional_state,
            use_parent_clip_rect,
        })
    }

    fn children(&mut self, count: usize) -> Result<Vec<Control>, Error> {
        let mut out = Vec::new();
        for _ in 0..count {
            out.push(self.control()?);
        }
        Ok(out)
    }

    fn window(&mut self) -> Result<WindowDef, Error> {
        let name = self.s()?;
        let head = self.head(name)?;
        let part = self.window_part()?;
        let count = self.u32()? as usize;
        let children = self.children(count)?;
        Ok(WindowDef { head, part, children })
    }

    fn control(&mut self) -> Result<Control, Error> {
        let kind = self.s()?;
        let name = self.s()?;
        let head = self.head(name)?;
        if kind == "Window" {
            let part = self.window_part()?;
            let count = self.u32()? as usize;
            let children = self.children(count)?;
            return Ok(Control { kind, head, props: Props::Window(Box::new(part)), children });
        }
        let props = self.props(&kind)?;
        let children = match &props {
            Props::ScrollArea { child_count, .. } => {
                let count = usize::try_from(*child_count).map_err(|_| self.invalid())?;
                self.children(count)?
            }
            _ => Vec::new(),
        };
        Ok(Control { kind, head, props, children })
    }

    fn gauge(&mut self) -> Result<GaugeProps, Error> {
        Ok(GaugeProps {
            name: self.s()?,
            values: self.ints()?,
            textures: self.strings()?,
            tail: self.ints()?,
        })
    }

    fn props(&mut self, kind: &str) -> Result<Props, Error> {
        Ok(match kind {
            "MultiSellItemInfo" | "MultiSellNeededItem" | "NameCtrl" | "PropertyController" => Props::Plain,
            "Button" => Props::Button(ButtonProps {
                textures: self.strings()?,
                button_name_id: self.i32()?,
                caption_text: self.s()?,
                no_highlight: self.i32()?,
                default_sound_on: self.i32()?,
                disable_time: self.i32()?,
            }),
            "StatusIconCtrl" => Props::StatusIcon(self.ints()?),
            "Texture" => Props::Texture(TextureProps {
                texture: self.s()?,
                alpha_mask: self.s()?,
                values: self.ints()?,
                auto_rotate: self.s()?,
                tail: self.ints()?,
            }),
            "TextBox" => Props::TextBox(TextBoxProps {
                text: self.s()?,
                text_align: self.i32()?,
                text_valign: self.i32()?,
                font_type: self.i32()?,
                back_tex: self.s()?,
                sysstring: self.i32()?,
                system_msg: self.i32()?,
                text_color: self.i32()?,
                emoticon: self.i32()?,
                auto_size: self.i32()?,
            }),
            "EditBox" => Props::EditBox(self.ints()?),
            "CheckBox" => Props::CheckBox(CheckBoxProps {
                text_id: self.i32()?,
                caption: self.s()?,
                values: self.ints()?,
                textures: self.strings()?,
            }),
            "ScrollArea" => Props::ScrollArea { content: self.i32()?, child_count: self.i32()? },
            "TreeCtrl" => Props::TreeCtrl(self.ints()?),
            "MultiEdit" => Props::MultiEdit(self.ints()?),
            "ShortcutItemWindow" => Props::ShortcutItemWindow(self.ints()?),
            "TextListBox" => Props::TextListBox(self.ints()?),
            "RadioButton" => Props::RadioButton(self.ints()?),
            "ListBox" => Props::ListBox(self.ints()?),
            "MinimapCtrl" => Props::MinimapCtrl(self.ints()?),
            "HtmlCtrl" => Props::HtmlCtrl { source: self.s()? },
            "BarCtrl" => Props::BarCtrl { textures: self.strings()?, tail: self.ints()? },
            "Tab" => {
                let count = self.u32()? as usize;
                let mut pages = Vec::new();
                for _ in 0..count {
                    pages.push(TabPage {
                        id: self.i32()?,
                        label: self.s()?,
                        window: self.s()?,
                        extent: self.ints()?,
                        textures: self.strings()?,
                        values: self.ints()?,
                    });
                }
                Props::Tab(pages)
            }
            "ComboBox" => {
                let count = self.u32()? as usize;
                let mut items = Vec::new();
                for _ in 0..count {
                    items.push(ComboItem {
                        text_id: self.i32()?,
                        sentinel: self.i32()?,
                        text: self.s()?,
                        value: self.i32()?,
                    });
                }
                Props::ComboBox(items)
            }
            "ListCtrl" => {
                let head = self.ints()?;
                let count = self.u32()? as usize;
                let mut columns = Vec::new();
                for _ in 0..count {
                    columns.push(self.ints()?);
                }
                Props::ListCtrl { head, columns }
            }
            "StatusBar" => Props::StatusBar(Box::new(self.gauge()?)),
            "InvenWeight" => Props::InvenWeight(Box::new(self.gauge()?)),
            "ChatWindow" => Props::ChatWindow(self.ints()?),
            "ItemWindow" => Props::ItemWindow(Box::new(self.item_window()?)),
            "EffectButton" => Props::EffectButton { count: self.i32()?, textures: self.strings()? },
            "FishViewportWindow" => Props::FishViewport { textures: self.strings()? },
            "CharacterViewportWindow" => Props::CharacterViewport {
                scale: self.f32()?,
                values: self.ints()?,
                mask: self.s()?,
                tail: self.i32()?,
            },
            "MoviePlayerCtrl" => Props::MoviePlayer { path: self.s()?, value: self.i32()? },
            "SliderCtrl" => Props::SliderCtrl(SliderProps {
                head: self.ints()?,
                textures: self.strings()?,
                values: self.ints()?,
                mark: self.s()?,
                tail: self.i32()?,
            }),
            "Progress" => Props::Progress(ProgressProps {
                name: self.s()?,
                textures: self.strings()?,
                tail: self.ints()?,
            }),
            "RadarMapCtrl" => {
                let map = self.s()?;
                let scale = self.f32()?;
                let count = self.u32()? as usize;
                let mut icons = Vec::new();
                for _ in 0..count {
                    icons.push(RadarIcon {
                        name: self.s()?,
                        textures: self.strings()?,
                        values: self.ints()?,
                    });
                }
                Props::RadarMap(RadarMapProps { map, scale, icons })
            }
            _ => return Err(self.invalid()),
        })
    }

    fn item_window(&mut self) -> Result<ItemWindowProps, Error> {
        let scroll_type = self.s()?;
        let head = self.ints()?;
        let mut groups = Vec::new();
        for _ in 0..3 {
            groups.push(ItemGroup { texture: self.s()?, values: self.ints()? });
        }
        let groups: [ItemGroup; 3] = groups.try_into().map_err(|_| self.invalid())?;
        let slot = self.s()?;
        let values = self.ints()?;
        let frame = self.s()?;
        let frame_down = self.s()?;
        let fill = self.ints()?;
        let count = self.u32()? as usize;
        let mut extras = Vec::new();
        for _ in 0..count {
            extras.push(ItemExtra { values: self.ints()?, texture: self.s()? });
        }
        Ok(ItemWindowProps { scroll_type, head, groups, slot, values, frame, frame_down, fill, extras })
    }

    fn fonts(&mut self) -> Result<FontTable, Error> {
        let head = self.i32()?;
        let default_name = self.s()?;
        let default_color = self.u32()?;
        let count = self.u32()? as usize;
        let mut fonts = Vec::new();
        for _ in 0..count {
            fonts.push(FontDef {
                name: self.s()?,
                file: self.s()?,
                charset: self.s()?,
                values: self.ints()?,
            });
        }
        Ok(FontTable { head, default_name, default_color, fonts })
    }
}

pub fn dump_window(window: &WindowDef) -> String {
    let mut out = String::new();
    out.push_str(&format!("Window {}\n", window.head.name));
    dump_head(&mut out, &window.head, 1);
    dump_part(&mut out, &window.part, 1);
    for child in &window.children {
        dump_control(&mut out, child, 1);
    }
    out
}

fn pad(depth: usize) -> String {
    "  ".repeat(depth)
}

fn dump_head(out: &mut String, head: &Head, depth: usize) {
    let pad = pad(depth);
    match &head.size {
        Some(Size::Fixed { width, height }) => out.push_str(&format!("{pad}size {width}x{height}\n")),
        Some(Size::Scaled { tag, scale, extent }) => {
            out.push_str(&format!("{pad}size scaled tag={tag:?} scale={scale:?} extent={extent:?}\n"))
        }
        None => {}
    }
    if let Some(anchor) = &head.anchor {
        out.push_str(&format!(
            "{pad}anchor kind={} mode={:?} target={:?} at ({}, {})\n",
            anchor.kind, anchor.mode, anchor.target, anchor.x, anchor.y
        ));
    }
    if head.template != "undefined" {
        out.push_str(&format!("{pad}template {}\n", head.template));
    }
    if !head.parent.is_empty() {
        out.push_str(&format!("{pad}parent {}\n", head.parent));
    }
    for style in &head.styles {
        out.push_str(&format!(
            "{pad}style style_name={:?} font_name={:?} font_color={:#010x} font_line={}\n",
            style.style_name, style.font_name, style.font_color, style.font_line
        ));
    }
    if head.tooltip_type != "undefined" {
        out.push_str(&format!("{pad}tooltip_type {:?}\n", head.tooltip_type));
    }
}

fn dump_part(out: &mut String, part: &WindowPart, depth: usize) {
    let pad = pad(depth);
    out.push_str(&format!(
        "{pad}state {:?} back_tex {:?} script {:?} window_parent {:?} working_configuration {:?}\n",
        part.state, part.back_tex, part.script, part.window_parent, part.working_configuration
    ));
    if !part.additional_state.is_empty() {
        out.push_str(&format!("{pad}additional_state {:?}\n", part.additional_state));
    }
}

fn dump_props(out: &mut String, props: &Props, depth: usize) {
    let pad = pad(depth);
    match props {
        Props::Button(b) => out.push_str(&format!(
            "{pad}textures {:?} button_name_id {} caption_text {:?}\n",
            b.textures, b.button_name_id, b.caption_text
        )),
        Props::Texture(t) => {
            out.push_str(&format!("{pad}texture {:?} auto_rotate {:?} values {:?}\n", t.texture, t.auto_rotate, t.values))
        }
        Props::TextBox(t) => out.push_str(&format!(
            "{pad}text {:?} back_tex {:?} text_align={} text_valign={} font_type={} sysstring={}\n",
            t.text, t.back_tex, t.text_align, t.text_valign, t.font_type, t.sysstring
        )),
        Props::CheckBox(c) => out.push_str(&format!("{pad}text_id {} textures {:?}\n", c.text_id, c.textures)),
        Props::ItemWindow(i) => out.push_str(&format!(
            "{pad}scroll_type {:?} head {:?} values {:?} fill {:?} groups {:?} slot {:?} frame {:?}/{:?} extras {:?}\n",
            i.scroll_type, i.head, i.values, i.fill, i.groups, i.slot, i.frame, i.frame_down, i.extras
        )),
        Props::StatusBar(g) | Props::InvenWeight(g) => out.push_str(&format!("{pad}gauge {:?} {:?}\n", g.name, g.textures)),
        Props::BarCtrl { textures, .. } => out.push_str(&format!("{pad}textures {textures:?}\n")),
        Props::Tab(pages) => {
            for page in pages {
                out.push_str(&format!("{pad}page id={} window={:?} textures={:?}\n", page.id, page.window, page.textures));
            }
        }
        Props::ComboBox(items) => {
            for item in items {
                out.push_str(&format!("{pad}item text_id={} text={:?}\n", item.text_id, item.text));
            }
        }
        Props::ListCtrl { head, columns } => out.push_str(&format!("{pad}head {head:?} columns {columns:?}\n")),
        Props::HtmlCtrl { source } => out.push_str(&format!("{pad}source {source:?}\n")),
        Props::SliderCtrl(s) => out.push_str(&format!("{pad}textures {:?} mark {:?}\n", s.textures, s.mark)),
        _ => {}
    }
}

fn dump_control(out: &mut String, control: &Control, depth: usize) {
    let pad = pad(depth);
    out.push_str(&format!("{pad}{} {}\n", control.kind, control.head.name));
    dump_head(out, &control.head, depth + 1);
    dump_props(out, &control.props, depth + 1);
    if let Props::Window(part) = &control.props {
        dump_part(out, part, depth + 1);
    }
    for child in &control.children {
        dump_control(out, child, depth + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn u(value: u32) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    fn i(value: i32) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    fn f(value: f32) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    fn s(text: &str) -> Vec<u8> {
        if text.is_empty() {
            return vec![0];
        }
        let mut out = vec![(text.len() + 1) as u8];
        out.extend(text.bytes());
        out.push(0);
        out
    }

    fn wide(text: &str) -> Vec<u8> {
        let mut out = vec![0x80 | (text.chars().count() + 1) as u8];
        out.extend(text.encode_utf16().chain([0]).flat_map(u16::to_le_bytes));
        out
    }

    fn cat(parts: Vec<Vec<u8>>) -> Vec<u8> {
        parts.concat()
    }

    fn head(name: &str, parent: &str, width: i32, height: i32, x: f32, y: f32, styles: Vec<Vec<u8>>) -> Vec<u8> {
        let mut out = cat(vec![
            s(name),
            s("undefined"),
            i(-1),
            i(-1),
            s(parent),
            s("undefined"),
            s("undefined"),
            i(1),
            i(-1),
            i(1),
            i(1),
            i(width),
            i(height),
            i(1),
            i(1),
            i(1),
            s(""),
            f(x),
            f(y),
            i(styles.len() as i32),
        ]);
        out.extend(styles.concat());
        out.extend(cat(vec![i(1), i(0), s("undefined")]));
        out.extend(cat(vec![i(SENTINEL); 5]));
        out
    }

    fn part(state: &str, observers: &[&str]) -> Vec<u8> {
        let mut out = cat(vec![s("undefined"), s("bg.tex"), s("Wnd"), s(state)]);
        out.extend(cat((0..14).map(i).collect()));
        out.extend(cat(vec![u(SENTINEL_BITS); 4]));
        out.extend(cat((0..5).map(i).collect()));
        out.extend(s(""));
        out.extend(cat((0..6).map(i).collect()));
        out.extend(cat(vec![s("undefined"), i(SENTINEL), i(-1), s("Game")]));
        out.extend(cat(vec![s("undefined"); 7]));
        out.extend(cat(vec![i(SENTINEL), i(SENTINEL), vec![observers.len() as u8]]));
        for name in observers {
            out.extend(s(name));
        }
        out.extend(cat(vec![i(-1)]));
        out
    }

    fn fixture() -> Vec<u8> {
        let mut out = Vec::new();
        out.extend(u(1));
        out.extend(cat(vec![s("GamingStateShortcut"), s("GamingState"), u(1), u(1)]));
        out.extend(cat(vec![u(67), u(18), u(0), s("ShowWindow Name=ActionWnd"), u(1), u(0)]));
        out.extend(u(1));
        out.extend(head("TestWnd", "", 100, 80, 4.0, 8.0, vec![]));
        out.extend(part("GamingState", &["ObsState"]));
        out.extend(u(3));
        let mut text = s("TextBox");
        text.extend(head("txtTitle", "TestWnd", 50, 12, 1.5, 2.5, vec![cat(vec![s(""), s("HS12"), u(0xFF00_0000), i(0)])]));
        text.extend(cat(vec![wide("각ab"), i(1), i(0), i(-1), s("undefined"), i(7), i(SENTINEL), u(0xFFDC_DCDC), i(1), i(1)]));
        out.extend(text);
        let mut area = s("ScrollArea");
        area.extend(head("scroll", "TestWnd", 60, 30, 0.0, 0.0, vec![]));
        area.extend(cat(vec![i(200), i(1)]));
        area.extend(s("Button"));
        area.extend(head("btnInner", "scroll", 20, 10, 0.0, 0.0, vec![]));
        area.extend(cat(vec![s("n.tex"), s("p.tex"), s(""), s(""), i(140), s("undefined"), i(-1), i(-1), i(SENTINEL)]));
        out.extend(area);
        let mut inner = s("Window");
        inner.extend(head("SubWnd", "TestWnd", 10, 10, 0.0, 0.0, vec![]));
        inner.extend(part("GamingState", &[]));
        inner.extend(u(0));
        out.extend(inner);
        out.extend(cat(vec![i(1), u(1)]));
        out.extend(cat(vec![s("TestWnd"), i(1), i(10), i(20), i(0), i(SENTINEL), i(SENTINEL)]));
        out.extend(cat(vec![i(1), s("GameDefault"), u(0xFFDC_DCDC), u(1)]));
        out.extend(cat(vec![s("GameDefault"), s("tahoma.ttf"), s("windows"), cat((0..10).map(i).collect())]));
        out.extend(cat(vec![i(0), u(2), i(0), u(0xFFDC_DCDC), i(1), u(0xFFFF_7200)]));
        out.extend(cat(vec![i(0), i(67), i(216), i(-1733207327), i(0)]));
        out
    }

    #[test]
    fn reads_every_section_of_a_synthetic_file() {
        let data = fixture();
        let interface = read_interface(&data).unwrap();
        assert_eq!(interface.shortcut_sets.len(), 1);
        assert_eq!(interface.shortcut_sets[0].keys[0].command, "ShowWindow Name=ActionWnd");
        assert_eq!(interface.shortcut_sets[0].keys[0].modifier_a, 18);
        assert_eq!(interface.windows.len(), 1);
        let window = interface.window("TestWnd").unwrap();
        assert_eq!(window.head.size, Some(Size::Fixed { width: 100, height: 80 }));
        assert_eq!(window.part.additional_state, ["ObsState"]);
        assert_eq!(window.part.state, "GamingState");
        assert_eq!(window.children.len(), 3);
        assert_eq!(interface.control_count(), 4);
        assert_eq!(interface.layouts[0].values, [1, 10, 20, 0, SENTINEL, SENTINEL]);
        assert_eq!(interface.fonts.fonts[0].file, "tahoma.ttf");
        assert_eq!(interface.palette.len(), 2);
        assert_eq!(interface.palette[1].color, 0xFFFF_7200);
        assert_eq!(interface.trailer, [0, 67, 216, -1733207327, 0]);
    }

    #[test]
    fn text_box_keeps_style_and_utf16_text() {
        let data = fixture();
        let windows = read(&data).unwrap();
        let text = &windows[0].children[0];
        assert_eq!(text.kind, "TextBox");
        assert_eq!(text.head.styles[0].font_name, "HS12");
        assert_eq!(text.head.anchor.as_ref().unwrap().x, 1.5);
        let Props::TextBox(props) = &text.props else { panic!("kind") };
        assert_eq!(props.text, "각ab");
        assert_eq!(props.sysstring, 7);
    }

    #[test]
    fn scroll_area_nests_its_children_and_windows_nest_windows() {
        let data = fixture();
        let windows = read(&data).unwrap();
        let area = &windows[0].children[1];
        assert_eq!(area.children.len(), 1);
        assert_eq!(area.children[0].head.name, "btnInner");
        let Props::Button(button) = &area.children[0].props else { panic!("kind") };
        assert_eq!(button.button_name_id, 140);
        let sub = &windows[0].children[2];
        assert!(matches!(sub.props, Props::Window(_)));
    }

    #[test]
    fn leftover_bytes_are_rejected() {
        let mut data = fixture();
        data.push(0);
        assert!(read(&data).is_err());
    }

    #[test]
    fn truncation_is_rejected_at_every_prefix() {
        let data = fixture();
        for len in (0..data.len()).step_by(7) {
            assert!(read(&data[..len]).is_err(), "len {len}");
        }
    }

    #[test]
    fn unknown_control_kind_is_rejected() {
        let mut data = fixture();
        let at = data.windows(7).position(|w| w == b"ScrollA").unwrap();
        data[at] = b'X';
        assert!(read(&data).is_err());
    }

    #[test]
    fn dump_lists_tree_with_geometry() {
        let data = fixture();
        let windows = read(&data).unwrap();
        let text = dump_window(&windows[0]);
        assert!(text.contains("Window TestWnd"));
        assert!(text.contains("size 100x80"));
        assert!(text.contains("TextBox txtTitle"));
        assert!(text.contains("anchor kind=1"));
    }
}
