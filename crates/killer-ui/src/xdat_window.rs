use crate::color::{Argb, BUTTON_TEXT, BUTTON_TEXT_DISABLED, TEXT};
use crate::draw::DrawList;
use crate::font::BitmapFont;
use crate::window::Rect;
use l2_dat::xdat::{Anchor, Control, GaugeProps, Props, Size, TextureProps, WindowDef};

pub const UNDEFINED: &str = "undefined";
pub const UNSET: i32 = -9999;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Point {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    Center,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Point {
    pub fn from_code(code: i32) -> Option<Point> {
        Some(match code {
            1 => Point::TopLeft,
            2 => Point::TopCenter,
            3 => Point::TopRight,
            4 => Point::MiddleLeft,
            5 => Point::Center,
            6 => Point::MiddleRight,
            7 => Point::BottomLeft,
            8 => Point::BottomCenter,
            9 => Point::BottomRight,
            _ => return None,
        })
    }

    pub fn at(self, rect: Rect) -> (i32, i32) {
        let x = match self {
            Point::TopLeft | Point::MiddleLeft | Point::BottomLeft => rect.x,
            Point::TopCenter | Point::Center | Point::BottomCenter => rect.x + rect.w / 2,
            Point::TopRight | Point::MiddleRight | Point::BottomRight => rect.x + rect.w,
        };
        let y = match self {
            Point::TopLeft | Point::TopCenter | Point::TopRight => rect.y,
            Point::MiddleLeft | Point::Center | Point::MiddleRight => rect.y + rect.h / 2,
            Point::BottomLeft | Point::BottomCenter | Point::BottomRight => rect.y + rect.h,
        };
        (x, y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Item<'a> {
    pub control: &'a Control,
    pub rect: Rect,
    pub depth: usize,
}

#[derive(Debug, Clone)]
pub struct Layout<'a> {
    pub window: &'a WindowDef,
    pub rect: Rect,
    pub items: Vec<Item<'a>>,
}

impl<'a> Layout<'a> {
    pub fn name(&self) -> &str {
        &self.window.head.name
    }

    pub fn find(&self, name: &str) -> Option<&Item<'a>> {
        self.items.iter().find(|item| item.control.head.name == name)
    }

    pub fn button_at(&self, x: i32, y: i32) -> Option<&Item<'a>> {
        self.items.iter().rev().find(|item| matches!(item.control.props, Props::Button(_)) && item.rect.contains(x, y))
    }

    pub fn shifted(mut self, dx: i32, dy: i32) -> Layout<'a> {
        self.rect = self.rect.offset(dx, dy);
        for item in &mut self.items {
            item.rect = item.rect.offset(dx, dy);
        }
        self
    }
}

pub fn visible_items<'a, 'b>(layout: &'b Layout<'a>, hidden: &dyn Fn(&str) -> bool) -> Vec<&'b Item<'a>> {
    let mut out = Vec::new();
    let mut skip_depth: Option<usize> = None;
    for item in &layout.items {
        if let Some(depth) = skip_depth {
            if item.depth > depth {
                continue;
            }
            skip_depth = None;
        }
        if hidden(&item.control.head.name) {
            skip_depth = Some(item.depth);
            continue;
        }
        out.push(item);
    }
    out
}

pub fn hit<'a, 'b>(layout: &'b Layout<'a>, hidden: &dyn Fn(&str) -> bool, x: i32, y: i32) -> Option<&'b Item<'a>> {
    visible_items(layout, hidden).into_iter().rev().find(|item| item.rect.contains(x, y) && !matches!(item.control.props, Props::Texture(_) | Props::TextBox(_) | Props::Window(_) | Props::Plain))
}

fn size_of(size: Option<&Size>, parent: Rect) -> (i32, i32) {
    match size {
        Some(Size::Fixed { width, height }) => (*width, *height),
        Some(Size::Scaled { scale, extent, .. }) => ((scale[0] * parent.w as f32) as i32 + extent[0], (scale[1] * parent.h as f32) as i32 + extent[1]),
        None => (0, 0),
    }
}

