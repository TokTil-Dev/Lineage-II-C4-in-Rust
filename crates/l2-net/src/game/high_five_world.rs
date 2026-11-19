use super::packets::{
    CHAT_BOAT, CHAT_TELL, CharInfo, InventoryItem, InventoryUpdate, ItemChange, ItemList, MagicSkillUse,
    NPC_TYPE_OFFSET, NpcInfo, NpcSay, PAPERDOLL_SLOTS, PaperdollSlot, Say2, Say2Body, SkillEntry, SkillList, Shortcut,
    ShortcutKind, ShortCutInit, SpawnItem, SystemMessage, SystemMessageArg, UserInfoHead, UserSocial, UserStats,
    VISIBLE_EQUIP_SLOTS, WAIT_TYPE_SIT, DropItem, EtcStatusUpdate, ETC_STATUS_VALUES, StorageMaxCount,
    MOVEMENT_MODE_MOUSE, SHORTCUT_ACTION, SHORTCUT_ITEM, SHORTCUT_MACRO, SHORTCUT_RECIPE, SHORTCUT_SKILL,
    counted, read_d_or_zero,
    BLOCK_ADD, BLOCK_REMOVE, Macro,
    ItemOrder, PartyMember, PartySmallWindowAdd, PartySmallWindowAll, PledgeMember, PledgeShowInfoUpdate,
    PledgeShowMemberListAll, RelationChanged, TradeAdd, TradeItem, TradeStart,
    BuyOffer, CropProcure, CropSetting, SellToStoreItem, StoreBuyOrder, StoreOffer,
};
use crate::wire::{Reader, Result, WireError, Writer};

pub const OP_ENTER_WORLD: u8 = 0x11;
pub const OP_REQUEST_RESTART: u8 = 0x57;
pub const OP_GAME_GUARD_REPLY: u8 = 0xcb;

pub const OP_DIE: u8 = 0x00;
pub const OP_REVIVE: u8 = 0x01;
pub const OP_SPAWN_ITEM: u8 = 0x05;
pub const OP_DELETE_OBJECT: u8 = 0x08;
pub const OP_NPC_INFO: u8 = 0x0c;
pub const OP_ITEM_LIST: u8 = 0x11;
pub const OP_DROP_ITEM: u8 = 0x16;
pub const OP_GET_ITEM: u8 = 0x17;
pub const OP_STATUS_UPDATE: u8 = 0x18;
pub const OP_NPC_HTML_MESSAGE: u8 = 0x19;
pub const OP_ACTION_FAILED: u8 = 0x1f;
pub const OP_INVENTORY_UPDATE: u8 = 0x21;
pub const OP_TELEPORT_TO_LOCATION: u8 = 0x22;
pub const OP_TARGET_SELECTED: u8 = 0x23;
pub const OP_TARGET_UNSELECTED: u8 = 0x24;
pub const OP_AUTO_ATTACK_START: u8 = 0x25;
pub const OP_AUTO_ATTACK_STOP: u8 = 0x26;
pub const OP_SOCIAL_ACTION: u8 = 0x27;
pub const OP_CHANGE_MOVE_TYPE: u8 = 0x28;
pub const OP_CHANGE_WAIT_TYPE: u8 = 0x29;
pub const OP_MOVE_TO_LOCATION: u8 = 0x2f;
pub const OP_NPC_SAY: u8 = 0x30;
pub const OP_CHAR_INFO: u8 = 0x31;
pub const OP_USER_INFO: u8 = 0x32;
pub const OP_ATTACK: u8 = 0x33;
pub const OP_SHORT_CUT_REGISTER: u8 = 0x44;
pub const OP_SHORT_CUT_INIT: u8 = 0x45;
pub const OP_SHORT_CUT_DELETE: u8 = 0x46;
pub const OP_STOP_MOVE: u8 = 0x47;
pub const OP_MAGIC_SKILL_USE: u8 = 0x48;
pub const OP_CREATURE_SAY: u8 = 0x4a;
pub const OP_MAGIC_SKILL_LAUNCHED: u8 = 0x54;
pub const OP_SKILL_LIST: u8 = 0x5f;
pub const OP_SYSTEM_MESSAGE: u8 = 0x62;
pub const OP_RESTART_RESPONSE: u8 = 0x71;
pub const OP_GAME_GUARD_QUERY: u8 = 0x74;
pub const OP_VALIDATE_LOCATION: u8 = 0x79;
pub const OP_LEAVE_WORLD: u8 = 0x84;
pub const OP_ABNORMAL_STATUS_UPDATE: u8 = 0x85;
pub const OP_QUEST_LIST: u8 = 0x86;
pub const OP_MY_TARGET_SELECTED: u8 = 0xb9;
pub const OP_SKILL_COOL_TIME: u8 = 0xc7;
pub const OP_HENNA_INFO: u8 = 0xe5;
pub const OP_ETC_STATUS_UPDATE: u8 = 0xf9;
pub const OP_EX_SERVER: u8 = 0xfe;

pub const EX_STORAGE_MAX_COUNT: u16 = 0x2f;
pub const EX_BASIC_ACTION_LIST: u16 = 0x5f;
pub const EX_ROTATION: u16 = 0xc1;

pub const OP_MOVE_BACKWARD_TO_LOCATION: u8 = 0x0f;
pub const OP_REQUEST_ITEM_LIST: u8 = 0x14;
pub const OP_REQUEST_UNEQUIP_ITEM: u8 = 0x16;
pub const OP_REQUEST_DROP_ITEM: u8 = 0x17;
pub const OP_USE_ITEM: u8 = 0x19;
pub const OP_ACTION: u8 = 0x1f;
pub const OP_REQUEST_LINK_HTML: u8 = 0x22;
pub const OP_REQUEST_BYPASS_TO_SERVER: u8 = 0x23;
pub const OP_ATTACK_REQUEST: u8 = 0x32;
pub const OP_REQUEST_MAGIC_SKILL_USE: u8 = 0x39;
pub const OP_APPEARING: u8 = 0x3a;
pub const OP_REQUEST_SHORT_CUT_REG: u8 = 0x3d;
pub const OP_REQUEST_SHORT_CUT_DEL: u8 = 0x3f;
pub const OP_CANNOT_MOVE_ANYMORE: u8 = 0x47;
pub const OP_REQUEST_TARGET_CANCEL: u8 = 0x48;
pub const OP_SAY2_CLIENT: u8 = 0x49;
pub const OP_REQUEST_SKILL_LIST: u8 = 0x50;
pub const OP_REQUEST_ACTION_USE: u8 = 0x56;
pub const OP_VALIDATE_POSITION: u8 = 0x59;
pub const OP_REQUEST_DESTROY_ITEM: u8 = 0x60;
pub const OP_REQUEST_QUEST_LIST: u8 = 0x62;
pub const OP_REQUEST_SKILL_COOL_TIME: u8 = 0xa6;
pub const OP_REQUEST_GIVE_NICK_NAME: u8 = 0x0b;
pub const OP_REQUEST_SOCIAL_ACTION: u8 = 0x34;
pub const OP_REQUEST_MAGIC_SKILL_LIST: u8 = 0x38;
pub const OP_REQUEST_DISMISS_PARTY: u8 = 0x46;
pub const OP_REQUEST_GET_ON_VEHICLE: u8 = 0x53;
pub const OP_REQUEST_GET_OFF_VEHICLE: u8 = 0x54;
pub const OP_START_ROTATING_CLIENT: u8 = 0x5b;
pub const OP_FINISH_ROTATING_CLIENT: u8 = 0x5c;
pub const OP_REQUEST_SHOW_MINIMAP: u8 = 0x6c;
pub const OP_REQUEST_ACQUIRE_SKILL_INFO: u8 = 0x73;
pub const OP_SEND_BYPASS_BUILD_CMD: u8 = 0x74;
pub const OP_REQUEST_MOVE_TO_IN_VEHICLE: u8 = 0x75;
pub const OP_CANNOT_MOVE_ANYMORE_IN_VEHICLE: u8 = 0x76;
pub const OP_REQUEST_ACQUIRE_SKILL: u8 = 0x7c;
pub const OP_REQUEST_RESTART_POINT: u8 = 0x7d;
pub const OP_REQUEST_GM_COMMAND: u8 = 0x7e;
pub const OP_REQUEST_GM_LIST: u8 = 0x8b;
pub const OP_SEND_TIME_CHECK: u8 = 0xa0;
pub const OP_REQUEST_BLOCK: u8 = 0xa9;
pub const OP_REQUEST_REMAIN_TIME: u8 = 0xb2;
pub const OP_REQUEST_USER_COMMAND: u8 = 0xb3;
pub const OP_GM_SNOOP_END: u8 = 0xb4;
pub const OP_DLG_ANSWER: u8 = 0xc6;
pub const OP_REQUEST_MAKE_MACRO: u8 = 0xcd;
pub const OP_REQUEST_DELETE_MACRO: u8 = 0xce;

pub const ACTION_SIT_STAND: i32 = 0;
pub const ACTION_WALK_RUN: i32 = 1;

pub const SHORTCUT_OWNER_PLAYER: i32 = 1;

pub const EX_REQUEST_MANOR_LIST: u16 = 0x01;
pub const EX_REQUEST_WRITE_HERO_WORDS: u16 = 0x05;
pub const EX_REQUEST_ASK_JOIN_MPCC: u16 = 0x06;
pub const EX_REQUEST_ACCEPT_JOIN_MPCC: u16 = 0x07;
pub const EX_REQUEST_OUST_FROM_MPCC: u16 = 0x08;
pub const EX_REQUEST_AUTO_SOUL_SHOT: u16 = 0x0d;
pub const EX_REQUEST_ENCHANT_SKILL_INFO: u16 = 0x0e;
pub const EX_REQUEST_ENCHANT_SKILL: u16 = 0x0f;
pub const EX_REQUEST_PLEDGE_CREST_LARGE: u16 = 0x10;
pub const EX_REQUEST_SET_PLEDGE_CREST_LARGE: u16 = 0x11;
pub const EX_REQUEST_OLYMPIAD_OBSERVER_END: u16 = 0x29;

pub const USER_INFO_PAPERDOLL_SLOTS: usize = 26;
pub const CHAR_INFO_PAPERDOLL_SLOTS: usize = 21;
pub const INVENTORY_ITEM_BYTES: usize = 68;

pub fn enter_world() -> Vec<u8> {
    vec![OP_ENTER_WORLD]
}

pub fn request_restart() -> Vec<u8> {
    vec![OP_REQUEST_RESTART]
}

pub fn game_guard_reply(keys: [i32; 4]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_GAME_GUARD_REPLY);
    for key in keys {
        w.write_d(key);
    }
    w.into_bytes()
}

pub fn extended(sub: u16) -> Writer {
    let mut w = Writer::new();
    w.write_c(super::high_five::OP_EXTENDED);
    w.write_h(sub);
    w
}

fn bare(opcode: u8) -> Vec<u8> {
    vec![opcode]
}

fn opcode_ints(opcode: u8, values: &[i32]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(opcode);
    for value in values {
        w.write_d(*value);
    }
    w.into_bytes()
}

pub fn request_give_nick_name(target: &str, title: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_GIVE_NICK_NAME);
    w.write_s(target);
    w.write_s(title);
    w.into_bytes()
}

pub fn request_social_action(action: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_SOCIAL_ACTION, action)
}

pub fn request_magic_skill_list(first: i32, second: i32, third: i32) -> Vec<u8> {
    opcode_ints(OP_REQUEST_MAGIC_SKILL_LIST, &[first, second, third])
}

pub fn request_dismiss_party() -> Vec<u8> {
    bare(OP_REQUEST_DISMISS_PARTY)
}

pub fn request_get_on_vehicle(vehicle_id: i32, position: [i32; 3]) -> Vec<u8> {
    opcode_ints(OP_REQUEST_GET_ON_VEHICLE, &[vehicle_id, position[0], position[1], position[2]])
}

pub fn request_get_off_vehicle(vehicle_id: i32, position: [i32; 3]) -> Vec<u8> {
    opcode_ints(OP_REQUEST_GET_OFF_VEHICLE, &[vehicle_id, position[0], position[1], position[2]])
}

pub fn start_rotating(degree: i32, side: i32) -> Vec<u8> {
    opcode_ints(OP_START_ROTATING_CLIENT, &[degree, side])
}

pub fn finish_rotating(degree: i32) -> Vec<u8> {
    opcode_int(OP_FINISH_ROTATING_CLIENT, degree)
}

pub fn request_show_minimap() -> Vec<u8> {
    bare(OP_REQUEST_SHOW_MINIMAP)
}

pub fn request_acquire_skill_info(skill_id: i32, level: i32, mode: i32) -> Vec<u8> {
    opcode_ints(OP_REQUEST_ACQUIRE_SKILL_INFO, &[skill_id, level, mode])
}

pub fn send_bypass_build_cmd(command: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_SEND_BYPASS_BUILD_CMD);
    w.write_s(command);
    w.into_bytes()
}

pub fn request_move_to_in_vehicle(vehicle_id: i32, target: [i32; 3], origin: [i32; 3]) -> Vec<u8> {
    opcode_ints(
        OP_REQUEST_MOVE_TO_IN_VEHICLE,
        &[vehicle_id, target[0], target[1], target[2], origin[0], origin[1], origin[2]],
    )
}

pub fn cannot_move_anymore_in_vehicle(vehicle_id: i32, position: [i32; 3], heading: i32) -> Vec<u8> {
    opcode_ints(
        OP_CANNOT_MOVE_ANYMORE_IN_VEHICLE,
        &[vehicle_id, position[0], position[1], position[2], heading],
    )
}

pub fn request_acquire_skill(skill_id: i32, level: i32, mode: i32) -> Vec<u8> {
    opcode_ints(OP_REQUEST_ACQUIRE_SKILL, &[skill_id, level, mode])
}

pub fn request_restart_point(point: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_RESTART_POINT, point)
}

pub fn request_gm_command(name: &str, kind: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_GM_COMMAND);
    w.write_s(name);
    w.write_d(kind);
    w.into_bytes()
}

pub fn request_gm_list() -> Vec<u8> {
    bare(OP_REQUEST_GM_LIST)
}

pub fn send_time_check(first: i32, second: i32) -> Vec<u8> {
    opcode_ints(OP_SEND_TIME_CHECK, &[first, second])
}

pub fn request_block(kind: i32, name: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_BLOCK);
    w.write_d(kind);
    if kind == BLOCK_ADD || kind == BLOCK_REMOVE {
        w.write_s(name);
    }
    w.into_bytes()
}

pub fn request_remain_time() -> Vec<u8> {
    bare(OP_REQUEST_REMAIN_TIME)
}

pub fn request_user_command(command: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_USER_COMMAND, command)
}

pub fn request_gm_snoop_end(snoop_id: i32) -> Vec<u8> {
    opcode_int(OP_GM_SNOOP_END, snoop_id)
}

pub fn dlg_answer(message_id: i32, answer: i32, requester_id: i32) -> Vec<u8> {
    opcode_ints(OP_DLG_ANSWER, &[message_id, answer, requester_id])
}

pub fn request_make_macro(entry: &Macro) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_MAKE_MACRO);
    w.write_d(entry.id);
    w.write_s(&entry.name);
    w.write_s(&entry.description);
    w.write_s(&entry.acronym);
    w.write_c(entry.icon);
    w.write_c(entry.commands.len() as u8);
    for command in &entry.commands {
        w.write_c(command.index);
        w.write_c(command.kind);
        w.write_d(command.skill_id);
        w.write_c(command.shortcut_id);
        w.write_s(&command.command);
    }
    w.into_bytes()
}

pub fn request_delete_macro(id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_DELETE_MACRO, id)
}

fn id_ctrl_shift(opcode: u8, id: i32, ctrl: bool, shift: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(opcode);
    w.write_d(id);
    w.write_d(ctrl as i32);
    w.write_c(shift as u8);
    w.into_bytes()
}

fn object_origin_shift(opcode: u8, object_id: i32, origin: [i32; 3], shift: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(opcode);
    w.write_d(object_id);
    for v in origin {
        w.write_d(v);
    }
    w.write_c(shift as u8);
    w.into_bytes()
}

pub fn move_backward_to_location(target: [i32; 3], origin: [i32; 3]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_MOVE_BACKWARD_TO_LOCATION);
    for v in target.into_iter().chain(origin) {
        w.write_d(v);
    }
    w.write_d(MOVEMENT_MODE_MOUSE);
    w.into_bytes()
}

pub fn action(object_id: i32, origin: [i32; 3], shift: bool) -> Vec<u8> {
    object_origin_shift(OP_ACTION, object_id, origin, shift)
}

pub fn attack_request(object_id: i32, origin: [i32; 3], shift: bool) -> Vec<u8> {
    object_origin_shift(OP_ATTACK_REQUEST, object_id, origin, shift)
}

pub fn use_item(object_id: i32, ctrl: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_USE_ITEM);
    w.write_d(object_id);
    w.write_d(ctrl as i32);
    w.into_bytes()
}

pub fn request_magic_skill_use(skill_id: i32, ctrl: bool, shift: bool) -> Vec<u8> {
    id_ctrl_shift(OP_REQUEST_MAGIC_SKILL_USE, skill_id, ctrl, shift)
}

pub fn request_action_use(action_id: i32, ctrl: bool, shift: bool) -> Vec<u8> {
    id_ctrl_shift(OP_REQUEST_ACTION_USE, action_id, ctrl, shift)
}

pub fn say2(text: &str, kind: i32, target: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_SAY2_CLIENT);
    w.write_s(text);
    w.write_d(kind);
    if kind == CHAT_TELL {
        w.write_s(target);
    }
    w.into_bytes()
}

pub fn validate_position(position: [i32; 3], heading: i32, vehicle_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_VALIDATE_POSITION);
    for v in position {
        w.write_d(v);
    }
    w.write_d(heading);
    w.write_d(vehicle_id);
    w.into_bytes()
}

pub fn cannot_move_anymore(position: [i32; 3], heading: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_CANNOT_MOVE_ANYMORE);
    for v in position {
        w.write_d(v);
    }
    w.write_d(heading);
    w.into_bytes()
}

pub fn request_item_list() -> Vec<u8> {
    bare(OP_REQUEST_ITEM_LIST)
}

pub fn request_skill_list() -> Vec<u8> {
    bare(OP_REQUEST_SKILL_LIST)
}

pub fn request_quest_list() -> Vec<u8> {
    bare(OP_REQUEST_QUEST_LIST)
}

pub fn request_skill_cool_time() -> Vec<u8> {
    bare(OP_REQUEST_SKILL_COOL_TIME)
}

pub fn appearing() -> Vec<u8> {
    bare(OP_APPEARING)
}

pub fn request_target_cancel(unselect: u16) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_TARGET_CANCEL);
    w.write_h(unselect);
    w.into_bytes()
}

