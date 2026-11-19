use killer_ui::color::Argb;
use l2_uscript::vm::{Error, Host, StructValue, Value};

use super::host_impl::ScriptHost;
use super::natives_table::HostOp;
use super::*;

pub fn int64(value: &Value) -> i64 {
    match value {
        Value::Struct(s) => {
            let left = s.get("nLeft").map_or(0, Value::as_int);
            let right = s.get("nRight").map_or(0, Value::as_int);
            (i64::from(left) << 32) | i64::from(right as u32)
        }
        other => i64::from(other.as_int()),
    }
}

pub fn int64_value(value: i64) -> Value {
    Value::Struct(Box::new(StructValue {
        name: "INT64".into(),
        fields: vec![("nLeft".into(), Value::Int((value >> 32) as i32)), ("nRight".into(), Value::Int(value as i32))],
    }))
}

pub fn color(value: &Value) -> Argb {
    let channel = |name: &str| value.field(name).as_int().clamp(0, 255) as u32;
    let alpha = match value.struct_ref().and_then(|s| s.get("A")) {
        Some(a) => a.as_int().clamp(0, 255) as u32,
        None => 255,
    };
    Argb((alpha << 24) | (channel("R") << 16) | (channel("G") << 8) | channel("B"))
}

pub fn color_value(color: Argb) -> Value {
    Value::Struct(Box::new(StructValue {
        name: "Color".into(),
        fields: vec![
            ("R".into(), Value::Byte(((color.0 >> 16) & 0xff) as u8)),
            ("G".into(), Value::Byte(((color.0 >> 8) & 0xff) as u8)),
            ("B".into(), Value::Byte((color.0 & 0xff) as u8)),
            ("A".into(), Value::Byte(((color.0 >> 24) & 0xff) as u8)),
        ],
    }))
}

pub fn rect_value(rect: Rect) -> Value {
    Value::Struct(Box::new(StructValue {
        name: "Rect".into(),
        fields: vec![
            ("nX".into(), Value::Int(rect.x)),
            ("nY".into(), Value::Int(rect.y)),
            ("nWidth".into(), Value::Int(rect.width)),
            ("nHeight".into(), Value::Int(rect.height)),
        ],
    }))
}

pub fn text_align(value: &Value) -> TextAlign {
    match value.as_int() {
        1 => TextAlign::Left,
        2 => TextAlign::Center,
        3 => TextAlign::Right,
        4 => TextAlign::MacroIcon,
        _ => TextAlign::Undefined,
    }
}

pub fn name_ctrl_type(value: &Value) -> NameCtrlType {
    if value.as_int() == 1 { NameCtrlType::Item } else { NameCtrlType::Normal }
}

pub fn interface_sound(value: &Value) -> InterfaceSound {
    match value.as_int() {
        0 => InterfaceSound::Click1,
        1 => InterfaceSound::Click2,
        2 => InterfaceSound::ClickFailed,
        3 => InterfaceSound::Pickup,
        4 => InterfaceSound::TrashBasket,
        5 => InterfaceSound::WindowOpen,
        6 => InterfaceSound::WindowClose,
        7 => InterfaceSound::QuestTutorial,
        8 => InterfaceSound::MinimapOpenClose,
        9 => InterfaceSound::CooltimeEnd,
        10 => InterfaceSound::Petition,
        11 => InterfaceSound::StatusWndOpen,
        12 => InterfaceSound::StatusWndClose,
        13 => InterfaceSound::InvenWndOpen,
        14 => InterfaceSound::InvenWndClose,
        15 => InterfaceSound::MapWndOpen,
        16 => InterfaceSound::MapWndClose,
        17 => InterfaceSound::SystemWndOpen,
        _ => InterfaceSound::SystemWndClose,
    }
}

pub fn vector(value: &Value) -> Vector {
    let (x, y, z) = value.xyz();
    Vector { x, y, z }
}

pub fn status_icon_info(value: &Value) -> StatusIconInfo {
    StatusIconInfo {
        name: value.field("Name").as_str(),
        icon_name: value.field("IconName").as_str(),
        size: value.field("Size").as_int(),
        description: value.field("Description").as_str(),
        back_tex: value.field("BackTex").as_str(),
        remain_time: value.field("RemainTime").as_int(),
        class_id: value.field("ClassID").as_int(),
        level: value.field("Level").as_int(),
        show: value.field("bShow").as_bool(),
        short_item: value.field("bShortItem").as_bool(),
        etc_item: value.field("bEtcItem").as_bool(),
        debuff: value.field("bDeBuff").as_bool(),
    }
}

pub fn status_icon_value(info: &StatusIconInfo) -> Value {
    Value::Struct(Box::new(StructValue {
        name: "StatusIconInfo".into(),
        fields: vec![
            ("Name".into(), Value::Str(info.name.clone())),
            ("IconName".into(), Value::Str(info.icon_name.clone())),
            ("Size".into(), Value::Int(info.size)),
            ("Description".into(), Value::Str(info.description.clone())),
            ("BackTex".into(), Value::Str(info.back_tex.clone())),
            ("RemainTime".into(), Value::Int(info.remain_time)),
            ("ClassID".into(), Value::Int(info.class_id)),
            ("Level".into(), Value::Int(info.level)),
            ("bShow".into(), Value::Bool(info.show)),
            ("bShortItem".into(), Value::Bool(info.short_item)),
            ("bEtcItem".into(), Value::Bool(info.etc_item)),
            ("bDeBuff".into(), Value::Bool(info.debuff)),
        ],
    }))
}

pub const ITEM_FIELDS: [&str; 53] = [
    "Name",
    "AdditionalName",
    "IconName",
    "IconNameEx1",
    "IconNameEx2",
    "IconNameEx3",
    "IconNameEx4",
    "ForeTexture",
    "Description",
    "DragSrcName",
    "DragSrcReserved",
    "MacroCommand",
    "ItemType",
    "ItemSubType",
    "ServerID",
    "ClassID",
    "ItemNum",
    "Price",
    "Level",
    "SlotBitType",
    "Weight",
    "MaterialType",
    "WeaponType",
    "PhysicalDamage",
    "MagicalDamage",
    "PhysicalDefense",
    "MagicalDefense",
    "ShieldDefense",
    "ShieldDefenseRate",
    "Durability",
    "CrystalType",
    "RandomDamage",
    "Critical",
    "HitModify",
    "AttackSpeed",
    "MpConsume",
    "ArmorType",
    "AvoidModify",
    "Damaged",
    "Enchanted",
    "MpBonus",
    "SoulshotCount",
    "SpiritshotCount",
    "PopMsgNum",
    "BodyPart",
    "RefineryOp1",
    "RefineryOp2",
    "CurrentDurability",
    "Reserved",
    "DefaultPrice",
    "ConsumeType",
    "Blessed",
    "AllItemCount",
];

