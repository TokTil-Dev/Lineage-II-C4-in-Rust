use super::{HennaInfo, UiApi, UserInfo, WindowScript, parse_int, rgb, uc_clamp, uc_left, uc_string_float};

pub const WINDOW: &str = "DetailStatusWnd";
pub const EVENTS: [i32; 8] = [180, 260, 190, 200, 210, 220, 230, 240];
pub const STATUS_BAR_HEIGHT: i32 = 12;
pub const STATUS_SMALL_BAR_SIZE: i32 = 85;
pub const PLEDGE_SYSSTRING: i32 = 431;
pub const HERO_TEXTURE: &str = "L2UI_CH3.PlayerStatusWnd.myinfo_heroicon";
pub const NOBLE_TEXTURE: &str = "L2UI_CH3.PlayerStatusWnd.myinfo_nobleicon";
pub const WEIGHT_BARS: [&str; 4] = [
    "L2UI_CH3.PlayerStatusWnd.ps_weightbar1",
    "L2UI_CH3.PlayerStatusWnd.ps_weightbar2",
    "L2UI_CH3.PlayerStatusWnd.ps_weightbar3",
    "L2UI_CH3.PlayerStatusWnd.ps_weightbar4",
];
pub const WEIGHT_STEP_2: f32 = 66.66366;
pub const FOCUS_ORDER: [&str; 66] = [
    "txtName1",
    "txtName2",
    "txtHeadPledge",
    "texPledgeCrest",
    "txtPledge",
    "texHero",
    "txtLvHead",
    "txtLvName",
    "txtHeadRank",
    "txtRank",
    "texHP",
    "texMP",
    "texExp",
    "texCP",
    "texWeight",
    "txtHP",
    "txtMP",
    "txtExp",
    "txtCP",
    "txtWeight",
    "txtSP",
    "txtHeadFight",
    "txtHeadPhysicalAttack",
    "txtHeadPhysicalDefense",
    "txtHeadHitRate",
    "txtHeadCriticalRate",
    "txtHeadPhysicalAttackSpeed",
    "txtHeadMagicalAttack",
    "txtHeadMagicDefense",
    "txtHeadPhysicalAvoid",
    "txtHeadMovingSpeed",
    "txtHeadMagicCastingSpeed",
    "txtHeadBasic",
    "txtHeadSTR",
    "txtHeadDEX",
    "txtHeadCON",
    "txtHeadINT",
    "txtHeadWIT",
    "txtHeadMEN",
    "txtHeadSocial",
    "txtHeadCriminalRate",
    "txtHeadPVP",
    "txtHeadSociality",
    "txtHeadRemainSulffrage",
    "txtPhysicalAttack",
    "txtPhysicalDefense",
    "txtHitRate",
    "txtCriticalRate",
    "txtPhysicalAttackSpeed",
    "txtMagicalAttack",
    "txtMagicDefense",
    "txtPhysicalAvoid",
    "txtGmMoving",
    "txtMovingSpeed",
    "txtMagicCastingSpeed",
    "txtSTR",
    "txtDEX",
    "txtCON",
    "txtINT",
    "txtWIT",
    "txtMEN",
    "txtCriminalRate",
    "txtPVP",
    "txtSociality",
    "txtRemainSulffrage",
    "txtClass",
];

#[derive(Debug)]
pub struct Script {
    pub window_name: String,
    pub user_id: i32,
    pub received_user_info: bool,
    pub show: bool,
    pub henna_info: HennaInfo,
}

impl Default for Script {
    fn default() -> Self {
        Script { window_name: WINDOW.to_string(), user_id: 0, received_user_info: false, show: false, henna_info: HennaInfo::default() }
    }
}

fn stat_text(value: i32, change: i32) -> String {
    if change > 0 {
        format!("{value}(+{change})")
    } else if change < 0 {
        format!("{value}({change})")
    } else {
        value.to_string()
    }
}

fn bar_size(value: i32, max_value: i32) -> i32 {
    let mut size = 0;
    if max_value > 0 {
        size = STATUS_SMALL_BAR_SIZE;
        if value < max_value {
            size = STATUS_SMALL_BAR_SIZE * value / max_value;
        }
    }
    size
}

