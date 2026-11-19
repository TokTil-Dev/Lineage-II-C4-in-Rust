use super::{ItemInfo, UiApi, Vector, WindowScript, app_atoi, parse_int, parse_string};

pub const WINDOW: &str = "InventoryWnd";
pub const EQUIP_SLOT_COUNT: usize = 15;
pub const EQUIPITEM_UNDERWEAR: usize = 0;
pub const EQUIPITEM_HEAD: usize = 1;
pub const EQUIPITEM_HAIR: usize = 2;
pub const EQUIPITEM_HAIR2: usize = 3;
pub const EQUIPITEM_NECK: usize = 4;
pub const EQUIPITEM_RHAND: usize = 5;
pub const EQUIPITEM_CHEST: usize = 6;
pub const EQUIPITEM_LHAND: usize = 7;
pub const EQUIPITEM_REAR: usize = 8;
pub const EQUIPITEM_LEAR: usize = 9;
pub const EQUIPITEM_GLOVES: usize = 10;
pub const EQUIPITEM_LEGS: usize = 11;
pub const EQUIPITEM_FEET: usize = 12;
pub const EQUIPITEM_RFINGER: usize = 13;
pub const EQUIPITEM_LFINGER: usize = 14;
pub const DIALOG_USE_RECIPE: i32 = 1111;
pub const DIALOG_POPUP: i32 = 2222;
pub const DIALOG_DROPITEM: i32 = 3333;
pub const DIALOG_DROPITEM_ASKCOUNT: i32 = 4444;
pub const DIALOG_DROPITEM_ALL: i32 = 5555;
pub const DIALOG_DESTROYITEM: i32 = 6666;
pub const DIALOG_DESTROYITEM_ALL: i32 = 7777;
pub const DIALOG_DESTROYITEM_ASKCOUNT: i32 = 8888;
pub const DIALOG_CRYSTALLIZE: i32 = 9999;
pub const DIALOG_DROPITEM_PETASKCOUNT: i32 = 10000;
pub const EQUIP_ITEM_HANDLES: [&str; EQUIP_SLOT_COUNT] = [
    "EquipItem_Underwear",
    "EquipItem_Head",
    "EquipItem_Hair",
    "EquipItem_Hair2",
    "EquipItem_Neck",
    "EquipItem_RHand",
    "EquipItem_Chest",
    "EquipItem_LHand",
    "EquipItem_REar",
    "EquipItem_LEar",
    "EquipItem_Gloves",
    "EquipItem_Legs",
    "EquipItem_Feet",
    "EquipItem_RFinger",
    "EquipItem_LFinger",
];
pub const DUALCAP_SLOTS: [usize; 6] = [EQUIPITEM_LHAND, EQUIPITEM_HEAD, EQUIPITEM_GLOVES, EQUIPITEM_LEGS, EQUIPITEM_FEET, EQUIPITEM_HAIR2];
pub const DUALCAP_TEXTURE: &str = "L2UI.InventoryWnd.Icon_dualcap";
pub const HENNA_ITEM: &str = "HennaItem";
pub const SORT_BUTTON: &str = "InventoryWnd.SortButton";
pub const ENCHANT_BUTTON: &str = "InventoryWnd.EnchantButton";
pub const ENCHANT_WINDOW: &str = "NPHRN_ItemEnchantWnd";
pub const ENCHANT_IDLE_VAR: &str = "L2jBRVar141";
pub const CHAT_EDIT_BOX: &str = "ChatWnd.ChatEditBox";
pub const PRIVATE_SHOP_WINDOW: &str = "PrivateShopWnd";
pub const PRIVATE_SHOP_TYPE_VAR: &str = "m_type";
pub const MINIMIZE_ICON: &str = "Interface.Icon_df_MenuWnd_Inventory";
pub const MINIMIZE_TOOLTIP: i32 = 195;
pub const KEY_SHIFT: i32 = 16;
pub const SOUND_TRASH_BASKET: i32 = 4;
pub const SOUND_INVENWND_OPEN: i32 = 13;
pub const SOUND_INVENWND_CLOSE: i32 = 14;
pub const ITEM_TYPE_WEAPON: i32 = 0;
pub const ITEM_TYPE_ARMOR: i32 = 1;
pub const ITEM_TYPE_ACCESSORY: i32 = 2;
pub const ITEM_TYPE_QUEST: i32 = 3;
pub const ITEM_TYPE_ASSET: i32 = 4;
pub const ITEM_TYPE_ETC: i32 = 5;
pub const WEAPON_TYPE_BOW: i32 = 6;
pub const WEAPON_TYPE_FISHING_ROD: i32 = 10;
pub const FOCUS_ORDER: [&str; 26] = [
    "InventoryWnd.ItemCount",
    "InventoryWnd.EquipItem_Underwear",
    "InventoryWnd.EquipItem_Head",
    "InventoryWnd.EquipItem_Hair",
    "InventoryWnd.EquipItem_Hair2",
    "InventoryWnd.EquipItem_Neck",
    "InventoryWnd.EquipItem_RHand",
    "InventoryWnd.EquipItem_Chest",
    "InventoryWnd.EquipItem_LHand",
    "InventoryWnd.EquipItem_REar",
    "InventoryWnd.EquipItem_LEar",
    "InventoryWnd.EquipItem_Gloves",
    "InventoryWnd.EquipItem_Legs",
    "InventoryWnd.EquipItem_Feet",
    "InventoryWnd.EquipItem_RFinger",
    "InventoryWnd.EquipItem_LFinger",
    "InventoryWnd.HennaItem",
    "InventoryWnd.InventoryItem",
    "InventoryWnd.QuestItem",
    "InventoryWnd.InventoryTab",
    "InventoryWnd.CrystallizeButton",
    "InventoryWnd.TrashButton",
    "InventoryWnd.AdenaText",
    "InventoryWnd.AdenaIcon",
    "InventoryWnd.InvenWeight",
    "InventoryWnd.BtnClose",
];

pub fn param_to_item_info(param: &str, info: &mut ItemInfo) {
    let int = |name: &str, slot: &mut i32| {
        if let Some(value) = parse_int(param, name) {
            *slot = value;
        }
    };
    int("classID", &mut info.class_id);
    int("level", &mut info.level);
    let text = |name: &str, slot: &mut String| {
        if let Some(value) = parse_string(param, name) {
            *slot = value;
        }
    };
    text("name", &mut info.name);
    text("additionalName", &mut info.additional_name);
    text("iconName", &mut info.icon_name);
    text("description", &mut info.description);
    let int = |name: &str, slot: &mut i32| {
        if let Some(value) = parse_int(param, name) {
            *slot = value;
        }
    };
    int("itemType", &mut info.item_type);
    int("serverID", &mut info.server_id);
    int("itemNum", &mut info.item_num);
    int("slotBitType", &mut info.slot_bit_type);
    int("enchanted", &mut info.enchanted);
    int("blessed", &mut info.blessed);
    int("damaged", &mut info.damaged);
    if let Some(value) = parse_int(param, "equipped") {
        info.equipped = value != 0;
    }
    int("price", &mut info.price);
    int("reserved", &mut info.reserved);
    int("defaultPrice", &mut info.default_price);
    int("refineryOp1", &mut info.refinery_op1);
    int("refineryOp2", &mut info.refinery_op2);
    int("currentDurability", &mut info.current_durability);
    int("weight", &mut info.weight);
    int("materialType", &mut info.material_type);
    int("weaponType", &mut info.weapon_type);
    int("physicalDamage", &mut info.physical_damage);
    int("magicalDamage", &mut info.magical_damage);
    int("shieldDefense", &mut info.shield_defense);
    int("shieldDefenseRate", &mut info.shield_defense_rate);
    int("durability", &mut info.durability);
    int("crystalType", &mut info.crystal_type);
    int("randomDamage", &mut info.random_damage);
    int("critical", &mut info.critical);
    int("hitModify", &mut info.hit_modify);
    int("attackSpeed", &mut info.attack_speed);
    int("mpConsume", &mut info.mp_consume);
    int("avoidModify", &mut info.avoid_modify);
    int("soulshotCount", &mut info.soulshot_count);
    int("spiritshotCount", &mut info.spiritshot_count);
    int("armorType", &mut info.armor_type);
    int("physicalDefense", &mut info.physical_defense);
    int("magicalDefense", &mut info.magical_defense);
    int("mpBonus", &mut info.mp_bonus);
    int("consumeType", &mut info.consume_type);
    int("ItemSubType", &mut info.item_sub_type);
    let text = |name: &str, slot: &mut String| {
        if let Some(value) = parse_string(param, name) {
            *slot = value;
        }
    };
    text("iconNameEx1", &mut info.icon_name_ex1);
    text("iconNameEx2", &mut info.icon_name_ex2);
    text("iconNameEx3", &mut info.icon_name_ex3);
    text("iconNameEx4", &mut info.icon_name_ex4);
    if let Some(value) = parse_int(param, "arrow") {
        info.arrow = value != 0;
    }
    if let Some(value) = parse_int(param, "recipe") {
        info.recipe = value != 0;
    }
}

