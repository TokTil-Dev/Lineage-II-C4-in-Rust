use killer_ui::draw::DrawList;
use killer_ui::font::BitmapFont;
use killer_ui::lobby_select::{HostLayout, Hit as SelectHit, SelectScreen};
use killer_ui::window::Rect;
use killer_ui::xdat_window::{self, Layout, Paint};
use l2_dat::xdat::{Control, Interface, Props, WindowDef};
use std::collections::HashMap;

pub const LOGIN_MENU: &str = "LoginMenuWnd";
pub const CHARACTER_WINDOWS: [&str; 4] = ["LobbyMenuWnd", "LobbyStartWnd", "LobbyCharacterSelectWnd", "LobbyCreateMenuWnd"];
pub const LOGIN_FUNCTIONS: [(&str, usize); 5] = [("btnNewAccount", 0), ("btnLossAccount", 1), ("btnOption", 2), ("btnCredit", 3), ("btnReplay", 4)];
pub const CREATE_WINDOWS: [&str; 4] = ["CharacterCreateMenuWnd", "CharacterCreateSetupWnd", "CharacterCreateClassInfoWnd", "CharacterCreateFunctionWnd"];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterfaceKind {
    PortedC4,
    Xdat,
}

impl InterfaceKind {
    pub fn for_chronicle(chronicle: l2_dat::Chronicle) -> InterfaceKind {
        match chronicle {
            l2_dat::Chronicle::HighFive => InterfaceKind::Xdat,
            _ => InterfaceKind::PortedC4,
        }
    }
}

pub struct H5Lobby {
    interface: Interface,
    sizes: HashMap<String, Option<(i32, i32)>>,
    primed: bool,
    reported: std::cell::RefCell<Vec<String>>,
    pub missing: std::cell::RefCell<Vec<String>>,
}

fn collect(controls: &[Control], out: &mut Vec<String>) {
    for control in controls {
        match &control.props {
            Props::Button(button) => out.extend(button.textures.iter().cloned()),
            Props::Texture(texture) => out.push(texture.texture.clone()),
            Props::StatusBar(gauge) | Props::InvenWeight(gauge) => out.extend(gauge.textures.iter().cloned()),
            Props::BarCtrl { textures, .. } => out.extend(textures.iter().cloned()),
            _ => {}
        }
        collect(&control.children, out);
    }
}

fn key(name: &str) -> String {
    name.to_ascii_lowercase()
}

impl H5Lobby {
    pub fn new(interface: Interface) -> H5Lobby {
        H5Lobby { interface, sizes: HashMap::new(), primed: false, reported: Default::default(), missing: Default::default() }
    }

    fn window(&self, name: &str) -> Option<&WindowDef> {
        self.interface.window(name)
    }

