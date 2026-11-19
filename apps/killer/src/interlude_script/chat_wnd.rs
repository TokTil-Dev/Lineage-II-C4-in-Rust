use super::{UiApi, WindowScript, param_add, parse_int, parse_string};
use killer_ui::color::Argb;

pub const INTRO_TIMER: i32 = 1;
pub const CHAT_UNION_MAX: usize = 35;
pub const CHAT_WINDOW_SYSTEM: i32 = 5;
pub const CHAT_WINDOW_COUNT: i32 = 5;
pub const CHAT_WINDOW_ALLY: i32 = 4;
pub const CHAT_WINDOW_CLAN: i32 = 3;
pub const CHAT_WINDOW_PARTY: i32 = 2;
pub const CHAT_WINDOW_TRADE: i32 = 1;
pub const CHAT_WINDOW_NORMAL: i32 = 0;

pub const CHAT_NORMAL: i32 = 0;
pub const CHAT_SHOUT: i32 = 1;
pub const CHAT_TELL: i32 = 2;
pub const CHAT_PARTY: i32 = 3;
pub const CHAT_CLAN: i32 = 4;
pub const CHAT_SYSTEM: i32 = 5;
pub const CHAT_USER_PET: i32 = 6;
pub const CHAT_GM_PET: i32 = 7;
pub const CHAT_MARKET: i32 = 8;
pub const CHAT_ALLIANCE: i32 = 9;
pub const CHAT_ANNOUNCE: i32 = 10;
pub const CHAT_COMMANDER_CHAT: i32 = 15;
pub const CHAT_INTER_PARTYMASTER_CHAT: i32 = 16;
pub const CHAT_HERO: i32 = 17;
pub const CHAT_CRITICAL_ANNOUNCE: i32 = 18;

pub const SYSTEM_NONE: i32 = 0;
pub const SYSTEM_BATTLE: i32 = 1;
pub const SYSTEM_SERVER: i32 = 2;
pub const SYSTEM_DAMAGE: i32 = 3;
pub const SYSTEM_PETITION: i32 = 6;
pub const SYSTEM_USEITEMS: i32 = 7;

pub const EVENT_CHAT_MESSAGE: i32 = 540;
pub const EVENT_IME_STATUS_CHANGE: i32 = 1500;
pub const EVENT_CHAT_WND_STATUS_CHANGE: i32 = 550;
pub const EVENT_SET_STRING: i32 = 560;
pub const EVENT_SET_FOCUS: i32 = 570;
pub const EVENT_MSN_STATUS: i32 = 571;
pub const EVENT_MACRO_COMMAND: i32 = 572;
pub const EVENT_ON_SCREEN_MESSAGE: i32 = 140;
pub const REGISTERED_EVENTS: [i32; 7] = [540, 1500, 550, 560, 570, 571, 572];

pub const SECTION_NAMES: [&str; 5] = ["entire_tab", "pledge_tab", "party_tab", "market_tab", "ally_tab"];
pub const CHAT_FILTER_FILE: &str = "chatfilter.ini";
pub const SCROLL_BAR_POSITION: (i32, i32, i32) = (5, 10, -25);
pub const DEFAULT_ANCHOR_OFFSET: (i32, i32) = (0, -17);
pub const MERGED_HEIGHT_INSET: i32 = 46;
pub const INTRO_MESSAGE: &str = "Interface developed by Neophron.";
pub const INTRO_MESSAGE_2: &str = "Interface nulled by Grundor.";

pub const CHAT_WND: &str = "ChatWnd";
pub const NORMAL_CHAT: &str = "ChatWnd.NormalChat";
pub const TRADE_CHAT: &str = "ChatWnd.TradeChat";
pub const PARTY_CHAT: &str = "ChatWnd.PartyChat";
pub const CLAN_CHAT: &str = "ChatWnd.ClanChat";
pub const ALLY_CHAT: &str = "ChatWnd.AllyChat";
pub const SYSTEM_MSG_LIST: &str = "SystemMsgWnd.SystemMsgList";
pub const CHAT_TAB_CTRL: &str = "ChatWnd.ChatTabCtrl";
pub const CHAT_EDIT_BOX: &str = "ChatWnd.ChatEditBox";
pub const LANGUAGE_TEXTURE: &str = "ChatWnd.LanguageTexture";
pub const MESSENGER_BTN: &str = "Chatwnd.MessengerBtn";
pub const ALL_BLOCK_BTN: &str = "ChatWnd.BtnAllBlock";
pub const SYSTEM_MSG_WND: &str = "SystemMsgWnd";
pub const CHAT_FILTER_WND: &str = "ChatFilterWnd";
pub const PARTY_MATCH_WND: &str = "PartyMatchWnd";
pub const TAB_BUTTONS: [&str; 5] = ["ChatTabCtrl0", "ChatTabCtrl1", "ChatTabCtrl2", "ChatTabCtrl3", "ChatTabCtrl4"];
pub const TAB_SYSSTRINGS: [i32; 5] = [144, 355, 188, 128, 559];
pub const FILTER_TITLE_SYSMSG: i32 = 1995;

pub const SYSTEM_COLOR: Argb = Argb(0xFFB09B79);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ChatFilterInfo {
    pub system: i32,
    pub chat: i32,
    pub damage: i32,
    pub normal: i32,
    pub shout: i32,
    pub clan: i32,
    pub party: i32,
    pub trade: i32,
    pub whisper: i32,
    pub ally: i32,
    pub useitem: i32,
    pub hero: i32,
    pub union: i32,
}

