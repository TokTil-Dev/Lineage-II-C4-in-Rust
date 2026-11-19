use crate::color::Argb;
use crate::window::Rect;

pub const TAB_DEFAULT_TEX_NORMAL: &str = "Default.BlackTexture";
pub const TAB_DEFAULT_TEX_PUSHED: &str = "Default.Darkyellow";
pub const TAB_AUTO_WIDTH_PAD: i32 = 10;
pub const TAB_AUTO_HEIGHT_PAD: i32 = 5;
pub const TAB_LABEL_COLOR: Argb = Argb(0xFFE6DCBE);

pub const SCROLL_BAR_WIDTH: i32 = 15;
pub const SCROLL_BUTTON_SIZE: i32 = 15;
pub const SCROLL_PIECE_HEIGHT: i32 = 8;
pub const SCROLL_THUMB_MIN: i32 = 15;
pub const SCROLL_UP_BTN: &str = "L2UI_CH3.ScrollBar.ScrollBarUpBtn";
pub const SCROLL_UP_BTN_ON: &str = "L2UI_CH3.ScrollBar.ScrollBarUpOnBtn";
pub const SCROLL_DOWN_BTN: &str = "L2UI_CH3.ScrollBar.ScrollBarDownBtn";
pub const SCROLL_DOWN_BTN_ON: &str = "L2UI_CH3.ScrollBar.ScrollBarDownOnBtn";
pub const SCROLL_THUMB_TOP: &str = "L2UI_CH3.ScrollBar.SliderBarTop";
pub const SCROLL_THUMB_CENTER: &str = "L2UI_CH3.ScrollBar.SliderBarCenter";
pub const SCROLL_THUMB_BOTTOM: &str = "L2UI_CH3.ScrollBar.SliderBarBottom";

pub const RADIO_TEX_UNCHECKED: &str = "L2UI_CH3.RadioCtrl.RadioButton1";
pub const RADIO_TEX_CHECKED: &str = "L2UI_CH3.RadioCtrl.RadioButton2";
pub const RADIO_BOX_SIZE: i32 = 12;
pub const RADIO_TEXT_GAP: i32 = 4;
pub const RADIO_TEXT_COLOR: Argb = Argb(0xFFDCDCDC);

pub const EFFECT_GLOW_SIZE: i32 = 32;
pub const EFFECT_FRAME_SIZE: i32 = 128;
pub const EFFECT_FRAME_OFFSET: i32 = -48;