fn place(anchor: Option<&Anchor>, width: i32, height: i32, parent: Rect, reference: Option<Rect>) -> Rect {
    let Some(anchor) = anchor else { return Rect::new(parent.x, parent.y, width, height) };
    let reference = reference.unwrap_or(parent);
    let own = Point::from_code(anchor.mode[0]).unwrap_or(Point::TopLeft);
    let other = Point::from_code(anchor.mode[1]).unwrap_or(Point::TopLeft);
    let (rx, ry) = other.at(reference);
    let (ox, oy) = own.at(Rect::new(0, 0, width, height));
    Rect::new(rx + anchor.x as i32 - ox, ry + anchor.y as i32 - oy, width, height)
}

fn template_size<'a>(control: &'a Control, siblings: &'a [Control]) -> Option<&'a Size> {
    let mut current = control;
    for _ in 0..8 {
        if current.head.size.is_some() {
            return current.head.size.as_ref();
        }
        if !defined(&current.head.template) {
            return None;
        }
        current = siblings.iter().find(|other| other.head.name == current.head.template)?;
    }
    None
}

fn last_segment(name: &str) -> &str {
    name.rsplit('.').next().unwrap_or(name)
}

pub type Defaults<'a> = &'a dyn Fn(&str) -> Option<[i32; 3]>;

fn corner_place(values: [i32; 3], width: i32, height: i32, parent: Rect) -> Option<Rect> {
    let point = Point::from_code(values[0])?;
    let (px, py) = point.at(parent);
    let (ox, oy) = point.at(Rect::new(0, 0, width, height));
    Some(Rect::new(px + values[1] - ox, py + values[2] - oy, width, height))
}

fn resolve(index: usize, children: &[Control], parent: Rect, defaults: Defaults, rects: &mut Vec<Option<Rect>>, visiting: &mut Vec<bool>) -> Rect {
    if let Some(rect) = rects[index] {
        return rect;
    }
    let control = &children[index];
    let (width, height) = size_of(template_size(control, children), parent);
    if control.kind == "Window"
        && let Some(rect) = defaults(&control.head.name).and_then(|values| corner_place(values, width, height, parent))
    {
        rects[index] = Some(rect);
        return rect;
    }
    let mut reference = None;
    if let Some(anchor) = control.head.anchor.as_ref().filter(|anchor| !anchor.target.is_empty()) {
        let found = (0..index).rev().chain(index + 1..children.len()).find(|other| children[*other].head.name == last_segment(&anchor.target));
        if let Some(other) = found.filter(|other| !visiting[*other]) {
            visiting[index] = true;
            reference = Some(resolve(other, children, parent, defaults, rects, visiting));
            visiting[index] = false;
        }
    }
    let rect = place(control.head.anchor.as_ref(), width, height, parent, reference);
    rects[index] = Some(rect);
    rect
}

fn place_children<'a>(children: &'a [Control], parent: Rect, depth: usize, defaults: Defaults, out: &mut Vec<Item<'a>>) {
    let mut rects = vec![None; children.len()];
    let mut visiting = vec![false; children.len()];
    for index in 0..children.len() {
        let rect = resolve(index, children, parent, defaults, &mut rects, &mut visiting);
        out.push(Item { control: &children[index], rect, depth });
        place_children(&children[index].children, rect, depth + 1, defaults, out);
    }
}

pub fn layout<'a>(window: &'a WindowDef, viewport: Rect, target: &dyn Fn(&str) -> Option<Rect>) -> Layout<'a> {
    layout_with(window, viewport, target, &|_| None)
}

pub fn layout_with<'a>(window: &'a WindowDef, viewport: Rect, target: &dyn Fn(&str) -> Option<Rect>, defaults: Defaults) -> Layout<'a> {
    let (width, height) = match window.head.size.as_ref() {
        Some(size) => size_of(Some(size), viewport),
        None => (viewport.w, viewport.h),
    };
    let reference = window.head.anchor.as_ref().filter(|anchor| !anchor.target.is_empty()).and_then(|anchor| target(last_segment(&anchor.target)));
    let rect = defaults(&window.head.name)
        .and_then(|values| corner_place(values, width, height, viewport))
        .unwrap_or_else(|| place(window.head.anchor.as_ref(), width, height, viewport, reference));
    let mut items = Vec::new();
    place_children(&window.children, rect, 0, defaults, &mut items);
    Layout { window, rect, items }
}

