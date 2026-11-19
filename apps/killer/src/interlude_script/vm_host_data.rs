use std::collections::HashMap;

use killer_assets::client::{Client, DatTables};
use killer_ui::color::Argb;
use l2_uscript::vm::{StructValue, Value};

use super::host_impl::ScriptHost;
use super::events::param_add_int;
use super::vm_host::{color_value, int64_value, pet_info_value, user_info_value};
use super::{HennaInfo, ItemInfo, SkillInfo, UserInfo, Vector};

pub const SYSTEMMSG_FILE: &str = "systemmsg-e.dat";
pub const QUESTNAME_FILE: &str = "questname-e.dat";
pub const ITEM_DATA_WEAPON: i32 = 0;
pub const ITEM_DATA_ARMOR: i32 = 1;
pub const ITEM_DATA_ETC: i32 = 2;
pub const TARGET_COLOR_FAR_BELOW: Argb = Argb(0xFFFF_0000);
pub const TARGET_COLOR_BELOW: Argb = Argb(0xFFFF_9191);
pub const TARGET_COLOR_SLIGHTLY_BELOW: Argb = Argb(0xFFFA_FE91);
pub const TARGET_COLOR_EVEN: Argb = Argb(0xFFDC_DCDC);
pub const TARGET_COLOR_SLIGHTLY_ABOVE: Argb = Argb(0xFFA2_FFAB);
pub const TARGET_COLOR_ABOVE: Argb = Argb(0xFF00_FEFF);
pub const TARGET_COLOR_FAR_ABOVE: Argb = Argb(0xFFA2_A6FC);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ClanData {
    pub name: String,
    pub alliance_name: String,
    pub crest_texture: Option<String>,
    pub alliance_crest_texture: Option<String>,
    pub emblem_texture: Option<String>,
    pub name_value: Option<i32>,
    pub clan_type: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StaticObjectData {
    pub name_id: i32,
    pub kind: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub show_hp: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct GameTipData {
    pub id: i32,
    pub priority: i32,
    pub target_level: i32,
    pub validity: bool,
    pub tip: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MacroData {
    pub id: i32,
    pub name: String,
    pub icon_name: String,
    pub icon_texture_name: String,
    pub description: String,
    pub commands: Vec<String>,
}

pub fn target_name_color(level_diff: i32) -> Argb {
    if level_diff < -8 {
        TARGET_COLOR_FAR_BELOW
    } else if level_diff < -5 {
        TARGET_COLOR_BELOW
    } else if level_diff < -2 {
        TARGET_COLOR_SLIGHTLY_BELOW
    } else if level_diff < 3 {
        TARGET_COLOR_EVEN
    } else if level_diff < 6 {
        TARGET_COLOR_SLIGHTLY_ABOVE
    } else if level_diff < 9 {
        TARGET_COLOR_ABOVE
    } else {
        TARGET_COLOR_FAR_ABOVE
    }
}

pub fn exp_rate(exp: i64, level: i32) -> f32 {
    match killer_ui::interlude_exp::level_bounds(level) {
        Some((low, high)) if high > low => ((exp - low) as f64 / (high - low) as f64) as f32,
        _ => 0.0,
    }
}

fn read_dat(client: &Client, file: &str) -> Option<Vec<u8>> {
    let path = client.root().join("system").join(file);
    let raw = std::fs::read(&path).ok()?;
    l2_crypto::decrypt(&raw, &path).ok().map(|plain| plain.into_owned())
}

fn vector_value(v: Vector) -> Value {
    Value::vector(v.x, v.y, v.z)
}

fn vector_of(loc: [f32; 3]) -> Value {
    Value::vector(loc[0], loc[1], loc[2])
}

fn out(args: &mut [Value], index: usize, value: Value) {
    if let Some(slot) = args.get_mut(index) {
        *slot = value;
    }
}

pub fn item_info_value(info: &ItemInfo) -> Value {
    let s = |name: &str, value: &str| (name.to_string(), Value::Str(value.to_string()));
    let i = |name: &str, value: i32| (name.to_string(), Value::Int(value));
    Value::Struct(Box::new(StructValue {
        name: "ItemInfo".into(),
        fields: vec![
            s("Name", &info.name),
            s("AdditionalName", &info.additional_name),
            s("IconName", &info.icon_name),
            s("IconNameEx1", &info.icon_name_ex1),
            s("IconNameEx2", &info.icon_name_ex2),
            s("IconNameEx3", &info.icon_name_ex3),
            s("IconNameEx4", &info.icon_name_ex4),
            s("ForeTexture", &info.fore_texture),
            s("Description", &info.description),
            s("DragSrcName", &info.drag_src_name),
            i("DragSrcReserved", info.drag_src_reserved),
            s("MacroCommand", &info.macro_command),
            i("ItemType", info.item_type),
            i("ItemSubType", info.item_sub_type),
            i("ServerID", info.server_id),
            i("ClassID", info.class_id),
            i("ItemNum", info.item_num),
            i("Price", info.price),
            i("Level", info.level),
            i("SlotBitType", info.slot_bit_type),
            i("Weight", info.weight),
            i("MaterialType", info.material_type),
            i("WeaponType", info.weapon_type),
            i("PhysicalDamage", info.physical_damage),
            i("MagicalDamage", info.magical_damage),
            i("PhysicalDefense", info.physical_defense),
            i("MagicalDefense", info.magical_defense),
            i("ShieldDefense", info.shield_defense),
            i("ShieldDefenseRate", info.shield_defense_rate),
            i("Durability", info.durability),
            i("CrystalType", info.crystal_type),
            i("RandomDamage", info.random_damage),
            i("Critical", info.critical),
            i("HitModify", info.hit_modify),
            i("AttackSpeed", info.attack_speed),
            i("MpConsume", info.mp_consume),
            i("ArmorType", info.armor_type),
            i("AvoidModify", info.avoid_modify),
            i("Damaged", info.damaged),
            i("Enchanted", info.enchanted),
            i("MpBonus", info.mp_bonus),
            i("SoulshotCount", info.soulshot_count),
            i("SpiritshotCount", info.spiritshot_count),
            i("PopMsgNum", info.pop_msg_num),
            i("BodyPart", info.body_part),
            i("RefineryOp1", info.refinery_op1),
            i("RefineryOp2", info.refinery_op2),
            i("CurrentDurability", info.current_durability),
            i("Reserved", info.reserved),
            i("DefaultPrice", info.default_price),
            i("ConsumeType", info.consume_type),
            i("Blessed", info.blessed),
        ],
    }))
}

fn skill_info_value(info: &SkillInfo) -> Value {
    Value::Struct(Box::new(StructValue {
        name: "SkillInfo".into(),
        fields: vec![
            ("SkillName".into(), Value::Str(info.skill_name.clone())),
            ("SkillDesc".into(), Value::Str(info.skill_desc.clone())),
            ("SkillID".into(), Value::Int(info.skill_id)),
            ("SkillLevel".into(), Value::Int(info.skill_level)),
            ("OperateType".into(), Value::Int(info.operate_type)),
            ("MpConsume".into(), Value::Int(info.mp_consume)),
            ("HpConsume".into(), Value::Int(info.hp_consume)),
            ("CastRange".into(), Value::Int(info.cast_range)),
            ("CastStyle".into(), Value::Int(info.cast_style)),
            ("HitTime".into(), Value::Float(info.hit_time)),
            ("IsUsed".into(), Value::Bool(info.is_used)),
            ("IsMagic".into(), Value::Int(info.is_magic)),
            ("AnimName".into(), Value::Str(info.anim_name.clone())),
            ("SkillPresetName".into(), Value::Str(info.skill_preset_name.clone())),
            ("TexName".into(), Value::Str(info.tex_name.clone())),
            ("IsDebuff".into(), Value::Bool(info.is_debuff)),
            ("EnchantName".into(), Value::Str(info.enchant_name.clone())),
            ("EnchantDesc".into(), Value::Str(info.enchant_desc.clone())),
            ("Enchanted".into(), Value::Int(info.enchanted)),
            ("EnchantSkillLevel".into(), Value::Int(info.enchant_skill_level)),
            ("RumbleSelf".into(), Value::Int(info.rumble_self)),
            ("RumbleTarget".into(), Value::Int(info.rumble_target)),
        ],
    }))
}

fn henna_value(info: &HennaInfo) -> Value {
    let int = |name: &str, value: i32| (name.to_string(), Value::Int(value));
    Value::Struct(Box::new(StructValue {
        name: "HennaInfo".into(),
        fields: vec![
            int("HennaID", info.henna_id),
            int("ClassID", info.class_id),
            int("Num", info.num),
            int("Fee", info.fee),
            int("CanUse", info.can_use),
            int("INTnow", info.int_now),
            int("INTchange", info.int_change),
            int("STRnow", info.str_now),
            int("STRchange", info.str_change),
            int("CONnow", info.con_now),
            int("CONchange", info.con_change),
            int("MENnow", info.men_now),
            int("MENchange", info.men_change),
            int("DEXnow", info.dex_now),
            int("DEXchange", info.dex_change),
            int("WITnow", info.wit_now),
            int("WITchange", info.wit_change),
        ],
    }))
}

fn game_tip_value(tip: &GameTipData) -> Value {
    Value::Struct(Box::new(StructValue {
        name: "GameTipData".into(),
        fields: vec![
            ("Id".into(), Value::Int(tip.id)),
            ("Priority".into(), Value::Int(tip.priority)),
            ("TargetLevel".into(), Value::Int(tip.target_level)),
            ("Validity".into(), Value::Bool(tip.validity)),
            ("TipMsg".into(), Value::Str(tip.tip.clone())),
        ],
    }))
}

fn macro_value(info: &MacroData) -> Value {
    let mut commands: Vec<Value> = info.commands.iter().map(|command| Value::Str(command.clone())).collect();
    commands.resize(12, Value::Str(String::new()));
    Value::Struct(Box::new(StructValue {
        name: "MacroInfo".into(),
        fields: vec![
            ("Id".into(), Value::Int(info.id)),
            ("Name".into(), Value::Str(info.name.clone())),
            ("IconName".into(), Value::Str(info.icon_name.clone())),
            ("IconTextureName".into(), Value::Str(info.icon_texture_name.clone())),
            ("Description".into(), Value::Str(info.description.clone())),
            ("CommandList".into(), Value::Array(commands)),
        ],
    }))
}

impl ScriptHost {
    pub fn load_data(&mut self, client: &mut Client) {
        if let Some(body) = read_dat(client, SYSTEMMSG_FILE)
            && let Ok(rows) = l2_dat::systemmsg::read_interlude(&body)
        {
            self.own_systemmsg = rows.into_iter().map(|(message, _)| (message.id, message.text)).collect();
        }
        if let Some(body) = read_dat(client, QUESTNAME_FILE)
            && let Ok(rows) = l2_dat::questname::read_interlude(&body)
        {
            for (quest, extra) in rows {
                self.quest_zones.insert((quest.id, quest.level), extra.extra[0]);
                self.own_quests.insert((quest.id, quest.level), quest);
            }
        }
    }

    pub fn attach_tables(&mut self, tables: std::sync::Arc<DatTables>) {
        self.tables = Some(tables);
    }

    fn tables(&self) -> Option<&DatTables> {
        self.tables.as_deref()
    }

    fn sorted_ids<I: Ord + Copy>(ids: impl Iterator<Item = I>) -> Vec<I> {
        let mut ids: Vec<I> = ids.collect();
        ids.sort_unstable();
        ids
    }

    fn item_ids(&self) -> Vec<u32> {
        self.tables().map_or_else(Vec::new, |tables| Self::sorted_ids(tables.itemname.keys().copied()))
    }

    fn skill_ids(&self) -> Vec<u32> {
        self.tables().map_or_else(Vec::new, |tables| {
            let mut ids: Vec<u32> = tables.skillname.keys().map(|(id, _)| *id).collect();
            ids.sort_unstable();
            ids.dedup();
            ids
        })
    }

    fn npc_ids(&self) -> Vec<u32> {
        self.tables().map_or_else(Vec::new, |tables| Self::sorted_ids(tables.npcname.keys().copied()))
    }

    fn quest_ids(&self) -> Vec<i32> {
        let mut ids: Vec<i32> = self.own_quests.keys().map(|(id, _)| *id).collect();
        if ids.is_empty()
            && let Some(tables) = self.tables()
        {
            ids = tables.questname.keys().map(|(id, _)| *id).collect();
        }
        ids.sort_unstable();
        ids.dedup();
        ids
    }

    fn cursor_first(ids: &[impl Copy + Into<i64>], cursor: &mut usize) -> Value {
        *cursor = 0;
        ids.first().map_or(Value::Int(-1), |id| Value::Int((*id).into() as i32))
    }

    fn cursor_next(ids: &[impl Copy + Into<i64>], cursor: &mut usize) -> Value {
        *cursor += 1;
        ids.get(*cursor).map_or(Value::Int(-1), |id| Value::Int((*id).into() as i32))
    }

    fn quest(&self, id: i32, level: i32) -> Option<l2_dat::questname::QuestName> {
        if let Some(quest) = self.own_quests.get(&(id, level)) {
            return Some(quest.clone());
        }
        self.tables().and_then(|tables| tables.questname.get(&(id, level)).cloned())
    }

    fn quest_any_level(&self, id: i32) -> Option<l2_dat::questname::QuestName> {
        let mut found: Vec<&l2_dat::questname::QuestName> = self.own_quests.values().filter(|quest| quest.id == id).collect();
        if found.is_empty()
            && let Some(tables) = self.tables()
        {
            found = tables.questname.values().filter(|quest| quest.id == id).collect();
        }
        found.sort_by_key(|quest| quest.level);
        found.first().map(|quest| (*quest).clone())
    }

    pub fn item_data_type(&self, class_id: u32) -> Option<i32> {
        let tables = self.tables()?;
        if tables.weapongrp.contains_key(&class_id) {
            Some(ITEM_DATA_WEAPON)
        } else if tables.armorgrp.contains_key(&class_id) {
            Some(ITEM_DATA_ARMOR)
        } else if tables.etcitemgrp.contains_key(&class_id) {
            Some(ITEM_DATA_ETC)
        } else {
            None
        }
    }

    pub fn item_crystallizable(&self, class_id: u32) -> bool {
        self.tables().is_some_and(|tables| {
            tables.weapongrp.get(&class_id).is_some_and(|weapon| weapon.crystallizable)
                || tables.armorgrp.get(&class_id).is_some_and(|armor| armor.crystallizable)
                || tables.etcitemgrp.get(&class_id).is_some_and(|etc| etc.crystallizable)
        })
    }

    pub fn item_weight(&self, class_id: u32) -> i32 {
        self.tables().and_then(|tables| tables.etcitemgrp.get(&class_id)).map_or(0, |etc| etc.weight as i32)
    }

    pub fn item_crystal_type(&self, class_id: u32) -> i32 {
        self.tables().and_then(|tables| tables.etcitemgrp.get(&class_id)).map_or(0, |etc| etc.grade as i32)
    }

    pub fn item_texture(&self, class_id: u32) -> String {
        self.tables().and_then(|tables| tables.item_icons.texture(class_id)).unwrap_or_default()
    }

    pub fn item_info_of(&self, class_id: u32) -> Option<ItemInfo> {
        let tables = self.tables()?;
        let name = tables.itemname.get(&class_id)?;
        let item_type = self.item_data_type(class_id).unwrap_or(ITEM_DATA_ETC);
        Some(ItemInfo {
            name: name.name.clone(),
            additional_name: name.add_name.clone(),
            icon_name: self.item_texture(class_id),
            description: name.description.clone(),
            item_type,
            class_id: class_id as i32,
            weight: self.item_weight(class_id),
            crystal_type: self.item_crystal_type(class_id),
            durability: -1,
            ..Default::default()
        })
    }

    pub fn skill_info_of(&self, id: u32, level: u32) -> Option<SkillInfo> {
        let tables = self.tables()?;
        let name = tables.skillname.get(&(id, level));
        let skill = tables.skillgrp.lookup(id, level);
        if name.is_none() && skill.is_none() {
            return None;
        }
        Some(SkillInfo {
            skill_name: name.map(|n| n.name.clone()).unwrap_or_default(),
            skill_desc: name.map(|n| n.description.clone()).unwrap_or_default(),
            skill_id: id as i32,
            skill_level: level as i32,
            operate_type: skill.map_or(0, |s| s.oper_type as i32),
            mp_consume: skill.map_or(0, |s| s.mp_consume as i32),
            hp_consume: skill.map_or(0, |s| s.hp_consume as i32),
            cast_range: skill.map_or(0, |s| s.cast_range),
            cast_style: skill.map_or(0, |s| s.cast_style as i32),
            hit_time: skill.map_or(0.0, |s| s.hit_time),
            is_magic: skill.map_or(0, |s| s.is_magic),
            anim_name: skill.map(|s| s.ani_char.clone()).unwrap_or_default(),
            skill_preset_name: skill.map(|s| s.visual_effect.clone()).unwrap_or_default(),
            tex_name: skill.map(|s| s.icon.clone()).unwrap_or_default(),
            is_debuff: skill.is_some_and(l2_dat::skillgrp::Skill::is_debuff),
            enchant_name: name.map(|n| n.desc_add1.clone()).unwrap_or_default(),
            enchant_desc: name.map(|n| n.desc_add2.clone()).unwrap_or_default(),
            enchanted: skill.map_or(0, |s| s.is_ench as i32),
            enchant_skill_level: skill.map_or(0, |s| s.ench_skill_id as i32),
            rumble_self: skill.map_or(0, |s| s.rumble_self),
            rumble_target: skill.map_or(0, |s| s.rumble_target),
            ..Default::default()
        })
    }

    pub fn target_npc_shows_bars(&self) -> bool {
        let Some(target) = self.target.as_ref().filter(|target| target.npc) else { return false };
        self.tables().and_then(|tables| u32::try_from(target.class_id).ok().and_then(|id| tables.npcgrp.get(id))).is_some_and(|npc| npc.shows_bars != 0)
    }

    pub fn system_message_text(&self, id: u32) -> String {
        if let Some(text) = self.own_systemmsg.get(&id) {
            return text.clone();
        }
        self.tables().and_then(|tables| tables.systemmsg.get(&id)).map(|message| message.text.clone()).unwrap_or_default()
    }

    pub fn user_name_of(&self, server_id: i32) -> String {
        if let Some(player) = self.player.as_ref().filter(|player| player.id == server_id) {
            return player.name.clone();
        }
        if let Some(target) = self.target.as_ref().filter(|target| target.id == server_id) {
            return target.name.clone();
        }
        self.users.get(&server_id).map(|user| user.name.clone()).unwrap_or_default()
    }

    pub fn user_info_of(&self, server_id: i32) -> Option<UserInfo> {
        self.player
            .as_ref()
            .filter(|player| player.id == server_id)
            .or_else(|| self.target.as_ref().filter(|target| target.id == server_id))
            .cloned()
            .or_else(|| self.users.get(&server_id).cloned())
    }

    fn quest_item_param(quest: &l2_dat::questname::QuestName) -> String {
        let mut param = String::new();
        param_add_int(&mut param, "Max", &quest.item_ids.len().to_string());
        for (index, (id, num)) in quest.item_ids.iter().zip(quest.item_nums.iter().chain(std::iter::repeat(&0))).enumerate() {
            param_add_int(&mut param, &format!("ItemID_{index}"), &id.to_string());
            param_add_int(&mut param, &format!("ItemNum_{index}"), &num.to_string());
        }
        param
    }

    #[allow(clippy::too_many_lines)]
    pub fn data_native(&mut self, class: &str, function: &str, args: &mut [Value]) -> Option<Value> {
        let int = |index: usize| args.get(index).map_or(0, Value::as_int);
        let text = |index: usize| args.get(index).map_or_else(String::new, Value::as_str);
        let quest_of = |host: &Self| host.quest(int(0), int(1)).or_else(|| host.quest_any_level(int(0)));
        Some(match (class, function) {
            ("UIDATA_ITEM", "GetFirstID") => {
                let ids = self.item_ids();
                Self::cursor_first(&ids, &mut self.item_cursor)
            }
            ("UIDATA_ITEM", "GetNextID") => {
                let ids = self.item_ids();
                Self::cursor_next(&ids, &mut self.item_cursor)
            }
            ("UIDATA_ITEM", "GetDataCount") => Value::Int(self.item_ids().len() as i32),
            ("UIDATA_ITEM", "GetItemName") => Value::Str(self.tables().and_then(|t| t.itemname.get(&(int(0) as u32))).map(|n| n.name.clone()).unwrap_or_default()),
            ("UIDATA_ITEM", "GetItemAdditionalName") => {
                Value::Str(self.tables().and_then(|t| t.itemname.get(&(int(0) as u32))).map(|n| n.add_name.clone()).unwrap_or_default())
            }
            ("UIDATA_ITEM", "GetItemDescription") => {
                Value::Str(self.tables().and_then(|t| t.itemname.get(&(int(0) as u32))).map(|n| n.description.clone()).unwrap_or_default())
            }
            ("UIDATA_ITEM", "GetItemTextureName") => Value::Str(self.item_texture(int(0) as u32)),
            ("UIDATA_ITEM", "GetItemClassID") => Value::Int(if self.tables().is_some_and(|t| t.itemname.contains_key(&(int(0) as u32))) { int(0) } else { 0 }),
            ("UIDATA_ITEM", "GetItemWeight") => Value::Int(self.item_weight(int(0) as u32)),
            ("UIDATA_ITEM", "GetItemDataType") => Value::Int(self.item_data_type(int(0) as u32).unwrap_or(0)),
            ("UIDATA_ITEM", "GetItemCrystalType") => Value::Int(self.item_crystal_type(int(0) as u32)),
            ("UIDATA_ITEM", "IsCrystallizable") => Value::Bool(self.item_crystallizable(int(0) as u32)),
            ("UIDATA_ITEM", "GetItemInfo") => match self.item_info_of(int(0) as u32) {
                Some(info) => {
                    out(args, 1, item_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIDATA_ITEM", "GetRefineryItemName") => Value::Str(text(0)),
            ("UIDATA_SKILL", "GetFirstID") => {
                let ids = self.skill_ids();
                Self::cursor_first(&ids, &mut self.skill_cursor)
            }
            ("UIDATA_SKILL", "GetNextID") => {
                let ids = self.skill_ids();
                Self::cursor_next(&ids, &mut self.skill_cursor)
            }
            ("UIDATA_SKILL", "GetDataCount") => Value::Int(self.skill_ids().len() as i32),
            ("UIDATA_SKILL", "GetName") => Value::Str(self.tables().and_then(|t| t.skillname.get(&(int(0) as u32, int(1) as u32))).map(|n| n.name.clone()).unwrap_or_default()),
            ("UIDATA_SKILL", "GetDescription") => {
                Value::Str(self.tables().and_then(|t| t.skillname.get(&(int(0) as u32, int(1) as u32))).map(|n| n.description.clone()).unwrap_or_default())
            }
            ("UIDATA_SKILL", "GetIconName") => Value::Str(self.tables().and_then(|t| t.skillgrp.lookup(int(0) as u32, int(1) as u32)).map(|s| s.icon.clone()).unwrap_or_default()),
            ("UIDATA_SKILL", "GetMpConsume") => Value::Int(self.tables().and_then(|t| t.skillgrp.lookup(int(0) as u32, int(1) as u32)).map_or(0, |s| s.mp_consume as i32)),
            ("UIDATA_SKILL", "GetHpConsume") => Value::Int(self.tables().and_then(|t| t.skillgrp.lookup(int(0) as u32, int(1) as u32)).map_or(0, |s| s.hp_consume as i32)),
            ("UIDATA_SKILL", "GetCastRange") => Value::Int(self.tables().and_then(|t| t.skillgrp.lookup(int(0) as u32, int(1) as u32)).map_or(0, |s| s.cast_range)),
            ("UIDATA_SKILL", "GetOperateType") => Value::Str(self.tables().and_then(|t| t.skillgrp.lookup(int(0) as u32, int(1) as u32)).map(|s| s.oper_type.to_string()).unwrap_or_default()),
            ("UIDATA_SKILL", "GetEnchantName") => Value::Str(self.tables().and_then(|t| t.skillname.get(&(int(0) as u32, int(1) as u32))).map(|n| n.desc_add1.clone()).unwrap_or_default()),
            ("UIDATA_SKILL", "GetEnchantSkillLevel") => Value::Int(self.tables().and_then(|t| t.skillgrp.lookup(int(0) as u32, int(1) as u32)).map_or(0, |s| s.ench_skill_id as i32)),
            ("UIDATA_NPC", "GetFirstID") => {
                let ids = self.npc_ids();
                Self::cursor_first(&ids, &mut self.npc_cursor)
            }
            ("UIDATA_NPC", "GetNextID") => {
                let ids = self.npc_ids();
                Self::cursor_next(&ids, &mut self.npc_cursor)
            }
            ("UIDATA_NPC", "IsValidData") => Value::Bool(self.tables().is_some_and(|t| t.npcname.contains_key(&(int(0) as u32)))),
            ("UIDATA_NPC", "GetNPCName") => Value::Str(self.tables().and_then(|t| t.npcname.get(&(int(0) as u32))).map(|n| n.name.clone()).unwrap_or_default()),
            ("UIDATA_NPC", "GetNpcProperty") => Value::Bool(false),
            ("UIDATA_QUEST", "GetFirstID") => {
                let ids = self.quest_ids();
                Self::cursor_first(&ids, &mut self.quest_cursor)
            }
            ("UIDATA_QUEST", "GetNextID") => {
                let ids = self.quest_ids();
                Self::cursor_next(&ids, &mut self.quest_cursor)
            }
            ("UIDATA_QUEST", "IsValidData") => Value::Bool(self.quest_any_level(int(0)).is_some()),
            ("UIDATA_QUEST", "GetQuestName") => Value::Str(self.quest_any_level(int(0)).map(|q| q.title).unwrap_or_default()),
            ("UIDATA_QUEST", "GetQuestJournalName") => Value::Str(quest_of(self).map(|q| q.sub_name).unwrap_or_default()),
            ("UIDATA_QUEST", "GetQuestDescription") => Value::Str(quest_of(self).map(|q| q.description).unwrap_or_default()),
            ("UIDATA_QUEST", "GetIntro") => Value::Str(quest_of(self).map(|q| q.intro).unwrap_or_default()),
            ("UIDATA_QUEST", "GetRequirement") => Value::Str(quest_of(self).map(|q| q.requirement).unwrap_or_default()),
            ("UIDATA_QUEST", "GetMinLevel") => Value::Int(quest_of(self).map_or(0, |q| q.level)),
            ("UIDATA_QUEST", "GetMaxLevel") => Value::Int(quest_of(self).map_or(0, |q| q.quest_level)),
            ("UIDATA_QUEST", "GetQuestType") => Value::Int(quest_of(self).map_or(0, |q| q.quest_type)),
            ("UIDATA_QUEST", "GetQuestZone") => Value::Int(self.quest_zones.get(&(int(0), int(1))).copied().unwrap_or(0)),
            ("UIDATA_QUEST", "GetStartNPCID") => Value::Int(quest_of(self).map_or(0, |q| q.start_npc_id)),
            ("UIDATA_QUEST", "GetStartNPCLoc") => quest_of(self).map_or(Value::vector(0.0, 0.0, 0.0), |q| vector_of(q.start_npc_loc)),
            ("UIDATA_QUEST", "GetTargetLoc") => quest_of(self).map_or(Value::vector(0.0, 0.0, 0.0), |q| vector_of(q.target_loc)),
            ("UIDATA_QUEST", "GetTargetName") => Value::Str(quest_of(self).map(|q| q.target_name).unwrap_or_default()),
            ("UIDATA_QUEST", "IsMinimapOnly") => Value::Bool(quest_of(self).is_some_and(|q| q.minimap_only != 0)),
            ("UIDATA_QUEST", "IsShowableJournalQuest") => Value::Bool(quest_of(self).is_some_and(|q| q.journal_disp != 0)),
            ("UIDATA_QUEST", "IsShowableItemNumQuest") => Value::Bool(quest_of(self).is_some_and(|q| !q.item_ids.is_empty())),
            ("UIDATA_QUEST", "GetQuestItem") => Value::Str(quest_of(self).map(|q| Self::quest_item_param(&q)).unwrap_or_default()),
            ("UIDATA_QUEST", "GetClearedQuest") => Value::Int(0),
            ("UIDATA_PET", "GetPetID") => Value::Int(self.pet.as_ref().map_or(0, |p| p.id)),
            ("UIDATA_PET", "GetPetName") => Value::Str(self.pet.as_ref().map(|p| p.name.clone()).unwrap_or_default()),
            ("UIDATA_PET", "GetPetHP") => Value::Int(self.pet.as_ref().map_or(0, |p| p.cur_hp)),
            ("UIDATA_PET", "GetPetMaxHP") => Value::Int(self.pet.as_ref().map_or(0, |p| p.max_hp)),
            ("UIDATA_PET", "GetPetMP") => Value::Int(self.pet.as_ref().map_or(0, |p| p.cur_mp)),
            ("UIDATA_PET", "GetPetMaxMP") => Value::Int(self.pet.as_ref().map_or(0, |p| p.max_mp)),
            ("UIDATA_PET", "GetPetSP") => Value::Int(self.pet.as_ref().map_or(0, |p| p.sp)),
            ("UIDATA_PET", "GetPetLevel") => Value::Int(self.pet.as_ref().map_or(0, |p| p.level)),
            ("UIDATA_PET", "GetPetFatigue") => Value::Int(self.pet.as_ref().map_or(0, |p| p.fatigue)),
            ("UIDATA_PET", "GetPetMaxFatigue") => Value::Int(self.pet.as_ref().map_or(0, |p| p.max_fatigue)),
            ("UIDATA_PET", "GetPetCarryWeight") => Value::Int(self.pet.as_ref().map_or(0, |p| p.carry_weight)),
            ("UIDATA_PET", "GetPetCarringWeight") => Value::Int(self.pet.as_ref().map_or(0, |p| p.carring_weight)),
            ("UIDATA_PET", "GetPetEXP") => int64_value(self.pet.as_ref().map_or(0, |p| p.cur_exp)),
            ("UIDATA_PET", "GetPetMaxEXP") => int64_value(self.pet.as_ref().map_or(0, |p| p.max_exp)),
            ("UIDATA_PET", "GetPetMinEXP") => int64_value(self.pet.as_ref().map_or(0, |p| p.min_exp)),
            ("UIDATA_PET", "GetPetEXPRate") => Value::Float(self.pet.as_ref().map_or(0.0, |p| {
                if p.max_exp > p.min_exp { ((p.cur_exp - p.min_exp) as f64 / (p.max_exp - p.min_exp) as f64) as f32 } else { 0.0 }
            })),
            ("UIDATA_PET", "GetIsPetOrSummoned") => Value::Int(i32::from(self.pet.as_ref().is_some_and(|p| p.is_pet_or_summoned))),
            ("UIDATA_PLAYER", "GetPlayerID") => Value::Int(self.player.as_ref().map_or(0, |p| p.id)),
            ("UIDATA_PLAYER", "IsHero") => Value::Bool(self.player.as_ref().is_some_and(|p| p.hero)),
            ("UIDATA_PLAYER", "GetPlayerEXPRate") => Value::Float(self.player.as_ref().map_or(0.0, |p| exp_rate(p.cur_exp, p.level))),
            ("UIDATA_PLAYER", "GetInventoryLimit") => Value::Int(self.inventory_limit),
            ("UIDATA_PLAYER", "HasCrystallizeAbility") => Value::Bool(self.crystallize_ability),
            ("UIDATA_PLAYER", "GetRecipeShopMsg") => Value::Str(self.recipe_shop_msg.clone()),
            ("UIDATA_PLAYER", "GetPlayerMoveType") => Value::Int(self.player_move_type),
            ("UIDATA_PLAYER", "GetPlayerEnvironment") => Value::Int(self.player_environment),
            ("UIDATA_TARGET", "GetTargetID") => Value::Int(self.target.as_ref().map_or(0, |t| t.id)),
            ("UIDATA_TARGET", "GetTargetUserRank") => Value::Int(self.target.as_ref().map_or(0, |t| t.user_rank)),
            ("UIDATA_TARGET", "GetTargetName") => Value::Str(self.target.as_ref().map(|t| t.name.clone()).unwrap_or_default()),
            ("UIDATA_TARGET", "GetTargetHP") => Value::Int(self.target.as_ref().map_or(0, |t| t.cur_hp)),
            ("UIDATA_TARGET", "GetTargetMaxHP") => Value::Int(self.target.as_ref().map_or(0, |t| t.max_hp)),
            ("UIDATA_TARGET", "GetTargetMP") => Value::Int(self.target.as_ref().map_or(0, |t| t.cur_mp)),
            ("UIDATA_TARGET", "GetTargetMaxMP") => Value::Int(self.target.as_ref().map_or(0, |t| t.max_mp)),
            ("UIDATA_TARGET", "GetTargetPledgeID") => Value::Int(self.target.as_ref().map_or(0, |t| t.clan_id)),
            ("UIDATA_TARGET", "GetTargetClassID") => Value::Int(self.target.as_ref().map_or(0, |t| t.class_id)),
            ("UIDATA_TARGET", "GetTargetNameColor") => color_value(target_name_color(int(0))),
            ("UIDATA_TARGET", "IsNpc") => Value::Bool(self.target.as_ref().is_some_and(|t| t.npc)),
            ("UIDATA_TARGET", "IsPet") => Value::Bool(self.target.as_ref().is_some_and(|t| t.pet) || self.target_pet),
            ("UIDATA_TARGET", "IsCanBeAttacked") => Value::Bool(self.target.as_ref().is_some_and(|t| t.can_be_attacked) || self.target_can_be_attacked),
            ("UIDATA_TARGET", "IsServerObject") => Value::Bool(self.target_static_object.is_some()),
            ("UIDATA_TARGET", "IsHPShowableNPC") => Value::Bool(self.target_hp_showable || self.target_npc_shows_bars()),
            ("UIDATA_USER", "GetUserName") => Value::Str(self.user_name_of(int(0))),
            ("UIDATA_USER", "GetClanType") => match self.clans.get(&int(0)).and_then(|clan| clan.clan_type) {
                Some(kind) => {
                    out(args, 1, Value::Int(kind));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIDATA_PARTY", "GetMemberName") => Value::Str(self.user_name_of(int(0))),
            ("UIDATA_CLAN", "GetName") => Value::Str(self.clans.get(&int(0)).map(|c| c.name.clone()).unwrap_or_default()),
            ("UIDATA_CLAN", "GetAllianceName") => Value::Str(self.clans.get(&int(0)).map(|c| c.alliance_name.clone()).unwrap_or_default()),
            ("UIDATA_CLAN", "GetNameValue") => match self.clans.get(&int(0)).and_then(|c| c.name_value) {
                Some(value) => {
                    out(args, 1, Value::Int(value));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIDATA_CLAN", "GetCrestTexture") => Value::Bool(self.clans.get(&int(0)).is_some_and(|c| c.crest_texture.is_some())),
            ("UIDATA_CLAN", "GetAllianceCrestTexture") => Value::Bool(self.clans.get(&int(0)).is_some_and(|c| c.alliance_crest_texture.is_some())),
            ("UIDATA_CLAN", "GetEmblemTexture") => Value::Bool(self.clans.get(&int(0)).is_some_and(|c| c.emblem_texture.is_some())),
            ("UIDATA_STATICOBJECT", "GetServerObjectNameID") => Value::Int(self.static_objects.get(&int(0)).map_or(0, |o| o.name_id)),
            ("UIDATA_STATICOBJECT", "GetServerObjectType") => Value::Int(self.static_objects.get(&int(0)).map_or(0, |o| o.kind)),
            ("UIDATA_STATICOBJECT", "GetServerObjectHP") => Value::Int(self.static_objects.get(&int(0)).map_or(0, |o| o.hp)),
            ("UIDATA_STATICOBJECT", "GetServerObjectMaxHP") => Value::Int(self.static_objects.get(&int(0)).map_or(0, |o| o.max_hp)),
            ("UIDATA_STATICOBJECT", "GetStaticObjectShowHP") => Value::Bool(self.static_objects.get(&int(0)).is_some_and(|o| o.show_hp)),
            ("UIDATA_STATICOBJECT", "GetStaticObjectName") => Value::Str(self.tables().and_then(|t| t.staticobject.get(&(int(0) as u32)).cloned()).unwrap_or_default()),
            ("UIDATA_RECIPE", "GetRecipeClassID") => Value::Int(self.tables().and_then(|t| t.recipes.get(&int(0))).map_or(0, |r| r.id)),
            ("UIDATA_RECIPE", "GetRecipeIconName") => Value::Str(self.tables().and_then(|t| t.recipes.get(&int(0))).map(|r| self.item_texture(r.id as u32)).unwrap_or_default()),
            ("UIDATA_RECIPE", "GetRecipeProductID") => Value::Int(self.tables().and_then(|t| t.recipes.get(&int(0))).map_or(0, |r| r.product_id)),
            ("UIDATA_RECIPE", "GetRecipeProductNum") => Value::Int(self.tables().and_then(|t| t.recipes.get(&int(0))).map_or(0, |r| r.product_num)),
            ("UIDATA_RECIPE", "GetRecipeCrystalType") => Value::Int(self.tables().and_then(|t| t.recipes.get(&int(0))).map_or(0, |r| self.item_crystal_type(r.product_id as u32))),
            ("UIDATA_RECIPE", "GetRecipeMpConsume") => Value::Int(self.tables().and_then(|t| t.recipes.get(&int(0))).map_or(0, |r| r.mp_consume)),
            ("UIDATA_RECIPE", "GetRecipeLevel") => Value::Int(self.tables().and_then(|t| t.recipes.get(&int(0))).map_or(0, |r| r.level)),
            ("UIDATA_RECIPE", "GetRecipeIndex") => Value::Int(self.tables().and_then(|t| t.recipe_order.iter().position(|id| *id == int(0))).map_or(-1, |i| i as i32)),
            ("UIDATA_RECIPE", "GetRecipeSuccessRate") => Value::Int(self.tables().and_then(|t| t.recipes.get(&int(0))).map_or(0, |r| r.success_rate)),
            ("UIDATA_RECIPE", "GetRecipeDescription") => Value::Str(self.tables().and_then(|t| t.recipes.get(&int(0))).map(|r| r.name.clone()).unwrap_or_default()),
            ("UIDATA_RECIPE", "GetRecipeMaterialItem") => {
                let mut param = String::new();
                if let Some(recipe) = self.tables().and_then(|t| t.recipes.get(&int(0))) {
                    param_add_int(&mut param, "Max", &recipe.materials.len().to_string());
                    for (index, material) in recipe.materials.iter().enumerate() {
                        param_add_int(&mut param, &format!("ItemID_{index}"), &material.item_id.to_string());
                        param_add_int(&mut param, &format!("ItemNum_{index}"), &material.count.to_string());
                    }
                }
                Value::Str(param)
            }
            ("UIDATA_HENNA", "GetItemName") | ("UIDATA_HENNA", "GetDescription") | ("UIDATA_HENNA", "GetIconTex") => {
                let henna = self.tables().and_then(|t| t.hennagrp.get(&(int(0) as u32)));
                match henna {
                    Some(henna) => {
                        let value = match function {
                            "GetItemName" => henna.name.clone(),
                            "GetDescription" => henna.description.clone(),
                            _ => henna.icon.clone(),
                        };
                        out(args, 1, Value::Str(value));
                        Value::Bool(true)
                    }
                    None => Value::Bool(false),
                }
            }
            ("HennaAPI", "GetHennaInfoCount") => Value::Int(self.hennas.len() as i32),
            ("HennaAPI", "GetHennaInfo") => match self.hennas.get(int(0).max(0) as usize) {
                Some(info) => {
                    out(args, 1, henna_value(info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIDATA_GAMETIP", "GetDataCount") => Value::Int(self.game_tips.len() as i32),
            ("UIDATA_GAMETIP", "GetDataByIndex") => match self.game_tips.get(int(0).max(0) as usize) {
                Some(tip) => {
                    out(args, 1, game_tip_value(tip));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIDATA_MACRO", "GetMacroCount") => Value::Int(self.macros.len() as i32),
            ("UIDATA_MACRO", "GetMacroInfo") => match self.macros.iter().find(|m| m.id == int(0)) {
                Some(info) => {
                    out(args, 1, macro_value(info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIDATA_RAID" | "UIDATA_HUNTINGZONE", "IsValidData") => Value::Bool(false),
            ("UIDATA_RAID" | "UIDATA_HUNTINGZONE", _) => return None,
            ("UIDATA_REFINERYOPTION", _) => return None,
            ("UIScript", "GetPlayerInfo") => match self.player.clone() {
                Some(info) => {
                    out(args, 0, user_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIScript", "GetTargetInfo") => match self.target.clone() {
                Some(info) => {
                    out(args, 0, user_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIScript", "GetUserInfo") => match self.user_info_of(int(0)) {
                Some(info) => {
                    out(args, 1, user_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIScript", "GetPetInfo") => match self.pet.clone() {
                Some(info) => {
                    out(args, 0, pet_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIScript", "GetSkillInfo") => match self.skill_info_of(int(0) as u32, int(1) as u32) {
                Some(info) => {
                    out(args, 2, skill_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIScript", "GetSystemString") => Value::Str(self.sysstrings.get(&u32::try_from(int(0)).unwrap_or(0)).cloned().unwrap_or_default()),
            ("UIScript", "GetSystemMessage") => Value::Str(self.system_message_text(u32::try_from(int(0)).unwrap_or(0))),
            ("UIScript", "GetPartyMemberCount") => Value::Int(self.party_members.len() as i32),
            ("UIScript", "GetPartyMemberLocation") => match self.party_locations.get(int(0).max(0) as usize) {
                Some(location) if int(0) >= 0 => {
                    out(args, 1, vector_value(*location));
                    Value::Bool(true)
                }
                _ => Value::Bool(false),
            },
            ("UIScript", "GetQuestLocation") => match self.quest_location {
                Some(location) if location != Vector::default() => {
                    out(args, 0, vector_value(location));
                    Value::Bool(true)
                }
                _ => Value::Bool(false),
            },
            ("UIScript", "GetCurrentZoneName") => Value::Str(self.zone_name.clone()),
            ("UIScript", "IsPKMode") => Value::Bool(self.pk_mode),
            _ => return None,
        })
    }
}

pub fn item_name_map(names: &[l2_dat::itemname::ItemName]) -> HashMap<u32, l2_dat::itemname::ItemName> {
    names.iter().map(|name| (name.id, name.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interlude_script::PetInfo;

    fn call(host: &mut ScriptHost, class: &str, function: &str, args: &mut [Value]) -> Value {
        host.data_native(class, function, args).unwrap()
    }

    fn quest(id: i32, level: i32) -> l2_dat::questname::QuestName {
        l2_dat::questname::QuestName {
            index: 0,
            valid: 1,
            id,
            state: level,
            title: format!("Quest {id}"),
            sub_name: "journal".into(),
            description: "desc".into(),
            item_ids: vec![57, 1835],
            item_nums: vec![10, 2],
            target_loc: [1.0, 2.0, 3.0],
            level: 20,
            quest_level: 30,
            quest_type: 3,
            target_name: "Gremlin".into(),
            minimap_only: 1,
            journal_disp: 1,
            unknown: 0,
            start_npc_id: 30001,
            start_npc_loc: [4.0, 5.0, 6.0],
            requirement: "req".into(),
            intro: "intro".into(),
        }
    }

    #[test]
    fn quest_natives_read_the_dat_fields_the_dll_reads_and_build_the_item_param() {
        let mut host = ScriptHost::default();
        host.own_quests.insert((7, 1), quest(7, 1));
        host.quest_zones.insert((7, 1), 42);
        assert_eq!(call(&mut host, "UIDATA_QUEST", "GetFirstID", &mut []), Value::Int(7));
        assert_eq!(call(&mut host, "UIDATA_QUEST", "GetNextID", &mut []), Value::Int(-1));
        assert_eq!(call(&mut host, "UIDATA_QUEST", "GetQuestName", &mut [Value::Int(7)]), Value::Str("Quest 7".into()));
        assert_eq!(call(&mut host, "UIDATA_QUEST", "GetMinLevel", &mut [Value::Int(7), Value::Int(1)]), Value::Int(20));
        assert_eq!(call(&mut host, "UIDATA_QUEST", "GetMaxLevel", &mut [Value::Int(7), Value::Int(1)]), Value::Int(30));
        assert_eq!(call(&mut host, "UIDATA_QUEST", "GetQuestZone", &mut [Value::Int(7), Value::Int(1)]), Value::Int(42));
        assert_eq!(call(&mut host, "UIDATA_QUEST", "GetStartNPCLoc", &mut [Value::Int(7), Value::Int(1)]).xyz(), (4.0, 5.0, 6.0));
        assert_eq!(
            call(&mut host, "UIDATA_QUEST", "GetQuestItem", &mut [Value::Int(7), Value::Int(1)]),
            Value::Str("Max=2 ItemID_0=57 ItemNum_0=10 ItemID_1=1835 ItemNum_1=2".into())
        );
        assert_eq!(call(&mut host, "UIDATA_QUEST", "IsMinimapOnly", &mut [Value::Int(7), Value::Int(1)]), Value::Bool(true));
        assert_eq!(call(&mut host, "UIDATA_QUEST", "IsValidData", &mut [Value::Int(8)]), Value::Bool(false));
    }

    #[test]
    fn player_pet_target_and_user_natives_read_the_host_state() {
        let mut host = ScriptHost {
            player: Some(UserInfo { id: 7, name: "Killer".into(), level: 52, cur_exp: 0, hero: true, ..Default::default() }),
            target: Some(UserInfo { id: 9, name: "Gremlin".into(), npc: true, cur_hp: 40, max_hp: 100, clan_id: 3, class_id: 20001, ..Default::default() }),
            pet: Some(PetInfo { id: 11, name: "Wolf".into(), cur_exp: 150, min_exp: 100, max_exp: 200, ..Default::default() }),
            ..Default::default()
        };
        host.users.insert(21, UserInfo { id: 21, name: "Larissa".into(), ..Default::default() });
        host.inventory_limit = 80;
        host.party_members = vec![7, 21];
        host.party_locations = vec![Vector { x: 1.0, y: 2.0, z: 3.0 }, Vector { x: 4.0, y: 5.0, z: 6.0 }];
        assert_eq!(call(&mut host, "UIDATA_PLAYER", "GetPlayerID", &mut []), Value::Int(7));
        assert_eq!(call(&mut host, "UIDATA_PLAYER", "IsHero", &mut []), Value::Bool(true));
        assert_eq!(call(&mut host, "UIDATA_PLAYER", "GetInventoryLimit", &mut []), Value::Int(80));
        assert_eq!(call(&mut host, "UIDATA_TARGET", "GetTargetHP", &mut []), Value::Int(40));
        assert_eq!(call(&mut host, "UIDATA_TARGET", "IsNpc", &mut []), Value::Bool(true));
        assert_eq!(call(&mut host, "UIDATA_TARGET", "GetTargetPledgeID", &mut []), Value::Int(3));
        assert_eq!(call(&mut host, "UIDATA_TARGET", "GetTargetNameColor", &mut [Value::Int(-9)]), color_value(TARGET_COLOR_FAR_BELOW));
        assert_eq!(call(&mut host, "UIDATA_TARGET", "GetTargetNameColor", &mut [Value::Int(0)]), color_value(TARGET_COLOR_EVEN));
        assert_eq!(call(&mut host, "UIDATA_TARGET", "GetTargetNameColor", &mut [Value::Int(7)]), color_value(TARGET_COLOR_ABOVE));
        assert_eq!(call(&mut host, "UIDATA_TARGET", "GetTargetNameColor", &mut [Value::Int(9)]), color_value(TARGET_COLOR_FAR_ABOVE));
        assert_eq!(call(&mut host, "UIDATA_PET", "GetPetEXPRate", &mut []), Value::Float(0.5));
        assert_eq!(call(&mut host, "UIDATA_PET", "GetPetEXP", &mut []), int64_value(150));
        assert_eq!(call(&mut host, "UIDATA_USER", "GetUserName", &mut [Value::Int(21)]), Value::Str("Larissa".into()));
        assert_eq!(call(&mut host, "UIDATA_PARTY", "GetMemberName", &mut [Value::Int(9)]), Value::Str("Gremlin".into()));
        assert_eq!(call(&mut host, "UIScript", "GetPartyMemberCount", &mut []), Value::Int(2));
        let mut args = vec![Value::Int(1), Value::None];
        assert_eq!(call(&mut host, "UIScript", "GetPartyMemberLocation", &mut args), Value::Bool(true));
        assert_eq!(args[1].xyz(), (4.0, 5.0, 6.0));
        let mut args = vec![Value::None];
        assert_eq!(call(&mut host, "UIScript", "GetPlayerInfo", &mut args), Value::Bool(true));
        assert_eq!(args[0].field("nLevel").as_int(), 52);
        let mut args = vec![Value::Int(21), Value::None];
        assert_eq!(call(&mut host, "UIScript", "GetUserInfo", &mut args), Value::Bool(true));
        assert_eq!(args[1].field("Name").as_str(), "Larissa");
        assert_eq!(call(&mut host, "UIScript", "GetQuestLocation", &mut [Value::None]), Value::Bool(false));
        host.quest_location = Some(Vector { x: 9.0, y: 8.0, z: 7.0 });
        let mut args = vec![Value::None];
        assert_eq!(call(&mut host, "UIScript", "GetQuestLocation", &mut args), Value::Bool(true));
        assert_eq!(args[0].xyz(), (9.0, 8.0, 7.0));
        assert!(host.data_native("UIDATA_RAID", "GetRaidLoc", &mut [Value::Int(1)]).is_none());
        assert_eq!(call(&mut host, "UIDATA_RAID", "IsValidData", &mut [Value::Int(1)]), Value::Bool(false));
    }

    #[test]
    fn exp_rate_follows_the_interlude_level_table() {
        let (low, high) = killer_ui::interlude_exp::level_bounds(10).unwrap();
        assert_eq!(exp_rate(low, 10), 0.0);
        assert!((exp_rate((low + high) / 2, 10) - 0.5).abs() < 0.01);
        assert_eq!(exp_rate(0, 0), 0.0);
    }

    #[test]
    fn system_messages_prefer_the_clients_own_table() {
        let mut host = ScriptHost::default();
        host.own_systemmsg.insert(125, "Quit the game?".into());
        host.sysstrings.insert(144, "Inventory".into());
        assert_eq!(call(&mut host, "UIScript", "GetSystemMessage", &mut [Value::Int(125)]), Value::Str("Quit the game?".into()));
        assert_eq!(call(&mut host, "UIScript", "GetSystemMessage", &mut [Value::Int(126)]), Value::Str(String::new()));
        assert_eq!(call(&mut host, "UIScript", "GetSystemString", &mut [Value::Int(144)]), Value::Str("Inventory".into()));
    }
}