pub const SLIDER_TICK_SIZE: (i32, i32) = (8, 16);
pub const SLIDER_TICK_OFFSET: i32 = -4;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabSpec {
    pub width: i32,
    pub height: i32,
    pub gap: i32,
    pub label_size: (i32, i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabHeader {
    pub rect: Rect,
    pub selected: bool,
}

pub fn tab_headers(tabs: &[TabSpec], selected: usize) -> Vec<TabHeader> {
    let mut x = 0;
    tabs.iter()
        .enumerate()
        .map(|(index, tab)| {
            let (width, height) = if tab.width == 0 || tab.height == 0 {
                (tab.label_size.0 + TAB_AUTO_WIDTH_PAD, tab.label_size.1 + TAB_AUTO_HEIGHT_PAD)
            } else {
                (tab.width, tab.height)
            };
            let rect = Rect::new(x, 0, width, height);
            x += width + tab.gap;
            TabHeader { rect, selected: index == selected }
        })
        .collect()
}

pub fn tab_label_origin(header: &TabHeader, label_size: (i32, i32)) -> (i32, i32) {
    (
        header.rect.x + (header.rect.w as f32 * 0.5 - (label_size.0 / 2) as f32) as i32,
        header.rect.y + (header.rect.h as f32 * 0.5 - (label_size.1 / 2) as f32) as i32,
    )
}

pub fn tab_texture<'a>(normal: &'a str, pushed: &'a str, selected: bool) -> &'a str {
    let normal = if normal.is_empty() { TAB_DEFAULT_TEX_NORMAL } else { normal };
    let pushed = if pushed.is_empty() { TAB_DEFAULT_TEX_PUSHED } else { pushed };
    if selected { pushed } else { normal }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScrollBar {
    pub up: Rect,
    pub down: Rect,
    pub track: Rect,
    pub thumb: Option<Rect>,
}

pub fn scroll_thumb_length(track_height: i32, total: i32, visible: i32) -> i32 {
    if visible < total && total > 0 {
        (((visible as f32 * track_height as f32) / total as f32) as i32).max(SCROLL_THUMB_MIN)
    } else {
        track_height.max(SCROLL_THUMB_MIN)
    }
}

pub fn scroll_thumb_y(track_height: i32, thumb: i32, total: i32, visible: i32, position: i32) -> i32 {
    if visible < total { ((track_height - thumb) * position) / (total - visible) } else { 0 }
}

pub fn scroll_bar(area: (i32, i32), total: i32, visible: i32, position: i32) -> ScrollBar {
    let (w, h) = area;
    let x = w - SCROLL_BAR_WIDTH;
    let track = Rect::new(x, SCROLL_BUTTON_SIZE, SCROLL_BAR_WIDTH, h - 2 * SCROLL_BUTTON_SIZE);
    let thumb = (visible < total).then(|| {
        let length = scroll_thumb_length(track.h, total, visible);
        Rect::new(x, track.y + scroll_thumb_y(track.h, length, total, visible, position), SCROLL_BAR_WIDTH, length)
    });
    ScrollBar {
        up: Rect::new(x, 0, SCROLL_BUTTON_SIZE, SCROLL_BUTTON_SIZE),
        down: Rect::new(x, h - SCROLL_BUTTON_SIZE, SCROLL_BUTTON_SIZE, SCROLL_BUTTON_SIZE),
        track,
        thumb,
    }
}

pub fn scroll_thumb_pieces(thumb: Rect) -> [(Rect, &'static str); 3] {
    [
        (Rect::new(thumb.x, thumb.y, SCROLL_BAR_WIDTH, SCROLL_PIECE_HEIGHT), SCROLL_THUMB_TOP),
        (
            Rect::new(thumb.x, thumb.y + SCROLL_PIECE_HEIGHT, SCROLL_BAR_WIDTH, thumb.h - 2 * SCROLL_PIECE_HEIGHT),
            SCROLL_THUMB_CENTER,
        ),
        (
            Rect::new(thumb.x, thumb.y + thumb.h - SCROLL_PIECE_HEIGHT, SCROLL_BAR_WIDTH, SCROLL_PIECE_HEIGHT),
            SCROLL_THUMB_BOTTOM,
        ),
    ]
}

pub fn scroll_content_offset(position: i32) -> i32 {
    -position
}

pub fn radio_texture(checked: bool) -> &'static str {
    if checked { RADIO_TEX_CHECKED } else { RADIO_TEX_UNCHECKED }
}

pub fn radio_layout(left_aligned: bool, label_width: i32) -> (Rect, i32) {
    if left_aligned {
        (Rect::new(label_width + RADIO_TEXT_GAP, 0, RADIO_BOX_SIZE, RADIO_BOX_SIZE), 0)
    } else {
        (Rect::new(0, 0, RADIO_BOX_SIZE, RADIO_BOX_SIZE), RADIO_BOX_SIZE + RADIO_TEXT_GAP)
    }
}

pub fn effect_glow_rect() -> Rect {
    Rect::new(0, 0, EFFECT_GLOW_SIZE, EFFECT_GLOW_SIZE)
}

pub fn effect_frames(progress_percent: i32) -> (Rect, Rect) {
    let size = (progress_percent << 7) / 100;
    let inner = 16 - size / 2;
    (
        Rect::new(EFFECT_FRAME_OFFSET, EFFECT_FRAME_OFFSET, EFFECT_FRAME_SIZE, EFFECT_FRAME_SIZE),
        Rect::new(inner, inner, size, size),
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliderSpec {
    pub size: (i32, i32),
    pub num_of_tick: i32,
    pub current_tick: i32,
    pub thumb: (i32, i32),
    pub push_button: Option<(i32, i32)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliderLayout {
    pub ticks: i32,
    pub length: i32,
    pub step: i32,
    pub tick_rects: Vec<Rect>,
    pub thumb: Rect,
    pub push_left: Option<Rect>,
    pub push_right: Option<Rect>,
}

pub fn slider_layout(spec: &SliderSpec) -> SliderLayout {
    let (w, h) = spec.size;
    let ticks = spec.num_of_tick.max(2);
    let length = match spec.push_button {
        Some((push_w, _)) => w - push_w * 2,
        None => w,
    };
    let step = length / (ticks - 1);
    let tick_rects = (0..ticks)
        .map(|i| Rect::new(step * i + SLIDER_TICK_OFFSET, 0, SLIDER_TICK_SIZE.0, SLIDER_TICK_SIZE.1))
        .collect();
    let half = spec.thumb.0 / 2;
    let tick = spec.current_tick;
    let mut x = ((w - length) as f32 * 0.5 + (step * tick) as f32 - half as f32) as i32;
    if tick == 0 {
        x += half;
    } else if tick == ticks - 1 {
        x -= half;
    }
    let y = ((h - spec.thumb.1) as f32 * 0.5) as i32;
    let thumb = Rect::new(x, y, spec.thumb.0, spec.thumb.1);
    let push = spec.push_button.map(|(pw, ph)| {
        let py = ((h - ph) as f32 * 0.5) as i32;
        (Rect::new(0, py, pw, ph), Rect::new(w - pw, py, pw, ph))
    });
    SliderLayout {
        ticks,
        length,
        step,
        tick_rects,
        thumb,
        push_left: push.map(|(left, _)| left),
        push_right: push.map(|(_, right)| right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tab_buttons_follow_each_other_by_width_plus_gap_starting_at_zero() {
        let tabs = vec![
            TabSpec { width: 55, height: 28, gap: 1, label_size: (20, 12) },
            TabSpec { width: 55, height: 28, gap: 1, label_size: (20, 12) },
            TabSpec { width: 0, height: 0, gap: 3, label_size: (30, 12) },
        ];
        let headers = tab_headers(&tabs, 1);
        assert_eq!(headers[0].rect, Rect::new(0, 0, 55, 28));
        assert_eq!(headers[1].rect, Rect::new(56, 0, 55, 28));
        assert_eq!(headers[2].rect, Rect::new(112, 0, 40, 17));
        assert!(!headers[0].selected && headers[1].selected && !headers[2].selected);
        assert_eq!(tab_label_origin(&headers[0], (20, 12)), (17, 8));
        assert_eq!(tab_texture("", "", true), TAB_DEFAULT_TEX_PUSHED);
        assert_eq!(tab_texture("A", "B", false), "A");
    }

    #[test]
    fn scroll_bar_sits_at_the_right_edge_with_fifteen_pixel_buttons() {
        let bar = scroll_bar((200, 100), 400, 100, 0);
        assert_eq!(bar.up, Rect::new(185, 0, 15, 15));
        assert_eq!(bar.down, Rect::new(185, 85, 15, 15));
        assert_eq!(bar.track, Rect::new(185, 15, 15, 70));
        assert_eq!(bar.thumb, Some(Rect::new(185, 15, 15, 17)));
        let moved = scroll_bar((200, 100), 400, 100, 300);
        assert_eq!(moved.thumb, Some(Rect::new(185, 68, 15, 17)));
        assert_eq!(scroll_bar((200, 100), 50, 100, 0).thumb, None);
        assert_eq!(scroll_thumb_length(70, 1000, 10), 15);
        let pieces = scroll_thumb_pieces(Rect::new(185, 15, 15, 40));
        assert_eq!(pieces[0].0, Rect::new(185, 15, 15, 8));
        assert_eq!(pieces[1].0, Rect::new(185, 23, 15, 24));
        assert_eq!(pieces[2].0, Rect::new(185, 47, 15, 8));
        assert_eq!(scroll_content_offset(30), -30);
    }

    #[test]
    fn radio_buttons_are_twelve_pixel_check_boxes_with_the_radio_textures() {
        assert_eq!(radio_texture(true), RADIO_TEX_CHECKED);
        assert_eq!(radio_layout(false, 40), (Rect::new(0, 0, 12, 12), 16));
        assert_eq!(radio_layout(true, 40), (Rect::new(44, 0, 12, 12), 0));
    }

    #[test]
    fn effect_frames_grow_from_the_button_centre() {
        assert_eq!(effect_glow_rect(), Rect::new(0, 0, 32, 32));
        let (outer, inner) = effect_frames(50);
        assert_eq!(outer, Rect::new(-48, -48, 128, 128));
        assert_eq!(inner, Rect::new(-16, -16, 64, 64));
    }

    #[test]
    fn slider_ticks_and_thumb_follow_the_dll_arithmetic() {
        let spec = SliderSpec { size: (100, 20), num_of_tick: 5, current_tick: 2, thumb: (10, 16), push_button: None };
        let layout = slider_layout(&spec);
        assert_eq!((layout.ticks, layout.length, layout.step), (5, 100, 25));
        assert_eq!(layout.tick_rects[1], Rect::new(21, 0, 8, 16));
        assert_eq!(layout.thumb, Rect::new(45, 2, 10, 16));
        let first = slider_layout(&SliderSpec { current_tick: 0, ..spec.clone() });
        assert_eq!(first.thumb.x, 0);
        let last = slider_layout(&SliderSpec { current_tick: 4, ..spec.clone() });
        assert_eq!(last.thumb.x, 90);
        let pushed = slider_layout(&SliderSpec { push_button: Some((12, 12)), ..spec });
        assert_eq!((pushed.length, pushed.step), (76, 19));
        assert_eq!(pushed.push_left, Some(Rect::new(0, 4, 12, 12)));
        assert_eq!(pushed.push_right, Some(Rect::new(88, 4, 12, 12)));
        assert_eq!(pushed.thumb.x, 12 + 38 - 5);
    }
}