    fn texture_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for window in std::iter::once(LOGIN_MENU).chain(CHARACTER_WINDOWS).chain(CREATE_WINDOWS) {
            if let Some(window) = self.window(window) {
                names.push(window.part.back_tex.clone());
                collect(&window.children, &mut names);
            }
        }
        names.extend(original::texture_names());
        names.retain(|name| xdat_window::UNDEFINED != name && !name.is_empty());
        names
    }

    pub fn prime(&mut self, size_of: &mut dyn FnMut(&str) -> Option<(i32, i32)>) {
        if self.primed {
            return;
        }
        self.primed = true;
        for name in self.texture_names() {
            self.sizes.entry(key(&name)).or_insert_with(|| size_of(&name));
        }
    }

    fn size(&self, name: &str) -> Option<(i32, i32)> {
        self.sizes.get(&key(name)).copied().flatten()
    }

    pub fn original_texture_size(&self, name: &str) -> Option<(i32, i32)> {
        self.size(name)
    }

    fn layouts(&self, names: &[&str], size: (i32, i32)) -> Vec<Layout<'_>> {
        let viewport = Rect::new(0, 0, size.0, size.1);
        let mut placed: Vec<(String, Rect)> = Vec::new();
        let mut out = Vec::new();
        for name in names {
            let Some(window) = self.window(name) else { continue };
            let layout = xdat_window::layout(window, viewport, &|target| placed.iter().find(|(seen, _)| seen == target).map(|(_, rect)| *rect));
            placed.push((layout.name().to_string(), layout.rect));
            out.push(layout);
        }
        out
    }

    pub fn select_host(&self, size: (i32, i32)) -> Option<HostLayout> {
        let layouts = self.layouts(&CHARACTER_WINDOWS, size);
        let rect = |name: &str| layouts.iter().find_map(|layout| layout.find(name)).map(|item| item.rect);
        Some(HostLayout {
            combo: rect("cbCharacterName")?,
            start: rect("btnStartGame")?,
            create: rect("btnCreateCharacter")?,
            delete: rect("btnDeleteCharacter")?,
            exit: rect("btnCancel")?,
        })
    }

    pub fn login_functions(&self, size: (i32, i32)) -> Vec<(usize, Rect)> {
        let layouts = self.layouts(&[LOGIN_MENU], size);
        LOGIN_FUNCTIONS.iter().filter_map(|(name, index)| layouts.iter().find_map(|layout| layout.find(name)).map(|item| (*index, item.rect))).collect()
    }

    pub fn paint_login_menu(&self, list: &mut DrawList, size: (i32, i32), font: &BitmapFont, sysstring: &dyn Fn(u32) -> Option<String>, cursor: (i32, i32), pressed: Option<usize>) {
        let layouts = self.layouts(&[LOGIN_MENU], size);
        let pressed_name = pressed.and_then(|index| LOGIN_FUNCTIONS.iter().find(|(_, other)| *other == index)).map(|(name, _)| *name);
        let texture_size = |name: &str| self.size(name);
        let paint = Paint {
            font,
            texture_size: &texture_size,
            sysstring,
            cursor: Some(cursor),
            pressed: pressed_name,
            disabled: &|_| false,
            hidden: &|_| false,
            gauge: &|_| None,
            value: &|_| None,
            color: &|_| None,
            texture: &|_| None,
            custom: &|_, _| false,
        };
        let mut missing = Vec::new();
        for layout in &layouts {
            missing.extend(xdat_window::paint_layout(layout, &paint, list));
        }
        drop(layouts);
        self.note_missing(missing);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn paint_characters(&self, list: &mut DrawList, size: (i32, i32), select: &SelectScreen, font: &BitmapFont, sysstring: &dyn Fn(u32) -> Option<String>, cursor: (i32, i32), tags: &[killer_ui::lobby_select::NameTag]) {
        select.paint_tags(list, font, tags);
        let layouts = self.layouts(&CHARACTER_WINDOWS, size);
        let card = select.selected.and_then(|index| select.cards.get(index));
        let pressed_name = select.pressed.and_then(|hit| match hit {
            SelectHit::Start => Some("btnStartGame"),
            SelectHit::Create => Some("btnCreateCharacter"),
            SelectHit::Delete => Some("btnDeleteCharacter"),
            SelectHit::Exit => Some("btnCancel"),
            _ => None,
        });
        let value = |name: &str| -> Option<String> {
            let text = match name {
                "txtVarLevel" => card.map(|card| card.level.to_string()),
                "txtVarClassName" => card.map(|card| card.class_name.clone()),
                "txtVarSP" => card.map(|card| card.sp.to_string()),
                "txtVarPropensity" => card.map(|card| card.karma.to_string()),
                _ => return None,
            };
            Some(text.unwrap_or_default())
        };
        let gauge = |name: &str| -> Option<f32> {
            let card = card?;
            match name {
                "sbHPBar" => Some(if card.max_hp > 0 { card.hp as f32 / card.max_hp as f32 } else { 0.0 }),
                "sbMPBar" => Some(if card.max_mp > 0 { card.mp as f32 / card.max_mp as f32 } else { 0.0 }),
                "sbEXPBar" => Some(killer_ui::exp::exp_percent(card.level, card.exp) / 100.0),
                _ => None,
            }
        };
        let texture_size = |name: &str| self.size(name);
        let disabled = |name: &str| match name {
            "btnStartGame" => !select.start_enabled,
            "btnDeleteCharacter" => !select.delete_enabled,
            "btnCreateCharacter" | "btnCancel" => select.locked,
            _ => false,
        };
        let hidden = |name: &str| name.starts_with("Vitality");
        let paint = Paint {
            font,
            texture_size: &texture_size,
            sysstring,
            cursor: Some(cursor),
            pressed: pressed_name,
            disabled: &disabled,
            hidden: &hidden,
            gauge: &gauge,
            value: &value,
            color: &|_| None,
            texture: &|_| None,
            custom: &|_, _| false,
        };
        let mut missing = Vec::new();
        for layout in &layouts {
            missing.extend(xdat_window::paint_layout(layout, &paint, list));
        }
        drop(layouts);
        self.paint_combo_text(list, select, font);
        if select.dialog.is_some() {
            self.report("diálogo de confirmação do lobby (sem janela no xdat)");
        }
        self.note_missing(missing);
    }

    pub fn report(&self, what: &str) {
        let mut seen = self.reported.borrow_mut();
        if !seen.iter().any(|item| item == what) {
            log::warn!("janela {what} do H5 sem definição no xdat/ainda não portada");
            seen.push(what.to_string());
        }
    }

    fn paint_combo_text(&self, list: &mut DrawList, select: &SelectScreen, font: &BitmapFont) {
        self.report("moldura do ComboBox cbCharacterName (o xdat só traz o controle)");
        let rect = select.combo.rect;
        let line = font.line_height as i32;
        if let Some(text) = select.combo.selected_text() {
            list.draw_text(rect.x + select.combo.indent, rect.y + (rect.h - line) / 2, killer_ui::color::TEXT, text, font);
        }
        if select.combo.open {
            for (index, text) in select.combo.entries.iter().enumerate() {
                list.draw_text(rect.x + select.combo.indent, rect.y + rect.h + select.combo.row_height * index as i32, killer_ui::color::TEXT, text, font);
            }
        }
    }

    pub fn paint_create(&self, list: &mut DrawList, size: (i32, i32), font: &BitmapFont, sysstring: &dyn Fn(u32) -> Option<String>, cursor: (i32, i32), rotate: (bool, bool)) {
        let layouts = self.layouts(&CREATE_WINDOWS, size);
        let texture_size = |name: &str| self.size(name);
        let hidden = |name: &str| create_button_hidden(name, rotate.0, rotate.1);
        let paint = Paint {
            font,
            texture_size: &texture_size,
            sysstring,
            cursor: Some(cursor),
            pressed: None,
            disabled: &|_| false,
            hidden: &hidden,
            gauge: &|_| None,
            value: &|_| None,
            color: &|_| None,
            texture: &|_| None,
            custom: &|_, _| false,
        };
        let mut missing = Vec::new();
        for layout in &layouts {
            missing.extend(xdat_window::paint_layout(layout, &paint, list));
        }
        drop(layouts);
        self.report("EditBox ebName e ComboBox cb* de CharacterCreateSetupWnd (o xdat só traz os controles)");
        self.report("lógica de criação (seletores, câmera, envio) ligada às janelas CharacterCreate* do xdat");
        self.note_missing(missing);
    }

    fn note_missing(&self, names: Vec<String>) {
        let mut seen = self.missing.borrow_mut();
        for name in names {
            if !seen.contains(&name) {
                log::warn!("lobby H5: textura do xdat ausente no catálogo: {name}");
                seen.push(name);
            }
        }
    }
}