impl ChatFilterInfo {
    #[allow(clippy::too_many_arguments)]
    const fn new(
        system: i32,
        chat: i32,
        normal: i32,
        shout: i32,
        clan: i32,
        party: i32,
        trade: i32,
        whisper: i32,
        damage: i32,
        ally: i32,
        useitem: i32,
        hero: i32,
        union: i32,
    ) -> ChatFilterInfo {
        ChatFilterInfo { system, chat, damage, normal, shout, clan, party, trade, whisper, ally, useitem, hero, union }
    }
}

pub const DEFAULT_FILTERS: [ChatFilterInfo; 6] = [
    ChatFilterInfo::new(1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1),
    ChatFilterInfo::new(1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0),
    ChatFilterInfo::new(1, 1, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0),
    ChatFilterInfo::new(1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 0),
    ChatFilterInfo::new(1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0),
    ChatFilterInfo::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
];

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Script {
    pub no_union_commander_message: i32,
    pub section_name: Vec<String>,
    pub chat_type: i32,
    pub filter_info: Vec<ChatFilterInfo>,
}

pub fn msn_color(name: &str) -> Argb {
    let (r, g, b) = match name {
        "Yellow" => (255, 255, 0),
        "System" => (176, 155, 121),
        "Amber" => (218, 165, 32),
        "White" => (255, 255, 255),
        "Dim" => (177, 173, 172),
        "Magenta" => (255, 0, 255),
        _ => (0, 0, 0),
    };
    rgb(r, g, b)
}

pub fn rgb(r: i32, g: i32, b: i32) -> Argb {
    Argb(0xFF00_0000 | ((r as u32 & 0xFF) << 16) | ((g as u32 & 0xFF) << 8) | (b as u32 & 0xFF))
}

fn chat_window_of_tab(tab_button: &str) -> Option<&'static str> {
    match tab_button {
        "ChatTabCtrl0" => Some(NORMAL_CHAT),
        "ChatTabCtrl1" => Some(TRADE_CHAT),
        "ChatTabCtrl2" => Some(PARTY_CHAT),
        "ChatTabCtrl3" => Some(CLAN_CHAT),
        "ChatTabCtrl4" => Some(ALLY_CHAT),
        _ => None,
    }
}

fn left(text: &str, count: usize) -> String {
    text.chars().take(count).collect()
}

fn right(text: &str, count: usize) -> String {
    let total = text.chars().count();
    text.chars().skip(total.saturating_sub(count)).collect()
}

impl Script {
    pub fn init_handle(&mut self) {}

    pub fn init_global_setting(&mut self, api: &mut dyn UiApi) {
        api.checkbox_set_check("ChatFilterWnd.CheckBoxCommand", self.no_union_commander_message != 0);
    }

    pub fn init_scroll_bar_position(&mut self, api: &mut dyn UiApi) {
        let (x, y, h) = SCROLL_BAR_POSITION;
        for window in [NORMAL_CHAT, TRADE_CHAT, PARTY_CHAT, CLAN_CHAT, ALLY_CHAT] {
            api.chat_set_scroll_bar_position(window, x, y, h);
        }
    }

    pub fn clear(&mut self, api: &mut dyn UiApi) {
        for window in [CHAT_EDIT_BOX, NORMAL_CHAT, PARTY_CHAT, CLAN_CHAT, TRADE_CHAT, ALLY_CHAT, SYSTEM_MSG_LIST] {
            api.clear(window);
        }
    }

    pub fn init_all_block_tooltip(&mut self, api: &mut dyn UiApi) {
        if api.get_option_bool("Neophron", "AllBlock") {
            api.set_tooltip_simple_text(ALL_BLOCK_BTN, "Block All");
        } else {
            api.set_tooltip_simple_text(ALL_BLOCK_BTN, "Stop Blocking");
        }
    }

    pub fn get_all_block(&mut self, api: &mut dyn UiApi) {
        let all_block = if api.get_option_bool("Neophron", "AllBlock") {
            api.execute_command("/allunblock");
            api.set_tooltip_simple_text(ALL_BLOCK_BTN, "Block All");
            false
        } else {
            api.execute_command("/allblock");
            api.set_tooltip_simple_text(ALL_BLOCK_BTN, "Stop Blocking");
            true
        };
        api.set_option_bool("Neophron", "AllBlock", all_block);
    }

    pub fn handle_tab_click(&mut self, api: &mut dyn UiApi, _id: &str) {
        self.chat_type = api.tab_get_top_index(CHAT_TAB_CTRL);
        self.set_chat_filter_button(api);
        if api.get_option_bool("Game", "OldChatting") {
            let mut input = api.edit_get_string(CHAT_EDIT_BOX);
            let len = input.chars().count();
            let prefix = left(&input, 1);
            if api.is_same_chat_prefix(CHAT_MARKET, &prefix)
                || api.is_same_chat_prefix(CHAT_PARTY, &prefix)
                || api.is_same_chat_prefix(CHAT_CLAN, &prefix)
                || api.is_same_chat_prefix(CHAT_ALLIANCE, &prefix)
            {
                input = right(&input, len.saturating_sub(1));
            }
            if self.chat_type != 0 {
                let prefix = api.get_chat_prefix(self.get_chat_type_by_tab_index(self.chat_type));
                input = format!("{prefix}{input}");
            }
            api.edit_set_string(CHAT_EDIT_BOX, &input);
        }
    }

