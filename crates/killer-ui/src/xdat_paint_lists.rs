use crate::color::Argb;
use crate::draw::DrawList;
use crate::font::BitmapFont;
use crate::window::Rect;
use crate::xdat_paint_tabs::{SCROLL_BAR_WIDTH, SCROLL_BUTTON_SIZE, ScrollBar, scroll_thumb_length, scroll_thumb_y};
use l2_xdat::{ListCtrl, TextListBox};

pub const UNDEFINED: i32 = -9999;

pub const LIST_ROW_HEIGHT: i32 = 17;
pub const LIST_SHOW_ROW: i32 = 10;
pub const LIST_MAX_ROW: i32 = 100;
pub const LIST_HEADER_HEIGHT: i32 = 19;
pub const LIST_TEXT_X: i32 = 0;
pub const LIST_TEXT_Y: i32 = 3;
pub const LIST_TEXT_GAP: i32 = 10;
pub const LIST_CELL_CLIP_HEIGHT: i32 = 15;
pub const LIST_ICONS_PER_CELL: usize = 16;
pub const LIST_BACK_LINE: &str = "L2UI_CH3.Etc.textbackline";
pub const LIST_BACK_LINE_TEXEL: (i32, i32) = (8, 15);
pub const LIST_SELECT: &str = "L2UI_CH3.ListCtrl.TextSelect";
pub const LIST_SELECT_CAP: &str = "L2UI_CH3.ListCtrl.TextSelect2";
pub const LIST_SELECT_TEXEL: (i32, i32) = (16, 13);
pub const LIST_SELECT_CAP_TEXEL: (i32, i32) = (32, 13);
pub const LIST_SELECT_X: i32 = 1;
pub const LIST_SELECT_RIGHT_MARGIN: i32 = 49;

pub const HEADER_PIECES: [&str; 3] = ["L2UI_CH3.ListCtrl.tab1", "L2UI_CH3.ListCtrl.tab2", "L2UI_CH3.ListCtrl.tab3"];
pub const HEADER_PIECES_DOWN: [&str; 3] =
    ["L2UI_CH3.ListCtrl.tab1_down", "L2UI_CH3.ListCtrl.tab2_down", "L2UI_CH3.ListCtrl.tab3_down"];
pub const HEADER_PIECE_WIDTH: i32 = 8;
pub const HEADER_TEXEL_HEIGHT: i32 = 19;
pub const HEADER_TEXT_COLOR: Argb = Argb(0xFFE6DCBE);

pub const LIST_BOX_ROW_BASE: i32 = 12;
pub const LIST_BOX_TEXT_MARGIN: i32 = 15;
pub const LIST_BOX_SELECT_COLOR: Argb = Argb(0xFF646464);
pub const LIST_BOX_SELECT_TEXT_COLOR: Argb = Argb(0xFFDCDCDC);
pub const TEXT_LIST_TEXT_X: i32 = 2;
pub const TEXT_LIST_BASELINE: i32 = 9;

pub const COMBO_ROW_HEIGHT: i32 = 17;
pub const COMBO_TEXT_X: i32 = 5;
pub const COMBO_TEXT_Y: i32 = 0;
pub const COMBO_PIECE_WIDTH: i32 = 8;
pub const COMBO_TEXEL_HEIGHT: i32 = 17;
pub const COMBO_PIECES: [&str; 3] = ["L2UI_CH3.Etc.textbox1", "L2UI_CH3.Etc.textbox2", "L2UI_CH3.Etc.textbox3"];
pub const COMBO_PIECES_DISABLED: [&str; 3] =
    ["L2UI_CH3.Etc.textbox1_disable", "L2UI_CH3.Etc.textbox2_disable", "L2UI_CH3.Etc.textbox3_disable"];
pub const COMBO_TEXT_COLOR: Argb = Argb(0xFFDCDCDC);
pub const COMBO_TEXT_DISABLED_COLOR: Argb = Argb(0xFF7E7E7E);
pub const COMBO_SELECT_HEIGHT: i32 = 16;
pub const COMBO_SELECT_RIGHT_MARGIN: i32 = 32;
pub const COMBO_BUTTON_NORMAL: &str = "L2UI_CH3.Button.downbutton";
pub const COMBO_BUTTON_DOWN: &str = "L2UI_CH3.Button.downbutton_down";
pub const COMBO_BUTTON_X: i32 = -1;
pub const COMBO_BUTTON_Y_OFFSET: i32 = -7;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListCtrlSpec {
    pub max_row: i32,
    pub show_row: i32,
    pub use_scroll: bool,
    pub contents_height: i32,
    pub header_height: i32,
}

