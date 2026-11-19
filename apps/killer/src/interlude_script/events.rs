use killer_assets::client::DatTables;
use killer_world::{World, WorldEvent};
use l2_net::game::{self, PartyMember, PledgeMember};

use super::param_add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamKind {
    Int,
    Str,
    Int64,
    Float,
    Loop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParamSpec {
    pub name: &'static str,
    pub kind: ParamKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EventSpec {
    pub id: i32,
    pub params: &'static [ParamSpec],
    pub source: &'static str,
}

pub const EVENT_RESTART_POINT: i32 = 50;
pub const EVENT_REGEN: i32 = 70;
pub const EVENT_USER_INFO: i32 = 180;
pub const EVENT_CURRENT_HP: i32 = 190;
pub const EVENT_MAX_HP: i32 = 200;
pub const EVENT_CURRENT_MP: i32 = 210;
pub const EVENT_MAX_MP: i32 = 220;
pub const EVENT_CURRENT_CP: i32 = 230;
pub const EVENT_MAX_CP: i32 = 240;
pub const EVENT_PET_STATUS: i32 = 250;
pub const EVENT_HENNA_INFO: i32 = 260;
pub const EVENT_ATTACK: i32 = 280;
pub const EVENT_SKILL_USE: i32 = 290;
pub const EVENT_TARGET_LEVEL_DIFF: i32 = 300;
pub const EVENT_PLEDGE_INFO: i32 = 330;
pub const EVENT_PLEDGE_MEMBER_ADD_ALL: i32 = 410;
pub const EVENT_PLEDGE_MEMBER_LIST: i32 = 420;
pub const EVENT_PLEDGE_MEMBER: i32 = 440;
pub const EVENT_PLEDGE_MEMBER_DELETE: i32 = 450;
pub const EVENT_PARTY_COUNT: i32 = 520;
pub const EVENT_CHAT_MESSAGE: i32 = 540;
pub const EVENT_PARTY_COUNT_CHAT: i32 = 550;
pub const EVENT_SHORTCUT_UPDATE: i32 = 630;
pub const EVENT_SHORTCUT_LIST: i32 = 650;
pub const EVENT_QUEST_LIST: i32 = 700;
pub const EVENT_QUEST_LIST_DONE: i32 = 720;
pub const EVENT_RECIPE_SHOP_SELL_LIST: i32 = 780;
pub const EVENT_RECIPE_SHOP_ITEM_INFO: i32 = 800;
pub const EVENT_RECIPE_BOOK: i32 = 820;
pub const EVENT_RECIPE_MAKE_INFO: i32 = 840;
pub const EVENT_RECIPE_SHOP_MANAGE: i32 = 850;
pub const EVENT_OLYMPIAD_USER: i32 = 920;
pub const EVENT_BUFFS: i32 = 950;
pub const EVENT_TARGET_SELECTED: i32 = 980;
pub const EVENT_TARGET_UNSELECTED: i32 = 990;
pub const EVENT_PET_SPELLED: i32 = 1050;
pub const EVENT_SUMMON_SPELLED: i32 = 1110;
pub const EVENT_SUMMON_REMAIN_TIME: i32 = 1120;
pub const EVENT_PARTY_ADD: i32 = 1140;
pub const EVENT_PARTY_UPDATE: i32 = 1150;
pub const EVENT_PARTY_DELETE: i32 = 1160;
pub const EVENT_PARTY_DELETE_ALL: i32 = 1170;
pub const EVENT_PARTY_SPELLED: i32 = 1180;
pub const EVENT_MACRO_LIST: i32 = 1230;
pub const EVENT_SKILL_LIST: i32 = 1290;
pub const EVENT_UNION_DETAIL: i32 = 1420;
pub const EVENT_TUTORIAL_QUESTION: i32 = 1510;
pub const EVENT_QUEST_MARK: i32 = 1520;
pub const EVENT_MAIL: i32 = 1530;
pub const EVENT_PARTY_ROOM_DETAIL: i32 = 1550;
pub const EVENT_PARTY_ROOM_MEMBERS: i32 = 1580;
pub const EVENT_PARTY_ROOM_MANAGE: i32 = 1590;
pub const EVENT_PARTY_MATCH_LIST: i32 = 1610;
pub const EVENT_PARTY_MATCH_ROOM: i32 = 1620;
pub const EVENT_HENNA_EQUIP_LIST: i32 = 1640;
pub const EVENT_HENNA_ITEM_INFO: i32 = 1660;
pub const EVENT_HENNA_UNEQUIP_LIST: i32 = 1670;
pub const EVENT_HENNA_UNEQUIP_INFO: i32 = 1690;
pub const EVENT_RADAR_ADD: i32 = 1730;
pub const EVENT_RADAR_DELETE: i32 = 1740;
pub const EVENT_RADAR_DELETE_ALL: i32 = 1750;
pub const EVENT_TOWN_MAP: i32 = 1770;
pub const EVENT_MINIMAP_ADD: i32 = 1790;
pub const EVENT_MINIMAP_DELETE: i32 = 1800;
pub const EVENT_MINIMAP_DELETE_ALL: i32 = 1810;
pub const EVENT_TRADE_START: i32 = 1950;
pub const EVENT_TRADE_DONE: i32 = 1970;
pub const EVENT_TUTORIAL_HTML: i32 = 2430;
pub const EVENT_ITEM_LIST: i32 = 2570;
pub const EVENT_ITEM_LIST_SHOW: i32 = 2580;
pub const EVENT_INVENTORY_TRADE: i32 = 2590;

pub fn spec(id: i32) -> Option<&'static EventSpec> {
    SPECS.iter().find(|spec| spec.id == id)
}

pub fn param_add_int(param: &mut String, name: &str, value: &str) {
    if name.is_empty() {
        return;
    }
    if !param.is_empty() {
        param.push(' ');
    }
    param.push_str(name);
    param.push('=');
    param.push_str(value);
}

fn int(param: &mut String, name: &str, value: impl std::fmt::Display) {
    param_add_int(param, name, &value.to_string());
}

fn flag(value: bool) -> i32 {
    i32::from(value)
}

pub fn status_event(attribute: i32) -> Option<(i32, &'static str)> {
    match attribute {
        game::ATTRIBUTE_CURRENT_HP => Some((EVENT_CURRENT_HP, "CurrentHP")),
        game::ATTRIBUTE_MAX_HP => Some((EVENT_MAX_HP, "MaxHP")),
        game::ATTRIBUTE_CURRENT_MP => Some((EVENT_CURRENT_MP, "CurrentMP")),
        game::ATTRIBUTE_MAX_MP => Some((EVENT_MAX_MP, "MaxMP")),
        game::ATTRIBUTE_CURRENT_CP => Some((EVENT_CURRENT_CP, "CurrentCP")),
        game::ATTRIBUTE_MAX_CP => Some((EVENT_MAX_CP, "MaxCP")),
        _ => None,
    }
}

pub fn party_member_param(member: &PartyMember) -> String {
    let mut param = String::new();
    param_add(&mut param, "Name", &member.name);
    int(&mut param, "ID", member.object_id);
    int(&mut param, "CurCP", member.cp);
    int(&mut param, "MaxCP", member.max_cp);
    int(&mut param, "CurHP", member.hp);
    int(&mut param, "MaxHP", member.max_hp);
    int(&mut param, "CurMP", member.mp);
    int(&mut param, "MaxMP", member.max_mp);
    int(&mut param, "ClassID", member.class_id);
    int(&mut param, "Level", member.level);
    param
}

pub fn pledge_member_param(member: &PledgeMember, pledge_type: i32) -> String {
    let mut param = String::new();
    param_add(&mut param, "Name", &member.name);
    int(&mut param, "Level", member.level);
    int(&mut param, "Class", member.class_id);
    int(&mut param, "Gender", member.unknown);
    int(&mut param, "Race", member.flag);
    int(&mut param, "ID", member.online_object_id);
    int(&mut param, "PledgeType", pledge_type);
    param
}

fn spelled_param(effects: &[game::PartyBuff], object_id: i32) -> String {
    let mut param = String::new();
    int(&mut param, "Max", effects.len());
    int(&mut param, "ID", object_id);
    for (index, buff) in effects.iter().enumerate() {
        int(&mut param, &format!("SkillID_{index}"), buff.skill_id);
        int(&mut param, &format!("Level_{index}"), buff.level);
        int(&mut param, &format!("Sec_{index}"), buff.duration);
    }
    param
}

fn buffs_param(world: &World, tables: Option<&DatTables>) -> String {
    let view = crate::ui2d::abnormal_view(world, tables, killer_platform::time::Instant::now());
    let mut param = String::new();
    int(&mut param, "Max", view.buffs.len());
    for (index, buff) in view.buffs.iter().enumerate() {
        int(&mut param, &format!("SkillID_{index}"), buff.skill_id);
        int(&mut param, &format!("SkillLevel_{index}"), buff.level);
        int(&mut param, &format!("RemainTime_{index}"), buff.remaining.unwrap_or(-1));
        let (name, description) = tables
            .and_then(|tables| tables.skillname.get(&(u32::try_from(buff.skill_id).unwrap_or(0), u32::try_from(buff.level).unwrap_or(1))))
            .map(|record| (record.name.clone(), record.description.clone()))
            .unwrap_or_default();
        param_add(&mut param, &format!("Name_{index}"), &name);
        param_add(&mut param, &format!("IconName_{index}"), buff.icon.as_deref().unwrap_or_default());
        param_add(&mut param, &format!("Description_{index}"), &description);
    }
    param
}

fn henna_item_param(info: &game::HennaItemInfo, tables: Option<&DatTables>) -> String {
    let mut param = String::new();
    int(&mut param, "Adena", info.adena);
    let dye = tables.and_then(|tables| tables.itemname.get(&u32::try_from(info.dye_item_id).unwrap_or(0)));
    param_add(&mut param, "DyeIconName", "");
    param_add(&mut param, "DyeName", dye.map(|item| item.name.as_str()).unwrap_or_default());
    int(&mut param, "HennaID", info.symbol_id);
    int(&mut param, "ClassID", info.dye_item_id);
    int(&mut param, "NumOfItem", info.dye_amount);
    int(&mut param, "Fee", info.price);
    param_add(&mut param, "TattooIconName", "");
    param_add(&mut param, "TattooName", "");
    param_add(&mut param, "TattooAddName", "");
    for (label, (now, change)) in [("INT", info.int_), ("STR", info.str_), ("CON", info.con), ("MEN", info.men), ("DEX", info.dex), ("WIT", info.wit)] {
        int(&mut param, &format!("{label}now"), now);
        int(&mut param, &format!("{label}change"), change);
    }
    param
}

fn radar_events(control: &game::RadarControl) -> Vec<(i32, String)> {
    let mut coords = String::new();
    int(&mut coords, "X", control.x);
    int(&mut coords, "Y", control.y);
    int(&mut coords, "Z", control.z);
    let mut out = Vec::new();
    let ids = |base: i32| match control.show {
        0 => Some(base),
        1 => Some(base + 10),
        2 => Some(base + 20),
        _ => None,
    };
    if matches!(control.kind, 0 | 2)
        && let Some(id) = ids(EVENT_RADAR_ADD)
    {
        out.push((id, coords.clone()));
    }
    if matches!(control.kind, 1 | 2)
        && let Some(id) = ids(EVENT_MINIMAP_ADD)
    {
        out.push((id, coords.clone()));
    }
    out
}

pub fn bridge(event: &WorldEvent, world: &World, tables: Option<&DatTables>) -> Vec<(i32, String)> {
    let me = world.me.as_ref().map(|me| me.object_id);
    let mut out = Vec::new();
    match event {
        WorldEvent::StatusUpdate(status) => {
            for (attribute, value) in &status.attributes {
                let Some((id, name)) = status_event(*attribute) else { continue };
                let mut param = String::new();
                int(&mut param, "ServerID", status.object_id);
                int(&mut param, name, value);
                out.push((id, param));
            }
        }
        WorldEvent::Die(die) if me == Some(die.object_id) && (die.to_village || die.to_hideout || die.to_castle || die.to_siege_hq || die.to_fixed_spot) => {
            let mut param = String::new();
            int(&mut param, "Village", flag(die.to_village));
            int(&mut param, "Agit", flag(die.to_hideout));
            int(&mut param, "Castle", flag(die.to_castle));
            int(&mut param, "BattleCamp", flag(die.to_siege_hq));
            int(&mut param, "Original", flag(die.to_fixed_spot));
            out.push((EVENT_RESTART_POINT, param));
        }
        WorldEvent::RegenMax(regen) => {
            let mut param = String::new();
            int(&mut param, "Type", regen.values[0]);
            int(&mut param, "Duration", regen.values[1]);
            int(&mut param, "Ticks", regen.values[2]);
            param_add_int(&mut param, "Amount", &l2_uscript::vm::format_float(regen.amount as f32));
            out.push((EVENT_REGEN, param));
        }
        WorldEvent::Attack(attack) => {
            for hit in &attack.hits {
                let mut param = String::new();
                int(&mut param, "AttackerID", attack.attacker_id);
                param_add(&mut param, "AttackerName", world.find(attack.attacker_id).map(|entity| entity.name.as_str()).unwrap_or_default());
                int(&mut param, "DefenderID", hit.target_id);
                param_add(&mut param, "DefenderName", world.find(hit.target_id).map(|entity| entity.name.as_str()).unwrap_or_default());
                int(&mut param, "Miss", flag(hit.flags & game::HIT_MISS != 0));
                int(&mut param, "ShieldDefense", flag(hit.flags & game::HIT_SHIELD != 0));
                int(&mut param, "Critical", flag(hit.flags & game::HIT_CRITICAL != 0));
                out.push((EVENT_ATTACK, param));
            }
        }
        WorldEvent::MagicSkillUse(cast) => {
            let mut param = String::new();
            int(&mut param, "AttackerID", cast.caster_id);
            int(&mut param, "DefenderID", cast.target_id);
            int(&mut param, "SkillID", cast.skill_id);
            out.push((EVENT_SKILL_USE, param));
        }
        WorldEvent::MyTargetSelected(target) => {
            out.push((EVENT_TARGET_SELECTED, String::new()));
            let mut param = String::new();
            int(&mut param, "LevelDiff", target.color);
            out.push((EVENT_TARGET_LEVEL_DIFF, param));
        }
        WorldEvent::TargetUnselected(target) if me == Some(target.object_id) => out.push((EVENT_TARGET_UNSELECTED, String::new())),
        WorldEvent::MagicEffectIcons(_) => out.push((EVENT_BUFFS, buffs_param(world, tables))),
        WorldEvent::PartySpelled(spelled) => {
            let id = match spelled.summon_type {
                1 => EVENT_PET_SPELLED,
                2 => EVENT_SUMMON_SPELLED,
                _ => EVENT_PARTY_SPELLED,
            };
            out.push((id, spelled_param(&spelled.effects, spelled.object_id)));
        }
        WorldEvent::PartySmallWindowAll(party) => {
            let mut count = String::new();
            int(&mut count, "PartyMemberCount", 0);
            out.push((EVENT_PARTY_DELETE_ALL, String::new()));
            out.push((EVENT_PARTY_COUNT, count.clone()));
            out.push((EVENT_PARTY_COUNT_CHAT, count));
            for member in &party.members {
                out.push((EVENT_PARTY_ADD, party_member_param(member)));
            }
        }
        WorldEvent::PartySmallWindowAdd(party) => out.push((EVENT_PARTY_ADD, party_member_param(&party.member))),
        WorldEvent::PartySmallWindowUpdate(member) => out.push((EVENT_PARTY_UPDATE, party_member_param(member))),
        WorldEvent::PartySmallWindowDelete(member) => {
            let mut param = String::new();
            int(&mut param, "ID", member.object_id);
            out.push((EVENT_PARTY_DELETE, param));
            let mut count = String::new();
            int(&mut count, "ID", member.object_id);
            int(&mut count, "PartyMemberCount", world.party.members.len());
            out.push((EVENT_PARTY_COUNT, count));
        }
        WorldEvent::PartySmallWindowDeleteAll(_) => {
            let mut count = String::new();
            int(&mut count, "PartyMemberCount", 0);
            out.push((EVENT_PARTY_DELETE_ALL, String::new()));
            out.push((EVENT_PARTY_COUNT, count.clone()));
            out.push((EVENT_PARTY_COUNT_CHAT, count));
        }
        WorldEvent::ShortCutInit(init) => {
            out.push((EVENT_SHORTCUT_LIST, String::new()));
            for shortcut in &init.shortcuts {
                let mut param = String::new();
                int(&mut param, "ShortcutID", shortcut.slot);
                out.push((EVENT_SHORTCUT_UPDATE, param));
            }
        }
        WorldEvent::ShortCutRegister(shortcut) => {
            let mut param = String::new();
            int(&mut param, "ShortcutID", shortcut.slot);
            out.push((EVENT_SHORTCUT_UPDATE, param));
        }
        WorldEvent::ShortCutDelete(delete) => {
            let mut param = String::new();
            int(&mut param, "ShortcutID", delete.slot);
            out.push((EVENT_SHORTCUT_UPDATE, param));
        }
        WorldEvent::Say2(message) => {
            let Some(tables) = tables else { return out };
            let Some(line) = crate::ui2d::say_line(tables, message) else { return out };
            let color = killer_ui::chat::ChatEntry::new(message.kind, "", "").color();
            let mut param = String::new();
            int(&mut param, "Type", message.kind);
            param_add(&mut param, "Msg", &line.text);
            int(&mut param, "ColorR", (color.0 >> 16) & 0xff);
            int(&mut param, "ColorG", (color.0 >> 8) & 0xff);
            int(&mut param, "ColorB", color.0 & 0xff);
            out.push((EVENT_CHAT_MESSAGE, param));
        }
        WorldEvent::PetStatusUpdate(_) | WorldEvent::PetInfo(_) | WorldEvent::PetDelete(_) => out.push((EVENT_PET_STATUS, String::new())),
        WorldEvent::SetSummonRemainTime(time) => {
            let mut param = String::new();
            int(&mut param, "RemainTime", time.remaining_time);
            int(&mut param, "MaxTime", time.max_time);
            out.push((EVENT_SUMMON_REMAIN_TIME, param));
        }
        WorldEvent::HennaInfo(_) => out.push((EVENT_HENNA_INFO, String::new())),
        WorldEvent::HennaEquipList(list) => {
            let mut param = String::new();
            int(&mut param, "Adena", list.adena);
            out.push((EVENT_HENNA_EQUIP_LIST, param));
        }
        WorldEvent::HennaUnequipList(list) => {
            let mut param = String::new();
            int(&mut param, "Adena", list.adena);
            out.push((EVENT_HENNA_UNEQUIP_LIST, param));
        }
        WorldEvent::HennaItemInfo(info) => out.push((EVENT_HENNA_ITEM_INFO, henna_item_param(info, tables))),
        WorldEvent::HennaUnequipInfo(info) => out.push((EVENT_HENNA_UNEQUIP_INFO, henna_item_param(info, tables))),
        WorldEvent::PledgeShowInfoUpdate(info) => {
            let mut param = String::new();
            int(&mut param, "ClanID", info.pledge_id);
            int(&mut param, "CrestID", info.crest_id);
            int(&mut param, "SkillLevel", info.level);
            int(&mut param, "CastleID", info.castle_id);
            int(&mut param, "AgitID", info.hideout_id);
            int(&mut param, "ClanNameValue", info.leader_level);
            int(&mut param, "Status", info.dissolving);
            int(&mut param, "AllianceID", info.ally_id);
            param_add(&mut param, "AllianceName", &info.ally_name);
            int(&mut param, "AllianceCrestID", info.ally_crest_id);
            int(&mut param, "InWar", flag(info.at_war));
            out.push((EVENT_PLEDGE_INFO, param));
        }
        WorldEvent::PledgeShowMemberListAll(list) => {
            out.push((EVENT_PLEDGE_MEMBER_LIST, String::new()));
            for member in &list.members {
                let mut param = String::new();
                int(&mut param, "ClanType", 0);
                param.push_str(&pledge_member_param(member, 0));
                out.push((EVENT_PLEDGE_MEMBER_ADD_ALL, param));
            }
        }
        WorldEvent::PledgeShowMemberListAdd(member) | WorldEvent::PledgeShowMemberListUpdate(member) => {
            out.push((EVENT_PLEDGE_MEMBER, pledge_member_param(member, 0)));
        }
        WorldEvent::PledgeShowMemberListDelete(delete) => {
            let mut param = String::new();
            param_add(&mut param, "Name", &delete.name);
            out.push((EVENT_PLEDGE_MEMBER_DELETE, param));
        }
        WorldEvent::QuestList(_) => {
            out.push((EVENT_QUEST_LIST, String::new()));
            out.push((EVENT_QUEST_LIST_DONE, String::new()));
        }
        WorldEvent::ShowQuestMark(mark) => {
            let mut param = String::new();
            int(&mut param, "QuestID", mark.value);
            out.push((EVENT_QUEST_MARK, param));
        }
        WorldEvent::TutorialShowQuestionMark(mark) => {
            let mut param = String::new();
            int(&mut param, "QuestionID", mark.mark_id);
            out.push((EVENT_TUTORIAL_QUESTION, param));
        }
        WorldEvent::TutorialShowHtml(html) => {
            let mut param = String::new();
            param_add(&mut param, "HtmlString", &html.html);
            out.push((EVENT_TUTORIAL_HTML, param));
        }
        WorldEvent::MailArrived => {
            let mut param = String::new();
            int(&mut param, "IdxMail", 0);
            out.push((EVENT_MAIL, param));
        }
        WorldEvent::SendMacroList(_) => out.push((EVENT_MACRO_LIST, String::new())),
        WorldEvent::SkillList(_) => out.push((EVENT_SKILL_LIST, String::new())),
        WorldEvent::RadarControl(control) => out.extend(radar_events(control)),
        WorldEvent::ShowTownMap(map) => {
            let mut param = String::new();
            param_add(&mut param, "TownMapName", &map.texture);
            int(&mut param, "UserPosX", map.x);
            int(&mut param, "UserPosY", map.y);
            out.push((EVENT_TOWN_MAP, param));
        }
        WorldEvent::TradeStart(trade) => {
            let mut param = String::new();
            int(&mut param, "targetID", trade.partner_id);
            out.push((EVENT_INVENTORY_TRADE, param.clone()));
            out.push((EVENT_TRADE_START, param));
        }
        WorldEvent::TradeDone(_) => out.push((EVENT_TRADE_DONE, String::new())),
        WorldEvent::ItemList(list) => {
            out.push((EVENT_ITEM_LIST, String::new()));
            if list.show_window {
                out.push((EVENT_ITEM_LIST_SHOW, String::new()));
            }
        }
        WorldEvent::InventoryUpdate(_) => out.push((EVENT_ITEM_LIST, String::new())),
        WorldEvent::RecipeBookItemList(book) => {
            let mut param = String::new();
            int(&mut param, "Type", flag(!book.common));
            int(&mut param, "CurrentMP", world.me.as_ref().and_then(|me| me.mp).map_or(0, |mp| mp.0));
            out.push((EVENT_RECIPE_BOOK, param));
        }
        WorldEvent::RecipeItemMakeInfo(info) => {
            let mut param = String::new();
            int(&mut param, "RecipeID", info.recipe_id);
            int(&mut param, "CurrentMP", info.mp);
            int(&mut param, "MaxMP", info.max_mp);
            int(&mut param, "MakingResult", info.status);
            int(&mut param, "Type", flag(!info.common));
            out.push((EVENT_RECIPE_MAKE_INFO, param));
        }
        WorldEvent::RecipeShopManageList(list) => {
            let mut param = String::new();
            int(&mut param, "Type", flag(!list.common));
            out.push((EVENT_RECIPE_SHOP_MANAGE, param));
        }
        WorldEvent::RecipeShopSellList(list) => {
            let mut param = String::new();
            int(&mut param, "ServerID", list.manufacturer_id);
            int(&mut param, "CurrentMP", list.mp);
            int(&mut param, "MaxMP", list.max_mp);
            int(&mut param, "Adena", list.adena);
            out.push((EVENT_RECIPE_SHOP_SELL_LIST, param));
        }
        WorldEvent::RecipeShopItemInfo(info) => {
            let mut param = String::new();
            int(&mut param, "MerchantID", info.manufacturer_id);
            int(&mut param, "RecipeID", info.recipe_id);
            int(&mut param, "CurrentMP", info.mp);
            int(&mut param, "MaxMP", info.max_mp);
            int(&mut param, "MakingResult", info.status);
            int(&mut param, "Adena", info.cost);
            out.push((EVENT_RECIPE_SHOP_ITEM_INFO, param));
        }
        WorldEvent::OlympiadUserInfo(info) => {
            let mut param = String::new();
            int(&mut param, "PlayerNum", info.flag);
            int(&mut param, "ID", info.value);
            param_add(&mut param, "Name", &info.name);
            for (name, value) in ["ClassID", "CurHP", "MaxHP", "CurCP", "MaxCP"].into_iter().zip(info.values) {
                int(&mut param, name, value);
            }
            out.push((EVENT_OLYMPIAD_USER, param));
        }
        WorldEvent::MultiPartyChannelInfo(info) => {
            let mut param = String::new();
            int(&mut param, "MemberCount", info.members.len());
            for (index, member) in info.members.iter().enumerate() {
                param_add(&mut param, &format!("Name_{index}"), &member.name);
                int(&mut param, &format!("ServerID_{index}"), member.value);
            }
            out.push((EVENT_UNION_DETAIL, param));
        }
        WorldEvent::PartyRoomMember(room) => {
            let mut param = String::new();
            int(&mut param, "MyMembershipType", room.header[1]);
            int(&mut param, "MemberCount", room.entries.len());
            for (index, entry) in room.entries.iter().enumerate() {
                int(&mut param, &format!("MemberID_{index}"), entry.id);
                param_add(&mut param, &format!("MemberName_{index}"), &entry.name);
                int(&mut param, &format!("ClassID_{index}"), entry.values[0]);
                int(&mut param, &format!("Level_{index}"), entry.values[1]);
                int(&mut param, &format!("ZoneID_{index}"), entry.values[2]);
                int(&mut param, &format!("MembershipType_{index}"), entry.values[3]);
            }
            out.push((EVENT_PARTY_ROOM_MEMBERS, param));
        }
        WorldEvent::ManagePartyRoomMember(manage) => {
            let mut param = String::new();
            int(&mut param, "UpdateType", manage.header[0]);
            int(&mut param, "MemberID", manage.header[1]);
            param_add(&mut param, "MemberName", &manage.name);
            for (name, value) in ["ClassID", "Level", "ZoneID", "MembershipType"].into_iter().zip(manage.values) {
                int(&mut param, name, value);
            }
            out.push((EVENT_PARTY_ROOM_MANAGE, param));
        }
        WorldEvent::PartyMatchList(list) => {
            let mut param = String::new();
            int(&mut param, "AllCount", list.rooms.len());
            int(&mut param, "Count", list.rooms.len());
            out.push((EVENT_PARTY_MATCH_LIST, param));
            for room in &list.rooms {
                let mut param = String::new();
                param_add(&mut param, "Name", &room.owner_name);
                out.push((EVENT_PARTY_MATCH_ROOM, param));
            }
        }
        WorldEvent::PartyMatchDetail(detail) => {
            let mut param = String::new();
            int(&mut param, "RoomNum", detail.id);
            int(&mut param, "MaxMember", detail.max_members);
            int(&mut param, "MinLevel", detail.min_level);
            int(&mut param, "MaxLevel", detail.max_level);
            int(&mut param, "LootingMethod", detail.loot_type);
            int(&mut param, "Location", detail.location);
            param_add(&mut param, "RoomName", &detail.title);
            out.push((EVENT_PARTY_ROOM_DETAIL, param));
        }
        _ => {}
    }
    out
}

pub const SPECS: &[EventSpec] = &[
    EventSpec { id: 10, params: &[ParamSpec { name: "INT64", kind: ParamKind::Int64 }, ParamSpec { name: "INT64", kind: ParamKind::Int64 }], source: "0x101078d0#1" },
    EventSpec { id: 50, params: &[ParamSpec { name: "Village", kind: ParamKind::Int }, ParamSpec { name: "Agit", kind: ParamKind::Int }, ParamSpec { name: "Castle", kind: ParamKind::Int }, ParamSpec { name: "BattleCamp", kind: ParamKind::Int }, ParamSpec { name: "Original", kind: ParamKind::Int }], source: "0x1014f510#1" },
    EventSpec { id: 70, params: &[ParamSpec { name: "Type", kind: ParamKind::Int }, ParamSpec { name: "Duration", kind: ParamKind::Int }, ParamSpec { name: "Ticks", kind: ParamKind::Int }, ParamSpec { name: "Amount", kind: ParamKind::Float }], source: "0x10164d70#1" },
    EventSpec { id: 80, params: &[], source: "0x100430d0#1" },
    EventSpec { id: 90, params: &[], source: "0x101711f7#1" },
    EventSpec { id: 110, params: &[ParamSpec { name: "ZoneCode", kind: ParamKind::Int }], source: "0x10167540#1" },
    EventSpec { id: 120, params: &[ParamSpec { name: "RaidRank", kind: ParamKind::Int }, ParamSpec { name: "SeasonPoint", kind: ParamKind::Int }, ParamSpec { name: "Count", kind: ParamKind::Int }, ParamSpec { name: "MonsterID%d", kind: ParamKind::Loop }, ParamSpec { name: "CurrentPoint%d", kind: ParamKind::Loop }, ParamSpec { name: "TotalPoint%d", kind: ParamKind::Loop }], source: "0x10167630#1" },
    EventSpec { id: 130, params: &[], source: "0x10167840#1" },
    EventSpec { id: 140, params: &[ParamSpec { name: "MsgType", kind: ParamKind::Int }, ParamSpec { name: "MsgNo", kind: ParamKind::Int }, ParamSpec { name: "WindowType", kind: ParamKind::Int }, ParamSpec { name: "FontSize", kind: ParamKind::Int }, ParamSpec { name: "FontType", kind: ParamKind::Int }, ParamSpec { name: "MsgColor", kind: ParamKind::Int }, ParamSpec { name: "ShadowType", kind: ParamKind::Int }, ParamSpec { name: "BackgroundType", kind: ParamKind::Int }, ParamSpec { name: "LifeTime", kind: ParamKind::Int }, ParamSpec { name: "AnimationType", kind: ParamKind::Int }], source: "0x10167900#1" },
    EventSpec { id: 150, params: &[], source: "0x1006f070#1" },
    EventSpec { id: 160, params: &[], source: "0x1006f2c4#1" },
    EventSpec { id: 170, params: &[ParamSpec { name: "ServerAgeLimit", kind: ParamKind::Int }], source: "0x1014def0#1" },
    EventSpec { id: 180, params: &[], source: "0x10165c00#1" },
    EventSpec { id: 190, params: &[ParamSpec { name: "ServerID", kind: ParamKind::Int }, ParamSpec { name: "CurrentHP", kind: ParamKind::Int }], source: "0x101565f0#1" },
    EventSpec { id: 200, params: &[ParamSpec { name: "ServerID", kind: ParamKind::Int }, ParamSpec { name: "MaxHP", kind: ParamKind::Int }], source: "0x10156810#1" },
    EventSpec { id: 210, params: &[ParamSpec { name: "ServerID", kind: ParamKind::Int }, ParamSpec { name: "CurrentMP", kind: ParamKind::Int }], source: "0x101564e0#1" },
    EventSpec { id: 220, params: &[ParamSpec { name: "ServerID", kind: ParamKind::Int }, ParamSpec { name: "MaxMP", kind: ParamKind::Int }], source: "0x10156700#1" },
    EventSpec { id: 230, params: &[ParamSpec { name: "ServerID", kind: ParamKind::Int }, ParamSpec { name: "CurrentCP", kind: ParamKind::Int }], source: "0x10156920#1" },
    EventSpec { id: 240, params: &[ParamSpec { name: "ServerID", kind: ParamKind::Int }, ParamSpec { name: "MaxCP", kind: ParamKind::Int }], source: "0x10156a30#1" },
    EventSpec { id: 250, params: &[], source: "0x1015df70#1" },
    EventSpec { id: 260, params: &[], source: "0x101575a0#1" },
    EventSpec { id: 280, params: &[ParamSpec { name: "AttackerID", kind: ParamKind::Int }, ParamSpec { name: "AttackerName", kind: ParamKind::Str }, ParamSpec { name: "DefenderID", kind: ParamKind::Int }, ParamSpec { name: "DefenderName", kind: ParamKind::Str }, ParamSpec { name: "Miss", kind: ParamKind::Int }, ParamSpec { name: "ShieldDefense", kind: ParamKind::Int }, ParamSpec { name: "Critical", kind: ParamKind::Int }], source: "0x101659c0#1" },
    EventSpec { id: 290, params: &[ParamSpec { name: "AttackerID", kind: ParamKind::Int }, ParamSpec { name: "DefenderID", kind: ParamKind::Int }, ParamSpec { name: "SkillID", kind: ParamKind::Int }], source: "0x10165870#1" },
    EventSpec { id: 300, params: &[ParamSpec { name: "LevelDiff", kind: ParamKind::Int }], source: "0x10158b70#1" },
    EventSpec { id: 310, params: &[], source: "0x1016ca60#1" },
    EventSpec { id: 320, params: &[], source: "0x10158f37#1" },
    EventSpec { id: 330, params: &[ParamSpec { name: "ClanID", kind: ParamKind::Int }, ParamSpec { name: "CrestID", kind: ParamKind::Int }, ParamSpec { name: "SkillLevel", kind: ParamKind::Int }, ParamSpec { name: "CastleID", kind: ParamKind::Int }, ParamSpec { name: "AgitID", kind: ParamKind::Int }, ParamSpec { name: "ClanRank", kind: ParamKind::Int }, ParamSpec { name: "ClanNameValue", kind: ParamKind::Int }, ParamSpec { name: "Status", kind: ParamKind::Int }, ParamSpec { name: "Guilty", kind: ParamKind::Int }, ParamSpec { name: "AllianceID", kind: ParamKind::Int }, ParamSpec { name: "AllianceName", kind: ParamKind::Str }, ParamSpec { name: "AllianceCrestID", kind: ParamKind::Int }, ParamSpec { name: "InWar", kind: ParamKind::Int }, ParamSpec { name: "LargeCrestID", kind: ParamKind::Int }], source: "0x101590a0#1" },
    EventSpec { id: 340, params: &[ParamSpec { name: "ClanMaster", kind: ParamKind::Int }, ParamSpec { name: "Join", kind: ParamKind::Int }, ParamSpec { name: "NickName", kind: ParamKind::Int }, ParamSpec { name: "ClanCrest", kind: ParamKind::Int }, ParamSpec { name: "Grade", kind: ParamKind::Int }, ParamSpec { name: "ManageMaster", kind: ParamKind::Int }, ParamSpec { name: "OustMember", kind: ParamKind::Int }], source: "0x1015a270#1" },
    EventSpec { id: 350, params: &[ParamSpec { name: "Count", kind: ParamKind::Int }, ParamSpec { name: "GradeID%d", kind: ParamKind::Loop }, ParamSpec { name: "GradeMemberCount%d", kind: ParamKind::Loop }], source: "0x10159760#1" },
    EventSpec { id: 380, params: &[ParamSpec { name: "Grade", kind: ParamKind::Int }, ParamSpec { name: "Name", kind: ParamKind::Str }, ParamSpec { name: "PowerValue%d", kind: ParamKind::Loop }], source: "0x1015a4e0#1" },
    EventSpec { id: 390, params: &[], source: "0x101596a0#1" },
    EventSpec { id: 400, params: &[], source: "0x10159b45#1" },
    EventSpec { id: 410, params: &[ParamSpec { name: "ClanType", kind: ParamKind::Int }, ParamSpec { name: "Name", kind: ParamKind::Str }, ParamSpec { name: "Level", kind: ParamKind::Int }, ParamSpec { name: "Class", kind: ParamKind::Int }, ParamSpec { name: "Gender", kind: ParamKind::Int }, ParamSpec { name: "Race", kind: ParamKind::Int }, ParamSpec { name: "HaveMaster", kind: ParamKind::Int }], source: "0x10159bc0#1" },
    EventSpec { id: 420, params: &[], source: "0x101598d0#1" },
    EventSpec { id: 430, params: &[], source: "0x1015a82d#1" },
    EventSpec { id: 440, params: &[ParamSpec { name: "Name", kind: ParamKind::Str }, ParamSpec { name: "Level", kind: ParamKind::Int }, ParamSpec { name: "Class", kind: ParamKind::Int }, ParamSpec { name: "Gender", kind: ParamKind::Int }, ParamSpec { name: "Race", kind: ParamKind::Int }, ParamSpec { name: "PledgeType", kind: ParamKind::Int }, ParamSpec { name: "HaveMaster", kind: ParamKind::Int }], source: "0x10159e40#1" },
    EventSpec { id: 450, params: &[ParamSpec { name: "Name", kind: ParamKind::Str }], source: "0x1015a0c0#1" },
    EventSpec { id: 490, params: &[ParamSpec { name: "Count", kind: ParamKind::Int }, ParamSpec { name: "SkillID%d", kind: ParamKind::Loop }, ParamSpec { name: "SkillLevel%d", kind: ParamKind::Loop }], source: "0x10159410#1" },
    EventSpec { id: 500, params: &[ParamSpec { name: "SkillID", kind: ParamKind::Int }, ParamSpec { name: "SkillLevel", kind: ParamKind::Int }], source: "0x10159580#1" },
    EventSpec { id: 520, params: &[ParamSpec { name: "PartyMemberCount", kind: ParamKind::Int }], source: "0x1015d110#2" },
    EventSpec { id: 530, params: &[], source: "0x10155280#1" },
    EventSpec { id: 550, params: &[], source: "0x1015d270#1" },
    EventSpec { id: 560, params: &[ParamSpec { name: "String", kind: ParamKind::Str }], source: "0x10108850#1" },
    EventSpec { id: 572, params: &[ParamSpec { name: "Command", kind: ParamKind::Str }], source: "0x101b2260#1" },
    EventSpec { id: 580, params: &[ParamSpec { name: "Index", kind: ParamKind::Int }], source: "0x1013cd50#1" },
    EventSpec { id: 630, params: &[ParamSpec { name: "ShortcutID", kind: ParamKind::Int }], source: "0x10046140#1" },
    EventSpec { id: 640, params: &[ParamSpec { name: "ShortcutPage", kind: ParamKind::Int }], source: "0x10046fe0#1" },
    EventSpec { id: 650, params: &[], source: "0x101586f0#1" },
    EventSpec { id: 660, params: &[ParamSpec { name: "OnOff", kind: ParamKind::Int }], source: "0x101624a0#1" },
    EventSpec { id: 700, params: &[], source: "0x1011e5f0#1" },
    EventSpec { id: 720, params: &[], source: "0x1011e5f0#2" },
    EventSpec { id: 730, params: &[ParamSpec { name: "QuestID", kind: ParamKind::Int }], source: "0x100065b0#1" },
    EventSpec { id: 740, params: &[ParamSpec { name: "SSQStatus", kind: ParamKind::Int }, ParamSpec { name: "Period", kind: ParamKind::Int }, ParamSpec { name: "MsgNum1", kind: ParamKind::Int }, ParamSpec { name: "MsgNum2", kind: ParamKind::Int }, ParamSpec { name: "SSQTeam", kind: ParamKind::Int }, ParamSpec { name: "SelectedSeal", kind: ParamKind::Int }, ParamSpec { name: "Contribution", kind: ParamKind::Int }, ParamSpec { name: "SealStoneAdena", kind: ParamKind::Int }, ParamSpec { name: "Team1HuntingMark", kind: ParamKind::Int }, ParamSpec { name: "Team1MainEventMark", kind: ParamKind::Int }, ParamSpec { name: "Team2HuntingMark", kind: ParamKind::Int }, ParamSpec { name: "Team2MainEventMark", kind: ParamKind::Int }, ParamSpec { name: "Team1Per", kind: ParamKind::Int }, ParamSpec { name: "Team2Per", kind: ParamKind::Int }, ParamSpec { name: "Team1TotalMark", kind: ParamKind::Int }, ParamSpec { name: "Team2TotalMark", kind: ParamKind::Int }], source: "0x101642a0#1" },
    EventSpec { id: 760, params: &[ParamSpec { name: "SSQStatus", kind: ParamKind::Int }, ParamSpec { name: "NeedPoint1", kind: ParamKind::Int }, ParamSpec { name: "NeedPoint2", kind: ParamKind::Int }, ParamSpec { name: "SealNum", kind: ParamKind::Int }, ParamSpec { name: "SealID_%d", kind: ParamKind::Loop }, ParamSpec { name: "OwnerTeamID_%d", kind: ParamKind::Loop }, ParamSpec { name: "Team2Mark_%d", kind: ParamKind::Loop }, ParamSpec { name: "Team1Mark_%d", kind: ParamKind::Loop }], source: "0x10164a50#1" },
    EventSpec { id: 770, params: &[ParamSpec { name: "Winner", kind: ParamKind::Int }, ParamSpec { name: "RoomNum", kind: ParamKind::Int }, ParamSpec { name: "SealNum_%d", kind: ParamKind::Loop }, ParamSpec { name: "Winner_%d", kind: ParamKind::Loop }, ParamSpec { name: "Msg_%d", kind: ParamKind::Loop }], source: "0x101640c0#1" },
    EventSpec { id: 780, params: &[ParamSpec { name: "ServerID", kind: ParamKind::Int }, ParamSpec { name: "CurrentMP", kind: ParamKind::Int }, ParamSpec { name: "MaxMP", kind: ParamKind::Int }, ParamSpec { name: "Adena", kind: ParamKind::Int }], source: "0x10156070#1" },
    EventSpec { id: 790, params: &[ParamSpec { name: "RecipeID", kind: ParamKind::Int }, ParamSpec { name: "CanbeMade", kind: ParamKind::Int }, ParamSpec { name: "MakingFee", kind: ParamKind::Int }], source: "0x101561e0#1" },
    EventSpec { id: 800, params: &[ParamSpec { name: "MerchantID", kind: ParamKind::Int }, ParamSpec { name: "RecipeID", kind: ParamKind::Int }, ParamSpec { name: "CurrentMP", kind: ParamKind::Int }, ParamSpec { name: "MaxMP", kind: ParamKind::Int }, ParamSpec { name: "MakingResult", kind: ParamKind::Int }, ParamSpec { name: "Adena", kind: ParamKind::Int }], source: "0x10156320#1" },
    EventSpec { id: 820, params: &[ParamSpec { name: "Type", kind: ParamKind::Int }, ParamSpec { name: "CurrentMP", kind: ParamKind::Int }], source: "0x101559e0#1" },
    EventSpec { id: 830, params: &[ParamSpec { name: "RecipeID", kind: ParamKind::Int }], source: "0x10155af0#1" },
    EventSpec { id: 840, params: &[ParamSpec { name: "RecipeID", kind: ParamKind::Int }, ParamSpec { name: "CurrentMP", kind: ParamKind::Int }, ParamSpec { name: "MaxMP", kind: ParamKind::Int }, ParamSpec { name: "MakingResult", kind: ParamKind::Int }, ParamSpec { name: "Type", kind: ParamKind::Int }], source: "0x10155bd0#1" },
    EventSpec { id: 850, params: &[ParamSpec { name: "Type", kind: ParamKind::Int }], source: "0x10155d70#1" },
    EventSpec { id: 860, params: &[ParamSpec { name: "RecipeID", kind: ParamKind::Int }], source: "0x10155e50#1" },
    EventSpec { id: 870, params: &[ParamSpec { name: "RecipeID", kind: ParamKind::Int }, ParamSpec { name: "CanbeMade", kind: ParamKind::Int }, ParamSpec { name: "MakingFee", kind: ParamKind::Int }], source: "0x10155f30#1" },
    EventSpec { id: 910, params: &[], source: "0x10154b70#1" },
    EventSpec { id: 920, params: &[ParamSpec { name: "PlayerNum", kind: ParamKind::Int }, ParamSpec { name: "Name", kind: ParamKind::Str }, ParamSpec { name: "ClassID", kind: ParamKind::Int }, ParamSpec { name: "CurHP", kind: ParamKind::Int }, ParamSpec { name: "MaxHP", kind: ParamKind::Int }, ParamSpec { name: "CurCP", kind: ParamKind::Int }, ParamSpec { name: "MaxCP", kind: ParamKind::Int }], source: "0x101545a0#1" },
    EventSpec { id: 950, params: &[], source: "0x101587b0#1" },
    EventSpec { id: 950, params: &[ParamSpec { name: "Max", kind: ParamKind::Int }, ParamSpec { name: "SkillID_%d", kind: ParamKind::Loop }, ParamSpec { name: "SkillLevel_%d", kind: ParamKind::Loop }, ParamSpec { name: "RemainTime_%d", kind: ParamKind::Loop }, ParamSpec { name: "Name_%d", kind: ParamKind::Loop }, ParamSpec { name: "IconName_%d", kind: ParamKind::Loop }, ParamSpec { name: "Description_%d", kind: ParamKind::Loop }], source: "0x1016abb0#1" },
    EventSpec { id: 960, params: &[], source: "0x10158910#1" },
    EventSpec { id: 960, params: &[ParamSpec { name: "Max", kind: ParamKind::Int }, ParamSpec { name: "SkillID_%d", kind: ParamKind::Loop }, ParamSpec { name: "SkillLevel_%d", kind: ParamKind::Loop }, ParamSpec { name: "RemainTime_%d", kind: ParamKind::Loop }, ParamSpec { name: "Name_%d", kind: ParamKind::Loop }, ParamSpec { name: "IconName_%d", kind: ParamKind::Loop }, ParamSpec { name: "Description_%d", kind: ParamKind::Loop }], source: "0x1016b060#1" },
    EventSpec { id: 970, params: &[], source: "0x10158860#1" },
    EventSpec { id: 970, params: &[ParamSpec { name: "Max", kind: ParamKind::Int }, ParamSpec { name: "SkillID_%d", kind: ParamKind::Loop }, ParamSpec { name: "SkillLevel_%d", kind: ParamKind::Loop }, ParamSpec { name: "RemainTime_%d", kind: ParamKind::Loop }, ParamSpec { name: "Name_%d", kind: ParamKind::Loop }, ParamSpec { name: "IconName_%d", kind: ParamKind::Loop }, ParamSpec { name: "Description_%d", kind: ParamKind::Loop }], source: "0x1016b420#1" },
    EventSpec { id: 980, params: &[], source: "0x1014e240#1" },
    EventSpec { id: 1000, params: &[ParamSpec { name: "Show", kind: ParamKind::Str }, ParamSpec { name: "Show", kind: ParamKind::Str }], source: "0x10163f50#1" },
    EventSpec { id: 1020, params: &[ParamSpec { name: "Show", kind: ParamKind::Int }], source: "0x1015deb0#1" },
    EventSpec { id: 1060, params: &[], source: "0x1011de70#1" },
    EventSpec { id: 1070, params: &[], source: "0x1011de70#2" },
    EventSpec { id: 1080, params: &[ParamSpec { name: "WorkType", kind: ParamKind::Int }], source: "0x1015e160#1" },
    EventSpec { id: 1120, params: &[ParamSpec { name: "RemainTime", kind: ParamKind::Int }, ParamSpec { name: "MaxTime", kind: ParamKind::Int }], source: "0x1015dca0#1" },
    EventSpec { id: 1130, params: &[], source: "0x1015dd90#1" },
    EventSpec { id: 1150, params: &[ParamSpec { name: "Name", kind: ParamKind::Str }, ParamSpec { name: "CurCP", kind: ParamKind::Int }, ParamSpec { name: "MaxCP", kind: ParamKind::Int }, ParamSpec { name: "CurHP", kind: ParamKind::Int }, ParamSpec { name: "MaxHP", kind: ParamKind::Int }, ParamSpec { name: "CurMP", kind: ParamKind::Int }, ParamSpec { name: "MaxMP", kind: ParamKind::Int }, ParamSpec { name: "ClassID", kind: ParamKind::Int }, ParamSpec { name: "Level", kind: ParamKind::Int }], source: "0x1015cd40#1" },
    EventSpec { id: 1160, params: &[], source: "0x1015d110#1" },
    EventSpec { id: 1170, params: &[], source: "0x1015d270#2" },
    EventSpec { id: 1190, params: &[ParamSpec { name: "Index", kind: ParamKind::Int }], source: "0x100065b0#2" },
    EventSpec { id: 1190, params: &[ParamSpec { name: "Init", kind: ParamKind::Int }], source: "0x1013c460#1" },
    EventSpec { id: 1210, params: &[], source: "0x1017f690#1" },
    EventSpec { id: 1220, params: &[ParamSpec { name: "HtmlString", kind: ParamKind::Str }], source: "0x1008bfb0#2" },
    EventSpec { id: 1221, params: &[ParamSpec { name: "HtmlString", kind: ParamKind::Str }], source: "0x1008bfb0#1" },
    EventSpec { id: 1230, params: &[], source: "0x1013c610#1" },
    EventSpec { id: 1270, params: &[ParamSpec { name: "MacroID", kind: ParamKind::Int }], source: "0x1011c550#1" },
    EventSpec { id: 1280, params: &[], source: "0x10155960#1" },
    EventSpec { id: 1290, params: &[], source: "0x1016ab15#1" },
    EventSpec { id: 1360, params: &[], source: "0x10155330#1" },
    EventSpec { id: 1370, params: &[], source: "0x101553f0#1" },
    EventSpec { id: 1380, params: &[ParamSpec { name: "OwnerName", kind: ParamKind::Str }, ParamSpec { name: "RoutingType", kind: ParamKind::Int }, ParamSpec { name: "PartyNum", kind: ParamKind::Int }, ParamSpec { name: "PartyMemberNum", kind: ParamKind::Int }], source: "0x10154c20#1" },
    EventSpec { id: 1390, params: &[ParamSpec { name: "MasterName", kind: ParamKind::Str }, ParamSpec { name: "MasterID", kind: ParamKind::Int }, ParamSpec { name: "PartyNum", kind: ParamKind::Int }], source: "0x10154dc0#1" },
    EventSpec { id: 1395, params: &[ParamSpec { name: "MasterName", kind: ParamKind::Str }, ParamSpec { name: "MasterID", kind: ParamKind::Int }, ParamSpec { name: "MemberCount", kind: ParamKind::Int }, ParamSpec { name: "Type", kind: ParamKind::Int }], source: "0x101550f0#1" },
    EventSpec { id: 1400, params: &[ParamSpec { name: "RoutingType", kind: ParamKind::Int }], source: "0x10167c20#1" },
    EventSpec { id: 1420, params: &[ParamSpec { name: "MemberCount", kind: ParamKind::Int }, ParamSpec { name: "Name_%d", kind: ParamKind::Loop }, ParamSpec { name: "ServerID_%d", kind: ParamKind::Loop }, ParamSpec { name: "ClassID_%d", kind: ParamKind::Loop }], source: "0x10154f30#1" },
    EventSpec { id: 1430, params: &[], source: "0x1014f7d0#1" },
    EventSpec { id: 1440, params: &[], source: "0x1014f750#1" },
    EventSpec { id: 1450, params: &[ParamSpec { name: "CastleID", kind: ParamKind::Int }, ParamSpec { name: "IsOwner", kind: ParamKind::Int }, ParamSpec { name: "OwnerName", kind: ParamKind::Str }, ParamSpec { name: "ClanID", kind: ParamKind::Int }, ParamSpec { name: "ClanName", kind: ParamKind::Str }, ParamSpec { name: "AllianceID", kind: ParamKind::Int }, ParamSpec { name: "AllianceName", kind: ParamKind::Str }, ParamSpec { name: "NowTime", kind: ParamKind::Int }, ParamSpec { name: "SiegeTime", kind: ParamKind::Int }], source: "0x1015b710#1" },
    EventSpec { id: 1460, params: &[ParamSpec { name: "Type", kind: ParamKind::Int }], source: "0x1015c500#1" },
    EventSpec { id: 1470, params: &[ParamSpec { name: "Type", kind: ParamKind::Int }, ParamSpec { name: "ClanID", kind: ParamKind::Int }, ParamSpec { name: "ClanName", kind: ParamKind::Str }, ParamSpec { name: "ClanMasterName", kind: ParamKind::Str }, ParamSpec { name: "ClanCrestID", kind: ParamKind::Int }, ParamSpec { name: "AllianceID", kind: ParamKind::Int }, ParamSpec { name: "AllianceName", kind: ParamKind::Str }, ParamSpec { name: "AllianceMasterName", kind: ParamKind::Str }, ParamSpec { name: "ProposeTime", kind: ParamKind::Int }], source: "0x1015bad0#1" },
    EventSpec { id: 1470, params: &[ParamSpec { name: "Type", kind: ParamKind::Int }, ParamSpec { name: "ClanID", kind: ParamKind::Int }, ParamSpec { name: "ClanName", kind: ParamKind::Str }, ParamSpec { name: "ClanMasterName", kind: ParamKind::Str }, ParamSpec { name: "ClanCrestID", kind: ParamKind::Int }, ParamSpec { name: "AllianceID", kind: ParamKind::Int }, ParamSpec { name: "AllianceName", kind: ParamKind::Str }, ParamSpec { name: "AllianceMasterName", kind: ParamKind::Str }, ParamSpec { name: "ProposeTime", kind: ParamKind::Int }, ParamSpec { name: "Status", kind: ParamKind::Int }], source: "0x1015bec0#1" },
    EventSpec { id: 1480, params: &[ParamSpec { name: "Type", kind: ParamKind::Int }], source: "0x1015c450#1" },
    EventSpec { id: 1490, params: &[ParamSpec { name: "TimeID", kind: ParamKind::Int }], source: "0x1015c2f0#1" },
    EventSpec { id: 1500, params: &[], source: "0x1002a930#1" },
    EventSpec { id: 1510, params: &[ParamSpec { name: "QuestionID", kind: ParamKind::Int }], source: "0x1015d9e0#1" },
    EventSpec { id: 1520, params: &[ParamSpec { name: "QuestID", kind: ParamKind::Int }], source: "0x10151da0#1" },
    EventSpec { id: 1530, params: &[ParamSpec { name: "IdxMail", kind: ParamKind::Int }], source: "0x101623c0#1" },
    EventSpec { id: 1540, params: &[], source: "0x10166510#1" },
    EventSpec { id: 1550, params: &[ParamSpec { name: "RoomNum", kind: ParamKind::Int }, ParamSpec { name: "MaxMember", kind: ParamKind::Int }, ParamSpec { name: "MinLevel", kind: ParamKind::Int }, ParamSpec { name: "MaxLevel", kind: ParamKind::Int }, ParamSpec { name: "LootingMethodID", kind: ParamKind::Int }, ParamSpec { name: "ZoneID", kind: ParamKind::Int }, ParamSpec { name: "RoomName", kind: ParamKind::Str }], source: "0x101665c0#1" },
    EventSpec { id: 1560, params: &[], source: "0x10166a80#1" },
    EventSpec { id: 1580, params: &[ParamSpec { name: "MyMembershipType", kind: ParamKind::Int }, ParamSpec { name: "MemberCount", kind: ParamKind::Int }, ParamSpec { name: "MemberID_%d", kind: ParamKind::Loop }, ParamSpec { name: "MemberName_%d", kind: ParamKind::Loop }, ParamSpec { name: "ClassID_%d", kind: ParamKind::Loop }, ParamSpec { name: "Level_%d", kind: ParamKind::Loop }, ParamSpec { name: "ZoneID_%d", kind: ParamKind::Loop }, ParamSpec { name: "MembershipType_%d", kind: ParamKind::Loop }], source: "0x101667e0#1" },
    EventSpec { id: 1590, params: &[ParamSpec { name: "UpdateType", kind: ParamKind::Int }, ParamSpec { name: "MemberID", kind: ParamKind::Int }, ParamSpec { name: "MemberName", kind: ParamKind::Str }, ParamSpec { name: "ClassID", kind: ParamKind::Int }, ParamSpec { name: "Level", kind: ParamKind::Int }, ParamSpec { name: "ZoneID", kind: ParamKind::Int }, ParamSpec { name: "MembershipType", kind: ParamKind::Int }], source: "0x10166b40#1" },
    EventSpec { id: 1600, params: &[ParamSpec { name: "ColorR", kind: ParamKind::Int }, ParamSpec { name: "ColorG", kind: ParamKind::Int }, ParamSpec { name: "ColorB", kind: ParamKind::Int }, ParamSpec { name: "ColorA", kind: ParamKind::Int }], source: "0x10136ec0#1" },
    EventSpec { id: 1600, params: &[], source: "0x101374f4#1" },
    EventSpec { id: 1610, params: &[ParamSpec { name: "AllCount", kind: ParamKind::Int }, ParamSpec { name: "Count", kind: ParamKind::Int }], source: "0x10167d10#1" },
    EventSpec { id: 1620, params: &[ParamSpec { name: "Name", kind: ParamKind::Str }, ParamSpec { name: "ClassID", kind: ParamKind::Int }, ParamSpec { name: "Level", kind: ParamKind::Int }], source: "0x10167e30#1" },
    EventSpec { id: 1630, params: &[], source: "0x1016e780#1" },
    EventSpec { id: 1640, params: &[ParamSpec { name: "Adena", kind: ParamKind::Int }], source: "0x10156c10#1" },
    EventSpec { id: 1660, params: &[ParamSpec { name: "Adena", kind: ParamKind::Int }, ParamSpec { name: "DyeIconName", kind: ParamKind::Str }, ParamSpec { name: "DyeName", kind: ParamKind::Str }, ParamSpec { name: "HennaID", kind: ParamKind::Int }, ParamSpec { name: "ClassID", kind: ParamKind::Int }, ParamSpec { name: "NumOfItem", kind: ParamKind::Int }, ParamSpec { name: "Fee", kind: ParamKind::Int }, ParamSpec { name: "TattooIconName", kind: ParamKind::Str }, ParamSpec { name: "TattooName", kind: ParamKind::Str }, ParamSpec { name: "TattooAddName", kind: ParamKind::Str }, ParamSpec { name: "INTnow", kind: ParamKind::Int }, ParamSpec { name: "INTchange", kind: ParamKind::Int }, ParamSpec { name: "STRnow", kind: ParamKind::Int }, ParamSpec { name: "STRchange", kind: ParamKind::Int }, ParamSpec { name: "CONnow", kind: ParamKind::Int }, ParamSpec { name: "CONchange", kind: ParamKind::Int }, ParamSpec { name: "MENnow", kind: ParamKind::Int }, ParamSpec { name: "MENchange", kind: ParamKind::Int }, ParamSpec { name: "DEXnow", kind: ParamKind::Int }, ParamSpec { name: "DEXchange", kind: ParamKind::Int }, ParamSpec { name: "WITnow", kind: ParamKind::Int }, ParamSpec { name: "WITchange", kind: ParamKind::Int }], source: "0x10157020#1" },
    EventSpec { id: 1670, params: &[ParamSpec { name: "Adena", kind: ParamKind::Int }], source: "0x101577b0#1" },
    EventSpec { id: 1690, params: &[ParamSpec { name: "Adena", kind: ParamKind::Int }, ParamSpec { name: "DyeIconName", kind: ParamKind::Str }, ParamSpec { name: "DyeName", kind: ParamKind::Str }, ParamSpec { name: "HennaID", kind: ParamKind::Int }, ParamSpec { name: "ClassID", kind: ParamKind::Int }, ParamSpec { name: "NumOfItem", kind: ParamKind::Int }, ParamSpec { name: "Fee", kind: ParamKind::Int }, ParamSpec { name: "TattooIconName", kind: ParamKind::Str }, ParamSpec { name: "TattooName", kind: ParamKind::Str }, ParamSpec { name: "TattooAddName", kind: ParamKind::Str }, ParamSpec { name: "INTnow", kind: ParamKind::Int }, ParamSpec { name: "INTchange", kind: ParamKind::Int }, ParamSpec { name: "STRnow", kind: ParamKind::Int }, ParamSpec { name: "STRchange", kind: ParamKind::Int }, ParamSpec { name: "CONnow", kind: ParamKind::Int }, ParamSpec { name: "CONchange", kind: ParamKind::Int }, ParamSpec { name: "MENnow", kind: ParamKind::Int }, ParamSpec { name: "MENchange", kind: ParamKind::Int }, ParamSpec { name: "DEXnow", kind: ParamKind::Int }, ParamSpec { name: "DEXchange", kind: ParamKind::Int }, ParamSpec { name: "WITnow", kind: ParamKind::Int }, ParamSpec { name: "WITchange", kind: ParamKind::Int }], source: "0x10157f20#1" },
    EventSpec { id: 1700, params: &[], source: "0x10160800#1" },
    EventSpec { id: 1750, params: &[], source: "0x1015f2f0#1" },
    EventSpec { id: 1770, params: &[ParamSpec { name: "TownMapName", kind: ParamKind::Str }, ParamSpec { name: "UserPosX", kind: ParamKind::Int }, ParamSpec { name: "UserPosY", kind: ParamKind::Int }], source: "0x10163af0#1" },
    EventSpec { id: 1780, params: &[ParamSpec { name: "SSQStatus", kind: ParamKind::Int }], source: "0x1013c380#1" },
    EventSpec { id: 1800, params: &[], source: "0x1015f560#1" },
    EventSpec { id: 1810, params: &[], source: "0x1015f2f0#3" },
    EventSpec { id: 1840, params: &[], source: "0x1015f6a0#1" },
    EventSpec { id: 1850, params: &[ParamSpec { name: "ID%d", kind: ParamKind::Loop }, ParamSpec { name: "NAME%d", kind: ParamKind::Loop }], source: "0x1015f750#1" },
    EventSpec { id: 1870, params: &[], source: "0x101886a0#1" },
    EventSpec { id: 1880, params: &[], source: "0x10185800#1" },
    EventSpec { id: 1890, params: &[ParamSpec { name: "GameHour", kind: ParamKind::Int }, ParamSpec { name: "GameMinute", kind: ParamKind::Int }], source: "0x10187270#1" },
    EventSpec { id: 1900, params: &[], source: "0x10162860#1" },
    EventSpec { id: 1910, params: &[ParamSpec { name: "TotalPoint", kind: ParamKind::Int }, ParamSpec { name: "AddPoint", kind: ParamKind::Int }, ParamSpec { name: "PeriodType", kind: ParamKind::Int }, ParamSpec { name: "RemainTime", kind: ParamKind::Int }, ParamSpec { name: "PointType", kind: ParamKind::Int }], source: "0x10167390#1" },
    EventSpec { id: 1920, params: &[], source: "0x1013ce80#1" },
    EventSpec { id: 1930, params: &[ParamSpec { name: "Message", kind: ParamKind::Str }, ParamSpec { name: "ColorR", kind: ParamKind::Int }, ParamSpec { name: "ColorG", kind: ParamKind::Int }, ParamSpec { name: "ColorB", kind: ParamKind::Int }, ParamSpec { name: "ColorA", kind: ParamKind::Int }], source: "0x1013ce80#2" },
    EventSpec { id: 1940, params: &[], source: "0x10164cc0#1" },
    EventSpec { id: 1950, params: &[ParamSpec { name: "targetID", kind: ParamKind::Int }], source: "0x101504a0#2" },
    EventSpec { id: 1960, params: &[], source: "0x101505b0#1" },
    EventSpec { id: 1970, params: &[], source: "0x10150a20#1" },
    EventSpec { id: 1980, params: &[], source: "0x10150b10#1" },
    EventSpec { id: 1990, params: &[ParamSpec { name: "type", kind: ParamKind::Str }, ParamSpec { name: "type", kind: ParamKind::Str }], source: "0x10150880#1" },
    EventSpec { id: 2010, params: &[ParamSpec { name: "Type", kind: ParamKind::Int }], source: "0x101554b0#1" },
    EventSpec { id: 2020, params: &[], source: "0x101558a0#2" },
    EventSpec { id: 2030, params: &[], source: "0x1016a37a#1" },
    EventSpec { id: 2050, params: &[], source: "0x101558a0#1" },
    EventSpec { id: 2080, params: &[ParamSpec { name: "type", kind: ParamKind::Str }, ParamSpec { name: "merchant", kind: ParamKind::Int }, ParamSpec { name: "adena", kind: ParamKind::Int }], source: "0x10151050#1" },
    EventSpec { id: 2080, params: &[ParamSpec { name: "merchant", kind: ParamKind::Int }, ParamSpec { name: "adena", kind: ParamKind::Int }], source: "0x101514a0#2" },
    EventSpec { id: 2090, params: &[], source: "0x101511b0#1" },
    EventSpec { id: 2100, params: &[ParamSpec { name: "category", kind: ParamKind::Int }, ParamSpec { name: "adena", kind: ParamKind::Int }], source: "0x10151910#2" },
    EventSpec { id: 2110, params: &[], source: "0x10151ac0#1" },
    EventSpec { id: 2120, params: &[ParamSpec { name: "userID", kind: ParamKind::Int }, ParamSpec { name: "adena", kind: ParamKind::Int }], source: "0x10160fe0#1" },
    EventSpec { id: 2130, params: &[], source: "0x10160bd0#1" },
    EventSpec { id: 2130, params: &[ParamSpec { name: "type", kind: ParamKind::Str }], source: "0x10160e90#1" },
    EventSpec { id: 2140, params: &[], source: "0x1015b440#1" },
    EventSpec { id: 2150, params: &[ParamSpec { name: "name", kind: ParamKind::Str }], source: "0x1015b440#2" },
    EventSpec { id: 2160, params: &[ParamSpec { name: "adena", kind: ParamKind::Int }, ParamSpec { name: "destinationID", kind: ParamKind::Int }], source: "0x1015b5c0#1" },
    EventSpec { id: 2170, params: &[], source: "0x1015b5c0#2" },
    EventSpec { id: 2180, params: &[], source: "0x101657c0#1" },
    EventSpec { id: 2190, params: &[ParamSpec { name: "MatchID", kind: ParamKind::Int }], source: "0x101653e0#1" },
    EventSpec { id: 2210, params: &[ParamSpec { name: "MatchID", kind: ParamKind::Int }, ParamSpec { name: "MatchStat", kind: ParamKind::Int }, ParamSpec { name: "FenceStat", kind: ParamKind::Int }, ParamSpec { name: "Team1Name", kind: ParamKind::Str }, ParamSpec { name: "Team2Name", kind: ParamKind::Str }, ParamSpec { name: "Team1Stat", kind: ParamKind::Int }, ParamSpec { name: "Team2Stat", kind: ParamKind::Int }], source: "0x10166d60#1" },
    EventSpec { id: 2240, params: &[], source: "0x101651f0#1" },
    EventSpec { id: 2250, params: &[ParamSpec { name: "TeamID", kind: ParamKind::Int }], source: "0x101652e0#1" },
    EventSpec { id: 2260, params: &[ParamSpec { name: "TeamID", kind: ParamKind::Int }, ParamSpec { name: "UserID", kind: ParamKind::Int }], source: "0x10165670#1" },
    EventSpec { id: 2270, params: &[ParamSpec { name: "Type", kind: ParamKind::Int }, ParamSpec { name: "Message", kind: ParamKind::Str }], source: "0x101650d0#1" },
    EventSpec { id: 2280, params: &[], source: "0x1013c6c0#1" },
    EventSpec { id: 2290, params: &[], source: "0x101af7c0#1" },
    EventSpec { id: 2300, params: &[], source: "0x1015e960#1" },
    EventSpec { id: 2310, params: &[], source: "0x1016bd95#1" },
    EventSpec { id: 2320, params: &[], source: "0x1015ea10#1" },
    EventSpec { id: 2340, params: &[], source: "0x1015eac0#1" },
    EventSpec { id: 2350, params: &[ParamSpec { name: "ClassID", kind: ParamKind::Int }, ParamSpec { name: "ItemCount", kind: ParamKind::Int }], source: "0x1015eb70#1" },
    EventSpec { id: 2360, params: &[ParamSpec { name: "category", kind: ParamKind::Int }, ParamSpec { name: "adena", kind: ParamKind::Int }], source: "0x1015ee20#1" },
    EventSpec { id: 2380, params: &[ParamSpec { name: "ClanID", kind: ParamKind::Int }, ParamSpec { name: "ClanType", kind: ParamKind::Int }, ParamSpec { name: "ClanName", kind: ParamKind::Str }, ParamSpec { name: "ClanMasterName", kind: ParamKind::Str }, ParamSpec { name: "CrestID", kind: ParamKind::Int }, ParamSpec { name: "SkillLevel", kind: ParamKind::Int }, ParamSpec { name: "CastleID", kind: ParamKind::Int }, ParamSpec { name: "AgitID", kind: ParamKind::Int }, ParamSpec { name: "ClanRank", kind: ParamKind::Int }, ParamSpec { name: "ClanNameValue", kind: ParamKind::Int }, ParamSpec { name: "Status", kind: ParamKind::Int }, ParamSpec { name: "Guilty", kind: ParamKind::Int }, ParamSpec { name: "AllianceID", kind: ParamKind::Int }, ParamSpec { name: "AllianceName", kind: ParamKind::Str }, ParamSpec { name: "AllianceCrestID", kind: ParamKind::Int }, ParamSpec { name: "InWar", kind: ParamKind::Int }], source: "0x1015e260#1" },
    EventSpec { id: 2390, params: &[], source: "0x1015e660#1" },
    EventSpec { id: 2400, params: &[ParamSpec { name: "Name", kind: ParamKind::Str }, ParamSpec { name: "Level", kind: ParamKind::Int }, ParamSpec { name: "Class", kind: ParamKind::Int }, ParamSpec { name: "Gender", kind: ParamKind::Int }, ParamSpec { name: "Race", kind: ParamKind::Int }, ParamSpec { name: "ClanType", kind: ParamKind::Int }], source: "0x1015e710#1" },
    EventSpec { id: 2401, params: &[], source: "0x1015ed60#1" },
    EventSpec { id: 2402, params: &[ParamSpec { name: "InvenLimit", kind: ParamKind::Int }], source: "0x1015ec80#1" },
    EventSpec { id: 2403, params: &[ParamSpec { name: "bActive", kind: ParamKind::Int }], source: "0x10157bc0#1" },
    EventSpec { id: 2404, params: &[], source: "0x10157cd0#1" },
    EventSpec { id: 2410, params: &[ParamSpec { name: "SnoopID", kind: ParamKind::Int }, ParamSpec { name: "SnoopName", kind: ParamKind::Str }, ParamSpec { name: "CreatureID", kind: ParamKind::Int }, ParamSpec { name: "Type", kind: ParamKind::Int }, ParamSpec { name: "CharacterName", kind: ParamKind::Str }, ParamSpec { name: "ChatMessage", kind: ParamKind::Str }], source: "0x10161d80#1" },
    EventSpec { id: 2430, params: &[ParamSpec { name: "HtmlString", kind: ParamKind::Str }], source: "0x1015d3e0#1" },
    EventSpec { id: 2440, params: &[], source: "0x1015dac0#1" },
    EventSpec { id: 2450, params: &[], source: "0x10163c60#1" },
    EventSpec { id: 2460, params: &[], source: "0x10163d20#1" },
    EventSpec { id: 2480, params: &[], source: "0x10166130#1" },
    EventSpec { id: 2530, params: &[ParamSpec { name: "shopID", kind: ParamKind::Int }], source: "0x101616a0#1" },
    EventSpec { id: 2550, params: &[ParamSpec { name: "refineryOp1", kind: ParamKind::Int }, ParamSpec { name: "refineryOp2", kind: ParamKind::Int }, ParamSpec { name: "ClassID", kind: ParamKind::Int }, ParamSpec { name: "count", kind: ParamKind::Int }, ParamSpec { name: "enchant", kind: ParamKind::Int }, ParamSpec { name: "inputRefineryOp1", kind: ParamKind::Int }, ParamSpec { name: "inputRefineryOp2", kind: ParamKind::Int }], source: "0x10161a50#1" },
    EventSpec { id: 2560, params: &[], source: "0x10161c80#1" },
    EventSpec { id: 2570, params: &[], source: "0x1014fa80#1" },
    EventSpec { id: 2580, params: &[], source: "0x1014fa80#2" },
    EventSpec { id: 2590, params: &[], source: "0x101504a0#1" },
    EventSpec { id: 2600, params: &[], source: "0x1014fc40#1" },
    EventSpec { id: 2610, params: &[ParamSpec { name: "type", kind: ParamKind::Str }, ParamSpec { name: "type", kind: ParamKind::Str }], source: "0x1014fd20#1" },
    EventSpec { id: 2620, params: &[], source: "0x1014ff00#1" },
    EventSpec { id: 2630, params: &[ParamSpec { name: "bActive", kind: ParamKind::Int }], source: "0x10157680#1" },
    EventSpec { id: 2631, params: &[], source: "0x1013c300#1" },
    EventSpec { id: 2647, params: &[], source: "0x10154060#1" },
    EventSpec { id: 2648, params: &[ParamSpec { name: "ManorID", kind: ParamKind::Int }, ParamSpec { name: "CropRemainCnt", kind: ParamKind::Int }, ParamSpec { name: "CropPrice", kind: ParamKind::Int }, ParamSpec { name: "ProcureType", kind: ParamKind::Int }, ParamSpec { name: "ManorName", kind: ParamKind::Str }], source: "0x10154060#2" },
    EventSpec { id: 2651, params: &[ParamSpec { name: "RewardType1", kind: ParamKind::Str }], source: "0x101521d1#1" },
    EventSpec { id: 2680, params: &[ParamSpec { name: "type", kind: ParamKind::Str }, ParamSpec { name: "merchant", kind: ParamKind::Int }, ParamSpec { name: "adena", kind: ParamKind::Int }], source: "0x10150bc0#1" },
    EventSpec { id: 2680, params: &[ParamSpec { name: "merchant", kind: ParamKind::Int }, ParamSpec { name: "adena", kind: ParamKind::Int }], source: "0x10150df0#2" },
    EventSpec { id: 2690, params: &[], source: "0x10150d30#1" },
    EventSpec { id: 2700, params: &[ParamSpec { name: "userName", kind: ParamKind::Str }, ParamSpec { name: "type", kind: ParamKind::Int }], source: "0x1015acc0#1" },
    EventSpec { id: 2710, params: &[ParamSpec { name: "type", kind: ParamKind::Int }], source: "0x1015ade0#1" },
    EventSpec { id: 2720, params: &[ParamSpec { name: "type", kind: ParamKind::Int }], source: "0x1015aea0#1" },
    EventSpec { id: 2730, params: &[ParamSpec { name: "type", kind: ParamKind::Int }], source: "0x1015af60#1" },
    EventSpec { id: 2740, params: &[ParamSpec { name: "userName", kind: ParamKind::Str }, ParamSpec { name: "class", kind: ParamKind::Int }, ParamSpec { name: "level", kind: ParamKind::Int }, ParamSpec { name: "currentHP", kind: ParamKind::Int }, ParamSpec { name: "maxHP", kind: ParamKind::Int }, ParamSpec { name: "currentMP", kind: ParamKind::Int }, ParamSpec { name: "maxMP", kind: ParamKind::Int }, ParamSpec { name: "currentCP", kind: ParamKind::Int }, ParamSpec { name: "maxCP", kind: ParamKind::Int }], source: "0x1015b020#1" },
    EventSpec { id: 2750, params: &[ParamSpec { name: "userName", kind: ParamKind::Str }, ParamSpec { name: "bEnemy", kind: ParamKind::Int }], source: "0x1015b2e0#1" },
    EventSpec { id: 2760, params: &[], source: "0x1015fe30#1" },
    EventSpec { id: 2770, params: &[ParamSpec { name: "TargetItemServerID", kind: ParamKind::Int }, ParamSpec { name: "TargetItemClassID", kind: ParamKind::Int }, ParamSpec { name: "Result", kind: ParamKind::Int }], source: "0x1015ff90#1" },
    EventSpec { id: 2780, params: &[ParamSpec { name: "RefinerItemServerID", kind: ParamKind::Int }, ParamSpec { name: "RefinerItemClassID", kind: ParamKind::Int }, ParamSpec { name: "GemStoneClassID", kind: ParamKind::Int }, ParamSpec { name: "GemStoneCount", kind: ParamKind::Int }, ParamSpec { name: "Result", kind: ParamKind::Int }], source: "0x101600d0#1" },
    EventSpec { id: 2790, params: &[ParamSpec { name: "GemStoneServerID", kind: ParamKind::Int }, ParamSpec { name: "GemStoneClassID", kind: ParamKind::Int }, ParamSpec { name: "GemStoneCount", kind: ParamKind::Int }, ParamSpec { name: "NecessaryGemStoneCount", kind: ParamKind::Int }, ParamSpec { name: "Result", kind: ParamKind::Int }], source: "0x10160270#1" },
    EventSpec { id: 2800, params: &[ParamSpec { name: "Option1", kind: ParamKind::Int }, ParamSpec { name: "Option2", kind: ParamKind::Int }, ParamSpec { name: "Result", kind: ParamKind::Int }], source: "0x10160410#1" },
    EventSpec { id: 2810, params: &[], source: "0x1015fee0#1" },
    EventSpec { id: 2820, params: &[ParamSpec { name: "CancelItemServerID", kind: ParamKind::Int }, ParamSpec { name: "CancelItemClassID", kind: ParamKind::Int }, ParamSpec { name: "Option1", kind: ParamKind::Int }, ParamSpec { name: "Option2", kind: ParamKind::Int }, ParamSpec { name: "Adena", kind: ParamKind::Int64 }, ParamSpec { name: "Result", kind: ParamKind::Int }], source: "0x10160550#1" },
    EventSpec { id: 2830, params: &[ParamSpec { name: "Result", kind: ParamKind::Int }], source: "0x10160720#1" },
    EventSpec { id: 2840, params: &[], source: "0x101691b0#1" },
    EventSpec { id: 2850, params: &[ParamSpec { name: "QuestID", kind: ParamKind::Int }, ParamSpec { name: "QuestName", kind: ParamKind::Str }, ParamSpec { name: "QuestRequirement", kind: ParamKind::Str }, ParamSpec { name: "QuestLevel", kind: ParamKind::Str }, ParamSpec { name: "QuestType", kind: ParamKind::Int }, ParamSpec { name: "QuestNpcName", kind: ParamKind::Str }, ParamSpec { name: "QuestDecription", kind: ParamKind::Str }], source: "0x101691b0#2" },
    EventSpec { id: 2860, params: &[ParamSpec { name: "ClassID", kind: ParamKind::Int }], source: "0x1014ff80#1" },
    EventSpec { id: 2880, params: &[], source: "0x1014ff80#2" },
    EventSpec { id: 2890, params: &[ParamSpec { name: "Result", kind: ParamKind::Int }], source: "0x101502c0#1" },
    EventSpec { id: 2900, params: &[ParamSpec { name: "NewWidth", kind: ParamKind::Int }, ParamSpec { name: "NewHeight", kind: ParamKind::Int }], source: "0x1015fc70#1" },
    EventSpec { id: 2910, params: &[ParamSpec { name: "TabNum", kind: ParamKind::Int }], source: "0x1016c520#1" },
    EventSpec { id: 2920, params: &[ParamSpec { name: "SourceType", kind: ParamKind::Int }, ParamSpec { name: "Text", kind: ParamKind::Str }], source: "0x101b6610#1" },
    EventSpec { id: 2920, params: &[ParamSpec { name: "SourceType", kind: ParamKind::Int }], source: "0x101b6860#1" },
    EventSpec { id: 2920, params: &[ParamSpec { name: "SourceType", kind: ParamKind::Int }, ParamSpec { name: "TooltipType", kind: ParamKind::Str }], source: "0x101b6a30#1" },
    EventSpec { id: 2920, params: &[ParamSpec { name: "SourceType", kind: ParamKind::Int }, ParamSpec { name: "TooltipType", kind: ParamKind::Str }, ParamSpec { name: "szReserved", kind: ParamKind::Str }, ParamSpec { name: "nReserved1", kind: ParamKind::Int }, ParamSpec { name: "nReserved2", kind: ParamKind::Int }, ParamSpec { name: "nReserved3", kind: ParamKind::Int }, ParamSpec { name: "MaxColumn", kind: ParamKind::Int }, ParamSpec { name: "szData_%d", kind: ParamKind::Loop }, ParamSpec { name: "szReserved_%d", kind: ParamKind::Loop }, ParamSpec { name: "nReserved1_%d", kind: ParamKind::Loop }, ParamSpec { name: "nReserved2_%d", kind: ParamKind::Loop }, ParamSpec { name: "nReserved3_%d", kind: ParamKind::Loop }], source: "0x101b6bc0#1" },
    EventSpec { id: 3000, params: &[], source: "0x1015fd80#1" },
    EventSpec { id: 1050, params: &[ParamSpec { name: "Max", kind: ParamKind::Int }, ParamSpec { name: "ID", kind: ParamKind::Int }, ParamSpec { name: "SkillID_%d", kind: ParamKind::Loop }, ParamSpec { name: "Level_%d", kind: ParamKind::Loop }, ParamSpec { name: "Sec_%d", kind: ParamKind::Loop }], source: "0x1015c660#1 (0x41a por tipo)" },
    EventSpec { id: 1110, params: &[ParamSpec { name: "Max", kind: ParamKind::Int }, ParamSpec { name: "ID", kind: ParamKind::Int }, ParamSpec { name: "SkillID_%d", kind: ParamKind::Loop }, ParamSpec { name: "Level_%d", kind: ParamKind::Loop }, ParamSpec { name: "Sec_%d", kind: ParamKind::Loop }], source: "0x1015c660#1 (0x456 por tipo)" },
    EventSpec { id: 1180, params: &[ParamSpec { name: "Max", kind: ParamKind::Int }, ParamSpec { name: "ID", kind: ParamKind::Int }, ParamSpec { name: "SkillID_%d", kind: ParamKind::Loop }, ParamSpec { name: "Level_%d", kind: ParamKind::Loop }, ParamSpec { name: "Sec_%d", kind: ParamKind::Loop }], source: "0x1015c660#1 (0x49c por tipo)" },
    EventSpec { id: 1730, params: &[ParamSpec { name: "X", kind: ParamKind::Int }, ParamSpec { name: "Y", kind: ParamKind::Int }, ParamSpec { name: "Z", kind: ParamKind::Int }], source: "0x1015f2f0 (0x6c2 show 0)" },
    EventSpec { id: 1740, params: &[ParamSpec { name: "X", kind: ParamKind::Int }, ParamSpec { name: "Y", kind: ParamKind::Int }, ParamSpec { name: "Z", kind: ParamKind::Int }], source: "0x1015f2f0 (0x6cc show 1)" },
    EventSpec { id: 1790, params: &[ParamSpec { name: "X", kind: ParamKind::Int }, ParamSpec { name: "Y", kind: ParamKind::Int }, ParamSpec { name: "Z", kind: ParamKind::Int }], source: "0x1015f2f0 (0x6fe show 0)" },
    EventSpec { id: 1800, params: &[ParamSpec { name: "X", kind: ParamKind::Int }, ParamSpec { name: "Y", kind: ParamKind::Int }, ParamSpec { name: "Z", kind: ParamKind::Int }], source: "0x1015f2f0 (0x708 show 1)" },
];

#[cfg(test)]
mod tests {
    use super::*;
    use l2_net::game::{Hit, StatusUpdate};

    fn only(event: &WorldEvent) -> Vec<(i32, String)> {
        bridge(event, &World::default(), None)
    }

    #[test]
    fn status_updates_become_one_event_per_attribute_with_server_id_first() {
        let event = WorldEvent::StatusUpdate(StatusUpdate { object_id: 7, attributes: vec![(game::ATTRIBUTE_CURRENT_HP, 400), (game::ATTRIBUTE_MAX_CP, 90), (3, 1)] });
        assert_eq!(only(&event), [(190, "ServerID=7 CurrentHP=400".to_string()), (240, "ServerID=7 MaxCP=90".to_string())]);
    }

    #[test]
    fn attacks_carry_one_event_per_hit_with_the_flags_as_ints() {
        let event = WorldEvent::Attack(game::Attack { attacker_id: 1, x: 0, y: 0, z: 0, hits: vec![Hit { target_id: 2, damage: 10, flags: game::HIT_CRITICAL }, Hit { target_id: 3, damage: 0, flags: game::HIT_MISS | game::HIT_SHIELD }] });
        let out = only(&event);
        assert_eq!(out[0], (280, "AttackerID=1 DefenderID=2 Miss=0 ShieldDefense=0 Critical=1".to_string()));
        assert_eq!(out[1].1, "AttackerID=1 DefenderID=3 Miss=1 ShieldDefense=1 Critical=0");
    }

    #[test]
    fn party_lists_clear_count_and_add_in_the_dll_order() {
        let member = PartyMember { object_id: 5, name: "Ana".into(), cp: 1, max_cp: 2, hp: 3, max_hp: 4, mp: 5, max_mp: 6, level: 7, class_id: 8 };
        let event = WorldEvent::PartySmallWindowAll(game::PartySmallWindowAll { leader_id: 5, loot_distribution: 0, members: vec![member.clone()] });
        let out = only(&event);
        assert_eq!(out[0].0, 1170);
        assert_eq!(out[1], (520, "PartyMemberCount=0".to_string()));
        assert_eq!(out[2], (550, "PartyMemberCount=0".to_string()));
        assert_eq!(out[3], (1140, "Name=\u{7f}Ana\u{7f} ID=5 CurCP=1 MaxCP=2 CurHP=3 MaxHP=4 CurMP=5 MaxMP=6 ClassID=8 Level=7".to_string()));
        let spelled = WorldEvent::PartySpelled(game::PartySpelled { summon_type: 1, object_id: 9, effects: vec![game::PartyBuff { skill_id: 1204, level: 2, duration: 30 }] });
        assert_eq!(only(&spelled), [(1050, "Max=1 ID=9 SkillID_0=1204 Level_0=2 Sec_0=30".to_string())]);
    }

    #[test]
    fn radar_control_picks_the_radar_and_minimap_ids_by_show_and_kind() {
        let event = WorldEvent::RadarControl(game::RadarControl { show: 2, kind: 2, x: 1, y: 2, z: 3 });
        assert_eq!(only(&event), [(1750, "X=1 Y=2 Z=3".to_string()), (1810, "X=1 Y=2 Z=3".to_string())]);
        let event = WorldEvent::RadarControl(game::RadarControl { show: 0, kind: 0, x: 1, y: 2, z: 3 });
        assert_eq!(only(&event), [(1730, "X=1 Y=2 Z=3".to_string())]);
    }

    #[test]
    fn shortcuts_trade_and_tutorial_follow_the_builders() {
        let init = WorldEvent::ShortCutInit(game::ShortCutInit { shortcuts: vec![game::Shortcut { slot: 3, kind: game::ShortcutKind::Item { object_id: 1 }, character_type: 1 }] });
        assert_eq!(only(&init), [(650, String::new()), (630, "ShortcutID=3".to_string())]);
        let trade = WorldEvent::TradeStart(game::TradeStart { partner_id: 44, items: Vec::new() });
        assert_eq!(only(&trade), [(2590, "targetID=44".to_string()), (1950, "targetID=44".to_string())]);
        let html = WorldEvent::TutorialShowHtml(game::TutorialShowHtml { html: "<html>oi</html>".into() });
        assert_eq!(only(&html), [(2430, "HtmlString=\u{7f}<html>oi</html>\u{7f}".to_string())]);
    }

    #[test]
    fn every_bridged_id_has_a_spec_from_the_dll_and_the_specs_are_unique_per_shape() {
        for id in [50, 70, 190, 200, 210, 220, 230, 240, 280, 290, 300, 330, 410, 420, 440, 450, 520, 550, 630, 650, 700, 720, 780, 800, 820, 840, 850, 920, 950, 980, 1050, 1110, 1050, 1110, 1120, 1150, 1160, 1170, 1180, 1230, 1290, 1420, 1510, 1520, 1530, 1550, 1580, 1590, 1610, 1620, 1640, 1660, 1670, 1690, 1730, 1740, 1750, 1770, 1790, 1800, 1810, 1950, 1970, 2430, 2570, 2580, 2590] {
            assert!(spec(id).is_some(), "sem spec para {id}");
        }
        let shapes: Vec<Vec<&str>> = SPECS.iter().filter(|spec| spec.id == 950).map(|spec| spec.params.iter().map(|p| p.name).collect()).collect();
        assert!(shapes.contains(&vec!["Max", "SkillID_%d", "SkillLevel_%d", "RemainTime_%d", "Name_%d", "IconName_%d", "Description_%d"]), "{shapes:?}");
    }
}