    pub fn handle_chat_wnd_macro_command(&mut self, api: &mut dyn UiApi, param: &str) {
        let Some(command) = parse_string(param, "Command") else { return };
        api.process_chat_message(&command, self.chat_type);
    }

    pub fn handle_chat_message(&mut self, api: &mut dyn UiApi, param: &str) {
        let chat_type = parse_int(param, "Type").unwrap_or(0);
        let text = parse_string(param, "Msg").unwrap_or_default();
        let r = parse_int(param, "ColorR").unwrap_or(0);
        let g = parse_int(param, "ColorG").unwrap_or(0);
        let b = parse_int(param, "ColorB").unwrap_or(0);
        let color = rgb(r, g, b);
        let system_type = if chat_type == CHAT_SYSTEM { parse_int(param, "SysType").unwrap_or(0) } else { SYSTEM_NONE };
        for (window, target) in [
            (CHAT_WINDOW_NORMAL, NORMAL_CHAT),
            (CHAT_WINDOW_PARTY, PARTY_CHAT),
            (CHAT_WINDOW_CLAN, CLAN_CHAT),
            (CHAT_WINDOW_TRADE, TRADE_CHAT),
            (CHAT_WINDOW_ALLY, ALLY_CHAT),
            (CHAT_WINDOW_SYSTEM, SYSTEM_MSG_LIST),
        ] {
            if self.check_filter(api, chat_type, window, system_type) {
                api.chat_add_string(target, &text, color);
            }
        }
        if chat_type == CHAT_COMMANDER_CHAT && self.no_union_commander_message == 0 {
            self.show_union_commander_message(api, &text);
        }
    }

    pub fn show_union_commander_message(&mut self, api: &mut dyn UiApi, message: &str) {
        let mut msg = message.to_string();
        let max_length = msg.chars().count();
        if max_length > CHAT_UNION_MAX {
            let head = left(&msg, CHAT_UNION_MAX);
            let tail = right(&msg, max_length - CHAT_UNION_MAX);
            msg = format!("{head}#{tail}");
        }
        api.debug(&msg);
        if !msg.is_empty() {
            let mut param = String::new();
            param_add(&mut param, "MsgType", "1");
            param_add(&mut param, "WindowType", "8");
            param_add(&mut param, "FontType", "0");
            param_add(&mut param, "BackgroundType", "0");
            param_add(&mut param, "LifeTime", "5000");
            param_add(&mut param, "AnimationType", "1");
            param_add(&mut param, "Msg", &msg);
            param_add(&mut param, "MsgColorR", "255");
            param_add(&mut param, "MsgColorG", "150");
            param_add(&mut param, "MsgColorB", "149");
            api.execute_event(EVENT_ON_SCREEN_MESSAGE, &param);
        }
    }

    pub fn handle_ime_status_change(&mut self, api: &mut dyn UiApi) {
        let texture = match api.get_current_ime_lang() {
            1 => "L2UI.ChatWnd.IME_kr",
            2 => "L2UI.ChatWnd.IME_en",
            3 | 4 => "L2UI.ChatWnd.IME_jp",
            5 => "L2UI.ChatWnd.IME_tw2",
            6 => "L2UI.ChatWnd.IME_tw3",
            7 => "L2UI.ChatWnd.IME_tw1",
            9 => "L2UI.ChatWnd.IME_cn1",
            10 => "L2UI.ChatWnd.IME_cn2",
            11 => "L2UI.ChatWnd.IME_cn3",
            12 | 13 => "L2UI.ChatWnd.IME_cn4",
            14 => "L2UI.ChatWnd.IME_th",
            _ => "L2UI.ChatWnd.IME_en",
        };
        api.set_texture(LANGUAGE_TEXTURE, texture);
    }

    pub fn check_filter(&self, api: &mut dyn UiApi, chat_type: i32, window_type: i32, system_type: i32) -> bool {
        if !(0..CHAT_WINDOW_COUNT).contains(&window_type) && window_type != CHAT_WINDOW_SYSTEM {
            api.debug(&format!("ChatWnd: Error invalid windowType {window_type}"));
            return false;
        }
        let filter = self.filter_info.get(window_type as usize).copied().unwrap_or_default();
        if chat_type == CHAT_MARKET && filter.trade != 0 {
            return true;
        }
        if chat_type == CHAT_NORMAL && filter.normal != 0 {
            return true;
        }
        if chat_type == CHAT_CLAN && filter.clan != 0 {
            return true;
        }
        if chat_type == CHAT_PARTY && filter.party != 0 {
            return true;
        }
        if chat_type == CHAT_SHOUT && filter.shout != 0 {
            return true;
        }
        if chat_type == CHAT_TELL && filter.whisper != 0 {
            return true;
        }
        if chat_type == CHAT_ALLIANCE && filter.ally != 0 {
            return true;
        }
        if chat_type == CHAT_HERO && filter.hero != 0 {
            return true;
        }
        if matches!(chat_type, CHAT_ANNOUNCE | CHAT_CRITICAL_ANNOUNCE | CHAT_USER_PET | CHAT_GM_PET) {
            return true;
        }
        if (chat_type == CHAT_INTER_PARTYMASTER_CHAT || chat_type == CHAT_COMMANDER_CHAT) && filter.union != 0 {
            return true;
        }
        if chat_type != CHAT_SYSTEM {
            return false;
        }
        if system_type == SYSTEM_SERVER || system_type == SYSTEM_PETITION {
            return true;
        }
        if window_type == CHAT_WINDOW_SYSTEM {
            if system_type == SYSTEM_DAMAGE {
                return api.get_option_bool("Game", "SystemMsgWndDamage");
            }
            if system_type == SYSTEM_USEITEMS {
                return api.get_option_bool("Game", "SystemMsgWndExpendableItem");
            }
            return system_type == SYSTEM_BATTLE || system_type == SYSTEM_NONE;
        }
        if filter.system != 0 {
            if system_type == SYSTEM_DAMAGE {
                return filter.damage != 0;
            }
            if system_type == SYSTEM_USEITEMS {
                return filter.useitem != 0;
            }
            return true;
        }
        false
    }