pub fn request_unequip_item(slot: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_UNEQUIP_ITEM);
    w.write_d(slot);
    w.into_bytes()
}

pub fn request_drop_item(object_id: i32, count: i64, position: [i32; 3]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_DROP_ITEM);
    w.write_d(object_id);
    w.write_q(count);
    for v in position {
        w.write_d(v);
    }
    w.into_bytes()
}

pub fn request_destroy_item(object_id: i32, count: i64) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_DESTROY_ITEM);
    w.write_d(object_id);
    w.write_q(count);
    w.into_bytes()
}

pub fn request_bypass_to_server(command: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_BYPASS_TO_SERVER);
    w.write_s(command);
    w.into_bytes()
}

pub fn request_link_html(link: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_LINK_HTML);
    w.write_s(link);
    w.into_bytes()
}

pub fn request_shortcut_reg(kind: i32, slot: i32, id: i32, level_or_owner: i32) -> Vec<u8> {
    let (level, owner) = if kind == SHORTCUT_SKILL { (level_or_owner, SHORTCUT_OWNER_PLAYER) } else { (0, level_or_owner) };
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_SHORT_CUT_REG);
    w.write_d(kind);
    w.write_d(slot);
    w.write_d(id);
    w.write_d(level);
    w.write_d(owner);
    w.into_bytes()
}

pub fn request_shortcut_del(slot: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_SHORT_CUT_DEL);
    w.write_d(slot);
    w.into_bytes()
}

pub fn request_manor_list() -> Vec<u8> {
    extended(EX_REQUEST_MANOR_LIST).into_bytes()
}

pub fn request_write_hero_words(words: &str) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_WRITE_HERO_WORDS);
    w.write_s(words);
    w.into_bytes()
}

pub fn request_ex_ask_join_mpcc(name: &str) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_ASK_JOIN_MPCC);
    w.write_s(name);
    w.into_bytes()
}

pub fn request_ex_accept_join_mpcc(accept: bool) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_ACCEPT_JOIN_MPCC);
    w.write_d(accept as i32);
    w.into_bytes()
}

pub fn request_ex_oust_from_mpcc(name: &str) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_OUST_FROM_MPCC);
    w.write_s(name);
    w.into_bytes()
}

pub fn request_auto_soul_shot(item_id: i32, enable: bool) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_AUTO_SOUL_SHOT);
    w.write_d(item_id);
    w.write_d(enable as i32);
    w.into_bytes()
}

pub fn request_ex_enchant_skill_info(skill_id: i32, level: i32) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_ENCHANT_SKILL_INFO);
    w.write_d(skill_id);
    w.write_d(level);
    w.into_bytes()
}

pub fn request_ex_enchant_skill(skill_id: i32, level: i32) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_ENCHANT_SKILL);
    w.write_d(skill_id);
    w.write_d(level);
    w.into_bytes()
}

pub fn request_ex_pledge_crest_large(crest_id: i32) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_PLEDGE_CREST_LARGE);
    w.write_d(crest_id);
    w.into_bytes()
}

pub fn request_ex_set_pledge_crest_large(data: &[u8]) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_SET_PLEDGE_CREST_LARGE);
    w.write_d(data.len() as i32);
    w.write_b(data);
    w.into_bytes()
}

pub fn request_olympiad_observer_end() -> Vec<u8> {
    extended(EX_REQUEST_OLYMPIAD_OBSERVER_END).into_bytes()
}

pub const EX_REQUEST_CHANGE_PARTY_LEADER: u16 = 0x0c;
pub const EX_REQUEST_PARTY_LOOT_MODIFICATION: u16 = 0x78;
pub const EX_ANSWER_PARTY_LOOT_MODIFICATION: u16 = 0x79;

pub fn request_change_party_leader(name: &str) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_CHANGE_PARTY_LEADER);
    w.write_s(name);
    w.into_bytes()
}

pub fn request_party_loot_modification(loot_type: i32) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_PARTY_LOOT_MODIFICATION);
    w.write_d(loot_type);
    w.into_bytes()
}

pub fn answer_party_loot_modification(accept: bool) -> Vec<u8> {
    let mut w = extended(EX_ANSWER_PARTY_LOOT_MODIFICATION);
    w.write_d(accept as i32);
    w.into_bytes()
}

fn clamp_count(value: i64) -> i32 {
    i32::try_from(value).unwrap_or(if value < 0 { i32::MIN } else { i32::MAX })
}

fn skip_d(r: &mut Reader, n: usize) -> Result<()> {
    for _ in 0..n {
        r.read_d()?;
    }
    Ok(())
}

fn read_paperdoll_block(r: &mut Reader, slots: usize) -> Result<Vec<i32>> {
    let mut values = Vec::with_capacity(slots);
    for _ in 0..slots {
        values.push(r.read_d()?);
    }
    Ok(values)
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserInfoExtrasHighFive {
    pub vehicle_id: i32,
    pub exp: i64,
    pub exp_percent: f64,
    pub weapon_flag: i32,
    pub paperdoll_objects: Vec<i32>,
    pub paperdoll_items: Vec<i32>,
    pub paperdoll_augmentations: Vec<i32>,
    pub talisman_slots: i32,
    pub can_equip_cloak: bool,
    pub clan_privileges: i32,
    pub mount_npc_id: i32,
    pub active_class_id: i32,
    pub team: u8,
    pub large_crest_id: i32,
    pub running: bool,
    pub pledge_class: i32,
    pub pledge_type: i32,
    pub title_color: i32,
    pub cursed_weapon_level: i32,
    pub transformation: i32,
    pub elements: [u16; 8],
    pub agathion: i32,
    pub fame: i32,
    pub minimap_allowed: bool,
    pub vitality: i32,
    pub abnormal_special: i32,
}

pub fn read_user_info(body: &[u8]) -> Result<(UserInfoHead, UserInfoExtrasHighFive)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let x = r.read_d()?;
    let y = r.read_d()?;
    let z = r.read_d()?;
    let vehicle_id = r.read_d()?;
    let object_id = r.read_d()?;
    let name = r.read_s()?;
    let race = r.read_d()?;
    let sex = r.read_d()?;
    let class_id = r.read_d()?;
    let level = r.read_d()?;
    let exp = r.read_q()?;
    let exp_percent = r.read_f()?;
    let str_ = r.read_d()?;
    let dex = r.read_d()?;
    let con = r.read_d()?;
    let int_ = r.read_d()?;
    let wit = r.read_d()?;
    let men = r.read_d()?;
    let max_hp = r.read_d()?;
    let hp = r.read_d()?;
    let max_mp = r.read_d()?;
    let mp = r.read_d()?;
    let sp = r.read_d()?;
    let load = r.read_d()?;
    let max_load = r.read_d()?;
    let weapon_flag = r.read_d()?;
    let paperdoll_objects = read_paperdoll_block(&mut r, USER_INFO_PAPERDOLL_SLOTS)?;
    let paperdoll_items = read_paperdoll_block(&mut r, USER_INFO_PAPERDOLL_SLOTS)?;
    let paperdoll_augmentations = read_paperdoll_block(&mut r, USER_INFO_PAPERDOLL_SLOTS)?;
    let talisman_slots = r.read_d()?;
    let can_equip_cloak = r.read_d()? != 0;
    let p_atk = r.read_d()?;
    let atk_spd = r.read_d()?;
    let p_def = r.read_d()?;
    let evasion = r.read_d()?;
    let accuracy = r.read_d()?;
    let critical = r.read_d()?;
    let m_atk = r.read_d()?;
    let cast_spd = r.read_d()?;
    r.read_d()?;
    let m_def = r.read_d()?;
    let pvp_flag = r.read_d()?;
    let karma = r.read_d()?;
    let run_speed = r.read_d()?;
    let walk_speed = r.read_d()?;
    skip_d(&mut r, 6)?;
    let speed_multiplier = r.read_f()?;
    let attack_speed_multiplier = r.read_f()?;
    let collision_radius = r.read_f()?;
    let collision_height = r.read_f()?;
    let hair_style = r.read_d()?;
    let hair_color = r.read_d()?;
    let face = r.read_d()?;
    let builder_level = r.read_d()?;
    let title = r.read_s()?;
    let clan_id = r.read_d()?;
    let clan_crest_id = r.read_d()?;
    let ally_id = r.read_d()?;
    let ally_crest_id = r.read_d()?;
    let relation = r.read_d()?;
    let mount_type = r.read_c()?;
    let private_store = r.read_c()?;
    let dwarven_craft = r.read_c()? != 0;
    let pk_kills = r.read_d()?;
    let pvp_kills = r.read_d()?;
    let cubic_count = r.read_h()?;
    let mut cubic_ids = Vec::new();
    for _ in 0..cubic_count {
        cubic_ids.push(r.read_h()?);
    }
    r.read_c()?;
    let abnormal_effect = r.read_d()?;
    r.read_c()?;
    let clan_privileges = r.read_d()?;
    let recom_left = r.read_h()?;
    let recom_have = r.read_h()?;
    let mount_npc_id = r.read_d()?;
    let inventory_limit = r.read_h()? as i32;
    let active_class_id = r.read_d()?;
    r.read_d()?;
    let max_cp = r.read_d()?;
    let cp = r.read_d()?;
    let enchant_effect = r.read_c()?;
    let team = r.read_c()?;
    let large_crest_id = r.read_d()?;
    let noble = r.read_c()? != 0;
    let hero = r.read_c()? != 0;
    r.read_c()?;
    skip_d(&mut r, 3)?;
    let name_color = r.read_d()?;
    let running = r.read_c()? != 0;
    let pledge_class = r.read_d()?;
    let pledge_type = r.read_d()?;
    let title_color = r.read_d()?;
    let cursed_weapon_level = r.read_d()?;
    let transformation = r.read_d()?;
    let mut elements = [0u16; 8];
    for value in &mut elements {
        *value = r.read_h()?;
    }
    let agathion = r.read_d()?;
    let fame = r.read_d()?;
    let minimap_allowed = r.read_d()? != 0;
    let vitality = r.read_d()?;
    let abnormal_special = r.read_d()?;
    let mut paperdoll = [PaperdollSlot::default(); PAPERDOLL_SLOTS];
    for (index, slot) in paperdoll.iter_mut().enumerate() {
        slot.object_id = paperdoll_objects[index];
        slot.item_id = paperdoll_items[index];
    }
    let social = UserSocial {
        clan_id,
        clan_crest_id,
        ally_id,
        ally_crest_id,
        mount_type,
        private_store,
        dwarven_craft,
        pk_kills,
        pvp_kills,
        cubic_ids,
        recom_left,
        recom_have,
        noble,
        hero,
        relation,
        pledge_rights: [false; 4],
    };
    let stats = UserStats {
        str_,
        dex,
        con,
        int_,
        wit,
        men,
        sp,
        load,
        max_load,
        attack_range: weapon_flag,
        p_atk,
        atk_spd,
        p_def,
        evasion,
        accuracy,
        critical,
        m_atk,
        cast_spd,
        m_def,
    };
    let head = UserInfoHead {
        abnormal_effect,
        x,
        y,
        z,
        heading: 0,
        object_id,
        name,
        race,
        sex,
        class_id,
        level,
        exp: clamp_count(exp),
        max_hp,
        hp,
        max_mp,
        mp,
        pvp_flag,
        karma,
        run_speed,
        walk_speed,
        speed_multiplier,
        attack_speed_multiplier,
        collision_radius,
        collision_height,
        hair_style,
        hair_color,
        face,
        builder_level,
        title,
        max_cp,
        cp,
        stats,
        paperdoll,
        inventory_limit,
        name_color,
        social,
        enchant_effect,
    };
    let extras = UserInfoExtrasHighFive {
        vehicle_id,
        exp,
        exp_percent,
        weapon_flag,
        paperdoll_objects,
        paperdoll_items,
        paperdoll_augmentations,
        talisman_slots,
        can_equip_cloak,
        clan_privileges,
        mount_npc_id,
        active_class_id,
        team,
        large_crest_id,
        running,
        pledge_class,
        pledge_type,
        title_color,
        cursed_weapon_level,
        transformation,
        elements,
        agathion,
        fame,
        minimap_allowed,
        vitality,
        abnormal_special,
    };
    Ok((head, extras))
}

#[derive(Debug, Clone, PartialEq)]
pub struct CharInfoExtrasHighFive {
    pub vehicle_id: i32,
    pub paperdoll_items: Vec<i32>,
    pub paperdoll_augmentations: Vec<i32>,
    pub talisman_slots: i32,
    pub can_equip_cloak: bool,
    pub party_room: u8,
    pub pledge_class: i32,
    pub pledge_type: i32,
    pub title_color: i32,
    pub cursed_weapon_level: i32,
    pub clan_reputation: i32,
    pub transformation: i32,
    pub agathion: i32,
    pub abnormal_special: i32,
}

pub fn read_char_info(body: &[u8]) -> Result<(CharInfo, CharInfoExtrasHighFive)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let x = r.read_d()?;
    let y = r.read_d()?;
    let z = r.read_d()?;
    let vehicle_id = r.read_d()?;
    let object_id = r.read_d()?;
    let name = r.read_s()?;
    let race = r.read_d()?;
    let sex = r.read_d()?;
    let class_id = r.read_d()?;
    let paperdoll_items = read_paperdoll_block(&mut r, CHAR_INFO_PAPERDOLL_SLOTS)?;
    let paperdoll_augmentations = read_paperdoll_block(&mut r, CHAR_INFO_PAPERDOLL_SLOTS)?;
    let talisman_slots = r.read_d()?;
    let can_equip_cloak = r.read_d()? != 0;
    let pvp_flag = r.read_d()?;
    let karma = r.read_d()?;
    let m_atk_speed = r.read_d()?;
    let p_atk_speed = r.read_d()?;
    r.read_d()?;
    let run_speed = r.read_d()?;
    let walk_speed = r.read_d()?;
    let swim_run_speed = r.read_d()?;
    let swim_walk_speed = r.read_d()?;
    let ground_run_speed = r.read_d()?;
    let ground_walk_speed = r.read_d()?;
    let fly_run_speed = r.read_d()?;
    let fly_walk_speed = r.read_d()?;
    let speed_multiplier = r.read_f()?;
    let attack_speed_multiplier = r.read_f()?;
    let collision_radius = r.read_f()?;
    let collision_height = r.read_f()?;
    let hair_style = r.read_d()?;
    let hair_color = r.read_d()?;
    let face = r.read_d()?;
    let title = r.read_s()?;
    let clan_id = r.read_d()?;
    let clan_crest_id = r.read_d()?;
    let ally_id = r.read_d()?;
    let ally_crest_id = r.read_d()?;
    let sitting = r.read_c()? == WAIT_TYPE_SIT;
    let running = r.read_c()? == 1;
    let in_combat = r.read_c()? == 1;
    let alike_dead = r.read_c()? == 1;
    let invisible = r.read_c()? == 1;
    let mount_type = r.read_c()?;
    let store_type = r.read_c()?;
    let count = r.read_h()? as usize;
    let mut cubics = Vec::with_capacity(count.min(16));
    for _ in 0..count {
        cubics.push(r.read_h()?);
    }
    let party_room = r.read_c()?;
    let abnormal_effect = r.read_d()?;
    r.read_c()?;
    let recommendations_received = r.read_h()? as i32;
    let mount_npc_id = r.read_d()?;
    let active_class_id = r.read_d()?;
    r.read_d()?;
    let enchant_effect = r.read_c()?;
    let team = r.read_c()?;
    let large_crest_id = r.read_d()?;
    let noble = r.read_c()?;
    let hero = r.read_c()?;
    let fishing = r.read_c()?;
    let fish_x = r.read_d()?;
    let fish_y = r.read_d()?;
    let fish_z = r.read_d()?;
    let name_color = r.read_d()?;
    let heading = r.read_d()?;
    let pledge_class = r.read_d()?;
    let pledge_type = r.read_d()?;
    let title_color = r.read_d()?;
    let cursed_weapon_level = r.read_d()?;
    let clan_reputation = r.read_d()?;
    let transformation = r.read_d()?;
    let agathion = r.read_d()?;
    r.read_d()?;
    let abnormal_special = r.read_d()?;
    let mut equipped = [0i32; VISIBLE_EQUIP_SLOTS];
    equipped.copy_from_slice(&paperdoll_items[..VISIBLE_EQUIP_SLOTS]);
    let info = CharInfo {
        x,
        y,
        z,
        heading,
        object_id,
        name,
        race,
        sex,
        class_id,
        equipped,
        pvp_flag,
        karma,
        m_atk_speed,
        p_atk_speed,
        run_speed,
        walk_speed,
        swim_run_speed,
        swim_walk_speed,
        ground_run_speed,
        ground_walk_speed,
        fly_run_speed,
        fly_walk_speed,
        speed_multiplier,
        attack_speed_multiplier,
        collision_radius,
        collision_height,
        hair_style,
        hair_color,
        face,
        title,
        clan_id,
        clan_crest_id,
        ally_id,
        ally_crest_id,
        sitting,
        running,
        in_combat,
        alike_dead,
        invisible,
        mount_type,
        store_type,
        cubics,
        finding_party: party_room,
        abnormal_effect,
        recommendations_left: 0,
        recommendations_received,
        mount_npc_id,
        active_class_id,
        enchant_effect,
        team,
        large_crest_id,
        noble,
        hero,
        fishing,
        fish_x,
        fish_y,
        fish_z,
        name_color,
    };
    let extras = CharInfoExtrasHighFive {
        vehicle_id,
        paperdoll_items,
        paperdoll_augmentations,
        talisman_slots,
        can_equip_cloak,
        party_room,
        pledge_class,
        pledge_type,
        title_color,
        cursed_weapon_level,
        clan_reputation,
        transformation,
        agathion,
        abnormal_special,
    };
    Ok((info, extras))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NpcInfoExtrasHighFive {
    pub name_string: i32,
    pub title_string: i32,
    pub enchant_effect: i32,
    pub flying: i32,
    pub color_effect: i32,
    pub targetable: bool,
    pub show_name: bool,
    pub abnormal_special: i32,
    pub display_effect: i32,
}