pub struct Paint<'a> {
    pub font: &'a BitmapFont,
    pub texture_size: &'a dyn Fn(&str) -> Option<(i32, i32)>,
    pub sysstring: &'a dyn Fn(u32) -> Option<String>,
    pub cursor: Option<(i32, i32)>,
    pub pressed: Option<&'a str>,
    pub disabled: &'a dyn Fn(&str) -> bool,
    pub hidden: &'a dyn Fn(&str) -> bool,
    pub gauge: &'a dyn Fn(&str) -> Option<f32>,
    pub value: &'a dyn Fn(&str) -> Option<String>,
    pub color: &'a dyn Fn(&str) -> Option<Argb>,
    pub texture: &'a dyn Fn(&str) -> Option<String>,
    pub custom: &'a dyn Fn(&Item, &mut DrawList) -> bool,
}

fn defined(name: &str) -> bool {
    !name.is_empty() && name != UNDEFINED
}

pub type TextureSize<'a> = &'a dyn Fn(&str) -> Option<(i32, i32)>;

fn blit(list: &mut DrawList, rect: Rect, name: &str, texture_size: TextureSize, missing: &mut Vec<String>) {
    if !defined(name) || rect.w <= 0 || rect.h <= 0 {
        return;
    }
    match texture_size(name) {
        Some((w, h)) => list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, w, h, name, 1.0),
        None => {
            if !missing.iter().any(|seen| seen == name) {
                missing.push(name.to_string());
            }
        }
    }
}

fn float(raw: i32) -> f32 {
    f32::from_bits(raw as u32)
}

fn blit_texture_control(list: &mut DrawList, rect: Rect, texture: &TextureProps, name: &str, texture_size: TextureSize, missing: &mut Vec<String>) {
    if !defined(name) || rect.w <= 0 || rect.h <= 0 {
        return;
    }
    let Some((tw, th)) = texture_size(name) else {
        if !missing.iter().any(|seen| seen == name) {
            missing.push(name.to_string());
        }
        return;
    };
    let origin = (float(texture.values[2]).max(0.0) as i32, float(texture.values[3]).max(0.0) as i32);
    let span = |value: i32, full: i32| {
        let wanted = float(value) as i32;
        if wanted > 0 { wanted } else { full }
    };
    let (sw, sh) = (span(texture.values[4], tw - origin.0), span(texture.values[5], th - origin.1));
    match texture.values[0] {
        1 => {
            let start = list.quads.len();
            list.draw_texture(rect.x, rect.y, sw, sh, origin.0, origin.1, sw, sh, name, 1.0);
            list.clip_from(start, rect);
        }
        2 => {
            let start = list.quads.len();
            let (sw, sh) = (sw.max(1), sh.max(1));
            let columns = ((rect.w + sw - 1) / sw).min(256);
            let rows = ((rect.h + sh - 1) / sh).min(256);
            for row in 0..rows {
                for column in 0..columns {
                    list.draw_texture(rect.x + column * sw, rect.y + row * sh, sw, sh, origin.0, origin.1, sw, sh, name, 1.0);
                }
            }
            list.clip_from(start, rect);
        }
        _ => list.draw_texture(rect.x, rect.y, rect.w, rect.h, origin.0, origin.1, sw, sh, name, 1.0),
    }
}

fn blit_part(list: &mut DrawList, rect: Rect, name: &str, texture_size: TextureSize, missing: &mut Vec<String>) -> i32 {
    match texture_size(name) {
        Some((w, _)) => {
            blit(list, rect, name, texture_size, missing);
            w
        }
        None => {
            blit(list, rect, name, texture_size, missing);
            0
        }
    }
}