    pub fn init_filter_info(&mut self, api: &mut dyn UiApi) {
        self.set_default_filter_value();
        let mut i = 0usize;
        while i < CHAT_WINDOW_COUNT as usize {
            let section = self.section_name.get(i).cloned().unwrap_or_default();
            let read = |key: &str| api.get_ini_bool(&section, key, CHAT_FILTER_FILE);
            let values = [
                read("system"),
                read("chat"),
                read("normal"),
                read("shout"),
                read("pledge"),
                read("party"),
                read("market"),
                read("tell"),
                read("damage"),
                read("ally"),
                read("useitems"),
                read("hero"),
                read("union"),
            ];
            let info = &mut self.filter_info[i];
            let slots: [&mut i32; 13] = [
                &mut info.system,
                &mut info.chat,
                &mut info.normal,
                &mut info.shout,
                &mut info.clan,
                &mut info.party,
                &mut info.trade,
                &mut info.whisper,
                &mut info.damage,
                &mut info.ally,
                &mut info.useitem,
                &mut info.hero,
                &mut info.union,
            ];
            for (slot, value) in slots.into_iter().zip(values) {
                if let Some(value) = value {
                    *slot = value;
                }
            }
            i += 1;
        }
        self.set_default_filter_on();
        if let Some(value) = api.get_ini_bool("global", "command", CHAT_FILTER_FILE) {
            self.no_union_commander_message = value;
        }
    }

    pub fn set_default_filter_on(&mut self) {
        self.filter_info[1].trade = 1;
        self.filter_info[2].party = 1;
        self.filter_info[3].clan = 1;
        self.filter_info[4].ally = 1;
    }

    pub fn set_default_filter_value(&mut self) {
        for (slot, value) in self.filter_info.iter_mut().zip(DEFAULT_FILTERS) {
            *slot = value;
        }
        self.no_union_commander_message = 0;
    }

    pub fn set_chat_filter_button(&mut self, api: &mut dyn UiApi) {
        let system_msg_wnd = api.get_option_bool("Game", "SystemMsgWnd");
        api.checkbox_set_check("ChatFilterWnd.SystemMsgBox", system_msg_wnd);
        let option = api.get_option_bool("Game", "SystemMsgWndDamage");
        api.checkbox_set_check("ChatFilterWnd.DamageBox", option);
        let option = api.get_option_bool("Game", "SystemMsgWndExpendableItem");
        api.checkbox_set_check("ChatFilterWnd.ItemBox", option);
        if !(0..CHAT_WINDOW_COUNT).contains(&self.chat_type) {
            return;
        }
        if let Some(id) = TAB_SYSSTRINGS.get(self.chat_type as usize) {
            let title = api.make_full_system_msg(&api.get_system_message(FILTER_TITLE_SYSMSG), &api.get_system_string(*id), "");
            api.set_text("ChatFilterWnd.CurrentText", &title);
        }
        let filter = self.filter_info[self.chat_type as usize];
        api.checkbox_set_check("ChatFilterWnd.CheckBoxSystem", filter.system != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxNormal", filter.normal != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxShout", filter.shout != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxPledge", filter.clan != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxParty", filter.party != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxTrade", filter.trade != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxWhisper", filter.whisper != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxDamage", filter.damage != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxAlly", filter.ally != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxItem", filter.useitem != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxHero", filter.hero != 0);
        api.checkbox_set_check("ChatFilterWnd.CheckBoxUnion", filter.union != 0);
        let system_checked = api.checkbox_is_checked("ChatFilterWnd.CheckBoxSystem");
        api.checkbox_set_disable("ChatFilterWnd.CheckBoxDamage", !system_checked);
        api.checkbox_set_disable("ChatFilterWnd.CheckBoxItem", !system_checked);
        for name in [
            "ChatFilterWnd.CheckBoxNormal",
            "ChatFilterWnd.CheckBoxShout",
            "ChatFilterWnd.CheckBoxPledge",
            "ChatFilterWnd.CheckBoxParty",
            "ChatFilterWnd.CheckBoxTrade",
            "ChatFilterWnd.CheckBoxWhisper",
            "ChatFilterWnd.CheckBoxAlly",
            "ChatFilterWnd.CheckBoxHero",
            "ChatFilterWnd.CheckBoxUnion",
        ] {
            api.checkbox_set_disable(name, false);
        }
        match self.chat_type {
            1 => api.checkbox_set_disable("ChatFilterWnd.CheckBoxTrade", true),
            2 => api.checkbox_set_disable("ChatFilterWnd.CheckBoxParty", true),
            3 => api.checkbox_set_disable("ChatFilterWnd.CheckBoxPledge", true),
            4 => api.checkbox_set_disable("ChatFilterWnd.CheckBoxAlly", true),
            _ => {}
        }
    }