fn sort_by_weight_desc(list: &mut [ItemInfo]) {
    let count = list.len();
    for i in 0..count {
        for j in 0..count - i {
            if j + 1 < count && list[j].weight < list[j + 1].weight {
                list.swap(j, j + 1);
            }
        }
    }
}

fn sort_recipes_like_the_script(list: &mut [ItemInfo], bolt_list: &[ItemInfo]) {
    let count = list.len();
    for i in 0..count {
        for j in 0..count - i {
            if j + 1 < count && list[j].weight < list[j + 1].weight {
                let temp = bolt_list.get(j).cloned().unwrap_or_default();
                list[j] = list[j + 1].clone();
                list[j + 1] = temp;
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct Script {
    pub window: String,
    pub item_order: Vec<i32>,
    pub click_location: Vector,
    pub ear_item_list: Vec<ItemInfo>,
    pub finger_item_list: Vec<ItemInfo>,
}

impl Script {
    pub fn new() -> Script {
        Script { window: WINDOW.to_string(), ..Default::default() }
    }

    pub fn inven_item(&self) -> String {
        format!("{}.InventoryItem", self.window)
    }

    pub fn quest_item(&self) -> String {
        format!("{}.QuestItem", self.window)
    }

    pub fn adena_text(&self) -> String {
        format!("{}.AdenaText", self.window)
    }

    fn equip(slot: usize) -> &'static str {
        EQUIP_ITEM_HANDLES[slot]
    }

    fn dialog_show(&self, api: &mut dyn UiApi, kind: i32, message: &str) {
        api.dialog_show(kind, message, &self.window);
    }

    fn dialog_is_mine(&self, api: &dyn UiApi) -> bool {
        api.dialog_get_target() == self.window
    }

    fn enchant_idle(api: &dyn UiApi) -> bool {
        api.script_get_bool(ENCHANT_WINDOW, ENCHANT_IDLE_VAR)
    }

    pub fn init_handle(&mut self, api: &mut dyn UiApi) {
        for slot in DUALCAP_SLOTS {
            api.item_set_disable_tex(Self::equip(slot), DUALCAP_TEXTURE);
        }
        api.set_tooltip_simple_text(SORT_BUTTON, "Sorting");
        api.set_tooltip_simple_text(ENCHANT_BUTTON, "Enchant Item");
    }

    pub fn on_click_enchant_button(&mut self, api: &mut dyn UiApi) {
        if !api.is_show_window(ENCHANT_WINDOW) {
            api.show_window(ENCHANT_WINDOW);
        } else {
            api.hide_window(ENCHANT_WINDOW);
        }
    }

    pub fn sort_item(&mut self, api: &mut dyn UiApi) {
        api.debug("Sorting Inven Item");
        let inven = self.inven_item();
        let mut asset = Vec::new();
        let mut weapon = Vec::new();
        let mut armor = Vec::new();
        let mut accessory = Vec::new();
        let mut etc = Vec::new();
        let ancient_crystal_enchant_am: Vec<ItemInfo> = Vec::new();
        let ancient_crystal_enchant_wp: Vec<ItemInfo> = Vec::new();
        let crystal_enchant_am: Vec<ItemInfo> = Vec::new();
        let crystal_enchant_wp: Vec<ItemInfo> = Vec::new();
        let mut bless_enchant_am = Vec::new();
        let mut bless_enchant_wp = Vec::new();
        let mut enchant_am = Vec::new();
        let mut enchant_wp = Vec::new();
        let inc_enchant_prop_am: Vec<ItemInfo> = Vec::new();
        let inc_enchant_prop_wp: Vec<ItemInfo> = Vec::new();
        let mut potion = Vec::new();
        let mut elixir = Vec::new();
        let mut arrow = Vec::new();
        let bolt: Vec<ItemInfo> = Vec::new();
        let mut recipe = Vec::new();
        let limit = api.item_get_item_num(&inven);
        for i in 0..limit {
            let item = api.item_get(&inven, i).unwrap_or_default();
            match item.item_type {
                ITEM_TYPE_ASSET => asset.push(item),
                ITEM_TYPE_WEAPON => weapon.push(item),
                ITEM_TYPE_ARMOR => armor.push(item),
                ITEM_TYPE_ACCESSORY => accessory.push(item),
                ITEM_TYPE_ETC => match item.item_sub_type {
                    22 => bless_enchant_am.push(item),
                    21 => bless_enchant_wp.push(item),
                    20 => enchant_am.push(item),
                    19 => enchant_wp.push(item),
                    3 => potion.push(item),
                    24 => elixir.push(item),
                    2 => arrow.push(item),
                    5 => recipe.push(item),
                    _ => etc.push(item),
                },
                _ => {
                    api.debug("huh???");
                    etc.push(item);
                }
            }
        }
        let mut ancient_crystal_enchant_am = ancient_crystal_enchant_am;
        let mut ancient_crystal_enchant_wp = ancient_crystal_enchant_wp;
        let mut crystal_enchant_am = crystal_enchant_am;
        let mut crystal_enchant_wp = crystal_enchant_wp;
        let mut inc_enchant_prop_am = inc_enchant_prop_am;
        let mut inc_enchant_prop_wp = inc_enchant_prop_wp;
        let mut bolt = bolt;
        sort_by_weight_desc(&mut asset);
        sort_by_weight_desc(&mut weapon);
        sort_by_weight_desc(&mut armor);
        sort_by_weight_desc(&mut accessory);
        sort_by_weight_desc(&mut ancient_crystal_enchant_am);
        sort_by_weight_desc(&mut ancient_crystal_enchant_wp);
        sort_by_weight_desc(&mut crystal_enchant_am);
        sort_by_weight_desc(&mut crystal_enchant_wp);
        sort_by_weight_desc(&mut bless_enchant_am);
        sort_by_weight_desc(&mut bless_enchant_wp);
        sort_by_weight_desc(&mut enchant_am);
        sort_by_weight_desc(&mut enchant_wp);
        sort_by_weight_desc(&mut inc_enchant_prop_am);
        sort_by_weight_desc(&mut inc_enchant_prop_wp);
        sort_by_weight_desc(&mut potion);
        sort_by_weight_desc(&mut elixir);
        sort_by_weight_desc(&mut arrow);
        sort_by_weight_desc(&mut bolt);
        sort_recipes_like_the_script(&mut recipe, &bolt);
        sort_by_weight_desc(&mut etc);
        let mut next_slot = 0;
        for list in [
            &asset,
            &weapon,
            &armor,
            &accessory,
            &crystal_enchant_am,
            &crystal_enchant_wp,
            &bless_enchant_am,
            &bless_enchant_wp,
            &enchant_am,
            &enchant_wp,
            &potion,
            &elixir,
            &arrow,
            &recipe,
            &etc,
        ] {
            for (i, item) in list.iter().enumerate() {
                api.item_set(&inven, next_slot + i as i32, item);
            }
            next_slot += list.len() as i32;
        }
        self.save_item_order(api);
    }

    fn chat_item_text(edit: &str, item: &ItemInfo) -> String {
        if item.item_num > 1 {
            return format!("{edit}{} ({}) ", item.name, item.item_num);
        }
        if item.item_type == ITEM_TYPE_WEAPON {
            if !item.additional_name.is_empty() && item.enchanted > 0 {
                format!("{edit}{} +{} ({}) ", item.name, item.enchanted, item.additional_name)
            } else if item.enchanted > 0 {
                format!("{edit}{} +{} ", item.name, item.enchanted)
            } else if !item.additional_name.is_empty() {
                format!("{edit}{} ({}) ", item.name, item.additional_name)
            } else {
                format!("{edit}{} ", item.name)
            }
        } else if item.item_type == ITEM_TYPE_ARMOR || item.item_type == ITEM_TYPE_ACCESSORY {
            if !item.additional_name.is_empty() && item.enchanted > 0 {
                format!("{edit}{} +{} ({}) ", item.name, item.enchanted, item.additional_name)
            } else if item.enchanted > 0 {
                format!("{edit}{} +{} ", item.name, item.enchanted)
            } else {
                format!("{edit}{} ", item.name)
            }
        } else {
            format!("{edit}{} ", item.name)
        }
    }

    pub fn on_select_item(&mut self, api: &mut dyn UiApi, handle: &str, _index: i32) {
        let edit = api.edit_get_string(CHAT_EDIT_BOX);
        if api.is_key_down(KEY_SHIFT) {
            let item = api.item_get_selected(handle).unwrap_or_default();
            api.set_chat_message(&Self::chat_item_text(&edit, &item));
            api.set_focus(CHAT_EDIT_BOX);
        }
        if handle == self.inven_item() {
            return;
        }
        if handle == self.quest_item() {
            return;
        }
        for slot in 0..EQUIP_SLOT_COUNT {
            if handle != Self::equip(slot) {
                api.item_clear_select(Self::equip(slot));
            }
        }
    }

    fn move_item(api: &mut dyn UiApi, handle: &str, from_index: i32, to_index: i32) {
        let mut from = from_index;
        while from < to_index {
            api.item_swap(handle, from, from + 1);
            from += 1;
        }
        while to_index < from {
            api.item_swap(handle, from, from - 1);
            from -= 1;
        }
    }

    fn move_to_end(api: &mut dyn UiApi, handle: &str, to_index: i32, count: i32) {
        let mut to = to_index;
        while to < count - 1 {
            api.item_swap(handle, to, to + 1);
            to += 1;
        }
    }

    pub fn drop_item(&mut self, api: &mut dyn UiApi, target: &str, info: &ItemInfo, x: i32, y: i32) {
        let src = info.drag_src_name.as_str();
        let from_equip = src.contains("EquipItem");
        if !(src == "InventoryItem" || src == "QuestItem" || from_equip || src == "PetInvenWnd") {
            return;
        }
        let inven = self.inven_item();
        let quest = self.quest_item();
        if target == "InventoryItem" {
            if src == "InventoryItem" {
                let to_index = api.item_get_index_at(&inven, x, y, 1, 1);
                if to_index >= 0 {
                    let from_index = api.item_find_with_server_id(&inven, info.server_id);
                    if to_index != from_index {
                        Self::move_item(api, &inven, from_index, to_index);
                    }
                } else {
                    let count = api.item_get_item_num(&inven);
                    Self::move_to_end(api, &inven, to_index, count);
                }
            } else if from_equip {
                api.request_unequip_item(info.server_id, info.slot_bit_type);
            } else if src == "PetInvenWnd" {
                if api.is_stackable_item(info.consume_type) && info.item_num > 1 {
                    if info.all_item_count > 0 {
                        if api.check_item_limit(info.class_id, info.all_item_count) {
                            api.request_get_item_from_pet(info.server_id, info.all_item_count, false);
                        }
                    } else {
                        api.dialog_set_id(DIALOG_DROPITEM_PETASKCOUNT);
                        api.dialog_set_reserved_int(info.server_id);
                        api.dialog_set_param_int(info.item_num);
                        let message = api.make_full_system_msg(&api.get_system_message(72), &info.name, "");
                        self.dialog_show(api, 6, &message);
                    }
                } else {
                    api.request_get_item_from_pet(info.server_id, 1, false);
                }
            }
        } else if target == "QuestItem" {
            if src == "QuestItem" {
                let to_index = api.item_get_index_at(&quest, x, y, 1, 1);
                if to_index >= 0 {
                    let from_index = api.item_find_with_server_id(&quest, info.server_id);
                    if to_index != from_index {
                        Self::move_item(api, &quest, from_index, to_index);
                    }
                } else {
                    let count = api.item_get_item_num(&inven);
                    Self::move_to_end(api, &inven, to_index, count);
                }
            }
        } else if target.contains("EquipItem") {
            if src == "PetInvenWnd" {
                api.request_get_item_from_pet(info.server_id, 1, true);
            } else if !from_equip && info.item_type != ITEM_TYPE_ETC {
                api.request_use_item(info.server_id);
            }
        } else if target == "TrashButton" {
            if api.is_stackable_item(info.consume_type) && info.item_num > 1 {
                if info.all_item_count > 0 {
                    api.dialog_set_id(DIALOG_DESTROYITEM_ALL);
                    api.dialog_set_reserved_int(info.server_id);
                    api.dialog_set_reserved_int2(info.all_item_count);
                    let message = api.make_full_system_msg(&api.get_system_message(74), &info.name, "");
                    self.dialog_show(api, 4, &message);
                } else {
                    api.dialog_set_id(DIALOG_DESTROYITEM_ASKCOUNT);
                    api.dialog_set_reserved_int(info.server_id);
                    api.dialog_set_param_int(info.item_num);
                    let message = api.make_full_system_msg(&api.get_system_message(73), &info.name, "");
                    self.dialog_show(api, 6, &message);
                }
            } else {
                api.dialog_set_id(DIALOG_DESTROYITEM);
                api.dialog_set_reserved_int(info.server_id);
                let message = api.make_full_system_msg(&api.get_system_message(74), &info.name, "");
                self.dialog_show(api, 4, &message);
            }
        } else if target == "CrystallizeButton"
            && (src == "InventoryItem" || from_equip)
            && api.player_has_crystallize_ability()
            && api.item_is_crystallizable(info.class_id)
        {
            api.dialog_set_id(DIALOG_CRYSTALLIZE);
            api.dialog_set_reserved_int(info.server_id);
            let message = api.make_full_system_msg(&api.get_system_message(336), &info.name, "");
            self.dialog_show(api, 4, &message);
        }
    }

    pub fn drop_item_source(&mut self, api: &mut dyn UiApi, target: &str, info: &ItemInfo) {
        if target != "Console" {
            return;
        }
        let src = info.drag_src_name.as_str();
        if !(src == "InventoryItem" || src == "QuestItem" || src.contains("EquipItem")) {
            return;
        }
        self.click_location = api.get_click_location();
        if api.is_stackable_item(info.consume_type) && info.item_num > 1 {
            if info.all_item_count > 0 {
                api.dialog_hide();
                api.dialog_set_id(DIALOG_DROPITEM_ALL);
                api.dialog_set_reserved_int(info.server_id);
                api.dialog_set_reserved_int2(info.all_item_count);
                let message = api.make_full_system_msg(&api.get_system_message(1833), &info.name, "");
                self.dialog_show(api, 4, &message);
            } else {
                api.dialog_hide();
                api.dialog_set_id(DIALOG_DROPITEM_ASKCOUNT);
                api.dialog_set_reserved_int(info.server_id);
                api.dialog_set_param_int(info.item_num);
                let message = api.make_full_system_msg(&api.get_system_message(71), &info.name, "");
                self.dialog_show(api, 6, &message);
            }
        } else {
            api.dialog_hide();
            api.dialog_set_id(DIALOG_DROPITEM);
            api.dialog_set_reserved_int(info.server_id);
            let message = api.make_full_system_msg(&api.get_system_message(400), &info.name, "");
            self.dialog_show(api, 4, &message);
        }
    }

    pub fn is_equip_item(info: &ItemInfo) -> bool {
        info.equipped
    }

    pub fn is_quest_item(info: &ItemInfo) -> bool {
        info.item_type == ITEM_TYPE_QUEST
    }

    pub fn handle_clear(&mut self, api: &mut dyn UiApi) {
        self.equip_item_clear(api);
        api.item_clear(&self.inven_item());
        api.item_clear(&self.quest_item());
        self.ear_item_list.clear();
        self.finger_item_list.clear();
    }

    pub fn equip_item_get_item_num(&self, api: &dyn UiApi) -> i32 {
        let mut count = 0;
        for slot in 0..EQUIP_SLOT_COUNT {
            if api.is_enable_window(Self::equip(slot)) {
                count += api.item_get_item_num(Self::equip(slot));
            }
        }
        count
    }

    pub fn equip_item_clear(&mut self, api: &mut dyn UiApi) {
        for slot in 0..EQUIP_SLOT_COUNT {
            api.item_clear(Self::equip(slot));
        }
    }

    pub fn equip_item_find(&self, api: &dyn UiApi, server_id: i32) -> bool {
        (0..EQUIP_SLOT_COUNT).any(|slot| api.item_find_with_server_id(Self::equip(slot), server_id) != -1)
    }

    pub fn equip_item_delete(&mut self, api: &mut dyn UiApi, server_id: i32) {
        for slot in 0..EQUIP_SLOT_COUNT {
            if api.item_find_with_server_id(Self::equip(slot), server_id) != -1 {
                api.item_clear(Self::equip(slot));
                if slot == EQUIPITEM_LHAND
                    && let Some(rhand) = api.item_get(Self::equip(EQUIPITEM_RHAND), 0)
                    && rhand.slot_bit_type == 16384
                {
                    api.item_clear(Self::equip(EQUIPITEM_LHAND));
                    api.item_add(Self::equip(EQUIPITEM_LHAND), &rhand);
                    api.disable_window(Self::equip(EQUIPITEM_LHAND));
                }
            }
        }
    }

    pub fn ear_item_update(&mut self, api: &mut dyn UiApi) {
        let mut left = None;
        let mut right = None;
        let mut i = 0;
        while i < self.ear_item_list.len() {
            match self.is_l_or_r_ear(api, self.ear_item_list[i].server_id) {
                -1 => left = Some(i),
                0 => {
                    self.ear_item_list.remove(i);
                }
                1 => right = Some(i),
                _ => {}
            }
            i += 1;
        }
        if let Some(index) = left.filter(|index| *index < self.ear_item_list.len()) {
            api.item_clear(Self::equip(EQUIPITEM_LEAR));
            api.item_add(Self::equip(EQUIPITEM_LEAR), &self.ear_item_list[index].clone());
        }
        if let Some(index) = right.filter(|index| *index < self.ear_item_list.len()) {
            api.item_clear(Self::equip(EQUIPITEM_REAR));
            api.item_add(Self::equip(EQUIPITEM_REAR), &self.ear_item_list[index].clone());
        }
    }

    pub fn finger_item_update(&mut self, api: &mut dyn UiApi) {
        let mut left = None;
        let mut right = None;
        let mut i = 0;
        while i < self.finger_item_list.len() {
            match self.is_l_or_r_finger(api, self.finger_item_list[i].server_id) {
                -1 => left = Some(i),
                0 => {
                    self.finger_item_list.remove(i);
                }
                1 => right = Some(i),
                _ => {}
            }
            i += 1;
        }
        if let Some(index) = left.filter(|index| *index < self.finger_item_list.len()) {
            api.item_clear(Self::equip(EQUIPITEM_LFINGER));
            api.item_add(Self::equip(EQUIPITEM_LFINGER), &self.finger_item_list[index].clone());
        }
        if let Some(index) = right.filter(|index| *index < self.finger_item_list.len()) {
            api.item_clear(Self::equip(EQUIPITEM_RFINGER));
            api.item_add(Self::equip(EQUIPITEM_RFINGER), &self.finger_item_list[index].clone());
        }
    }

    fn replace_slot(api: &mut dyn UiApi, slot: usize, info: &ItemInfo, disable: bool) {
        api.item_clear(Self::equip(slot));
        api.item_add(Self::equip(slot), info);
        if disable {
            api.disable_window(Self::equip(slot));
        }
    }

    pub fn equip_item_update(&mut self, api: &mut dyn UiApi, info: &ItemInfo) {
        let mut target: Option<usize> = None;
        match info.slot_bit_type {
            1 => target = Some(EQUIPITEM_UNDERWEAR),
            2 | 4 | 6 => {
                if !self.ear_item_list.iter().any(|item| item.server_id == info.server_id) {
                    self.ear_item_list.push(info.clone());
                }
                self.ear_item_update(api);
            }
            8 => target = Some(EQUIPITEM_NECK),
            16 | 32 | 48 => {
                if !self.finger_item_list.iter().any(|item| item.server_id == info.server_id) {
                    self.finger_item_list.push(info.clone());
                }
                self.finger_item_update(api);
            }
            64 => {
                target = Some(EQUIPITEM_HEAD);
                api.enable_window(Self::equip(EQUIPITEM_HEAD));
            }
            128 => target = Some(EQUIPITEM_RHAND),
            256 => {
                target = Some(EQUIPITEM_LHAND);
                api.enable_window(Self::equip(EQUIPITEM_LHAND));
            }
            512 => {
                target = Some(EQUIPITEM_GLOVES);
                api.enable_window(Self::equip(EQUIPITEM_GLOVES));
            }
            1024 => target = Some(EQUIPITEM_CHEST),
            2048 => {
                target = Some(EQUIPITEM_LEGS);
                api.enable_window(Self::equip(EQUIPITEM_LEGS));
            }
            4096 => {
                target = Some(EQUIPITEM_FEET);
                api.enable_window(Self::equip(EQUIPITEM_FEET));
            }
            8192 => target = Some(EQUIPITEM_UNDERWEAR),
            16384 => {
                target = Some(EQUIPITEM_RHAND);
                let mut clear_lhand = true;
                if Self::is_bow_or_fishing_rod(info)
                    && let Some(lhand) = api.item_get(Self::equip(EQUIPITEM_LHAND), 0)
                    && Self::is_arrow(&lhand)
                {
                    clear_lhand = false;
                }
                if clear_lhand {
                    if !info.icon_name_ex1.is_empty() {
                        let mut rhand = info.clone();
                        let mut lhand = info.clone();
                        rhand.icon_index = 1;
                        lhand.icon_index = 2;
                        Self::replace_slot(api, EQUIPITEM_RHAND, &rhand, false);
                        Self::replace_slot(api, EQUIPITEM_LHAND, &lhand, true);
                        target = None;
                    } else {
                        Self::replace_slot(api, EQUIPITEM_LHAND, info, true);
                    }
                }
            }
            32768 => {
                target = Some(EQUIPITEM_CHEST);
                let mut legs = info.clone();
                legs.icon_name = info.icon_name_ex2.clone();
                Self::replace_slot(api, EQUIPITEM_LEGS, &legs, true);
            }
            65536 => target = Some(EQUIPITEM_HAIR),
            131072 => {
                target = Some(EQUIPITEM_CHEST);
                let mut hair2 = info.clone();
                let mut gloves = info.clone();
                let mut legs = info.clone();
                let mut feet = info.clone();
                hair2.icon_name = info.icon_name_ex1.clone();
                gloves.icon_name = info.icon_name_ex2.clone();
                legs.icon_name = info.icon_name_ex3.clone();
                feet.icon_name = info.icon_name_ex4.clone();
                Self::replace_slot(api, EQUIPITEM_HEAD, &hair2, true);
                Self::replace_slot(api, EQUIPITEM_GLOVES, &gloves, true);
                Self::replace_slot(api, EQUIPITEM_LEGS, &legs, true);
                Self::replace_slot(api, EQUIPITEM_FEET, &feet, true);
            }
            262144 => {
                target = Some(EQUIPITEM_HAIR2);
                api.enable_window(Self::equip(EQUIPITEM_HAIR2));
            }
            524288 => {
                target = Some(EQUIPITEM_HAIR);
                Self::replace_slot(api, EQUIPITEM_HAIR2, info, true);
            }
            _ => {}
        }
        if let Some(slot) = target {
            api.item_clear(Self::equip(slot));
            api.item_add(Self::equip(slot), info);
        }
    }

    pub fn handle_show_window(&mut self, api: &mut dyn UiApi) {
        self.load_item_order(api);
        api.show_window(&self.window);
        api.set_focus(&self.window);
    }

    pub fn handle_hide_window(&mut self, api: &mut dyn UiApi) {
        api.hide_window(&self.window);
    }

    pub fn handle_add_item(&mut self, api: &mut dyn UiApi, param: &str) {
        let mut info = ItemInfo::default();
        param_to_item_info(param, &mut info);
        if Self::is_equip_item(&info) {
            self.equip_item_update(api, &info);
        } else if Self::is_quest_item(&info) {
            api.item_add(&self.quest_item(), &info);
        } else {
            api.item_add(&self.inven_item(), &info);
        }
    }

    fn move_last_to_front(api: &mut dyn UiApi, handle: &str) {
        let mut index = api.item_get_item_num(handle) - 1;
        while index > 0 {
            api.item_swap(handle, index - 1, index);
            index -= 1;
        }
    }

    pub fn handle_update_item(&mut self, api: &mut dyn UiApi, param: &str) {
        let kind = parse_string(param, "type").unwrap_or_default();
        let mut info = ItemInfo::default();
        param_to_item_info(param, &mut info);
        let inven = self.inven_item();
        let quest = self.quest_item();
        if kind == "add" {
            if Self::is_equip_item(&info) {
                self.equip_item_update(api, &info);
            } else if Self::is_quest_item(&info) {
                api.item_add(&quest, &info);
                Self::move_last_to_front(api, &quest);
            } else {
                api.item_add(&inven, &info);
                Self::move_last_to_front(api, &inven);
            }
        } else if kind == "update" {
            if Self::is_equip_item(&info) {
                if self.equip_item_find(api, info.server_id) {
                    self.equip_item_update(api, &info);
                } else {
                    let index = api.item_find_with_server_id(&inven, info.server_id);
                    if index != -1 {
                        api.item_delete(&inven, index);
                    }
                    self.equip_item_update(api, &info);
                }
            } else if Self::is_quest_item(&info) {
                let index = api.item_find_with_server_id(&quest, info.server_id);
                if index != -1 {
                    api.item_set(&quest, index, &info);
                } else {
                    self.equip_item_delete(api, info.server_id);
                    api.item_add(&quest, &info);
                }
            } else {
                let index = api.item_find_with_server_id(&inven, info.server_id);
                if index != -1 {
                    api.item_set(&inven, index, &info);
                } else {
                    self.equip_item_delete(api, info.server_id);
                    api.item_add(&inven, &info);
                    Self::move_last_to_front(api, &inven);
                }
            }
        } else if kind == "delete" {
            if Self::is_equip_item(&info) {
                self.equip_item_delete(api, info.server_id);
            } else if Self::is_quest_item(&info) {
                let index = api.item_find_with_server_id(&quest, info.server_id);
                api.item_delete(&quest, index);
            } else {
                let index = api.item_find_with_server_id(&inven, info.server_id);
                api.item_delete(&inven, index);
            }
        }
        self.set_adena_text(api);
        self.update_item_count(api);
    }

    pub fn handle_item_list_end(&mut self, api: &mut dyn UiApi) {
        self.set_adena_text(api);
        self.update_item_count(api);
        self.order_item(api);
    }

    pub fn update_henna_info(&mut self, api: &mut dyn UiApi) {
        let mut class_step = 0;
        if let Some(player) = api.get_player_info() {
            class_step = api.get_class_step(player.sub_class);
            match class_step {
                1..=3 => api.item_set_row(HENNA_ITEM, class_step),
                _ => api.item_set_row(HENNA_ITEM, 0),
            }
        }
        api.item_clear(HENNA_ITEM);
        let mut count = api.henna_info_count();
        if count > class_step {
            count = class_step;
        }
        for index in 0..count {
            let Some((henna_id, is_active)) = api.henna_info(index) else { continue };
            let Some(name) = api.henna_item_name(henna_id) else { continue };
            let Some(description) = api.henna_description(henna_id) else { continue };
            let Some(icon_name) = api.henna_icon_tex(henna_id) else { continue };
            let info = ItemInfo { name, description, icon_name, disabled: is_active == 0, ..Default::default() };
            api.item_add(HENNA_ITEM, &info);
        }
    }

    pub fn set_adena_text(&mut self, api: &mut dyn UiApi) {
        let adena = api.get_adena().to_string();
        let text = api.make_cost_string(&adena);
        api.set_text(&self.adena_text(), &text);
        let tooltip = api.convert_num_to_text(&adena);
        api.set_tooltip_string(&self.adena_text(), &tooltip);
    }

    pub fn use_item(&mut self, api: &mut dyn UiApi, handle: &str, index: i32) {
        let Some(info) = api.item_get(handle, index) else { return };
        if info.recipe {
            api.dialog_set_reserved_int(info.server_id);
            api.dialog_set_id(DIALOG_USE_RECIPE);
            let message = api.get_system_message(798);
            self.dialog_show(api, 4, &message);
        } else if info.pop_msg_num > 0 {
            api.dialog_set_id(DIALOG_POPUP);
            api.dialog_set_reserved_int(info.server_id);
            let message = api.get_system_message(info.pop_msg_num);
            self.dialog_show(api, 4, &message);
        } else {
            api.request_use_item(info.server_id);
        }
    }

    pub fn save_item_order(&mut self, api: &mut dyn UiApi) {
        let inven = self.inven_item();
        let count = api.item_get_item_num(&inven).max(0) as usize;
        self.item_order = (0..count).map(|i| api.item_get(&inven, i as i32).unwrap_or_default().class_id).collect();
        api.save_inventory_order(&self.item_order);
    }

    pub fn load_item_order(&mut self, api: &mut dyn UiApi) {
        if let Some(order) = api.load_inventory_order() {
            self.item_order = order;
        }
    }

    pub fn order_item(&mut self, api: &mut dyn UiApi) {
        let inven = self.inven_item();
        let mut new_item_index = 0;
        let count = api.item_get_item_num(&inven);
        for item_index in 0..count {
            let info = api.item_get(&inven, item_index).unwrap_or_default();
            let matched = self.item_order.contains(&info.class_id);
            if !matched {
                api.item_swap(&inven, item_index, new_item_index);
                new_item_index += 1;
            }
        }
        for order_index in 0..self.item_order.len() {
            for item_index in 0..count {
                let info = api.item_get(&inven, item_index).unwrap_or_default();
                if info.class_id == self.item_order[order_index] {
                    api.item_swap(&inven, item_index, new_item_index);
                    new_item_index += 1;
                    break;
                }
            }
        }
    }

    pub fn get_my_inventory_limit(&self, api: &dyn UiApi) -> i32 {
        api.player_inventory_limit()
    }

    pub fn update_item_count(&mut self, api: &mut dyn UiApi) {
        let count = api.item_get_item_num(&self.inven_item()) + api.item_get_item_num(&self.quest_item()) + self.equip_item_get_item_num(api);
        let limit = self.get_my_inventory_limit(api);
        api.set_text(&format!("{}.ItemCount", self.window), &format!("({count}/{limit})"));
    }

    pub fn handle_dialog_ok(&mut self, api: &mut dyn UiApi) {
        if !self.dialog_is_mine(api) {
            return;
        }
        let id = api.dialog_get_id();
        let reserved = api.dialog_get_reserved_int();
        let reserved2 = api.dialog_get_reserved_int2();
        let mut count = app_atoi(&api.dialog_get_edit_message());
        if id == DIALOG_USE_RECIPE || id == DIALOG_POPUP {
            api.request_use_item(reserved);
        } else if id == DIALOG_DROPITEM {
            api.request_drop_item(reserved, 1, self.click_location);
        } else if id == DIALOG_DROPITEM_ASKCOUNT {
            if count == 0 {
                count = 1;
            }
            api.request_drop_item(reserved, count, self.click_location);
        } else if id == DIALOG_DROPITEM_ALL {
            api.request_drop_item(reserved, reserved2, self.click_location);
        } else if id == DIALOG_DESTROYITEM {
            api.request_destroy_item(reserved, 1);
            api.play_console_sound(super::InterfaceSound::TrashBasket);
        } else if id == DIALOG_DESTROYITEM_ASKCOUNT {
            api.request_destroy_item(reserved, count);
            api.play_console_sound(super::InterfaceSound::TrashBasket);
        } else if id == DIALOG_DESTROYITEM_ALL {
            api.request_destroy_item(reserved, reserved2);
            api.play_console_sound(super::InterfaceSound::TrashBasket);
        } else if id == DIALOG_CRYSTALLIZE {
            api.request_crystallize_item(reserved, 1);
            api.play_console_sound(super::InterfaceSound::TrashBasket);
        } else if id == DIALOG_DROPITEM_PETASKCOUNT {
            api.request_get_item_from_pet(reserved, count, false);
        }
    }

    pub fn handle_update_user_info(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window(&self.window) {
            self.ear_item_update(api);
            self.finger_item_update(api);
        }
    }

    pub fn handle_toggle_window(&mut self, api: &mut dyn UiApi) {
        if api.is_show_window(&self.window) {
            api.hide_window(&self.window);
            api.play_console_sound(super::InterfaceSound::InvenWndClose);
        } else if self.is_show_inventory_wnd_upon_event(api) {
            api.request_item_list();
            api.show_window(&self.window);
            api.play_console_sound(super::InterfaceSound::InvenWndOpen);
        }
    }

    pub fn is_show_inventory_wnd_upon_event(&self, api: &dyn UiApi) -> bool {
        for name in ["WarehouseWnd", "WarehouseWnd", "TradeWnd", "ShopWnd", "MultiSellWnd", "DeliverWnd"] {
            if api.is_show_window(name) {
                return false;
            }
        }
        if api.is_show_window(PRIVATE_SHOP_WINDOW) && api.script_get_int(PRIVATE_SHOP_WINDOW, PRIVATE_SHOP_TYPE_VAR) == 2 {
            return false;
        }
        true
    }

    pub fn is_l_or_r_ear(&self, api: &dyn UiApi, server_id: i32) -> i32 {
        let (left_ear, right_ear, _, _) = api.get_accessory_server_id().unwrap_or_default();
        if server_id == left_ear {
            -1
        } else if server_id == right_ear {
            1
        } else {
            0
        }
    }

    pub fn is_l_or_r_finger(&self, api: &dyn UiApi, server_id: i32) -> i32 {
        let (_, _, left_finger, right_finger) = api.get_accessory_server_id().unwrap_or_default();
        if server_id == left_finger {
            -1
        } else if server_id == right_finger {
            1
        } else {
            0
        }
    }

    pub fn is_bow_or_fishing_rod(info: &ItemInfo) -> bool {
        info.weapon_type == WEAPON_TYPE_BOW || info.weapon_type == WEAPON_TYPE_FISHING_ROD
    }

    pub fn is_arrow(info: &ItemInfo) -> bool {
        info.arrow
    }

    pub fn set_focus(&mut self, api: &mut dyn UiApi) {
        for name in FOCUS_ORDER {
            api.set_focus(name);
        }
    }
}

impl WindowScript for Script {
    fn on_load(&mut self, api: &mut dyn UiApi) {
        for event in [2570, 2580, 2590, 2600, 2610, 2620, 2630, 2631, 260, 180, 1710] {
            api.register_event(event);
        }
        self.init_handle(api);
    }

    fn on_event(&mut self, api: &mut dyn UiApi, event_id: i32, param: &str) {
        if !Self::enchant_idle(api) {
            return;
        }
        match event_id {
            2570 => self.handle_clear(api),
            2580 => self.handle_show_window(api),
            2590 => self.handle_hide_window(api),
            2600 => self.handle_add_item(api, param),
            2610 => self.handle_update_item(api, param),
            2620 => self.handle_item_list_end(api),
            2630 | 260 => self.update_henna_info(api),
            2631 => self.handle_toggle_window(api),
            1710 => self.handle_dialog_ok(api),
            180 => self.handle_update_user_info(api),
            _ => {}
        }
    }

    fn on_show(&mut self, api: &mut dyn UiApi) {
        let crystallize = format!("{}.CrystallizeButton", self.window);
        if api.player_has_crystallize_ability() {
            api.show_window(&crystallize);
        } else {
            api.hide_window(&crystallize);
        }
        self.set_adena_text(api);
        self.update_item_count(api);
        self.update_henna_info(api);
        api.show_window(SORT_BUTTON);
        self.set_focus(api);
    }

    fn on_hide(&mut self, api: &mut dyn UiApi) {
        self.save_item_order(api);
    }

    fn on_db_click_item_with_handle(&mut self, api: &mut dyn UiApi, handle: &str, index: i32) {
        self.use_item(api, handle, index);
    }

    fn on_r_click_item_with_handle(&mut self, api: &mut dyn UiApi, handle: &str, index: i32) {
        self.use_item(api, handle, index);
    }

    fn on_click_button(&mut self, api: &mut dyn UiApi, id: &str) {
        match id {
            "SortButton" => self.sort_item(api),
            "BtnClose" => api.hide_window(WINDOW),
            "BtnMin" => api.iconize(WINDOW, MINIMIZE_ICON, MINIMIZE_TOOLTIP),
            "EnchantButton" => self.on_click_enchant_button(api),
            _ => {}
        }
    }

    fn on_select_item_with_handle(&mut self, api: &mut dyn UiApi, handle: &str, index: i32) {
        self.on_select_item(api, handle, index);
    }

    fn on_drop_item(&mut self, api: &mut dyn UiApi, target: &str, info: &ItemInfo, x: i32, y: i32) {
        self.drop_item(api, target, info, x, y);
    }

    fn on_drop_item_source(&mut self, api: &mut dyn UiApi, target: &str, info: &ItemInfo) {
        self.drop_item_source(api, target, info);
    }
}

#[cfg(test)]
mod tests {
    use super::super::mock::RecordingApi;
    use super::*;

    fn idle_api() -> RecordingApi {
        let mut api = RecordingApi::default();
        api.script_bools.push((ENCHANT_WINDOW.into(), ENCHANT_IDLE_VAR.into(), true));
        api.inventory_limit = 80;
        api.adena = 1234;
        api
    }

    fn item(server_id: i32, class_id: i32, item_type: i32, weight: i32) -> ItemInfo {
        ItemInfo { server_id, class_id, item_type, weight, item_num: 1, name: format!("item{server_id}"), ..Default::default() }
    }

    #[test]
    fn on_load_registers_the_eleven_events_and_sets_the_dualcap_textures() {
        let mut api = idle_api();
        let mut script = Script::new();
        script.on_load(&mut api);
        let events: Vec<&String> = api.calls.iter().filter(|call| call.starts_with("RegisterEvent")).collect();
        assert_eq!(events.len(), 11);
        assert_eq!(events[0], "RegisterEvent(2570)");
        assert_eq!(events[10], "RegisterEvent(1710)");
        let dualcap: Vec<&String> = api.calls.iter().filter(|call| call.starts_with("SetDisableTex")).collect();
        assert_eq!(dualcap.len(), 6);
        assert_eq!(dualcap[0], "SetDisableTex(EquipItem_LHand,L2UI.InventoryWnd.Icon_dualcap)");
        assert!(api.calls.contains(&"SetTooltipCustomType(InventoryWnd.SortButton,Sorting)".to_string()));
    }

    #[test]
    fn param_to_item_info_reads_every_field_the_script_parses() {
        let mut info = ItemInfo::default();
        param_to_item_info("classID=57 level=0 name=\u{7f}Adena\u{7f} itemType=4 serverID=99 itemNum=500 slotBitType=0 equipped=0 weight=1 consumeType=2 ItemSubType=0 arrow=0 recipe=1 iconNameEx1=\u{7f}a.b\u{7f}", &mut info);
        assert_eq!((info.class_id, info.name.as_str(), info.item_type, info.server_id, info.item_num), (57, "Adena", 4, 99, 500));
        assert_eq!((info.equipped, info.recipe, info.arrow, info.consume_type, info.icon_name_ex1.as_str()), (false, true, false, 2, "a.b"));
    }

    #[test]
    fn events_are_ignored_while_the_enchant_window_is_busy() {
        let mut api = RecordingApi::default();
        let mut script = Script::new();
        script.on_event(&mut api, 2580, "");
        assert!(api.calls.is_empty());
        api.script_bools.push((ENCHANT_WINDOW.into(), ENCHANT_IDLE_VAR.into(), true));
        script.on_event(&mut api, 2580, "");
        assert_eq!(api.calls, ["ShowWindow(InventoryWnd)", "SetFocus(InventoryWnd)"]);
        api.calls.clear();
        script.on_event(&mut api, 2590, "");
        assert_eq!(api.calls, ["HideWindow(InventoryWnd)"]);
    }

    #[test]
    fn add_item_routes_equipped_quest_and_plain_items_and_update_moves_new_items_to_the_front() {
        let mut api = idle_api();
        let mut script = Script::new();
        script.on_event(&mut api, 2600, "serverID=1 classID=10 itemType=5 itemNum=1");
        script.on_event(&mut api, 2600, "serverID=2 classID=11 itemType=3 itemNum=1");
        script.on_event(&mut api, 2600, "serverID=3 classID=12 itemType=1 equipped=1 slotBitType=1024");
        assert_eq!(api.item_get_item_num("InventoryWnd.InventoryItem"), 1);
        assert_eq!(api.item_get_item_num("InventoryWnd.QuestItem"), 1);
        assert_eq!(api.item_get("EquipItem_Chest", 0).map(|item| item.server_id), Some(3));
        api.calls.clear();
        script.on_event(&mut api, 2610, "type=add serverID=4 classID=13 itemType=5 itemNum=1");
        assert_eq!(api.item_get("InventoryWnd.InventoryItem", 0).map(|item| item.server_id), Some(4));
        assert!(api.calls.contains(&"SwapItems(InventoryWnd.InventoryItem,0,1)".to_string()));
        assert!(api.calls.contains(&"SetText(InventoryWnd.AdenaText,cost:1234)".to_string()));
        assert!(api.calls.contains(&"SetText(InventoryWnd.ItemCount,(4/80))".to_string()));
        script.on_event(&mut api, 2610, "type=update serverID=4 classID=13 itemType=5 itemNum=7");
        assert_eq!(api.item_get("InventoryWnd.InventoryItem", 0).map(|item| item.item_num), Some(7));
        script.on_event(&mut api, 2610, "type=delete serverID=4 classID=13 itemType=5");
        assert_eq!(api.item_get_item_num("InventoryWnd.InventoryItem"), 1);
        script.on_event(&mut api, 2610, "type=delete serverID=3 classID=12 itemType=1 equipped=1 slotBitType=1024");
        assert_eq!(api.item_get_item_num("EquipItem_Chest"), 0);
    }

    #[test]
    fn two_handed_weapons_fill_both_hands_and_full_armor_fills_four_slots() {
        let mut api = idle_api();
        let mut script = Script::new();
        let mut sword = item(5, 50, 0, 100);
        sword.slot_bit_type = 16384;
        sword.icon_name_ex1 = "icon.two".into();
        sword.equipped = true;
        script.equip_item_update(&mut api, &sword);
        assert_eq!(api.item_get("EquipItem_RHand", 0).map(|item| item.icon_index), Some(1));
        assert_eq!(api.item_get("EquipItem_LHand", 0).map(|item| item.icon_index), Some(2));
        assert!(api.calls.contains(&"DisableWindow(EquipItem_LHand)".to_string()));
        let mut armor = item(6, 60, 1, 200);
        armor.slot_bit_type = 131072;
        armor.icon_name_ex1 = "h".into();
        armor.icon_name_ex2 = "g".into();
        armor.icon_name_ex3 = "l".into();
        armor.icon_name_ex4 = "f".into();
        script.equip_item_update(&mut api, &armor);
        assert_eq!(api.item_get("EquipItem_Head", 0).map(|item| item.icon_name), Some("h".into()));
        assert_eq!(api.item_get("EquipItem_Gloves", 0).map(|item| item.icon_name), Some("g".into()));
        assert_eq!(api.item_get("EquipItem_Legs", 0).map(|item| item.icon_name), Some("l".into()));
        assert_eq!(api.item_get("EquipItem_Feet", 0).map(|item| item.icon_name), Some("f".into()));
        assert_eq!(api.item_get("EquipItem_Chest", 0).map(|item| item.server_id), Some(6));
        script.equip_item_delete(&mut api, 7);
        assert_eq!(api.item_get("EquipItem_LHand", 0).map(|item| item.server_id), Some(5));
    }

    #[test]
    fn ear_and_finger_lists_follow_the_accessory_server_ids() {
        let mut api = idle_api();
        api.accessory = Some((21, 22, 31, 32));
        let mut script = Script::new();
        for (server_id, slot) in [(21, 2), (22, 4), (23, 6)] {
            let mut ring = item(server_id, 70, 2, 1);
            ring.slot_bit_type = slot;
            script.equip_item_update(&mut api, &ring);
        }
        assert_eq!(script.ear_item_list.iter().map(|item| item.server_id).collect::<Vec<_>>(), [21, 22]);
        assert_eq!(api.item_get("EquipItem_LEar", 0).map(|item| item.server_id), Some(21));
        assert_eq!(api.item_get("EquipItem_REar", 0).map(|item| item.server_id), Some(22));
        for (server_id, slot) in [(31, 16), (32, 32)] {
            let mut ring = item(server_id, 71, 2, 1);
            ring.slot_bit_type = slot;
            script.equip_item_update(&mut api, &ring);
        }
        assert_eq!(api.item_get("EquipItem_LFinger", 0).map(|item| item.server_id), Some(31));
        assert_eq!(api.item_get("EquipItem_RFinger", 0).map(|item| item.server_id), Some(32));
        assert_eq!(script.is_l_or_r_ear(&api, 22), 1);
        assert_eq!(script.is_l_or_r_finger(&api, 99), 0);
    }

    #[test]
    fn sort_groups_by_type_then_by_weight_descending_and_saves_the_order() {
        let mut api = idle_api();
        let mut script = Script::new();
        let inven = script.inven_item();
        let mut potion = item(1, 100, 5, 5);
        potion.item_sub_type = 3;
        api.item_add(&inven, &potion);
        api.item_add(&inven, &item(2, 101, 0, 10));
        api.item_add(&inven, &item(3, 102, 0, 30));
        api.item_add(&inven, &item(4, 103, 4, 0));
        let mut unknown = item(5, 104, 9, 1);
        unknown.item_sub_type = 0;
        api.item_add(&inven, &unknown);
        script.sort_item(&mut api);
        let order: Vec<i32> = (0..5).map(|i| api.item_get(&inven, i).map(|item| item.server_id).unwrap()).collect();
        assert_eq!(order, [4, 3, 2, 1, 5]);
        assert_eq!(api.inventory_order, Some(vec![103, 102, 101, 100, 104]));
        assert!(api.calls.contains(&"SaveInventoryOrder([103, 102, 101, 100, 104])".to_string()));
    }

    #[test]
    fn order_item_puts_unknown_classes_first_then_follows_the_saved_order() {
        let mut api = idle_api();
        let mut script = Script::new();
        let inven = script.inven_item();
        for (server_id, class_id) in [(1, 10), (2, 20), (3, 30), (4, 40)] {
            api.item_add(&inven, &item(server_id, class_id, 5, 1));
        }
        api.inventory_order = Some(vec![40, 20]);
        script.handle_show_window(&mut api);
        script.handle_item_list_end(&mut api);
        let order: Vec<i32> = (0..4).map(|i| api.item_get(&inven, i).map(|item| item.server_id).unwrap()).collect();
        assert_eq!(order, [1, 3, 4, 2]);
    }

    #[test]
    fn drop_item_moves_swaps_unequips_and_opens_the_dialogs_of_the_script() {
        let mut api = idle_api();
        let mut script = Script::new();
        let inven = script.inven_item();
        for server_id in 1..=4 {
            api.item_add(&inven, &item(server_id, server_id, 5, 1));
        }
        api.item_index_at.push((inven.clone(), (10, 10), 3));
        let mut dragged = item(1, 1, 5, 1);
        dragged.drag_src_name = "InventoryItem".into();
        script.on_drop_item(&mut api, "InventoryItem", &dragged, 10, 10);
        let order: Vec<i32> = (0..4).map(|i| api.item_get(&inven, i).map(|item| item.server_id).unwrap()).collect();
        assert_eq!(order, [2, 3, 4, 1]);
        api.calls.clear();
        let mut equipped = item(9, 9, 1, 1);
        equipped.drag_src_name = "EquipItem_Chest".into();
        equipped.slot_bit_type = 1024;
        script.on_drop_item(&mut api, "InventoryItem", &equipped, 0, 0);
        assert_eq!(api.calls, ["RequestUnequipItem(9,1024)"]);
        api.calls.clear();
        let mut stack = item(7, 7, 5, 1);
        stack.drag_src_name = "InventoryItem".into();
        stack.consume_type = 1;
        stack.item_num = 20;
        script.on_drop_item(&mut api, "TrashButton", &stack, 0, 0);
        assert_eq!(api.calls, ["DialogSetID(8888)", "DialogSetReservedInt(7)", "DialogSetParamInt(20)", "DialogShow(6,sysmsg73|item7|,InventoryWnd)"]);
        api.calls.clear();
        script.on_drop_item(&mut api, "EquipItem_RHand", &dragged, 0, 0);
        assert_eq!(api.calls, Vec::<String>::new());
        let mut weapon = item(8, 8, 0, 1);
        weapon.drag_src_name = "InventoryItem".into();
        script.on_drop_item(&mut api, "EquipItem_RHand", &weapon, 0, 0);
        assert_eq!(api.calls, ["RequestUseItem(8)"]);
        api.calls.clear();
        let mut pet = item(5, 5, 5, 1);
        pet.drag_src_name = "PetInvenWnd".into();
        script.on_drop_item(&mut api, "EquipItem_RHand", &pet, 0, 0);
        assert_eq!(api.calls, ["RequestGetItemFromPet(5,1,true)"]);
        api.calls.clear();
        let mut other = item(6, 6, 5, 1);
        other.drag_src_name = "WarehouseWnd".into();
        script.on_drop_item(&mut api, "InventoryItem", &other, 0, 0);
        assert!(api.calls.is_empty());
    }

    #[test]
    fn dropping_on_the_console_asks_before_dropping_and_dialog_ok_sends_the_request() {
        let mut api = idle_api();
        api.click_location = Vector { x: 1.0, y: 2.0, z: 3.0 };
        let mut script = Script::new();
        let mut dragged = item(1, 1, 5, 1);
        dragged.drag_src_name = "InventoryItem".into();
        script.on_drop_item_source(&mut api, "Console", &dragged);
        assert_eq!(api.calls, ["DialogHide()", "DialogSetID(3333)", "DialogSetReservedInt(1)", "DialogShow(4,sysmsg400|item1|,InventoryWnd)"]);
        api.calls.clear();
        script.on_event(&mut api, 1710, "");
        assert_eq!(api.calls, ["RequestDropItem(1,1,1,2,3)"]);
        api.calls.clear();
        api.dialog_target = "OtherWnd".into();
        script.on_event(&mut api, 1710, "");
        assert!(api.calls.is_empty());
        api.dialog_target = WINDOW.into();
        api.dialog_id = DIALOG_DROPITEM_ASKCOUNT;
        api.dialog_edit = "0".into();
        script.on_event(&mut api, 1710, "");
        assert_eq!(api.calls, ["RequestDropItem(1,1,1,2,3)"]);
        api.calls.clear();
        api.dialog_id = DIALOG_DESTROYITEM_ASKCOUNT;
        api.dialog_edit = "12".into();
        script.on_event(&mut api, 1710, "");
        assert_eq!(api.calls, ["RequestDestroyItem(1,12)", "PlayConsoleSound(TrashBasket)"]);
    }

    #[test]
    fn use_item_asks_for_recipes_and_pop_messages_and_toggle_follows_the_other_windows() {
        let mut api = idle_api();
        let mut script = Script::new();
        let inven = script.inven_item();
        let mut recipe = item(1, 1, 5, 1);
        recipe.recipe = true;
        api.item_add(&inven, &recipe);
        let mut pop = item(2, 2, 5, 1);
        pop.pop_msg_num = 55;
        api.item_add(&inven, &pop);
        api.item_add(&inven, &item(3, 3, 5, 1));
        api.calls.clear();
        script.on_db_click_item_with_handle(&mut api, &inven, 0);
        script.on_r_click_item_with_handle(&mut api, &inven, 1);
        script.use_item(&mut api, &inven, 2);
        assert_eq!(
            api.calls,
            [
                "DialogSetReservedInt(1)",
                "DialogSetID(1111)",
                "DialogShow(4,sysmsg798,InventoryWnd)",
                "DialogSetID(2222)",
                "DialogSetReservedInt(2)",
                "DialogShow(4,sysmsg55,InventoryWnd)",
                "RequestUseItem(3)"
            ]
        );
        api.calls.clear();
        script.on_event(&mut api, 2631, "");
        assert_eq!(api.calls, ["RequestItemList()", "ShowWindow(InventoryWnd)", "PlayConsoleSound(InvenWndOpen)"]);
        api.calls.clear();
        script.on_event(&mut api, 2631, "");
        assert_eq!(api.calls, ["HideWindow(InventoryWnd)", "PlayConsoleSound(InvenWndClose)"]);
        api.show_window("TradeWnd");
        api.calls.clear();
        script.on_event(&mut api, 2631, "");
        assert!(api.calls.is_empty());
        api.hide_window("TradeWnd");
        api.show_window(PRIVATE_SHOP_WINDOW);
        api.script_ints.push((PRIVATE_SHOP_WINDOW.into(), PRIVATE_SHOP_TYPE_VAR.into(), 2));
        assert!(!script.is_show_inventory_wnd_upon_event(&api));
    }

    #[test]
    fn on_show_sets_adena_count_henna_and_focus_and_shift_select_writes_the_item_to_the_chat() {
        let mut api = idle_api();
        api.crystallize_ability = true;
        api.class_step = 2;
        api.player = Some(super::super::UserInfo { sub_class: 3, ..Default::default() });
        api.hennas = vec![(1, 1), (2, 0), (3, 1)];
        let mut script = Script::new();
        script.on_show(&mut api);
        assert_eq!(api.calls[0], "ShowWindow(InventoryWnd.CrystallizeButton)");
        assert!(api.calls.contains(&"SetRow(HennaItem,2)".to_string()));
        assert_eq!(api.item_get_item_num(HENNA_ITEM), 2);
        assert_eq!(api.item_get(HENNA_ITEM, 1).map(|item| item.disabled), Some(true));
        assert_eq!(api.calls.last().map(String::as_str), Some("SetFocus(InventoryWnd.BtnClose)"));
        assert!(api.calls.contains(&"SetTooltipString(InventoryWnd.AdenaText,text:1234)".to_string()));
        api.calls.clear();
        let inven = script.inven_item();
        let mut sword = item(1, 1, 0, 1);
        sword.enchanted = 4;
        sword.additional_name = "Focus".into();
        api.item_add(&inven, &sword);
        api.item_selected.push((inven.clone(), 0));
        api.keys_down.push(KEY_SHIFT);
        api.set_text(CHAT_EDIT_BOX, "hi ");
        api.calls.clear();
        script.on_select_item_with_handle(&mut api, &inven, 0);
        assert_eq!(api.calls, ["SetChatMessage(hi item1 +4 (Focus) )", "SetFocus(ChatWnd.ChatEditBox)"]);
        api.calls.clear();
        script.on_select_item_with_handle(&mut api, "EquipItem_Head", 0);
        assert_eq!(api.calls.iter().filter(|call| call.starts_with("ClearSelect")).count(), 14);
        assert_eq!(Script::chat_item_text("", &item(2, 2, 5, 1)), "item2 ");
        let mut many = item(3, 3, 5, 1);
        many.item_num = 9;
        assert_eq!(Script::chat_item_text("", &many), "item3 (9) ");
    }

    #[test]
    fn close_and_minimize_buttons_and_the_enchant_toggle_follow_the_script() {
        let mut api = idle_api();
        let mut script = Script::new();
        script.on_click_button(&mut api, "BtnClose");
        script.on_click_button(&mut api, "BtnMin");
        script.on_click_button(&mut api, "EnchantButton");
        script.on_click_button(&mut api, "EnchantButton");
        assert_eq!(
            api.calls,
            [
                "HideWindow(InventoryWnd)",
                "Iconize(InventoryWnd,Interface.Icon_df_MenuWnd_Inventory,195)",
                "ShowWindow(NPHRN_ItemEnchantWnd)",
                "HideWindow(NPHRN_ItemEnchantWnd)"
            ]
        );
        api.calls.clear();
        script.on_hide(&mut api);
        assert_eq!(api.calls, ["SaveInventoryOrder([])"]);
    }
}