pub fn read_npc_info(body: &[u8]) -> Result<(NpcInfo, NpcInfoExtrasHighFive)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let object_id = r.read_d()?;
    let npc_id = r.read_d()?.wrapping_sub(NPC_TYPE_OFFSET);
    let attackable = r.read_d()? == 1;
    let x = r.read_d()?;
    let y = r.read_d()?;
    let z = r.read_d()?;
    let heading = r.read_d()?;
    r.read_d()?;
    let m_atk_speed = r.read_d()?;
    let p_atk_speed = r.read_d()?;
    let run_speed = r.read_d()?;
    let walk_speed = r.read_d()?;
    skip_d(&mut r, 6)?;
    let speed_multiplier = r.read_f()?;
    let attack_speed_multiplier = r.read_f()?;
    let collision_radius = r.read_f()?;
    let collision_height = r.read_f()?;
    let rhand = r.read_d()?;
    let chest = r.read_d()?;
    let lhand = r.read_d()?;
    let name_above = r.read_c()? == 1;
    let running = r.read_c()? == 1;
    let in_combat = r.read_c()? == 1;
    let alike_dead = r.read_c()? == 1;
    let spawn_animation = r.read_c()?;
    let name_string = r.read_d()?;
    let name = r.read_s()?;
    let title_string = r.read_d()?;
    let title = r.read_s()?;
    let playable_flag = r.read_d()?;
    let pvp_flag = r.read_d()?;
    let karma = r.read_d()?;
    let abnormal_effect = r.read_d()?;
    let clan_id = r.read_d()?;
    let clan_crest_id = r.read_d()?;
    let ally_id = r.read_d()?;
    let ally_crest_id = r.read_d()?;
    r.read_c()?;
    let team = r.read_c()?;
    let base_collision_radius = r.read_f()?;
    let base_collision_height = r.read_f()?;
    let enchant_effect = r.read_d()?;
    let flying = r.read_d()?;
    r.read_d()?;
    let color_effect = r.read_d()?;
    let targetable = r.read_c()? != 0;
    let show_name = r.read_c()? != 0;
    let abnormal_special = r.read_d()?;
    let display_effect = r.read_d()?;
    let info = NpcInfo {
        object_id,
        npc_id,
        attackable,
        x,
        y,
        z,
        heading,
        m_atk_speed,
        p_atk_speed,
        run_speed,
        walk_speed,
        speed_multiplier,
        attack_speed_multiplier,
        collision_radius,
        collision_height,
        rhand,
        chest,
        lhand,
        name_above,
        running,
        in_combat,
        alike_dead,
        spawn_animation,
        name,
        title,
        playable_flag,
        pvp_flag,
        karma,
        abnormal_effect,
        clan_id,
        clan_crest_id,
        ally_id,
        ally_crest_id,
        team,
        base_collision_radius,
        base_collision_height,
    };
    let extras = NpcInfoExtrasHighFive {
        name_string,
        title_string,
        enchant_effect,
        flying,
        color_effect,
        targetable,
        show_name,
        abnormal_special,
        display_effect,
    };
    Ok((info, extras))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ItemExtrasHighFive {
    pub location: i32,
    pub augmentation: i32,
    pub mana: i32,
    pub remaining_seconds: i32,
    pub attack_element: [u16; 2],
    pub defence_elements: [u16; 6],
    pub enchant_options: [u16; 3],
    pub count: i64,
}

fn read_item(r: &mut Reader) -> Result<(InventoryItem, ItemExtrasHighFive)> {
    let object_id = r.read_d()?;
    let item_id = r.read_d()?;
    let location = r.read_d()?;
    let count = r.read_q()?;
    let type2 = r.read_h()? as i16;
    let custom_type1 = r.read_h()? as i16;
    let equipped = r.read_h()? != 0;
    let body_part = r.read_d()?;
    let enchant = r.read_h()? as i16;
    let custom_type2 = r.read_h()? as i16;
    let augmentation = r.read_d()?;
    let mana = r.read_d()?;
    let remaining_seconds = r.read_d()?;
    let attack_element = [r.read_h()?, r.read_h()?];
    let mut defence_elements = [0u16; 6];
    for value in &mut defence_elements {
        *value = r.read_h()?;
    }
    let mut enchant_options = [0u16; 3];
    for value in &mut enchant_options {
        *value = r.read_h()?;
    }
    let item = InventoryItem {
        type1: 0,
        object_id,
        item_id,
        count: clamp_count(count),
        type2,
        custom_type1,
        equipped,
        body_part,
        enchant,
        custom_type2,
    };
    let extras = ItemExtrasHighFive {
        location,
        augmentation,
        mana,
        remaining_seconds,
        attack_element,
        defence_elements,
        enchant_options,
        count,
    };
    Ok((item, extras))
}

pub fn read_item_list(body: &[u8]) -> Result<(ItemList, Vec<ItemExtrasHighFive>)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let show_window = r.read_h()? != 0;
    let declared = r.read_h()?;
    let count = counted(&r, declared as i32, INVENTORY_ITEM_BYTES)?;
    let mut items = Vec::with_capacity(count);
    let mut extras = Vec::with_capacity(count);
    for _ in 0..count {
        let (item, extra) = read_item(&mut r)?;
        items.push(item);
        extras.push(extra);
    }
    Ok((ItemList { show_window, items }, extras))
}

pub fn read_inventory_update(body: &[u8]) -> Result<(InventoryUpdate, Vec<ItemExtrasHighFive>)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let declared = r.read_h()?;
    let count = counted(&r, declared as i32, 2 + INVENTORY_ITEM_BYTES)?;
    let mut changes = Vec::with_capacity(count);
    let mut extras = Vec::with_capacity(count);
    for _ in 0..count {
        let change = r.read_h()?;
        let (item, extra) = read_item(&mut r)?;
        changes.push(ItemChange { change, item });
        extras.push(extra);
    }
    Ok((InventoryUpdate { changes }, extras))
}

pub fn read_skill_list(body: &[u8]) -> Result<(SkillList, Vec<(bool, bool)>)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let declared = r.read_d()?;
    let count = counted(&r, declared, 14)?;
    let mut skills = Vec::with_capacity(count);
    let mut flags = Vec::with_capacity(count);
    for _ in 0..count {
        let passive = r.read_d()? != 0;
        let level = r.read_d()?;
        let id = r.read_d()?;
        let disabled = r.read_c()? != 0;
        let enchanted = r.read_c()? != 0;
        skills.push(SkillEntry { passive, level, id });
        flags.push((disabled, enchanted));
    }
    Ok((SkillList { skills }, flags))
}

pub const ARGUMENT_BIG_NUMBER: i32 = 6;
pub const ARGUMENT_PLACE_NAME: i32 = 7;
pub const ARGUMENT_PLAYER_NAME: i32 = 12;

pub fn read_system_message(body: &[u8]) -> Result<SystemMessage> {
    use super::packets::{
        ARGUMENT_CASTLE_NAME, ARGUMENT_ITEM_NAME, ARGUMENT_NPC_NAME, ARGUMENT_NUMBER, ARGUMENT_SKILL_NAME,
        ARGUMENT_TEXT, SYSTEM_MESSAGE_TEXT_MAX_CHARS,
    };
    let mut r = Reader::new(body);
    r.read_c()?;
    let id = r.read_d()?;
    let declared = r.read_d()?;
    let count = counted(&r, declared, 8)?;
    let mut args = Vec::with_capacity(count);
    for _ in 0..count {
        let kind = r.read_d()?;
        let arg = match kind {
            ARGUMENT_TEXT | ARGUMENT_PLAYER_NAME => {
                let text = r.read_s()?;
                if text.encode_utf16().count() > SYSTEM_MESSAGE_TEXT_MAX_CHARS {
                    return Err(WireError::TooLong);
                }
                SystemMessageArg::Text(text)
            }
            ARGUMENT_NUMBER => SystemMessageArg::Number(r.read_d()?),
            ARGUMENT_NPC_NAME => SystemMessageArg::NpcName(r.read_d()?),
            ARGUMENT_ITEM_NAME => SystemMessageArg::ItemName(r.read_d()?),
            ARGUMENT_CASTLE_NAME => SystemMessageArg::CastleName(r.read_d()?),
            ARGUMENT_SKILL_NAME => {
                let id = r.read_d()?;
                let level = r.read_d()?;
                SystemMessageArg::SkillName { id, level }
            }
            ARGUMENT_BIG_NUMBER => {
                let value = r.read_q()?;
                match i32::try_from(value) {
                    Ok(number) => SystemMessageArg::Number(number),
                    Err(_) => SystemMessageArg::Text(value.to_string()),
                }
            }
            ARGUMENT_PLACE_NAME => {
                skip_d(&mut r, 3)?;
                SystemMessageArg::Unknown { kind }
            }
            _ => {
                r.read_d()?;
                SystemMessageArg::Unknown { kind }
            }
        };
        args.push(arg);
    }
    Ok(SystemMessage { id, args })
}

pub fn read_creature_say(body: &[u8]) -> Result<(Say2, i32)> {
    use super::packets::{SAY2_NAME_MAX_CHARS, SAY2_TEXT_MAX_CHARS};
    let mut r = Reader::new(body);
    r.read_c()?;
    let object_id = r.read_d()?;
    let kind = r.read_d()?;
    if kind == CHAT_BOAT {
        let sys_string_id = r.read_d()?;
        let sys_msg_id = r.read_d()?;
        return Ok((Say2 { object_id, kind, body: Say2Body::Tables { sys_string_id, sys_msg_id } }, sys_msg_id));
    }
    let name = r.read_s()?;
    let npc_string = r.read_d()?;
    let mut parts = Vec::new();
    while r.remaining() >= 2 {
        parts.push(r.read_s()?);
    }
    let text = parts.join(" ");
    if name.encode_utf16().count() > SAY2_NAME_MAX_CHARS || text.encode_utf16().count() > SAY2_TEXT_MAX_CHARS {
        return Err(WireError::TooLong);
    }
    Ok((Say2 { object_id, kind, body: Say2Body::Text { name, text } }, npc_string))
}

pub fn read_npc_say(body: &[u8]) -> Result<(NpcSay, i32)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let object_id = r.read_d()?;
    let kind = r.read_d()?;
    let npc_id = r.read_d()?.wrapping_sub(NPC_TYPE_OFFSET);
    let npc_string = r.read_d()?;
    let mut parts = Vec::new();
    while r.remaining() >= 2 {
        parts.push(r.read_s()?);
    }
    Ok((NpcSay { object_id, kind, npc_id, text: parts.join(" ") }, npc_string))
}

pub fn read_spawn_item(body: &[u8]) -> Result<SpawnItem> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let object_id = r.read_d()?;
    let item_id = r.read_d()?;
    let x = r.read_d()?;
    let y = r.read_d()?;
    let z = r.read_d()?;
    let stackable = r.read_d()? != 0;
    let count = clamp_count(r.read_q()?);
    r.read_d()?;
    r.read_d()?;
    Ok(SpawnItem { object_id, item_id, x, y, z, stackable, count, heading: 0 })
}

pub fn read_drop_item(body: &[u8]) -> Result<DropItem> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let player_id = r.read_d()?;
    let object_id = r.read_d()?;
    let item_id = r.read_d()?;
    let x = r.read_d()?;
    let y = r.read_d()?;
    let z = r.read_d()?;
    let stackable = r.read_d()? != 0;
    let count = clamp_count(r.read_q()?);
    r.read_d()?;
    Ok(DropItem { player_id, object_id, item_id, x, y, z, stackable, count, heading: 0 })
}

pub fn read_magic_skill_use(body: &[u8]) -> Result<MagicSkillUse> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let caster_id = r.read_d()?;
    let target_id = r.read_d()?;
    let skill_id = r.read_d()?;
    let skill_level = r.read_d()?;
    let hit_time = r.read_d()?;
    let reuse_delay = r.read_d()?;
    let x = r.read_d()?;
    let y = r.read_d()?;
    let z = r.read_d()?;
    r.read_h()?;
    let count = r.read_h()? as usize;
    let mut positions = Vec::with_capacity(count.min(16));
    for _ in 0..count {
        positions.push([r.read_d()?, r.read_d()?, r.read_d()?]);
    }
    let _ = read_d_or_zero(&mut r)?;
    Ok(MagicSkillUse {
        caster_id,
        target_id,
        skill_id,
        skill_level,
        hit_time,
        reuse_delay,
        x,
        y,
        z,
        extra: Vec::new(),
        positions,
    })
}

pub fn read_etc_status_update(body: &[u8]) -> Result<EtcStatusUpdate> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let charges = r.read_d()?;
    let weight_penalty = r.read_d()?;
    let chat_blocked = r.read_d()?;
    let danger_zone = r.read_d()?;
    let weapon_penalty = r.read_d()?;
    let armor_penalty = r.read_d()?;
    let mut values = [0i32; ETC_STATUS_VALUES];
    values[0] = charges;
    values[1] = weight_penalty;
    values[2] = chat_blocked;
    values[3] = danger_zone;
    values[4] = (weapon_penalty != 0 || armor_penalty != 0) as i32;
    Ok(EtcStatusUpdate { values })
}

fn read_shortcut(r: &mut Reader) -> Result<Shortcut> {
    let kind = r.read_d()?;
    let slot = r.read_d()?;
    let (kind, character_type) = match kind {
        SHORTCUT_ITEM => {
            let object_id = r.read_d()?;
            let character_type = r.read_d()?;
            skip_d(r, 4)?;
            (ShortcutKind::Item { object_id }, character_type)
        }
        SHORTCUT_SKILL => {
            let skill_id = r.read_d()?;
            let level = r.read_d()?;
            r.read_c()?;
            (ShortcutKind::Skill { skill_id, level }, r.read_d()?)
        }
        SHORTCUT_ACTION | SHORTCUT_MACRO | SHORTCUT_RECIPE => {
            let id = r.read_d()?;
            let character_type = r.read_d()?;
            let kind = match kind {
                SHORTCUT_ACTION => ShortcutKind::Action { id },
                SHORTCUT_MACRO => ShortcutKind::Macro { id },
                _ => ShortcutKind::Recipe { id },
            };
            (kind, character_type)
        }
        other => {
            skip_d(r, 2)?;
            (ShortcutKind::Other { kind: other }, 0)
        }
    };
    Ok(Shortcut { slot, kind, character_type })
}

pub fn read_short_cut_init(body: &[u8]) -> Result<ShortCutInit> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let declared = r.read_d()?;
    let count = counted(&r, declared, 16)?;
    let mut shortcuts = Vec::with_capacity(count);
    for _ in 0..count {
        shortcuts.push(read_shortcut(&mut r)?);
    }
    Ok(ShortCutInit { shortcuts })
}

pub fn read_short_cut_register(body: &[u8]) -> Result<Shortcut> {
    let mut r = Reader::new(body);
    r.read_c()?;
    read_shortcut(&mut r)
}

pub fn read_storage_max_count(body: &[u8]) -> Result<(StorageMaxCount, [i32; 2])> {
    let mut r = Reader::new(body);
    r.read_c()?;
    r.read_h()?;
    let mut values = [0i32; 7];
    for value in &mut values {
        *value = r.read_d()?;
    }
    let extra = [read_d_or_zero(&mut r)?, read_d_or_zero(&mut r)?];
    Ok((StorageMaxCount { values }, extra))
}

pub fn read_basic_action_list(body: &[u8]) -> Result<Vec<i32>> {
    let mut r = Reader::new(body);
    r.read_c()?;
    r.read_h()?;
    let declared = r.read_d()?;
    let count = counted(&r, declared, 4)?;
    let mut actions = Vec::with_capacity(count);
    for _ in 0..count {
        actions.push(r.read_d()?);
    }
    Ok(actions)
}

pub fn read_rotation(body: &[u8]) -> Result<(i32, i32)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    r.read_h()?;
    let object_id = r.read_d()?;
    let heading = r.read_d()?;
    Ok((object_id, heading))
}

pub fn read_abnormal_status_update(body: &[u8]) -> Result<super::packets::MagicEffectIcons> {
    super::packets::read_magic_effect_icons(body)
}

pub fn read_attack(body: &[u8]) -> Result<(super::packets::Attack, [i32; 3])> {
    let attack = super::packets::read_attack(body)?;
    let mut r = Reader::new(body);
    r.read_c()?;
    skip_d(&mut r, 2)?;
    r.read_d()?;
    r.read_c()?;
    skip_d(&mut r, 3)?;
    let count = r.read_h()? as usize;
    for _ in 0..count {
        r.read_d()?;
        r.read_d()?;
        r.read_c()?;
    }
    let target_position = [read_d_or_zero(&mut r)?, read_d_or_zero(&mut r)?, read_d_or_zero(&mut r)?];
    Ok((attack, target_position))
}

pub const OP_REQUEST_START_PLEDGE_WAR: u8 = 0x03;
pub const OP_REQUEST_STOP_PLEDGE_WAR: u8 = 0x05;
pub const OP_REQUEST_SURRENDER_PLEDGE_WAR: u8 = 0x07;
pub const OP_REQUEST_SET_PLEDGE_CREST: u8 = 0x09;
pub const OP_REQUEST_JOIN_PLEDGE: u8 = 0x26;
pub const OP_REQUEST_ANSWER_JOIN_PLEDGE: u8 = 0x27;
pub const OP_REQUEST_WITHDRAWAL_PLEDGE: u8 = 0x28;
pub const OP_REQUEST_OUST_PLEDGE_MEMBER: u8 = 0x29;
pub const OP_REQUEST_ALLY_INFO: u8 = 0x2e;
pub const OP_TRADE_REQUEST_CLIENT: u8 = 0x1a;
pub const OP_ADD_TRADE_ITEM: u8 = 0x1b;
pub const OP_TRADE_DONE_CLIENT: u8 = 0x1c;
pub const OP_REQUEST_SELL_ITEM: u8 = 0x37;
pub const OP_REQUEST_BUY_ITEM: u8 = 0x40;
pub const OP_REQUEST_JOIN_PARTY: u8 = 0x42;
pub const OP_REQUEST_ANSWER_JOIN_PARTY: u8 = 0x43;
pub const OP_REQUEST_WITHDRAWAL_PARTY: u8 = 0x44;
pub const OP_REQUEST_OUST_PARTY_MEMBER: u8 = 0x45;
pub const OP_REQUEST_PLEDGE_MEMBER_LIST: u8 = 0x4d;
pub const OP_ANSWER_TRADE_REQUEST: u8 = 0x55;
pub const OP_REQUEST_QUEST_ABORT: u8 = 0x63;
pub const OP_REQUEST_PLEDGE_INFO: u8 = 0x65;
pub const OP_REQUEST_PLEDGE_CREST: u8 = 0x67;
pub const OP_REQUEST_SEND_FRIEND_MESSAGE: u8 = 0x6b;
pub const OP_REQUEST_FRIEND_INVITE: u8 = 0x77;
pub const OP_REQUEST_ANSWER_FRIEND_INVITE: u8 = 0x78;
pub const OP_REQUEST_FRIEND_LIST: u8 = 0x79;
pub const OP_REQUEST_FRIEND_DELETE: u8 = 0x7a;
pub const OP_REQUEST_JOIN_ALLY: u8 = 0x8c;
pub const OP_REQUEST_ANSWER_JOIN_ALLY: u8 = 0x8d;
pub const OP_ALLY_LEAVE: u8 = 0x8e;
pub const OP_ALLY_DISMISS: u8 = 0x8f;
pub const OP_REQUEST_DISMISS_ALLY: u8 = 0x90;
pub const OP_REQUEST_SET_ALLY_CREST: u8 = 0x91;
pub const OP_REQUEST_ALLY_CREST: u8 = 0x92;
pub const OP_REQUEST_NET_PING: u8 = 0xb1;