fn uc_div(a: f32, b: f32) -> f32 {
    if b == 0.0 { 0.0 } else { a / b }
}

impl Script {
    fn ctrl(&self, suffix: &str) -> String {
        format!("{}{suffix}", self.window_name)
    }

    pub fn handle_update_status_packet(&mut self, api: &mut dyn UiApi, param: &str) {
        let server_id = parse_int(param, "ServerID").unwrap_or(0);
        if self.user_id == server_id || !self.received_user_info {
            self.received_user_info = true;
            self.handle_update_user_info(api);
        }
    }

    pub fn handle_update_henna_info(&mut self, param: &str) {
        let fields: [(&str, &mut i32); 17] = [
            ("HennaID", &mut self.henna_info.henna_id),
            ("ClassID", &mut self.henna_info.class_id),
            ("Num", &mut self.henna_info.num),
            ("Fee", &mut self.henna_info.fee),
            ("CanUse", &mut self.henna_info.can_use),
            ("INTnow", &mut self.henna_info.int_now),
            ("INTchange", &mut self.henna_info.int_change),
            ("STRnow", &mut self.henna_info.str_now),
            ("STRchange", &mut self.henna_info.str_change),
            ("CONnow", &mut self.henna_info.con_now),
            ("CONchange", &mut self.henna_info.con_change),
            ("MENnow", &mut self.henna_info.men_now),
            ("MENchange", &mut self.henna_info.men_change),
            ("DEXnow", &mut self.henna_info.dex_now),
            ("DEXchange", &mut self.henna_info.dex_change),
            ("WITnow", &mut self.henna_info.wit_now),
            ("WITchange", &mut self.henna_info.wit_change),
        ];
        for (name, slot) in fields {
            if let Some(value) = parse_int(param, name) {
                *slot = value;
            }
        }
    }

    pub fn get_my_user_info(&self, api: &dyn UiApi) -> Option<UserInfo> {
        api.get_player_info()
    }

    pub fn get_moving_speed(&self, api: &dyn UiApi, info: &UserInfo) -> String {
        let move_type = api.player_move_type();
        let env_type = api.player_environment();
        let mut moving_speed = 0;
        let scaled = |speed: i32| (speed as f32 * info.non_attack_speed_modifier) as i32;
        if move_type == 2 {
            moving_speed = scaled(info.ground_max_speed);
            match env_type {
                2 => moving_speed = scaled(info.water_max_speed),
                3 => moving_speed = scaled(info.air_max_speed),
                _ => {}
            }
        } else if move_type == 1 {
            moving_speed = scaled(info.ground_min_speed);
            match env_type {
                2 => moving_speed = scaled(info.water_min_speed),
                3 => moving_speed = scaled(info.air_min_speed),
                _ => {}
            }
        }
        moving_speed.to_string()
    }

    pub fn get_my_exp_rate(&self, api: &dyn UiApi) -> f32 {
        api.player_exp_rate() * 100.0
    }

