use crate::color::{Argb, BUTTON_TEXT};
use crate::draw::DrawList;
use crate::font::BitmapFont;
use crate::window::Rect;
use l2_xdat::{Alignment, Anchor, EditKind, ItemWindow, Kind, Node, Tab, TextAlign, TextureKind, TextureLayer, Window};

pub const UNDEFINED: i32 = -9999;
pub const ICON: i32 = 32;

#[derive(Debug, Clone, PartialEq)]
pub struct Placed {
    pub path: Vec<usize>,
    pub name: String,
    pub class: &'static str,
    pub rect: Rect,
    pub parent: Rect,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layout {
    pub window: String,
    pub size: (i32, i32),
    pub placed: Vec<Placed>,
    pub unsupported: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cell {
    pub icon: String,
    pub selected: bool,
    pub count: Option<i32>,
    pub equipped: bool,
}

pub trait Data {
    fn text(&self, ctrl: &str) -> Option<String>;
    fn sysstring(&self, id: i32) -> Option<String>;
    fn items(&self, ctrl: &str) -> Vec<Cell>;
    fn hidden(&self, _ctrl: &str) -> bool {
        false
    }
    fn has_texture(&self, _file: &str) -> bool {
        true
    }
    fn weight(&self) -> Option<(i32, i32)> {
        None
    }
    fn bar(&self, _ctrl: &str) -> Option<BarPoint> {
        None
    }
    fn regen(&self, _ctrl: &str) -> f64 {
        0.0
    }
    fn name(&self, _ctrl: &str) -> Option<(String, TextAlign, Argb)> {
        None
    }
    fn buff_rows(&self, _ctrl: &str) -> Vec<Vec<BuffCell>> {
        Vec::new()
    }
    fn buff_size(&self) -> i32 {
        0
    }
    fn minimap(&self) -> Option<MinimapState> {
        None
    }
    fn checked(&self, _ctrl: &str) -> bool {
        false
    }
    fn font_package(&self) -> &str {
        FONT_PACKAGE
    }
    fn alpha_override(&self, _ctrl: &str) -> Option<i32> {
        None
    }
    fn texture_override(&self, _ctrl: &str) -> Option<String> {
        None
    }
    fn chat_scroll_bar(&self, _ctrl: &str) -> (i32, i32, i32) {
        CHAT_SCROLL_BAR_DEFAULT
    }
    fn scroll(&self, _ctrl: &str) -> i32 {
        0
    }
    fn lines(&self, _ctrl: &str) -> Vec<(Argb, String)> {
        Vec::new()
    }
    fn list_records(&self, _ctrl: &str) -> Vec<Vec<crate::xdat_paint_lists::ListCell>> {
        Vec::new()
    }
    fn list_selected(&self, _ctrl: &str) -> Option<usize> {
        None
    }
    fn list_top(&self, _ctrl: &str) -> usize {
        0
    }
    fn combo_items(&self, _ctrl: &str) -> Option<(Vec<String>, Option<usize>)> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuffCell {
    pub icon: String,
    pub back: String,
    pub remaining: Option<i32>,
    pub short_item: bool,
    pub etc_item: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TownMap {
    pub texture: String,
    pub origin: (i32, i32),
    pub size: (i32, i32),
    pub scale: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MinimapState {
    pub town: Option<TownMap>,
    pub player: Option<(i32, i32)>,
    pub yaw: i32,
    pub targets: Vec<(i32, i32)>,
    pub party: Vec<(i32, i32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarPoint {
    Point { current: i64, max: i64 },
    Percent { current: i64, max: i64, base: i64 },
}

impl BarPoint {
    pub fn point(current: i32, max: i32) -> Self {
        BarPoint::Point { current: i64::from(current), max: i64::from(max) }
    }

    pub fn exp(exp: i64, level: i32) -> Option<Self> {
        let (base, max) = crate::interlude_exp::level_bounds(level)?;
        Some(BarPoint::Percent { current: exp, max, base })
    }

    fn fields(self) -> (i32, i64, i64, i64) {
        match self {
            BarPoint::Point { current, max } => (0, current, max, 0),
            BarPoint::Percent { current, max, base } => (1, current, max, base),
        }
    }

    pub fn warn(self) -> bool {
        let (_, current, max, _) = self.fields();
        current < max / 10 * 3
    }

    pub fn fill(self, width: i32) -> i32 {
        let (mode, current, max, base) = self.fields();
        if current == 0 {
            return 0;
        }
        let (part, whole) = if mode == 0 { (current, max) } else { (current - base, max - base) };
        if whole == 0 {
            return 0;
        }
        (part as f64 * f64::from(width) / whole as f64) as i32
    }
}

pub fn title_sysstring(window: &Window) -> Option<i32> {
    Some(window.title).filter(|id| *id > 0)
}

pub fn title(window: &Window, data: &dyn Data) -> Option<String> {
    let id = title_sysstring(window)?;
    data.sysstring(id).or_else(|| data.text(&format!("{}.Title", window.base.name)))
}

pub const FRAME_BACK_LEFT: &str = "L2UI_CH3.FrameCtrl.FrameBackLeft";
pub const FRAME_BACK_MID: &str = "L2UI_CH3.FrameCtrl.FrameBackMid";
pub const FRAME_BACK_RIGHT: &str = "L2UI_CH3.FrameCtrl.FrameBackRight";
pub const SMALLBAR_H1: &str = "L2UI_CH3.FrameCtrl.smallbar_h1";
pub const SMALLBAR_H2: &str = "L2UI_CH3.FrameCtrl.smallbar_h2";
pub const SMALLBAR_H3: &str = "L2UI_CH3.FrameCtrl.smallbar_h3";
pub const SMALLBAR1: &str = "L2UI_CH3.FrameCtrl.smallbar1";
pub const SMALLBAR2: &str = "L2UI_CH3.FrameCtrl.smallbar2";
pub const SMALLBAR3: &str = "L2UI_CH3.FrameCtrl.smallbar3";
pub const FRAME_TITLE_COLOR: Argb = Argb(0xFFC8D2DC);
pub const FRAME_TEXTURES: [&str; 9] = [
    FRAME_BACK_LEFT,
    FRAME_BACK_MID,
    FRAME_BACK_RIGHT,
    SMALLBAR_H1,
    SMALLBAR_H2,
    SMALLBAR_H3,
    SMALLBAR1,
    SMALLBAR2,
    SMALLBAR3,
];

pub fn frame_ctrl_rect(window: &Window, size: (i32, i32)) -> Option<Rect> {
    if window.frame == 0 {
        return None;
    }
    Some(match (window.frame_direction == 1, window.frame_size) {
        (true, 1) => Rect::new(0, 0, 12, size.1),
        (false, 1) => Rect::new(0, 0, size.0, 12),
        _ => Rect::new(0, 0, size.0, 20),
    })
}

pub fn frame_grip(window: &Window, size: (i32, i32)) -> Option<Rect> {
    frame_ctrl_rect(window, size)
}

pub const FRAME_CLOSE_BTN: &str = "L2UI_CH3.FrameCtrl.FrameCloseBtn";
pub const FRAME_CLOSE_ON_BTN: &str = "L2UI_CH3.FrameCtrl.FrameCloseOnBtn";
pub const FRAME_MINI_BTN: &str = "L2UI_CH3.FrameCtrl.FrameMiniBtn";
pub const FRAME_MINI_ON_BTN: &str = "L2UI_CH3.FrameCtrl.FrameMiniOnBtn";
pub const FRAME_BUTTON_SIZE: i32 = 15;

pub fn frame_close_button(window: &Window, size: (i32, i32)) -> Option<Rect> {
    (window.frame != 0 && window.frame_exit_button != 0).then(|| frame_close_button_rect(size))
}

pub fn frame_mini_button(window: &Window, size: (i32, i32)) -> Option<Rect> {
    (window.frame != 0 && window.iconable != 0).then(|| {
        let right = if window.frame_exit_button != 0 { 25 } else { 8 };
        Rect::new(size.0 - right - FRAME_BUTTON_SIZE, 3, FRAME_BUTTON_SIZE, FRAME_BUTTON_SIZE)
    })
}

#[allow(clippy::too_many_arguments)]
fn paint_frame_ctrl(
    list: &mut DrawList,
    data: &dyn Data,
    window: &Window,
    x: i32,
    y: i32,
    size: (i32, i32),
    font: &BitmapFont,
    alpha: f32,
) {
    let Some(frame) = frame_ctrl_rect(window, size) else { return };
    let has_texture = |name: &str| data.has_texture(name);
    let title = title(window, data);
    paint_frame_bar(list, &has_texture, frame.offset(x, y), window.frame_size, window.frame_direction == 1, title.as_deref(), font, alpha);
    for (button, texture) in [(frame_close_button(window, size), FRAME_CLOSE_BTN), (frame_mini_button(window, size), FRAME_MINI_BTN)] {
        if let Some(button) = button {
            paint_frame_button(list, &has_texture, button.offset(x, y), texture, alpha);
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn paint_frame_bar(
    list: &mut DrawList,
    has_texture: &dyn Fn(&str) -> bool,
    frame: Rect,
    frame_size: i32,
    vertical: bool,
    title: Option<&str>,
    font: &BitmapFont,
    alpha: f32,
) {
    let (w, h) = (frame.w, frame.h);
    let start = list.quads.len();
    let mut draw = |x: i32, y: i32, w: i32, h: i32, uw: i32, vh: i32, texture: &str| {
        if has_texture(texture) {
            list.draw_texture(frame.x + x, frame.y + y, w, h, 0, 0, uw, vh, texture, alpha);
        }
    };
    match (vertical, frame_size) {
        (false, 0) => {
            draw(0, 0, 16, 20, 16, 20, FRAME_BACK_LEFT);
            draw(w - 16, 0, 16, 20, 16, 20, FRAME_BACK_RIGHT);
            if has_texture(FRAME_BACK_MID) {
                let tiles = list.quads.len();
                for tile in 0..(w - 32 + 31) / 32 {
                    list.draw_texture(frame.x + 16 + tile * 32, frame.y, 32, 20, 0, 0, 32, 20, FRAME_BACK_MID, alpha);
                }
                clip(list, tiles, Rect::new(frame.x + 16, frame.y, w - 32, 20));
            }
            if let Some(title) = title.filter(|title| !title.is_empty()) {
                let ty = (h as f32 * 0.5 - (font.line_height as i32 / 2) as f32 + 2.0) as i32;
                list.draw_text(frame.x + 20, frame.y + ty, FRAME_TITLE_COLOR, title, font);
            }
        }
        (false, 1) => {
            draw(0, 0, 8, 12, 8, 12, SMALLBAR_H3);
            draw(8, 0, w - 16, 12, 8, 12, SMALLBAR_H2);
            draw(w - 8, 0, 8, 12, 8, 12, SMALLBAR_H1);
        }
        (true, 1) => {
            draw(0, 0, 12, 8, 12, 8, SMALLBAR1);
            draw(0, 8, 12, h - 16, 12, 8, SMALLBAR2);
            draw(0, h - 8, 12, 8, 12, 8, SMALLBAR3);
        }
        _ => {}
    }
    clip(list, start, frame);
}

pub fn paint_frame_button(list: &mut DrawList, has_texture: &dyn Fn(&str) -> bool, button: Rect, texture: &str, alpha: f32) {
    if has_texture(texture) {
        list.draw_button_texture(button.x, button.y, button.w, button.h, 0, 0, button.w, button.h, texture, alpha, true);
    }
}

pub fn frame_close_button_rect(size: (i32, i32)) -> Rect {
    Rect::new(size.0 - 8 - FRAME_BUTTON_SIZE, 3, FRAME_BUTTON_SIZE, FRAME_BUTTON_SIZE)
}

#[allow(clippy::too_many_arguments)]
pub fn paint_scroll_bar(
    list: &mut DrawList,
    has_texture: &dyn Fn(&str) -> bool,
    rect: Rect,
    total: i32,
    visible: i32,
    position: i32,
    alpha: f32,
) {
    let bar = crate::xdat_paint_tabs::scroll_bar((rect.w, rect.h), total, visible, position);
    for (piece, texture) in [(bar.up, crate::xdat_paint_tabs::SCROLL_UP_BTN), (bar.down, crate::xdat_paint_tabs::SCROLL_DOWN_BTN)] {
        if has_texture(texture) {
            list.draw_button_texture(rect.x + piece.x, rect.y + piece.y, piece.w, piece.h, 0, 0, piece.w, piece.h, texture, alpha, true);
        }
    }
    if let Some(thumb) = bar.thumb {
        for (piece, texture) in crate::xdat_paint_tabs::scroll_thumb_pieces(thumb) {
            if has_texture(texture) && piece.h > 0 {
                list.draw_texture(rect.x + piece.x, rect.y + piece.y, piece.w, piece.h, 0, 0, piece.w, piece.h, texture, alpha);
            }
        }
    }
}

fn clip(list: &mut DrawList, start: usize, rect: Rect) {
    if rect.w > 0 && rect.h > 0 {
        list.clip_from(start, rect);
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct State {
    pub cursor: Option<(i32, i32)>,
    pub pressed: Option<String>,
    pub tabs: Vec<(String, usize)>,
}

impl State {
    pub fn tab(&self, ctrl: &str) -> usize {
        self.tabs.iter().find(|(name, _)| name == ctrl).map_or(0, |(_, index)| *index)
    }

    pub fn select_tab(&mut self, ctrl: &str, index: usize) {
        match self.tabs.iter_mut().find(|(name, _)| name == ctrl) {
            Some(entry) => entry.1 = index,
            None => self.tabs.push((ctrl.to_string(), index)),
        }
    }
}

fn axis(align: Alignment, size: i32, horizontal: bool) -> f32 {
    let third = match (align, horizontal) {
        (Alignment::None | Alignment::TopLeft | Alignment::CenterLeft | Alignment::BottomLeft, true) => 0,
        (Alignment::TopCenter | Alignment::Center | Alignment::BottomCenter, true) => 1,
        (Alignment::TopRight | Alignment::CenterRight | Alignment::BottomRight, true) => 2,
        (Alignment::None | Alignment::TopLeft | Alignment::TopCenter | Alignment::TopRight, false) => 0,
        (Alignment::CenterLeft | Alignment::Center | Alignment::CenterRight, false) => 1,
        (Alignment::BottomLeft | Alignment::BottomCenter | Alignment::BottomRight, false) => 2,
    };
    match third {
        0 => 0.0,
        1 => size as f32 * 0.5,
        _ => size as f32,
    }
}

pub fn place(anchor: Option<&Anchor>, reference: Rect, child: (i32, i32)) -> (i32, i32) {
    let Some(anchor) = anchor else { return (reference.x, reference.y) };
    let (own, target) =
        if anchor.parent == Alignment::None { (anchor.this, anchor.this) } else { (anchor.parent, anchor.this) };
    let dx = (axis(target, reference.w, true) - axis(own, child.0, true)) as i32;
    let dy = (axis(target, reference.h, false) - axis(own, child.1, false)) as i32;
    (reference.x + dx + anchor.x, reference.y + dy + anchor.y)
}

pub fn window_size(size: &l2_xdat::Size, parent: (i32, i32)) -> (i32, i32) {
    match &size.percent {
        Some(rate) if rate.width < 0.0 && rate.height < 0.0 => (0, 0),
        Some(rate) => (
            (rate.width * parent.0 as f32 + size.width as f32) as i32,
            (rate.height * parent.1 as f32 + size.height as f32) as i32,
        ),
        None => (size.width.max(0), size.height.max(0)),
    }
}

pub fn client_height(frame: i32, frame_direction: i32, height: i32) -> i32 {
    if frame != 0 && matches!(frame_direction, 0 | 3) { height - 20 } else { height }
}

pub fn clip_to_parent(rect: Rect, parent: Rect) -> Rect {
    let x = rect.x.max(parent.x);
    let y = rect.y.max(parent.y);
    let w = if rect.x + rect.w <= parent.x + parent.w { rect.w } else { parent.x + parent.w - rect.x };
    let h = if rect.y + rect.h <= parent.y + parent.h { rect.h } else { parent.y + parent.h - rect.y };
    Rect::new(x, y, w, h)
}

pub const SUPPORTED: [&str; 24] = [
    "Window",
    "Texture",
    "Button",
    "EffectButton",
    "TextBox",
    "NameCtrl",
    "ItemWindow",
    "Tab",
    "InvenWeight",
    "StatusBar",
    "BarCtrl",
    "Progress",
    "CheckBox",
    "StatusIconCtrl",
    "TreeCtrl",
    "ComboBox",
    "ListCtrl",
    "TextListBox",
    "EditBox",
    "ChatWindow",
    "ScrollArea",
    "ShortcutItemWindow",
    "MinimapCtrl",
    "Radar",
];

pub fn scope_of(window: &Window, path: &[usize]) -> String {
    let mut scope = String::new();
    for depth in 1..path.len() {
        if let Some(node) = node_at(window, &path[..depth])
            && matches!(node.kind, Kind::Window(_))
        {
            scope = node.base.name.clone();
        }
    }
    scope
}

pub fn node_at<'a>(window: &'a Window, path: &[usize]) -> Option<&'a Node> {
    let (first, rest) = path.split_first()?;
    let mut node = window.children.get(*first)?;
    for index in rest {
        node = node.children().get(*index)?;
    }
    Some(node)
}

pub fn layout(window: &Window) -> Layout {
    layout_sized(window, window.base.size.as_ref().map_or((0, 0), |size| (size.width, size.height)))
}

pub fn saved_size(window: &Window, xdat: (i32, i32), saved: Option<(i32, i32)>) -> (i32, i32) {
    let Some(saved) = saved.filter(|_| window.save_size != 0) else { return xdat };
    if window.resize_frame == 0 {
        (saved.0.max(0), saved.1.max(0))
    } else {
        match window.resize_frame_direction {
            2 => (saved.0.max(0), xdat.1),
            3 => (xdat.0, saved.1.max(0)),
            _ => xdat,
        }
    }
}

pub fn layout_sized(window: &Window, size: (i32, i32)) -> Layout {
    let root = Rect::new(0, 0, size.0, size.1);
    let client = (size.0, client_height(window.frame, window.frame_direction, size.1));
    let mut placed = Vec::new();
    let mut path = Vec::new();
    walk(&window.children, root, client, root, &window.base.name, &mut placed, &mut path);
    let mut unsupported: Vec<&'static str> =
        placed.iter().map(|p| p.class).filter(|class| !SUPPORTED.contains(class)).collect();
    unsupported.sort_unstable();
    unsupported.dedup();
    Layout { window: window.base.name.clone(), size, placed, unsupported }
}

fn walk(
    nodes: &[Node],
    parent: Rect,
    client: (i32, i32),
    root: Rect,
    window: &str,
    placed: &mut Vec<Placed>,
    path: &mut Vec<usize>,
) {
    for (index, node) in nodes.iter().enumerate() {
        let size = node.base.size.as_ref().map_or((0, 0), |size| window_size(size, client));
        let reference = match node.base.anchor.as_ref().map(|anchor| anchor.ctrl.as_str()) {
            Some("") | None => parent,
            Some(ctrl) if ctrl == window => root,
            Some(ctrl) => placed.iter().rev().find(|p| p.name == ctrl).map_or(parent, |p| p.rect),
        };
        let (x, y) = place(node.base.anchor.as_ref(), reference, size);
        let rect = Rect::new(x, y, size.0, size.1);
        path.push(index);
        placed.push(Placed {
            path: path.clone(),
            name: node.base.name.clone(),
            class: node.kind.class_name(),
            rect,
            parent,
        });
        let inner = match &node.kind {
            Kind::Window(inner) => (rect.w, client_height(inner.frame, inner.frame_direction, rect.h)),
            _ => (rect.w, rect.h),
        };
        walk(node.children(), rect, inner, root, window, placed, path);
        path.pop();
    }
}

impl Layout {
    pub fn find(&self, name: &str) -> Option<&Placed> {
        self.placed.iter().find(|p| p.name == name)
    }

    pub fn hidden_paths(&self, window: &Window, state: &State) -> Vec<Vec<usize>> {
        let mut hidden = Vec::new();
        for placed in &self.placed {
            let Some(Kind::Tab(tab)) = node_at(window, &placed.path).map(|node| &node.kind) else { continue };
            let selected = state.tab(&placed.name);
            for (index, element) in tab.tabs.iter().enumerate() {
                if index != selected
                    && let Some(target) = self.placed.iter().find(|p| p.name == element.target)
                {
                    hidden.push(target.path.clone());
                }
            }
        }
        hidden
    }

    pub fn hit(&self, window: &Window, state: &State, x: i32, y: i32) -> Option<&Placed> {
        let hidden = self.hidden_paths(window, state);
        self.placed.iter().rev().filter(|p| !hidden.iter().any(|h| p.path.starts_with(h))).find(|p| {
            matches!(p.class, "Button" | "EffectButton" | "Tab" | "ItemWindow" | "CheckBox" | "ShortcutItemWindow")
                && p.rect.contains(x, y)
        })
    }
}

pub fn tab_at(tab: &Tab, rect: Rect, x: i32, y: i32) -> Option<usize> {
    let mut left = rect.x;
    for (index, element) in tab.tabs.iter().enumerate() {
        if Rect::new(left, rect.y, element.width, element.height).contains(x, y) {
            return Some(index);
        }
        left += element.width + element.gap;
    }
    None
}

pub fn grid(item: &ItemWindow) -> (usize, usize, i32, i32, i32, i32) {
    let cols = usize::try_from(item.col).unwrap_or(1).max(1);
    let rows = usize::try_from(item.row).unwrap_or(1).max(1);
    let icon_w = if item.icon_width > 0 { item.icon_width } else { ICON };
    let icon_h = if item.icon_height > 0 { item.icon_height } else { ICON };
    let gap_x = if item.gap_x == UNDEFINED { 0 } else { item.gap_x };
    let gap_y = if item.gap_y == UNDEFINED { 0 } else { item.gap_y };
    (cols, rows, icon_w, icon_h, gap_x, gap_y)
}

pub fn cell_at(item: &ItemWindow, rect: Rect, x: i32, y: i32) -> Option<usize> {
    let (cols, rows, icon_w, icon_h, gap_x, gap_y) = grid(item);
    let (lx, ly) = (x - rect.x, y - rect.y);
    if lx < 0 || ly < 0 {
        return None;
    }
    let col = (lx / (icon_w + gap_x)) as usize;
    let row = (ly / (icon_h + gap_y)) as usize;
    if col >= cols || row >= rows || lx % (icon_w + gap_x) >= icon_w || ly % (icon_h + gap_y) >= icon_h {
        return None;
    }
    Some(row * cols + col)
}

fn inherit_str(own: &mut String, from: &str) {
    if defined(own).is_none() && defined(from).is_some() {
        *own = from.to_string();
    }
}

fn inherit_int(own: &mut i32, from: i32) {
    if *own == UNDEFINED {
        *own = from;
    }
}

fn inherit(node: &mut Node, from: &Node) {
    if node.base.size.is_none() {
        node.base.size = from.base.size.clone();
    }
    if node.base.anchor.is_none() {
        node.base.anchor = from.base.anchor.clone();
    }
    match (&mut node.kind, &from.kind) {
        (Kind::Texture(a), Kind::Texture(b)) => {
            inherit_str(&mut a.file, &b.file);
            inherit_int(&mut a.u, b.u);
            inherit_int(&mut a.v, b.v);
            if a.u_size == UNDEFINED as f32 {
                a.u_size = b.u_size;
            }
            if a.v_size == UNDEFINED as f32 {
                a.v_size = b.v_size;
            }
            inherit_int(&mut a.alpha, b.alpha);
            inherit_int(&mut a.anim_tex, b.anim_tex);
        }
        (Kind::Button(a), Kind::Button(b)) => {
            inherit_str(&mut a.normal_tex, &b.normal_tex);
            inherit_str(&mut a.pushed_tex, &b.pushed_tex);
            inherit_str(&mut a.highlight_tex, &b.highlight_tex);
            inherit_str(&mut a.drop_tex, &b.drop_tex);
            inherit_int(&mut a.button_name, b.button_name);
            inherit_int(&mut a.no_highlight, b.no_highlight);
            inherit_int(&mut a.default_sound_on, b.default_sound_on);
            inherit_int(&mut a.disable_time, b.disable_time);
        }
        (Kind::TextBox(a), Kind::TextBox(b)) => {
            inherit_str(&mut a.text, &b.text);
            inherit_int(&mut a.offset_y, b.offset_y);
            inherit_str(&mut a.back_tex, &b.back_tex);
            inherit_int(&mut a.font_type, b.font_type);
            inherit_int(&mut a.sysstring, b.sysstring);
            inherit_int(&mut a.system_msg, b.system_msg);
            if a.text_color == UNDEFINED as u32 {
                a.text_color = b.text_color;
            }
            inherit_int(&mut a.emoticon, b.emoticon);
            inherit_int(&mut a.autosize, b.autosize);
        }
        (Kind::StatusBar(a), Kind::StatusBar(b)) => {
            inherit_str(&mut a.title, &b.title);
            inherit_int(&mut a.texture_width, b.texture_width);
            inherit_int(&mut a.texture_height, b.texture_height);
            inherit_str(&mut a.fore_texture, &b.fore_texture);
            inherit_str(&mut a.back_texture, &b.back_texture);
            inherit_str(&mut a.warn_texture, &b.warn_texture);
            inherit_str(&mut a.regen_texture, &b.regen_texture);
        }
        (Kind::BarCtrl(a), Kind::BarCtrl(b)) => {
            inherit_str(&mut a.fore_texture, &b.fore_texture);
            inherit_str(&mut a.back_texture, &b.back_texture);
            inherit_int(&mut a.u_size, b.u_size);
            inherit_int(&mut a.v_size, b.v_size);
        }
        (Kind::CheckBox(a), Kind::CheckBox(b)) => {
            inherit_int(&mut a.title_index, b.title_index);
            inherit_str(&mut a.check_texture, &b.check_texture);
            inherit_str(&mut a.uncheck_texture, &b.uncheck_texture);
            inherit_str(&mut a.disable_texture, &b.disable_texture);
            inherit_str(&mut a.disable_check_texture, &b.disable_check_texture);
        }
        _ => {}
    }
}

fn find_node<'a>(nodes: &'a [Node], name: &str) -> Option<&'a Node> {
    for node in nodes {
        if node.base.name == name {
            return Some(node);
        }
        if let Some(found) = find_node(node.children(), name) {
            return Some(found);
        }
    }
    None
}

fn children_mut(node: &mut Node) -> &mut [Node] {
    match &mut node.kind {
        Kind::Window(window) => &mut window.children,
        Kind::ScrollArea(area) => &mut area.children,
        _ => &mut [],
    }
}

fn resolve_nodes(nodes: &mut [Node], all: &[Node]) {
    for node in nodes.iter_mut() {
        if let Some(name) = defined(&node.base.extends) {
            let name = name.rsplit('.').next().unwrap_or(name);
            if let Some(from) = find_node(all, name).filter(|from| from.base.name != node.base.name) {
                inherit(node, from);
            }
        }
        resolve_nodes(children_mut(node), all);
    }
}

pub fn resolve_extends(window: &mut Window) {
    for _ in 0..4 {
        let snapshot = window.children.clone();
        resolve_nodes(&mut window.children, &snapshot);
        if window.children == snapshot {
            break;
        }
    }
}

pub const CHECK_BOX_SIZE: i32 = 12;
pub fn status_icon_size(rows: &[Vec<BuffCell>], size: i32, has_texture: &dyn Fn(&str) -> bool) -> (i32, i32) {
    let mut width = 0;
    let mut height = 0;
    for row in rows {
        let mut x = 0;
        let mut row_height = 0;
        for cell in row {
            let span = if defined(&cell.back).is_some_and(has_texture) { size + 2 } else { size };
            x += span;
            row_height = row_height.max(span);
        }
        width = width.max(x);
        height += row_height;
    }
    (width, height)
}

pub const COMBO_BUTTON_SIZE: (i32, i32) = (15, 15);
pub const CHAT_ROW: i32 = 15;
pub const CHAT_SCROLL_BAR_DEFAULT: (i32, i32, i32) = (5, 10, 0);
pub const CHAT_TEXT_X: i32 = 23;
pub const CHAT_TEXT_Y: i32 = 14;
pub const EDIT_BOX_LEFT: &str = "L2UI_CH3.Etc.textbox1";
pub const EDIT_BOX_MID: &str = "L2UI_CH3.Etc.textbox2";
pub const EDIT_BOX_RIGHT: &str = "L2UI_CH3.Etc.textbox3";

pub const SHORTCUT_OUTLINE: &str = "L2UI_CH3.Etc.menu_outline";
pub const SHORTCUT_OUTLINE_DOWN: &str = "L2UI_CH3.Etc.menu_outline_down";

pub fn centered_text(width: i32, height: i32, text_width: i32, text_height: i32) -> (i32, i32) {
    (
        (width as f32 * 0.5 - (text_width / 2) as f32) as i32,
        (height as f32 * 0.5 - (text_height / 2) as f32) as i32,
    )
}

pub const WORLD_MAP_PREFIX: &str = "Minimap.ch4_worldmap";
pub const TOWN_MAP_GROUP: &str = "TownMap";
pub const FONT_PACKAGE: &str = "L2Font";
pub const MINIMAP_POS: &str = "L2UI.MinimapWnd.MinimapPos";
pub const MINIMAP_TARGET: &str = "L2UI.MinimapWnd.TargetPos";
pub const MINIMAP_PARTY: &str = "L2UI_CH3.Minimap.minimap_party";
pub const WORLD_MAP_SIZE: (i32, i32) = (0x70c, 0xa40);

pub const WORLD_MAP_TILE: i32 = 0x400;
pub const WORLD_MAP_COLUMNS: i32 = 2;
pub const WORLD_MAP_ROWS: i32 = 3;
pub const WORLD_MAP_ORIGIN: (i32, i32) = (0x20000, 0x40000);
pub const WORLD_MAP_SCALE: f64 = 0.0050048828125;

pub fn world_to_map(x: i32, y: i32) -> (i32, i32) {
    (
        (f64::from(x.wrapping_add(WORLD_MAP_ORIGIN.0)) * WORLD_MAP_SCALE) as i32,
        (f64::from(y.wrapping_add(WORLD_MAP_ORIGIN.1)) * WORLD_MAP_SCALE) as i32,
    )
}

pub fn minimap_rotation(yaw: i32) -> f32 {
    (f64::from(yaw) * std::f64::consts::TAU / 65536.0 + std::f64::consts::FRAC_PI_2) as f32
}

pub fn town_to_map(town: &TownMap, x: i32, y: i32) -> (i32, i32) {
    ((x.wrapping_sub(town.origin.0) as f32 * town.scale) as i32, (y.wrapping_sub(town.origin.1) as f32 * town.scale) as i32)
}

pub fn minimap_scroll(player: Option<(i32, i32)>, view: (i32, i32)) -> (i32, i32) {
    minimap_scroll_in(WORLD_MAP_SIZE, player, view)
}

pub fn minimap_scroll_in(map: (i32, i32), player: Option<(i32, i32)>, view: (i32, i32)) -> (i32, i32) {
    let (width, height) = map;
    let Some((px, py)) = player else { return (0, 0) };
    let axis = |v: i32, size: i32, view: i32| (v - view / 2).clamp(0, (size - view).max(0));
    (axis(px, width, view.0), axis(py, height, view.1))
}

pub fn minimap_tile(col: i32, row: i32, scroll: (i32, i32), view: (i32, i32)) -> Option<(Rect, (i32, i32))> {
    let tile = WORLD_MAP_TILE;
    let axis = |index: i32, scroll: i32, view: i32| -> Option<(i32, i32, i32)> {
        let start = index * tile;
        if scroll >= start && scroll < start + tile {
            let u = scroll - start;
            Some((0, (tile - u).min(view), u))
        } else if scroll >= start - tile && scroll < start {
            let offset = start - scroll;
            Some((offset, (view - offset).min(tile), 0))
        } else {
            None
        }
    };
    let (dx, w, u) = axis(col, scroll.0, view.0)?;
    let (dy, h, v) = axis(row, scroll.1, view.1)?;
    (w > 0 && h > 0).then_some((Rect::new(dx, dy, w, h), (u, v)))
}

pub fn frame_offsets(frame: i32, frame_size: i32, frame_direction: i32) -> (i32, i32) {
    if frame == 0 {
        return (0, 0);
    }
    let thickness = match frame_size {
        0 => 20,
        1 => 12,
        _ => 0,
    };
    match frame_direction {
        0 | 3 => (0, thickness),
        1 => (thickness, 0),
        _ => (0, 0),
    }
}

fn paint_window_back(list: &mut DrawList, data: &dyn Data, window: &Window, rect: Rect, alpha: f32) {
    let Some(back) = available(data, &window.back_tex) else { return };
    let (ox, oy) = frame_offsets(window.frame, window.frame_size, window.frame_direction);
    let (w, h) = (rect.w - ox, rect.h - oy);
    let start = list.quads.len();
    list.draw_texture(rect.x + ox, rect.y + oy, w, h, 0, 0, w, h, back, alpha);
    clip(list, start, rect);
}

pub fn text_x(align: TextAlign, width: i32, text_width: i32) -> i32 {
    match align {
        TextAlign::Center => ((width as f32 * 0.5 - (text_width / 2) as f32) as i32).max(0),
        TextAlign::Right => ((width as f32 - text_width as f32) as i32).max(0),
        _ => 0,
    }
}

pub fn name_with_dots(text: &str, width: i32, font: &BitmapFont) -> String {
    let full = font.measure(text) as i32;
    let over = (full as f32 - width as f32) as i32;
    if over <= 0 {
        return text.to_string();
    }
    let dots = font.measure("..") as i32;
    let mut kept: Vec<char> = text.chars().collect();
    while kept.pop().is_some() {
        let prefix: String = kept.iter().collect();
        if (full - dots) - font.measure(&prefix) as i32 > over {
            break;
        }
    }
    kept.into_iter().chain("..".chars()).collect()
}

fn available<'a>(data: &dyn Data, name: &'a str) -> Option<&'a str> {
    defined(name).filter(|file| data.has_texture(file))
}

fn defined(text: &str) -> Option<&str> {
    (!text.is_empty() && !text.get(..9).is_some_and(|head| head.eq_ignore_ascii_case("undefined"))).then_some(text)
}

fn texture_alpha(alpha: i32) -> f32 {
    if alpha == UNDEFINED { 1.0 } else { (alpha.clamp(0, 255) as f32) / 255.0 }
}

#[allow(clippy::too_many_arguments)]
pub fn paint(
    list: &mut DrawList,
    window: &Window,
    layout: &Layout,
    x: i32,
    y: i32,
    state: &State,
    data: &dyn Data,
    font: &BitmapFont,
    alpha: f32,
) {
    paint_frame_ctrl(list, data, window, x, y, layout.size, font, alpha);
    paint_window_back(list, data, window, Rect::new(x, y, layout.size.0, layout.size.1), alpha);
    let hidden = layout.hidden_paths(window, state);
    let background = |placed: &Placed| matches!(node_at(window, &placed.path).map(|node| &node.kind), Some(Kind::Texture(texture)) if texture.layer == TextureLayer::Background);
    let ordered = layout
        .placed
        .iter()
        .filter(|placed| background(placed))
        .chain(layout.placed.iter().filter(|placed| !background(placed)));
    let mut hidden_extra: Vec<Vec<usize>> = Vec::new();
    for placed in ordered {
        if hidden.iter().chain(hidden_extra.iter()).any(|h| placed.path.starts_with(h)) {
            continue;
        }
        let Some(node) = node_at(window, &placed.path) else { continue };
        let scope = scope_of(window, &placed.path);
        let key = if scope.is_empty() { placed.name.clone() } else { format!("{scope}.{}", placed.name) };
        if data.hidden(&key) {
            hidden_extra.push(placed.path.clone());
            continue;
        }
        let rect = placed.rect.offset(x, y);
        let hovered = state.cursor.is_some_and(|(cx, cy)| placed.rect.contains(cx, cy));
        let pressed = state.pressed.as_deref() == Some(placed.name.as_str());
        match &node.kind {
            Kind::Texture(texture) => {
                let overridden = data.texture_override(&key);
                let file = overridden.as_deref().unwrap_or(&texture.file);
                let Some(file) = defined(file).filter(|file| data.has_texture(file)) else { continue };
                let visible = clip_to_parent(rect, placed.parent.offset(x, y));
                let bx = if texture.u_size != 0.0 { texture.u_size as i32 } else { rect.w };
                let by = if texture.v_size != 0.0 { texture.v_size as i32 } else { rect.h };
                let (u, v) = (texture.u, texture.v);
                let a = texture_alpha(data.alpha_override(&key).unwrap_or(texture.alpha)) * alpha;
                let start = list.quads.len();
                match texture.kind {
                    TextureKind::Stretch => list.draw_texture(rect.x, rect.y, rect.w, rect.h, u, v, bx, by, file, a),
                    TextureKind::Normal | TextureKind::Control => {
                        list.draw_texture(rect.x, rect.y, bx, by, u, v, bx, by, file, a)
                    }
                    TextureKind::Tile => {
                        if bx > 0 && by > 0 {
                            for row in 0..(rect.h + by - 1) / by {
                                for col in 0..(rect.w + bx - 1) / bx {
                                    list.draw_texture(
                                        rect.x + col * bx,
                                        rect.y + row * by,
                                        bx,
                                        by,
                                        u,
                                        v,
                                        bx,
                                        by,
                                        file,
                                        a,
                                    );
                                }
                            }
                        }
                    }
                    TextureKind::Draggable => {
                        let u = u.clamp(0, (texture.u_size as i32 - rect.w).max(0));
                        let v = v.clamp(0, (texture.v_size as i32 - rect.h).max(0));
                        list.draw_texture(rect.x, rect.y, rect.w, rect.h, u, v, rect.w, rect.h, file, a);
                    }
                    TextureKind::Mask => {}
                }
                clip(list, start, visible);
            }
            Kind::Button(button) => {
                let start = list.quads.len();
                let face = if pressed && hovered {
                    defined(&button.pushed_tex)
                } else if hovered {
                    defined(&button.highlight_tex)
                } else {
                    None
                };
                if let Some(face) = face.or_else(|| defined(&button.normal_tex)).filter(|face| data.has_texture(face)) {
                    list.draw_button_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, face, alpha, true);
                }
                let label = if button.button_name >= 0 { data.sysstring(button.button_name) } else { data.text(&placed.name) };
                if let Some(label) = label.filter(|label| !label.is_empty()) {
                    let (tx, ty) = centered_text(rect.w, rect.h, font.measure(&label) as i32, font.line_height as i32);
                    list.draw_text(rect.x + tx, rect.y + ty, BUTTON_TEXT, &label, font);
                }
                clip(list, start, rect);
            }
            Kind::TextBox(text_box) => {
                let start = list.quads.len();
                if let Some(back) = available(data, &text_box.back_tex) {
                    list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, back, alpha);
                }
                let text = data
                    .text(&placed.name)
                    .or_else(|| (text_box.sysstring >= 0).then(|| data.sysstring(text_box.sysstring)).flatten())
                    .or_else(|| defined(&text_box.text).map(str::to_string));
                if let Some(text) = text.filter(|text| !text.is_empty()) {
                    let color = Argb(text_box.text_color);
                    let lines: Vec<&str> = text.split("\\n").collect();
                    let line_height = font.line_height as i32;
                    for (row, line) in lines.iter().enumerate() {
                        let tx = text_x(text_box.align, rect.w, font.measure(line) as i32 + 1);
                        let ty = rect.y + if lines.len() > 1 { line_height * row as i32 } else { 0 };
                        match text_box.font_type {
                            10 => list.draw_digits(rect.x + tx, rect.y + text_box.offset_y, line, color),
                            11 => list.draw_digits_with(crate::draw::XLARGE_NUMBER_STRIP, rect.x + tx, rect.y + text_box.offset_y, line, color),
                            8.. => {}
                            _ => {
                                if text_box.align != TextAlign::MacroIcon {
                                    list.draw_text(rect.x + tx, ty, color, line, font);
                                }
                            }
                        }
                    }
                }
                clip(list, start, rect);
            }
            Kind::ItemWindow(item) => {
                let cells: Vec<Option<crate::xdat_paint_items::ItemCell>> = data
                    .items(&key)
                    .into_iter()
                    .map(|cell| {
                        Some(crate::xdat_paint_items::ItemCell {
                            icon: cell.icon,
                            selected: cell.selected,
                            number: cell.count,
                            ..Default::default()
                        })
                    })
                    .collect();
                crate::xdat_paint_items::paint_item_window(list, rect, item, 0, &cells, None, &|t| data.has_texture(t), font, alpha);
            }
            Kind::StatusBar(bar) => {
                let start = list.quads.len();
                let point = data.bar(&key);
                let (mode, current, max, base) = point.map_or((0, 0, 0, 0), BarPoint::fields);
                let (tw, th) = (bar.texture_width, bar.texture_height);
                if let Some(back) = available(data, &bar.back_texture) {
                    list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, tw, th, back, alpha);
                }
                let fill = point.map_or(0, |point| point.fill(rect.w));
                if fill > 0 && let Some(fore) = available(data, &bar.fore_texture) {
                    list.draw_texture(rect.x, rect.y, fill, rect.h, 0, 0, tw, th, fore, alpha);
                }
                if let Some(regen) = available(data, &bar.regen_texture) {
                    let amount = data.regen(&key);
                    let width = if amount > 0.0 && max != 0 { (f64::from(rect.w) * amount / max as f64) as i32 } else { 0 };
                    if width > 0 {
                        list.draw_texture(rect.x + fill, rect.y, width, rect.h, 0, 0, tw, th, regen, alpha * 128.0 / 255.0);
                    }
                }
                let warn = point.map_or(defined(&bar.warn_texture).is_some(), BarPoint::warn);
                if warn && fill > 0 && let Some(texture) = available(data, &bar.warn_texture) {
                    list.draw_texture(rect.x, rect.y, fill, rect.h, 0, 0, tw, th, texture, alpha);
                }
                let color = Argb(0xFFDCDCDC);
                let half = rect.w as f32 * 0.5;
                let centered = |text: &str| (half - (font.measure(text) as i32 / 2) as f32) as i32;
                if mode == 0 {
                    list.draw_digits(rect.x + 9, rect.y + 2, &bar.title, color);
                    list.draw_digits(rect.x + centered("/"), rect.y + 2, "/", color);
                    let current = (current as i32).to_string();
                    let left = (half - font.measure(&current) as f32 - 6.0 - 2.0) as i32;
                    list.draw_digits(rect.x + left, rect.y + 2, &current, color);
                    let max = (max as i32).to_string();
                    list.draw_digits(rect.x + (half + 6.0 - 2.0) as i32, rect.y + 2, &max, color);
                } else {
                    let percent = format!("{:.2}%", (current - base) as f64 * 100.0 / (max - base) as f64);
                    list.draw_digits(rect.x + centered(&percent), rect.y + 2, &percent, color);
                }
                clip(list, start, rect);
            }
            Kind::BarCtrl(bar) => {
                let start = list.quads.len();
                let (tw, th) = (bar.u_size, bar.v_size);
                if let Some(back) = available(data, &bar.back_texture) {
                    list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, tw, th, back, alpha);
                }
                if let Some(fore) = available(data, &bar.fore_texture)
                    && let Some(BarPoint::Point { current, max }) = data.bar(&key)
                    && max != 0
                {
                    let fill = (current as f32 * rect.w as f32 / max as f32) as i32;
                    if fill > 0 {
                        list.draw_texture(rect.x, rect.y, fill, rect.h, 0, 0, tw, th, fore, alpha);
                    }
                }
                clip(list, start, rect);
            }
            Kind::Progress(progress) => {
                let start = list.quads.len();
                if let Some(back) = available(data, &progress.back_texture) {
                    list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, back, alpha);
                }
                if let Some(bar) = available(data, &progress.bar_texture)
                    && let Some(BarPoint::Point { current, max }) = data.bar(&key)
                    && max > 0
                {
                    let gap = progress.gap;
                    let inner = rect.w - gap * 2;
                    let elapsed = (current.max(0).min(max) as u64 * inner.max(0) as u64 / max as u64) as i32;
                    let width = inner - elapsed;
                    if width > 0 {
                        list.draw_texture(rect.x + gap, rect.y + gap, width, rect.h - gap * 2, 0, 0, rect.w, rect.h, bar, alpha);
                    }
                }
                clip(list, start, rect);
            }
            Kind::StatusIconCtrl(_) => {
                let size = data.buff_size();
                let mut y = 0;
                for row in data.buff_rows(&key) {
                    let mut x = 0;
                    let mut row_height = 0;
                    for cell in &row {
                        let framed = available(data, &cell.back).is_some();
                        let span = if framed { size + 2 } else { size };
                        let inset = i32::from(framed);
                        if let Some(back) = available(data, &cell.back) {
                            list.draw_texture(rect.x + x, rect.y + y, span, span, 0, 0, span, span, back, alpha);
                        }
                        if data.has_texture(&cell.icon) {
                            list.draw_texture(rect.x + x + inset, rect.y + y + inset, size, size, 0, 0, size, size, &cell.icon, alpha);
                        }
                        if let Some(remaining) = cell.remaining.filter(|r| (0..60).contains(r) && !cell.etc_item) {
                            let text = remaining.to_string();
                            if cell.short_item {
                                let tw = font.measure(&text) as i32;
                                list.draw_digits(rect.x + x - tw / 2 + 12, rect.y + y + 8, &text, Argb(0xFFDCDCDC));
                            } else {
                                let dx = if remaining < 10 { 7 } else { 2 };
                                list.draw_digits_with(crate::draw::XLARGE_NUMBER_STRIP, rect.x + x + dx, rect.y + y + 10, &text, Argb(0xCCDCDCDC));
                            }
                        }
                        x += span;
                        row_height = row_height.max(span);
                    }
                    y += row_height;
                }
            }
            Kind::ComboBox(combo) => {
                let (items, selected) = match data.combo_items(&key) {
                    Some(host) => host,
                    None => {
                        let items: Vec<String> = combo
                            .values
                            .iter()
                            .map(|value| if value.sys_string > 0 { data.sysstring(value.sys_string).unwrap_or_default() } else { value.text.clone() })
                            .collect();
                        let selected = (!items.is_empty()).then_some(0);
                        (items, selected)
                    }
                };
                crate::xdat_paint_lists::paint_combo_box(
                    list,
                    rect,
                    &items,
                    selected,
                    None,
                    false,
                    true,
                    font.line_height as i32,
                    COMBO_BUTTON_SIZE,
                    &|name| data.has_texture(name),
                    font,
                    alpha,
                );
            }
            Kind::ListCtrl(ctrl) => {
                let spec = crate::xdat_paint_lists::ListCtrlSpec::from_xdat(ctrl);
                let columns = crate::xdat_paint_lists::list_columns(ctrl, &|id| data.sysstring(id));
                let states = vec![crate::xdat_paint_lists::HeaderState::Normal; columns.len()];
                let records = data.list_records(&key);
                crate::xdat_paint_lists::paint_list_ctrl(
                    list,
                    rect,
                    &spec,
                    &columns,
                    &records,
                    data.list_top(&key),
                    data.list_selected(&key),
                    &states,
                    None,
                    &|name| data.has_texture(name),
                    font,
                    alpha,
                );
            }
            Kind::TextListBox(spec) => {
                let items: Vec<crate::xdat_paint_lists::ListBoxItem> = data
                    .lines(&key)
                    .into_iter()
                    .map(|(color, text)| crate::xdat_paint_lists::ListBoxItem { text, color, indent: 0 })
                    .collect();
                crate::xdat_paint_lists::paint_text_list_box(
                    list,
                    rect,
                    spec,
                    &items,
                    0,
                    font.line_height as i32,
                    font,
                );
            }
            Kind::MinimapCtrl(_) => {
                let Some(state) = data.minimap() else { continue };
                let start = list.quads.len();
                let view = (rect.w, rect.h);
                let to_map = |x: i32, y: i32| match &state.town {
                    Some(town) => town_to_map(town, x, y),
                    None => world_to_map(x, y),
                };
                let player = state.player.map(|(x, y)| to_map(x, y));
                let scroll = match &state.town {
                    Some(town) => minimap_scroll_in(town.size, player, view),
                    None => minimap_scroll(player, view),
                };
                if let Some(town) = &state.town {
                    if data.has_texture(&town.texture) {
                        list.draw_texture(rect.x, rect.y, rect.w, rect.h, scroll.0, scroll.1, rect.w, rect.h, &town.texture, alpha);
                    }
                } else {
                    for row in 0..WORLD_MAP_ROWS {
                        for col in 0..WORLD_MAP_COLUMNS {
                            if let Some((tile, (u, v))) = minimap_tile(col, row, scroll, view) {
                                let texture = format!("{}.{WORLD_MAP_PREFIX}{}", data.font_package(), 1 + col + row * WORLD_MAP_COLUMNS);
                                if data.has_texture(&texture) {
                                    list.draw_texture(rect.x + tile.x, rect.y + tile.y, tile.w, tile.h, u, v, tile.w, tile.h, &texture, alpha);
                                }
                            }
                        }
                    }
                }
                let inside = |px: i32, py: i32| px > 11 && px <= rect.w - 12 && py > 11 && py < rect.h - 12;
                for &(wx, wy) in &state.party {
                    let (mx, my) = to_map(wx, wy);
                    let (px, py) = (mx - scroll.0, my - scroll.1);
                    if inside(px, py) && data.has_texture(MINIMAP_PARTY) {
                        list.draw_texture(rect.x + px, rect.y + py, 16, 16, 0, 0, 16, 16, MINIMAP_PARTY, alpha);
                    }
                }
                if let Some((mx, my)) = player {
                    let (px, py) = (mx - scroll.0, my - scroll.1);
                    if inside(px, py) && data.has_texture(MINIMAP_POS) {
                        list.draw_texture_rotated(rect.x + px - 16, rect.y + py - 16, 32, 32, 32, 32, MINIMAP_POS, minimap_rotation(state.yaw), alpha);
                    }
                }
                for &(wx, wy) in &state.targets {
                    let (mx, my) = to_map(wx, wy);
                    let (px, py) = (mx - scroll.0, my - scroll.1);
                    if inside(px, py) && data.has_texture(MINIMAP_TARGET) {
                        list.draw_texture(rect.x + px - 16, rect.y + py - 31, 32, 32, 0, 0, 32, 32, MINIMAP_TARGET, alpha);
                    }
                }
                clip(list, start, rect);
            }
            Kind::ScrollArea(area) if area.area_height > rect.h => {
                paint_scroll_bar(list, &|name| data.has_texture(name), rect, area.area_height, rect.h, 0, alpha);
            }
            Kind::NameCtrl => {
                let Some((text, align, color)) = data.name(&key) else { continue };
                let start = list.quads.len();
                let text = name_with_dots(&text, rect.w, font);
                let tx = match align {
                    TextAlign::Left => Some(0),
                    TextAlign::Center => Some((rect.w as f32 * 0.5 - (font.measure(&text) as i32 / 2) as f32) as i32),
                    _ => None,
                };
                if let Some(tx) = tx {
                    list.draw_text(rect.x + tx, rect.y, color, &text, font);
                }
                clip(list, start, rect);
            }
            Kind::Window(inner) => {
                paint_window_back(list, data, inner, rect, alpha);
            }
            Kind::CheckBox(check) => {
                let start = list.quads.len();
                let face = if data.checked(&key) { &check.check_texture } else { &check.uncheck_texture };
                let (bw, bh) = (CHECK_BOX_SIZE, CHECK_BOX_SIZE);
                let label = (check.title_index >= 0).then(|| data.sysstring(check.title_index)).flatten().unwrap_or_default();
                let (box_x, text_x) = if check.left_aligned == 0 { (0, bw + 4) } else { (font.measure(&label) as i32 + 4, 0) };
                if let Some(face) = defined(face).filter(|f| data.has_texture(f)) {
                    list.draw_button_texture(rect.x + box_x, rect.y, bw, bh, 0, 0, bw, bh, face, alpha, true);
                }
                if !label.is_empty() {
                    list.draw_text(rect.x + text_x, rect.y, Argb(0xFFDCDCDC), &label, font);
                }
                clip(list, start, rect);
            }
            Kind::EffectButton(button) => {
                let face = if pressed && hovered {
                    defined(&button.pushed_tex)
                } else if hovered {
                    defined(&button.highlight_tex)
                } else {
                    None
                };
                let Some(face) = face.or_else(|| defined(&button.normal_tex)).filter(|face| data.has_texture(face))
                else {
                    continue;
                };
                list.draw_button_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, face, alpha, true);
            }
            Kind::EditBox(edit) => {
                let start = list.quads.len();
                for (texture, x, w) in [
                    (EDIT_BOX_LEFT, 0, 8),
                    (EDIT_BOX_MID, 8, rect.w - 16),
                    (EDIT_BOX_RIGHT, rect.w - 8, 8),
                ] {
                    if data.has_texture(texture) && w > 0 {
                        let tiles = list.quads.len();
                        for row in 0..(rect.h + 16) / 17 {
                            list.draw_texture(rect.x + x, rect.y + row * 17, w, 17, 0, 0, 8, 17, texture, alpha);
                        }
                        clip(list, tiles, Rect::new(rect.x + x, rect.y, w, rect.h));
                    }
                }
                if let Some(text) = data.text(&key).filter(|text| !text.is_empty()) {
                    let shown = if edit.kind == EditKind::Password { "*".repeat(text.chars().count()) } else { text };
                    list.draw_text(rect.x + 2, rect.y + 2, Argb(0xFFDCDCDC), &shown, font);
                }
                clip(list, start, rect);
            }
            Kind::ChatWindow(_) => {
                if data.hidden(&key) {
                    continue;
                }
                let start = list.quads.len();
                let rows = (rect.h / CHAT_ROW).max(0) as usize;
                let mut wrapped: Vec<(Argb, i32, String)> = Vec::new();
                for (color, line) in data.lines(&key) {
                    for (indent, part) in crate::chat::wrap_line(&line, rect.w - 22, 0, font) {
                        wrapped.push((color, indent, part));
                    }
                }
                let scroll = (data.scroll(&key).max(0) as usize).min(wrapped.len().saturating_sub(rows));
                let end = wrapped.len() - scroll;
                let first = end.saturating_sub(rows);
                for (row, (color, indent, part)) in wrapped[first..end].iter().enumerate() {
                    list.draw_text(rect.x + CHAT_TEXT_X + indent, rect.y + CHAT_ROW * row as i32 + CHAT_TEXT_Y, *color, part, font);
                }
                let (sx, sy, bottom) = data.chat_scroll_bar(&key);
                let size = crate::xdat_paint_tabs::SCROLL_BUTTON_SIZE;
                let bar_h = rect.h + bottom - sy;
                let bar = crate::xdat_paint_tabs::scroll_bar((sx + size, bar_h), wrapped.len() as i32, rows as i32, first as i32);
                for (piece, texture) in [(bar.up, crate::xdat_paint_tabs::SCROLL_UP_BTN), (bar.down, crate::xdat_paint_tabs::SCROLL_DOWN_BTN)] {
                    if data.has_texture(texture) {
                        list.draw_button_texture(rect.x + piece.x, rect.y + sy + piece.y, piece.w, piece.h, 0, 0, piece.w, piece.h, texture, alpha, true);
                    }
                }
                if let Some(thumb) = bar.thumb {
                    for (piece, texture) in crate::xdat_paint_tabs::scroll_thumb_pieces(thumb) {
                        if data.has_texture(texture) && piece.h > 0 {
                            list.draw_texture(rect.x + piece.x, rect.y + sy + piece.y, piece.w, piece.h, 0, 0, piece.w, piece.h, texture, alpha);
                        }
                    }
                }
                clip(list, start, rect);
            }
            Kind::ShortcutItemWindow(item) => {
                let start = list.quads.len();
                let cell = data.items(&key).into_iter().next();
                if let Some(cell) = &cell {
                    let (iw, ih) = (rect.w - 4, rect.h - 4);
                    list.draw_texture(rect.x + 1, rect.y + 1, iw, ih, 0, 0, iw, ih, &cell.icon, alpha);
                }
                if (cell.is_some() || item.always_show_outline) && data.has_texture(SHORTCUT_OUTLINE) {
                    let outline = if pressed && hovered { SHORTCUT_OUTLINE_DOWN } else { SHORTCUT_OUTLINE };
                    list.draw_button_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, outline, alpha, true);
                }
                clip(list, start, rect);
            }
            Kind::InvenWeight => {
                let Some((load, max_load)) = data.weight() else { continue };
                crate::xdat_paint_items::paint_inven_weight(list, rect, load, max_load, 0, &|t| data.has_texture(t), font, alpha);
            }
            Kind::Tab(tab) => {
                let start = list.quads.len();
                let selected = state.tab(&placed.name);
                let labels: Vec<Option<String>> = tab
                    .tabs
                    .iter()
                    .map(|element| (element.button_name >= 0).then(|| data.sysstring(element.button_name)).flatten().filter(|label| !label.is_empty()))
                    .collect();
                let specs: Vec<crate::xdat_paint_tabs::TabSpec> = tab
                    .tabs
                    .iter()
                    .zip(&labels)
                    .map(|(element, label)| crate::xdat_paint_tabs::TabSpec {
                        width: element.width,
                        height: element.height,
                        gap: element.gap,
                        label_size: label.as_ref().map_or((0, 0), |label| (font.measure(label) as i32, font.line_height as i32)),
                    })
                    .collect();
                for ((header, element), label) in crate::xdat_paint_tabs::tab_headers(&specs, selected).iter().zip(&tab.tabs).zip(&labels) {
                    let face = crate::xdat_paint_tabs::tab_texture(&element.normal_tex, &element.pushed_tex, header.selected);
                    let face_rect = header.rect.offset(rect.x, rect.y);
                    if data.has_texture(face) {
                        list.draw_button_texture(face_rect.x, face_rect.y, face_rect.w, face_rect.h, 0, 0, face_rect.w, face_rect.h, face, alpha, true);
                    }
                    if let Some(label) = label {
                        let size = (font.measure(label) as i32, font.line_height as i32);
                        let (tx, ty) = crate::xdat_paint_tabs::tab_label_origin(header, size);
                        list.draw_text(rect.x + tx, rect.y + ty, crate::xdat_paint_tabs::TAB_LABEL_COLOR, label, font);
                    }
                }
                clip(list, start, rect);
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::Glyphs;
    use l2_xdat::{Base, Button, Percent, Size, StatusBar, TabElement, Texture};

    fn base(name: &str, size: Option<(i32, i32)>, anchor: Option<(Alignment, Alignment, &str, i32, i32)>) -> Base {
        Base {
            name: name.into(),
            extends: "undefined".into(),
            always_on_top: -1,
            always_on_back: -1,
            parent_name: String::new(),
            tab_pre_order: "undefined".into(),
            tab_post_order: "undefined".into(),
            anchored: 0,
            size: size.map(|(width, height)| Size { percent: None, width, height }),
            anchor: anchor.map(|(parent, this, ctrl, x, y)| Anchor { parent, this, ctrl: ctrl.into(), x, y }),
            virtual_: -1,
            pass_through: -1,
            unk68: 0,
            tooltip_type: "undefined".into(),
            tooltip_text: UNDEFINED,
        }
    }

    fn texture(
        name: &str,
        file: &str,
        size: Option<(i32, i32)>,
        anchor: (Alignment, Alignment, &str, i32, i32),
        kind: TextureKind,
    ) -> Node {
        Node {
            base: base(name, size, Some(anchor)),
            kind: Kind::Texture(Texture {
                file: file.into(),
                kind,
                layer: TextureLayer::Normal,
                u: 0,
                v: 0,
                u_size: 0.0,
                v_size: 0.0,
                alpha: UNDEFINED,
                anim_tex: -1,
            }),
        }
    }

    fn relative(width_rate: f32, height_rate: f32, width: i32, height: i32) -> Size {
        Size {
            percent: Some(Percent { relative_to: String::new(), width: width_rate, height: height_rate }),
            width,
            height,
        }
    }

    fn window(name: &str, size: (i32, i32), children: Vec<Node>) -> Window {
        Window {
            base: base(name, Some(size), None),
            parent: String::new(),
            back_tex: String::new(),
            script: String::new(),
            state: String::new(),
            frame: 0,
            iconable: 0,
            stuckable: 0,
            hidden: 0,
            always_full_alpha: 0,
            save_position: 0,
            save_size: 0,
            title: 0,
            resize_frame: 0,
            frame_size: 0,
            frame_direction: 0,
            frame_exit_button: 0,
            draggable: 0,
            resize_frame_direction: 0,
            resize_frame_rect: [0; 4],
            resize_max_increase: 0,
            expand_button: 0,
            expand_org_height: 0,
            expand_exp_height: 0,
            drawer_direction: 0,
            drawer_offset: 0,
            drawer_direction_fixed: 0,
            drawer_owner_window: String::new(),
            show_anim: 0,
            hide_anim: 0,
            show_anim_info: [0; 2],
            hide_anim_info: [0; 2],
            icon_name: String::new(),
            tooltip_idx: 0,
            hook_key_input: 0,
            working_configuration: String::new(),
            children,
        }
    }

    fn font() -> BitmapFont {
        BitmapFont::new(String::new(), Glyphs { tex_width: 1, tex_height: 1, glyphs: Default::default() })
    }

    struct NoData;

    impl Data for NoData {
        fn text(&self, _: &str) -> Option<String> {
            None
        }
        fn sysstring(&self, _: i32) -> Option<String> {
            None
        }
        fn items(&self, ctrl: &str) -> Vec<Cell> {
            if ctrl == "Grid" { vec![Cell { icon: "icon.a".into(), selected: false, count: None, equipped: false }; 3] } else { Vec::new() }
        }
    }

    #[test]
    fn chat_windows_paint_only_the_selected_tab_with_the_bar_from_set_scroll_bar_position() {
        struct ChatData;
        impl Data for ChatData {
            fn text(&self, _: &str) -> Option<String> {
                None
            }
            fn sysstring(&self, _: i32) -> Option<String> {
                None
            }
            fn items(&self, _: &str) -> Vec<Cell> {
                Vec::new()
            }
            fn hidden(&self, ctrl: &str) -> bool {
                ctrl.ends_with("TradeChat")
            }
            fn chat_scroll_bar(&self, _: &str) -> (i32, i32, i32) {
                (5, 10, -30)
            }
            fn lines(&self, _: &str) -> Vec<(Argb, String)> {
                (0..12).map(|i| (Argb(0xFFFFFFFF), format!("linha {i}"))).collect()
            }
        }
        let chat = |name: &str| Node {
            base: base(name, Some((343, 150)), Some((Alignment::TopLeft, Alignment::TopLeft, "", 0, 0))),
            kind: Kind::ChatWindow(l2_xdat::ChatWindow { line_gap: 0 }),
        };
        let window = window("ChatWnd", (348, 200), vec![chat("NormalChat"), chat("TradeChat")]);
        let layout = layout(&window);
        let mut list = DrawList::default();
        paint(&mut list, &window, &layout, 0, 0, &State::default(), &ChatData, &font(), 1.0);
        let up: Vec<_> = list.quads.iter().filter(|q| q.texture == crate::xdat_paint_tabs::SCROLL_UP_BTN).collect();
        let down: Vec<_> = list.quads.iter().filter(|q| q.texture == crate::xdat_paint_tabs::SCROLL_DOWN_BTN).collect();
        assert_eq!((up.len(), down.len()), (1, 1));
        assert_eq!((up[0].x, up[0].y, up[0].w, up[0].h), (5.0, 10.0, 15.0, 15.0));
        assert_eq!((down[0].x, down[0].y), (5.0, (150 - 30 - 15) as f32));
        let thumb: Vec<_> = list.quads.iter().filter(|q| q.texture == crate::xdat_paint_tabs::SCROLL_THUMB_TOP).collect();
        assert_eq!(thumb.len(), 1);
        assert_eq!(thumb[0].y, 25.0 + (80.0 - thumb_height(80, 12, 10) as f32));
    }

    fn thumb_height(track: i32, total: i32, visible: i32) -> i32 {
        crate::xdat_paint_tabs::scroll_bar((15, track + 30), total, visible, 0).thumb.map_or(0, |thumb| thumb.h)
    }

    #[test]
    fn the_first_anchor_point_is_the_windows_own_and_the_second_is_the_references_like_ncwnd_moveto() {
        let anchor = Anchor { parent: Alignment::BottomLeft, this: Alignment::TopLeft, ctrl: "ChatWnd".into(), x: 0, y: -5 };
        let chat = Rect::new(0, 560, 348, 101);
        assert_eq!(place(Some(&anchor), chat, (348, 125)), (0, 560 - 125 - 5));
        let anchor = Anchor { parent: Alignment::TopRight, this: Alignment::TopLeft, ctrl: String::new(), x: 3, y: 0 };
        assert_eq!(place(Some(&anchor), Rect::new(100, 50, 200, 80), (40, 20)), (100 - 40 + 3, 50));
    }

    #[test]
    fn same_alignment_anchor_matches_the_c4_anchor_origin_rule() {
        let modes = [
            Alignment::TopLeft,
            Alignment::TopCenter,
            Alignment::TopRight,
            Alignment::CenterLeft,
            Alignment::Center,
            Alignment::CenterRight,
            Alignment::BottomLeft,
            Alignment::BottomCenter,
            Alignment::BottomRight,
        ];
        for (mode, align) in modes.into_iter().enumerate() {
            for (parent, child, offset) in [
                ((566, 400), (39, 40), (-8, 350)),
                ((257, 131), (12, 12), (3, -5)),
                ((1024, 1024), (573, 412), (-4, -1)),
            ] {
                let expected = crate::broadcast::anchor_origin(mode as u32 + 1, offset, parent, child).unwrap();
                let anchor = Anchor { parent: align, this: align, ctrl: String::new(), x: offset.0, y: offset.1 };
                assert_eq!(place(Some(&anchor), Rect::new(0, 0, parent.0, parent.1), child), expected, "{align:?}");
            }
        }
    }

    #[test]
    fn the_status_window_pieces_follow_the_ncwnd_size_rules() {
        let wnd = window(
            "StatusWnd",
            (176, 84),
            vec![
                texture(
                    "Left",
                    "L2UI.l",
                    Some((16, 84)),
                    (Alignment::TopLeft, Alignment::TopLeft, "", 12, 0),
                    TextureKind::Stretch,
                ),
                {
                    let mut center = texture(
                        "Center",
                        "L2UI.c",
                        None,
                        (Alignment::TopLeft, Alignment::TopLeft, "", 28, 0),
                        TextureKind::Stretch,
                    );
                    center.base.size = Some(relative(1.0, 1.0, -32, 0));
                    center
                },
                texture(
                    "Right",
                    "L2UI.r",
                    Some((4, 84)),
                    (Alignment::TopRight, Alignment::TopRight, "", -4, 0),
                    TextureKind::Stretch,
                ),
            ],
        );
        let layout = layout(&wnd);
        assert_eq!(layout.find("Left").unwrap().rect, Rect::new(12, 0, 16, 84));
        assert_eq!(layout.find("Center").unwrap().rect, Rect::new(28, 0, 144, 84));
        assert_eq!(layout.find("Center").unwrap().parent, Rect::new(0, 0, 176, 84));
        assert_eq!(layout.find("Right").unwrap().rect, Rect::new(168, 0, 4, 84));
        assert!(layout.unsupported.is_empty());
        assert_eq!(window_size(&relative(1.0, 0.5, -32, 4), (176, 84)), (144, 46));
        assert_eq!(window_size(&relative(1.0, 1.0, -50, -162), (176, 84)), (126, -78));
        assert_eq!(window_size(&relative(-1.0, -1.0, 30, 30), (176, 84)), (0, 0));
        assert_eq!(window_size(&Size { percent: None, width: -32, height: 0 }, (176, 84)), (0, 0));
        assert_eq!(client_height(1, 3, 84), 64);
        assert_eq!(client_height(1, 0, 84), 64);
        assert_eq!(client_height(1, 1, 84), 84);
        assert_eq!(client_height(0, 3, 84), 84);
    }

    #[test]
    fn none_alignment_counts_as_top_left_and_a_missing_anchor_sits_at_the_reference_origin() {
        let anchor = Anchor { parent: Alignment::None, this: Alignment::TopLeft, ctrl: String::new(), x: 91, y: 52 };
        assert_eq!(place(Some(&anchor), Rect::new(10, 20, 566, 400), (0, 0)), (101, 72));
        let centered = Anchor { parent: Alignment::None, this: Alignment::Center, ctrl: String::new(), x: 0, y: 0 };
        assert_eq!(place(Some(&centered), Rect::new(0, 0, 100, 100), (10, 10)), (45, 45));
        assert_eq!(place(None, Rect::new(10, 20, 566, 400), (30, 30)), (10, 20));
    }

    #[test]
    fn layout_resolves_control_references_and_window_relative_anchors() {
        let container = Node {
            base: base("Container", Some((1024, 1024)), Some((Alignment::Center, Alignment::Center, "", 0, 0))),
            kind: Kind::Window(Box::new(window(
                "Container",
                (1024, 1024),
                vec![
                    texture(
                        "Tex",
                        "Was.Frame",
                        Some((573, 412)),
                        (Alignment::BottomLeft, Alignment::BottomLeft, "InventoryWnd", -4, -1),
                        TextureKind::Stretch,
                    ),
                    texture(
                        "Frame",
                        "Was.Outline",
                        None,
                        (Alignment::TopLeft, Alignment::TopLeft, "InventoryWnd", 211, 49),
                        TextureKind::Normal,
                    ),
                ],
            ))),
        };
        let close = Node {
            base: base("BtnClose", Some((12, 12)), Some((Alignment::TopRight, Alignment::TopRight, "Tex", -18, 18))),
            kind: Kind::Button(Button {
                normal_tex: "Was.Close".into(),
                pushed_tex: "undefined".into(),
                highlight_tex: "undefined".into(),
                drop_tex: "undefined".into(),
                button_name: UNDEFINED,
                no_highlight: -1,
                default_sound_on: -1,
                disable_time: UNDEFINED,
            }),
        };
        let wnd = window("InventoryWnd", (566, 400), vec![container, close]);
        let layout = layout(&wnd);
        assert_eq!(layout.size, (566, 400));
        assert_eq!(layout.find("Container").unwrap().rect, Rect::new(-229, -312, 1024, 1024));
        assert_eq!(layout.find("Tex").unwrap().rect, Rect::new(-4, -13, 573, 412));
        assert_eq!(layout.find("Frame").unwrap().rect, Rect::new(211, 49, 0, 0));
        assert_eq!(layout.find("BtnClose").unwrap().rect, Rect::new(-4 + 573 - 12 - 18, -13 + 18, 12, 12));
        assert_eq!(layout.find("BtnClose").unwrap().path, vec![1]);
        assert_eq!(layout.find("Frame").unwrap().path, vec![0, 1]);
    }

    #[test]
    fn paint_draws_textures_buttons_tabs_and_only_the_selected_tab_target() {
        let grid = |name: &str| Node {
            base: base(name, Some((100, 36)), Some((Alignment::TopLeft, Alignment::TopLeft, "", 10, 40))),
            kind: Kind::ItemWindow(Box::new(ItemWindow {
                col: 3,
                row: 1,
                max_item_num: 3,
                icon_width: 32,
                icon_height: 32,
                gap_x: 2,
                gap_y: 2,
                offset_x: UNDEFINED,
                offset_y: UNDEFINED,
                background_item_width: UNDEFINED,
                background_item_height: UNDEFINED,
                background_item_tex: "undefined".into(),
                selected_item_width: UNDEFINED,
                selected_item_height: UNDEFINED,
                selected_item_tex: "undefined".into(),
                unselected_item_width: UNDEFINED,
                unselected_item_height: UNDEFINED,
                unselected_item_tex: "undefined".into(),
                no_select_item: -1,
                no_item_drag: -1,
                button_click: -1,
                use_cool_time: -1,
                no_scroll: -1,
                out_line_up: "undefined".into(),
                out_line_down: "undefined".into(),
            })),
        };
        let tab = Node {
            base: base("Tabs", Some((120, 23)), Some((Alignment::TopLeft, Alignment::TopLeft, "", 10, 10))),
            kind: Kind::Tab(Tab {
                tabs: vec![
                    TabElement {
                        button_name: UNDEFINED,
                        target: "Grid".into(),
                        width: 55,
                        height: 28,
                        normal_tex: "Was.TabA".into(),
                        pushed_tex: "Was.TabA_on".into(),
                        movable: false,
                        gap: 1,
                        tooltip: -1,
                        no_highlight: -1,
                    },
                    TabElement {
                        button_name: UNDEFINED,
                        target: "Other".into(),
                        width: 55,
                        height: 28,
                        normal_tex: "Was.TabB".into(),
                        pushed_tex: "Was.TabB_on".into(),
                        movable: false,
                        gap: 1,
                        tooltip: -1,
                        no_highlight: -1,
                    },
                ],
            }),
        };
        let back = texture(
            "Back",
            "Was.Back",
            Some((200, 100)),
            (Alignment::TopLeft, Alignment::TopLeft, "", 0, 0),
            TextureKind::Stretch,
        );
        let wnd = window("W", (200, 100), vec![back, tab, grid("Grid"), grid("Other")]);
        let layout = layout(&wnd);
        let mut list = DrawList::default();
        let state = State::default();
        paint(&mut list, &wnd, &layout, 5, 7, &state, &NoData, &font(), 1.0);
        let names: Vec<&str> = list.quads.iter().map(|q| q.texture.as_str()).collect();
        assert_eq!(names, ["Was.Back", "Was.TabA_on", "Was.TabB", "icon.a", "icon.a", "icon.a"]);
        let back = &list.quads[0];
        assert_eq!((back.x, back.y, back.w, back.h, back.uw, back.vh), (5.0, 7.0, 200.0, 100.0, 200.0, 100.0));
        assert_eq!((list.quads[2].x, list.quads[3].x, list.quads[4].x), (5.0 + 10.0 + 56.0, 16.0, 50.0));
        assert_eq!(list.buttons.len(), 2);
        assert_eq!(layout.hit(&wnd, &state, 20, 60).map(|p| p.name.as_str()), Some("Grid"));
        assert_eq!(tab_at(&Tab { tabs: Vec::new() }, Rect::new(0, 0, 0, 0), 0, 0), None);
        let Kind::Tab(tab) = &wnd.children[1].kind else { unreachable!() };
        assert_eq!(tab_at(tab, layout.find("Tabs").unwrap().rect, 70, 20), Some(1));
        let Kind::ItemWindow(item) = &wnd.children[2].kind else { unreachable!() };
        assert_eq!(cell_at(item, layout.find("Grid").unwrap().rect, 10 + 34 + 5, 45), Some(1));
        assert_eq!(cell_at(item, layout.find("Grid").unwrap().rect, 10 + 33, 45), None);
    }

    #[test]
    fn a_framed_parent_with_frame_direction_three_loses_twenty_pixels_for_relative_heights() {
        let mut bar = texture("Bar", "L2UI.b", None, (Alignment::TopLeft, Alignment::TopLeft, "", 16, 27), TextureKind::Stretch);
        bar.base.size = Some(relative(1.0, 1.0, -26, -10));
        let mut wnd = window("W", (176, 84), vec![bar]);
        wnd.frame = 1;
        wnd.frame_direction = 3;
        assert_eq!(layout(&wnd).find("Bar").unwrap().rect, Rect::new(16, 27, 150, 54));
        wnd.frame_direction = 1;
        assert_eq!(layout(&wnd).find("Bar").unwrap().rect, Rect::new(16, 27, 150, 74));
    }

    #[test]
    fn textures_paint_like_nctexturectrl_onpaint() {
        let mut tile = texture("Tile", "Was.Tile", Some((30, 20)), (Alignment::TopLeft, Alignment::TopLeft, "", 0, 0), TextureKind::Tile);
        if let Kind::Texture(texture) = &mut tile.kind {
            texture.u_size = 8.0;
            texture.v_size = 8.0;
        }
        let mut normal = texture("Normal", "Was.Normal", Some((50, 50)), (Alignment::TopLeft, Alignment::TopLeft, "", 0, 25), TextureKind::Normal);
        if let Kind::Texture(texture) = &mut normal.kind {
            texture.u = 4;
            texture.v = 6;
            texture.u_size = 16.0;
            texture.v_size = 12.0;
        }
        let edge = texture("Edge", "Was.Edge", Some((20, 10)), (Alignment::TopLeft, Alignment::TopLeft, "", 90, 0), TextureKind::Stretch);
        let mask = texture("Mask", "Was.Mask", Some((20, 10)), (Alignment::TopLeft, Alignment::TopLeft, "", 0, 0), TextureKind::Mask);
        let wnd = window("W", (100, 50), vec![tile, normal, edge, mask]);
        let layout = layout(&wnd);
        let mut list = DrawList::default();
        paint(&mut list, &wnd, &layout, 0, 0, &State::default(), &NoData, &font(), 1.0);
        let tiles: Vec<&crate::draw::Quad> = list.quads.iter().filter(|q| q.texture.as_str() == "Was.Tile").collect();
        assert_eq!(tiles.len(), 12);
        assert_eq!((tiles[0].x, tiles[0].y, tiles[0].w, tiles[0].h, tiles[0].uw, tiles[0].vh), (0.0, 0.0, 8.0, 8.0, 8.0, 8.0));
        assert_eq!((tiles[3].x, tiles[3].w, tiles[3].uw), (24.0, 6.0, 6.0));
        assert_eq!((tiles[11].x, tiles[11].y, tiles[11].w, tiles[11].h), (24.0, 16.0, 6.0, 4.0));
        let normal = list.quads.iter().find(|q| q.texture.as_str() == "Was.Normal").unwrap();
        assert_eq!((normal.x, normal.y, normal.w, normal.h, normal.u, normal.v, normal.uw, normal.vh), (0.0, 25.0, 16.0, 12.0, 4.0, 6.0, 16.0, 12.0));
        let edge = list.quads.iter().find(|q| q.texture.as_str() == "Was.Edge").unwrap();
        assert_eq!((edge.x, edge.w, edge.uw), (90.0, 10.0, 10.0));
        assert!(list.quads.iter().all(|q| q.texture.as_str() != "Was.Mask"));
    }

    struct BarData;

    impl Data for BarData {
        fn text(&self, _: &str) -> Option<String> {
            None
        }
        fn sysstring(&self, _: i32) -> Option<String> {
            None
        }
        fn items(&self, _: &str) -> Vec<Cell> {
            Vec::new()
        }
        fn bar(&self, ctrl: &str) -> Option<BarPoint> {
            match ctrl {
                "HPBar" => Some(BarPoint::point(30, 200)),
                "EXPBars" => BarPoint::exp(100, 2),
                _ => None,
            }
        }
    }

    #[test]
    fn status_bars_paint_like_ncstatusbarctrl_onpaint() {
        let bar = |name: &str, title: &str, y: i32| {
            let mut node = Node {
                base: base(name, None, Some((Alignment::TopLeft, Alignment::TopLeft, "", 16, y))),
                kind: Kind::StatusBar(StatusBar {
                    title: title.into(),
                    texture_width: 8,
                    texture_height: 12,
                    fore_texture: "Was.fore".into(),
                    back_texture: "Was.back".into(),
                    warn_texture: "Was.warn".into(),
                    regen_texture: "Was.regen".into(),
                }),
            };
            node.base.size = Some(relative(1.0, 1.0, -26, -72));
            node
        };
        let wnd = window("StatusWnd", (176, 84), vec![bar("HPBar", "HP", 41), bar("EXPBars", "EXP", 69), bar("MPBar", "MP", 55)]);
        let layout = layout(&wnd);
        assert_eq!(layout.find("HPBar").unwrap().rect, Rect::new(16, 41, 150, 12));
        let mut list = DrawList { capture_text: true, ..Default::default() };
        let font = font();
        paint(&mut list, &wnd, &layout, 0, 0, &State::default(), &BarData, &font, 1.0);
        let fill = (30.0f64 * 150.0 / 200.0) as i32;
        assert_eq!(fill, 22);
        let hp: Vec<(&str, f32, f32, f32, f32)> = list
            .quads
            .iter()
            .filter(|q| q.y < 60.0)
            .map(|q| (q.texture.as_str(), q.x, q.y, q.w, q.h))
            .collect();
        assert_eq!(hp[0], ("Was.back", 16.0, 41.0, 150.0, 12.0));
        assert_eq!((list.quads[0].uw, list.quads[0].vh), (8.0, 12.0));
        assert_eq!(hp[1], ("Was.fore", 16.0, 41.0, 22.0, 12.0));
        assert_eq!(hp[2], ("Was.warn", 16.0, 41.0, 22.0, 12.0));
        assert_eq!(hp[3], (crate::draw::DIGIT_TEXTURE, 25.0, 43.0, 8.0, 8.0));
        assert_eq!(hp[4].1, 33.0);
        let half = 75.0f32;
        let slash = (half - (font.measure("/") as i32 / 2) as f32) as i32;
        assert_eq!(hp[5], (crate::draw::DIGIT_TEXTURE, 16.0 + slash as f32, 43.0, 8.0, 8.0));
        let current = (half - font.measure("30") as f32 - 8.0) as i32;
        assert_eq!(hp[6].1, 16.0 + current as f32);
        assert_eq!(hp[8].1, 16.0 + (half + 4.0));
        let texts: Vec<&str> = list.text_runs.iter().map(|run| run.text.as_str()).collect();
        assert_eq!(texts[..4], ["HP", "/", "30", "200"]);
        assert_eq!(texts[4], "10.85%");
        let exp: Vec<&crate::draw::Quad> = list.quads.iter().filter(|q| q.y >= 69.0 && q.y < 81.0).collect();
        assert_eq!(exp[0].texture.as_str(), "Was.back");
        assert_eq!((exp[1].texture.as_str(), exp[1].w), ("Was.fore", ((32.0f64 * 150.0) / 295.0) as i32 as f32));
        assert_eq!((exp[2].texture.as_str(), exp[2].w), ("Was.warn", exp[1].w));
        let percent = (half - (font.measure("10.85%") as i32 / 2) as f32) as i32;
        assert_eq!((exp[3].texture.as_str(), exp[3].x), (crate::draw::DIGIT_TEXTURE, 16.0 + percent as f32));
        let mp: Vec<&crate::draw::Quad> = list.quads.iter().filter(|q| q.y >= 55.0 && q.y < 67.0).collect();
        assert_eq!(mp[0].texture.as_str(), "Was.back");
        assert_eq!(mp[1].texture.as_str(), crate::draw::DIGIT_TEXTURE);
        assert_eq!(texts[5..], ["MP", "/", "0", "0"]);
    }

    fn glyph_font(widths: &[(char, u32)]) -> BitmapFont {
        let glyphs = widths
            .iter()
            .map(|(c, w)| (*c, crate::font::GlyphRect { x: 0, width: *w, y: 0, height: 10 }))
            .collect();
        BitmapFont::new(String::new(), Glyphs { tex_width: 64, tex_height: 16, glyphs })
    }

    struct TextData;

    impl Data for TextData {
        fn text(&self, ctrl: &str) -> Option<String> {
            (ctrl == "Level").then(|| "42".to_string())
        }
        fn sysstring(&self, id: i32) -> Option<String> {
            (id == 7).then(|| "Inventory".to_string())
        }
        fn items(&self, _: &str) -> Vec<Cell> {
            Vec::new()
        }
        fn name(&self, ctrl: &str) -> Option<(String, TextAlign, Argb)> {
            (ctrl == "UserName").then(|| ("Abcdefgh".to_string(), TextAlign::Center, Argb(0xFFE6994D)))
        }
    }

    fn text_box(name: &str, size: (i32, i32), at: (i32, i32), align: TextAlign, text: &str) -> Node {
        Node {
            base: base(name, Some(size), Some((Alignment::TopLeft, Alignment::TopLeft, "", at.0, at.1))),
            kind: Kind::TextBox(l2_xdat::TextBox {
                text: text.into(),
                align,
                offset_y: 3,
                back_tex: "undefined".into(),
                font_type: 0,
                sysstring: -1,
                system_msg: -1,
                text_color: 0xFF112233,
                emoticon: -1,
                autosize: -1,
            }),
        }
    }

    #[test]
    fn text_boxes_paint_like_nctextbox_paintsingleline() {
        let font = glyph_font(&[('4', 6), ('2', 6), ('a', 5), ('b', 7), ('I', 3), ('n', 5), ('v', 5), ('e', 5), ('t', 3), ('o', 5), ('r', 4), ('y', 5)]);
        let mut back = text_box("Back", (40, 12), (10, 10), TextAlign::Center, "ab");
        if let Kind::TextBox(text_box) = &mut back.kind {
            text_box.back_tex = "Was.tb".into();
        }
        let right = text_box("Right", (40, 12), (10, 30), TextAlign::Right, "ab");
        let level = text_box("Level", (22, 20), (60, 10), TextAlign::Left, "undefined");
        let mut digits = text_box("Digits", (40, 12), (60, 30), TextAlign::Left, "12");
        if let Kind::TextBox(text_box) = &mut digits.kind {
            text_box.font_type = 10;
        }
        let mut sys = text_box("Sys", (60, 12), (10, 50), TextAlign::Left, "undefined");
        if let Kind::TextBox(text_box) = &mut sys.kind {
            text_box.sysstring = 7;
        }
        let wnd = window("W", (120, 80), vec![back, right, level, digits, sys]);
        let layout = layout(&wnd);
        let mut list = DrawList { capture_text: true, ..Default::default() };
        paint(&mut list, &wnd, &layout, 0, 0, &State::default(), &TextData, &font, 1.0);
        assert_eq!((list.quads[0].texture.as_str(), list.quads[0].x, list.quads[0].w), ("Was.tb", 10.0, 40.0));
        let runs: Vec<(&str, f32, f32)> = list.text_runs.iter().map(|run| (run.text.as_str(), run.rect[0], run.rect[1])).collect();
        assert_eq!(runs[0], ("ab", 10.0 + (20.0f32 - 6.0) as i32 as f32, 10.0));
        assert_eq!(runs[1], ("ab", 10.0 + 27.0, 30.0));
        assert_eq!(runs[2], ("42", 60.0, 10.0));
        assert_eq!(runs[3], ("12", 60.0, 33.0));
        assert_eq!(runs[4], ("Inventory", 10.0, 50.0));
        assert_eq!(list.text_runs[0].color, Argb(0xFF112233).rgba());
    }

    #[test]
    fn name_controls_center_and_shorten_like_ncnamectrl_drawname() {
        let font = glyph_font(&[('A', 10), ('b', 10), ('c', 10), ('d', 10), ('e', 10), ('f', 10), ('g', 10), ('h', 10), ('.', 3)]);
        assert_eq!(name_with_dots("Abcdefgh", 100, &font), "Abcdefgh");
        assert_eq!(name_with_dots("Abcdefgh", 80, &font), "Abcdefgh");
        assert_eq!(name_with_dots("Abcdefgh", 79, &font), "Abcdefg..");
        assert_eq!(name_with_dots("Abcdefgh", 50, &font), "Abcd..");
        assert_eq!(name_with_dots("Abcdefgh", 5, &font), "..");
        let mut name = Node {
            base: base("UserName", None, Some((Alignment::TopCenter, Alignment::TopCenter, "", 0, 6))),
            kind: Kind::NameCtrl,
        };
        name.base.size = Some(relative(1.0, 1.0, -50, -162));
        let wnd = window("StatusWnd", (176, 84), vec![name]);
        let layout = layout(&wnd);
        assert_eq!(layout.find("UserName").unwrap().rect, Rect::new(25, 6, 126, -78));
        let mut list = DrawList { capture_text: true, ..Default::default() };
        paint(&mut list, &wnd, &layout, 0, 0, &State::default(), &TextData, &font, 1.0);
        assert_eq!(list.text_runs.len(), 1);
        let run = &list.text_runs[0];
        assert_eq!((run.text.as_str(), run.rect[0], run.rect[1]), ("Abcdefgh", 25.0 + (63.0f32 - 40.0) as i32 as f32, 6.0));
        assert_eq!(run.color, Argb(0xFFE6994D).rgba());
    }

    #[test]
    fn framed_windows_paint_the_ncframectrl_bar_and_the_back_texture() {
        let mut wnd = window("StatusWnd", (176, 84), Vec::new());
        wnd.frame = 1;
        wnd.frame_size = 1;
        wnd.frame_direction = 1;
        assert_eq!(frame_ctrl_rect(&wnd, (176, 84)), Some(Rect::new(0, 0, 12, 84)));
        let layout = layout(&wnd);
        let mut list = DrawList::default();
        paint(&mut list, &wnd, &layout, 3, 4, &State::default(), &NoData, &font(), 1.0);
        let quads: Vec<(&str, f32, f32, f32, f32, f32, f32)> =
            list.quads.iter().map(|q| (q.texture.as_str(), q.x, q.y, q.w, q.h, q.uw, q.vh)).collect();
        assert_eq!(quads[0], (SMALLBAR1, 3.0, 4.0, 12.0, 8.0, 12.0, 8.0));
        assert_eq!(quads[1], (SMALLBAR2, 3.0, 12.0, 12.0, 68.0, 12.0, 8.0));
        assert_eq!(quads[2], (SMALLBAR3, 3.0, 80.0, 12.0, 8.0, 12.0, 8.0));
        let mut titled = window("Titled", (200, 100), Vec::new());
        titled.frame = 1;
        titled.frame_size = 0;
        titled.frame_direction = 3;
        titled.title = 7;
        titled.back_tex = "Was.Back".into();
        assert_eq!(frame_ctrl_rect(&titled, (200, 100)), Some(Rect::new(0, 0, 200, 20)));
        assert_eq!(frame_offsets(1, 0, 3), (0, 20));
        assert_eq!(frame_offsets(1, 1, 1), (12, 0));
        assert_eq!(frame_offsets(0, 0, 3), (0, 0));
        let titled_layout = super::layout(&titled);
        let mut list = DrawList { capture_text: true, ..Default::default() };
        let font = glyph_font(&[('I', 3), ('n', 5), ('v', 5), ('e', 5), ('t', 3), ('o', 5), ('r', 4), ('y', 5)]);
        paint(&mut list, &titled, &titled_layout, 0, 0, &State::default(), &TextData, &font, 1.0);
        let names: Vec<&str> = list.quads.iter().map(|q| q.texture.as_str()).collect();
        assert_eq!(&names[..2], &[FRAME_BACK_LEFT, FRAME_BACK_RIGHT]);
        let mids: Vec<&crate::draw::Quad> = list.quads.iter().filter(|q| q.texture.as_str() == FRAME_BACK_MID).collect();
        assert_eq!(mids.len(), 6);
        assert_eq!((mids[0].x, mids[0].w, mids[0].uw), (16.0, 32.0, 32.0));
        assert_eq!((mids[5].x, mids[5].w, mids[5].uw), (176.0, 8.0, 8.0));
        let title = &list.text_runs[0];
        assert_eq!((title.text.as_str(), title.rect[0], title.rect[1]), ("Inventory", 20.0, 7.0));
        assert_eq!(title.color, FRAME_TITLE_COLOR.rgba());
        let back = list.quads.iter().find(|q| q.texture.as_str() == "Was.Back").unwrap();
        assert_eq!((back.x, back.y, back.w, back.h, back.uw, back.vh), (0.0, 20.0, 200.0, 80.0, 200.0, 80.0));
        let unframed = window("Plain", (200, 100), Vec::new());
        assert_eq!(frame_ctrl_rect(&unframed, (200, 100)), None);
        assert_eq!(frame_close_button(&titled, (200, 100)), None);
        titled.frame_exit_button = 1;
        titled.iconable = 1;
        assert_eq!(frame_close_button(&titled, (200, 100)), Some(Rect::new(177, 3, 15, 15)));
        assert_eq!(frame_mini_button(&titled, (200, 100)), Some(Rect::new(160, 3, 15, 15)));
        titled.frame_exit_button = 0;
        assert_eq!(frame_mini_button(&titled, (200, 100)), Some(Rect::new(177, 3, 15, 15)));
    }

    struct GaugeData;

    impl Data for GaugeData {
        fn text(&self, _: &str) -> Option<String> {
            None
        }
        fn sysstring(&self, _: i32) -> Option<String> {
            None
        }
        fn items(&self, _: &str) -> Vec<Cell> {
            Vec::new()
        }
        fn bar(&self, ctrl: &str) -> Option<BarPoint> {
            match ctrl {
                "Gauge" => Some(BarPoint::point(30, 90)),
                "Cast" => Some(BarPoint::point(250, 1000)),
                _ => None,
            }
        }
    }

    #[test]
    fn bar_and_progress_controls_paint_like_ncbarctrl_and_ncprogressctrl() {
        let gauge = Node {
            base: base("Gauge", Some((120, 10)), Some((Alignment::TopLeft, Alignment::TopLeft, "", 5, 5))),
            kind: Kind::BarCtrl(l2_xdat::BarCtrl {
                fore_texture: "Was.fore".into(),
                back_texture: "Was.back".into(),
                u_size: 16,
                v_size: 8,
            }),
        };
        let cast = Node {
            base: base("Cast", Some((100, 14)), Some((Alignment::TopLeft, Alignment::TopLeft, "", 5, 30))),
            kind: Kind::Progress(l2_xdat::Progress {
                back_texture: "Was.pback".into(),
                bar_texture: "Was.pbar".into(),
                gap: 2,
            }),
        };
        let wnd = window("W", (200, 60), vec![gauge, cast]);
        let layout = layout(&wnd);
        let mut list = DrawList::default();
        paint(&mut list, &wnd, &layout, 0, 0, &State::default(), &GaugeData, &font(), 1.0);
        let quads: Vec<(&str, f32, f32, f32, f32, f32, f32)> =
            list.quads.iter().map(|q| (q.texture.as_str(), q.x, q.y, q.w, q.h, q.uw, q.vh)).collect();
        assert_eq!(quads[0], ("Was.back", 5.0, 5.0, 120.0, 10.0, 16.0, 8.0));
        assert_eq!(quads[1], ("Was.fore", 5.0, 5.0, 40.0, 10.0, 16.0, 8.0));
        assert_eq!(quads[2], ("Was.pback", 5.0, 30.0, 100.0, 14.0, 100.0, 14.0));
        assert_eq!(quads[3], ("Was.pbar", 7.0, 32.0, 72.0, 10.0, 100.0, 14.0));
    }

    #[test]
    fn extends_inherits_size_anchor_and_undefined_fields_like_the_xml_getters() {
        let first = texture("F1Tex", "Was.F1", Some((16, 16)), (Alignment::TopLeft, Alignment::TopLeft, "", 34, 6), TextureKind::Stretch);
        let mut second = texture("F2Tex", "undefined", None, (Alignment::None, Alignment::TopLeft, "", 71, 6), TextureKind::Stretch);
        second.base.extends = "F1Tex".into();
        if let Kind::Texture(texture) = &mut second.kind {
            texture.u = UNDEFINED;
            texture.v = UNDEFINED;
            texture.u_size = UNDEFINED as f32;
            texture.v_size = UNDEFINED as f32;
        }
        let mut third = texture("F3Tex", "undefined", None, (Alignment::None, Alignment::TopLeft, "", 108, 6), TextureKind::Stretch);
        third.base.extends = "F2Tex".into();
        let mut wnd = window("W", (512, 46), vec![first, second, third]);
        resolve_extends(&mut wnd);
        let layout = layout(&wnd);
        assert_eq!(layout.find("F2Tex").unwrap().rect, Rect::new(71, 6, 16, 16));
        assert_eq!(layout.find("F3Tex").unwrap().rect, Rect::new(108, 6, 16, 16));
        let Kind::Texture(texture) = &wnd.children[2].kind else { unreachable!() };
        assert_eq!((texture.file.as_str(), texture.u, texture.u_size), ("Was.F1", 0, 0.0));
    }
}