    pub fn handle_chat_wnd_status_change(&mut self, api: &mut dyn UiApi) {
        let info = api.get_player_info().unwrap_or_default();
        api.tab_set_disable(CHAT_TAB_CTRL, 3, info.clan_id <= 0);
        api.tab_set_disable(CHAT_TAB_CTRL, 4, info.alliance_id <= 0);
    }

    pub fn handle_set_string(&mut self, api: &mut dyn UiApi, param: &str) {
        if let Some(text) = parse_string(param, "String") {
            api.edit_set_string(CHAT_EDIT_BOX, &text);
        }
    }

    pub fn handle_set_focus(&mut self, api: &mut dyn UiApi) {
        if !api.is_focused(CHAT_EDIT_BOX) {
            api.set_focus(CHAT_EDIT_BOX);
        }
    }

    pub fn print(&self, api: &mut dyn UiApi, index: usize) {
        let f = self.filter_info.get(index).copied().unwrap_or_default();
        api.debug(&format!(
            "Print type({index}), system :{}, chat:{},Normal:{}, shout:{},pledge:{}, party:{}, trade:{}, whisper:{}, damage:{}, ally:{},useitem:{}, hero:{}",
            f.system, f.chat, f.normal, f.shout, f.clan, f.party, f.trade, f.whisper, f.damage, f.ally, f.useitem, f.hero
        ));
    }

    pub fn handle_msn_status(&mut self, api: &mut dyn UiApi, param: &str) {
        let status = parse_string(param, "status").unwrap_or_default();
        let textures = match status.as_str() {
            "online" => Some(("L2UI_CH3.Msn.chatting_msn1", "L2UI_CH3.Msn.chatting_msn1_down")),
            "berightback" | "idle" | "away" | "lunch" => Some(("L2UI_CH3.Msn.chatting_msn2", "L2UI_CH3.Msn.chatting_msn2_down")),
            "busy" | "onthephone" => Some(("L2UI_CH3.Msn.chatting_msn3", "L2UI_CH3.Msn.chatting_msn3_down")),
            "offline" | "invisible" => Some(("L2UI_CH3.Msn.chatting_msn4", "L2UI_CH3.Msn.chatting_msn4_down")),
            "none" => Some(("L2UI_CH3.Msn.chatting_msn5", "L2UI_CH3.Msn.chatting_msn5_down")),
            _ => None,
        };
        if let Some((fore, back)) = textures {
            api.button_set_texture(MESSENGER_BTN, fore, back, "");
        }
    }

    pub fn get_chat_type_by_tab_index(&self, _index: i32) -> i32 {
        match self.chat_type {
            0 => CHAT_NORMAL,
            1 => CHAT_MARKET,
            2 => CHAT_PARTY,
            3 => CHAT_CLAN,
            4 => CHAT_ALLIANCE,
            _ => CHAT_NORMAL,
        }
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        self.filter_info = vec![ChatFilterInfo::default(); CHAT_WINDOW_COUNT as usize + 1];
        for event in REGISTERED_EVENTS {
            api.register_event(event);
        }
        self.section_name = SECTION_NAMES.iter().map(|name| name.to_string()).collect();
        api.register_state(CHAT_WND, "OlympiadObserverState");
        self.init_handle();
        self.init_filter_info(api);
        self.init_global_setting(api);
        self.init_scroll_bar_position(api);
    }

    fn on_default_position(&mut self, api: &mut dyn UiApi) {
        for index in 1..=4 {
            api.tab_merge(CHAT_TAB_CTRL, index);
        }
        api.tab_set_top_order(CHAT_TAB_CTRL, 0, true);
        api.set_anchor(CHAT_WND, "", "BottomLeft", "BottomLeft", DEFAULT_ANCHOR_OFFSET.0, DEFAULT_ANCHOR_OFFSET.1);
        self.handle_tab_click(api, TAB_BUTTONS[0]);
    }

    fn on_complete_edit_box(&mut self, api: &mut dyn UiApi, id: &str) {
        if id != "ChatEditBox" {
            return;
        }
        let input = api.edit_get_string(CHAT_EDIT_BOX);
        if input.chars().count() < 1 {
            return;
        }
        api.process_chat_message(&input, self.chat_type);
        api.edit_set_string(CHAT_EDIT_BOX, "");
        if api.get_option_bool("Game", "OldChatting") {
            let chat_type = self.get_chat_type_by_tab_index(self.chat_type);
            if self.chat_type != 0 {
                let prefix = api.get_chat_prefix(chat_type);
                api.edit_add_string(CHAT_EDIT_BOX, &prefix);
            }
        }
        if api.get_option_bool("Game", "EnterChatting") {
            api.edit_release_focus(CHAT_EDIT_BOX);
        }
    }