    pub fn handle_update_user_info(&mut self, api: &mut dyn UiApi) {
        api.set_texture(&self.ctrl(".texPledgeCrest"), "");
        let rect = api.get_rect(&self.window_name);
        let mut name = String::new();
        let mut nick_name = String::new();
        let mut class_name = String::new();
        let mut user_rank = String::new();
        let (mut hp, mut max_hp, mut mp, mut max_mp, mut cp, mut max_cp) = (0, 0, 0, 0, 0, 0);
        let (mut carry_weight, mut carring_weight, mut sp, mut level) = (0, 0, 0, 0);
        let mut exp_rate = 0.0f32;
        let mut pledge_id = 0;
        let (mut hero, mut nobless) = (false, false);
        let (mut n_str, mut n_dex, mut n_con, mut n_int, mut n_wit, mut n_men) = (0, 0, 0, 0, 0, 0);
        let (mut physical_attack, mut physical_defense, mut hit_rate, mut critical_rate) = (0, 0, 0, 0);
        let (mut physical_attack_speed, mut magical_attack, mut magic_defense, mut physical_avoid) = (0, 0, 0, 0);
        let mut moving_speed = String::new();
        let mut magic_casting_speed = 0;
        let mut criminal_rate = 0;
        let mut str_criminal_rate = String::new();
        let (mut dual_count, mut pk_count, mut sociality, mut remain_sulffrage) = (0, 0, 0, 0);
        if let Some(info) = self.get_my_user_info(api) {
            self.user_id = info.id;
            name = info.name.clone();
            nick_name = info.nick_name.clone();
            class_name = api.get_class_type(info.sub_class);
            sp = info.sp;
            level = info.level;
            user_rank = api.get_user_rank_string(info.user_rank);
            hp = info.cur_hp;
            max_hp = info.max_hp;
            mp = info.cur_mp;
            max_mp = info.max_mp;
            carry_weight = info.carry_weight;
            carring_weight = info.carring_weight;
            cp = info.cur_cp;
            max_cp = info.max_cp;
            exp_rate = self.get_my_exp_rate(api);
            pledge_id = info.clan_id;
            hero = info.hero;
            nobless = info.nobless;
            n_str = info.str;
            n_dex = info.dex;
            n_con = info.con;
            n_int = info.int;
            n_wit = info.wit;
            n_men = info.men;
            physical_attack = info.physical_attack;
            physical_defense = info.physical_defense;
            hit_rate = info.hit_rate;
            critical_rate = info.critical_rate;
            physical_attack_speed = info.physical_attack_speed;
            magical_attack = info.magical_attack;
            magic_defense = info.magic_defense;
            physical_avoid = info.physical_avoid;
            magic_casting_speed = info.magic_casting_speed;
            moving_speed = self.get_moving_speed(api, &info);
            criminal_rate = info.criminal_rate;
            dual_count = info.dual_count;
            pk_count = info.pk_count;
            sociality = info.sociality;
            remain_sulffrage = info.remain_sulffrage;
            str_criminal_rate = if criminal_rate >= 999999 { format!("{criminal_rate}+") } else { criminal_rate.to_string() };
        }
        let mut crim_rate = criminal_rate;
        if crim_rate > 255 {
            crim_rate = 255;
        }
        if crim_rate > 0 {
            crim_rate = uc_clamp(crim_rate, 100 + crim_rate / 16, 255);
        }
        let name_color = rgb(255, 255 - crim_rate, 255 - crim_rate);
        let nick_name_color = rgb(162, 249, 236);
        if !nick_name.is_empty() {
            let (mut width1, _) = api.get_text_size(&name);
            let (mut width2, _) = api.get_text_size(&nick_name);
            if width1 + width2 > 220 {
                if width1 > 109 {
                    name = uc_left(&name, 8);
                    width1 = api.get_text_size(&name).0;
                }
                if width2 > 109 {
                    nick_name = uc_left(&nick_name, 8);
                    width2 = api.get_text_size(&nick_name).0;
                }
            }
            let _ = width1;
            api.set_text(&self.ctrl(".txtName1"), &nick_name);
            api.set_text_color(&self.ctrl(".txtName1"), nick_name_color);
            api.set_text(&self.ctrl(".txtName2"), &name);
            api.set_text_color(&self.ctrl(".txtName2"), name_color);
            api.move_to(&self.ctrl(".txtName2"), rect.x + 15 + width2 + 6, rect.y + 9);
        } else {
            api.set_text(&self.ctrl(".txtName1"), &name);
            api.set_text_color(&self.ctrl(".txtName1"), name_color);
            api.set_text(&self.ctrl(".txtName2"), "");
        }
        api.set_text(&self.ctrl(".txtClass"), &class_name);
        api.set_text(&self.ctrl(".txtLvl"), &level.to_string());
        api.set_text(&self.ctrl(".txtRank"), &user_rank);
        api.set_text(&self.ctrl(".txtHP"), &format!("{hp}/{max_hp}"));
        api.set_text(&self.ctrl(".txtMP"), &format!("{mp}/{max_mp}"));
        api.set_text(&self.ctrl(".txtExp"), &format!("{}%", uc_string_float(exp_rate)));
        api.set_text(&self.ctrl(".txtCP"), &format!("{cp}/{max_cp}"));
        api.set_text(&self.ctrl(".txtSP"), &sp.to_string());
        let weight_percent = uc_div(100.0 * carring_weight as f32, carry_weight as f32);
        api.set_text(&self.ctrl(".txtWeight"), &format!("{}%", uc_string_float(weight_percent)));
        let pledge_name;
        let pledge_name_color;
        let mut pledge_crest_texture = None;
        if pledge_id > 0 {
            pledge_crest_texture = api.clan_crest_texture(pledge_id);
            pledge_name = api.clan_name(pledge_id);
            pledge_name_color = rgb(176, 155, 121);
        } else {
            pledge_name = api.get_system_string(PLEDGE_SYSSTRING);
            pledge_name_color = rgb(255, 255, 255);
        }
        api.set_text(&self.ctrl(".txtPledge"), &pledge_name);
        api.set_text_color(&self.ctrl(".txtPledge"), pledge_name_color);
        match pledge_crest_texture {
            Some(texture) => {
                api.set_texture_with_object(&self.ctrl(".texPledgeCrest"), &texture);
                api.move_to(&self.ctrl(".txtPledge"), rect.x + 88, rect.y + 25);
            }
            None => api.move_to(&self.ctrl(".txtPledge"), rect.x + 68, rect.y + 25),
        }
        let hero_texture = if hero {
            HERO_TEXTURE
        } else if nobless {
            NOBLE_TEXTURE
        } else {
            ""
        };
        api.set_texture(&self.ctrl(".texHero"), hero_texture);
        api.set_text(&self.ctrl(".txtSTR"), &stat_text(n_str, self.henna_info.str_change));
        api.set_text(&self.ctrl(".txtDEX"), &stat_text(n_dex, self.henna_info.dex_change));
        api.set_text(&self.ctrl(".txtCON"), &stat_text(n_con, self.henna_info.con_change));
        api.set_text(&self.ctrl(".txtINT"), &stat_text(n_int, self.henna_info.int_change));
        api.set_text(&self.ctrl(".txtWIT"), &stat_text(n_wit, self.henna_info.wit_change));
        api.set_text(&self.ctrl(".txtMEN"), &stat_text(n_men, self.henna_info.men_change));
        api.set_text(&self.ctrl(".txtPhysicalAttack"), &physical_attack.to_string());
        api.set_text(&self.ctrl(".txtPhysicalDefense"), &physical_defense.to_string());
        api.set_text(&self.ctrl(".txtHitRate"), &hit_rate.to_string());
        api.set_text(&self.ctrl(".txtCriticalRate"), &critical_rate.to_string());
        api.set_text(&self.ctrl(".txtPhysicalAttackSpeed"), &physical_attack_speed.to_string());
        api.set_text(&self.ctrl(".txtMagicalAttack"), &magical_attack.to_string());
        api.set_text(&self.ctrl(".txtMagicDefense"), &magic_defense.to_string());
        api.set_text(&self.ctrl(".txtPhysicalAvoid"), &physical_avoid.to_string());
        api.set_text(&self.ctrl(".txtMovingSpeed"), &moving_speed);
        api.set_text(&self.ctrl(".txtMagicCastingSpeed"), &magic_casting_speed.to_string());
        api.set_text(&self.ctrl(".txtCriminalRate"), &str_criminal_rate);
        api.set_text(&self.ctrl(".txtPVP"), &format!("{dual_count} / {pk_count}"));
        api.set_text(&self.ctrl(".txtSociality"), &sociality.to_string());
        api.set_text(&self.ctrl(".txtRemainSulffrage"), &remain_sulffrage.to_string());
        self.update_hp_bar(api, hp, max_hp);
        self.update_mp_bar(api, mp, max_mp);
        self.update_cp_bar(api, cp, max_cp);
        self.update_exp_bar(api, exp_rate as i32, 100);
        self.update_weight_bar(api, carring_weight, carry_weight);
    }

