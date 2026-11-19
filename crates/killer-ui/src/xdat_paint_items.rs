use crate::color::Argb;
use crate::draw::DrawList;
use crate::font::BitmapFont;
use crate::window::Rect;
use l2_xdat::ItemWindow;

pub const UNDEFINED: i32 = -9999;
pub const OUTLINE_SIZE: i32 = 0x22;
pub const NUMBER_COLOR: Argb = Argb(0xFFFF0000);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemCell {
    pub icon: String,
    pub selected: bool,
    pub disabled: bool,
    pub number: Option<i32>,
    pub overlay: Option<(String, i32)>,
}

pub const WEIGHT_BAR_HEIGHT: i32 = 12;
pub const WEIGHT_TEXT_Y: i32 = 2;
pub const WEIGHT_TEXT_COLOR: Argb = Argb(0xFFDCDCDC);
pub const WEIGHT_BARS: [&str; 5] = [
    "L2UI_CH3.PlayerStatusWnd.PS_weightbar1",
    "L2UI_CH3.PlayerStatusWnd.PS_weightbar2",
    "L2UI_CH3.PlayerStatusWnd.PS_weightbar3",
    "L2UI_CH3.PlayerStatusWnd.PS_weightbar4",
    "L2UI_CH3.PlayerStatusWnd.PS_weightbar5",
];
pub const WEIGHT_PREVIEW: &str = "L2UI_CH3.PlayerStatusWnd.PS_mpbar";
const WEIGHT_STEPS: [f32; 3] = [50.0, 66.6, 80.0];

pub fn weight_bar_texture(percent: f32) -> &'static str {
    if percent <= WEIGHT_STEPS[0] {
        WEIGHT_BARS[0]
    } else if percent <= WEIGHT_STEPS[1] {
        WEIGHT_BARS[1]
    } else if percent <= WEIGHT_STEPS[2] {
        WEIGHT_BARS[2]
    } else if percent < 100.0 {
        WEIGHT_BARS[3]
    } else {
        WEIGHT_BARS[4]
    }
}

pub fn weight_text(load: i32, max: i32) -> String {
    let percent = if max == 0 { 0.0 } else { load as f32 / max as f32 * 100.0 };
    format!("{percent:.2}%")
}