    fn on_show(&mut self, api: &mut dyn UiApi) {
        if api.get_option_bool("Game", "SystemMsgWnd") {
            api.show_window(SYSTEM_MSG_WND);
        } else {
            api.hide_window(SYSTEM_MSG_WND);
        }
        self.handle_ime_status_change(api);
        self.init_all_block_tooltip(api);
        api.set_timer(CHAT_WND, INTRO_TIMER, 1000);
    }

    fn on_timer(&mut self, api: &mut dyn UiApi, timer_id: i32) {
        if timer_id == 1 {
            api.kill_timer(CHAT_WND, 1);
            api.add_system_message(INTRO_MESSAGE, msn_color("System"));
        }
        if timer_id == 2 {
            api.kill_timer(CHAT_WND, 2);
            api.add_system_message(INTRO_MESSAGE_2, msn_color("System"));
        }
    }

    fn on_click_button(&mut self, api: &mut dyn UiApi, id: &str) {
        match id {
            "ChatTabCtrl0" | "ChatTabCtrl1" | "ChatTabCtrl2" | "ChatTabCtrl3" | "ChatTabCtrl4" => self.handle_tab_click(api, id),
            "ChatFilterBtn" => {
                if api.is_show_window(CHAT_FILTER_WND) {
                    api.hide_window(CHAT_FILTER_WND);
                } else {
                    self.set_chat_filter_button(api);
                    api.show_window(CHAT_FILTER_WND);
                }
            }
            "MessengerBtn" => api.toggle_msn_window(),
            "PartyMatchingBtn" => {
                if api.is_show_window(PARTY_MATCH_WND) {
                    api.hide_window(PARTY_MATCH_WND);
                    api.script_on_send_packet_when_hiding(PARTY_MATCH_WND);
                } else {
                    api.request_open_party_match();
                }
            }
            "BtnAllBlock" => self.get_all_block(api),
            _ => {}
        }
    }

    fn on_tab_split(&mut self, api: &mut dyn UiApi, name: &str) {
        let Some(window) = chat_window_of_tab(name) else { return };
        self.handle_tab_click(api, name);
        api.set_window_size_rel(window, -1.0, -1.0, 0, 0);
        api.set_settled_wnd(window, true);
        api.chat_enable_texture(window, true);
    }

    fn on_tab_merge(&mut self, api: &mut dyn UiApi, name: &str) {
        let Some(window) = chat_window_of_tab(name) else { return };
        let rect = api.get_rect(NORMAL_CHAT);
        let (width, height) = api.get_window_size(NORMAL_CHAT);
        api.set_settled_wnd(window, false);
        api.move_to(window, rect.x, rect.y);
        api.set_window_size(window, width, height - MERGED_HEIGHT_INSET);
        api.set_window_size_rel(window, 1.0, 1.0, 0, -MERGED_HEIGHT_INSET);
        api.chat_enable_texture(window, false);
    }