pub const OP_TRADE_START: u8 = 0x14;
pub const OP_TRADE_OWN_ADD: u8 = 0x1a;
pub const OP_TRADE_OTHER_ADD: u8 = 0x1b;
pub const OP_TRADE_DONE: u8 = 0x1c;
pub const OP_SEND_TRADE_REQUEST: u8 = 0x70;
pub const OP_ASK_JOIN_PARTY: u8 = 0x39;
pub const OP_JOIN_PARTY: u8 = 0x3a;
pub const OP_PARTY_SMALL_WINDOW_DELETE_ALL: u8 = 0x50;
pub const OP_PARTY_SMALL_WINDOW_ALL: u8 = 0x4e;
pub const OP_PARTY_SMALL_WINDOW_ADD: u8 = 0x4f;
pub const OP_PARTY_SMALL_WINDOW_DELETE: u8 = 0x51;
pub const OP_PARTY_SMALL_WINDOW_UPDATE: u8 = 0x52;
pub const OP_PARTY_MEMBER_POSITION: u8 = 0xba;
pub const OP_ASK_JOIN_PLEDGE: u8 = 0x2c;
pub const OP_JOIN_PLEDGE: u8 = 0x2d;
pub const OP_PLEDGE_INFO: u8 = 0x89;
pub const OP_PLEDGE_CREST: u8 = 0x6a;
pub const OP_ALLY_CREST: u8 = 0xaf;
pub const OP_PLEDGE_STATUS_CHANGED: u8 = 0xcd;
pub const OP_PLEDGE_SHOW_MEMBER_LIST_ALL: u8 = 0x5a;
pub const OP_PLEDGE_SHOW_MEMBER_LIST_UPDATE: u8 = 0x5b;
pub const OP_PLEDGE_SHOW_MEMBER_LIST_ADD: u8 = 0x5c;
pub const OP_PLEDGE_SHOW_MEMBER_LIST_DELETE: u8 = 0x5d;
pub const OP_PLEDGE_SHOW_MEMBER_LIST_DELETE_ALL: u8 = 0x88;
pub const OP_PLEDGE_SHOW_INFO_UPDATE: u8 = 0x8e;
pub const OP_FRIEND_LIST: u8 = 0x75;
pub const OP_FRIEND_ADD_REQUEST: u8 = 0x83;
pub const OP_FRIEND_PACKET: u8 = 0x76;
pub const OP_FRIEND_STATUS: u8 = 0x77;
pub const OP_L2_FRIEND_SAY: u8 = 0x78;
pub const OP_RELATION_CHANGED: u8 = 0xce;
pub const OP_NET_PING: u8 = 0xd9;

pub const EX_VOTE_SYSTEM_INFO: u16 = 0xc9;
pub const EX_UI_SETTING: u16 = 0x70;

const FRIEND_OPERATION_ADD: i32 = 1;
const FRIEND_OPERATION_REMOVE: i32 = 3;

pub fn request_sell_item(list_id: i32, orders: &[ItemOrder]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_SELL_ITEM);
    w.write_d(list_id);
    w.write_d(orders.len() as i32);
    for order in orders {
        w.write_d(order.object_id);
        w.write_d(order.item_id);
        w.write_q(i64::from(order.count));
    }
    w.into_bytes()
}

pub fn request_buy_item(list_id: i32, orders: &[(i32, i32)]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_BUY_ITEM);
    w.write_d(list_id);
    w.write_d(orders.len() as i32);
    for (item_id, count) in orders {
        w.write_d(*item_id);
        w.write_q(i64::from(*count));
    }
    w.into_bytes()
}

pub fn request_trade(object_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_TRADE_REQUEST_CLIENT);
    w.write_d(object_id);
    w.into_bytes()
}

pub fn answer_trade_request(accept: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_ANSWER_TRADE_REQUEST);
    w.write_d(accept as i32);
    w.into_bytes()
}

pub fn add_trade_item(object_id: i32, count: i64) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_ADD_TRADE_ITEM);
    w.write_d(0);
    w.write_d(object_id);
    w.write_q(count);
    w.into_bytes()
}

pub fn request_trade_done(confirm: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_TRADE_DONE_CLIENT);
    w.write_d(confirm as i32);
    w.into_bytes()
}

pub fn request_join_party(name: &str, item_distribution: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_JOIN_PARTY);
    w.write_s(name);
    w.write_d(item_distribution);
    w.into_bytes()
}

pub fn request_answer_join_party(accept: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_ANSWER_JOIN_PARTY);
    w.write_d(accept as i32);
    w.into_bytes()
}

pub fn request_withdrawal_party() -> Vec<u8> {
    bare(OP_REQUEST_WITHDRAWAL_PARTY)
}

pub fn request_oust_party_member(name: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_OUST_PARTY_MEMBER);
    w.write_s(name);
    w.into_bytes()
}

pub fn request_join_pledge(object_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_JOIN_PLEDGE);
    w.write_d(object_id);
    w.write_d(0);
    w.into_bytes()
}

pub fn request_answer_join_pledge(accept: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_ANSWER_JOIN_PLEDGE);
    w.write_d(accept as i32);
    w.into_bytes()
}

pub fn request_withdrawal_pledge() -> Vec<u8> {
    bare(OP_REQUEST_WITHDRAWAL_PLEDGE)
}

pub fn request_oust_pledge_member(name: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_OUST_PLEDGE_MEMBER);
    w.write_s(name);
    w.into_bytes()
}

pub fn request_set_pledge_crest(data: &[u8]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_SET_PLEDGE_CREST);
    w.write_d(data.len() as i32);
    w.write_b(data);
    w.into_bytes()
}

pub fn request_set_ally_crest(data: &[u8]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_SET_ALLY_CREST);
    w.write_d(data.len() as i32);
    w.write_b(data);
    w.into_bytes()
}

pub fn request_pledge_crest(crest_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_PLEDGE_CREST);
    w.write_d(crest_id);
    w.into_bytes()
}

pub fn request_ally_crest(crest_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_ALLY_CREST);
    w.write_d(crest_id);
    w.into_bytes()
}

pub fn request_pledge_member_list() -> Vec<u8> {
    bare(OP_REQUEST_PLEDGE_MEMBER_LIST)
}

pub fn request_pledge_info(pledge_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_PLEDGE_INFO);
    w.write_d(pledge_id);
    w.into_bytes()
}

pub fn request_join_ally(object_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_JOIN_ALLY);
    w.write_d(object_id);
    w.into_bytes()
}

pub fn request_answer_join_ally(accept: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_ANSWER_JOIN_ALLY);
    w.write_d(accept as i32);
    w.into_bytes()
}

pub fn ally_leave() -> Vec<u8> {
    bare(OP_ALLY_LEAVE)
}

pub fn ally_dismiss(pledge_name: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_ALLY_DISMISS);
    w.write_s(pledge_name);
    w.into_bytes()
}

pub fn request_dismiss_ally() -> Vec<u8> {
    bare(OP_REQUEST_DISMISS_ALLY)
}

pub fn request_ally_info() -> Vec<u8> {
    bare(OP_REQUEST_ALLY_INFO)
}

pub fn request_friend_invite(name: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_FRIEND_INVITE);
    w.write_s(name);
    w.into_bytes()
}

pub fn request_answer_friend_invite(accept: bool) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_ANSWER_FRIEND_INVITE);
    w.write_d(accept as i32);
    w.into_bytes()
}

pub fn request_friend_list() -> Vec<u8> {
    bare(OP_REQUEST_FRIEND_LIST)
}

pub fn request_friend_delete(name: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_FRIEND_DELETE);
    w.write_s(name);
    w.into_bytes()
}

pub fn request_send_friend_message(message: &str, receiver: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_SEND_FRIEND_MESSAGE);
    w.write_s(message);
    w.write_s(receiver);
    w.into_bytes()
}

pub fn request_start_pledge_war(pledge_name: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_START_PLEDGE_WAR);
    w.write_s(pledge_name);
    w.into_bytes()
}

pub fn request_stop_pledge_war(pledge_name: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_STOP_PLEDGE_WAR);
    w.write_s(pledge_name);
    w.into_bytes()
}

pub fn request_surrender_pledge_war(pledge_name: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_SURRENDER_PLEDGE_WAR);
    w.write_s(pledge_name);
    w.into_bytes()
}

pub fn request_quest_abort(quest_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_QUEST_ABORT);
    w.write_d(quest_id);
    w.into_bytes()
}

pub fn request_net_ping() -> Vec<u8> {
    bare(OP_REQUEST_NET_PING)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TradeItemExtrasHighFive {
    pub location: i32,
    pub augmentation: i32,
    pub mana: i32,
    pub remaining_seconds: i32,
    pub attack_element: [u16; 2],
    pub defence_elements: [u16; 6],
    pub enchant_options: [u16; 3],
    pub count: i64,
}

fn trade_item_from_inventory(item: InventoryItem, extra: ItemExtrasHighFive) -> (TradeItem, TradeItemExtrasHighFive) {
    let trade = TradeItem {
        type1: 0,
        object_id: item.object_id,
        item_id: item.item_id,
        count: item.count,
        type2: item.type2,
        custom_type1: item.custom_type1,
        body_part: item.body_part,
        enchant: item.enchant,
        custom_type2: item.custom_type2,
    };
    let extras = TradeItemExtrasHighFive {
        location: extra.location,
        augmentation: extra.augmentation,
        mana: extra.mana,
        remaining_seconds: extra.remaining_seconds,
        attack_element: extra.attack_element,
        defence_elements: extra.defence_elements,
        enchant_options: extra.enchant_options,
        count: extra.count,
    };
    (trade, extras)
}

pub fn read_trade_start(body: &[u8]) -> Result<(TradeStart, Vec<TradeItemExtrasHighFive>)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let partner_id = r.read_d()?;
    let declared = r.read_h()?;
    let count = counted(&r, declared as i32, INVENTORY_ITEM_BYTES)?;
    let mut items = Vec::with_capacity(count);
    let mut extras = Vec::with_capacity(count);
    for _ in 0..count {
        let (item, extra) = read_item(&mut r)?;
        let (trade, trade_extra) = trade_item_from_inventory(item, extra);
        items.push(trade);
        extras.push(trade_extra);
    }
    Ok((TradeStart { partner_id, items }, extras))
}

fn read_trade_add_item(r: &mut Reader) -> Result<(TradeItem, TradeItemExtrasHighFive)> {
    r.read_h()?;
    r.read_h()?;
    let object_id = r.read_d()?;
    let item_id = r.read_d()?;
    let count = r.read_q()?;
    let type2 = r.read_h()? as i16;
    let type1 = r.read_h()? as i16;
    let body_part = r.read_d()?;
    let enchant = r.read_h()? as i16;
    r.read_h()?;
    let custom_type2 = r.read_h()? as i16;
    let augmentation = r.read_d()?;
    let mana = r.read_d()?;
    let remaining_seconds = r.read_d()?;
    let attack_element = [r.read_h()?, r.read_h()?];
    let mut defence_elements = [0u16; 6];
    for value in &mut defence_elements {
        *value = r.read_h()?;
    }
    let mut enchant_options = [0u16; 3];
    for value in &mut enchant_options {
        *value = r.read_h()?;
    }
    let item = TradeItem {
        type1,
        object_id,
        item_id,
        count: clamp_count(count),
        type2,
        custom_type1: 0,
        body_part,
        enchant,
        custom_type2,
    };
    let extras = TradeItemExtrasHighFive {
        location: 0,
        augmentation,
        mana,
        remaining_seconds,
        attack_element,
        defence_elements,
        enchant_options,
        count,
    };
    Ok((item, extras))
}

pub fn read_trade_own_add(body: &[u8]) -> Result<(TradeAdd, TradeItemExtrasHighFive)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let (item, extra) = read_trade_add_item(&mut r)?;
    Ok((TradeAdd { items: vec![item] }, extra))
}

pub fn read_trade_other_add(body: &[u8]) -> Result<(TradeAdd, TradeItemExtrasHighFive)> {
    read_trade_own_add(body)
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PartyMemberPetHighFive {
    pub object_id: i32,
    pub info_id: i32,
    pub summon_type: i32,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub mp: i32,
    pub max_mp: i32,
    pub level: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PartyMemberExtrasHighFive {
    pub race: i32,
    pub pet: Option<PartyMemberPetHighFive>,
}

fn read_party_member_high_five(r: &mut Reader) -> Result<PartyMember> {
    let object_id = r.read_d()?;
    let name = r.read_s()?;
    let cp = r.read_d()?;
    let max_cp = r.read_d()?;
    let hp = r.read_d()?;
    let max_hp = r.read_d()?;
    let mp = r.read_d()?;
    let max_mp = r.read_d()?;
    let level = r.read_d()?;
    let class_id = r.read_d()?;
    Ok(PartyMember { object_id, name, cp, max_cp, hp, max_hp, mp, max_mp, level, class_id })
}

fn read_party_member_tail_high_five(r: &mut Reader) -> Result<PartyMemberExtrasHighFive> {
    r.read_d()?;
    let race = r.read_d()?;
    r.read_d()?;
    r.read_d()?;
    let pet_marker = r.read_d()?;
    if pet_marker == 0 {
        return Ok(PartyMemberExtrasHighFive { race, pet: None });
    }
    let info_id = r.read_d()?;
    let summon_type = r.read_d()?;
    let name = r.read_s()?;
    let hp = r.read_d()?;
    let max_hp = r.read_d()?;
    let mp = r.read_d()?;
    let max_mp = r.read_d()?;
    let level = r.read_d()?;
    Ok(PartyMemberExtrasHighFive {
        race,
        pet: Some(PartyMemberPetHighFive { object_id: pet_marker, info_id, summon_type, name, hp, max_hp, mp, max_mp, level }),
    })
}

const PARTY_MEMBER_MIN_BYTES: usize = 10 * 4 + 2 + 4 * 4;

pub fn read_party_small_window_all(body: &[u8]) -> Result<(PartySmallWindowAll, Vec<PartyMemberExtrasHighFive>)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let leader_id = r.read_d()?;
    let loot_distribution = r.read_d()?;
    let declared = r.read_d()?;
    let count = counted(&r, declared, PARTY_MEMBER_MIN_BYTES)?;
    let mut members = Vec::with_capacity(count);
    let mut extras = Vec::with_capacity(count);
    for _ in 0..count {
        members.push(read_party_member_high_five(&mut r)?);
        extras.push(read_party_member_tail_high_five(&mut r)?);
    }
    Ok((PartySmallWindowAll { leader_id, loot_distribution, members }, extras))
}

pub fn read_party_small_window_add(body: &[u8]) -> Result<(PartySmallWindowAdd, PartyMemberExtrasHighFive)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let leader_id = r.read_d()?;
    let loot_distribution = r.read_d()?;
    let member = read_party_member_high_five(&mut r)?;
    let extras = read_party_member_tail_high_five(&mut r)?;
    Ok((PartySmallWindowAdd { leader_id, loot_distribution, member }, extras))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PledgeMemberExtrasHighFive {
    pub pledge_type: i32,
    pub has_sponsor: bool,
}

fn read_pledge_member_core(r: &mut Reader) -> Result<(String, i32, i32, i32, i32)> {
    let name = r.read_s()?;
    let level = r.read_d()?;
    let class_id = r.read_d()?;
    let sex = r.read_d()?;
    let race = r.read_d()?;
    Ok((name, level, class_id, sex, race))
}

pub fn read_pledge_show_member_list_add(body: &[u8]) -> Result<(PledgeMember, PledgeMemberExtrasHighFive)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let name = r.read_s()?;
    let level = r.read_d()?;
    let class_id = r.read_d()?;
    r.read_d()?;
    r.read_d()?;
    let online_object_id = r.read_d()?;
    let pledge_type = r.read_d()?;
    Ok((
        PledgeMember { name, level, class_id, unknown: 0, flag: 0, online_object_id },
        PledgeMemberExtrasHighFive { pledge_type, has_sponsor: false },
    ))
}

pub fn read_pledge_show_member_list_update(body: &[u8]) -> Result<(PledgeMember, PledgeMemberExtrasHighFive)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let (name, level, class_id, sex, race) = read_pledge_member_core(&mut r)?;
    let online_object_id = r.read_d()?;
    let pledge_type = r.read_d()?;
    let has_sponsor = r.read_d()? != 0;
    Ok((
        PledgeMember { name, level, class_id, unknown: sex, flag: race, online_object_id },
        PledgeMemberExtrasHighFive { pledge_type, has_sponsor },
    ))
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PledgeShowMemberListAllExtrasHighFive {
    pub fort_id: i32,
    pub reputation_rank: i32,
    pub reputation_score: i32,
    pub members: Vec<PledgeMemberExtrasHighFive>,
}