pub fn item_info(value: &Value) -> ItemInfo {
    let s = |name: &str| value.field(name).as_str();
    let i = |name: &str| value.field(name).as_int();
    ItemInfo {
        name: s("Name"),
        additional_name: s("AdditionalName"),
        icon_name: s("IconName"),
        icon_name_ex1: s("IconNameEx1"),
        icon_name_ex2: s("IconNameEx2"),
        icon_name_ex3: s("IconNameEx3"),
        icon_name_ex4: s("IconNameEx4"),
        fore_texture: s("ForeTexture"),
        description: s("Description"),
        drag_src_name: s("DragSrcName"),
        drag_src_reserved: i("DragSrcReserved"),
        macro_command: s("MacroCommand"),
        item_type: i("ItemType"),
        item_sub_type: i("ItemSubType"),
        server_id: i("ServerID"),
        class_id: i("ClassID"),
        item_num: i("ItemNum"),
        price: i("Price"),
        level: i("Level"),
        slot_bit_type: i("SlotBitType"),
        weight: i("Weight"),
        material_type: i("MaterialType"),
        weapon_type: i("WeaponType"),
        physical_damage: i("PhysicalDamage"),
        magical_damage: i("MagicalDamage"),
        physical_defense: i("PhysicalDefense"),
        magical_defense: i("MagicalDefense"),
        shield_defense: i("ShieldDefense"),
        shield_defense_rate: i("ShieldDefenseRate"),
        durability: i("Durability"),
        crystal_type: i("CrystalType"),
        random_damage: i("RandomDamage"),
        critical: i("Critical"),
        hit_modify: i("HitModify"),
        attack_speed: i("AttackSpeed"),
        mp_consume: i("MpConsume"),
        armor_type: i("ArmorType"),
        avoid_modify: i("AvoidModify"),
        damaged: i("Damaged"),
        enchanted: i("Enchanted"),
        mp_bonus: i("MpBonus"),
        soulshot_count: i("SoulshotCount"),
        spiritshot_count: i("SpiritshotCount"),
        pop_msg_num: i("PopMsgNum"),
        body_part: i("BodyPart"),
        refinery_op1: i("RefineryOp1"),
        refinery_op2: i("RefineryOp2"),
        current_durability: i("CurrentDurability"),
        reserved: i("Reserved"),
        default_price: i("DefaultPrice"),
        consume_type: i("ConsumeType"),
        blessed: i("Blessed"),
        ..Default::default()
    }
}

pub fn user_info_value(info: &UserInfo) -> Value {
    let int = |name: &str, value: i32| (name.to_string(), Value::Int(value));
    Value::Struct(Box::new(StructValue {
        name: "UserInfo".into(),
        fields: vec![
            int("nID", info.id),
            ("Name".into(), Value::Str(info.name.clone())),
            ("strNickName".into(), Value::Str(info.nick_name.clone())),
            int("nLevel", info.level),
            int("nClassID", info.class_id),
            int("nSubClass", info.sub_class),
            int("nSP", info.sp),
            int("nCurHP", info.cur_hp),
            int("nMaxHP", info.max_hp),
            int("nCurMP", info.cur_mp),
            int("nMaxMP", info.max_mp),
            int("nCurCP", info.cur_cp),
            int("nMaxCP", info.max_cp),
            ("nCurExp".into(), int64_value(info.cur_exp)),
            int("nUserRank", info.user_rank),
            int("nClanID", info.clan_id),
            int("nAllianceID", info.alliance_id),
            int("nCarryWeight", info.carry_weight),
            int("nCarringWeight", info.carring_weight),
            int("nPhysicalAttack", info.physical_attack),
            int("nPhysicalDefense", info.physical_defense),
            int("nHitRate", info.hit_rate),
            int("nCriticalRate", info.critical_rate),
            int("nPhysicalAttackSpeed", info.physical_attack_speed),
            int("nMagicalAttack", info.magical_attack),
            int("nMagicDefense", info.magic_defense),
            int("nPhysicalAvoid", info.physical_avoid),
            int("nWaterMaxSpeed", info.water_max_speed),
            int("nWaterMinSpeed", info.water_min_speed),
            int("nAirMaxSpeed", info.air_max_speed),
            int("nAirMinSpeed", info.air_min_speed),
            int("nGroundMaxSpeed", info.ground_max_speed),
            int("nGroundMinSpeed", info.ground_min_speed),
            ("fNonAttackSpeedModifier".into(), Value::Float(info.non_attack_speed_modifier)),
            int("nMagicCastingSpeed", info.magic_casting_speed),
            int("nStr", info.str),
            int("nDex", info.dex),
            int("nCon", info.con),
            int("nInt", info.int),
            int("nWit", info.wit),
            int("nMen", info.men),
            int("nCriminalRate", info.criminal_rate),
            int("nDualCount", info.dual_count),
            int("nPKCount", info.pk_count),
            int("nSociality", info.sociality),
            int("nRemainSulffrage", info.remain_sulffrage),
            ("bHero".into(), Value::Bool(info.hero)),
            ("bNobless".into(), Value::Bool(info.nobless)),
            ("bNpc".into(), Value::Bool(info.npc)),
            ("bPet".into(), Value::Bool(info.pet)),
            ("bCanBeAttacked".into(), Value::Bool(info.can_be_attacked)),
            ("loc".into(), Value::vector(info.loc.x, info.loc.y, info.loc.z)),
        ],
    }))
}

pub fn pet_info_value(info: &PetInfo) -> Value {
    let int = |name: &str, value: i32| (name.to_string(), Value::Int(value));
    Value::Struct(Box::new(StructValue {
        name: "PetInfo".into(),
        fields: vec![
            int("nID", info.id),
            ("Name".into(), Value::Str(info.name.clone())),
            int("nLevel", info.level),
            int("nSP", info.sp),
            int("nCurHP", info.cur_hp),
            int("nMaxHP", info.max_hp),
            int("nCurMP", info.cur_mp),
            int("nMaxMP", info.max_mp),
            ("nCurExp".into(), int64_value(info.cur_exp)),
            ("nMaxExp".into(), int64_value(info.max_exp)),
            ("nMinExp".into(), int64_value(info.min_exp)),
            int("nCarryWeight", info.carry_weight),
            int("nCarringWeight", info.carring_weight),
            int("nFatigue", info.fatigue),
            int("nMaxFatigue", info.max_fatigue),
            ("bIsPetOrSummoned".into(), Value::Bool(info.is_pet_or_summoned)),
        ],
    }))
}

fn out(args: &mut [Value], index: usize, value: Value) {
    if let Some(slot) = args.get_mut(index) {
        *slot = value;
    }
}

fn parse_native(args: &mut [Value], convert: impl Fn(&str) -> Value) -> Value {
    let (param, name) =
        (args.first().map_or_else(String::new, Value::as_str), args.get(1).map_or_else(String::new, Value::as_str));
    match parse_value(&param, &name) {
        Some(found) => {
            let value = convert(found);
            out(args, 2, value);
            Value::Bool(true)
        }
        None => Value::Bool(false),
    }
}

impl Host for ScriptHost {
    fn call_native(
        &mut self,
        class: &str,
        function: &str,
        this: Option<&Value>,
        args: &mut [Value],
    ) -> Result<Value, Error> {
        let handle = this.and_then(|value| match value {
            Value::Handle(name) => Some(name.clone()),
            _ => None,
        });
        let mut owned: Vec<Value> = Vec::with_capacity(args.len() + 1);
        if let Some(handle) = &handle {
            owned.push(Value::Str(handle.clone()));
        }
        owned.extend(args.iter().cloned());
        let result = self.dispatch(class, function, &mut owned);
        let shift = usize::from(handle.is_some());
        for (index, value) in owned.into_iter().enumerate().skip(shift) {
            if let Some(slot) = args.get_mut(index - shift) {
                *slot = value;
            }
        }
        Ok(result)
    }
}