fn tile_area(list: &mut DrawList, rect: Rect, name: &str, texture_size: TextureSize, missing: &mut Vec<String>) {
    if !defined(name) || rect.w <= 0 || rect.h <= 0 {
        return;
    }
    let Some((tw, th)) = texture_size(name) else {
        if !missing.iter().any(|seen| seen == name) {
            missing.push(name.to_string());
        }
        return;
    };
    let (tw, th) = (tw.max(1), th.max(1));
    let start = list.quads.len();
    let columns = ((rect.w + tw - 1) / tw).min(256);
    let rows = ((rect.h + th - 1) / th).min(256);
    for row in 0..rows {
        for column in 0..columns {
            list.draw_texture(rect.x + column * tw, rect.y + row * th, tw, th, 0, 0, tw, th, name, 1.0);
        }
    }
    list.clip_from(start, rect);
}

fn tile_part(list: &mut DrawList, rect: Rect, name: &str, texture_size: TextureSize, missing: &mut Vec<String>) {
    if !defined(name) || rect.w <= 0 || rect.h <= 0 {
        return;
    }
    let Some((tw, th)) = texture_size(name) else {
        if !missing.iter().any(|seen| seen == name) {
            missing.push(name.to_string());
        }
        return;
    };
    let tw = tw.max(1);
    let start = list.quads.len();
    let columns = ((rect.w + tw - 1) / tw).min(1024);
    for column in 0..columns {
        list.draw_texture(rect.x + column * tw, rect.y, tw, rect.h, 0, 0, tw, th, name, 1.0);
    }
    list.clip_from(start, rect);
}

pub fn bar(list: &mut DrawList, rect: Rect, parts: [&str; 6], ratio: Option<f32>, texture_size: TextureSize, missing: &mut Vec<String>) {
    let [center, left, right, bg_center, bg_left, bg_right] = parts;
    let strip = |list: &mut DrawList, width: i32, center: &str, left: &str, right: &str, missing: &mut Vec<String>| {
        let left_w = texture_size(left).map_or(0, |size| size.0).min(width);
        let right_w = texture_size(right).map_or(0, |size| size.0).min((width - left_w).max(0));
        blit_part(list, Rect::new(rect.x, rect.y, left_w, rect.h), left, texture_size, missing);
        let middle = width - left_w - right_w;
        if middle > 0 {
            tile_part(list, Rect::new(rect.x + left_w, rect.y, middle, rect.h), center, texture_size, missing);
        }
        blit_part(list, Rect::new(rect.x + width - right_w, rect.y, right_w, rect.h), right, texture_size, missing);
    };
    strip(list, rect.w, bg_center, bg_left, bg_right, missing);
    if let Some(ratio) = ratio.filter(|ratio| *ratio > 0.0) {
        let width = ((rect.w as f32 * ratio.min(1.0)) as i32).max(1);
        strip(list, width, center, left, right, missing);
    }
}

fn gauge_parts(gauge: &GaugeProps) -> [&str; 6] {
    [&gauge.textures[0], &gauge.textures[1], &gauge.textures[2], &gauge.textures[3], &gauge.textures[4], &gauge.textures[5]]
}

fn text_color(raw: i32) -> Argb {
    if raw == UNSET { TEXT } else { Argb(raw as u32) }
}

fn caption(list: &mut DrawList, rect: Rect, text: &str, color: Argb, paint: &Paint) {
    if text.is_empty() {
        return;
    }
    let width = paint.font.measure(text) as i32;
    let x = rect.x + (rect.w - width) / 2;
    let y = rect.y + (rect.h - paint.font.line_height as i32) / 2;
    list.draw_text(x, y, color, text, paint.font);
}

fn control_text(control: &Control, text: &str, head: i32, paint: &Paint) -> Option<String> {
    if let Some(value) = (paint.value)(&control.head.name) {
        return Some(value);
    }
    if defined(text) {
        return Some(text.to_string());
    }
    (head != UNSET && head > 0).then(|| (paint.sysstring)(head as u32)).flatten()
}

pub fn text_align(raw: i32) -> TextAlign {
    match raw {
        2 => TextAlign::Center,
        3 => TextAlign::Right,
        _ => TextAlign::Left,
    }
}

