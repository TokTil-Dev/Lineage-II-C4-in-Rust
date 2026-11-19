use crate::color::Argb;
use crate::draw::DrawList;
use crate::font::BitmapFont;
use crate::window::Rect;

pub const HTML_TEXT_COLOR: Argb = Argb(0xFFDCDCDC);
pub const HTML_LINK_COLOR: Argb = Argb(0xFF6699FF);
pub const HTML_LEVEL_COLOR: Argb = Argb(0xFFFFCC00);
pub const HTML_LINE_PADDING: i32 = 6;
pub const NPC_HTML_BACK: &str = "L2UI.etcwnd.NPCHtml_BACK";
pub const NPC_HTML_BACK_TOP: i32 = 0x14;
pub const NPC_WND_BACK: &str = "L2UI_ch3.NpcWnd.Npc1_back";
pub const NPC_VIEWER_SIZE: (i32, i32) = (0x136, 0x191);
pub const NPC_VIEWER_Y_OFFSET: f64 = 252.0;
pub const NPC_CONTENT_POS: (i32, i32) = (7, 0x1e);
pub const NPC_CONTENT_INSET: (i32, i32) = (14, 37);
pub const NPC_FRAME_HEIGHT: i32 = 0x14;
pub const NPC_TITLE_SYSSTRING: u32 = 0x1bc;
pub const HTML_ALIGN_MARGIN: i32 = 16;
pub const DIALOG_TEXT_COLOR: Argb = Argb(0xFFDCDCDC);
pub const DIALOG_PRICE_COLOR: Argb = Argb(0xFFE4CA7F);
pub const WARNING_ICON_RECT: Rect = Rect { x: 0xb, y: 0xf, w: 0x18, h: 0x18 };
pub const WARNING_TEXT_POS: (i32, i32) = (0x29, 0x10);
pub const WARNING_TEXT_WIDTH: i32 = 200;
pub const GARA_ICON_RECT: Rect = Rect { x: 0xb, y: 8, w: 0x18, h: 0x18 };
pub const GARA_TEXT_POS: (i32, i32) = (0x29, 9);
pub const GARA_TEXT_INSET: i32 = 0x30;
pub const GARA_PRICE_Y: i32 = 0x39;
pub const SLIDER_WIDTH: i32 = 0xf;
pub const SLIDER_CAP: i32 = 8;
pub const EDIT_PIECE: i32 = 8;
pub const EDIT_PIECE_HEIGHT: i32 = 0x11;
pub const EDIT_TEXT_POS: (i32, i32) = (2, 2);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HAlign {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VAlign {
    #[default]
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HtmlWidth {
    Pixels(i32),
    Percent(i32),
}

impl HtmlWidth {
    pub fn resolve(self, parent: i32) -> i32 {
        match self {
            HtmlWidth::Pixels(px) => px,
            HtmlWidth::Percent(ratio) => ratio * parent / 100,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HtmlText {
    pub text: String,
    pub color: Option<Argb>,
    pub link: bool,
    pub action: Option<String>,
    pub font_type: i32,
    pub align: HAlign,
    pub min_width: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlImage {
    pub texture: String,
    pub width: i32,
    pub height: i32,
    pub align: HAlign,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HtmlButton {
    pub label: String,
    pub back: String,
    pub fore: String,
    pub width: i32,
    pub height: i32,
    pub align: HAlign,
    pub action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HtmlCell {
    pub width: Option<HtmlWidth>,
    pub align: HAlign,
    pub valign: VAlign,
    pub children: Vec<HtmlNode>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HtmlTable {
    pub border: i32,
    pub cellpadding: i32,
    pub cellspacing: i32,
    pub width: Option<HtmlWidth>,
    pub height: Option<i32>,
    pub background: Option<Argb>,
    pub align: HAlign,
    pub rows: Vec<Vec<HtmlCell>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HtmlNode {
    Text(HtmlText),
    Break,
    Image(HtmlImage),
    Button(HtmlButton),
    Table(HtmlTable),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextSpan {
    pub text: String,
    pub color: Argb,
    pub action: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Placed {
    Text { rect: Rect, lines: Vec<Vec<TextSpan>>, font_type: i32 },
    Image { rect: Rect, texture: String },
    Button { rect: Rect, label: String, back: String, fore: String, action: Option<String> },
    Table(TableLayout),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CellLayout {
    pub rect: Rect,
    pub content: FrameLayout,
    pub valign_offset: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableLayout {
    pub rect: Rect,
    pub border: i32,
    pub background: Option<Argb>,
    pub cells: Vec<CellLayout>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct FrameLayout {
    pub width: i32,
    pub height: i32,
    pub items: Vec<Placed>,
}

pub fn parse_color(code: &str) -> Argb {
    if code == "LEVEL" {
        return HTML_LEVEL_COLOR;
    }
    u32::from_str_radix(code.trim_start_matches("0x").trim_start_matches("0X"), 16)
        .map_or(HTML_TEXT_COLOR, |rgb| Argb(0xFF00_0000 | (rgb & 0x00FF_FFFF)))
}

pub fn wrap_text(text: &str, width: i32, font: &BitmapFont) -> Vec<String> {
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut line = String::new();
        for word in paragraph.split(' ') {
            let candidate = if line.is_empty() { word.to_string() } else { format!("{line} {word}") };
            if line.is_empty() || font.measure(&candidate) as i32 <= width {
                line = candidate;
            } else {
                lines.push(std::mem::take(&mut line));
                line = word.to_string();
            }
        }
        lines.push(line);
    }
    lines
}

fn run_spans(run: &[&HtmlText]) -> Vec<TextSpan> {
    run.iter()
        .map(|text| TextSpan {
            text: text.text.clone(),
            color: text.color.unwrap_or(if text.link { HTML_LINK_COLOR } else { HTML_TEXT_COLOR }),
            action: text.action.clone(),
        })
        .collect()
}

fn split_lines(spans: &[TextSpan], width: i32, font: &BitmapFont) -> Vec<Vec<TextSpan>> {
    let joined: String = spans.iter().map(|span| span.text.as_str()).collect();
    let lines = wrap_text(&joined, width, font);
    let mut out = Vec::new();
    let mut cursor = 0usize;
    for line in lines {
        let mut remaining = line.chars().count();
        let mut row = Vec::new();
        while remaining > 0 {
            let mut offset = 0usize;
            let mut found = None;
            for span in spans {
                let len = span.text.chars().count();
                if cursor < offset + len {
                    found = Some((span, cursor - offset));
                    break;
                }
                offset += len;
            }
            let Some((span, start)) = found else { break };
            let take = (span.text.chars().count() - start).min(remaining);
            let piece: String = span.text.chars().skip(start).take(take).collect();
            row.push(TextSpan { text: piece, color: span.color, action: span.action.clone() });
            cursor += take;
            remaining -= take;
        }
        cursor += 1;
        out.push(row);
    }
    out
}

fn text_height(lines: usize, font: &BitmapFont) -> i32 {
    lines as i32 * font.line_height as i32 + HTML_LINE_PADDING
}

fn aligned_x(align: HAlign, width: i32, w: i32) -> i32 {
    match align {
        HAlign::Left => 0,
        HAlign::Center => ((width - w) / 2).max(0),
        HAlign::Right => width - w,
    }
}

pub fn layout_frame(children: &[HtmlNode], width: i32, font: &BitmapFont) -> FrameLayout {
    let mut items = Vec::new();
    let mut y = 0;
    let mut max_width = width;
    let mut index = 0;
    while index < children.len() {
        match &children[index] {
            HtmlNode::Text(first) => {
                let mut run = vec![first];
                let mut next = index + 1;
                while let Some(HtmlNode::Text(text)) = children.get(next) {
                    if text.align != first.align {
                        break;
                    }
                    run.push(text);
                    next += 1;
                }
                let line_width = width.max(first.min_width);
                let lines = split_lines(&run_spans(&run), line_width, font);
                let measured = lines
                    .iter()
                    .map(|line| line.iter().map(|span| font.measure(&span.text) as i32).sum::<i32>())
                    .max()
                    .unwrap_or(0);
                let h = text_height(lines.len(), font);
                let x = aligned_x(first.align, width, measured);
                items.push(Placed::Text { rect: Rect::new(x, y, measured, h), lines, font_type: first.font_type });
                max_width = max_width.max(measured);
                y += h;
                index = next;
            }
            HtmlNode::Break => {
                y += text_height(1, font);
                index += 1;
            }
            HtmlNode::Image(image) => {
                let x = aligned_x(image.align, width, image.width);
                items.push(Placed::Image {
                    rect: Rect::new(x, y, image.width, image.height),
                    texture: image.texture.clone(),
                });
                y += image.height;
                index += 1;
            }
            HtmlNode::Button(button) => {
                let x = aligned_x(button.align, width, button.width);
                items.push(Placed::Button {
                    rect: Rect::new(x, y, button.width, button.height),
                    label: button.label.clone(),
                    back: button.back.clone(),
                    fore: button.fore.clone(),
                    action: button.action.clone(),
                });
                y += button.height;
                index += 1;
            }
            HtmlNode::Table(table) => {
                let mut layout = layout_table(table, width, font);
                let x = aligned_x(table.align, width, layout.rect.w);
                layout.rect.x = x;
                layout.rect.y = y;
                y += layout.rect.h;
                max_width = max_width.max(layout.rect.w);
                items.push(Placed::Table(layout));
                index += 1;
            }
        }
    }
    FrameLayout { width: max_width, height: y, items }
}

pub fn layout_table(table: &HtmlTable, parent_width: i32, font: &BitmapFont) -> TableLayout {
    let pad = table.cellpadding;
    let spacing = table.cellspacing;
    let border = table.border;
    let fixed = table.width.map(|width| width.resolve(parent_width));
    let edge = pad + border * 2 + spacing;
    let mut cells = Vec::new();
    let mut y = edge;
    let mut widest = 0;
    let columns = table.rows.iter().map(Vec::len).max().unwrap_or(0);
    let column_widths: Vec<i32> = (0..columns)
        .map(|column| {
            table
                .rows
                .iter()
                .filter_map(|row| row.get(column))
                .map(|cell| cell.width.map_or(0, |width| width.resolve(fixed.unwrap_or(parent_width))))
                .max()
                .unwrap_or(0)
        })
        .collect();
    for row in &table.rows {
        let mut x = edge;
        let mut row_height = 0;
        let mut row_cells = Vec::new();
        for (column, cell) in row.iter().enumerate() {
            let content_width =
                column_widths.get(column).copied().filter(|w| *w > 0).unwrap_or(fixed.unwrap_or(parent_width));
            let content = layout_frame(&cell.children, content_width, font);
            let h = (content.height - HTML_LINE_PADDING).max(0);
            let w = if cell.width.is_some() { content_width } else { content.width.max(content_width) };
            row_height = row_height.max(h);
            row_cells.push((Rect::new(x, y, w, h), content, cell.valign));
            x += w + (border + pad) * 2 + spacing;
        }
        widest = widest.max(x - pad);
        for (rect, content, valign) in row_cells {
            let valign_offset = match valign {
                VAlign::Top => 0,
                VAlign::Middle => row_height / 2 - rect.h / 2,
                VAlign::Bottom => row_height - rect.h,
            };
            cells.push(CellLayout { rect: Rect::new(rect.x, rect.y, rect.w, rect.h), content, valign_offset });
        }
        y += spacing + (pad + border) * 2 + row_height;
    }
    let width = fixed.unwrap_or(widest);
    let height = table.height.map_or(y - pad, |fixed_height| fixed_height.max(y - pad));
    TableLayout { rect: Rect::new(0, 0, width, height), border, background: table.background, cells }
}

pub fn paint_frame(
    list: &mut DrawList,
    font: &BitmapFont,
    frame: &FrameLayout,
    origin: (i32, i32),
    scroll: i32,
    alpha: f32,
) {
    for item in &frame.items {
        match item {
            Placed::Text { rect, lines, .. } => {
                let mut y = origin.1 + rect.y + scroll;
                for line in lines {
                    let mut x = origin.0 + rect.x;
                    for span in line {
                        list.draw_text(x, y, span.color, &span.text, font);
                        x += font.measure(&span.text) as i32;
                    }
                    y += font.line_height as i32;
                }
            }
            Placed::Image { rect, texture } => {
                list.draw_texture(
                    origin.0 + rect.x,
                    origin.1 + rect.y + scroll,
                    rect.w,
                    rect.h,
                    0,
                    0,
                    rect.w,
                    rect.h,
                    texture,
                    alpha,
                );
            }
            Placed::Button { rect, label, back, .. } => {
                let (x, y) = (origin.0 + rect.x, origin.1 + rect.y + scroll);
                if !back.is_empty() {
                    list.draw_button_texture(x, y, rect.w, rect.h, 0, 0, rect.w, rect.h, back, alpha, true);
                }
                if !label.is_empty() {
                    let tw = font.measure(label) as i32;
                    let th = font.line_height as i32;
                    list.draw_text(
                        x + (rect.w as f32 * 0.5 - (tw / 2) as f32) as i32,
                        y + (rect.h as f32 * 0.5 - (th / 2) as f32) as i32,
                        crate::color::BUTTON_TEXT,
                        label,
                        font,
                    );
                }
            }
            Placed::Table(table) => paint_table(list, font, table, origin, scroll, alpha),
        }
    }
}

pub fn paint_table(
    list: &mut DrawList,
    font: &BitmapFont,
    table: &TableLayout,
    origin: (i32, i32),
    scroll: i32,
    alpha: f32,
) {
    let rect = Rect::new(origin.0 + table.rect.x, origin.1 + table.rect.y + scroll, table.rect.w, table.rect.h);
    if let Some(background) = table.background {
        list.draw_fill(rect.x, rect.y, rect.w, rect.h, background);
    }
    for inset in 0..table.border {
        let (x1, x2) = (rect.x + inset, rect.x + rect.w - 1 - inset);
        let (y1, y2) = (rect.y + inset, rect.y + rect.h - 1 - inset);
        list.draw_hline(x1, y1, x2 - x1 + 1, HTML_TEXT_COLOR);
        list.draw_hline(x1, y2, x2 - x1 + 1, HTML_TEXT_COLOR);
        list.draw_vline(x1, y1, y2 - y1 + 1, HTML_TEXT_COLOR);
        list.draw_vline(x2, y1, y2 - y1 + 1, HTML_TEXT_COLOR);
    }
    for cell in &table.cells {
        let cell_origin = (rect.x + cell.rect.x, rect.y + cell.rect.y + cell.valign_offset);
        paint_frame(list, font, &cell.content, cell_origin, 0, alpha);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn paint_viewer(
    list: &mut DrawList,
    font: &BitmapFont,
    rect: Rect,
    back: Option<&str>,
    content_origin: (i32, i32),
    scroll: i32,
    frame: &FrameLayout,
    alpha: f32,
) {
    let start = list.quads.len();
    if let Some(back) = back {
        list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, back, alpha);
    }
    paint_frame(list, font, frame, (rect.x + content_origin.0, rect.y + content_origin.1), scroll, alpha);
    list.clip_from(start, rect);
}

pub fn paint_npc_viewer_back(list: &mut DrawList, rect: Rect, alpha: f32) {
    paint_npc_back(list, rect, NPC_HTML_BACK, alpha);
}

pub fn paint_npc_back(list: &mut DrawList, rect: Rect, texture: &str, alpha: f32) {
    list.draw_texture(rect.x, rect.y + NPC_HTML_BACK_TOP, rect.w, rect.h, 0, 0, rect.w, rect.h, texture, alpha);
}

pub fn npc_viewer_position(viewport_height: i32) -> (i32, i32) {
    (0, (f64::from(viewport_height) * 0.5 - NPC_VIEWER_Y_OFFSET).round() as i32)
}

pub fn npc_content_rect(size: (i32, i32)) -> Rect {
    Rect::new(NPC_CONTENT_POS.0, NPC_CONTENT_POS.1, size.0 - NPC_CONTENT_INSET.0, size.1 - NPC_CONTENT_INSET.1)
}

pub fn align_width(viewer_width: i32) -> i32 {
    viewer_width - HTML_ALIGN_MARGIN
}

pub fn from_document(nodes: &[crate::html::Node]) -> Vec<HtmlNode> {
    let mut out = Vec::new();
    convert_nodes(nodes, None, HAlign::Left, None, &mut out);
    out
}

fn convert_nodes(
    nodes: &[crate::html::Node],
    color: Option<Argb>,
    align: HAlign,
    action: Option<&str>,
    out: &mut Vec<HtmlNode>,
) {
    use crate::html::Node;
    for node in nodes {
        match node {
            Node::Text(text) => out.push(HtmlNode::Text(HtmlText {
                text: text.clone(),
                color,
                link: action.is_some(),
                action: action.map(str::to_string),
                align,
                ..Default::default()
            })),
            Node::Br | Node::Br1 => out.push(HtmlNode::Break),
            Node::Font { color: inner, children } => convert_nodes(children, inner.or(color), align, action, out),
            Node::Link { action: inner, children } => convert_nodes(children, color, align, Some(inner), out),
            Node::Center(children) => convert_nodes(children, color, HAlign::Center, action, out),
            Node::Table { width, rows, attrs } => out.push(HtmlNode::Table(HtmlTable {
                border: attrs.border,
                cellpadding: attrs.cellpadding,
                cellspacing: attrs.cellspacing,
                width: width.map(HtmlWidth::Pixels),
                height: attrs.height,
                background: attrs.bgcolor,
                align: attrs.align.as_deref().map_or(align, halign),
                rows: rows
                    .iter()
                    .map(|row| {
                        row.iter()
                            .map(|cell| {
                                let mut children = Vec::new();
                                convert_nodes(&cell.children, color, HAlign::Left, action, &mut children);
                                HtmlCell {
                                    width: cell.width.map(HtmlWidth::Pixels),
                                    align: cell.align.as_deref().map_or(HAlign::Left, halign),
                                    valign: cell.valign.as_deref().map_or(VAlign::Top, valign),
                                    children,
                                }
                            })
                            .collect()
                    })
                    .collect(),
            })),
            Node::Img { src, width, height } => {
                out.push(HtmlNode::Image(HtmlImage { texture: src.clone(), width: *width, height: *height, align }))
            }
            Node::Button { value, action, width, height, back, fore } => out.push(HtmlNode::Button(HtmlButton {
                label: value.clone(),
                back: back.clone(),
                fore: fore.clone(),
                width: *width,
                height: *height,
                align,
                action: (!action.is_empty()).then(|| action.clone()),
            })),
            Node::Edit { .. } => {}
        }
    }
}

fn halign(name: &str) -> HAlign {
    if name.eq_ignore_ascii_case("center") {
        HAlign::Center
    } else if name.eq_ignore_ascii_case("right") {
        HAlign::Right
    } else {
        HAlign::Left
    }
}

fn valign(name: &str) -> VAlign {
    if name.eq_ignore_ascii_case("middle") || name.eq_ignore_ascii_case("center") {
        VAlign::Middle
    } else if name.eq_ignore_ascii_case("bottom") {
        VAlign::Bottom
    } else {
        VAlign::Top
    }
}

pub fn link_at<'a>(frame: &'a FrameLayout, font: &BitmapFont, x: i32, y: i32) -> Option<&'a str> {
    for item in &frame.items {
        match item {
            Placed::Text { rect, lines, .. } => {
                let row = font.line_height as i32;
                for (index, line) in lines.iter().enumerate() {
                    let top = rect.y + index as i32 * row;
                    if y < top || y >= top + row {
                        continue;
                    }
                    let mut sx = rect.x;
                    for span in line {
                        let w = font.measure(&span.text) as i32;
                        if x >= sx && x < sx + w {
                            return span.action.as_deref();
                        }
                        sx += w;
                    }
                }
            }
            Placed::Button { rect, action, .. } => {
                if rect.contains(x, y) {
                    return action.as_deref();
                }
            }
            Placed::Table(table) => {
                for cell in &table.cells {
                    let (ox, oy) = (table.rect.x + cell.rect.x, table.rect.y + cell.rect.y + cell.valign_offset);
                    if let Some(action) = link_at(&cell.content, font, x - ox, y - oy) {
                        return Some(action);
                    }
                }
            }
            Placed::Image { .. } => {}
        }
    }
    None
}

pub fn content_height(frame: &FrameLayout) -> i32 {
    frame.items.last().map_or(0, |last| match last {
        Placed::Text { rect, .. } | Placed::Image { rect, .. } | Placed::Button { rect, .. } => rect.y + rect.h,
        Placed::Table(table) => table.rect.y + table.rect.h,
    })
}

#[allow(clippy::too_many_arguments)]
pub fn paint_warning_dialog(
    list: &mut DrawList,
    font: &BitmapFont,
    rect: Rect,
    back: Option<&str>,
    icon: Option<&str>,
    text: &str,
    alpha: f32,
) {
    let start = list.quads.len();
    if let Some(back) = back {
        list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, back, alpha);
    }
    if let Some(icon) = icon {
        let r = WARNING_ICON_RECT;
        list.draw_texture(rect.x + r.x, rect.y + r.y, r.w, r.h, 0, 0, r.w, r.h, icon, alpha);
    }
    let mut y = rect.y + WARNING_TEXT_POS.1;
    for line in wrap_text(text, WARNING_TEXT_WIDTH, font) {
        list.draw_text(rect.x + WARNING_TEXT_POS.0, y, DIALOG_TEXT_COLOR, &line, font);
        y += font.line_height as i32;
    }
    list.clip_from(start, rect);
}

#[derive(Debug, Clone, PartialEq)]
pub struct GaraPrice {
    pub icon: Option<String>,
    pub icon_x: i32,
    pub icon_size: i32,
    pub label: String,
}

#[allow(clippy::too_many_arguments)]
pub fn paint_gara_dialog(
    list: &mut DrawList,
    font: &BitmapFont,
    rect: Rect,
    back: Option<&str>,
    icon: Option<&str>,
    text: &str,
    text_width: i32,
    price: Option<&GaraPrice>,
    alpha: f32,
) {
    let start = list.quads.len();
    if let Some(back) = back {
        list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, rect.w, rect.h, back, alpha);
    }
    if let Some(icon) = icon {
        let r = GARA_ICON_RECT;
        list.draw_texture(rect.x + r.x, rect.y + r.y, r.w, r.h, 0, 0, r.w, r.h, icon, alpha);
    }
    let mut y = rect.y + GARA_TEXT_POS.1;
    for line in wrap_text(text, text_width - GARA_TEXT_INSET, font) {
        list.draw_text(rect.x + GARA_TEXT_POS.0, y, DIALOG_TEXT_COLOR, &line, font);
        y += font.line_height as i32;
    }
    if let Some(price) = price {
        if let Some(icon) = &price.icon {
            list.draw_texture(
                rect.x + price.icon_x,
                rect.y,
                price.icon_size,
                price.icon_size,
                0,
                0,
                price.icon_size,
                price.icon_size,
                icon,
                alpha,
            );
        }
        let tw = font.measure(&price.label) as i32;
        list.draw_text(rect.x + (price.icon_x - tw) / 2, rect.y + GARA_PRICE_Y, DIALOG_PRICE_COLOR, &price.label, font);
    }
    list.clip_from(start, rect);
}

#[allow(clippy::too_many_arguments)]
pub fn paint_html_slider(
    list: &mut DrawList,
    x: i32,
    top: i32,
    length: i32,
    up: Option<&str>,
    down: Option<&str>,
    track: Option<&str>,
    alpha: f32,
) {
    if length <= 0 {
        return;
    }
    if let Some(up) = up {
        list.draw_texture(x, top, SLIDER_WIDTH, SLIDER_CAP, 0, 0, SLIDER_WIDTH, SLIDER_CAP, up, alpha);
    }
    if let Some(down) = down {
        list.draw_texture(
            x,
            top + length - SLIDER_CAP,
            SLIDER_WIDTH,
            SLIDER_CAP,
            0,
            0,
            SLIDER_WIDTH,
            SLIDER_CAP,
            down,
            alpha,
        );
    }
    if let Some(track) = track {
        let start = list.quads.len();
        let body = length - 2 * SLIDER_CAP;
        for row in 0..(body + SLIDER_CAP - 1) / SLIDER_CAP {
            list.draw_texture(
                x,
                top + SLIDER_CAP + row * SLIDER_CAP,
                SLIDER_WIDTH,
                SLIDER_CAP,
                0,
                0,
                SLIDER_WIDTH,
                SLIDER_CAP,
                track,
                alpha,
            );
        }
        list.clip_from(start, Rect::new(x, top + SLIDER_CAP, SLIDER_WIDTH, body));
    }
}

#[allow(clippy::too_many_arguments)]
pub fn paint_html_edit(list: &mut DrawList, font: &BitmapFont, rect: Rect, pieces: [&str; 3], text: &str, alpha: f32) {
    let start = list.quads.len();
    for (texture, x, w) in [
        (pieces[0], 0, EDIT_PIECE),
        (pieces[1], EDIT_PIECE, rect.w - 2 * EDIT_PIECE),
        (pieces[2], rect.w - EDIT_PIECE, EDIT_PIECE),
    ] {
        if w <= 0 || texture.is_empty() {
            continue;
        }
        let tiles = list.quads.len();
        for row in 0..(rect.h + EDIT_PIECE_HEIGHT - 1) / EDIT_PIECE_HEIGHT {
            list.draw_texture(
                rect.x + x,
                rect.y + row * EDIT_PIECE_HEIGHT,
                w,
                EDIT_PIECE_HEIGHT,
                0,
                0,
                EDIT_PIECE,
                EDIT_PIECE_HEIGHT,
                texture,
                alpha,
            );
        }
        list.clip_from(tiles, Rect::new(rect.x + x, rect.y, w, rect.h));
    }
    if !text.is_empty() {
        list.draw_text(rect.x + EDIT_TEXT_POS.0, rect.y + EDIT_TEXT_POS.1, HTML_TEXT_COLOR, text, font);
    }
    list.clip_from(start, rect);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::{GlyphRect, Glyphs};

    fn font() -> BitmapFont {
        let glyphs = "abcdefghijklmnopqrstuvwxyz 0123456789"
            .chars()
            .map(|c| (c, GlyphRect { x: 0, width: if c == ' ' { 3 } else { 6 }, y: 0, height: 10 }))
            .collect();
        BitmapFont::new(String::new(), Glyphs { tex_width: 64, tex_height: 16, glyphs })
    }

    fn text(s: &str) -> HtmlNode {
        HtmlNode::Text(HtmlText { text: s.into(), ..Default::default() })
    }

    #[test]
    fn documents_from_the_tag_parser_keep_color_center_and_link_actions() {
        let doc = crate::html::parse("<html><body><center><font color=\"LEVEL\">ab</font></center><br><a action=\"bypass -h x\">cd</a><table><tr><td width=40>ef</td></tr></table></body></html>");
        let nodes = from_document(&doc.body);
        let HtmlNode::Text(first) = &nodes[0] else { panic!("{nodes:?}") };
        assert_eq!((first.text.as_str(), first.align, first.color), ("ab", HAlign::Center, Some(HTML_LEVEL_COLOR)));
        assert_eq!(nodes[1], HtmlNode::Break);
        let HtmlNode::Text(link) = &nodes[2] else { panic!("{nodes:?}") };
        assert_eq!((link.text.as_str(), link.link, link.action.as_deref()), ("cd", true, Some("bypass -h x")));
        let HtmlNode::Table(table) = &nodes[3] else { panic!("{nodes:?}") };
        assert_eq!(table.rows[0][0].width, Some(HtmlWidth::Pixels(40)));
        let doc = crate::html::parse("<table border=1 cellpadding=2 cellspacing=3 bgcolor=\"112233\" height=50 align=center><tr><td align=right valign=bottom>x</td></tr></table>");
        let nodes = from_document(&doc.body);
        let HtmlNode::Table(table) = &nodes[0] else { panic!("{nodes:?}") };
        assert_eq!((table.border, table.cellpadding, table.cellspacing, table.height), (1, 2, 3, Some(50)));
        assert_eq!((table.background, table.align), (Some(Argb(0xFF112233)), HAlign::Center));
        assert_eq!((table.rows[0][0].align, table.rows[0][0].valign), (HAlign::Right, VAlign::Bottom));
    }

    #[test]
    fn link_at_finds_the_span_under_the_cursor_including_inside_tables() {
        let font = font();
        let link = |t: &str| HtmlNode::Text(HtmlText { text: t.into(), link: true, action: Some(format!("go {t}")), ..Default::default() });
        let table = HtmlNode::Table(HtmlTable {
            rows: vec![vec![HtmlCell { children: vec![link("ef")], ..Default::default() }]],
            ..Default::default()
        });
        let frame = layout_frame(&[text("ab"), link("cd"), HtmlNode::Break, table], 100, &font);
        assert_eq!(link_at(&frame, &font, 1, 1), None);
        assert_eq!(link_at(&frame, &font, 13, 1), Some("go cd"));
        assert_eq!(link_at(&frame, &font, 13, 12), None);
        let Placed::Table(placed) = &frame.items[1] else { panic!("{:?}", frame.items) };
        let cell = &placed.cells[0];
        assert_eq!(link_at(&frame, &font, placed.rect.x + cell.rect.x + 1, placed.rect.y + cell.rect.y + cell.valign_offset + 1), Some("go ef"));
    }

    #[test]
    fn npc_viewer_sits_at_the_left_edge_half_a_screen_minus_252_up() {
        assert_eq!(npc_viewer_position(720), (0, 108));
        assert_eq!(npc_viewer_position(961), (0, 229));
        assert_eq!(npc_content_rect(NPC_VIEWER_SIZE), Rect::new(7, 30, 296, 364));
        assert_eq!(align_width(296), 280);
    }

    #[test]
    fn colors_follow_the_tag_parser() {
        assert_eq!(parse_color("LEVEL"), HTML_LEVEL_COLOR);
        assert_eq!(parse_color("FF0000"), Argb(0xFFFF0000));
        assert_eq!(parse_color("0x00ff00"), Argb(0xFF00FF00));
        assert_eq!(parse_color("zz"), HTML_TEXT_COLOR);
    }

    #[test]
    fn consecutive_texts_form_one_run_with_six_pixels_below_each_block() {
        let font = font();
        let frame = layout_frame(&[text("ab"), text("cd"), HtmlNode::Break, text("ef")], 100, &font);
        assert_eq!(frame.items.len(), 2);
        let Placed::Text { rect, lines, .. } = &frame.items[0] else { panic!() };
        assert_eq!(*rect, Rect::new(0, 0, 24, 16));
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].iter().map(|s| s.text.as_str()).collect::<Vec<_>>(), ["ab", "cd"]);
        let Placed::Text { rect, .. } = &frame.items[1] else { panic!() };
        assert_eq!(*rect, Rect::new(0, 32, 12, 16));
        assert_eq!(frame.height, 48);
    }

    #[test]
    fn centered_text_is_centered_on_the_frame_width_and_links_are_blue() {
        let font = font();
        let node =
            HtmlNode::Text(HtmlText { text: "abcd".into(), align: HAlign::Center, link: true, ..Default::default() });
        let frame = layout_frame(&[node], 100, &font);
        let Placed::Text { rect, lines, .. } = &frame.items[0] else { panic!() };
        assert_eq!(rect.x, (100 - 24) / 2);
        assert_eq!(lines[0][0].color, HTML_LINK_COLOR);
    }

    #[test]
    fn text_wraps_on_spaces_inside_the_width() {
        let font = font();
        let lines = wrap_text("ab cd ef", 30, &font);
        assert_eq!(lines, ["ab cd", "ef"]);
        assert_eq!(wrap_text("ab\ncd", 100, &font), ["ab", "cd"]);
    }

    #[test]
    fn tables_place_cells_with_padding_spacing_and_border_like_nchtmltable_align() {
        let font = font();
        let cell = |t: &str, width: Option<HtmlWidth>| HtmlCell {
            width,
            align: HAlign::Left,
            valign: VAlign::Top,
            children: vec![text(t)],
        };
        let table = HtmlTable {
            border: 1,
            cellpadding: 2,
            cellspacing: 3,
            width: None,
            height: None,
            background: None,
            align: HAlign::Left,
            rows: vec![
                vec![cell("ab", Some(HtmlWidth::Pixels(40))), cell("cd", Some(HtmlWidth::Pixels(50)))],
                vec![cell("ef", None), cell("gh", None)],
            ],
        };
        let layout = layout_table(&table, 200, &font);
        let (pad, border, spacing) = (2, 1, 3);
        let edge = pad + border * 2 + spacing;
        assert_eq!(layout.cells[0].rect, Rect::new(edge, edge, 40, 10));
        assert_eq!(layout.cells[1].rect, Rect::new(edge + 40 + (1 + 2) * 2 + 3, edge, 50, 10));
        let second_row = edge + 3 + (2 + 1) * 2 + 10;
        assert_eq!(layout.cells[2].rect.y, second_row);
        assert_eq!(layout.rect.w, edge + 40 + 9 + 50 + 9 - 2);
        assert_eq!(layout.rect.h, second_row + 3 + 6 + 10 - 2);
        assert_eq!(layout.border, 1);
    }

    #[test]
    fn percent_widths_resolve_against_the_parent_and_cells_align_vertically() {
        let font = font();
        let tall = HtmlCell {
            width: None,
            align: HAlign::Left,
            valign: VAlign::Top,
            children: vec![text("ab"), HtmlNode::Break, text("cd")],
        };
        let short = HtmlCell { width: None, align: HAlign::Left, valign: VAlign::Middle, children: vec![text("ef")] };
        let table = HtmlTable {
            border: 0,
            cellpadding: 0,
            cellspacing: 0,
            width: Some(HtmlWidth::Percent(50)),
            height: None,
            background: Some(Argb(0xFF112233)),
            align: HAlign::Center,
            rows: vec![vec![tall, short]],
        };
        let layout = layout_table(&table, 300, &font);
        assert_eq!(layout.rect.w, 150);
        assert_eq!(layout.cells[0].rect.h, 42);
        assert_eq!(layout.cells[1].valign_offset, 42 / 2 - 10 / 2);
        let frame = layout_frame(&[HtmlNode::Table(table)], 300, &font);
        let Placed::Table(placed) = &frame.items[0] else { panic!() };
        assert_eq!(placed.rect.x, (300 - 150) / 2);
    }

    #[test]
    fn dialogs_put_icon_and_text_where_the_dll_does() {
        let font = font();
        let mut list = DrawList { capture_text: true, ..Default::default() };
        paint_warning_dialog(
            &mut list,
            &font,
            Rect::new(10, 20, 260, 100),
            Some("Was.back"),
            Some("Was.icon"),
            "ab",
            1.0,
        );
        assert_eq!((list.quads[0].texture.as_str(), list.quads[0].w, list.quads[0].h), ("Was.back", 260.0, 100.0));
        assert_eq!(
            (list.quads[1].texture.as_str(), list.quads[1].x, list.quads[1].y, list.quads[1].w),
            ("Was.icon", 21.0, 35.0, 24.0)
        );
        assert_eq!((list.text_runs[0].rect[0], list.text_runs[0].rect[1]), (51.0, 36.0));
        let mut list = DrawList { capture_text: true, ..Default::default() };
        let price = GaraPrice { icon: Some("Was.item".into()), icon_x: 100, icon_size: 32, label: "12".into() };
        paint_gara_dialog(
            &mut list,
            &font,
            Rect::new(0, 0, 300, 100),
            None,
            Some("Was.icon"),
            "cd",
            300,
            Some(&price),
            1.0,
        );
        assert_eq!((list.quads[0].x, list.quads[0].y), (11.0, 8.0));
        assert_eq!((list.text_runs[0].rect[0], list.text_runs[0].rect[1]), (41.0, 9.0));
        let item = list.quads.iter().find(|q| q.texture.as_str() == "Was.item").unwrap();
        assert_eq!((item.x, item.y, item.w), (100.0, 0.0, 32.0));
        assert_eq!((list.text_runs[1].rect[0], list.text_runs[1].rect[1]), (((100 - 12) / 2) as f32, 57.0));
        assert_eq!(list.text_runs[1].color, DIALOG_PRICE_COLOR.rgba());
    }

    #[test]
    fn the_slider_bar_and_the_edit_box_use_the_dll_pieces() {
        let mut list = DrawList::default();
        paint_html_slider(&mut list, 5, 7, 40, Some("Was.up"), Some("Was.down"), Some("Was.track"), 1.0);
        assert_eq!((list.quads[0].x, list.quads[0].y, list.quads[0].w, list.quads[0].h), (5.0, 7.0, 15.0, 8.0));
        assert_eq!((list.quads[1].y, list.quads[1].h), (39.0, 8.0));
        let tracks: Vec<&crate::draw::Quad> = list.quads.iter().filter(|q| q.texture.as_str() == "Was.track").collect();
        assert_eq!(tracks.len(), 3);
        assert_eq!((tracks[0].y, tracks[2].y + tracks[2].h), (15.0, 39.0));
        let mut list = DrawList { capture_text: true, ..Default::default() };
        paint_html_edit(&mut list, &font(), Rect::new(0, 0, 100, 17), ["Was.l", "Was.m", "Was.r"], "ab", 1.0);
        assert_eq!((list.quads[0].w, list.quads[1].x, list.quads[1].w, list.quads[2].x), (8.0, 8.0, 84.0, 92.0));
        assert_eq!((list.text_runs[0].rect[0], list.text_runs[0].rect[1]), (2.0, 2.0));
    }

    #[test]
    fn the_viewer_clips_to_its_rect_and_reports_the_content_height() {
        let font = font();
        let frame = layout_frame(
            &[
                text("ab"),
                HtmlNode::Image(HtmlImage { texture: "Was.img".into(), width: 32, height: 32, align: HAlign::Left }),
            ],
            100,
            &font,
        );
        assert_eq!(content_height(&frame), 16 + 32);
        let mut list = DrawList::default();
        paint_viewer(&mut list, &font, Rect::new(0, 0, 100, 20), Some("Was.back"), (0, 0), 0, &frame, 1.0);
        let image = list.quads.iter().find(|q| q.texture.as_str() == "Was.img").unwrap();
        assert_eq!((image.y, image.h), (16.0, 4.0));
        let mut list = DrawList::default();
        paint_npc_viewer_back(&mut list, Rect::new(0, 0, 100, 50), 1.0);
        assert_eq!((list.quads[0].texture.as_str(), list.quads[0].y), (NPC_HTML_BACK, 20.0));
    }
}
