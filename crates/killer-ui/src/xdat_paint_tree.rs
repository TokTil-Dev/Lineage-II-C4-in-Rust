use crate::color::Argb;
use crate::draw::DrawList;
use crate::font::BitmapFont;
use crate::window::Rect;

pub const TREE_PANEL_TOP: &str = "L2UI_CH3.QUESTWND.QUESTWNDPANEL1";
pub const TREE_PANEL_MID: &str = "L2UI_CH3.QUESTWND.QUESTWNDPANEL2";
pub const TREE_PANEL_BOTTOM: &str = "L2UI_CH3.QUESTWND.QUESTWNDPANEL3";
pub const TREE_LINE: &str = "L2UI_CH3.Etc.TextBackline";
pub const TREE_TEXT_RIGHT_MARGIN: i32 = 5;
pub const ITEM_NAME_COLOR: Argb = Argb(0xFFDCDCDC);
pub const ITEM_EXTRA_COLOR: Argb = Argb(0xFFFFD969);
pub const ITEM_GRADE_WIDTH: i32 = 14;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TreeBack {
    pub highlight: bool,
    pub highlight_height: i32,
    pub width: i32,
    pub u_width: i32,
    pub offset: (i32, i32),
    pub offset_bottom: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TreeBar {
    pub left: String,
    pub right: Option<String>,
    pub offset: (i32, i32),
    pub height: i32,
    pub right_width: i32,
    pub left_uv: (i32, i32),
    pub right_uv: (i32, i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TreeButton {
    pub size: (i32, i32),
    pub offset: (i32, i32),
    pub expand: String,
    pub collapse: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TreeItem {
    Blank {
        offset: (i32, i32),
        line_break: bool,
        height: i32,
    },
    Text {
        offset: (i32, i32),
        line_break: bool,
        text: String,
        color: Argb,
        one_line: bool,
    },
    Texture {
        offset: (i32, i32),
        line_break: bool,
        size: (i32, i32),
        uv: (i32, i32),
        texture: String,
        expanded: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TreeNode {
    pub offset: (i32, i32),
    pub expanded: bool,
    pub back: Option<TreeBack>,
    pub bar: Option<TreeBar>,
    pub button: Option<TreeButton>,
    pub items: Vec<TreeItem>,
    pub children: Vec<TreeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TreeLayout {
    pub rects: Vec<Rect>,
    pub height: i32,
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

pub fn paint_tree(
    list: &mut DrawList,
    font: &BitmapFont,
    root: &TreeNode,
    window: Rect,
    scroll: i32,
    alpha: f32,
) -> TreeLayout {
    let mut layout = TreeLayout::default();
    let height = paint_node(list, font, root, window, window.x, window.y - scroll, alpha, &mut layout);
    layout.height = height;
    layout
}

#[allow(clippy::too_many_arguments)]
fn paint_node(
    list: &mut DrawList,
    font: &BitmapFont,
    node: &TreeNode,
    window: Rect,
    x: i32,
    y: i32,
    alpha: f32,
    layout: &mut TreeLayout,
) -> i32 {
    let x = x + node.offset.0;
    let y = y + node.offset.1;
    let mut cursor_x = x;
    let mut cursor_y = y;
    let mut line_width = 0;
    let mut line_height = 0;
    let mut max_width = 0;
    let mut total_height = 0;
    if let Some(back) = &node.back {
        let bx = window.x + back.offset.0;
        let by = window.y + back.offset.1;
        let bh = window.h - back.offset.1 + back.offset_bottom;
        let bw = back.width;
        list.draw_texture(bx, by, bw, 8, 0, 0, back.u_width, 8, TREE_PANEL_TOP, alpha);
        list.draw_texture(bx, by + 8, bw, bh - 16, 0, 0, back.u_width, 8, TREE_PANEL_MID, alpha);
        list.draw_texture(bx, by + bh - 8, bw, 8, 0, 0, back.u_width, 8, TREE_PANEL_BOTTOM, alpha);
        if back.highlight && back.highlight_height > 0 {
            for row in (0..bh / back.highlight_height).filter(|row| row % 2 == 1) {
                list.draw_texture(bx, back.highlight_height * row - 2 + y, bw, 15, 0, 0, 8, 15, TREE_LINE, alpha);
            }
        }
    }
    if let Some(button) = &node.button {
        let face = if node.expanded { &button.collapse } else { &button.expand };
        list.draw_texture(
            x + button.offset.0,
            y + button.offset.1,
            button.size.0,
            button.size.1,
            0,
            0,
            button.size.0,
            button.size.1,
            face,
            alpha,
        );
        let advance = button.size.0 + button.offset.0;
        cursor_x += advance;
        line_width += advance;
        line_height = line_height.max(button.size.1 + button.offset.1);
    }
    if node.expanded
        && let Some(bar) = &node.bar
    {
        let bar_x = x + bar.offset.0;
        let bar_y = y + bar.offset.1;
        match &bar.right {
            None => {
                let width = window.x + window.w - bar_x;
                list.draw_texture(
                    bar_x,
                    bar_y,
                    width,
                    bar.height,
                    0,
                    0,
                    bar.left_uv.0,
                    bar.left_uv.1,
                    &bar.left,
                    alpha,
                );
            }
            Some(right) => {
                let width = window.x + window.w - bar_x - bar.right_width;
                list.draw_texture(
                    bar_x,
                    bar_y,
                    width,
                    bar.height,
                    0,
                    0,
                    bar.left_uv.0,
                    bar.left_uv.1,
                    &bar.left,
                    alpha,
                );
                list.draw_texture(
                    bar_x + width,
                    bar_y,
                    bar.right_width,
                    bar.height,
                    0,
                    0,
                    bar.right_uv.0,
                    bar.right_uv.1,
                    right,
                    alpha,
                );
            }
        }
    }
    for item in &node.items {
        let (offset, line_break) = match item {
            TreeItem::Blank { offset, line_break, .. }
            | TreeItem::Text { offset, line_break, .. }
            | TreeItem::Texture { offset, line_break, .. } => (*offset, *line_break),
        };
        if let TreeItem::Blank { height, .. } = item {
            line_height = 0;
            total_height += height;
            cursor_x = x;
            cursor_y += height;
            continue;
        }
        if !line_break && line_width > 0 {
            cursor_x += offset.0;
            line_width += offset.0;
        } else {
            cursor_x = x + offset.0;
            cursor_y += line_height;
            max_width = max_width.max(line_width);
            total_height += line_height;
            line_width = 0;
            line_height = 0;
        }
        match item {
            TreeItem::Text { text, color, one_line, .. } => {
                let lines = if *one_line {
                    vec![text.clone()]
                } else {
                    wrap_text(text, window.x + window.w - cursor_x - TREE_TEXT_RIGHT_MARGIN, font)
                };
                let mut text_width = 0;
                let mut text_height = 0;
                for (row, line) in lines.iter().enumerate() {
                    list.draw_text(
                        cursor_x,
                        cursor_y + offset.1 + font.line_height as i32 * row as i32,
                        *color,
                        line,
                        font,
                    );
                    text_width = text_width.max(font.measure(line) as i32);
                    text_height += font.line_height as i32;
                }
                cursor_x += text_width;
                line_width += text_width;
                line_height = line_height.max(text_height + offset.1);
            }
            TreeItem::Texture { size, uv, texture, expanded, .. } => {
                let face = expanded.as_deref().filter(|_| node.expanded).unwrap_or(texture);
                list.draw_texture(cursor_x, cursor_y + offset.1, size.0, size.1, 0, 0, uv.0, uv.1, face, alpha);
                cursor_x += size.0;
                line_width += size.0;
                line_height = line_height.max(size.1 + offset.1);
            }
            TreeItem::Blank { .. } => {}
        }
    }
    max_width = max_width.max(line_width);
    total_height += line_height;
    layout.rects.push(Rect::new(x, y, max_width.max(0), total_height.max(0)));
    if node.expanded {
        for child in &node.children {
            total_height += paint_node(list, font, child, window, x, y + total_height, alpha, layout);
        }
    }
    total_height + node.offset.1
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ItemName {
    pub name: String,
    pub enchant: i32,
    pub kind: i32,
    pub extra: Option<String>,
    pub grade: Option<String>,
    pub count: Option<i32>,
}

pub fn item_name_text(item: &ItemName) -> String {
    if item.enchant >= 1 && (0..=3).contains(&item.kind) {
        format!("+{} {}", item.enchant, item.name)
    } else {
        item.name.clone()
    }
}

pub fn shorten(text: &str, over: i32, font: &BitmapFont) -> (String, i32) {
    if over <= 0 {
        return (text.to_string(), 0);
    }
    let full = font.measure(text) as i32;
    let dots = font.measure("..") as i32;
    let mut kept: Vec<char> = text.chars().collect();
    let mut removed = 0;
    while kept.pop().is_some() {
        let prefix: String = kept.iter().collect();
        removed = (full - dots) - font.measure(&prefix) as i32;
        if removed > over {
            break;
        }
    }
    (kept.into_iter().chain("..".chars()).collect(), removed)
}

pub fn paint_item_name(list: &mut DrawList, font: &BitmapFont, item: &ItemName, origin: (i32, i32), width: i32) {
    let name = item_name_text(item);
    let name_width = font.measure(&name) as i32;
    let extra_width = item.extra.as_ref().map(|extra| font.measure(extra) as i32);
    let after_extra = match extra_width {
        Some(extra) => extra + 2 + name_width,
        None => name_width,
    };
    let grade = item.grade.as_ref().map(|grade| format!("`{grade}`"));
    let after_grade = if grade.is_some() { after_extra + ITEM_GRADE_WIDTH } else { after_extra };
    let count = item.count.filter(|count| *count >= 1).map(|count| format!("x{count}"));
    let count_width = count.as_ref().map(|count| font.measure(count) as i32);
    let total = match count_width {
        Some(count) => count + 2 + after_grade,
        None => after_grade,
    };
    let (name, removed) = shorten(&name, total - width, font);
    list.draw_text(origin.0, origin.1, ITEM_NAME_COLOR, &name, font);
    if let Some(extra) = &item.extra {
        list.draw_text(origin.0 + name_width - removed + 2, origin.1, ITEM_EXTRA_COLOR, extra, font);
    }
    if let Some(grade) = &grade {
        list.draw_text(origin.0 + after_extra - removed + 2, origin.1, ITEM_NAME_COLOR, grade, font);
    }
    if let Some(count) = &count {
        list.draw_text(origin.0 + after_grade - removed + 2, origin.1, ITEM_NAME_COLOR, count, font);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::{GlyphRect, Glyphs};

    fn font() -> BitmapFont {
        let glyphs = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+x`. "
            .chars()
            .map(|c| (c, GlyphRect { x: 0, width: if c == '.' { 3 } else { 6 }, y: 0, height: 12 }))
            .collect();
        BitmapFont::new(String::new(), Glyphs { tex_width: 64, tex_height: 16, glyphs })
    }

    fn runs(list: &DrawList) -> Vec<(String, f32, f32)> {
        list.text_runs.iter().map(|run| (run.text.clone(), run.rect[0], run.rect[1])).collect()
    }

    #[test]
    fn npc_info_nodes_lay_skill_icons_on_a_grid_like_the_target_window_script() {
        let icon = |index: i32| TreeNode {
            offset: ((index % 8) * 18, if index % 8 == 0 { if index > 0 { 3 } else { 0 } } else { -15 }),
            expanded: false,
            items: vec![TreeItem::Texture {
                offset: (0, 0),
                line_break: false,
                size: (15, 15),
                uv: (32, 32),
                texture: format!("Icon.skill{index}"),
                expanded: None,
            }],
            ..Default::default()
        };
        let root = TreeNode { expanded: true, children: (0..9).map(icon).collect(), ..Default::default() };
        let mut list = DrawList::default();
        let layout = paint_tree(&mut list, &font(), &root, Rect::new(18, 40, 158, 33), 0, 1.0);
        let icons: Vec<(f32, f32)> = list.quads.iter().map(|q| (q.x, q.y)).collect();
        assert_eq!(icons[0], (18.0, 40.0));
        assert_eq!(icons[1], (36.0, 40.0));
        assert_eq!(icons[7], (144.0, 40.0));
        assert_eq!(icons[8], (18.0, 58.0));
        assert_eq!((list.quads[0].w, list.quads[0].h, list.quads[0].uw, list.quads[0].vh), (15.0, 15.0, 32.0, 32.0));
        assert_eq!(layout.height, 33);
        assert_eq!(layout.rects[0], Rect::new(18, 40, 0, 0));
    }

    #[test]
    fn text_items_flow_on_the_same_line_until_a_line_break_and_backgrounds_use_three_panels() {
        let root = TreeNode {
            offset: (2, 3),
            expanded: true,
            back: Some(TreeBack {
                highlight: false,
                highlight_height: 0,
                width: 100,
                u_width: 32,
                offset: (1, 2),
                offset_bottom: 4,
            }),
            button: Some(TreeButton {
                size: (11, 11),
                offset: (1, 0),
                expand: "Was.plus".into(),
                collapse: "Was.minus".into(),
            }),
            items: vec![
                TreeItem::Text {
                    offset: (3, 1),
                    line_break: false,
                    text: "ab".into(),
                    color: Argb(0xFF112233),
                    one_line: true,
                },
                TreeItem::Text {
                    offset: (4, 0),
                    line_break: false,
                    text: "cd".into(),
                    color: Argb(0xFF112233),
                    one_line: true,
                },
                TreeItem::Text {
                    offset: (0, 2),
                    line_break: true,
                    text: "ef".into(),
                    color: Argb(0xFF445566),
                    one_line: true,
                },
            ],
            ..Default::default()
        };
        let mut list = DrawList { capture_text: true, ..Default::default() };
        let layout = paint_tree(&mut list, &font(), &root, Rect::new(10, 20, 150, 60), 5, 1.0);
        let panels: Vec<(&str, f32, f32, f32, f32)> =
            list.quads.iter().take(4).map(|q| (q.texture.as_str(), q.x, q.y, q.w, q.h)).collect();
        assert_eq!(panels[0], (TREE_PANEL_TOP, 11.0, 22.0, 100.0, 8.0));
        assert_eq!(panels[1], (TREE_PANEL_MID, 11.0, 30.0, 100.0, 46.0));
        assert_eq!(panels[2], (TREE_PANEL_BOTTOM, 11.0, 76.0, 100.0, 8.0));
        assert_eq!(panels[3], ("Was.minus", 13.0, 18.0, 11.0, 11.0));
        assert_eq!(runs(&list), vec![("ab".into(), 27.0, 19.0), ("cd".into(), 43.0, 18.0), ("ef".into(), 12.0, 33.0)]);
        assert_eq!(layout.height, 3 + 13 + 14);
        assert_eq!(layout.rects[0], Rect::new(12, 18, 43, 27));
    }

    #[test]
    fn item_names_add_enchant_grade_and_count_and_shorten_with_two_dots() {
        let font = font();
        let item = ItemName {
            name: "Sword".into(),
            enchant: 3,
            kind: 0,
            extra: None,
            grade: Some("D".into()),
            count: Some(5),
        };
        assert_eq!(item_name_text(&item), "+3 Sword");
        let mut list = DrawList { capture_text: true, ..Default::default() };
        paint_item_name(&mut list, &font, &item, (0, 0), 200);
        assert_eq!(
            runs(&list),
            vec![("+3 Sword".into(), 0.0, 0.0), ("`D`".into(), 50.0, 0.0), ("x5".into(), 64.0, 0.0)]
        );
        let weapon = ItemName { name: "Longsword".into(), enchant: 0, kind: 7, extra: None, grade: None, count: None };
        let mut list = DrawList { capture_text: true, ..Default::default() };
        paint_item_name(&mut list, &font, &weapon, (0, 0), 30);
        assert_eq!(runs(&list), vec![("Lon..".into(), 0.0, 0.0)]);
        assert_eq!(shorten("abcdef", 0, &font), ("abcdef".into(), 0));
    }

    #[test]
    fn wrapping_breaks_at_spaces_within_the_width() {
        assert_eq!(wrap_text("one two three", 48, &font()), vec!["one two", "three"]);
        assert_eq!(wrap_text("a\nb", 100, &font()), vec!["a", "b"]);
    }
}