    pub fn update_hp_bar(&self, api: &mut dyn UiApi, value: i32, max_value: i32) {
        api.set_window_size(&self.ctrl(".texHP"), bar_size(value, max_value), STATUS_BAR_HEIGHT);
    }

    pub fn update_mp_bar(&self, api: &mut dyn UiApi, value: i32, max_value: i32) {
        api.set_window_size(&self.ctrl(".texMP"), bar_size(value, max_value), STATUS_BAR_HEIGHT);
    }

    pub fn update_exp_bar(&self, api: &mut dyn UiApi, value: i32, max_value: i32) {
        api.set_window_size(&self.ctrl(".texEXP"), bar_size(value, max_value), STATUS_BAR_HEIGHT);
    }

    pub fn update_cp_bar(&self, api: &mut dyn UiApi, value: i32, max_value: i32) {
        api.set_window_size(&self.ctrl(".texCP"), bar_size(value, max_value), STATUS_BAR_HEIGHT);
    }

    pub fn update_weight_bar(&self, api: &mut dyn UiApi, value: i32, max_value: i32) {
        let mut size = 0;
        let mut texture = "";
        if max_value > 0 {
            let ratio = uc_div(100.0 * value as f32, max_value as f32);
            if ratio <= 50.0 {
                texture = WEIGHT_BARS[0];
            } else if ratio > 50.0 && ratio <= WEIGHT_STEP_2 {
                texture = WEIGHT_BARS[1];
            } else if ratio > WEIGHT_STEP_2 && ratio <= 80.0 {
                texture = WEIGHT_BARS[2];
            } else if ratio > 80.0 {
                texture = WEIGHT_BARS[3];
            }
            size = bar_size(value, max_value);
        }
        api.set_texture(&self.ctrl(".texWeight"), texture);
        api.set_window_size(&self.ctrl(".texWeight"), size, STATUS_BAR_HEIGHT);
    }