    fn on_enter_state(&mut self, api: &mut dyn UiApi, previous: &str) {
        if previous != "LoadingState" {
            self.clear(api);
        }
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        match event_id {
            EVENT_CHAT_MESSAGE => {
                self.handle_chat_message(api, param);
                self.handle_ime_status_change(api);
            }
            EVENT_IME_STATUS_CHANGE => self.handle_ime_status_change(api),
            EVENT_CHAT_WND_STATUS_CHANGE => self.handle_chat_wnd_status_change(api),
            EVENT_SET_FOCUS => self.handle_set_focus(api),
            EVENT_SET_STRING => self.handle_set_string(api, param),
            EVENT_MSN_STATUS => self.handle_msn_status(api, param),
            EVENT_MACRO_COMMAND => self.handle_chat_wnd_macro_command(api, param),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::RecordingApi;
    use super::super::{Rect, UserInfo};
    use super::*;

    fn loaded() -> (Script, RecordingApi) {
        let mut script = Script::default();
        let mut api = RecordingApi::default();
        script.on_load(&mut api);
        (script, api)
    }

    #[test]
    fn on_load_registers_the_seven_events_and_places_the_scroll_bars_at_5_10_minus_25() {
        let (script, api) = loaded();
        for event in REGISTERED_EVENTS {
            assert!(api.calls.contains(&format!("RegisterEvent({event})")), "{event}");
        }
        assert!(api.calls.contains(&"RegisterState(ChatWnd,OlympiadObserverState)".to_string()));
        for window in [NORMAL_CHAT, TRADE_CHAT, PARTY_CHAT, CLAN_CHAT, ALLY_CHAT] {
            assert!(api.calls.contains(&format!("SetScrollBarPosition({window},5,10,-25)")));
        }
        assert_eq!(script.section_name, SECTION_NAMES);
        assert_eq!(script.filter_info.len(), 6);
        assert_eq!(script.filter_info[0], DEFAULT_FILTERS[0]);
        assert_eq!(script.filter_info[1].trade, 1);
        assert_eq!(script.filter_info[5], ChatFilterInfo::default());
        assert!(api.calls.contains(&"SetCheck(ChatFilterWnd.CheckBoxCommand,false)".to_string()));
    }

    #[test]
    fn on_show_follows_the_system_msg_option_sets_the_ime_texture_and_arms_the_intro_timer() {
        let (mut script, mut api) = loaded();
        api.options_bool.push((("Game".into(), "SystemMsgWnd".into()), true));
        script.on_show(&mut api);
        assert!(api.calls.contains(&"ShowWindow(SystemMsgWnd)".to_string()));
        assert!(api.calls.contains(&"SetTexture(ChatWnd.LanguageTexture,L2UI.ChatWnd.IME_en)".to_string()));
        assert!(api.calls.contains(&"SetTooltipCustomType(ChatWnd.BtnAllBlock,Stop Blocking)".to_string()));
        assert!(api.calls.contains(&"SetTimer(ChatWnd,1,1000)".to_string()));
        script.on_timer(&mut api, 1);
        assert!(api.calls.contains(&"KillTimer(ChatWnd,1)".to_string()));
        assert!(api.calls.contains(&format!("AddSystemMessage({INTRO_MESSAGE},FFB09B79)")));
        let mut hidden = RecordingApi::default();
        script.on_show(&mut hidden);
        assert!(hidden.calls.contains(&"HideWindow(SystemMsgWnd)".to_string()));
    }

    #[test]
    fn tab_click_reads_the_top_index_and_prefixes_the_input_in_old_chatting_mode() {
        let (mut script, mut api) = loaded();
        api.options_bool.push((("Game".into(), "OldChatting".into()), true));
        api.tab_set_top_order(CHAT_TAB_CTRL, 2, true);
        api.edit_set_string(CHAT_EDIT_BOX, "+hello");
        script.on_click_button(&mut api, "ChatTabCtrl2");
        assert_eq!(script.chat_type, 2);
        assert_eq!(api.get_text(CHAT_EDIT_BOX), "#hello");
        assert!(api.calls.contains(&"CheckSetDisable(ChatFilterWnd.CheckBoxParty,true)".to_string()));
        assert!(api.calls.contains(&"SetText(ChatFilterWnd.CurrentText,sysmsg1995||)".to_string()));
    }

    #[test]
    fn tab_split_and_merge_resize_the_chat_window_like_the_script() {
        let (mut script, mut api) = loaded();
        api.rects.push((NORMAL_CHAT.into(), Rect { x: 3, y: 7, width: 343, height: 141 }));
        script.on_tab_split(&mut api, "ChatTabCtrl1");
        assert!(api.calls.contains(&"SetWindowSizeRel(ChatWnd.TradeChat,-1,-1,0,0)".to_string()));
        assert!(api.calls.contains(&"SetSettledWnd(ChatWnd.TradeChat,true)".to_string()));
        assert!(api.calls.contains(&"EnableTexture(ChatWnd.TradeChat,true)".to_string()));
        script.on_tab_merge(&mut api, "ChatTabCtrl1");
        assert!(api.calls.contains(&"MoveTo(ChatWnd.TradeChat,3,7)".to_string()));
        assert!(api.calls.contains(&"SetWindowSize(ChatWnd.TradeChat,343,95)".to_string()));
        assert!(api.calls.contains(&"SetWindowSizeRel(ChatWnd.TradeChat,1,1,0,-46)".to_string()));
        assert!(api.calls.contains(&"EnableTexture(ChatWnd.TradeChat,false)".to_string()));
        let before = api.calls.len();
        script.on_tab_split(&mut api, "Other");
        assert_eq!(api.calls.len(), before);
    }

    #[test]
    fn chat_message_540_goes_to_the_windows_whose_filter_accepts_it() {
        let (mut script, mut api) = loaded();
        script.on_event(&mut api, 540, "Type=0 Msg=\u{7f}oi gente\u{7f} ColorR=255 ColorG=10 ColorB=1");
        assert!(api.calls.contains(&"AddString(ChatWnd.NormalChat,oi gente,FFFF0A01)".to_string()));
        assert!(!api.calls.iter().any(|call| call.starts_with("AddString(ChatWnd.TradeChat")));
        assert!(api.calls.contains(&"SetTexture(ChatWnd.LanguageTexture,L2UI.ChatWnd.IME_en)".to_string()));
        api.calls.clear();
        script.on_event(&mut api, 540, "Type=8 Msg=\u{7f}selling\u{7f} ColorR=1 ColorG=2 ColorB=3");
        assert!(api.calls.contains(&"AddString(ChatWnd.TradeChat,selling,FF010203)".to_string()));
        assert!(!api.calls.iter().any(|call| call.starts_with("AddString(ChatWnd.NormalChat")));
        api.calls.clear();
        api.options_bool.push((("Game".into(), "SystemMsgWndDamage".into()), true));
        script.on_event(&mut api, 540, "Type=5 SysType=3 Msg=\u{7f}hit\u{7f} ColorR=0 ColorG=0 ColorB=0");
        assert!(api.calls.contains(&"AddString(SystemMsgWnd.SystemMsgList,hit,FF000000)".to_string()));
        assert!(api.calls.contains(&"AddString(ChatWnd.NormalChat,hit,FF000000)".to_string()));
        api.calls.clear();
        script.on_event(&mut api, 540, "Type=5 SysType=7 Msg=\u{7f}used\u{7f} ColorR=0 ColorG=0 ColorB=0");
        assert!(!api.calls.iter().any(|call| call.starts_with("AddString(")));
    }

    #[test]
    fn commander_chat_also_fires_the_on_screen_message_event_with_a_hash_after_35_chars() {
        let (mut script, mut api) = loaded();
        let long: String = "a".repeat(40);
        script.on_event(&mut api, 540, &format!("Type=15 Msg=\u{7f}{long}\u{7f} ColorR=0 ColorG=0 ColorB=0"));
        let expected = format!("{}#{}", "a".repeat(35), "a".repeat(5));
        assert!(api.calls.iter().any(|call| call.starts_with("ExecuteEvent(140,") && call.contains(&format!("Msg=\u{7f}{expected}\u{7f}"))));
        assert!(api.calls.contains(&format!("AddString(ChatWnd.NormalChat,{long},FF000000)")));
    }

    #[test]
    fn the_other_registered_events_drive_focus_string_tabs_msn_and_macro() {
        let (mut script, mut api) = loaded();
        script.on_event(&mut api, 570, "");
        assert!(api.calls.contains(&"SetFocus(ChatWnd.ChatEditBox)".to_string()));
        script.on_event(&mut api, 560, "String=\u{7f}hello there\u{7f}");
        assert_eq!(api.get_text(CHAT_EDIT_BOX), "hello there");
        api.player = Some(UserInfo { clan_id: 5, alliance_id: 0, ..Default::default() });
        script.on_event(&mut api, 550, "");
        assert!(api.calls.contains(&"TabSetDisable(ChatWnd.ChatTabCtrl,3,false)".to_string()));
        assert!(api.calls.contains(&"TabSetDisable(ChatWnd.ChatTabCtrl,4,true)".to_string()));
        script.on_event(&mut api, 571, "status=busy");
        assert!(api.calls.contains(&"ButtonSetTexture(Chatwnd.MessengerBtn,L2UI_CH3.Msn.chatting_msn3,L2UI_CH3.Msn.chatting_msn3_down,)".to_string()));
        script.on_event(&mut api, 572, "Command=\u{7f}/sit\u{7f}");
        assert!(api.calls.contains(&"ProcessChatMessage(/sit,0)".to_string()));
        script.on_event(&mut api, 1500, "");
        assert_eq!(api.calls.iter().filter(|call| call.starts_with("SetTexture(ChatWnd.LanguageTexture")).count(), 1);
    }

    #[test]
    fn buttons_toggle_the_filter_window_messenger_party_match_and_all_block() {
        let (mut script, mut api) = loaded();
        script.on_click_button(&mut api, "ChatFilterBtn");
        assert!(api.calls.contains(&"ShowWindow(ChatFilterWnd)".to_string()));
        script.on_click_button(&mut api, "ChatFilterBtn");
        assert!(api.calls.contains(&"HideWindow(ChatFilterWnd)".to_string()));
        script.on_click_button(&mut api, "MessengerBtn");
        assert!(api.calls.contains(&"ToggleMsnWindow()".to_string()));
        script.on_click_button(&mut api, "PartyMatchingBtn");
        assert!(api.calls.contains(&"RequestOpenPartyMatch()".to_string()));
        api.show_window(PARTY_MATCH_WND);
        script.on_click_button(&mut api, "PartyMatchingBtn");
        assert!(api.calls.contains(&"OnSendPacketWhenHiding(PartyMatchWnd)".to_string()));
        script.on_click_button(&mut api, "BtnAllBlock");
        assert!(api.calls.contains(&"ExecuteCommand(/allblock)".to_string()));
        assert!(api.get_option_bool("Neophron", "AllBlock"));
        script.on_click_button(&mut api, "BtnAllBlock");
        assert!(api.calls.contains(&"ExecuteCommand(/allunblock)".to_string()));
    }

    #[test]
    fn completing_the_edit_box_sends_the_message_and_keeps_the_prefix_in_old_chatting_mode() {
        let (mut script, mut api) = loaded();
        api.options_bool.push((("Game".into(), "OldChatting".into()), true));
        api.options_bool.push((("Game".into(), "EnterChatting".into()), true));
        script.chat_type = 3;
        api.edit_set_string(CHAT_EDIT_BOX, "@hi");
        script.on_complete_edit_box(&mut api, "ChatEditBox");
        assert!(api.calls.contains(&"ProcessChatMessage(@hi,3)".to_string()));
        assert_eq!(api.get_text(CHAT_EDIT_BOX), "@");
        assert!(api.calls.contains(&"ReleaseFocus(ChatWnd.ChatEditBox)".to_string()));
        api.edit_set_string(CHAT_EDIT_BOX, "");
        let before = api.calls.len();
        script.on_complete_edit_box(&mut api, "ChatEditBox");
        assert_eq!(api.calls.len(), before);
    }

    #[test]
    fn entering_a_state_other_than_loading_clears_every_list_and_the_default_position_merges_the_tabs() {
        let (mut script, mut api) = loaded();
        script.on_enter_state(&mut api, "GameState");
        assert!(api.calls.contains(&"Clear(SystemMsgWnd.SystemMsgList)".to_string()));
        let before = api.calls.len();
        script.on_enter_state(&mut api, "LoadingState");
        assert_eq!(api.calls.len(), before);
        script.on_default_position(&mut api);
        for index in 1..=4 {
            assert!(api.calls.contains(&format!("MergeTab(ChatWnd.ChatTabCtrl,{index})")));
        }
        assert!(api.calls.contains(&"SetAnchor(ChatWnd,,BottomLeft,BottomLeft,0,-17)".to_string()));
    }
}