fn create_button_hidden(name: &str, rotate_visible: bool, zoomed: bool) -> bool {
    match name {
        "btnRotateLeft" | "btnRotateRight" => !rotate_visible,
        "btnZoomIn" => !rotate_visible || zoomed,
        "btnZoomOut" => !rotate_visible || !zoomed,
        _ => false,
    }
}

#[cfg(test)]
mod create_buttons {
    use super::create_button_hidden;

    #[test]
    fn rotate_and_zoom_follow_enable_rotate_and_the_zoom_toggle() {
        for name in ["btnRotateLeft", "btnRotateRight", "btnZoomIn", "btnZoomOut"] {
            assert!(create_button_hidden(name, false, false));
            assert!(create_button_hidden(name, false, true));
        }
        assert!(!create_button_hidden("btnRotateLeft", true, false));
        assert!(!create_button_hidden("btnZoomIn", true, false));
        assert!(create_button_hidden("btnZoomOut", true, false));
        assert!(create_button_hidden("btnZoomIn", true, true));
        assert!(!create_button_hidden("btnZoomOut", true, true));
        assert!(!create_button_hidden("btnRequestCreateCharacter", false, false));
    }
}

pub mod original {
    use killer_ui::chat::{CARET, CARET_BACK};
    use killer_ui::color::{Argb, TEXT};
    use killer_ui::draw::DrawList;
    use killer_ui::font::BitmapFont;
    use killer_ui::lobby_credit::{CreditEntry, CreditScreen};
    use killer_ui::lobby_login::{self, Field, Hit as LoginHit, LoginScreen};
    use killer_ui::lobby_notice::{self, Hit as NoticeHit, Kind as NoticeKind, Notice};
    use killer_ui::lobby_servers::{self, Hit as ServerHit, ServerSelect, Status};
    use killer_ui::npc_viewer::paint_html_lines;
    use killer_ui::window::Rect;