#[allow(clippy::too_many_arguments)]
pub fn paint_inven_weight(
    list: &mut DrawList,
    rect: Rect,
    load: i32,
    max: i32,
    added: i32,
    has_texture: &dyn Fn(&str) -> bool,
    font: &BitmapFont,
    alpha: f32,
) {
    let start = list.quads.len();
    let ratio = if max == 0 { 0.0 } else { load as f32 / max as f32 };
    let percent = ratio * 100.0;
    if max != 0 {
        let width = (rect.w as f32 * ratio) as i32;
        if width >= 1 {
            let bar = weight_bar_texture(percent);
            if has_texture(bar) {
                list.draw_texture(rect.x, rect.y, width, WEIGHT_BAR_HEIGHT, 0, 0, width, WEIGHT_BAR_HEIGHT, bar, alpha);
            }
            if added > 0 && percent < 100.0 {
                let mut extra = (rect.w as f32 * (added as f32 / max as f32)) as i32;
                if rect.w < width + extra {
                    extra = rect.w - width;
                }
                let texture = if load + added < max { WEIGHT_PREVIEW } else { WEIGHT_BARS[4] };
                if has_texture(texture) {
                    list.draw_texture(
                        rect.x + width,
                        rect.y,
                        extra,
                        WEIGHT_BAR_HEIGHT,
                        0,
                        0,
                        extra,
                        WEIGHT_BAR_HEIGHT,
                        texture,
                        alpha,
                    );
                }
            }
        }
    }
    let text = format!("{percent:.2}%");
    let tx = (rect.w as f32 * 0.5 - (font.measure(&text) as i32 / 2) as f32) as i32;
    list.draw_digits(rect.x + tx, rect.y + WEIGHT_TEXT_Y, &text, WEIGHT_TEXT_COLOR);
    if rect.w > 0 && rect.h > 0 {
        list.clip_from(start, rect);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItemGrid {
    pub cols: i32,
    pub rows: i32,
    pub scroll_row: i32,
    pub icon: (i32, i32),
    pub gap: (i32, i32),
    pub origin: (i32, i32),
}

fn defined(text: &str) -> Option<&str> {
    (!text.is_empty() && !text.get(..9).is_some_and(|head| head.eq_ignore_ascii_case("undefined"))).then_some(text)
}

fn value(v: i32) -> i32 {
    if v == UNDEFINED { 0 } else { v }
}

impl ItemGrid {
    pub fn from_xdat(item: &ItemWindow, scroll_row: i32) -> ItemGrid {
        ItemGrid {
            cols: value(item.col).max(1),
            rows: value(item.row),
            scroll_row,
            icon: (value(item.icon_width), value(item.icon_height)),
            gap: (value(item.gap_x), value(item.gap_y)),
            origin: (value(item.offset_x) + 1, value(item.offset_y) + 1),
        }
    }

    pub fn cell_origin(&self, index: i32) -> Option<(i32, i32)> {
        let first = self.scroll_row * self.cols;
        if index < first {
            return None;
        }
        let row = (index - first) / self.cols;
        let col = (index - first) % self.cols;
        let y = self.origin.1 + row * (self.gap.1 + self.icon.1);
        if (self.icon.1 + 1) * self.rows <= y {
            return None;
        }
        Some((self.origin.0 + col * (self.gap.0 + self.icon.0), y))
    }
}

#[allow(clippy::too_many_arguments)]
pub fn paint_item_window(
    list: &mut DrawList,
    rect: Rect,
    item: &ItemWindow,
    scroll_row: i32,
    cells: &[Option<ItemCell>],
    disabled_texture: Option<&str>,
    has_texture: &dyn Fn(&str) -> bool,
    font: &BitmapFont,
    alpha: f32,
) {
    let start = list.quads.len();
    let grid = ItemGrid::from_xdat(item, scroll_row);
    let (iw, ih) = grid.icon;
    let centered = |origin: (i32, i32), w: i32, h: i32| (iw / 2 - w / 2 + origin.0, ih / 2 - h / 2 + origin.1);
    if let Some(back) = defined(&item.background_item_tex).filter(|t| has_texture(t)) {
        let (bw, bh) = (value(item.background_item_width), value(item.background_item_height));
        let mut index = grid.scroll_row * grid.cols;
        while let Some(origin) = grid.cell_origin(index) {
            if index >= grid.rows * grid.cols {
                break;
            }
            let (bx, by) = centered(origin, bw, bh);
            list.draw_texture(rect.x + bx, rect.y + by, bw, bh, 0, 0, bw, bh, back, alpha);
            index += 1;
        }
    }
    for (index, cell) in cells.iter().enumerate() {
        let Some(cell) = cell else { continue };
        let Some(origin) = grid.cell_origin(index as i32) else { continue };
        let (x, y) = (rect.x + origin.0, rect.y + origin.1);
        if has_texture(&cell.icon) {
            list.draw_texture(x, y, iw, ih, 0, 0, iw, ih, &cell.icon, alpha);
        }
        if item.button_click == 0 || item.button_click == UNDEFINED || item.button_click == -1 {
            if cell.selected
                && item.no_select_item <= 0
                && let Some(selected) = defined(&item.selected_item_tex).filter(|t| has_texture(t))
            {
                let (sw, sh) = (value(item.selected_item_width), value(item.selected_item_height));
                let (sx, sy) = centered(origin, sw, sh);
                list.draw_texture(rect.x + sx, rect.y + sy, sw, sh, 0, 0, sw, sh, selected, alpha);
            }
            if !cell.selected
                && let Some(unselected) = defined(&item.unselected_item_tex).filter(|t| has_texture(t))
            {
                let (uw, uh) = (value(item.unselected_item_width), value(item.unselected_item_height));
                let (ux, uy) = centered(origin, uw, uh);
                list.draw_texture(rect.x + ux, rect.y + uy, uw, uh, 0, 0, uw, uh, unselected, alpha);
            }
        } else {
            let outline = if cell.selected { &item.out_line_down } else { &item.out_line_up };
            if let Some(outline) = defined(outline).filter(|t| has_texture(t)) {
                list.draw_texture(
                    x - 1,
                    y - 1,
                    OUTLINE_SIZE,
                    OUTLINE_SIZE,
                    0,
                    0,
                    OUTLINE_SIZE,
                    OUTLINE_SIZE,
                    outline,
                    alpha,
                );
            }
        }
        if let Some(number) = cell.number {
            list.draw_text(x, y, NUMBER_COLOR, &number.to_string(), font);
        }
        if cell.disabled
            && let Some(disabled) = disabled_texture.filter(|t| has_texture(t))
        {
            list.draw_texture(x, y, iw, ih, 0, 0, iw, ih, disabled, alpha);
        }
        if let Some((overlay, height)) = &cell.overlay
            && has_texture(overlay)
        {
            list.draw_texture(x, y, iw, *height, 0, 0, iw, *height, overlay, alpha);
        }
    }
    if rect.w > 0 && rect.h > 0 {
        list.clip_from(start, rect);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::Glyphs;

    fn font() -> BitmapFont {
        let glyphs = "0123456789%."
            .chars()
            .map(|c| (c, crate::font::GlyphRect { x: 0, width: 6, y: 0, height: 10 }))
            .collect();
        BitmapFont::new(String::new(), Glyphs { tex_width: 64, tex_height: 16, glyphs })
    }

    fn grid(col: i32, row: i32, gap: (i32, i32)) -> ItemWindow {
        ItemWindow {
            col,
            row,
            max_item_num: col * row,
            icon_width: 32,
            icon_height: 32,
            gap_x: gap.0,
            gap_y: gap.1,
            offset_x: UNDEFINED,
            offset_y: UNDEFINED,
            background_item_width: UNDEFINED,
            background_item_height: UNDEFINED,
            background_item_tex: "undefined".into(),
            selected_item_width: 32,
            selected_item_height: 32,
            selected_item_tex: "Was.Sel".into(),
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
        }
    }

    #[test]
    fn cells_start_at_offset_plus_one_and_step_by_icon_plus_gap() {
        let g = ItemGrid::from_xdat(&grid(9, 8, (4, 4)), 0);
        assert_eq!(g.cell_origin(0), Some((1, 1)));
        assert_eq!(g.cell_origin(1), Some((37, 1)));
        assert_eq!(g.cell_origin(9), Some((1, 37)));
        assert_eq!(g.cell_origin(71), Some((289, 253)));
        assert_eq!(g.cell_origin(72), None);
        let scrolled = ItemGrid::from_xdat(&grid(9, 8, (4, 4)), 2);
        assert_eq!(scrolled.cell_origin(17), None);
        assert_eq!(scrolled.cell_origin(18), Some((1, 1)));
    }

    #[test]
    fn icons_selection_and_numbers_follow_the_dll() {
        let item = grid(9, 8, (4, 4));
        let cells = vec![
            Some(ItemCell { icon: "Icon.a".into(), selected: true, number: Some(5), ..Default::default() }),
            None,
            Some(ItemCell { icon: "Icon.b".into(), overlay: Some(("Was.Mark".into(), 10)), ..Default::default() }),
        ];
        let mut list = DrawList { capture_text: true, ..Default::default() };
        paint_item_window(&mut list, Rect::new(217, 55, 339, 288), &item, 0, &cells, None, &|_| true, &font(), 1.0);
        let quads: Vec<(&str, f32, f32, f32, f32)> =
            list.quads.iter().map(|q| (q.texture.as_str(), q.x, q.y, q.w, q.h)).collect();
        assert_eq!(quads[0], ("Icon.a", 218.0, 56.0, 32.0, 32.0));
        assert_eq!(quads[1], ("Was.Sel", 218.0, 56.0, 32.0, 32.0));
        assert_eq!(list.text_runs[0].text, "5");
        assert_eq!((list.text_runs[0].rect[0], list.text_runs[0].rect[1]), (218.0, 56.0));
        assert_eq!(list.text_runs[0].color, NUMBER_COLOR.rgba());
        let b = quads.iter().find(|q| q.0 == "Icon.b").unwrap();
        assert_eq!((b.1, b.2), (290.0, 56.0));
        let mark = quads.iter().find(|q| q.0 == "Was.Mark").unwrap();
        assert_eq!((mark.1, mark.2, mark.3, mark.4), (290.0, 56.0, 32.0, 10.0));
    }

    #[test]
    fn button_click_grids_draw_the_outline_one_pixel_out() {
        let mut item = grid(3, 1, (5, 3));
        item.button_click = 1;
        item.out_line_up = "Was.Up".into();
        item.out_line_down = "Was.Down".into();
        let cells = vec![
            Some(ItemCell { icon: "Icon.a".into(), selected: true, ..Default::default() }),
            Some(ItemCell { icon: "Icon.b".into(), ..Default::default() }),
        ];
        let mut list = DrawList::default();
        paint_item_window(&mut list, Rect::new(0, 0, 120, 40), &item, 0, &cells, None, &|_| true, &font(), 1.0);
        let quads: Vec<(&str, f32, f32, f32, f32)> =
            list.quads.iter().map(|q| (q.texture.as_str(), q.x, q.y, q.w, q.h)).collect();
        assert_eq!(quads[1], ("Was.Down", 0.0, 0.0, 34.0, 34.0));
        assert_eq!(quads[3], ("Was.Up", 37.0, 0.0, 34.0, 34.0));
    }

    #[test]
    fn weight_bar_picks_the_texture_by_percent_and_centers_the_text() {
        assert_eq!(weight_bar_texture(50.0), WEIGHT_BARS[0]);
        assert_eq!(weight_bar_texture(50.1), WEIGHT_BARS[1]);
        assert_eq!(weight_bar_texture(66.6), WEIGHT_BARS[1]);
        assert_eq!(weight_bar_texture(70.0), WEIGHT_BARS[2]);
        assert_eq!(weight_bar_texture(80.0), WEIGHT_BARS[2]);
        assert_eq!(weight_bar_texture(99.9), WEIGHT_BARS[3]);
        assert_eq!(weight_bar_texture(100.0), WEIGHT_BARS[4]);
        assert_eq!(weight_text(1, 3), "33.33%");
        let mut list = DrawList { capture_text: true, ..Default::default() };
        paint_inven_weight(&mut list, Rect::new(10, 20, 108, 10), 25, 100, 30, &|_| true, &font(), 1.0);
        let quads: Vec<(&str, f32, f32, f32, f32)> =
            list.quads.iter().map(|q| (q.texture.as_str(), q.x, q.y, q.w, q.h)).collect();
        assert_eq!(quads[0], (WEIGHT_BARS[0], 10.0, 20.0, 27.0, 10.0));
        assert_eq!(quads[1], (WEIGHT_PREVIEW, 37.0, 20.0, 32.0, 10.0));
        assert_eq!(list.text_runs[0].text, "25.00%");
        let tw = font().measure("25.00%") as i32;
        assert_eq!((list.text_runs[0].rect[0], list.text_runs[0].rect[1]), (10.0 + (54.0 - (tw / 2) as f32) as i32 as f32, 22.0));
        let mut over = DrawList::default();
        paint_inven_weight(&mut over, Rect::new(0, 0, 100, 12), 90, 100, 20, &|_| true, &font(), 1.0);
        assert_eq!(over.quads[1].texture.as_str(), WEIGHT_BARS[4]);
        assert_eq!(over.quads[1].w, 10.0);
    }
}