pub fn read_pledge_show_member_list_all(
    body: &[u8],
) -> Result<(PledgeShowMemberListAll, PledgeShowMemberListAllExtrasHighFive)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    r.read_d()?;
    let pledge_id = r.read_d()?;
    r.read_d()?;
    let name = r.read_s()?;
    let leader_name = r.read_s()?;
    let crest_id = r.read_d()?;
    let level = r.read_d()?;
    let castle_id = r.read_d()?;
    let hideout_id = r.read_d()?;
    let fort_id = r.read_d()?;
    let reputation_rank = r.read_d()?;
    let reputation_score = r.read_d()?;
    r.read_d()?;
    r.read_d()?;
    let ally_id = r.read_d()?;
    let ally_name = r.read_s()?;
    let ally_crest_id = r.read_d()?;
    let at_war = r.read_d()? != 0;
    let declared = r.read_d()?;
    let count = counted(&r, declared, 2 + 5 * 4)?;
    let mut members = Vec::with_capacity(count);
    let mut member_extras = Vec::with_capacity(count);
    for _ in 0..count {
        let (name, level, class_id, sex, race) = read_pledge_member_core(&mut r)?;
        let online_object_id = r.read_d()?;
        let sponsor = r.read_d()? != 0;
        members.push(PledgeMember { name, level, class_id, unknown: sex, flag: race, online_object_id });
        member_extras.push(PledgeMemberExtrasHighFive { pledge_type: 0, has_sponsor: sponsor });
    }
    Ok((
        PledgeShowMemberListAll {
            pledge_id,
            name,
            leader_name,
            crest_id,
            level,
            castle_id,
            hideout_id,
            my_level: 0,
            dissolving: 0,
            ally_id,
            ally_name,
            ally_crest_id,
            at_war,
            members,
        },
        PledgeShowMemberListAllExtrasHighFive { fort_id, reputation_rank, reputation_score, members: member_extras },
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PledgeShowInfoUpdateExtrasHighFive {
    pub fort_id: i32,
    pub reputation_rank: i32,
    pub reputation_score: i32,
}

pub fn read_pledge_show_info_update(body: &[u8]) -> Result<(PledgeShowInfoUpdate, PledgeShowInfoUpdateExtrasHighFive)> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let pledge_id = r.read_d()?;
    let crest_id = r.read_d()?;
    let level = r.read_d()?;
    let castle_id = r.read_d()?;
    let hideout_id = r.read_d()?;
    let fort_id = r.read_d()?;
    let reputation_rank = r.read_d()?;
    let reputation_score = r.read_d()?;
    r.read_d()?;
    r.read_d()?;
    let ally_id = r.read_d()?;
    let ally_name = r.read_s()?;
    let ally_crest_id = r.read_d()?;
    let at_war = r.read_d()? != 0;
    Ok((
        PledgeShowInfoUpdate {
            pledge_id,
            crest_id,
            level,
            castle_id,
            hideout_id,
            leader_level: 0,
            dissolving: 0,
            ally_id,
            ally_name,
            ally_crest_id,
            at_war,
        },
        PledgeShowInfoUpdateExtrasHighFive { fort_id, reputation_rank, reputation_score },
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FriendPacketHighFive {
    Added,
    Removed,
    Unknown { operation: i32 },
}

pub fn read_friend_packet(body: &[u8]) -> Result<FriendPacketHighFive> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let operation = r.read_d()?;
    r.read_d()?;
    r.read_s()?;
    r.read_d()?;
    r.read_d()?;
    Ok(match operation {
        FRIEND_OPERATION_ADD => FriendPacketHighFive::Added,
        FRIEND_OPERATION_REMOVE => FriendPacketHighFive::Removed,
        other => FriendPacketHighFive::Unknown { operation: other },
    })
}

pub fn read_relation_changed(body: &[u8]) -> Result<Vec<RelationChanged>> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let declared = r.read_d()?;
    let count = counted(&r, declared, 20)?;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        let object_id = r.read_d()?;
        let relation = r.read_d()?;
        let auto_attack = r.read_d()?;
        let karma = r.read_d()?;
        let pvp_flag = r.read_d()?;
        entries.push(RelationChanged { object_id, relation, auto_attack, karma, pvp_flag });
    }
    Ok(entries)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct VoteSystemInfoHighFive {
    pub recommendations_left: i32,
    pub recommendations_have: i32,
    pub bonus_time: i32,
    pub bonus_value: i32,
    pub bonus_type: i32,
}

pub fn read_vote_system_info(body: &[u8]) -> Result<VoteSystemInfoHighFive> {
    let mut r = Reader::new(body);
    r.read_c()?;
    r.read_h()?;
    let recommendations_left = r.read_d()?;
    let recommendations_have = r.read_d()?;
    let bonus_time = r.read_d()?;
    let bonus_value = r.read_d()?;
    let bonus_type = r.read_d()?;
    Ok(VoteSystemInfoHighFive { recommendations_left, recommendations_have, bonus_time, bonus_value, bonus_type })
}

pub fn read_ui_setting(body: &[u8]) -> Result<()> {
    let mut r = Reader::new(body);
    r.read_c()?;
    r.read_h()?;
    r.read_d()?;
    r.read_d()?;
    let total_keys = r.read_d()?;
    for _ in 0..total_keys {
        for _ in 0..2 {
            let n = r.read_c()?;
            for _ in 0..n {
                r.read_c()?;
            }
        }
        let n = r.read_d()?;
        for _ in 0..n {
            skip_d(&mut r, 5)?;
        }
    }
    r.read_d()?;
    r.read_d()?;
    Ok(())
}

pub const OP_REQUEST_GET_ITEM_FROM_PET: u8 = 0x2c;
pub const OP_REQUEST_CRYSTALLIZE_ITEM: u8 = 0x2f;
pub const OP_REQUEST_PRIVATE_STORE_MANAGE_SELL: u8 = 0x30;
pub const OP_SET_PRIVATE_STORE_LIST_SELL: u8 = 0x31;
pub const OP_SEND_WAREHOUSE_DEPOSIT_LIST: u8 = 0x3b;
pub const OP_SEND_WAREHOUSE_WITHDRAW_LIST: u8 = 0x3c;
pub const OP_REQUEST_ENCHANT_ITEM: u8 = 0x5f;
pub const OP_REQUEST_HENNA_EQUIP: u8 = 0x6f;
pub const OP_REQUEST_HENNA_UNEQUIP_LIST: u8 = 0x70;
pub const OP_REQUEST_HENNA_UNEQUIP_INFO: u8 = 0x71;
pub const OP_REQUEST_HENNA_UNEQUIP: u8 = 0x72;
pub const OP_REQUEST_PRIVATE_STORE_BUY: u8 = 0x83;
pub const OP_REQUEST_CHANGE_PET_NAME: u8 = 0x93;
pub const OP_REQUEST_PET_USE_ITEM: u8 = 0x94;
pub const OP_REQUEST_GIVE_ITEM_TO_PET: u8 = 0x95;
pub const OP_REQUEST_PRIVATE_STORE_QUIT_SELL: u8 = 0x96;
pub const OP_SET_PRIVATE_STORE_MSG_SELL: u8 = 0x97;
pub const OP_REQUEST_PET_GET_ITEM: u8 = 0x98;
pub const OP_REQUEST_PRIVATE_STORE_MANAGE_BUY: u8 = 0x99;
pub const OP_SET_PRIVATE_STORE_LIST_BUY: u8 = 0x9a;
pub const OP_REQUEST_PRIVATE_STORE_QUIT_BUY: u8 = 0x9c;
pub const OP_SET_PRIVATE_STORE_MSG_BUY: u8 = 0x9d;
pub const OP_REQUEST_PRIVATE_STORE_SELL: u8 = 0x9f;
pub const OP_REQUEST_PACKAGE_SENDABLE_ITEM_LIST: u8 = 0xa7;
pub const OP_REQUEST_PACKAGE_SEND: u8 = 0xa8;
pub const OP_MULTI_SELL_CHOOSE: u8 = 0xb0;
pub const OP_REQUEST_RECIPE_BOOK_OPEN: u8 = 0xb5;
pub const OP_REQUEST_RECIPE_ITEM_DELETE: u8 = 0xb6;
pub const OP_REQUEST_RECIPE_ITEM_MAKE_INFO: u8 = 0xb7;
pub const OP_REQUEST_RECIPE_ITEM_MAKE_SELF: u8 = 0xb8;
pub const OP_REQUEST_RECIPE_SHOP_MANAGE_LIST: u8 = 0xb9;
pub const OP_REQUEST_RECIPE_SHOP_MESSAGE_SET: u8 = 0xba;
pub const OP_REQUEST_RECIPE_SHOP_LIST_SET: u8 = 0xbb;
pub const OP_REQUEST_RECIPE_SHOP_MANAGE_QUIT: u8 = 0xbc;
pub const OP_REQUEST_RECIPE_SHOP_MANAGE_CANCEL: u8 = 0xbd;
pub const OP_REQUEST_RECIPE_SHOP_MAKE_INFO: u8 = 0xbe;
pub const OP_REQUEST_RECIPE_SHOP_MAKE_DO: u8 = 0xbf;
pub const OP_REQUEST_RECIPE_SHOP_SELL_LIST: u8 = 0xc0;
pub const OP_REQUEST_HENNA_LIST: u8 = 0xc3;
pub const OP_REQUEST_HENNA_ITEM_INFO: u8 = 0xc4;
pub const OP_REQUEST_BUY_SEED: u8 = 0xc5;
pub const OP_REQUEST_PREVIEW_ITEM: u8 = 0xc7;
pub const EX_REQUEST_PROCURE_CROP_LIST: u16 = 0x02;
pub const EX_REQUEST_SET_SEED: u16 = 0x03;
pub const EX_REQUEST_SET_CROP: u16 = 0x04;

fn opcode_int(opcode: u8, value: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(opcode);
    w.write_d(value);
    w.into_bytes()
}

fn opcode_string(opcode: u8, text: &str) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(opcode);
    w.write_s(text);
    w.into_bytes()
}

fn object_count_list(opcode: u8, orders: &[(i32, i32)]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(opcode);
    w.write_d(orders.len() as i32);
    for (object_id, count) in orders {
        w.write_d(*object_id);
        w.write_q(i64::from(*count));
    }
    w.into_bytes()
}

pub fn send_warehouse_deposit_list(orders: &[(i32, i32)]) -> Vec<u8> {
    object_count_list(OP_SEND_WAREHOUSE_DEPOSIT_LIST, orders)
}

pub fn send_warehouse_withdraw_list(orders: &[(i32, i32)]) -> Vec<u8> {
    object_count_list(OP_SEND_WAREHOUSE_WITHDRAW_LIST, orders)
}

pub fn request_package_sendable_item_list(target_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_PACKAGE_SENDABLE_ITEM_LIST, target_id)
}

pub fn request_package_send(target_id: i32, orders: &[(i32, i32)]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_PACKAGE_SEND);
    w.write_d(target_id);
    w.write_d(orders.len() as i32);
    for (object_id, count) in orders {
        w.write_d(*object_id);
        w.write_q(i64::from(*count));
    }
    w.into_bytes()
}

pub fn request_crystallize_item(object_id: i32, count: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_CRYSTALLIZE_ITEM);
    w.write_d(object_id);
    w.write_q(i64::from(count));
    w.into_bytes()
}

pub fn request_enchant_item(object_id: i32, support_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_ENCHANT_ITEM);
    w.write_d(object_id);
    w.write_d(support_id);
    w.into_bytes()
}

pub fn request_preview_item(header: i32, list_id: i32, items: &[i32]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_PREVIEW_ITEM);
    w.write_d(header);
    w.write_d(list_id);
    w.write_d(items.len() as i32);
    for item_id in items {
        w.write_d(*item_id);
    }
    w.into_bytes()
}

pub fn multi_sell_choose(list_id: i32, entry_id: i32, amount: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_MULTI_SELL_CHOOSE);
    w.write_d(list_id);
    w.write_d(entry_id);
    w.write_q(i64::from(amount));
    w.into_bytes()
}

pub fn request_private_store_manage_sell() -> Vec<u8> {
    bare(OP_REQUEST_PRIVATE_STORE_MANAGE_SELL)
}

pub fn request_private_store_manage_buy() -> Vec<u8> {
    bare(OP_REQUEST_PRIVATE_STORE_MANAGE_BUY)
}

pub fn request_private_store_quit_sell() -> Vec<u8> {
    bare(OP_REQUEST_PRIVATE_STORE_QUIT_SELL)
}

pub fn request_private_store_quit_buy() -> Vec<u8> {
    bare(OP_REQUEST_PRIVATE_STORE_QUIT_BUY)
}

pub fn set_private_store_msg_sell(message: &str) -> Vec<u8> {
    opcode_string(OP_SET_PRIVATE_STORE_MSG_SELL, message)
}

pub fn set_private_store_msg_buy(message: &str) -> Vec<u8> {
    opcode_string(OP_SET_PRIVATE_STORE_MSG_BUY, message)
}

pub fn set_private_store_list_sell(package_sale: bool, offers: &[StoreOffer]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_SET_PRIVATE_STORE_LIST_SELL);
    w.write_d(package_sale as i32);
    w.write_d(offers.len() as i32);
    for offer in offers {
        w.write_d(offer.object_id);
        w.write_q(i64::from(offer.count));
        w.write_q(i64::from(offer.price));
    }
    w.into_bytes()
}

pub fn set_private_store_list_buy(offers: &[BuyOffer]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_SET_PRIVATE_STORE_LIST_BUY);
    w.write_d(offers.len() as i32);
    for offer in offers {
        w.write_d(offer.item_id);
        w.write_d(i32::from(offer.enchant));
        w.write_q(i64::from(offer.count));
        w.write_q(i64::from(offer.price));
    }
    w.into_bytes()
}

pub fn request_private_store_buy(store_id: i32, orders: &[StoreBuyOrder]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_PRIVATE_STORE_BUY);
    w.write_d(store_id);
    w.write_d(orders.len() as i32);
    for order in orders {
        w.write_d(order.object_id);
        w.write_q(i64::from(order.count));
        w.write_q(i64::from(order.price));
    }
    w.into_bytes()
}

pub fn request_private_store_sell(store_id: i32, items: &[SellToStoreItem]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_PRIVATE_STORE_SELL);
    w.write_d(store_id);
    w.write_d(items.len() as i32);
    for item in items {
        w.write_d(item.object_id);
        w.write_d(item.item_id);
        w.write_d(i32::from(item.enchant));
        w.write_q(i64::from(item.count));
        w.write_q(i64::from(item.price));
    }
    w.into_bytes()
}

pub fn request_recipe_book_open(common: bool) -> Vec<u8> {
    opcode_int(OP_REQUEST_RECIPE_BOOK_OPEN, common as i32)
}

pub fn request_recipe_item_delete(recipe_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_RECIPE_ITEM_DELETE, recipe_id)
}

pub fn request_recipe_item_make_info(recipe_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_RECIPE_ITEM_MAKE_INFO, recipe_id)
}

pub fn request_recipe_item_make_self(recipe_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_RECIPE_ITEM_MAKE_SELF, recipe_id)
}

pub fn request_recipe_shop_manage_list() -> Vec<u8> {
    bare(OP_REQUEST_RECIPE_SHOP_MANAGE_LIST)
}

pub fn request_recipe_shop_manage_quit() -> Vec<u8> {
    bare(OP_REQUEST_RECIPE_SHOP_MANAGE_QUIT)
}

pub fn request_recipe_shop_manage_cancel() -> Vec<u8> {
    bare(OP_REQUEST_RECIPE_SHOP_MANAGE_CANCEL)
}

pub fn request_recipe_shop_message_set(message: &str) -> Vec<u8> {
    opcode_string(OP_REQUEST_RECIPE_SHOP_MESSAGE_SET, message)
}

pub fn request_recipe_shop_list_set(items: &[(i32, i32)]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_RECIPE_SHOP_LIST_SET);
    w.write_d(items.len() as i32);
    for (recipe_id, price) in items {
        w.write_d(*recipe_id);
        w.write_q(i64::from(*price));
    }
    w.into_bytes()
}

pub fn request_recipe_shop_make_info(manufacturer_id: i32, recipe_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_RECIPE_SHOP_MAKE_INFO);
    w.write_d(manufacturer_id);
    w.write_d(recipe_id);
    w.into_bytes()
}

pub fn request_recipe_shop_make_do(manufacturer_id: i32, recipe_id: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_RECIPE_SHOP_MAKE_DO);
    w.write_d(manufacturer_id);
    w.write_d(recipe_id);
    w.into_bytes()
}

pub fn request_recipe_shop_sell_list() -> Vec<u8> {
    bare(OP_REQUEST_RECIPE_SHOP_SELL_LIST)
}

pub fn request_henna_list() -> Vec<u8> {
    bare(OP_REQUEST_HENNA_LIST)
}

pub fn request_henna_item_info(symbol_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_HENNA_ITEM_INFO, symbol_id)
}

pub fn request_henna_equip(symbol_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_HENNA_EQUIP, symbol_id)
}

pub fn request_henna_unequip_list() -> Vec<u8> {
    bare(OP_REQUEST_HENNA_UNEQUIP_LIST)
}

pub fn request_henna_unequip_info(symbol_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_HENNA_UNEQUIP_INFO, symbol_id)
}

pub fn request_henna_unequip(symbol_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_HENNA_UNEQUIP, symbol_id)
}

pub fn request_change_pet_name(name: &str) -> Vec<u8> {
    opcode_string(OP_REQUEST_CHANGE_PET_NAME, name)
}

pub fn request_pet_use_item(object_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_PET_USE_ITEM, object_id)
}

pub fn request_pet_get_item(object_id: i32) -> Vec<u8> {
    opcode_int(OP_REQUEST_PET_GET_ITEM, object_id)
}

pub fn request_give_item_to_pet(object_id: i32, count: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_GIVE_ITEM_TO_PET);
    w.write_d(object_id);
    w.write_q(i64::from(count));
    w.into_bytes()
}

pub fn request_get_item_from_pet(object_id: i32, count: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_GET_ITEM_FROM_PET);
    w.write_d(object_id);
    w.write_q(i64::from(count));
    w.into_bytes()
}

pub fn request_buy_seed(manor_id: i32, orders: &[(i32, i32)]) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_REQUEST_BUY_SEED);
    w.write_d(manor_id);
    w.write_d(orders.len() as i32);
    for (item_id, count) in orders {
        w.write_d(*item_id);
        w.write_q(i64::from(*count));
    }
    w.into_bytes()
}

pub fn request_set_seed(manor_id: i32, seeds: &[(i32, i32, i32)]) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_SET_SEED);
    w.write_d(manor_id);
    w.write_d(seeds.len() as i32);
    for (seed_id, sales, price) in seeds {
        w.write_d(*seed_id);
        w.write_q(i64::from(*sales));
        w.write_q(i64::from(*price));
    }
    w.into_bytes()
}

pub fn request_set_crop(manor_id: i32, crops: &[CropSetting]) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_SET_CROP);
    w.write_d(manor_id);
    w.write_d(crops.len() as i32);
    for crop in crops {
        w.write_d(crop.crop_id);
        w.write_q(i64::from(crop.amount));
        w.write_q(i64::from(crop.price));
        w.write_c(crop.reward);
    }
    w.into_bytes()
}