    pub fn set_focus(&self, api: &mut dyn UiApi) {
        for ctrl in FOCUS_ORDER {
            api.set_focus(&format!("DetailStatusWnd.{ctrl}"));
        }
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        for event in EVENTS {
            api.register_event(event);
        }
        self.show = false;
    }

    fn on_enter_state(&mut self, api: &mut dyn UiApi, _previous: &str) {
        self.received_user_info = false;
        self.handle_update_user_info(api);
    }

    fn on_show(&mut self, api: &mut dyn UiApi) {
        self.handle_update_user_info(api);
        self.show = true;
        self.set_focus(api);
    }

    fn on_hide(&mut self, _api: &mut dyn UiApi) {
        self.show = false;
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        match event_id {
            180 => self.handle_update_user_info(api),
            260 => self.handle_update_henna_info(param),
            200 | 210 | 220 | 230 | 240 => self.handle_update_status_packet(api, param),
            _ => {}
        }
    }

    fn on_click_button(&mut self, api: &mut dyn UiApi, id: &str) {
        if id == "BtnClose" {
            api.hide_window(WINDOW);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::RecordingApi;
    use super::super::Rect;
    use super::*;

    fn player() -> UserInfo {
        UserInfo {
            id: 3,
            name: "Killer".into(),
            level: 52,
            sub_class: 88,
            sp: 5000,
            cur_hp: 400,
            max_hp: 900,
            cur_mp: 200,
            max_mp: 300,
            cur_cp: 100,
            max_cp: 100,
            carry_weight: 1000,
            carring_weight: 250,
            clan_id: 0,
            criminal_rate: 0,
            str: 40,
            dex: 30,
            dual_count: 2,
            pk_count: 1,
            ground_max_speed: 120,
            non_attack_speed_modifier: 1.5,
            ..Default::default()
        }
    }

    fn api() -> RecordingApi {
        RecordingApi {
            player: Some(player()),
            rects: vec![("DetailStatusWnd".into(), Rect { x: 10, y: 20, width: 256, height: 335 })],
            sysstrings: vec![(431, "Clan".into())],
            exp_rate: 0.9834,
            move_type: 2,
            ..Default::default()
        }
    }

    fn call(api: &RecordingApi, prefix: &str) -> Vec<String> {
        api.calls.iter().filter(|call| call.starts_with(prefix)).cloned().collect()
    }

    #[test]
    fn on_load_registers_events_and_on_show_fills_the_texts_bars_and_focus_order() {
        let mut api = api();
        let mut script = Script::default();
        script.on_load(&mut api);
        assert_eq!(api.calls.len(), 8);
        api.calls.clear();
        script.on_show(&mut api);
        assert!(script.show);
        assert_eq!(script.user_id, 3);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtName1"), ["SetText(DetailStatusWnd.txtName1,Killer)"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtClass"), ["SetText(DetailStatusWnd.txtClass,type88)"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtHP"), ["SetText(DetailStatusWnd.txtHP,400/900)"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtExp"), ["SetText(DetailStatusWnd.txtExp,98.34%)"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtWeight"), ["SetText(DetailStatusWnd.txtWeight,25.00%)"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtPledge,"), ["SetText(DetailStatusWnd.txtPledge,Clan)"]);
        assert_eq!(call(&api, "MoveTo(DetailStatusWnd.txtPledge"), ["MoveTo(DetailStatusWnd.txtPledge,78,45)"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtMovingSpeed"), ["SetText(DetailStatusWnd.txtMovingSpeed,180)"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtPVP"), ["SetText(DetailStatusWnd.txtPVP,2 / 1)"]);
        assert_eq!(call(&api, "SetWindowSize(DetailStatusWnd.texHP"), ["SetWindowSize(DetailStatusWnd.texHP,37,12)"]);
        assert_eq!(call(&api, "SetWindowSize(DetailStatusWnd.texEXP"), ["SetWindowSize(DetailStatusWnd.texEXP,83,12)"]);
        assert_eq!(call(&api, "SetTexture(DetailStatusWnd.texWeight"), ["SetTexture(DetailStatusWnd.texWeight,L2UI_CH3.PlayerStatusWnd.ps_weightbar1)"]);
        assert_eq!(call(&api, "SetWindowSize(DetailStatusWnd.texWeight"), ["SetWindowSize(DetailStatusWnd.texWeight,21,12)"]);
        let focus = call(&api, "SetFocus(");
        assert_eq!(focus.len(), 66);
        assert_eq!(focus[0], "SetFocus(DetailStatusWnd.txtName1)");
        assert_eq!(focus[65], "SetFocus(DetailStatusWnd.txtClass)");
    }