pub fn text_valign(raw: i32) -> TextVAlign {
    match raw {
        1 => TextVAlign::Top,
        3 => TextVAlign::Bottom,
        _ => TextVAlign::Middle,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextVAlign {
    Top,
    Middle,
    Bottom,
}

pub fn draw_aligned(list: &mut DrawList, rect: Rect, text: &str, color: Argb, align: TextAlign, valign: TextVAlign, font: &BitmapFont) {
    if text.is_empty() {
        return;
    }
    let width = font.measure(text) as i32;
    let x = match align {
        TextAlign::Left => rect.x,
        TextAlign::Center => rect.x + (rect.w - width) / 2,
        TextAlign::Right => rect.x + rect.w - width,
    };
    let y = match valign {
        TextVAlign::Top => rect.y,
        TextVAlign::Middle => rect.y + (rect.h - font.line_height as i32) / 2,
        TextVAlign::Bottom => rect.y + rect.h - font.line_height as i32,
    };
    let start = list.quads.len();
    list.draw_text(x, y, color, text, font);
    if rect.w > 0 {
        list.clip_from(start, rect);
    }
}

fn label_color(control: &Control, tail_color: i32) -> Argb {
    match control.head.styles.first() {
        Some(style) if style.font_color != 0 => Argb(style.font_color),
        _ => text_color(tail_color),
    }
}

pub fn paint_layout(layout: &Layout, paint: &Paint, list: &mut DrawList) -> Vec<String> {
    let mut missing = Vec::new();
    tile_area(list, layout.rect, &layout.window.part.back_tex, paint.texture_size, &mut missing);
    for item in visible_items(layout, paint.hidden) {
        let control = item.control;
        let name = control.head.name.as_str();
        if (paint.custom)(item, list) {
            continue;
        }
        match &control.props {
            Props::Window(part) => tile_area(list, item.rect, &part.back_tex, paint.texture_size, &mut missing),
            Props::Texture(texture) => {
                let swapped = (paint.texture)(name);
                blit_texture_control(list, item.rect, texture, swapped.as_deref().unwrap_or(&texture.texture), paint.texture_size, &mut missing);
            }
            Props::Button(button) => {
                let disabled = (paint.disabled)(name);
                let over = paint.cursor.is_some_and(|(x, y)| item.rect.contains(x, y));
                let pressed = paint.pressed == Some(name) && over;
                let pick = |slot: usize| defined(&button.textures[slot]).then_some(slot);
                let slot = if disabled {
                    pick(3).unwrap_or(0)
                } else if pressed {
                    pick(1).unwrap_or(0)
                } else if over {
                    pick(2).unwrap_or(0)
                } else {
                    0
                };
                blit(list, item.rect, &button.textures[slot], paint.texture_size, &mut missing);
                if button.button_name_id != UNSET
                    && button.button_name_id > 0
                    && let Some(label) = (paint.sysstring)(button.button_name_id as u32)
                {
                    caption(list, item.rect, &label, if disabled { BUTTON_TEXT_DISABLED } else { BUTTON_TEXT }, paint);
                }
            }
            Props::TextBox(text) => {
                if let Some(value) = control_text(control, &text.text, text.sysstring, paint) {
                    let color = (paint.color)(name).unwrap_or_else(|| label_color(control, text.text_color));
                    draw_aligned(list, item.rect, &value, color, text_align(text.text_align), text_valign(text.text_valign), paint.font);
                }
            }
            Props::StatusBar(gauge) | Props::InvenWeight(gauge) => {
                bar(list, item.rect, gauge_parts(gauge), (paint.gauge)(name), paint.texture_size, &mut missing);
            }
            Props::BarCtrl { textures, .. } => {
                let parts = [&textures[0], &textures[1], &textures[2], &textures[3], &textures[4], &textures[5]].map(String::as_str);
                bar(list, item.rect, parts, (paint.gauge)(name), paint.texture_size, &mut missing);
            }
            _ => {}
        }
    }
    missing
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn anchor(mode: [i32; 2], x: f32, y: f32) -> Anchor {
        Anchor { kind: 1, mode, target: String::new(), x, y }
    }

    #[test]
    fn nine_points_follow_the_numeric_grid() {
        let rect = Rect::new(10, 20, 100, 40);
        assert_eq!(Point::from_code(1).unwrap().at(rect), (10, 20));
        assert_eq!(Point::from_code(5).unwrap().at(rect), (60, 40));
        assert_eq!(Point::from_code(9).unwrap().at(rect), (110, 60));
        assert_eq!(Point::from_code(0), None);
    }

    #[test]
    fn bottom_right_anchor_places_the_login_menu_at_the_corner() {
        let viewport = Rect::new(0, 0, 1280, 960);
        let rect = place(Some(&anchor([9, 9], -9.0, -61.0)), 115, 175, viewport, None);
        assert_eq!(rect, Rect::new(1280 - 9 - 115, 960 - 61 - 175, 115, 175));
    }

    #[test]
    fn anchor_to_a_sibling_uses_its_right_edge() {
        let label = Rect::new(19, 16, 50, 12);
        let rect = place(Some(&anchor([1, 3], 8.0, -2.0)), 154, 17, Rect::new(0, 0, 257, 186), Some(label));
        assert_eq!(rect, Rect::new(77, 14, 154, 17));
    }

    #[test]
    fn scaled_size_multiplies_the_parent_and_adds_the_extent() {
        let size = Size::Scaled { tag: String::new(), scale: [0.97, 1.0], extent: [-10, -10] };
        assert_eq!(size_of(Some(&size), Rect::new(0, 0, 257, 210)), (239, 200));
    }

    #[test]
    fn a_full_gauge_tiles_its_four_pixel_center_texture_instead_of_stretching_one_copy_across_the_whole_bar() {
        let sizes: HashMap<&str, (i32, i32)> =
            HashMap::from([("center", (4, 16)), ("left", (4, 16)), ("right", (4, 16)), ("bg_center", (4, 16)), ("bg_left", (4, 16)), ("bg_right", (4, 16))]);
        let texture_size = |name: &str| sizes.get(name).copied();
        let mut list = DrawList::default();
        let mut missing = Vec::new();
        bar(&mut list, Rect::new(19, 36, 155, 13), ["center", "left", "right", "bg_center", "bg_left", "bg_right"], Some(1.0), &texture_size, &mut missing);
        let center_quads = list.quads.iter().filter(|quad| quad.texture.as_str() == "center").count();
        assert!(center_quads > 1, "textura center de 4px deveria se repetir (tile) ao longo da barra de 155px, não esticar em 1 quad só: {center_quads} quads");
        let covered: f32 = list.quads.iter().filter(|quad| quad.texture.as_str() == "center").map(|quad| quad.w).sum();
        let middle = 155 - 4 - 4;
        assert!(covered >= middle as f32, "repetição da textura center não cobre a região central da barra (esperado >= {middle}): soma de larguras {covered}");
    }

    #[test]
    fn a_window_background_tiles_in_both_directions_instead_of_stretching_one_copy_across_the_whole_window() {
        let sizes: HashMap<&str, (i32, i32)> = HashMap::from([("bg", (128, 128))]);
        let texture_size = |name: &str| sizes.get(name).copied();
        let mut list = DrawList::default();
        let mut missing = Vec::new();
        tile_area(&mut list, Rect::new(0, 0, 446, 394), "bg", &texture_size, &mut missing);
        let quads: Vec<_> = list.quads.iter().filter(|quad| quad.texture.as_str() == "bg").collect();
        assert!(quads.len() > 1, "fundo de 128x128 deveria se repetir (tile) numa janela de 446x394, não esticar em 1 quad só: {} quads", quads.len());
        assert!(quads.iter().all(|quad| quad.w <= 128.0 && quad.h <= 128.0), "cada ladrilho deveria ficar no tamanho nativo (até 128x128), nunca esticado além disso: {quads:?}");
        let max_x = quads.iter().map(|quad| quad.x + quad.w).fold(0.0_f32, f32::max);
        let max_y = quads.iter().map(|quad| quad.y + quad.h).fold(0.0_f32, f32::max);
        assert!(max_x >= 446.0, "ladrilhos não cobrem a largura toda da janela: alcançou {max_x}");
        assert!(max_y >= 394.0, "ladrilhos não cobrem a altura toda da janela: alcançou {max_y}");
    }
}