pub fn request_procure_crop_list(orders: &[CropProcure]) -> Vec<u8> {
    let mut w = extended(EX_REQUEST_PROCURE_CROP_LIST);
    w.write_d(orders.len() as i32);
    for order in orders {
        w.write_d(order.object_id);
        w.write_d(order.crop_id);
        w.write_d(order.manor_id);
        w.write_q(i64::from(order.count));
    }
    w.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::packets::{
        ARGUMENT_ITEM_NAME, ARGUMENT_NUMBER, ARGUMENT_SKILL_NAME, ARGUMENT_TEXT, CHAT_ALL, CHAT_SHOUT, MacroCommand,
        PAPERDOLL_CHEST, PAPERDOLL_HAIR, PAPERDOLL_R_HAND,
    };

    fn ds(op: u8, values: &[i32]) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_c(op);
        for v in values {
            w.write_d(*v);
        }
        w.into_bytes()
    }

    fn user_info_body() -> Vec<u8> {
        let mut w = Writer::new();
        w.write_c(OP_USER_INFO);
        for v in [-71000, 258000, -3000, 0, 268_480_001] {
            w.write_d(v);
        }
        w.write_s("Luz");
        for v in [2, 1, 18, 40] {
            w.write_d(v);
        }
        w.write_q(5_000_000_000);
        w.write_f(33.5);
        for v in [40, 41, 42, 43, 44, 45, 900, 800, 500, 400, 12345, 3000, 69000, 40] {
            w.write_d(v);
        }
        for block in 0..3 {
            for slot in 0..USER_INFO_PAPERDOLL_SLOTS as i32 {
                w.write_d(block * 1000 + slot);
            }
        }
        for v in [3, 1, 101, 300, 102, 50, 60, 70, 103, 400, 300, 104, 1, -5, 125, 80, 50, 50, 0, 0, 0, 0] {
            w.write_d(v);
        }
        for v in [1.1, 1.2, 8.0, 23.0] {
            w.write_f(v);
        }
        for v in [4, 5, 6, 1] {
            w.write_d(v);
        }
        w.write_s("Titulo");
        for v in [77, 78, 79, 80, 0x40] {
            w.write_d(v);
        }
        for v in [1, 0, 1] {
            w.write_c(v);
        }
        w.write_d(9);
        w.write_d(8);
        w.write_h(2);
        w.write_h(11);
        w.write_h(12);
        w.write_c(0);
        w.write_d(0x1000);
        w.write_c(1);
        w.write_d(0xff);
        w.write_h(5);
        w.write_h(6);
        w.write_d(0);
        w.write_h(80);
        w.write_d(18);
        w.write_d(0);
        w.write_d(1200);
        w.write_d(1100);
        w.write_c(7);
        w.write_c(0);
        w.write_d(55);
        w.write_c(1);
        w.write_c(1);
        w.write_c(0);
        for v in [0, 0, 0, 0xffffff] {
            w.write_d(v);
        }
        w.write_c(1);
        for v in [3, 4, 5, 6, 0] {
            w.write_d(v);
        }
        for v in 1..=8u16 {
            w.write_h(v);
        }
        for v in [10, 20000, 1, 15000, 2] {
            w.write_d(v);
        }
        w.into_bytes()
    }

    #[test]
    fn user_info_maps_onto_the_c4_head_and_keeps_the_new_fields() {
        assert_eq!(user_info_body().len(), 396 + 26 * 12 + 8 + 14 + 2 * 2);
        let (head, extras) = read_user_info(&user_info_body()).unwrap();
        assert_eq!((head.x, head.y, head.z, head.heading, head.object_id), (-71000, 258000, -3000, 0, 268_480_001));
        assert_eq!((head.name.as_str(), head.race, head.sex, head.class_id, head.level), ("Luz", 2, 1, 18, 40));
        assert_eq!(head.exp, i32::MAX);
        assert_eq!(extras.exp, 5_000_000_000);
        assert_eq!(extras.exp_percent, 33.5);
        assert_eq!((head.stats.str_, head.stats.dex, head.stats.con), (40, 41, 42));
        assert_eq!((head.stats.int_, head.stats.wit, head.stats.men), (43, 44, 45));
        assert_eq!((head.max_hp, head.hp, head.max_mp, head.mp), (900, 800, 500, 400));
        assert_eq!((head.stats.sp, head.stats.load, head.stats.max_load, head.stats.attack_range), (12345, 3000, 69000, 40));
        assert_eq!(head.paperdoll[PAPERDOLL_R_HAND].object_id, 7);
        assert_eq!(head.paperdoll[PAPERDOLL_R_HAND].item_id, 1007);
        assert_eq!(head.paperdoll[PAPERDOLL_CHEST].item_id, 1010);
        assert_eq!(head.paperdoll[PAPERDOLL_HAIR].item_id, 1015);
        assert_eq!(extras.paperdoll_items.len(), 26);
        assert_eq!(extras.paperdoll_augmentations[25], 2025);
        assert_eq!((extras.talisman_slots, extras.can_equip_cloak), (3, true));
        assert_eq!((head.stats.p_atk, head.stats.atk_spd, head.stats.p_def), (101, 300, 102));
        assert_eq!((head.stats.evasion, head.stats.accuracy, head.stats.critical), (50, 60, 70));
        assert_eq!((head.stats.m_atk, head.stats.cast_spd, head.stats.m_def), (103, 400, 104));
        assert_eq!((head.pvp_flag, head.karma, head.run_speed, head.walk_speed), (1, -5, 125, 80));
        assert_eq!((head.speed_multiplier, head.attack_speed_multiplier), (1.1, 1.2));
        assert_eq!((head.collision_radius, head.collision_height), (8.0, 23.0));
        assert_eq!((head.hair_style, head.hair_color, head.face, head.builder_level), (4, 5, 6, 1));
        assert_eq!(head.title, "Titulo");
        assert_eq!((head.social.clan_id, head.social.clan_crest_id, head.social.ally_id, head.social.ally_crest_id), (77, 78, 79, 80));
        assert_eq!(head.social.relation, 0x40);
        assert_eq!((head.social.mount_type, head.social.private_store, head.social.dwarven_craft), (1, 0, true));
        assert_eq!((head.social.pk_kills, head.social.pvp_kills), (9, 8));
        assert_eq!(head.social.cubic_ids, vec![11, 12]);
        assert_eq!(head.abnormal_effect, 0x1000);
        assert_eq!(extras.clan_privileges, 0xff);
        assert_eq!((head.social.recom_left, head.social.recom_have, head.inventory_limit), (5, 6, 80));
        assert_eq!((extras.active_class_id, head.max_cp, head.cp), (18, 1200, 1100));
        assert_eq!((head.enchant_effect, extras.team, extras.large_crest_id), (7, 0, 55));
        assert_eq!((head.social.noble, head.social.hero), (true, true));
        assert_eq!(head.name_color, 0xffffff);
        assert!(extras.running);
        assert_eq!((extras.pledge_class, extras.pledge_type, extras.title_color, extras.cursed_weapon_level), (3, 4, 5, 6));
        assert_eq!(extras.elements, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!((extras.agathion, extras.fame, extras.minimap_allowed, extras.vitality), (10, 20000, true, 15000));
        assert_eq!(extras.abnormal_special, 2);
        assert_eq!(head.social.pledge_rights, [false; 4]);
    }

    #[test]
    fn user_info_rejects_every_truncation_and_ignores_trailing_bytes() {
        let body = user_info_body();
        for len in 0..body.len() {
            assert!(read_user_info(&body[..len]).is_err(), "len {len}");
        }
        let mut longer = body.clone();
        longer.extend_from_slice(&[9, 9, 9]);
        assert!(read_user_info(&longer).is_ok());
    }

    fn char_info_body() -> Vec<u8> {
        let mut w = Writer::new();
        w.write_c(OP_CHAR_INFO);
        for v in [10, 20, 30, 0, 555] {
            w.write_d(v);
        }
        w.write_s("Outro");
        for v in [1, 0, 10] {
            w.write_d(v);
        }
        for block in 0..2 {
            for slot in 0..CHAR_INFO_PAPERDOLL_SLOTS as i32 {
                w.write_d(block * 100 + slot + 1);
            }
        }
        for v in [2, 1, 2, 1, 333, 444, 0, 120, 80, 50, 40, 0, 0, 0, 0] {
            w.write_d(v);
        }
        for v in [1.0, 1.1, 7.5, 24.0] {
            w.write_f(v);
        }
        for v in [3, 2, 1] {
            w.write_d(v);
        }
        w.write_s("Tit");
        for v in [5, 6, 7, 8] {
            w.write_d(v);
        }
        for v in [1, 1, 1, 0, 0, 2, 1] {
            w.write_c(v);
        }
        w.write_h(1);
        w.write_h(9);
        w.write_c(1);
        w.write_d(0x20);
        w.write_c(0);
        w.write_h(4);
        for v in [1_000_000, 10] {
            w.write_d(v);
        }
        w.write_d(0);
        w.write_c(5);
        w.write_c(1);
        w.write_d(66);
        w.write_c(1);
        w.write_c(0);
        w.write_c(1);
        for v in [1, 2, 3, 0xabcdef, 49152, 3, 4, 5, 6, 700, 8, 9, 1, 0] {
            w.write_d(v);
        }
        w.into_bytes()
    }

    #[test]
    fn char_info_uses_the_oracle_order_and_takes_the_visible_slots() {
        assert_eq!(char_info_body().len(), 247 + 21 * 8 + 12 + 8 + 2);
        let (info, extras) = read_char_info(&char_info_body()).unwrap();
        assert_eq!((info.x, info.y, info.z, info.object_id), (10, 20, 30, 555));
        assert_eq!(info.heading, 49152);
        assert_eq!((info.name.as_str(), info.race, info.sex, info.class_id), ("Outro", 1, 0, 10));
        assert_eq!(info.equipped, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(extras.paperdoll_augmentations[0], 101);
        assert_eq!((info.pvp_flag, info.karma, info.m_atk_speed, info.p_atk_speed), (2, 1, 333, 444));
        assert_eq!(info.run_speed, 120);
        assert_eq!((info.walk_speed, info.swim_run_speed, info.swim_walk_speed), (80, 50, 40));
        assert_eq!((info.speed_multiplier, info.collision_radius, info.collision_height), (1.0, 7.5, 24.0));
        assert_eq!((info.hair_style, info.hair_color, info.face, info.title.as_str()), (3, 2, 1, "Tit"));
        assert_eq!((info.clan_id, info.clan_crest_id, info.ally_id, info.ally_crest_id), (5, 6, 7, 8));
        assert!(!info.sitting && info.running && info.in_combat && !info.alike_dead && !info.invisible);
        assert_eq!((info.mount_type, info.store_type), (2, 1));
        assert_eq!(info.cubics, vec![9]);
        assert_eq!(info.finding_party, 1);
        assert_eq!(info.abnormal_effect, 0x20);
        assert_eq!(info.recommendations_received, 4);
        assert_eq!((info.mount_npc_id, info.active_class_id), (1_000_000, 10));
        assert_eq!((info.enchant_effect, info.team, info.large_crest_id), (5, 1, 66));
        assert_eq!((info.noble, info.hero, info.fishing), (1, 0, 1));
        assert_eq!((info.fish_x, info.fish_y, info.fish_z, info.name_color), (1, 2, 3, 0xabcdef));
        assert_eq!((extras.pledge_class, extras.pledge_type, extras.title_color), (3, 4, 5));
        assert_eq!((extras.cursed_weapon_level, extras.clan_reputation, extras.transformation), (6, 700, 8));
        assert_eq!(extras.agathion, 9);
        assert_eq!(extras.abnormal_special, 0);
        let body = char_info_body();
        for len in 0..body.len() {
            assert!(read_char_info(&body[..len]).is_err(), "len {len}");
        }
    }

    fn npc_info_body() -> Vec<u8> {
        let mut w = Writer::new();
        w.write_c(OP_NPC_INFO);
        for v in [900, 1_020_000 + 5, 1, 1000, 2000, -300, 16384, 0, 253, 278, 88, 55, 0, 0, 0, 0, 0, 0] {
            w.write_d(v);
        }
        for v in [1.0, 1.0, 9.0, 18.0] {
            w.write_f(v);
        }
        for v in [20, 0, 21] {
            w.write_d(v);
        }
        for v in [1, 1, 0, 0, 2] {
            w.write_c(v);
        }
        w.write_d(-1);
        w.write_s("Lobo");
        w.write_d(-1);
        w.write_s("Selvagem");
        for v in [0xff, 0, 0, 0x8, 0, 0, 0, 0] {
            w.write_d(v);
        }
        w.write_c(1);
        w.write_c(0);
        w.write_f(9.0);
        w.write_f(18.0);
        for v in [0, 1, 0, 3] {
            w.write_d(v);
        }
        w.write_c(1);
        w.write_c(1);
        w.write_d(4);
        w.write_d(5);
        w.into_bytes()
    }

    #[test]
    fn npc_info_reads_the_high_five_layout_with_npc_strings() {
        assert_eq!(npc_info_body().len(), 206 + 10 + 18);
        let (info, extras) = read_npc_info(&npc_info_body()).unwrap();
        assert_eq!((info.object_id, info.npc_id, info.attackable), (900, 20005, true));
        assert_eq!((info.x, info.y, info.z, info.heading), (1000, 2000, -300, 16384));
        assert_eq!((info.m_atk_speed, info.p_atk_speed, info.run_speed, info.walk_speed), (253, 278, 88, 55));
        assert_eq!((info.rhand, info.chest, info.lhand), (20, 0, 21));
        assert!(info.name_above && info.running && !info.in_combat && !info.alike_dead);
        assert_eq!(info.spawn_animation, 2);
        assert_eq!((info.name.as_str(), info.title.as_str()), ("Lobo", "Selvagem"));
        assert_eq!((info.playable_flag, info.pvp_flag, info.karma, info.abnormal_effect), (0xff, 0, 0, 0x8));
        assert_eq!((info.base_collision_radius, info.base_collision_height), (9.0, 18.0));
        assert_eq!((extras.name_string, extras.title_string), (-1, -1));
        assert_eq!((extras.enchant_effect, extras.flying, extras.color_effect), (0, 1, 3));
        assert!(extras.targetable && extras.show_name);
        assert_eq!((extras.abnormal_special, extras.display_effect), (4, 5));
        let body = npc_info_body();
        for len in 0..body.len() {
            assert!(read_npc_info(&body[..len]).is_err(), "len {len}");
        }
    }

    fn item_bytes(w: &mut Writer, object_id: i32, item_id: i32, count: i64, equipped: bool) {
        w.write_d(object_id);
        w.write_d(item_id);
        w.write_d(-1);
        w.write_q(count);
        w.write_h(5);
        w.write_h(0);
        w.write_h(equipped as u16);
        w.write_d(0x4000);
        w.write_h(3);
        w.write_h(0);
        w.write_d(88);
        w.write_d(-1);
        w.write_d(-9999);
        w.write_h(1);
        w.write_h(20);
        for v in 1..=6u16 {
            w.write_h(v);
        }
        for v in [7u16, 8, 9] {
            w.write_h(v);
        }
    }

    #[test]
    fn item_list_and_inventory_update_use_68_byte_items_with_q_counts() {
        let mut one = Writer::new();
        item_bytes(&mut one, 1, 57, 99_000_000_000, false);
        assert_eq!(one.into_bytes().len(), INVENTORY_ITEM_BYTES);
        let mut w = Writer::new();
        w.write_c(OP_ITEM_LIST);
        w.write_h(1);
        w.write_h(2);
        item_bytes(&mut w, 501, 57, 99_000_000_000, false);
        item_bytes(&mut w, 502, 1463, 1, true);
        w.write_h(0);
        let body = w.into_bytes();
        assert_eq!(body.len(), 5 + 2 * INVENTORY_ITEM_BYTES + 2);
        let (list, extras) = read_item_list(&body).unwrap();
        assert!(list.show_window);
        assert_eq!(list.items.len(), 2);
        assert_eq!((list.items[0].object_id, list.items[0].item_id), (501, 57));
        assert_eq!(list.items[0].count, i32::MAX);
        assert_eq!(extras[0].count, 99_000_000_000);
        assert!(list.items[1].equipped && !list.items[0].equipped);
        assert_eq!((list.items[1].type2, list.items[1].body_part, list.items[1].enchant), (5, 0x4000, 3));
        assert_eq!((extras[1].location, extras[1].augmentation, extras[1].mana, extras[1].remaining_seconds), (-1, 88, -1, -9999));
        assert_eq!(extras[1].attack_element, [1, 20]);
        assert_eq!(extras[1].defence_elements, [1, 2, 3, 4, 5, 6]);
        assert_eq!(extras[1].enchant_options, [7, 8, 9]);
        for len in 0..body.len() - 2 {
            assert!(read_item_list(&body[..len]).is_err(), "len {len}");
        }
        let mut update = Writer::new();
        update.write_c(OP_INVENTORY_UPDATE);
        update.write_h(2);
        update.write_h(c4_item_added());
        item_bytes(&mut update, 7, 1060, 5, false);
        update.write_h(c4_item_removed());
        item_bytes(&mut update, 8, 1061, 1, false);
        let (inventory, _) = read_inventory_update(&update.into_bytes()).unwrap();
        assert_eq!(inventory.changes.len(), 2);
        assert_eq!((inventory.changes[0].change, inventory.changes[0].item.item_id), (1, 1060));
        assert_eq!((inventory.changes[1].change, inventory.changes[1].item.object_id), (3, 8));
        assert!(read_inventory_update(&[OP_INVENTORY_UPDATE, 0xff, 0xff]).is_err());
    }

    fn c4_item_added() -> u16 {
        crate::game::packets::ITEM_ADDED
    }

    fn c4_item_removed() -> u16 {
        crate::game::packets::ITEM_REMOVED
    }

    #[test]
    fn skill_list_reads_the_two_trailing_flags() {
        let mut w = Writer::new();
        w.write_c(OP_SKILL_LIST);
        w.write_d(2);
        for (passive, level, id, disabled, enchanted) in [(0, 3, 1177, 0, 0), (1, 1, 141, 1, 1)] {
            w.write_d(passive);
            w.write_d(level);
            w.write_d(id);
            w.write_c(disabled);
            w.write_c(enchanted);
        }
        let body = w.into_bytes();
        let (list, flags) = read_skill_list(&body).unwrap();
        assert_eq!(list.skills, vec![SkillEntry { passive: false, level: 3, id: 1177 }, SkillEntry { passive: true, level: 1, id: 141 }]);
        assert_eq!(flags, vec![(false, false), (true, true)]);
        assert!(read_skill_list(&body[..body.len() - 1]).is_err());
    }

    #[test]
    fn system_message_reads_every_parameter_type_of_the_oracle() {
        let mut w = Writer::new();
        w.write_c(OP_SYSTEM_MESSAGE);
        w.write_d(302);
        w.write_d(9);
        w.write_d(ARGUMENT_TEXT);
        w.write_s("a");
        w.write_d(ARGUMENT_NUMBER);
        w.write_d(5);
        w.write_d(ARGUMENT_ITEM_NAME);
        w.write_d(57);
        w.write_d(ARGUMENT_SKILL_NAME);
        w.write_d(1177);
        w.write_d(3);
        w.write_d(ARGUMENT_BIG_NUMBER);
        w.write_q(7);
        w.write_d(ARGUMENT_BIG_NUMBER);
        w.write_q(8_000_000_000);
        w.write_d(ARGUMENT_PLACE_NAME);
        w.write_d(1);
        w.write_d(2);
        w.write_d(3);
        w.write_d(ARGUMENT_PLAYER_NAME);
        w.write_s("Luz");
        w.write_d(13);
        w.write_d(99);
        let message = read_system_message(&w.into_bytes()).unwrap();
        assert_eq!(message.id, 302);
        assert_eq!(
            message.args,
            vec![
                SystemMessageArg::Text("a".into()),
                SystemMessageArg::Number(5),
                SystemMessageArg::ItemName(57),
                SystemMessageArg::SkillName { id: 1177, level: 3 },
                SystemMessageArg::Number(7),
                SystemMessageArg::Text("8000000000".into()),
                SystemMessageArg::Unknown { kind: ARGUMENT_PLACE_NAME },
                SystemMessageArg::Text("Luz".into()),
                SystemMessageArg::Unknown { kind: 13 },
            ]
        );
        assert!(read_system_message(&ds(OP_SYSTEM_MESSAGE, &[1, i32::MAX])).is_err());
    }

    #[test]
    fn creature_say_and_npc_say_carry_the_npc_string_slot() {
        let mut w = Writer::new();
        w.write_c(OP_CREATURE_SAY);
        w.write_d(5);
        w.write_d(CHAT_SHOUT);
        w.write_s("Luz");
        w.write_d(-1);
        w.write_s("oi");
        let (say, npc_string) = read_creature_say(&w.into_bytes()).unwrap();
        assert_eq!(say, Say2::text(5, CHAT_SHOUT, "Luz", "oi"));
        assert_eq!(npc_string, -1);
        let mut w = Writer::new();
        w.write_c(OP_CREATURE_SAY);
        w.write_d(0);
        w.write_d(CHAT_ALL);
        w.write_s("Guarda");
        w.write_d(1_000_123);
        w.write_s("um");
        w.write_s("dois");
        let (say, npc_string) = read_creature_say(&w.into_bytes()).unwrap();
        assert_eq!(say.body, Say2Body::Text { name: "Guarda".into(), text: "um dois".into() });
        assert_eq!(npc_string, 1_000_123);
        let (boat, _) = read_creature_say(&ds(OP_CREATURE_SAY, &[0, CHAT_BOAT, 801, 1500])).unwrap();
        assert_eq!(boat.body, Say2Body::Tables { sys_string_id: 801, sys_msg_id: 1500 });
        let mut w = Writer::new();
        w.write_c(OP_NPC_SAY);
        w.write_d(9);
        w.write_d(CHAT_ALL);
        w.write_d(1_000_000 + 20001);
        w.write_d(-1);
        w.write_s("grr");
        let (say, npc_string) = read_npc_say(&w.into_bytes()).unwrap();
        assert_eq!((say.object_id, say.kind, say.npc_id, say.text.as_str(), npc_string), (9, CHAT_ALL, 20001, "grr", -1));
        assert!(read_creature_say(&[OP_CREATURE_SAY, 1]).is_err());
    }

    #[test]
    fn items_on_the_ground_use_q_counts() {
        let mut w = Writer::new();
        w.write_c(OP_SPAWN_ITEM);
        for v in [70, 57, 1, 2, 3, 1] {
            w.write_d(v);
        }
        w.write_q(1500);
        w.write_d(0);
        w.write_d(0);
        let item = read_spawn_item(&w.into_bytes()).unwrap();
        assert_eq!((item.object_id, item.item_id, item.x, item.y, item.z, item.stackable, item.count), (70, 57, 1, 2, 3, true, 1500));
        let mut w = Writer::new();
        w.write_c(OP_DROP_ITEM);
        for v in [5, 71, 1060, 4, 5, 6, 0] {
            w.write_d(v);
        }
        w.write_q(1);
        w.write_d(1);
        let dropped = read_drop_item(&w.into_bytes()).unwrap();
        assert_eq!((dropped.player_id, dropped.object_id, dropped.item_id, dropped.count), (5, 71, 1060, 1));
        assert!(!dropped.stackable);
        assert!(read_spawn_item(&ds(OP_SPAWN_ITEM, &[1, 2])).is_err());
    }

    #[test]
    fn magic_skill_use_reads_type_and_location_counter() {
        let mut w = Writer::new();
        w.write_c(OP_MAGIC_SKILL_USE);
        for v in [5, 900, 1177, 3, 1500, 4000, 10, 20, 30] {
            w.write_d(v);
        }
        w.write_h(0);
        w.write_h(1);
        for v in [100, 200, 300, 11, 21, 31] {
            w.write_d(v);
        }
        let skill = read_magic_skill_use(&w.into_bytes()).unwrap();
        assert_eq!((skill.caster_id, skill.target_id, skill.skill_id, skill.skill_level), (5, 900, 1177, 3));
        assert_eq!((skill.hit_time, skill.reuse_delay, skill.x, skill.y, skill.z), (1500, 4000, 10, 20, 30));
        assert_eq!(skill.positions, vec![[100, 200, 300]]);
        assert!(skill.extra.is_empty());
    }

    #[test]
    fn attack_keeps_the_hits_and_reads_the_target_position() {
        let mut w = Writer::new();
        w.write_c(OP_ATTACK);
        for v in [5, 900, 33] {
            w.write_d(v);
        }
        w.write_c(0x20);
        for v in [1, 2, 3] {
            w.write_d(v);
        }
        w.write_h(1);
        w.write_d(901);
        w.write_d(12);
        w.write_c(0x80);
        for v in [4, 5, 6] {
            w.write_d(v);
        }
        let (attack, target) = read_attack(&w.into_bytes()).unwrap();
        assert_eq!(attack.attacker_id, 5);
        assert_eq!(attack.hits.len(), 2);
        assert!(attack.hits[0].critical() && attack.hits[1].miss());
        assert_eq!(target, [4, 5, 6]);
    }

    #[test]
    fn etc_status_shortcuts_and_extended_packets() {
        let etc = read_etc_status_update(&ds(OP_ETC_STATUS_UPDATE, &[2, 1, 0, 1, 0, 3, 0, 0, 0])).unwrap();
        assert_eq!(etc.values, [2, 1, 0, 1, 1]);
        assert!(read_etc_status_update(&ds(OP_ETC_STATUS_UPDATE, &[2, 1])).is_err());
        let mut w = Writer::new();
        w.write_c(OP_SHORT_CUT_INIT);
        w.write_d(5);
        for v in [SHORTCUT_ITEM, 0, 700, 1, -1, 0, 0, 0] {
            w.write_d(v);
        }
        for v in [SHORTCUT_SKILL, 1, 1177, 3] {
            w.write_d(v);
        }
        w.write_c(0);
        w.write_d(1);
        for v in [SHORTCUT_ACTION, 2, 0, 1, SHORTCUT_MACRO, 3, 9, 1] {
            w.write_d(v);
        }
        for v in [6, 4, 77, 1] {
            w.write_d(v);
        }
        let init = read_short_cut_init(&w.into_bytes()).unwrap();
        assert_eq!(
            init.shortcuts,
            vec![
                Shortcut { slot: 0, kind: ShortcutKind::Item { object_id: 700 }, character_type: 1 },
                Shortcut { slot: 1, kind: ShortcutKind::Skill { skill_id: 1177, level: 3 }, character_type: 1 },
                Shortcut { slot: 2, kind: ShortcutKind::Action { id: 0 }, character_type: 1 },
                Shortcut { slot: 3, kind: ShortcutKind::Macro { id: 9 }, character_type: 1 },
                Shortcut { slot: 4, kind: ShortcutKind::Other { kind: 6 }, character_type: 0 },
            ]
        );
        let mut w = Writer::new();
        w.write_c(OP_SHORT_CUT_REGISTER);
        for v in [SHORTCUT_ACTION, 11, 5, 1] {
            w.write_d(v);
        }
        let registered = read_short_cut_register(&w.into_bytes()).unwrap();
        assert_eq!((registered.slot, registered.kind), (11, ShortcutKind::Action { id: 5 }));
        let mut w = Writer::new();
        w.write_c(OP_EX_SERVER);
        w.write_h(EX_STORAGE_MAX_COUNT);
        for v in 1..=9 {
            w.write_d(v);
        }
        let (storage, extra) = read_storage_max_count(&w.into_bytes()).unwrap();
        assert_eq!(storage.values, [1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(extra, [8, 9]);
        let mut w = Writer::new();
        w.write_c(OP_EX_SERVER);
        w.write_h(EX_BASIC_ACTION_LIST);
        w.write_d(3);
        for v in [0, 1, 2] {
            w.write_d(v);
        }
        assert_eq!(read_basic_action_list(&w.into_bytes()).unwrap(), vec![0, 1, 2]);
        let mut w = Writer::new();
        w.write_c(OP_EX_SERVER);
        w.write_h(EX_ROTATION);
        w.write_d(5);
        w.write_d(16384);
        assert_eq!(read_rotation(&w.into_bytes()).unwrap(), (5, 16384));
    }

    #[test]
    fn client_packets_use_the_high_five_opcodes_and_layouts() {
        assert_eq!(enter_world(), [0x11]);
        assert_eq!(request_restart(), [0x57]);
        assert_eq!(game_guard_reply([1, 2, 3, 4]), ds(0xcb, &[1, 2, 3, 4]));
        assert_eq!(move_backward_to_location([1, 2, 3], [4, 5, 6]), ds(0x0f, &[1, 2, 3, 4, 5, 6, 1]));
        assert_eq!(validate_position([1, 2, 3], 4, 5), ds(0x59, &[1, 2, 3, 4, 5]));
        let mut expected = ds(0x1f, &[7, 1, 2, 3]);
        expected.push(1);
        assert_eq!(action(7, [1, 2, 3], true), expected);
        expected[0] = 0x32;
        assert_eq!(attack_request(7, [1, 2, 3], true), expected);
        assert_eq!(use_item(9, true), ds(0x19, &[9, 1]));
        let mut expected = ds(0x39, &[1177, 0]);
        expected.push(1);
        assert_eq!(request_magic_skill_use(1177, false, true), expected);
        expected[0] = 0x56;
        assert_eq!(request_action_use(1177, false, true), expected);
        let mut expected = vec![0x49];
        expected.extend("oi".encode_utf16().flat_map(|u| u.to_le_bytes()));
        expected.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
        assert_eq!(say2("oi", CHAT_ALL, "x"), expected);
        assert!(say2("oi", CHAT_TELL, "Luz").len() > expected.len());
        assert_eq!(cannot_move_anymore([1, 2, 3], 4), ds(0x47, &[1, 2, 3, 4]));
        assert_eq!(request_item_list(), [0x14]);
        assert_eq!(request_skill_list(), [0x50]);
        assert_eq!(request_quest_list(), [0x62]);
        assert_eq!(request_skill_cool_time(), [0xa6]);
        assert_eq!(appearing(), [0x3a]);
        assert_eq!(request_target_cancel(1), [0x48, 1, 0]);
        assert_eq!(request_unequip_item(0x80), ds(0x16, &[0x80]));
        let mut expected = ds(0x17, &[7]);
        expected.extend_from_slice(&5i64.to_le_bytes());
        expected.extend_from_slice(&[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
        assert_eq!(request_drop_item(7, 5, [1, 2, 3]), expected);
        let mut expected = ds(0x60, &[7]);
        expected.extend_from_slice(&5i64.to_le_bytes());
        assert_eq!(request_destroy_item(7, 5), expected);
        assert_eq!(request_shortcut_reg(SHORTCUT_SKILL, 3, 1177, 4), ds(0x3d, &[2, 3, 1177, 4, 1]));
        assert_eq!(request_shortcut_reg(SHORTCUT_ACTION, 3, 5, 1), ds(0x3d, &[3, 3, 5, 0, 1]));
        assert_eq!(request_shortcut_del(3), ds(0x3f, &[3]));
        assert_eq!(request_manor_list(), [0xd0, 0x01, 0x00]);
        assert_eq!(request_auto_soul_shot(1463, true), [0xd0, 0x0d, 0x00, 0xb7, 0x05, 0, 0, 1, 0, 0, 0]);
        assert_eq!(request_ex_enchant_skill(5, 6), [0xd0, 0x0f, 0, 5, 0, 0, 0, 6, 0, 0, 0]);
        assert_eq!(request_ex_enchant_skill_info(5, 6)[1], 0x0e);
        assert_eq!(request_ex_pledge_crest_large(9), [0xd0, 0x10, 0, 9, 0, 0, 0]);
        assert_eq!(request_ex_set_pledge_crest_large(&[1, 2])[..7], [0xd0, 0x11, 0, 2, 0, 0, 0]);
        assert_eq!(request_ex_accept_join_mpcc(true), [0xd0, 0x07, 0, 1, 0, 0, 0]);
        assert_eq!(request_ex_ask_join_mpcc("a")[1], 0x06);
        assert_eq!(request_ex_oust_from_mpcc("a")[1], 0x08);
        assert_eq!(request_write_hero_words("a")[1], 0x05);
        assert_eq!(request_olympiad_observer_end(), [0xd0, 0x29, 0x00]);
        assert_eq!(request_change_party_leader("a")[1], 0x0c);
        assert_eq!(request_party_loot_modification(2), [0xd0, 0x78, 0x00, 2, 0, 0, 0]);
        assert_eq!(answer_party_loot_modification(true), [0xd0, 0x79, 0x00, 1, 0, 0, 0]);
        assert_eq!(request_bypass_to_server("a")[0], 0x23);
        assert_eq!(request_link_html("a")[0], 0x22);
    }

    #[test]
    fn trade_start_reads_items_in_the_item_list_format() {
        let mut w = Writer::new();
        w.write_c(OP_TRADE_START);
        w.write_d(900);
        w.write_h(1);
        item_bytes(&mut w, 501, 57, 5_000_000_000, false);
        let body = w.into_bytes();
        let (start, extras) = read_trade_start(&body).unwrap();
        assert_eq!(start.partner_id, 900);
        assert_eq!(start.items.len(), 1);
        assert_eq!((start.items[0].object_id, start.items[0].item_id, start.items[0].type1), (501, 57, 0));
        assert_eq!(extras[0].count, 5_000_000_000);
        for len in 0..body.len() {
            assert!(read_trade_start(&body[..len]).is_err(), "len {len}");
        }
    }

    fn trade_add_item_body(op: u8) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_c(op);
        w.write_h(1);
        w.write_h(0);
        w.write_d(7);
        w.write_d(1463);
        w.write_q(3);
        w.write_h(5);
        w.write_h(2);
        w.write_d(0x4000);
        w.write_h(3);
        w.write_h(0);
        w.write_h(9);
        w.write_d(-1);
        w.write_d(99);
        w.write_d(-9999);
        for v in 1..=2u16 {
            w.write_h(v);
        }
        for v in 3..=8u16 {
            w.write_h(v);
        }
        for v in [9u16, 10, 11] {
            w.write_h(v);
        }
        w.into_bytes()
    }

    #[test]
    fn trade_own_add_and_trade_other_add_read_a_single_item() {
        let body = trade_add_item_body(OP_TRADE_OWN_ADD);
        let (own, extra) = read_trade_own_add(&body).unwrap();
        assert_eq!(own.items.len(), 1);
        assert_eq!((own.items[0].object_id, own.items[0].item_id, own.items[0].type1), (7, 1463, 2));
        assert_eq!((own.items[0].type2, own.items[0].custom_type2), (5, 9));
        assert_eq!(extra.count, 3);
        assert_eq!((extra.augmentation, extra.mana, extra.remaining_seconds), (-1, 99, -9999));
        let body = trade_add_item_body(OP_TRADE_OTHER_ADD);
        let (other, _) = read_trade_other_add(&body).unwrap();
        assert_eq!(other.items[0].body_part, 0x4000);
        for len in 0..body.len() {
            assert!(read_trade_own_add(&body[..len]).is_err(), "len {len}");
        }
    }

    fn party_member_tail(w: &mut Writer, race: i32, pet: Option<i32>) {
        w.write_d(0);
        w.write_d(race);
        w.write_d(0);
        w.write_d(0);
        match pet {
            Some(pet_id) => {
                w.write_d(pet_id);
                for v in [1001, 2] {
                    w.write_d(v);
                }
                w.write_s("Bichinho");
                for v in [100, 120, 50, 60, 70] {
                    w.write_d(v);
                }
            }
            None => w.write_d(0),
        }
    }

    #[test]
    fn party_small_window_all_and_add_read_members_with_the_pet_marker() {
        let mut w = Writer::new();
        w.write_c(OP_PARTY_SMALL_WINDOW_ALL);
        w.write_d(5);
        w.write_d(1);
        w.write_d(2);
        for (id, pet) in [(7, None), (8, Some(900))] {
            w.write_d(id);
            w.write_s("M");
            for v in [1, 2, 3, 4, 5, 6, 7, 8] {
                w.write_d(v);
            }
            party_member_tail(&mut w, 2, pet);
        }
        let body = w.into_bytes();
        let (all, extras) = read_party_small_window_all(&body).unwrap();
        assert_eq!((all.leader_id, all.loot_distribution, all.members.len()), (5, 1, 2));
        assert_eq!(all.members[0].object_id, 7);
        assert!(extras[0].pet.is_none());
        assert_eq!(extras[1].pet.as_ref().unwrap().object_id, 900);
        assert_eq!(extras[1].pet.as_ref().unwrap().name, "Bichinho");
        for len in 0..body.len() {
            assert!(read_party_small_window_all(&body[..len]).is_err(), "len {len}");
        }
        let mut w = Writer::new();
        w.write_c(OP_PARTY_SMALL_WINDOW_ADD);
        w.write_d(5);
        w.write_d(1);
        w.write_d(9);
        w.write_s("N");
        for v in [1, 2, 3, 4, 5, 6, 7, 8] {
            w.write_d(v);
        }
        party_member_tail(&mut w, 3, None);
        let (add, extra) = read_party_small_window_add(&w.into_bytes()).unwrap();
        assert_eq!((add.leader_id, add.member.object_id, extra.race), (5, 9, 3));
    }

    #[test]
    fn pledge_show_member_list_add_and_update_map_onto_the_c4_member() {
        let mut w = Writer::new();
        w.write_c(OP_PLEDGE_SHOW_MEMBER_LIST_ADD);
        w.write_s("Luz");
        for v in [40, 18, 0, 1, 900, 0] {
            w.write_d(v);
        }
        let (member, extra) = read_pledge_show_member_list_add(&w.into_bytes()).unwrap();
        assert_eq!((member.name.as_str(), member.level, member.class_id, member.online_object_id), ("Luz", 40, 18, 900));
        assert_eq!(extra.pledge_type, 0);
        let mut w = Writer::new();
        w.write_c(OP_PLEDGE_SHOW_MEMBER_LIST_UPDATE);
        w.write_s("Luz");
        for v in [40, 18, 1, 2, 900, 1, 1] {
            w.write_d(v);
        }
        let (member, extra) = read_pledge_show_member_list_update(&w.into_bytes()).unwrap();
        assert_eq!((member.unknown, member.flag, member.online_object_id), (1, 2, 900));
        assert_eq!((extra.pledge_type, extra.has_sponsor), (1, true));
    }

    #[test]
    fn pledge_show_member_list_all_keeps_the_header_and_reputation_extras() {
        let mut w = Writer::new();
        w.write_c(OP_PLEDGE_SHOW_MEMBER_LIST_ALL);
        w.write_d(0);
        w.write_d(5);
        w.write_d(0);
        w.write_s("Clã");
        w.write_s("Chefe");
        for v in [10, 3, 100, 200] {
            w.write_d(v);
        }
        for v in [300, 1, 500] {
            w.write_d(v);
        }
        w.write_d(0);
        w.write_d(0);
        w.write_d(9);
        w.write_s("Aliança");
        w.write_d(11);
        w.write_d(1);
        w.write_d(1);
        w.write_s("Luz");
        for v in [40, 18, 1, 2, 900, 1] {
            w.write_d(v);
        }
        let body = w.into_bytes();
        let (all, extra) = read_pledge_show_member_list_all(&body).unwrap();
        assert_eq!((all.pledge_id, all.name.as_str(), all.leader_name.as_str()), (5, "Clã", "Chefe"));
        assert_eq!((all.crest_id, all.castle_id, all.hideout_id), (10, 100, 200));
        assert_eq!((all.ally_id, all.ally_name.as_str(), all.ally_crest_id, all.at_war), (9, "Aliança", 11, true));
        assert_eq!(all.members.len(), 1);
        assert_eq!((extra.fort_id, extra.reputation_rank, extra.reputation_score), (300, 1, 500));
        assert!(extra.members[0].has_sponsor);
        for len in 0..body.len() {
            assert!(read_pledge_show_member_list_all(&body[..len]).is_err(), "len {len}");
        }
    }

    #[test]
    fn pledge_show_info_update_keeps_the_extra_clan_fields() {
        let mut w = Writer::new();
        w.write_c(OP_PLEDGE_SHOW_INFO_UPDATE);
        for v in [5, 10, 3, 100, 200, 300, 1, 500, 0, 0, 9] {
            w.write_d(v);
        }
        w.write_s("Aliança");
        w.write_d(11);
        w.write_d(1);
        let body = w.into_bytes();
        let (update, extra) = read_pledge_show_info_update(&body).unwrap();
        assert_eq!((update.pledge_id, update.crest_id, update.castle_id, update.hideout_id), (5, 10, 100, 200));
        assert_eq!((update.ally_id, update.ally_name.as_str(), update.ally_crest_id, update.at_war), (9, "Aliança", 11, true));
        assert_eq!((extra.fort_id, extra.reputation_rank, extra.reputation_score), (300, 1, 500));
        for len in 0..body.len() {
            assert!(read_pledge_show_info_update(&body[..len]).is_err(), "len {len}");
        }
    }

    #[test]
    fn friend_packet_reads_add_and_remove_by_the_operation_field() {
        let mut w = Writer::new();
        w.write_c(OP_FRIEND_PACKET);
        w.write_d(1);
        w.write_d(900);
        w.write_s("Luz");
        w.write_d(1);
        w.write_d(900);
        assert_eq!(read_friend_packet(&w.into_bytes()).unwrap(), FriendPacketHighFive::Added);
        let mut w = Writer::new();
        w.write_c(OP_FRIEND_PACKET);
        w.write_d(3);
        w.write_d(900);
        w.write_s("");
        w.write_d(0);
        w.write_d(0);
        assert_eq!(read_friend_packet(&w.into_bytes()).unwrap(), FriendPacketHighFive::Removed);
        let mut w = Writer::new();
        w.write_c(OP_FRIEND_PACKET);
        w.write_d(99);
        w.write_d(0);
        w.write_s("");
        w.write_d(0);
        w.write_d(0);
        assert_eq!(read_friend_packet(&w.into_bytes()).unwrap(), FriendPacketHighFive::Unknown { operation: 99 });
    }

    #[test]
    fn relation_changed_reads_the_first_entry_of_a_batch() {
        let mut w = Writer::new();
        w.write_c(OP_RELATION_CHANGED);
        w.write_d(2);
        for v in [7, 2, 1, 30, 1] {
            w.write_d(v);
        }
        for v in [8, 0, 0, -5, 0] {
            w.write_d(v);
        }
        let body = w.into_bytes();
        let entries = read_relation_changed(&body).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0], RelationChanged { object_id: 7, relation: 2, auto_attack: 1, karma: 30, pvp_flag: 1 });
        for len in 0..body.len() {
            assert!(read_relation_changed(&body[..len]).is_err(), "len {len}");
        }
    }

    #[test]
    fn vote_system_info_and_ui_setting_validate_their_shape() {
        let mut w = Writer::new();
        w.write_c(OP_EX_SERVER);
        w.write_h(EX_VOTE_SYSTEM_INFO);
        for v in [10, 20, 100, 5, 1] {
            w.write_d(v);
        }
        let info = read_vote_system_info(&w.into_bytes()).unwrap();
        assert_eq!(
            (info.recommendations_left, info.recommendations_have, info.bonus_time, info.bonus_value, info.bonus_type),
            (10, 20, 100, 5, 1)
        );
        let mut w = Writer::new();
        w.write_c(OP_EX_SERVER);
        w.write_h(EX_UI_SETTING);
        w.write_d(0);
        w.write_d(0);
        w.write_d(1);
        w.write_c(0);
        w.write_c(0);
        w.write_d(0);
        w.write_d(0x11);
        w.write_d(0x10);
        assert!(read_ui_setting(&w.into_bytes()).is_ok());
    }

    #[test]
    fn client_writers_for_shop_clan_party_trade_and_friends_use_the_high_five_opcodes() {
        let orders = [ItemOrder { object_id: 7, item_id: 57, count: 5 }];
        let mut expected = ds(0x37, &[1, 1, 7, 57]);
        expected.extend_from_slice(&5i64.to_le_bytes());
        assert_eq!(request_sell_item(1, &orders), expected);
        let mut expected = ds(0x40, &[1, 1, 57]);
        expected.extend_from_slice(&3i64.to_le_bytes());
        assert_eq!(request_buy_item(1, &[(57, 3)]), expected);
        assert_eq!(request_trade(9), ds(0x1a, &[9]));
        assert_eq!(answer_trade_request(true), ds(0x55, &[1]));
        let mut expected = ds(0x1b, &[0, 7]);
        expected.extend_from_slice(&3i64.to_le_bytes());
        assert_eq!(add_trade_item(7, 3), expected);
        assert_eq!(request_trade_done(true), ds(0x1c, &[1]));
        assert_eq!(request_withdrawal_party(), [0x44]);
        assert_eq!(request_oust_party_member("a")[0], 0x45);
        assert_eq!(request_join_pledge(9), ds(0x26, &[9, 0]));
        assert_eq!(request_answer_join_pledge(true), ds(0x27, &[1]));
        assert_eq!(request_withdrawal_pledge(), [0x28]);
        assert_eq!(request_oust_pledge_member("a")[0], 0x29);
        let mut expected = ds(0x09, &[2]);
        expected.extend_from_slice(&[1, 2]);
        assert_eq!(request_set_pledge_crest(&[1, 2]), expected);
        assert_eq!(request_pledge_crest(9), ds(0x67, &[9]));
        assert_eq!(request_ally_crest(9), ds(0x92, &[9]));
        assert_eq!(request_pledge_member_list(), [0x4d]);
        assert_eq!(request_pledge_info(9), ds(0x65, &[9]));
        assert_eq!(request_join_ally(9), ds(0x8c, &[9]));
        assert_eq!(request_answer_join_ally(true), ds(0x8d, &[1]));
        assert_eq!(ally_leave(), [0x8e]);
        assert_eq!(ally_dismiss("a")[0], 0x8f);
        assert_eq!(request_dismiss_ally(), [0x90]);
        assert_eq!(request_ally_info(), [0x2e]);
        assert_eq!(request_friend_invite("a")[0], 0x77);
        assert_eq!(request_answer_friend_invite(true), ds(0x78, &[1]));
        assert_eq!(request_friend_list(), [0x79]);
        assert_eq!(request_friend_delete("a")[0], 0x7a);
        assert_eq!(request_send_friend_message("oi", "Luz")[0], 0x6b);
        assert_eq!(request_start_pledge_war("a")[0], 0x03);
        assert_eq!(request_stop_pledge_war("a")[0], 0x05);
        assert_eq!(request_surrender_pledge_war("a")[0], 0x07);
        assert_eq!(request_quest_abort(5), ds(0x63, &[5]));
        assert_eq!(request_net_ping(), [0xb1]);
    }

    #[test]
    fn gm_and_user_commands_carry_the_high_five_opcodes_of_the_server() {
        let mut gm = Writer::new();
        gm.write_c(0x7e);
        gm.write_s("Luz");
        gm.write_d(1);
        assert_eq!(request_gm_command("Luz", 1), gm.into_bytes());
        assert_eq!(request_user_command(109), ds(0xb3, &[109]));
        assert_eq!(request_gm_list(), [0x8b]);
        assert_eq!(request_gm_snoop_end(77), ds(0xb4, &[77]));
        let mut build = Writer::new();
        build.write_c(0x74);
        build.write_s("admin");
        assert_eq!(send_bypass_build_cmd("admin"), build.into_bytes());
    }

    #[test]
    fn dialog_answer_carries_the_third_field_the_high_five_server_reads() {
        assert_eq!(dlg_answer(614, 1, 0), ds(0xc6, &[614, 1, 0]));
        assert_eq!(dlg_answer(614, 1, 268_480_001), ds(0xc6, &[614, 1, 268_480_001]));
    }

    #[test]
    fn rotation_and_timing_packets_match_the_server_readers() {
        assert_eq!(start_rotating(90, 1), ds(0x5b, &[90, 1]));
        assert_eq!(finish_rotating(90), ds(0x5c, &[90]));
        assert_eq!(send_time_check(1, 2), ds(0xa0, &[1, 2]));
        assert_eq!(request_remain_time(), [0xb2]);
    }

    #[test]
    fn vehicle_packets_carry_the_boat_id_before_the_coordinates() {
        assert_eq!(request_get_on_vehicle(500, [1, -2, 3]), ds(0x53, &[500, 1, -2, 3]));
        assert_eq!(request_get_off_vehicle(500, [1, -2, 3]), ds(0x54, &[500, 1, -2, 3]));
        assert_eq!(
            request_move_to_in_vehicle(500, [1, -2, 3], [4, 5, -6]),
            ds(0x75, &[500, 1, -2, 3, 4, 5, -6])
        );
        assert_eq!(
            cannot_move_anymore_in_vehicle(5, [1, 2, 3], 4),
            ds(0x76, &[5, 1, 2, 3, 4])
        );
    }

    #[test]
    fn skill_social_and_minimap_packets_match_the_server_readers() {
        assert_eq!(request_acquire_skill_info(3, 1, 0), ds(0x73, &[3, 1, 0]));
        assert_eq!(request_acquire_skill(3, 1, 0), ds(0x7c, &[3, 1, 0]));
        assert_eq!(request_magic_skill_list(2, 1, 3), ds(0x38, &[2, 1, 3]));
        assert_eq!(request_social_action(3), ds(0x34, &[3]));
        assert_eq!(request_show_minimap(), [0x6c]);
        assert_eq!(request_restart_point(1), ds(0x7d, &[1]));
        assert_eq!(request_dismiss_party(), [0x46]);
    }

    #[test]
    fn block_writes_the_name_only_for_add_and_remove() {
        let mut add = Writer::new();
        add.write_c(0xa9);
        add.write_d(BLOCK_ADD);
        add.write_s("Luz");
        assert_eq!(request_block(BLOCK_ADD, "Luz"), add.into_bytes());
        assert_eq!(request_block(2, "Luz"), ds(0xa9, &[2]));
    }

    #[test]
    fn macro_packets_keep_the_field_order_the_server_reads() {
        let entry = Macro {
            id: 3,
            name: "atk".into(),
            description: "bater".into(),
            acronym: "atk".into(),
            icon: 7,
            commands: vec![MacroCommand { index: 1, kind: 3, skill_id: 1177, shortcut_id: 0, command: "/atk".into() }],
        };
        let mut expected = Writer::new();
        expected.write_c(0xcd);
        expected.write_d(3);
        expected.write_s("atk");
        expected.write_s("bater");
        expected.write_s("atk");
        expected.write_c(7);
        expected.write_c(1);
        expected.write_c(1);
        expected.write_c(3);
        expected.write_d(1177);
        expected.write_c(0);
        expected.write_s("/atk");
        assert_eq!(request_make_macro(&entry), expected.into_bytes());
        assert_eq!(request_delete_macro(3), ds(0xce, &[3]));
    }

    #[test]
    fn give_nick_name_writes_target_then_title() {
        let mut expected = Writer::new();
        expected.write_c(0x0b);
        expected.write_s("Luz");
        expected.write_s("chefe");
        assert_eq!(request_give_nick_name("Luz", "chefe"), expected.into_bytes());
    }

    fn dq(op: u8, ints: &[i32], longs: &[i64]) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_c(op);
        for v in ints {
            w.write_d(*v);
        }
        for v in longs {
            w.write_q(*v);
        }
        w.into_bytes()
    }

    #[test]
    fn warehouse_and_package_count_fields_are_sixty_four_bits() {
        assert_eq!(send_warehouse_deposit_list(&[(7, 5)]), dq(0x3b, &[1, 7], &[5]));
        assert_eq!(send_warehouse_withdraw_list(&[(7, 5)]), dq(0x3c, &[1, 7], &[5]));
        assert_eq!(request_package_sendable_item_list(9), ds(0xa7, &[9]));
        assert_eq!(request_package_send(9, &[(7, 5)]), dq(0xa8, &[9, 1, 7], &[5]));
    }

    #[test]
    fn item_requests_follow_the_high_five_widths() {
        assert_eq!(request_crystallize_item(7, 5), dq(0x2f, &[7], &[5]));
        assert_eq!(request_enchant_item(7, 0), ds(0x5f, &[7, 0]));
        assert_eq!(request_preview_item(1, 2, &[10, 11]), ds(0xc7, &[1, 2, 2, 10, 11]));
        assert_eq!(multi_sell_choose(3, 4, 5), dq(0xb0, &[3, 4], &[5]));
    }

    #[test]
    fn private_store_packets_carry_sixty_four_bit_counts_and_prices() {
        assert_eq!(request_private_store_manage_sell(), [0x30]);
        assert_eq!(request_private_store_manage_buy(), [0x99]);
        assert_eq!(request_private_store_quit_sell(), [0x96]);
        assert_eq!(request_private_store_quit_buy(), [0x9c]);
        let offers = [StoreOffer { object_id: 7, count: 2, price: 300 }];
        assert_eq!(set_private_store_list_sell(true, &offers), dq(0x31, &[1, 1, 7], &[2, 300]));
        let buys = [BuyOffer { item_id: 57, enchant: 4, custom_type2: 0, count: 2, price: 300 }];
        assert_eq!(set_private_store_list_buy(&buys), dq(0x9a, &[1, 57, 4], &[2, 300]));
        let orders = [StoreBuyOrder { object_id: 7, count: 2, price: 300 }];
        assert_eq!(request_private_store_buy(11, &orders), dq(0x83, &[11, 1, 7], &[2, 300]));
        let items = [SellToStoreItem { object_id: 7, item_id: 57, enchant: 4, custom_type2: 0, count: 2, price: 300 }];
        assert_eq!(request_private_store_sell(11, &items), dq(0x9f, &[11, 1, 7, 57, 4], &[2, 300]));
        let mut expected = Writer::new();
        expected.write_c(0x97);
        expected.write_s("vendo");
        assert_eq!(set_private_store_msg_sell("vendo"), expected.into_bytes());
        let mut expected = Writer::new();
        expected.write_c(0x9d);
        expected.write_s("compro");
        assert_eq!(set_private_store_msg_buy("compro"), expected.into_bytes());
    }

    #[test]
    fn recipe_packets_use_the_high_five_opcodes_and_price_width() {
        assert_eq!(request_recipe_book_open(true), ds(0xb5, &[1]));
        assert_eq!(request_recipe_item_delete(42), ds(0xb6, &[42]));
        assert_eq!(request_recipe_item_make_info(42), ds(0xb7, &[42]));
        assert_eq!(request_recipe_item_make_self(42), ds(0xb8, &[42]));
        assert_eq!(request_recipe_shop_manage_list(), [0xb9]);
        assert_eq!(request_recipe_shop_manage_quit(), [0xbc]);
        assert_eq!(request_recipe_shop_manage_cancel(), [0xbd]);
        assert_eq!(request_recipe_shop_list_set(&[(42, 500)]), dq(0xbb, &[1, 42], &[500]));
        assert_eq!(request_recipe_shop_make_info(7, 42), ds(0xbe, &[7, 42]));
        assert_eq!(request_recipe_shop_make_do(7, 42), ds(0xbf, &[7, 42]));
        assert_eq!(request_recipe_shop_sell_list(), [0xc0]);
        let mut expected = Writer::new();
        expected.write_c(0xba);
        expected.write_s("faco tudo");
        assert_eq!(request_recipe_shop_message_set("faco tudo"), expected.into_bytes());
    }

    #[test]
    fn henna_packets_match_the_high_five_opcodes() {
        assert_eq!(request_henna_list(), [0xc3]);
        assert_eq!(request_henna_item_info(10), ds(0xc4, &[10]));
        assert_eq!(request_henna_equip(10), ds(0x6f, &[10]));
        assert_eq!(request_henna_unequip_list(), [0x70]);
        assert_eq!(request_henna_unequip_info(10), ds(0x71, &[10]));
        assert_eq!(request_henna_unequip(10), ds(0x72, &[10]));
    }

    #[test]
    fn pet_packets_split_plain_ids_from_sixty_four_bit_counts() {
        assert_eq!(request_pet_use_item(7), ds(0x94, &[7]));
        assert_eq!(request_pet_get_item(7), ds(0x98, &[7]));
        assert_eq!(request_give_item_to_pet(7, 5), dq(0x95, &[7], &[5]));
        assert_eq!(request_get_item_from_pet(7, 5), dq(0x2c, &[7], &[5]));
        let mut expected = Writer::new();
        expected.write_c(0x93);
        expected.write_s("Bicho");
        assert_eq!(request_change_pet_name("Bicho"), expected.into_bytes());
    }

    #[test]
    fn manor_packets_keep_the_extended_header_and_widen_the_amounts() {
        assert_eq!(request_buy_seed(1, &[(57, 5)]), dq(0xc5, &[1, 1, 57], &[5]));
        let mut expected = extended(0x03);
        expected.write_d(1);
        expected.write_d(1);
        expected.write_d(57);
        expected.write_q(5);
        expected.write_q(300);
        assert_eq!(request_set_seed(1, &[(57, 5, 300)]), expected.into_bytes());
        let mut expected = extended(0x04);
        expected.write_d(1);
        expected.write_d(1);
        expected.write_d(57);
        expected.write_q(5);
        expected.write_q(300);
        expected.write_c(2);
        assert_eq!(
            request_set_crop(1, &[CropSetting { crop_id: 57, amount: 5, price: 300, reward: 2 }]),
            expected.into_bytes()
        );
        let mut expected = extended(0x02);
        expected.write_d(1);
        expected.write_d(7);
        expected.write_d(57);
        expected.write_d(1);
        expected.write_q(5);
        assert_eq!(
            request_procure_crop_list(&[CropProcure { object_id: 7, crop_id: 57, manor_id: 1, count: 5 }]),
            expected.into_bytes()
        );
    }
}