    #[test]
    fn henna_changes_decorate_the_stats_and_criminal_rate_tints_the_name() {
        let mut api = api();
        let mut script = Script::default();
        script.on_event(&mut api, 260, "HennaID=1 ClassID=88 Num=1 Fee=10 CanUse=1 STRnow=41 STRchange=1 DEXchange=-2 ");
        assert_eq!((script.henna_info.str_change, script.henna_info.dex_change, script.henna_info.con_change), (1, -2, 0));
        api.player.as_mut().unwrap().criminal_rate = 1000000;
        script.on_event(&mut api, 180, "");
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtSTR"), ["SetText(DetailStatusWnd.txtSTR,40(+1))"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtDEX"), ["SetText(DetailStatusWnd.txtDEX,30(-2))"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtCON"), ["SetText(DetailStatusWnd.txtCON,0)"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtCriminalRate"), ["SetText(DetailStatusWnd.txtCriminalRate,1000000+)"]);
        assert_eq!(call(&api, "SetTextColor(DetailStatusWnd.txtName1"), ["SetTextColor(DetailStatusWnd.txtName1,FFFF0000)"]);
    }

    #[test]
    fn status_packets_refresh_only_for_the_own_id_and_the_nickname_splits_the_name_line() {
        let mut api = api();
        api.player.as_mut().unwrap().nick_name = "Nick".into();
        let mut script = Script::default();
        script.on_event(&mut api, 200, "ServerID=9 ");
        assert!(script.received_user_info);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtName1"), ["SetText(DetailStatusWnd.txtName1,Nick)"]);
        assert_eq!(call(&api, "SetText(DetailStatusWnd.txtName2"), ["SetText(DetailStatusWnd.txtName2,Killer)"]);
        assert_eq!(call(&api, "MoveTo(DetailStatusWnd.txtName2"), ["MoveTo(DetailStatusWnd.txtName2,55,29)"]);
        api.calls.clear();
        script.on_event(&mut api, 210, "ServerID=9 ");
        assert!(api.calls.is_empty());
        script.on_click_button(&mut api, "BtnClose");
        assert_eq!(api.calls, ["HideWindow(DetailStatusWnd)"]);
    }
}