impl ListCtrlSpec {
    pub fn from_xdat(list: &ListCtrl) -> ListCtrlSpec {
        let [max_row, show_row, use_scroll, contents_height, header_height] = list.unk100_104;
        ListCtrlSpec {
            max_row: if max_row == UNDEFINED { LIST_MAX_ROW } else { max_row },
            show_row,
            use_scroll: use_scroll != 0,
            contents_height,
            header_height: if header_height == UNDEFINED { LIST_HEADER_HEIGHT } else { header_height },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListColumn {
    pub title: String,
    pub width: i32,
}

pub fn list_columns(list: &ListCtrl, sysstring: &dyn Fn(i32) -> Option<String>) -> Vec<ListColumn> {
    list.values
        .iter()
        .map(|column| ListColumn {
            title: if column.text_string_id >= 0 {
                sysstring(column.text_string_id).unwrap_or_default()
            } else {
                String::new()
            },
            width: column.width,
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderState {
    Normal,
    Pressed,
    Over,
}

pub fn header_pieces(state: HeaderState) -> [&'static str; 3] {
    match state {
        HeaderState::Normal | HeaderState::Over => HEADER_PIECES,
        HeaderState::Pressed => HEADER_PIECES_DOWN,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellIcon {
    pub texture: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub u: i32,
    pub v: i32,
    pub uw: i32,
    pub vh: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListCell {
    pub text: String,
    pub color: Argb,
    pub icons: Vec<CellIcon>,
    pub texture: Option<(String, i32, i32)>,
    pub show_text: bool,
}

impl ListCell {
    pub fn text(text: impl Into<String>, color: Argb) -> ListCell {
        ListCell { text: text.into(), color, icons: Vec::new(), texture: None, show_text: true }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListLayout {
    pub header: Vec<Rect>,
    pub rows: Vec<(usize, Rect)>,
    pub scroll: Option<ScrollBar>,
}

pub fn header_rects(columns: &[ListColumn], header_height: i32) -> Vec<Rect> {
    let mut x = LIST_TEXT_X;
    columns
        .iter()
        .map(|column| {
            let rect = Rect::new(x, 0, column.width, header_height);
            x += column.width;
            rect
        })
        .collect()
}

pub fn header_title_origin(size: (i32, i32), label: (i32, i32)) -> (i32, i32) {
    ((size.0 as f32 * 0.5 - (label.0 / 2) as f32) as i32, (size.1 as f32 * 0.5 - (label.1 / 2) as f32) as i32 + 1)
}

pub fn list_scroll_bar(area: (i32, i32), header_height: i32, total: i32, visible: i32, position: i32) -> ScrollBar {
    let (w, h) = area;
    let x = w - SCROLL_BAR_WIDTH;
    let track =
        Rect::new(x, header_height + SCROLL_BUTTON_SIZE, SCROLL_BAR_WIDTH, h - 2 * SCROLL_BUTTON_SIZE - header_height);
    let thumb = (visible < total).then(|| {
        let length = scroll_thumb_length(track.h, total, visible);
        Rect::new(x, track.y + scroll_thumb_y(track.h, length, total, visible, position), SCROLL_BAR_WIDTH, length)
    });
    ScrollBar {
        up: Rect::new(x, header_height, SCROLL_BUTTON_SIZE, SCROLL_BUTTON_SIZE),
        down: Rect::new(x, h - SCROLL_BUTTON_SIZE, SCROLL_BUTTON_SIZE, SCROLL_BUTTON_SIZE),
        track,
        thumb,
    }
}

#[allow(clippy::too_many_arguments)]
pub fn draw_tiled_h(list: &mut DrawList, x: i32, y: i32, w: i32, h: i32, texel: (i32, i32), texture: &str, alpha: f32) {
    let (tw, th) = texel;
    if tw <= 0 || w <= 0 {
        return;
    }
    let mut drawn = 0;
    while drawn < w {
        let piece = tw.min(w - drawn);
        list.draw_texture(x + drawn, y, piece, h, 0, 0, piece, th, texture, alpha);
        drawn += piece;
    }
}

pub fn paint_header(
    list: &mut DrawList,
    rect: Rect,
    title: &str,
    state: HeaderState,
    has_texture: &dyn Fn(&str) -> bool,
    font: &BitmapFont,
    alpha: f32,
) {
    let start = list.quads.len();
    let pieces = header_pieces(state);
    let (x, y, w, h) = (rect.x, rect.y, rect.w, rect.h);
    if has_texture(pieces[0]) {
        draw_tiled_h(list, x, y, HEADER_PIECE_WIDTH, h, (HEADER_PIECE_WIDTH, HEADER_TEXEL_HEIGHT), pieces[0], alpha);
    }
    if has_texture(pieces[1]) {
        draw_tiled_h(
            list,
            x + HEADER_PIECE_WIDTH,
            y,
            w - 2 * HEADER_PIECE_WIDTH,
            h,
            (HEADER_PIECE_WIDTH, HEADER_TEXEL_HEIGHT),
            pieces[1],
            alpha,
        );
    }
    if has_texture(pieces[2]) {
        draw_tiled_h(
            list,
            x + w - HEADER_PIECE_WIDTH,
            y,
            HEADER_PIECE_WIDTH,
            h,
            (HEADER_PIECE_WIDTH, HEADER_TEXEL_HEIGHT),
            pieces[2],
            alpha,
        );
    }
    if !title.is_empty() {
        let label = (font.measure(title) as i32, font.line_height as i32);
        let (tx, ty) = header_title_origin((w, h), label);
        list.draw_text(x + tx, y + ty, HEADER_TEXT_COLOR, title, font);
    }
    if w > 0 && h > 0 {
        list.clip_from(start, rect);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn paint_list_ctrl(
    list: &mut DrawList,
    rect: Rect,
    spec: &ListCtrlSpec,
    columns: &[ListColumn],
    records: &[Vec<ListCell>],
    top: usize,
    selected: Option<usize>,
    header_states: &[HeaderState],
    back: Option<&str>,
    has_texture: &dyn Fn(&str) -> bool,
    font: &BitmapFont,
    alpha: f32,
) -> ListLayout {
    let start = list.quads.len();
    let (x, y, w, h) = (rect.x, rect.y, rect.w, rect.h);
    let header_height = spec.header_height;
    let row_height = LIST_ROW_HEIGHT;
    if let Some(back) = back.filter(|t| has_texture(t)) {
        list.draw_texture(x, y, w, h, 0, 0, w, h, back, alpha);
    }
    if has_texture(LIST_BACK_LINE) {
        for stripe in 0..=spec.show_row {
            if stripe % 2 == 1 {
                let (uw, vh) = LIST_BACK_LINE_TEXEL;
                list.draw_texture(
                    x,
                    y + row_height * stripe - 1 + header_height,
                    w,
                    row_height,
                    0,
                    0,
                    uw,
                    vh,
                    LIST_BACK_LINE,
                    alpha,
                );
            }
        }
    }
    let header = header_rects(columns, header_height);
    for (index, (column, cell)) in columns.iter().zip(&header).enumerate() {
        let state = header_states.get(index).copied().unwrap_or(HeaderState::Normal);
        let rect = Rect::new(x + cell.x, y + cell.y, cell.w, cell.h);
        paint_header(list, rect, &column.title, state, has_texture, font, alpha);
    }
    let mut rows = Vec::new();
    let visible = spec.show_row.max(0) as usize;
    for (row, record) in records.iter().enumerate().skip(top).take(visible) {
        let index = (row - top) as i32;
        let row_y = y + index * row_height + header_height;
        rows.push((row, Rect::new(x, row_y - 1, w, row_height)));
        if selected == Some(row) {
            if has_texture(LIST_SELECT) {
                draw_tiled_h(
                    list,
                    x + LIST_SELECT_X,
                    row_y - 1,
                    w - LIST_SELECT_RIGHT_MARGIN,
                    row_height,
                    LIST_SELECT_TEXEL,
                    LIST_SELECT,
                    alpha,
                );
            }
            if has_texture(LIST_SELECT_CAP) {
                let (uw, vh) = LIST_SELECT_CAP_TEXEL;
                list.draw_texture(
                    x + w + 1 - LIST_SELECT_RIGHT_MARGIN,
                    row_y - 1,
                    uw,
                    row_height,
                    0,
                    0,
                    uw,
                    vh,
                    LIST_SELECT_CAP,
                    alpha,
                );
            }
        }
        let text_y = row_y + LIST_TEXT_Y;
        let mut column_x = LIST_TEXT_X;
        for (column, cell) in columns.iter().zip(record) {
            let cell_start = list.quads.len();
            let clip = Rect::new(x + column_x, text_y - 1, column.width, LIST_CELL_CLIP_HEIGHT);
            let mut icons_width = 0;
            for icon in cell.icons.iter().take(LIST_ICONS_PER_CELL) {
                if !has_texture(&icon.texture) {
                    continue;
                }
                let uw = if icon.uw == 0 { icon.w } else { icon.uw };
                let vh = if icon.vh == 0 { icon.h } else { icon.vh };
                list.draw_texture(
                    x + column_x + icon.x,
                    text_y + icon.y,
                    icon.w,
                    icon.h,
                    icon.u,
                    icon.v,
                    uw,
                    vh,
                    &icon.texture,
                    alpha,
                );
                icons_width += icon.w;
            }
            let mut texture_width = 0;
            if let Some((texture, tw, th)) = &cell.texture
                && has_texture(texture)
            {
                texture_width = *tw;
                list.draw_texture(
                    x + column_x + column.width / 2 - tw / 2,
                    y + index * row_height - th / 2 + header_height + row_height / 2,
                    *tw,
                    *th,
                    0,
                    0,
                    *tw,
                    *th,
                    texture,
                    alpha,
                );
            }
            if cell.show_text {
                list.draw_text(
                    x + column_x + texture_width + icons_width + LIST_TEXT_GAP,
                    text_y,
                    cell.color,
                    &cell.text,
                    font,
                );
            }
            if clip.w > 0 && clip.h > 0 {
                list.clip_from(cell_start, clip);
            }
            column_x += column.width;
        }
    }
    let scroll = spec
        .use_scroll
        .then(|| list_scroll_bar((w, h), header_height, records.len() as i32, spec.show_row, top as i32));
    if w > 0 && h > 0 {
        list.clip_from(start, rect);
    }
    ListLayout { header, rows, scroll }
}

pub fn show_row_count(show_row: i32, height: i32) -> i32 {
    if show_row == -1 { (height as f32 / LIST_BOX_ROW_BASE as f32) as i32 } else { show_row }
}

pub fn fit_text(text: &str, width: i32, font: &BitmapFont) -> String {
    let mut out = String::new();
    let mut used = 0;
    for c in text.chars() {
        if c == '\n' {
            break;
        }
        if c == '\r' || (c as u32) < 0x20 {
            out.push(c);
            continue;
        }
        if let Some(glyph) = font.glyph(c) {
            used += glyph.width as i32;
            if used > width {
                break;
            }
        }
        out.push(c);
    }
    out
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListBoxItem {
    pub text: String,
    pub color: Argb,
    pub indent: i32,
}

#[allow(clippy::too_many_arguments)]
pub fn paint_list_box(
    list: &mut DrawList,
    rect: Rect,
    items: &[ListBoxItem],
    top: usize,
    selected: Option<usize>,
    show_row: i32,
    line_gap: i32,
    back: Option<&str>,
    has_texture: &dyn Fn(&str) -> bool,
    font: &BitmapFont,
    alpha: f32,
) -> Vec<(usize, Rect)> {
    let start = list.quads.len();
    let (x, y, w, h) = (rect.x, rect.y, rect.w, rect.h);
    if let Some(back) = back.filter(|t| has_texture(t)) {
        list.draw_texture(x, y, w, h, 0, 0, w, h, back, alpha);
    }
    let pitch = line_gap + LIST_BOX_ROW_BASE;
    let visible = show_row_count(show_row, h).max(0) as usize;
    let text_width = w - LIST_BOX_TEXT_MARGIN;
    let mut rows = Vec::new();
    for (row, item) in items.iter().enumerate().skip(top).take(visible) {
        let index = (row - top) as i32;
        let mut color = item.color;
        if selected == Some(row) {
            list.draw_fill(x, y + pitch * index, text_width, pitch, LIST_BOX_SELECT_COLOR);
            color = LIST_BOX_SELECT_TEXT_COLOR;
        }
        let text = fit_text(&item.text, text_width, font);
        list.draw_text(x, y + pitch * index, color, &text, font);
        rows.push((row, Rect::new(x, y + pitch * index, w, pitch)));
    }
    if w > 0 && h > 0 {
        list.clip_from(start, rect);
    }
    rows
}

pub fn text_list_row_y(line_gap: i32, index: i32, char_height: i32) -> i32 {
    line_gap * index - char_height / 2 + TEXT_LIST_BASELINE
}

#[allow(clippy::too_many_arguments)]
pub fn paint_text_list_box(
    list: &mut DrawList,
    rect: Rect,
    spec: &TextListBox,
    items: &[ListBoxItem],
    top: usize,
    char_height: i32,
    font: &BitmapFont,
) -> Vec<(usize, Rect)> {
    let start = list.quads.len();
    let (x, y, w, h) = (rect.x, rect.y, rect.w, rect.h);
    let visible = show_row_count(spec.show_row, h).max(0) as usize;
    let mut rows = Vec::new();
    for (row, item) in items.iter().enumerate().skip(top).take(visible) {
        let index = (row - top) as i32;
        let row_y = y + text_list_row_y(spec.line_gap, index, char_height);
        list.draw_text(x + TEXT_LIST_TEXT_X + item.indent, row_y, item.color, &item.text, font);
        rows.push((row, Rect::new(x, y + spec.line_gap * index, w, spec.line_gap)));
    }
    if w > 0 && h > 0 {
        list.clip_from(start, rect);
    }
    rows
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboLayout {
    pub button: Rect,
    pub rows: Vec<Rect>,
}

pub fn combo_text_y(row_height: i32, char_height: i32) -> i32 {
    row_height / 2 - char_height / 2
}

fn paint_combo_frame(
    list: &mut DrawList,
    x: i32,
    y: i32,
    w: i32,
    pieces: [&str; 3],
    has_texture: &dyn Fn(&str) -> bool,
    alpha: f32,
) {
    let (pw, th) = (COMBO_PIECE_WIDTH, COMBO_TEXEL_HEIGHT);
    if has_texture(pieces[0]) {
        list.draw_texture(x, y, pw, COMBO_ROW_HEIGHT, 0, 0, pw, th, pieces[0], alpha);
    }
    if has_texture(pieces[1]) {
        list.draw_texture(x + pw, y, w - 2 * pw, COMBO_ROW_HEIGHT, 0, 0, pw, th, pieces[1], alpha);
    }
    if has_texture(pieces[2]) {
        list.draw_texture(x + w - pw, y, pw, COMBO_ROW_HEIGHT, 0, 0, pw, th, pieces[2], alpha);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn paint_combo_box(
    list: &mut DrawList,
    rect: Rect,
    items: &[String],
    selected: Option<usize>,
    hovered: Option<usize>,
    expanded: bool,
    enabled: bool,
    char_height: i32,
    button_size: (i32, i32),
    has_texture: &dyn Fn(&str) -> bool,
    font: &BitmapFont,
    alpha: f32,
) -> ComboLayout {
    let (x, y, w, h) = (rect.x, rect.y, rect.w, rect.h);
    let row_height = COMBO_ROW_HEIGHT;
    let text_offset = combo_text_y(row_height, char_height);
    let frame_y = h / 2 - row_height / 2;
    let (pieces, color) =
        if enabled { (COMBO_PIECES, COMBO_TEXT_COLOR) } else { (COMBO_PIECES_DISABLED, COMBO_TEXT_DISABLED_COLOR) };
    paint_combo_frame(list, x, y + frame_y, w, pieces, has_texture, alpha);
    if let Some(text) = selected.and_then(|index| items.get(index)) {
        list.draw_text(x + COMBO_TEXT_X, y + frame_y + COMBO_TEXT_Y + text_offset, color, text, font);
    }
    let button = Rect::new(
        x + w + COMBO_BUTTON_X - button_size.0,
        y + h / 2 + COMBO_BUTTON_Y_OFFSET,
        button_size.0,
        button_size.1,
    );
    if has_texture(COMBO_BUTTON_NORMAL) {
        list.draw_texture(button.x, button.y, button.w, button.h, 0, 0, button.w, button.h, COMBO_BUTTON_NORMAL, alpha);
    }
    let mut rows = Vec::new();
    if expanded {
        for index in 0..items.len() as i32 {
            let row_y = y + row_height * index + h;
            paint_combo_frame(list, x, row_y, w, COMBO_PIECES, has_texture, alpha);
            rows.push(Rect::new(x, row_y, w, row_height));
        }
        for (index, text) in items.iter().enumerate() {
            let row_y = y + row_height * index as i32 + h;
            if hovered == Some(index) {
                if has_texture(LIST_SELECT) {
                    draw_tiled_h(
                        list,
                        x + LIST_SELECT_X,
                        row_y,
                        w - COMBO_SELECT_RIGHT_MARGIN,
                        COMBO_SELECT_HEIGHT,
                        LIST_SELECT_TEXEL,
                        LIST_SELECT,
                        alpha,
                    );
                }
                if has_texture(LIST_SELECT_CAP) {
                    let (uw, vh) = LIST_SELECT_CAP_TEXEL;
                    list.draw_texture(
                        x + w - COMBO_SELECT_RIGHT_MARGIN + 1,
                        row_y,
                        uw,
                        COMBO_SELECT_HEIGHT,
                        0,
                        0,
                        uw,
                        vh,
                        LIST_SELECT_CAP,
                        alpha,
                    );
                }
            }
            list.draw_text(x + COMBO_TEXT_X, row_y + text_offset, COMBO_TEXT_COLOR, text, font);
        }
    }
    ComboLayout { button, rows }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::{GlyphRect, Glyphs};
    use l2_xdat::ListElement;

    fn font() -> BitmapFont {
        let glyphs = ('A'..='Z')
            .chain('a'..='z')
            .chain('0'..='9')
            .chain(" .".chars())
            .map(|c| (c, GlyphRect { x: 0, width: 6, y: 0, height: 10 }))
            .collect();
        BitmapFont::new(String::new(), Glyphs { tex_width: 64, tex_height: 16, glyphs })
    }

    fn capturing() -> DrawList {
        DrawList { capture_text: true, ..Default::default() }
    }

    fn any(_: &str) -> bool {
        true
    }

    fn quads(list: &DrawList, texture: &str) -> Vec<(i32, i32, i32, i32)> {
        list.quads
            .iter()
            .filter(|q| q.texture.as_str() == texture)
            .map(|q| (q.x as i32, q.y as i32, q.w as i32, q.h as i32))
            .collect()
    }

    fn texts(list: &DrawList) -> Vec<(String, i32, i32, u32)> {
        list.text_runs
            .iter()
            .map(|r| {
                let c = r.color;
                let argb = ((c[3] * 255.0).round() as u32) << 24
                    | ((c[0] * 255.0).round() as u32) << 16
                    | ((c[1] * 255.0).round() as u32) << 8
                    | (c[2] * 255.0).round() as u32;
                (r.text.clone(), r.rect[0] as i32, r.rect[1] as i32, argb)
            })
            .collect()
    }

    fn xdat_list() -> ListCtrl {
        ListCtrl {
            unk100_104: [UNDEFINED, 6, 1, 19, UNDEFINED],
            values: vec![
                ListElement { text_string_id: 10, width: 60, unk108: false, unk109: true, unk110: false },
                ListElement { text_string_id: -1, width: 40, unk108: false, unk109: true, unk110: false },
            ],
        }
    }

    #[test]
    fn spec_defaults_follow_create() {
        let spec = ListCtrlSpec::from_xdat(&xdat_list());
        assert_eq!(
            spec,
            ListCtrlSpec { max_row: 100, show_row: 6, use_scroll: true, contents_height: 19, header_height: 19 }
        );
        let columns = list_columns(&xdat_list(), &|id| Some(format!("S{id}")));
        assert_eq!(columns[0], ListColumn { title: "S10".into(), width: 60 });
        assert_eq!(columns[1], ListColumn { title: String::new(), width: 40 });
    }

    #[test]
    fn header_pieces_and_title() {
        let mut list = capturing();
        paint_header(&mut list, Rect::new(10, 20, 60, 19), "Ab", HeaderState::Normal, &any, &font(), 1.0);
        assert_eq!(quads(&list, HEADER_PIECES[0]), vec![(10, 20, 8, 19)]);
        assert_eq!(quads(&list, HEADER_PIECES[1]).len(), 6);
        assert_eq!(quads(&list, HEADER_PIECES[1])[0], (18, 20, 8, 19));
        assert_eq!(quads(&list, HEADER_PIECES[1])[5], (58, 20, 4, 19));
        assert_eq!(quads(&list, HEADER_PIECES[2]), vec![(62, 20, 8, 19)]);
        assert_eq!(texts(&list), vec![("Ab".into(), 10 + 24, 20 + 4 + 1, 0xFFE6DCBE)]);
        let mut pressed = DrawList::default();
        paint_header(&mut pressed, Rect::new(0, 0, 24, 19), "", HeaderState::Pressed, &any, &font(), 1.0);
        assert_eq!(quads(&pressed, HEADER_PIECES_DOWN[1]), vec![(8, 0, 8, 19)]);
        assert!(pressed.text_runs.is_empty());
    }

    #[test]
    fn list_rows_selection_and_stripes() {
        let spec = ListCtrlSpec::from_xdat(&xdat_list());
        let columns = list_columns(&xdat_list(), &|_| Some("T".into()));
        let records: Vec<Vec<ListCell>> = (0..8)
            .map(|r| vec![ListCell::text(format!("r{r}"), Argb(0xFFFFFFFF)), ListCell::text("x", Argb(0xFF00FF00))])
            .collect();
        let mut list = capturing();
        let layout = paint_list_ctrl(
            &mut list,
            Rect::new(100, 200, 120, 140),
            &spec,
            &columns,
            &records,
            1,
            Some(2),
            &[],
            None,
            &any,
            &font(),
            1.0,
        );
        assert_eq!(layout.header, vec![Rect::new(0, 0, 60, 19), Rect::new(60, 0, 40, 19)]);
        assert_eq!(layout.rows.len(), 6);
        assert_eq!(layout.rows[0], (1, Rect::new(100, 200 + 19 - 1, 120, 17)));
        assert_eq!(layout.rows[5].0, 6);
        let stripes = quads(&list, LIST_BACK_LINE);
        assert_eq!(
            stripes,
            vec![
                (100, 200 + 17 - 1 + 19, 120, 17),
                (100, 200 + 51 - 1 + 19, 120, 17),
                (100, 200 + 85 - 1 + 19, 120, 17)
            ]
        );
        let select = quads(&list, LIST_SELECT);
        assert_eq!(select[0], (101, 200 + 17 - 1 + 19, 16, 17));
        assert_eq!(select.iter().map(|q| q.2).sum::<i32>(), 120 - 49);
        assert_eq!(quads(&list, LIST_SELECT_CAP), vec![(100 + 120 + 1 - 49, 200 + 17 - 1 + 19, 32, 17)]);
        let runs = texts(&list);
        assert_eq!(runs[1], ("r1".into(), 100 + 10, 200 + 19 + 3, 0xFFFFFFFF));
        assert_eq!(runs[2], ("x".into(), 100 + 60 + 10, 200 + 19 + 3, 0xFF00FF00));
        assert_eq!(runs[3].0, "r2");
        let scroll = layout.scroll.unwrap();
        assert_eq!(scroll.up, Rect::new(105, 19, 15, 15));
        assert_eq!(scroll.down, Rect::new(105, 125, 15, 15));
        assert_eq!(scroll.track, Rect::new(105, 34, 15, 140 - 30 - 19));
        assert!(scroll.thumb.is_some());
    }

    #[test]
    fn list_cell_icons_and_texture() {
        let spec =
            ListCtrlSpec { max_row: 100, show_row: 3, use_scroll: false, contents_height: 19, header_height: 20 };
        let columns = vec![ListColumn { title: String::new(), width: 80 }];
        let icon = CellIcon { texture: "I".into(), x: 2, y: 1, w: 12, h: 12, u: 0, v: 0, uw: 0, vh: 0 };
        let cell = ListCell {
            text: "t".into(),
            color: Argb(0xFFFFFFFF),
            icons: vec![icon],
            texture: Some(("C".into(), 10, 6)),
            show_text: true,
        };
        let mut list = capturing();
        let layout = paint_list_ctrl(
            &mut list,
            Rect::new(0, 0, 100, 80),
            &spec,
            &columns,
            &[vec![cell]],
            0,
            None,
            &[],
            Some("B"),
            &any,
            &font(),
            1.0,
        );
        assert!(layout.scroll.is_none());
        assert_eq!(quads(&list, "B"), vec![(0, 0, 100, 80)]);
        assert_eq!(quads(&list, "I"), vec![(2, 20 + 3 + 1, 12, 12)]);
        assert_eq!(
            list.quads.iter().find(|q| q.texture.as_str() == "I").map(|q| (q.uw as i32, q.vh as i32)),
            Some((12, 12))
        );
        assert_eq!(quads(&list, "C"), vec![(40 - 5, 20 + 8 - 3, 10, 6)]);
        assert_eq!(texts(&list)[0], ("t".into(), 10 + 12 + 10, 23, 0xFFFFFFFF));
    }

    #[test]
    fn list_box_rows_fill_and_fit() {
        let items: Vec<ListBoxItem> = (0..3)
            .map(|i| ListBoxItem { text: format!("row{i} is a very long line"), color: Argb(0xFF112233), indent: 0 })
            .collect();
        let mut list = capturing();
        let rows =
            paint_list_box(&mut list, Rect::new(0, 0, 75, 48), &items, 0, Some(1), -1, 2, None, &any, &font(), 1.0);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1], (1, Rect::new(0, 14, 75, 14)));
        let fill = list.quads.iter().find(|q| q.texture.as_str() == crate::draw::LINE_TEXTURE).unwrap();
        assert_eq!((fill.x as i32, fill.y as i32, fill.w as i32, fill.h as i32), (0, 14, 60, 14));
        let runs = texts(&list);
        assert_eq!(runs[0], ("row0 is a ".into(), 0, 0, 0xFF112233));
        assert_eq!(runs[1], ("row1 is a ".into(), 0, 14, 0xFFDCDCDC));
        assert_eq!(show_row_count(-1, 48), 4);
        assert_eq!(show_row_count(2, 48), 2);
    }

    #[test]
    fn text_list_rows() {
        let spec = TextListBox { max_row: 100, show_row: -1, line_gap: 16, is_show_scroll: 1 };
        let items = vec![
            ListBoxItem { text: "a".into(), color: Argb(0xFF0000FF), indent: 0 },
            ListBoxItem { text: "b".into(), color: Argb(0xFF00FF00), indent: 4 },
        ];
        let mut list = capturing();
        let rows = paint_text_list_box(&mut list, Rect::new(10, 10, 100, 48), &spec, &items, 0, 10, &font());
        assert_eq!(rows.len(), 2);
        assert_eq!(
            texts(&list),
            vec![("a".into(), 12, 10 - 5 + 9, 0xFF0000FF), ("b".into(), 16, 10 + 16 - 5 + 9, 0xFF00FF00)]
        );
    }

    #[test]
    fn combo_collapsed_and_expanded() {
        let items = vec!["one".to_string(), "two".to_string()];
        let mut list = capturing();
        let layout = paint_combo_box(
            &mut list,
            Rect::new(0, 0, 100, 21),
            &items,
            Some(1),
            None,
            false,
            true,
            10,
            (15, 15),
            &any,
            &font(),
            1.0,
        );
        assert_eq!(quads(&list, COMBO_PIECES[0]), vec![(0, 2, 8, 17)]);
        assert_eq!(quads(&list, COMBO_PIECES[1]), vec![(8, 2, 84, 17)]);
        assert_eq!(quads(&list, COMBO_PIECES[2]), vec![(92, 2, 8, 17)]);
        assert_eq!(texts(&list), vec![("two".into(), 5, 2 + 3, 0xFFDCDCDC)]);
        assert_eq!(layout.button, Rect::new(100 - 1 - 15, 10 - 7, 15, 15));
        assert!(layout.rows.is_empty());
        let mut open = capturing();
        let layout = paint_combo_box(
            &mut open,
            Rect::new(0, 0, 100, 21),
            &items,
            None,
            Some(1),
            true,
            false,
            10,
            (15, 15),
            &any,
            &font(),
            1.0,
        );
        assert_eq!(quads(&open, COMBO_PIECES_DISABLED[0]), vec![(0, 2, 8, 17)]);
        assert_eq!(layout.rows, vec![Rect::new(0, 21, 100, 17), Rect::new(0, 38, 100, 17)]);
        assert_eq!(quads(&open, COMBO_PIECES[0]), vec![(0, 21, 8, 17), (0, 38, 8, 17)]);
        assert_eq!(quads(&open, LIST_SELECT)[0], (1, 38, 16, 16));
        assert_eq!(quads(&open, LIST_SELECT).iter().map(|q| q.2).sum::<i32>(), 68);
        assert_eq!(quads(&open, LIST_SELECT_CAP), vec![(69, 38, 32, 16)]);
        assert_eq!(texts(&open), vec![("one".into(), 5, 21 + 3, 0xFFDCDCDC), ("two".into(), 5, 38 + 3, 0xFFDCDCDC)]);
    }
}