    pub const WINDOW_BACK: &str = "L2UI_CT1.Windows.Windows_DF_Drawer_Bg";
    pub const BUTTON: &str = "L2UI_CT1.Button.Button_DF";
    pub const EDIT_BACK: &str = "L2UI_CT1.EditBox.EditBox_DF";
    pub const SERVER_BACK: &str = "L2UI_CT1.Misc.MISC_DF_SERVERSELECT_BACK";
    pub const CREDIT_SECONDS: u32 = 6;

    pub type TextureSize<'a> = &'a dyn Fn(&str) -> Option<(i32, i32)>;

    pub fn credit_slide(index: u32) -> String {
        format!("L2UI_CT1.SlideShow.SlideShow_DF_Credit_{:02}", index + 1)
    }

    pub fn credit_entries() -> Vec<CreditEntry> {
        (0..10).map(|index| CreditEntry { id: index, html: String::new(), image: credit_slide(index), time: CREDIT_SECONDS, align: 0 }).collect()
    }

    pub fn texture_names() -> Vec<String> {
        let mut names = vec![WINDOW_BACK.to_string(), BUTTON.to_string(), EDIT_BACK.to_string(), SERVER_BACK.to_string()];
        names.extend((0..10).map(credit_slide));
        names
    }

    fn blit(list: &mut DrawList, rect: Rect, name: &str, texture_size: TextureSize) {
        if rect.w <= 0 || rect.h <= 0 {
            return;
        }
        if let Some((tw, th)) = texture_size(name) {
            list.draw_texture(rect.x, rect.y, rect.w, rect.h, 0, 0, tw, th, name, 1.0);
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn button(list: &mut DrawList, ox: i32, oy: i32, rect: Rect, label: &str, pressed: bool, font: &BitmapFont, texture_size: TextureSize) {
        let placed = Rect::new(ox + rect.x, oy + rect.y, rect.w, rect.h);
        if let Some((tw, th)) = texture_size(BUTTON) {
            list.draw_texture(placed.x, placed.y, placed.w, placed.h, 0, 0, tw, th, BUTTON, if pressed { 0.75 } else { 1.0 });
        }
        if !label.is_empty() {
            let width = font.measure(label) as i32;
            let x = placed.x + (placed.w - width) / 2;
            let y = placed.y + (placed.h - font.line_height as i32) / 2;
            list.draw_text(x, y, TEXT, label, font);
        }
    }

    pub fn paint_login_form(list: &mut DrawList, screen: &LoginScreen, font: &BitmapFont, texture_size: TextureSize) {
        if !screen.auth_visible {
            return;
        }
        let (ox, oy) = screen.auth_origin();
        blit(list, Rect::new(ox, oy, lobby_login::AUTH_SIZE.0, lobby_login::AUTH_SIZE.1), WINDOW_BACK, texture_size);
        if let Some(card) = screen.card {
            let text = screen.card_prompt.replace("$s1", &card.position.to_string());
            list.draw_text(ox + 10, oy + 16, TEXT, &text, font);
        } else {
            for (label, rect) in [(&screen.labels.id, lobby_login::AUTH_ID_RECT), (&screen.labels.password, lobby_login::AUTH_PASSWORD_RECT)] {
                if label.is_empty() {
                    continue;
                }
                let width = font.measure(label) as i32;
                list.draw_text(ox + rect.x - width - 6, oy + rect.y, TEXT, label, font);
            }
        }
        let masked: String = "*".repeat(screen.password.text.chars().count());
        for (field, rect, text) in [(Field::Id, lobby_login::AUTH_ID_RECT, screen.id.text.as_str()), (Field::Password, lobby_login::AUTH_PASSWORD_RECT, masked.as_str())] {
            if field == Field::Id && screen.card.is_some() {
                continue;
            }
            let placed = Rect::new(ox + rect.x, oy + rect.y, rect.w, rect.h);
            blit(list, placed, EDIT_BACK, texture_size);
            let (tx, ty) = (placed.x + 2, placed.y + 2);
            list.draw_text_clipped(tx, ty, placed.w - 2, TEXT, text, font);
            if screen.focus == field {
                let before = match field {
                    Field::Id => screen.id.before_cursor().to_string(),
                    Field::Password => "*".repeat(screen.password.before_cursor().chars().count()),
                    Field::Login => String::new(),
                };
                list.draw_text(tx + font.measure(&before) as i32 - CARET_BACK, ty, TEXT, &CARET.to_string(), font);
            }
        }
        button(list, ox, oy, lobby_login::AUTH_LOGIN_RECT, &screen.labels.login, screen.pressed == Some(LoginHit::Login), font, texture_size);
        button(list, ox, oy, lobby_login::AUTH_EXIT_RECT, &screen.labels.exit, screen.pressed == Some(LoginHit::Exit), font, texture_size);
    }

    fn server_status(screen: &ServerSelect, status: Status) -> (String, Argb) {
        match status {
            Status::Down => (screen.labels.down.clone(), lobby_servers::RED),
            Status::Full => (screen.labels.full.clone(), lobby_servers::FULL_COLOR),
            Status::Normal => (screen.labels.normal.clone(), lobby_servers::NORMAL_COLOR),
            Status::Good => (screen.labels.good.clone(), lobby_servers::GOOD_COLOR),
            Status::Unknown => (String::new(), TEXT),
        }
    }

    fn paint_server_header(list: &mut DrawList, screen: &ServerSelect, font: &BitmapFont, ox: i32, oy: i32, texture_size: TextureSize) {
        let mut x = ox + lobby_servers::LIST_RECT.x;
        let y = oy + lobby_servers::LIST_RECT.y;
        for (index, width) in lobby_servers::COLUMN_WIDTHS.iter().enumerate() {
            let rect = Rect::new(x, y, *width, lobby_servers::HEADER_HEIGHT);
            blit(list, rect, BUTTON, texture_size);
            let label = &screen.labels.header[index];
            let tx = x + width / 2 - font.measure(label) as i32 / 2;
            let ty = y + lobby_servers::HEADER_HEIGHT / 2 - font.line_height as i32 / 2;
            list.draw_text(tx, ty, TEXT, label, font);
            x += width;
        }
    }

    fn paint_server_rows(list: &mut DrawList, screen: &ServerSelect, font: &BitmapFont, ox: i32, oy: i32) {
        let top = oy + lobby_servers::LIST_RECT.y + lobby_servers::HEADER_HEIGHT;
        for (offset, entry) in screen.entries.iter().enumerate().skip(screen.first_row).take(ServerSelect::visible_rows()) {
            let y = top + (offset - screen.first_row) as i32 * lobby_servers::ROW_HEIGHT;
            if screen.selected == Some(offset) {
                list.draw_text(ox + lobby_servers::LIST_RECT.x + 1, y + 2, TEXT, ">", font);
            }
            let number = if entry.flags & lobby_servers::SERVER_GM_FLAG != 0 { String::new() } else { lobby_servers::server_number(entry.id, entry.flags) };
            let (status_text, status_color) = server_status(screen, entry.load_status());
            let cells = [(number, TEXT), (entry.name.clone(), if entry.forbidden { lobby_servers::RED } else { TEXT }), (status_text, status_color), (entry.ping.to_string(), TEXT)];
            let mut x = ox + lobby_servers::LIST_RECT.x + 12;
            for ((text, color), width) in cells.into_iter().zip(lobby_servers::COLUMN_WIDTHS) {
                if !text.is_empty() {
                    list.draw_text_clipped(x, y + 3, width - 12, color, &text, font);
                }
                x += width;
            }
        }
    }

    pub fn paint_servers(list: &mut DrawList, screen: &ServerSelect, font: &BitmapFont, texture_size: TextureSize) {
        let (ox, oy) = screen.origin();
        blit(list, Rect::new(ox, oy, lobby_servers::CONTAINER_SIZE.0, lobby_servers::CONTAINER_SIZE.1), SERVER_BACK, texture_size);
        paint_server_header(list, screen, font, ox, oy, texture_size);
        paint_server_rows(list, screen, font, ox, oy);
        let pressed = |hit: ServerHit| screen.pressed == Some(hit);
        button(list, ox, oy, lobby_servers::QUALITY_RECT, &screen.labels.quality, pressed(ServerHit::Quality), font, texture_size);
        button(list, ox, oy, lobby_servers::OK_RECT, &screen.labels.ok, pressed(ServerHit::Ok), font, texture_size);
        button(list, ox, oy, lobby_servers::CANCEL_RECT, &screen.labels.cancel, pressed(ServerHit::Cancel), font, texture_size);
        paint_html_lines(list, ox + lobby_servers::HELP_RECT.x, oy + lobby_servers::HELP_RECT.y, (lobby_servers::HELP_RECT.w, lobby_servers::HELP_RECT.h), &screen.help_lines, screen.help_scroll, font, 1.0);
    }

    pub fn paint_notice(list: &mut DrawList, screen: &Notice, font: &BitmapFont, texture_size: TextureSize) {
        let (ox, oy) = screen.origin();
        blit(list, Rect::new(ox, oy, lobby_notice::SIZE.0, lobby_notice::SIZE.1), WINDOW_BACK, texture_size);
        if screen.kind != NoticeKind::Announce && !screen.text.is_empty() {
            let rx = ox + lobby_notice::READER_RECT.x;
            let ry = oy + lobby_notice::READER_RECT.y;
            let lines = killer_ui::text_align::bitmap_lines(&screen.text, lobby_notice::READER_RECT.w - 4, font, 1, false);
            let start = list.quads.len();
            killer_ui::text_align::paint(list, (rx + 2, ry + 2 + screen.scroll.offset), &lines, TEXT, font, true);
            list.clip_from(start, lobby_notice::READER_RECT.offset(ox, oy));
        }
        let buttons: Vec<(Rect, &str, NoticeHit)> = match screen.kind {
            NoticeKind::Eula => vec![(lobby_notice::AGREE_RECT, screen.labels.agree.as_str(), NoticeHit::Agree), (lobby_notice::DISAGREE_RECT, screen.labels.disagree.as_str(), NoticeHit::Disagree)],
            NoticeKind::Warning | NoticeKind::Announce => vec![(lobby_notice::OK_RECT, screen.labels.ok.as_str(), NoticeHit::Ok)],
        };
        for (rect, label, hit) in buttons {
            button(list, ox, oy, rect, label, screen.pressed == Some(hit), font, texture_size);
        }
    }

    pub fn paint_credits(list: &mut DrawList, screen: &CreditScreen, size: (i32, i32), texture_size: TextureSize) {
        if !screen.visible {
            return;
        }
        let Some(entry) = screen.entry() else { return };
        if let Some((tw, th)) = texture_size(&entry.image) {
            list.draw_texture(0, 0, size.0, size.1, 0, 0, tw, th, &entry.image, 1.0);
        }
    }
}