impl ScriptHost {
    #[allow(clippy::too_many_lines)]
    fn run_op(&mut self, op: HostOp, args: &mut [Value]) -> Value {
        let text = |index: usize| args.get(index).map_or_else(String::new, Value::as_str);
        let int = |index: usize| args.get(index).map_or(0, Value::as_int);
        let name = text(0);
        match op {
            HostOp::ShowWindow => {
                self.show_window(&name);
                Value::None
            }
            HostOp::HideWindow => {
                self.hide_window(&name);
                Value::None
            }
            HostOp::IsShowWindow => Value::Bool(self.is_show_window(&name)),
            HostOp::SetAlpha => {
                self.set_alpha(&name, int(1), args.get(2).map_or(0.0, Value::as_float));
                Value::None
            }
            HostOp::MoveTo => {
                self.move_to(&name, int(1), int(2));
                Value::None
            }
            HostOp::Move => {
                self.move_by(&name, int(1), int(2), args.get(3).map_or(0.0, Value::as_float));
                Value::None
            }
            HostOp::SetWindowSize => {
                self.set_window_size(&name, int(1), int(2));
                Value::None
            }
            HostOp::SetFrameSize => {
                self.set_frame_size(&name, int(1), int(2));
                Value::None
            }
            HostOp::SetAnchor => {
                let (window, relative, anchor) = (text(1), text(2), text(3));
                self.set_anchor(&name, &window, &relative, &anchor, int(4), int(5));
                Value::None
            }
            HostOp::ClearAnchor => {
                self.clear_anchor(&name);
                Value::None
            }
            HostOp::SetFocus => {
                self.set_focus(&name);
                Value::None
            }
            HostOp::IsFocused => Value::Bool(self.is_focused(&name)),
            HostOp::SetTooltipText | HostOp::SetTooltipString => {
                let value = text(1);
                self.set_tooltip_text(&name, &value);
                Value::None
            }
            HostOp::GetTooltipText => Value::Str(String::new()),
            HostOp::GetRect => rect_value(self.get_rect(&name)),
            HostOp::EnableWindow => {
                self.enable_window(&name);
                Value::None
            }
            HostOp::IsEnableWindow => Value::Bool(self.is_enable_window(&name)),
            HostOp::Clear => {
                self.clear(&name);
                Value::None
            }
            HostOp::SetTabOrder | HostOp::SetTooltipType => Value::None,
            HostOp::SetUITimer => {
                self.set_ui_timer(&name, int(1), int(2));
                Value::None
            }
            HostOp::KillUITimer => {
                self.kill_ui_timer(&name, int(1));
                Value::None
            }
            HostOp::SetText => {
                let value = text(1);
                self.set_text(&name, &value);
                Value::None
            }
            HostOp::GetText => Value::Str(self.get_text(&name)),
            HostOp::SetPoint => {
                self.set_point(&name, int(1), int(2));
                Value::None
            }
            HostOp::SetPointPercent => {
                let (current, min, max) = (args.get(1).map_or(0, int64), args.get(2).map_or(0, int64), args.get(3).map_or(0, int64));
                self.set_point_percent(&name, current, min, max);
                Value::None
            }
            HostOp::SetPointExp => {
                let current = args.get(1).map_or(0, int64);
                self.set_point_exp(&name, current, int(2));
                Value::None
            }
            HostOp::SetRegenInfo => {
                self.set_regen_info(&name, int(1), int(2), args.get(3).map_or(0.0, Value::as_float));
                Value::None
            }
            HostOp::BarSetValue => {
                self.bar_set_value(&name, int(1), int(2));
                Value::None
            }
            HostOp::BarGetValue => {
                let (max, current) = self.bar_get_value(&name);
                out(args, 1, Value::Int(max));
                out(args, 2, Value::Int(current));
                Value::None
            }
            HostOp::BarClear => {
                self.overlay.points.remove(&name);
                Value::None
            }
            HostOp::SetTexture => {
                let value = text(1);
                self.set_texture(&name, &value);
                Value::None
            }
            HostOp::SetUV => {
                self.set_uv(&name, int(1), int(2));
                Value::None
            }
            HostOp::SetName => {
                let value = text(1);
                let (kind, align) = (args.get(2).map_or(NameCtrlType::Normal, name_ctrl_type), args.get(3).map_or(TextAlign::Undefined, text_align));
                self.set_name(&name, &value, kind, align);
                Value::None
            }
            HostOp::SetNameWithColor => {
                let value = text(1);
                let (kind, align) = (args.get(2).map_or(NameCtrlType::Normal, name_ctrl_type), args.get(3).map_or(TextAlign::Undefined, text_align));
                let color = args.get(4).map_or(Argb(0xFFDC_DCDC), color);
                self.set_name_with_color(&name, &value, kind, align, color);
                Value::None
            }
            HostOp::CheckSetCheck => {
                self.checkbox_set_check(&name, args.get(1).is_some_and(Value::as_bool));
                Value::None
            }
            HostOp::CheckIsChecked => Value::Bool(self.checkbox_is_checked(&name)),
            HostOp::CheckSetDisable => {
                self.checkbox_set_disable(&name, args.get(1).is_some_and(Value::as_bool));
                Value::None
            }
            HostOp::CheckIsDisable => Value::Bool(self.checkbox_is_disable(&name)),
            HostOp::TabSetTopOrder => {
                self.tab_set_top_order(&name, int(1), args.get(2).is_some_and(Value::as_bool));
                Value::None
            }
            HostOp::TabGetTopIndex => Value::Int(self.tab_get_top_index(&name)),
            HostOp::TabSetDisable => {
                self.tab_set_disable(&name, int(1), args.get(2).is_some_and(Value::as_bool));
                Value::None
            }
            HostOp::ChatAddString => {
                let value = text(1);
                let color = args.get(2).map_or(Argb(0xFFDC_DCDC), color);
                self.chat_add_string(&name, &value, color);
                Value::None
            }
            HostOp::ComboAddString => {
                let value = text(1);
                self.combo_add_string(&name, &value, int(2));
                Value::None
            }
            HostOp::ComboClear => {
                self.combo_clear(&name);
                Value::None
            }
            HostOp::ComboAddSysString => {
                let value = self.get_system_string(int(1));
                self.combo_add_string(&name, &value, int(2));
                Value::None
            }
            HostOp::UpdateShortcut => {
                self.update_shortcut(&name, int(1));
                Value::None
            }
            HostOp::StatusIconAddRow => {
                self.status_icon_add_row(&name);
                Value::None
            }
            HostOp::StatusIconAddCol => {
                let info = args.get(2).map(status_icon_info).unwrap_or_default();
                self.status_icon_add_col(&name, int(1), &info);
                Value::None
            }
            HostOp::StatusIconInsertRow => {
                self.status_icon_insert_row(&name, int(1));
                Value::None
            }
            HostOp::StatusIconInsertCol => {
                let info = args.get(3).map(status_icon_info).unwrap_or_default();
                self.status_icon_insert_col(&name, int(1), int(2), &info);
                Value::None
            }
            HostOp::StatusIconRowCount => Value::Int(self.status_icon_row_count(&name)),
            HostOp::StatusIconColCount => Value::Int(self.status_icon_col_count(&name, int(1))),
            HostOp::StatusIconDelItem => {
                self.status_icon_del_item(&name, int(1), int(2));
                Value::None
            }
            HostOp::StatusIconGetItem => match self.status_icon_get_item(&name, int(1), int(2)) {
                Some(info) => {
                    out(args, 3, status_icon_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            HostOp::StatusIconClear => {
                self.status_grids.remove(&name);
                Value::None
            }
            HostOp::EditSetString => {
                let value = text(1);
                self.edit_set_string(&name, &value);
                Value::None
            }
            HostOp::EditGetString => Value::Str(self.edit_get_string(&name)),
        }
    }

    fn dispatch(&mut self, class: &str, function: &str, args: &mut [Value]) -> Value {
        if let Some(op) = super::natives_table::host_op(class, function) {
            return self.run_op(op, args);
        }
        if let Some(value) = self.data_native(class, function, args) {
            return value;
        }
        if let Some(value) = self.script_native(class, function, args) {
            return value;
        }
        if let Some(value) = self.control_native(class, function, args) {
            return value;
        }
        match (class, function) {
            (_, "GetHandle") => Value::Handle(args.first().map_or_else(String::new, Value::as_str)),
            (_, "ParseInt") => parse_native(args, |text| Value::Int(app_atoi(text))),
            (_, "ParseString") => parse_native(args, |text| Value::Str(text.to_string())),
            (_, "ParseFloat") => parse_native(args, |text| Value::Float(l2_uscript::vm::app_atof(text))),
            (_, "ParseInt64") => parse_native(args, |text| int64_value(text.trim().parse().unwrap_or(0))),
            (_, "ParamAdd") => {
                let mut param = args.first().map_or_else(String::new, Value::as_str);
                let (name, value) = (
                    args.get(1).map_or_else(String::new, Value::as_str),
                    args.get(2).map_or_else(String::new, Value::as_str),
                );
                param_add(&mut param, &name, &value);
                out(args, 0, Value::Str(param));
                Value::None
            }
            (_, "ParamAddInt64") => {
                let mut param = args.first().map_or_else(String::new, Value::as_str);
                let name = args.get(1).map_or_else(String::new, Value::as_str);
                let value = args.get(2).map_or(0, int64);
                param_add(&mut param, &name, &value.to_string());
                out(args, 0, Value::Str(param));
                Value::None
            }
            (_, "RegisterEvent") => {
                self.register_event(args.first().map_or(0, Value::as_int));
                Value::None
            }
            (_, "ExecuteEvent") => {
                let param = args.get(1).map_or_else(String::new, Value::as_str);
                self.execute_event(args.first().map_or(0, Value::as_int), &param);
                Value::None
            }
            (_, "GetPlayerInfo") => match self.get_player_info() {
                Some(info) => {
                    out(args, 0, user_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            (_, "GetTargetInfo") => match self.get_target_info() {
                Some(info) => {
                    out(args, 0, user_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            (_, "GetUserInfo") => match self.get_user_info(args.first().map_or(0, Value::as_int)) {
                Some(info) => {
                    out(args, 1, user_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            (_, "GetPetInfo") => match self.get_pet_info() {
                Some(info) => {
                    out(args, 0, pet_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            (_, "GetScript") => {
                let name = args.first().map_or_else(String::new, Value::as_str);
                self.script_objects
                    .get(&name)
                    .map_or(Value::None, |object| Value::Object(l2_uscript::vm::ObjId(*object)))
            }
            (_, "GetLanguage") => Value::Str(self.language.clone()),
            (class, "GetFirstID" | "GetNextID") if class.starts_with("UIDATA_") => Value::Int(-1),
            (class, "IsValidData") if class.starts_with("UIDATA_") => Value::Bool(false),
            (class, "GetDataCount" | "GetMacroCount") if class.starts_with("UIDATA_") => Value::Int(0),
            ("UIDATA_TARGET", "GetTargetID") => Value::Int(self.target.as_ref().map_or(0, |target| target.id)),
            ("UIDATA_TARGET", "GetTargetName") => {
                Value::Str(self.target.as_ref().map_or_else(String::new, |target| target.name.clone()))
            }
            ("UIDATA_TARGET", "GetTargetHP") => Value::Int(self.target.as_ref().map_or(0, |target| target.cur_hp)),
            ("UIDATA_TARGET", "GetTargetMaxHP") => Value::Int(self.target.as_ref().map_or(0, |target| target.max_hp)),
            ("UIDATA_TARGET", "GetTargetMP") => Value::Int(self.target.as_ref().map_or(0, |target| target.cur_mp)),
            ("UIDATA_TARGET", "GetTargetMaxMP") => Value::Int(self.target.as_ref().map_or(0, |target| target.max_mp)),
            ("UIDATA_TARGET", "GetTargetClassID") => {
                Value::Int(self.target.as_ref().map_or(0, |target| target.class_id))
            }
            ("UIDATA_TARGET", "GetTargetPledgeID") => {
                Value::Int(self.target.as_ref().map_or(0, |target| target.clan_id))
            }
            ("UIDATA_TARGET", "GetTargetUserRank") => {
                Value::Int(self.target.as_ref().map_or(0, |target| target.user_rank))
            }
            ("UIDATA_TARGET", "IsNpc") => Value::Bool(self.target.as_ref().is_some_and(|target| target.npc)),
            ("UIDATA_PLAYER", "GetPlayerID") => Value::Int(self.player.as_ref().map_or(0, |player| player.id)),
            ("UIDATA_USER", "GetUserName") => {
                Value::Str(self.player.as_ref().map_or_else(String::new, |player| player.name.clone()))
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetTabOrder" | "SetTooltipCustomType" | "SetTooltipType") => Value::None,
            (_, "GetPlayerPosition") => {
                let position = self.get_player_position();
                Value::vector(position.x, position.y, position.z)
            }
            ("UIAPI_WINDOW" | "WindowHandle", "GetRect") => {
                rect_value(self.get_rect(&args.first().map_or_else(String::new, Value::as_str)))
            }
            ("UIAPI_WINDOW" | "WindowHandle", "Clear") => {
                self.clear(&args.first().map_or_else(String::new, Value::as_str));
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "Clear") => {
                self.combo_clear(&args.first().map_or_else(String::new, Value::as_str));
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "Clear") => {
                self.item_clear(&args.first().map_or_else(String::new, Value::as_str));
                Value::None
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "Clear") => {
                self.status_grids.remove(&args.first().map_or_else(String::new, Value::as_str));
                Value::None
            }
            ("UIAPI_BARCTRL" | "BarHandle", "GetValue") => {
                let (max, current) = self.bar_get_value(&args.first().map_or_else(String::new, Value::as_str));
                out(args, 1, Value::Int(max));
                out(args, 2, Value::Int(current));
                Value::None
            }
            (_, "GetTextSize") => {
                let (width, height) = self.get_text_size(&args.first().map_or_else(String::new, Value::as_str));
                out(args, 1, Value::Int(width));
                out(args, 2, Value::Int(height));
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "GetWindowSize") => {
                let (width, height) = self.get_window_size(&args.first().map_or_else(String::new, Value::as_str));
                out(args, 1, Value::Int(width));
                out(args, 2, Value::Int(height));
                Value::None
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "GetItem") => {
                let name = args.first().map_or_else(String::new, Value::as_str);
                let (row, col) = (args.get(1).map_or(0, Value::as_int), args.get(2).map_or(0, Value::as_int));
                match self.status_icon_get_item(&name, row, col) {
                    Some(info) => {
                        out(args, 3, status_icon_value(&info));
                        Value::Bool(true)
                    }
                    None => Value::Bool(false),
                }
            }
            (
                "UIAPI_CHATWINDOW" | "ChatWindowHandle" | "TextListBoxHandle" | "UIAPI_TEXTLISTBOX" | "ListBoxHandle"
                | "UIAPI_LISTBOX",
                "AddString",
            ) => {
                let name = args.first().map_or_else(String::new, Value::as_str);
                let text = args.get(1).map_or_else(String::new, Value::as_str);
                let color = args.get(2).map_or(Argb(0xFFDC_DCDC), color);
                self.chat_add_string(&name, &text, color);
                Value::None
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "GetString") => {
                Value::Str(self.edit_get_string(&args.first().map_or_else(String::new, Value::as_str)))
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "SetString") => {
                let name = args.first().map_or_else(String::new, Value::as_str);
                let text = args.get(1).map_or_else(String::new, Value::as_str);
                self.edit_set_string(&name, &text);
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "GetString") => {
                let name = args.first().map_or_else(String::new, Value::as_str);
                Value::Str(self.combo_get_string(&name, args.get(1).map_or(0, Value::as_int)))
            }
            (_, "GetINIBool") => {
                let (section, key) = (
                    args.first().map_or_else(String::new, Value::as_str),
                    args.get(1).map_or_else(String::new, Value::as_str),
                );
                let file = args.get(3).map_or_else(String::new, Value::as_str);
                match self.get_ini_bool(&section, &key, &file) {
                    Some(value) => {
                        out(args, 2, Value::Int(value));
                        Value::Bool(true)
                    }
                    None => Value::Bool(false),
                }
            }
            (_, "GetINIString") => {
                let (section, key) = (
                    args.first().map_or_else(String::new, Value::as_str),
                    args.get(1).map_or_else(String::new, Value::as_str),
                );
                let file = args.get(3).map_or_else(String::new, Value::as_str);
                match self.get_ini_string(&section, &key, &file) {
                    Some(value) => {
                        out(args, 2, Value::Str(value));
                        Value::Bool(true)
                    }
                    None => Value::Bool(false),
                }
            }
            ("UIAPI_WINDOW" | "WindowHandle", "IsShowWindow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Bool(self.is_show_window(&s0))
            }
            ("UIAPI_WINDOW" | "WindowHandle", "ShowWindow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.show_window(&s0);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "HideWindow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.hide_window(&s0);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetUiTimer") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_ui_timer(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "KillUiTimer") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.kill_ui_timer(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetWindowTitle") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.set_window_title(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetWindowTitleByText") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.set_window_title_by_text(&s0, &s1);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "EnableWindow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.enable_window(&s0);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "DisableWindow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.disable_window(&s0);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "IsEnableWindow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Bool(self.is_enable_window(&s0))
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetAlwaysOnTop") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.set_always_on_top(&s0, a1.as_bool());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetAlpha") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_alpha(&s0, a1.as_int(), a2.as_float());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "Move") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.move_by(&s0, a1.as_int(), a2.as_int(), a3.as_float());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "MoveTo") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.move_to(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "Iconize") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.iconize(&s0, &s1, a2.as_int());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetFocus") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.set_focus(&s0);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "IsFocused") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Bool(self.is_focused(&s0))
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetWindowSize") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_window_size(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetWindowSizeRel") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                let a4 = args.get(4).cloned().unwrap_or_default();
                self.set_window_size_rel(&s0, a1.as_float(), a2.as_float(), a3.as_int(), a4.as_int());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetFrameSize") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_frame_size(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetAnchor") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let s2 = a2.as_str();
                let a3 = args.get(3).cloned().unwrap_or_default();
                let s3 = a3.as_str();
                let a4 = args.get(4).cloned().unwrap_or_default();
                let a5 = args.get(5).cloned().unwrap_or_default();
                self.set_anchor(&s0, &s1, &s2, &s3, a4.as_int(), a5.as_int());
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "ClearAnchor") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.clear_anchor(&s0);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetTooltipText") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.set_tooltip_text(&s0, &s1);
                Value::None
            }
            ("UIAPI_TEXTBOX" | "TextBoxHandle", "SetText") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.set_text(&s0, &s1);
                Value::None
            }
            ("UIAPI_TEXTBOX" | "TextBoxHandle", "SetTextColor") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.set_text_color(&s0, color(&a1));
                Value::None
            }
            ("UIAPI_TEXTBOX" | "TextBoxHandle", "SetAlign") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.set_text_align(&s0, text_align(&a1));
                Value::None
            }
            ("UIAPI_TEXTBOX" | "TextBoxHandle", "SetInt") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.set_int(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_TEXTBOX" | "TextBoxHandle", "GetText") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Str(self.get_text(&s0))
            }
            ("UIAPI_TEXTURECTRL" | "TextureHandle", "SetTexture") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.set_texture(&s0, &s1);
                Value::None
            }
            ("UIAPI_TEXTURECTRL" | "TextureHandle", "SetUV") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_uv(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_TEXTURECTRL" | "TextureHandle", "GetTextureName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Str(self.get_texture_name(&s0))
            }
            ("UIAPI_NAMECTRL" | "NameCtrlHandle", "SetName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.set_name(&s0, &s1, name_ctrl_type(&a2), text_align(&a3));
                Value::None
            }
            ("UIAPI_NAMECTRL" | "NameCtrlHandle", "SetNameWithColor") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                let a4 = args.get(4).cloned().unwrap_or_default();
                self.set_name_with_color(&s0, &s1, name_ctrl_type(&a2), text_align(&a3), color(&a4));
                Value::None
            }
            ("UIAPI_STATUSBARCTRL" | "StatusBarHandle", "SetPoint") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_point(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_STATUSBARCTRL" | "StatusBarHandle", "SetPointPercent") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.set_point_percent(&s0, int64(&a1), int64(&a2), int64(&a3));
                Value::None
            }
            ("UIAPI_STATUSBARCTRL" | "StatusBarHandle", "SetPointExp") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_point_exp(&s0, int64(&a1), a2.as_int());
                Value::None
            }
            ("UIAPI_STATUSBARCTRL" | "StatusBarHandle", "SetRegenInfo") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.set_regen_info(&s0, a1.as_int(), a2.as_int(), a3.as_float());
                Value::None
            }
            ("UIAPI_BARCTRL" | "BarHandle", "SetValue") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.bar_set_value(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_CHECKBOX" | "CheckBoxHandle", "SetTitle") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.checkbox_set_title(&s0, &s1);
                Value::None
            }
            ("UIAPI_CHECKBOX" | "CheckBoxHandle", "SetCheck") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.checkbox_set_check(&s0, a1.as_bool());
                Value::None
            }
            ("UIAPI_CHECKBOX" | "CheckBoxHandle", "IsChecked") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Bool(self.checkbox_is_checked(&s0))
            }
            ("UIAPI_CHECKBOX" | "CheckBoxHandle", "SetDisable") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.checkbox_set_disable(&s0, a1.as_bool());
                Value::None
            }
            ("UIAPI_CHECKBOX" | "CheckBoxHandle", "IsDisable") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Bool(self.checkbox_is_disable(&s0))
            }
            ("UIAPI_SHORTCUTITEMWINDOW" | "ShortcutItemWindowHandle", "UpdateShortcut") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.update_shortcut(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "AddRow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.status_icon_add_row(&s0);
                Value::None
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "AddCol") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.status_icon_add_col(&s0, a1.as_int(), &status_icon_info(&a2));
                Value::None
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "InsertRow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.status_icon_insert_row(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "InsertCol") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.status_icon_insert_col(&s0, a1.as_int(), a2.as_int(), &status_icon_info(&a3));
                Value::None
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "RowCount") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Int(self.status_icon_row_count(&s0))
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "ColCount") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                Value::Int(self.status_icon_col_count(&s0, a1.as_int()))
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "DelItem") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.status_icon_del_item(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_TABCTRL" | "TabHandle", "INIt") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.tab_init(&s0);
                Value::None
            }
            ("UIAPI_TABCTRL" | "TabHandle", "SetTopOrder") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.tab_set_top_order(&s0, a1.as_int(), a2.as_bool());
                Value::None
            }
            ("UIAPI_TABCTRL" | "TabHandle", "GetTopIndex") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Int(self.tab_get_top_index(&s0))
            }
            ("UIAPI_TABCTRL" | "TabHandle", "SetDisable") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.tab_set_disable(&s0, a1.as_int(), a2.as_bool());
                Value::None
            }
            ("UIAPI_TABCTRL" | "TabHandle", "Merge") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.tab_merge(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_CHATWINDOW" | "ChatWindowHandle", "EnableTexture") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.chat_enable_texture(&s0, a1.as_bool());
                Value::None
            }
            ("UIAPI_CHATWINDOW" | "ChatWindowHandle", "SetScrollBarPosition") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.chat_set_scroll_bar_position(&s0, a1.as_int(), a2.as_int(), a3.as_int());
                Value::None
            }
            (_, "SetChatMessage") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.set_chat_message(&s0);
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "AddString") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.combo_add_string(&s0, &s1, a2.as_int());
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "GetSelectedNum") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Int(self.combo_get_selected_num(&s0))
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "SetSelectedNum") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.combo_set_selected_num(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "GetReserved") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                Value::Int(self.combo_get_reserved(&s0, a1.as_int()))
            }
            ("UIAPI_RADAR" | "RadarHandle", "AddTarget") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.radar_add_target(&s0, a1.as_int(), a2.as_int(), a3.as_int());
                Value::None
            }
            ("UIAPI_RADAR" | "RadarHandle", "DeleteTarget") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.radar_delete_target(&s0, a1.as_int(), a2.as_int(), a3.as_int());
                Value::None
            }
            ("UIAPI_RADAR" | "RadarHandle", "DeleteAllTarget") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.radar_delete_all_target(&s0);
                Value::None
            }
            ("UIAPI_RADAR" | "RadarHandle", "SetColor") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.radar_set_color(&s0, color(&a1), a2.as_float());
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "AdjustMapView") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.minimap_adjust_map_view(&s0, vector(&a1), a2.as_bool(), a3.as_bool());
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "INItPosition") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.minimap_init_position(&s0);
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "AddTarget") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.minimap_add_target(&s0, vector(&a1));
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "DeleteTarget") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.minimap_delete_target(&s0, vector(&a1));
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "DeleteAllTarget") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.minimap_delete_all_target(&s0);
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "SetShowQuest") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.minimap_set_show_quest(&s0, a1.as_bool());
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "SetSSQStatus") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.minimap_set_ssq_status(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "DrawGridIcon") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let s2 = a2.as_str();
                let a3 = args.get(3).cloned().unwrap_or_default();
                let a4 = args.get(4).cloned().unwrap_or_default();
                let a5 = args.get(5).cloned().unwrap_or_default();
                let a6 = args.get(6).cloned().unwrap_or_default();
                let a7 = args.get(7).cloned().unwrap_or_default();
                let s7 = a7.as_str();
                self.minimap_draw_grid_icon(&s0, &s1, &s2, vector(&a3), a4.as_bool(), a5.as_int(), a6.as_int(), &s7);
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "RequestReduceBtn") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.minimap_request_reduce_btn(&s0);
                Value::None
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "IsOverlapped") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                let a4 = args.get(4).cloned().unwrap_or_default();
                Value::Bool(self.minimap_is_overlapped(&s0, a1.as_int(), a2.as_int(), a3.as_int(), a4.as_int()))
            }
            ("UIAPI_MINIMAPCTRL" | "MinimapHandle" | "MiniMapHandle", "DeleteAllCursedWeaponIcon") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.minimap_delete_all_cursed_weapon_icon(&s0);
                Value::None
            }
            (_, "RegisterState") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.register_state(&s0, &s1);
                Value::None
            }
            (_, "SetUiState") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.set_ui_state(&s0);
                Value::None
            }
            (_, "GetUiState") => Value::Str(self.get_ui_state()),
            (_, "GetOptionBool") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                Value::Bool(self.get_option_bool(&s0, &s1))
            }
            (_, "GetOptionInt") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                Value::Int(self.get_option_int(&s0, &s1))
            }
            (_, "GetOptionString") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                Value::Str(self.get_option_string(&s0, &s1))
            }
            (_, "SetOptionBool") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_option_bool(&s0, &s1, a2.as_bool());
                Value::None
            }
            (_, "SetOptionInt") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_option_int(&s0, &s1, a2.as_int());
                Value::None
            }
            (_, "GetSystemString") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.get_system_string(a0.as_int()))
            }
            (_, "GetSystemMessage") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.get_system_message(a0.as_int()))
            }
            (_, "MakeFullSystemMsg") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let s2 = a2.as_str();
                Value::Str(self.make_full_system_msg(&s0, &s1, &s2))
            }
            (_, "PlaySound") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.play_sound(&s0);
                Value::None
            }
            (_, "PlayConsoleSound") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.play_console_sound(interface_sound(&a0));
                Value::None
            }
            (_, "Debug") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.debug(&s0);
                Value::None
            }
            (_, "SetTimer") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_timer(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            (_, "KillTimer") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.kill_timer(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "AddString") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.edit_add_string(&s0, &s1);
                Value::None
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "ReleaseFocus") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.edit_release_focus(&s0);
                Value::None
            }
            (_, "SetSettledWnd") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.set_settled_wnd(&s0, a1.as_bool());
                Value::None
            }
            ("UIAPI_BUTTON" | "ButtonHandle", "SetTexture") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let s2 = a2.as_str();
                let a3 = args.get(3).cloned().unwrap_or_default();
                let s3 = a3.as_str();
                self.button_set_texture(&s0, &s1, &s2, &s3);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetTooltipSimpleText") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.set_tooltip_simple_text(&s0, &s1);
                Value::None
            }
            (_, "GetCurrentIMELang") => Value::Int(self.get_current_ime_lang()),
            (_, "ProcessChatMessage") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.process_chat_message(&s0, a1.as_int());
                Value::None
            }
            (_, "IsSameChatPrefix") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                Value::Bool(self.is_same_chat_prefix(a0.as_int(), &s1))
            }
            (_, "GetChatPrefix") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.get_chat_prefix(a0.as_int()))
            }
            (_, "AddSystemMessage") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.add_system_message(&s0, color(&a1));
                Value::None
            }
            (_, "ToggleMsnWindow") => {
                self.toggle_msn_window();
                Value::None
            }
            (_, "ExecuteCommand") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.execute_command(&s0);
                Value::None
            }
            (_, "RequestOpenPartyMatch") => {
                self.request_open_party_match();
                Value::None
            }
            (_, "ScriptOnSendPacketWhenHiding") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.script_on_send_packet_when_hiding(&s0);
                Value::None
            }
            (_, "SetShortcutPage") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.set_shortcut_page(a0.as_int());
                Value::None
            }
            (_, "RequestCursedWeaponList") => {
                self.request_cursed_weapon_list();
                Value::None
            }
            (_, "RequestCursedWeaponLocation") => {
                self.request_cursed_weapon_location();
                Value::None
            }
            (_, "GetPartyMemberCount") => Value::Int(self.get_party_member_count()),
            (_, "GetCurrentZoneName") => Value::Str(self.get_current_zone_name()),
            (_, "RequestSelfTarget") => {
                self.request_self_target();
                Value::None
            }
            (_, "RequestTargetCancel") => {
                self.request_target_cancel();
                Value::None
            }
            (_, "RequestStartTrade") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.request_start_trade(a0.as_int());
                Value::None
            }
            (_, "RequestInviteParty") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.request_invite_party(&s0);
                Value::None
            }
            (_, "RequestTargetUser") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.request_target_user(a0.as_int());
                Value::None
            }
            (_, "RequestAttack") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.request_attack(a0.as_int(), vector(&a1));
                Value::None
            }
            (_, "RequestAction") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.request_action(a0.as_int(), vector(&a1));
                Value::None
            }
            (_, "RequestAssist") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.request_assist(a0.as_int(), vector(&a1));
                Value::None
            }
            (_, "IsPKMode") => Value::Bool(self.is_pk_mode()),
            (_, "GetClassType") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.get_class_type(a0.as_int()))
            }
            (_, "GetClassStr") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.get_class_str(a0.as_int()))
            }
            (_, "GetClassIconName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.get_class_icon_name(a0.as_int()))
            }
            (_, "GetUserRankString") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.get_user_rank_string(a0.as_int()))
            }
            (_, "GetRoutingString") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.get_routing_string(a0.as_int()))
            }
            (_, "IsDebuff") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                Value::Bool(self.is_debuff(a0.as_int(), a1.as_int()))
            }
            (_, "DialogGetString") => Value::Str(self.dialog_get_string()),
            (_, "ScriptCall") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.script_call(&s0, &s1);
                Value::None
            }
            (_, "ScriptGetIntArray") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                Value::Int(self.script_get_int_array(&s0, &s1, a2.as_int()))
            }
            (_, "TargetID") => Value::Int(self.target_id()),
            (_, "TargetNameColor") => {
                let a0 = args.first().cloned().unwrap_or_default();
                color_value(self.target_name_color(a0.as_int()))
            }
            (_, "TargetIsServerObject") => Value::Bool(self.target_is_server_object()),
            (_, "TargetIsHPShowableNPC") => Value::Bool(self.target_is_hp_showable_npc()),
            (_, "StaticObjectNameID") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Int(self.static_object_name_id(a0.as_int()))
            }
            (_, "StaticObjectName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.static_object_name(a0.as_int()))
            }
            (_, "ServerObjectType") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Int(self.server_object_type(a0.as_int()))
            }
            (_, "StaticObjectShowHP") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Bool(self.static_object_show_hp(a0.as_int()))
            }
            (_, "ServerObjectHP") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Int(self.server_object_hp(a0.as_int()))
            }
            (_, "ServerObjectMaxHP") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Int(self.server_object_max_hp(a0.as_int()))
            }
            (_, "PartyMemberName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.party_member_name(a0.as_int()))
            }
            (_, "PlayerID") => Value::Int(self.player_id()),
            (_, "PetID") => Value::Int(self.pet_id()),
            (_, "ClanName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.clan_name(a0.as_int()))
            }
            (_, "ClanType") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.clan_type(a0.as_int()).map_or(Value::None, Value::Int)
            }
            (_, "ClanNameValue") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.clan_name_value(a0.as_int()).map_or(Value::None, Value::Int)
            }
            (_, "ClanCrestTexture") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.clan_crest_texture(a0.as_int()).map_or(Value::None, Value::Str)
            }
            (_, "ClanAllianceName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.clan_alliance_name(a0.as_int()))
            }
            (_, "ClanAllianceCrestTexture") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.clan_alliance_crest_texture(a0.as_int()).map_or(Value::None, Value::Str)
            }
            (_, "PlayerMoveType") => Value::Int(self.player_move_type()),
            (_, "PlayerEnvironment") => Value::Int(self.player_environment()),
            (_, "PlayerEXPRate") => Value::Float(self.player_exp_rate()),
            (_, "SkillIconName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                Value::Str(self.skill_icon_name(a0.as_int(), a1.as_int()))
            }
            (_, "SkillName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                Value::Str(self.skill_name(a0.as_int(), a1.as_int()))
            }
            (_, "SkillDescription") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                Value::Str(self.skill_description(a0.as_int(), a1.as_int()))
            }
            (_, "SetTextureWithObject") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.set_texture_with_object(&s0, &s1);
                Value::None
            }
            (_, "SetTextureSize") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_texture_size(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            (_, "SetTextureCtrlType") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.set_texture_ctrl_type(&s0, a1.as_int());
                Value::None
            }
            (_, "SetButtonTexture") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let s2 = a2.as_str();
                let a3 = args.get(3).cloned().unwrap_or_default();
                let s3 = a3.as_str();
                self.set_button_texture(&s0, &s1, &s2, &s3);
                Value::None
            }
            (_, "GetName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Str(self.get_name(&s0))
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetResizeFrameSize") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.set_resize_frame_size(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "SetItem") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                self.status_icon_set_item(&s0, a1.as_int(), a2.as_int(), &status_icon_info(&a3));
                Value::None
            }
            (_, "ReleaseFocus") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.release_focus(&s0);
                Value::None
            }
            ("UIAPI_TREECTRL" | "TreeHandle", "Clear") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.tree_clear(&s0);
                Value::None
            }
            ("UIAPI_TREECTRL" | "TreeHandle", "ShowScrollBar") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.tree_show_scroll_bar(&s0, a1.as_bool());
                Value::None
            }
            (_, "Log") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.log(&s0);
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "GetSelectedNum") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Int(self.item_get_selected_num(&s0))
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "GetItemNum") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Int(self.item_get_item_num(&s0))
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "ClearSelect") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                self.item_clear_select(&s0);
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "Add") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.item_add(&s0, &item_info(&a1));
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "Delete") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.item_delete(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "FindWithServerID") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                Value::Int(self.item_find_with_server_id(&s0, a1.as_int()))
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "Swap") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.item_swap(&s0, a1.as_int(), a2.as_int());
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "GetIndexAt") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let a3 = args.get(3).cloned().unwrap_or_default();
                let a4 = args.get(4).cloned().unwrap_or_default();
                Value::Int(self.item_get_index_at(&s0, a1.as_int(), a2.as_int(), a3.as_int(), a4.as_int()))
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "SetDisableTex") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.item_set_disable_tex(&s0, &s1);
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "SetRow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.item_set_row(&s0, a1.as_int());
                Value::None
            }
            ("UIAPI_TEXTBOX" | "TextBoxHandle", "SetTooltipString") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                self.set_tooltip_string(&s0, &s1);
                Value::None
            }
            (_, "RequestUseItem") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.request_use_item(a0.as_int());
                Value::None
            }
            (_, "RequestDestroyItem") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.request_destroy_item(a0.as_int(), a1.as_int());
                Value::None
            }
            (_, "RequestDropItem") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.request_drop_item(a0.as_int(), a1.as_int(), vector(&a2));
                Value::None
            }
            (_, "RequestUnequipItem") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.request_unequip_item(a0.as_int(), a1.as_int());
                Value::None
            }
            (_, "RequestCrystallizeItem") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                self.request_crystallize_item(a0.as_int(), a1.as_int());
                Value::None
            }
            (_, "RequestItemList") => {
                self.request_item_list();
                Value::None
            }
            (_, "RequestOpenMinimap") => {
                self.request_open_minimap();
                Value::None
            }
            (_, "RequestExit") => {
                self.request_exit();
                Value::None
            }
            (_, "RequestGetItemFromPet") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let a2 = args.get(2).cloned().unwrap_or_default();
                self.request_get_item_from_pet(a0.as_int(), a1.as_int(), a2.as_bool());
                Value::None
            }
            (_, "GetClassStep") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Int(self.get_class_step(a0.as_int()))
            }
            (_, "GetAdena") => Value::Int(self.get_adena()),
            (_, "IsKeyDown") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Bool(self.is_key_down(a0.as_int()))
            }
            (_, "MakeCostString") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Str(self.make_cost_string(&s0))
            }
            (_, "ConvertNumToText") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                Value::Str(self.convert_num_to_text(&s0))
            }
            (_, "CheckItemLimit") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                Value::Bool(self.check_item_limit(a0.as_int(), a1.as_int()))
            }
            (_, "GetClickLocation") => {
                let v = self.get_click_location();
                Value::vector(v.x, v.y, v.z)
            }
            (_, "IsStackableItem") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Bool(self.is_stackable_item(a0.as_int()))
            }
            (_, "PlayerHasCrystallizeAbility") => Value::Bool(self.player_has_crystallize_ability()),
            (_, "PlayerInventoryLimit") => Value::Int(self.player_inventory_limit()),
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "IsCrystallizable") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Bool(self.item_is_crystallizable(a0.as_int()))
            }
            (_, "HennaInfoCount") => Value::Int(self.henna_info_count()),
            (_, "HennaItemName") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.henna_item_name(a0.as_int()).map_or(Value::None, Value::Str)
            }
            (_, "HennaDescription") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.henna_description(a0.as_int()).map_or(Value::None, Value::Str)
            }
            (_, "HennaIconTex") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.henna_icon_tex(a0.as_int()).map_or(Value::None, Value::Str)
            }
            (_, "DialogShow") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                let a2 = args.get(2).cloned().unwrap_or_default();
                let s2 = a2.as_str();
                self.dialog_show(a0.as_int(), &s1, &s2);
                Value::None
            }
            (_, "DialogHide") => {
                self.dialog_hide();
                Value::None
            }
            (_, "DialogSetID") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.dialog_set_id(a0.as_int());
                Value::None
            }
            (_, "DialogSetParamInt") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.dialog_set_param_int(a0.as_int());
                Value::None
            }
            (_, "DialogSetReservedInt") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.dialog_set_reserved_int(a0.as_int());
                Value::None
            }
            (_, "DialogSetReservedInt2") => {
                let a0 = args.first().cloned().unwrap_or_default();
                self.dialog_set_reserved_int2(a0.as_int());
                Value::None
            }
            (_, "DialogGetTarget") => Value::Str(self.dialog_get_target()),
            (_, "DialogGetEditMessage") => Value::Str(self.dialog_get_edit_message()),
            (_, "DialogGetID") => Value::Int(self.dialog_get_id()),
            (_, "DialogGetReservedInt") => Value::Int(self.dialog_get_reserved_int()),
            (_, "DialogGetReservedInt2") => Value::Int(self.dialog_get_reserved_int2()),
            (_, "ScriptGetBool") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                Value::Bool(self.script_get_bool(&s0, &s1))
            }
            (_, "ScriptGetInt") => {
                let a0 = args.first().cloned().unwrap_or_default();
                let s0 = a0.as_str();
                let a1 = args.get(1).cloned().unwrap_or_default();
                let s1 = a1.as_str();
                Value::Int(self.script_get_int(&s0, &s1))
            }
            (_, "ClassLabelTexture") => {
                let a0 = args.first().cloned().unwrap_or_default();
                Value::Str(self.class_label_texture(a0.as_int()))
            }
            _ => {
                let key = format!("{class}.{function}");
                if !self.unhandled_natives.contains(&key) {
                    self.unhandled_natives.push(key);
                }
                Value::None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natives_reach_the_host_with_handles_as_control_names_and_out_params_written_back() {
        let mut host = ScriptHost { current: "StatusWnd".into(), ..Default::default() };
        host.player = Some(UserInfo { id: 7, name: "Killer".into(), cur_hp: 400, max_hp: 900, ..Default::default() });
        let mut args = vec![Value::Int(70)];
        host.call_native("UIEventManager", "RegisterEvent", None, &mut args).unwrap();
        assert_eq!(host.events.listeners(70), ["StatusWnd"]);
        let handle = Value::Handle("StatusWnd.HPBar".into());
        let mut args = vec![Value::Int(400), Value::Int(900)];
        host.call_native("UIAPI_STATUSBARCTRL", "SetPoint", Some(&handle), &mut args).unwrap();
        assert_eq!(host.overlay.point("StatusWnd.HPBar"), Some((400, 900)));
        let mut args = vec![Value::None];
        let found = host.call_native("UIScript", "GetPlayerInfo", None, &mut args).unwrap();
        assert_eq!(found, Value::Bool(true));
        assert_eq!(args[0].field("Name").as_str(), "Killer");
        assert_eq!(args[0].field("nMaxHP").as_int(), 900);
        let mut args = vec![Value::Str("ServerID=7 MaxHP=950 ".into()), Value::Str("MaxHP".into()), Value::Int(0)];
        host.call_native("UIEventManager", "ParseInt", None, &mut args).unwrap();
        assert_eq!(args[2], Value::Int(950));
        let mut args = vec![Value::Str("SystemMsgWnd".into())];
        host.call_native("UIAPI_WINDOW", "ShowWindow", None, &mut args).unwrap();
        assert!(host.is_show_window("SystemMsgWnd"));
        let mut args = vec![Value::Str("Nope".into())];
        host.call_native("UIDATA_NOPE", "GetNothing", None, &mut args).unwrap();
        assert_eq!(host.unhandled_natives, ["UIDATA_NOPE.GetNothing"]);
    }
}
