use std::collections::HashMap;

use killer_ui::color::Argb;
use l2_uscript::vm::{StructValue, Value};

use super::host_impl::ScriptHost;
use super::vm_host::{color, color_value, item_info};
use super::{CustomTooltip, DrawItemInfo, DrawItemType, ItemInfo, TreeNodeInfo, TreeNodeItemInfo};

pub const TREE_NAME_SEPARATOR: &str = ".";
pub const TREE_LIST_SEPARATOR: &str = "|";
pub const TREE_ITEM_BLANK: i32 = 0;
pub const TREE_ITEM_TEXT: i32 = 1;
pub const TREE_ITEM_TEXTURE: i32 = 2;
pub const STYLE_SETTLED: u32 = 0x80000;
pub const STYLE_DRAGGABLE: u32 = 0x400000;
pub const STYLE_ALWAYS_ON_TOP: u32 = 0x800000;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ItemSlot {
    pub info: ItemInfo,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ItemWindowState {
    pub items: Vec<ItemSlot>,
    pub selected: Option<usize>,
    pub cols: Option<i32>,
    pub rows: Option<i32>,
    pub faded: bool,
    pub disable_tex: String,
    pub scroll_bar: Option<bool>,
}

impl ItemWindowState {
    pub fn item_num(&self) -> i32 {
        self.items.len() as i32
    }

    pub fn selected_num(&self) -> i32 {
        match self.selected {
            Some(index) if index < self.items.len() => index as i32,
            _ => -1,
        }
    }

    pub fn get(&self, index: i32) -> Option<&ItemInfo> {
        usize::try_from(index).ok().and_then(|index| self.items.get(index)).map(|slot| &slot.info)
    }

    pub fn get_selected(&self) -> Option<&ItemInfo> {
        let selected = self.selected_num();
        (selected != -1).then(|| self.get(selected)).flatten()
    }

    pub fn selected_item_id(&self) -> i32 {
        self.get_selected().map_or(-1, |info| info.class_id)
    }

    pub fn find_with_server_id(&self, server_id: i32) -> i32 {
        self.items.iter().position(|slot| slot.info.server_id == server_id).map_or(-1, |index| index as i32)
    }

    pub fn find_with_class_id(&self, class_id: i32) -> i32 {
        self.items.iter().position(|slot| slot.info.class_id == class_id).map_or(-1, |index| index as i32)
    }

    pub fn add(&mut self, info: ItemInfo) {
        self.items.push(ItemSlot { info, selected: false });
    }

    pub fn set(&mut self, index: i32, info: ItemInfo) -> bool {
        match usize::try_from(index).ok().and_then(|index| self.items.get_mut(index)) {
            Some(slot) => {
                *slot = ItemSlot { info, selected: false };
                true
            }
            None => false,
        }
    }

    pub fn delete(&mut self, index: i32) -> bool {
        let Ok(index) = usize::try_from(index) else { return false };
        if index >= self.items.len() {
            return false;
        }
        self.items.remove(index);
        self.selected = None;
        true
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.selected = None;
        self.faded = false;
    }

    pub fn clear_select(&mut self) {
        for slot in &mut self.items {
            slot.selected = false;
        }
    }

    pub fn swap(&mut self, first: i32, second: i32) {
        if first < 0 || second < 0 {
            return;
        }
        let (first, second) = (first as usize, second as usize);
        if first < self.items.len() && second < self.items.len() && first != second {
            self.items.swap(first, second);
        }
    }

    pub fn cells(&self) -> Vec<killer_ui::xdat_view::Cell> {
        self.items
            .iter()
            .map(|slot| killer_ui::xdat_view::Cell {
                icon: slot.info.icon_name.clone(),
                selected: slot.selected,
                count: (slot.info.item_num > 1).then_some(slot.info.item_num),
                equipped: slot.info.equipped,
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListTexture {
    pub texture: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub u: i32,
    pub v: i32,
    pub ul: i32,
    pub vl: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListData {
    pub text: String,
    pub reserved: String,
    pub use_text_color: bool,
    pub text_color: Argb,
    pub reserved_ints: [i32; 3],
    pub texture: String,
    pub texture_width: i32,
    pub texture_height: i32,
    pub textures: Vec<ListTexture>,
}

impl Default for ListData {
    fn default() -> Self {
        ListData {
            text: String::new(),
            reserved: String::new(),
            use_text_color: false,
            text_color: Argb(0),
            reserved_ints: [0; 3],
            texture: String::new(),
            texture_width: 0,
            texture_height: 0,
            textures: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListRecord {
    pub cells: Vec<ListData>,
    pub reserved: String,
    pub reserved_ints: [i32; 3],
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ListCtrlState {
    pub records: Vec<ListRecord>,
    pub selected: Option<usize>,
    pub top: usize,
    pub scroll_bar: Option<bool>,
}

impl ListCtrlState {
    pub fn count(&self) -> i32 {
        self.records.len() as i32
    }

    pub fn selected_index(&self) -> i32 {
        self.selected.map_or(-1, |index| index as i32)
    }

    pub fn insert(&mut self, record: ListRecord, max_rows: Option<i32>) -> bool {
        if max_rows.is_some_and(|max| max <= self.count()) {
            return false;
        }
        self.records.push(record);
        true
    }

    pub fn modify(&mut self, index: i32, record: &ListRecord) -> bool {
        let Some(target) = usize::try_from(index).ok().and_then(|index| self.records.get_mut(index)) else {
            return false;
        };
        target.reserved = record.reserved.clone();
        target.reserved_ints = record.reserved_ints;
        for (column, data) in record.cells.iter().enumerate() {
            if let Some(cell) = target.cells.get_mut(column) {
                cell.text = data.text.clone();
                cell.reserved = data.reserved.clone();
                cell.reserved_ints = data.reserved_ints;
            }
        }
        true
    }

    pub fn delete(&mut self, index: i32) {
        if let Ok(index) = usize::try_from(index)
            && index < self.records.len()
        {
            self.records.remove(index);
        }
    }

    pub fn delete_all(&mut self) {
        self.records.clear();
        self.selected = None;
        self.top = 0;
    }

    pub fn get(&self, index: i32) -> Option<&ListRecord> {
        usize::try_from(index).ok().and_then(|index| self.records.get(index))
    }

    pub fn get_selected(&self) -> Option<&ListRecord> {
        self.get(self.selected_index())
    }

    pub fn set_selected(&mut self, index: i32, move_to_row: bool, show_rows: Option<i32>) {
        let Ok(index) = usize::try_from(index) else { return };
        if index >= self.records.len() {
            return;
        }
        self.selected = Some(index);
        if let Some(shown) = show_rows.filter(|_| move_to_row)
            && index as i32 > self.top as i32 + shown - 1
        {
            let top = index as i32 - shown + 1;
            if top > -1 {
                self.top = top as usize;
            }
        }
    }

    pub fn painter_records(&self) -> Vec<Vec<killer_ui::xdat_paint_lists::ListCell>> {
        self.records
            .iter()
            .map(|record| {
                record
                    .cells
                    .iter()
                    .map(|data| killer_ui::xdat_paint_lists::ListCell {
                        text: data.text.clone(),
                        color: if data.use_text_color { data.text_color } else { killer_ui::color::BUTTON_TEXT },
                        icons: data
                            .textures
                            .iter()
                            .map(|texture| killer_ui::xdat_paint_lists::CellIcon {
                                texture: texture.texture.clone(),
                                x: texture.x,
                                y: texture.y,
                                w: texture.width,
                                h: texture.height,
                                u: texture.u,
                                v: texture.v,
                                uw: texture.ul,
                                vh: texture.vl,
                            })
                            .collect(),
                        texture: (!data.texture.is_empty())
                            .then(|| (data.texture.clone(), data.texture_width, data.texture_height)),
                        show_text: true,
                    })
                    .collect()
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ComboEntry {
    pub sysstring: i32,
    pub sysmsg: i32,
    pub text: String,
    pub reserved: i32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ComboState {
    pub entries: Vec<ComboEntry>,
    pub selected: Option<usize>,
}

impl ComboState {
    pub fn count(&self) -> i32 {
        self.entries.len() as i32
    }

    pub fn selected_num(&self) -> i32 {
        self.selected.map_or(-1, |index| index as i32)
    }

    pub fn reserved(&self, index: i32) -> i32 {
        usize::try_from(index).ok().and_then(|index| self.entries.get(index)).map_or(-1, |entry| entry.reserved)
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        self.selected = None;
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EditState {
    pub text: String,
    pub max_length: i32,
    pub highlight: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeItemState {
    pub info: TreeNodeItemInfo,
    pub stop_mouse_focus: bool,
    pub height: i32,
    pub text_id: i32,
    pub color: Option<Argb>,
    pub draw_one_line: bool,
    pub texture_mouse_on: String,
    pub texture_expanded: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeNodeState {
    pub name: String,
    pub parent: Option<String>,
    pub expanded: bool,
    pub info: TreeNodeInfo,
    pub items: Vec<TreeItemState>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TreeState {
    pub nodes: Vec<TreeNodeState>,
    pub scroll_bar: Option<bool>,
}

impl TreeState {
    pub fn find(&self, name: &str) -> Option<&TreeNodeState> {
        if name.is_empty() {
            return None;
        }
        self.nodes.iter().find(|node| node.name == name)
    }

    fn find_mut(&mut self, name: &str) -> Option<&mut TreeNodeState> {
        if name.is_empty() {
            return None;
        }
        self.nodes.iter_mut().find(|node| node.name == name)
    }

    pub fn children(&self, name: &str) -> impl Iterator<Item = &TreeNodeState> {
        self.nodes.iter().filter(move |node| node.parent.as_deref() == Some(name))
    }

    pub fn insert_node(&mut self, parent: &str, info: TreeNodeInfo) -> String {
        if info.name.is_empty() {
            return String::new();
        }
        let parent = self.find(parent).map(|node| node.name.clone());
        let name = match &parent {
            None => info.name.clone(),
            Some(parent) => format!("{parent}{TREE_NAME_SEPARATOR}{}", info.name),
        };
        if parent.is_some() && self.find(&name).is_some() {
            return String::new();
        }
        self.nodes.push(TreeNodeState {
            name: name.clone(),
            expanded: parent.is_none(),
            parent,
            info,
            items: Vec::new(),
        });
        name
    }

    pub fn insert_node_item(&mut self, node: &str, item: TreeItemState) {
        if let Some(node) = self.find_mut(node) {
            node.items.push(item);
        }
    }

    pub fn delete_node(&mut self, name: &str) -> bool {
        if self.find(name).is_none() {
            return false;
        }
        let prefix = format!("{name}{TREE_NAME_SEPARATOR}");
        self.nodes.retain(|node| node.name != name && !node.name.starts_with(&prefix));
        true
    }

    pub fn set_expanded(&mut self, name: &str, expanded: bool) {
        if let Some(node) = self.find_mut(name) {
            node.expanded = expanded;
        }
    }

    pub fn is_expanded(&self, name: &str) -> bool {
        self.find(name).is_some_and(|node| node.expanded)
    }

    pub fn expanded_nodes(&self, name: &str) -> String {
        let Some(node) = self.find(name) else { return String::new() };
        self.children(&node.name)
            .filter(|child| child.expanded)
            .map(|child| child.name.as_str())
            .collect::<Vec<_>>()
            .join(TREE_LIST_SEPARATOR)
    }

    pub fn child_nodes(&self, name: &str) -> String {
        let Some(node) = self.find(name) else { return String::new() };
        self.children(&node.name).map(|child| child.name.as_str()).collect::<Vec<_>>().join(TREE_LIST_SEPARATOR)
    }

    pub fn parent_node(&self, name: &str) -> String {
        self.find(name).and_then(|node| node.parent.clone()).unwrap_or_default()
    }

    pub fn set_node_item_text(&mut self, name: &str, text_id: i32, text: &str) {
        if let Some(node) = self.find_mut(name)
            && let Some(item) = node.items.iter_mut().find(|item| item.text_id == text_id)
        {
            item.info.text = text.to_string();
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HtmlState {
    pub file: String,
    pub source: String,
    pub buffer: String,
    pub page_lock: bool,
    pub bypasses: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProgressState {
    pub time: i32,
    pub pos: i32,
    pub running: bool,
    pub starts: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AnimState {
    pub playing: bool,
    pub paused: bool,
    pub loop_count: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ButtonTextures {
    pub fore: String,
    pub back: String,
    pub highlight: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct WindowStyle {
    pub added: u32,
    pub removed: u32,
}

impl WindowStyle {
    pub fn set(&mut self, style: u32, on: bool) {
        if on {
            self.added |= style;
            self.removed &= !style;
        } else {
            self.removed |= style;
            self.added &= !style;
        }
    }

    pub fn get(&self, style: u32) -> Option<bool> {
        if self.added & style != 0 {
            Some(true)
        } else if self.removed & style != 0 {
            Some(false)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ControlState {
    pub item_windows: HashMap<String, ItemWindowState>,
    pub list_ctrls: HashMap<String, ListCtrlState>,
    pub list_limits: HashMap<String, (i32, i32)>,
    pub trees: HashMap<String, TreeState>,
    pub combos: HashMap<String, ComboState>,
    pub edits: HashMap<String, EditState>,
    pub html: HashMap<String, HtmlState>,
    pub progress: HashMap<String, ProgressState>,
    pub weights: HashMap<String, i32>,
    pub sliders: HashMap<String, i32>,
    pub anims: HashMap<String, AnimState>,
    pub draw_panels: HashMap<String, Vec<DrawItemInfo>>,
    pub button_names: HashMap<String, (i32, String)>,
    pub button_textures: HashMap<String, ButtonTextures>,
    pub texture_uv: HashMap<String, (i32, i32)>,
    pub clan_crests: HashMap<String, i32>,
    pub object_textures: Vec<String>,
    pub window_titles: HashMap<String, String>,
    pub window_styles: HashMap<String, WindowStyle>,
    pub resize_frames: HashMap<String, (i32, i32)>,
    pub size_rels: HashMap<String, (f32, f32, i32, i32)>,
    pub icons: HashMap<String, (String, i32)>,
    pub alarms: Vec<String>,
    pub text_aligns: HashMap<String, i32>,
    pub status_icon_sizes: HashMap<String, i32>,
    pub effect_buttons: HashMap<String, i32>,
    pub tab_merges: Vec<(String, i32)>,
    pub scroll_positions: HashMap<String, i32>,
    pub scroll_bars: HashMap<String, bool>,
    pub custom_tooltips: HashMap<String, CustomTooltip>,
}

impl ControlState {
    pub fn item_window(&self, ctrl: &str) -> Option<&ItemWindowState> {
        self.item_windows.get(ctrl)
    }

    pub fn list_ctrl(&self, ctrl: &str) -> Option<&ListCtrlState> {
        self.list_ctrls.get(ctrl)
    }

    pub fn combo(&self, ctrl: &str) -> Option<&ComboState> {
        self.combos.get(ctrl)
    }

    pub fn edit(&self, ctrl: &str) -> Option<&EditState> {
        self.edits.get(ctrl)
    }

    pub fn tree(&self, ctrl: &str) -> Option<&TreeState> {
        self.trees.get(ctrl)
    }

    pub fn combo_items(
        &self,
        ctrl: &str,
        sysstring: &dyn Fn(i32) -> Option<String>,
    ) -> Option<(Vec<String>, Option<usize>)> {
        let combo = self.combos.get(ctrl)?;
        let items = combo.entries.iter().map(|entry| combo_text(entry, sysstring)).collect();
        Some((items, combo.selected))
    }
}

pub fn combo_text(entry: &ComboEntry, sysstring: &dyn Fn(i32) -> Option<String>) -> String {
    if entry.sysstring >= 0 { sysstring(entry.sysstring).unwrap_or_default() } else { entry.text.clone() }
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
            i("AllItemCount", info.all_item_count),
        ],
    }))
}

pub fn list_data(value: &Value) -> ListData {
    ListData {
        text: value.field("szData").as_str(),
        reserved: value.field("szReserved").as_str(),
        use_text_color: value.field("bUseTextColor").as_bool(),
        text_color: color(&value.field("TextColor")),
        reserved_ints: [
            value.field("nReserved1").as_int(),
            value.field("nReserved2").as_int(),
            value.field("nReserved3").as_int(),
        ],
        texture: value.field("szTexture").as_str(),
        texture_width: value.field("nTextureWidth").as_int(),
        texture_height: value.field("nTextureHeight").as_int(),
        textures: match value.field("arrTexture") {
            Value::Array(items) => items
                .iter()
                .map(|item| ListTexture {
                    texture: item.field("objTex").as_str(),
                    x: item.field("X").as_int(),
                    y: item.field("Y").as_int(),
                    width: item.field("Width").as_int(),
                    height: item.field("Height").as_int(),
                    u: item.field("U").as_int(),
                    v: item.field("V").as_int(),
                    ul: item.field("UL").as_int(),
                    vl: item.field("VL").as_int(),
                })
                .collect(),
            _ => Vec::new(),
        },
    }
}

pub fn list_record(value: &Value) -> ListRecord {
    ListRecord {
        cells: match value.field("LVDataList") {
            Value::Array(items) => items.iter().map(list_data).collect(),
            _ => Vec::new(),
        },
        reserved: value.field("szReserved").as_str(),
        reserved_ints: [
            value.field("nReserved1").as_int(),
            value.field("nReserved2").as_int(),
            value.field("nReserved3").as_int(),
        ],
    }
}

pub fn list_data_value(data: &ListData) -> Value {
    let s = |name: &str, value: &str| (name.to_string(), Value::Str(value.to_string()));
    let i = |name: &str, value: i32| (name.to_string(), Value::Int(value));
    Value::Struct(Box::new(StructValue {
        name: "LVData".into(),
        fields: vec![
            s("szData", &data.text),
            s("szReserved", &data.reserved),
            ("bUseTextColor".into(), Value::Bool(data.use_text_color)),
            ("TextColor".into(), color_value(data.text_color)),
            i("nReserved1", data.reserved_ints[0]),
            i("nReserved2", data.reserved_ints[1]),
            i("nReserved3", data.reserved_ints[2]),
            s("szTexture", &data.texture),
            i("nTextureWidth", data.texture_width),
            i("nTextureHeight", data.texture_height),
            (
                "arrTexture".into(),
                Value::Array(
                    data.textures
                        .iter()
                        .map(|texture| {
                            Value::Struct(Box::new(StructValue {
                                name: "LVTexture".into(),
                                fields: vec![
                                    s("objTex", &texture.texture),
                                    i("X", texture.x),
                                    i("Y", texture.y),
                                    i("Width", texture.width),
                                    i("Height", texture.height),
                                    i("U", texture.u),
                                    i("V", texture.v),
                                    i("UL", texture.ul),
                                    i("VL", texture.vl),
                                ],
                            }))
                        })
                        .collect(),
                ),
            ),
        ],
    }))
}

pub fn list_record_value(record: &ListRecord) -> Value {
    let i = |name: &str, value: i32| (name.to_string(), Value::Int(value));
    Value::Struct(Box::new(StructValue {
        name: "LVDataRecord".into(),
        fields: vec![
            ("LVDataList".into(), Value::Array(record.cells.iter().map(list_data_value).collect())),
            ("szReserved".into(), Value::Str(record.reserved.clone())),
            i("nReserved1", record.reserved_ints[0]),
            i("nReserved2", record.reserved_ints[1]),
            i("nReserved3", record.reserved_ints[2]),
        ],
    }))
}

fn struct_color(value: &Value) -> Option<Argb> {
    let color = color(value);
    (color.0 != 0).then_some(color)
}

pub fn draw_item_info(value: &Value) -> DrawItemInfo {
    DrawItemInfo {
        kind: match value.field("eType").as_int() {
            TREE_ITEM_TEXT => DrawItemType::Text,
            TREE_ITEM_TEXTURE => DrawItemType::Texture,
            _ => DrawItemType::Blank,
        },
        offset_x: value.field("nOffSetX").as_int(),
        offset_y: value.field("nOffSetY").as_int(),
        line_break: value.field("bLineBreak").as_bool(),
        height: value.field("b_nHeight").as_int(),
        text_id: value.field("t_ID").as_int(),
        text: value.field("t_strText").as_str(),
        color: struct_color(&value.field("t_color")),
        draw_one_line: value.field("t_bDrawOneLine").as_bool(),
        texture_width: value.field("u_nTextureWidth").as_int(),
        texture_height: value.field("u_nTextureHeight").as_int(),
        texture_u_width: value.field("u_nTextureUWidth").as_int(),
        texture_u_height: value.field("u_nTextureUHeight").as_int(),
        texture: value.field("u_strTexture").as_str(),
        condition: value.field("Condition").as_str(),
    }
}

pub fn custom_tooltip(value: &Value) -> CustomTooltip {
    CustomTooltip {
        minimum_width: value.field("MinimumWidth").as_int(),
        simple_line_count: value.field("SimpleLineCount").as_int(),
        draw_list: match value.field("DrawList") {
            Value::Array(items) => items.iter().map(draw_item_info).collect(),
            _ => Vec::new(),
        },
    }
}

pub fn tree_node_info(value: &Value) -> TreeNodeInfo {
    TreeNodeInfo {
        name: value.field("strName").as_str(),
        offset_x: value.field("nOffSetX").as_int(),
        offset_y: value.field("nOffSetY").as_int(),
        show_button: value.field("bShowButton").as_int(),
        tooltip: custom_tooltip(&value.field("ToolTip")),
    }
}

pub fn tree_item_state(value: &Value) -> TreeItemState {
    let kind = value.field("eType").as_int();
    let width = value.field("u_nTextureWidth").as_int();
    let height = value.field("u_nTextureHeight").as_int();
    let u_width = value.field("u_nTextureUWidth").as_int();
    let u_height = value.field("u_nTextureUHeight").as_int();
    TreeItemState {
        info: TreeNodeItemInfo {
            kind,
            offset_x: value.field("nOffSetX").as_int(),
            offset_y: value.field("nOffSetY").as_int(),
            line_break: value.field("bLineBreak").as_bool(),
            texture_width: width,
            texture_height: height,
            texture_u_width: if u_width < 1 { width } else { u_width },
            texture_u_height: if u_height < 1 { height } else { u_height },
            texture: value.field("u_strTexture").as_str(),
            text: value.field("t_strText").as_str(),
        },
        stop_mouse_focus: value.field("bStopMouseFocus").as_bool(),
        height: value.field("b_nHeight").as_int(),
        text_id: value.field("t_nTextID").as_int(),
        color: struct_color(&value.field("t_color")),
        draw_one_line: value.field("t_bDrawOneLine").as_bool(),
        texture_mouse_on: value.field("u_strTextureMouseOn").as_str(),
        texture_expanded: value.field("u_strTextureExpanded").as_str(),
    }
}

fn arg_str(args: &[Value], index: usize) -> String {
    args.get(index).map_or_else(String::new, Value::as_str)
}

fn arg_int(args: &[Value], index: usize) -> i32 {
    args.get(index).map_or(0, Value::as_int)
}

fn arg_bool(args: &[Value], index: usize) -> bool {
    args.get(index).is_some_and(Value::as_bool)
}

fn arg_float(args: &[Value], index: usize) -> f32 {
    args.get(index).map_or(0.0, Value::as_float)
}

fn set_out(args: &mut [Value], index: usize, value: Value) {
    if let Some(slot) = args.get_mut(index) {
        *slot = value;
    }
}

impl ScriptHost {
    fn item_window_mut(&mut self, name: &str) -> &mut ItemWindowState {
        self.controls.item_windows.entry(name.to_string()).or_default()
    }

    fn list_ctrl_mut(&mut self, name: &str) -> &mut ListCtrlState {
        self.controls.list_ctrls.entry(name.to_string()).or_default()
    }

    fn combo_mut(&mut self, name: &str) -> &mut ComboState {
        self.controls.combos.entry(name.to_string()).or_default()
    }

    fn edit_mut(&mut self, name: &str) -> &mut EditState {
        self.controls.edits.entry(name.to_string()).or_default()
    }

    fn tree_mut(&mut self, name: &str) -> &mut TreeState {
        self.controls.trees.entry(name.to_string()).or_default()
    }

    fn html_mut(&mut self, name: &str) -> &mut HtmlState {
        self.controls.html.entry(name.to_string()).or_default()
    }

    fn style_mut(&mut self, name: &str) -> &mut WindowStyle {
        self.controls.window_styles.entry(name.to_string()).or_default()
    }

    pub fn control_item_add(&mut self, name: &str, info: ItemInfo) {
        self.item_window_mut(name).add(info);
    }

    pub fn control_item_set(&mut self, name: &str, index: i32, info: ItemInfo) -> bool {
        self.item_window_mut(name).set(index, info)
    }

    pub fn control_item_delete(&mut self, name: &str, index: i32) -> bool {
        self.item_window_mut(name).delete(index)
    }

    pub fn control_item_clear(&mut self, name: &str) {
        self.item_window_mut(name).clear();
    }

    pub fn control_item_clear_select(&mut self, name: &str) {
        self.item_window_mut(name).clear_select();
    }

    pub fn control_item_swap(&mut self, name: &str, first: i32, second: i32) {
        self.item_window_mut(name).swap(first, second);
    }

    pub fn control_item_get(&self, name: &str, index: i32) -> Option<ItemInfo> {
        self.controls.item_window(name).and_then(|window| window.get(index)).cloned()
    }

    pub fn control_item_get_selected(&self, name: &str) -> Option<ItemInfo> {
        self.controls.item_window(name).and_then(ItemWindowState::get_selected).cloned()
    }

    pub fn control_item_num(&self, name: &str) -> i32 {
        self.controls.item_window(name).map_or(0, ItemWindowState::item_num)
    }

    pub fn control_item_selected_num(&self, name: &str) -> i32 {
        self.controls.item_window(name).map_or(-1, ItemWindowState::selected_num)
    }

    pub fn control_item_find_server_id(&self, name: &str, server_id: i32) -> i32 {
        self.controls.item_window(name).map_or(-1, |window| window.find_with_server_id(server_id))
    }

    pub fn control_item_find_class_id(&self, name: &str, class_id: i32) -> i32 {
        self.controls.item_window(name).map_or(-1, |window| window.find_with_class_id(class_id))
    }

    pub fn control_combo_add(&mut self, name: &str, entry: ComboEntry) {
        self.combo_mut(name).entries.push(entry);
    }

    pub fn control_combo_clear(&mut self, name: &str) {
        self.combo_mut(name).clear();
    }

    pub fn control_combo_select(&mut self, name: &str, index: i32) {
        self.combo_mut(name).selected = usize::try_from(index).ok();
    }

    pub fn control_combo_selected_num(&self, name: &str) -> i32 {
        self.controls.combo(name).map_or(-1, ComboState::selected_num)
    }

    pub fn control_combo_string(&self, name: &str, index: i32) -> String {
        let Some(entry) = usize::try_from(index).ok().and_then(|index| self.controls.combo(name)?.entries.get(index))
        else {
            return String::new();
        };
        combo_text(entry, &|id| self.sysstrings.get(&(id as u32)).cloned())
    }

    pub fn control_combo_reserved(&self, name: &str, index: i32) -> i32 {
        self.controls.combo(name).map_or(-1, |combo| combo.reserved(index))
    }

    pub fn control_edit_string(&self, name: &str) -> String {
        self.controls.edit(name).map(|edit| edit.text.clone()).unwrap_or_default()
    }

    pub fn control_edit_add(&mut self, name: &str, text: &str) {
        self.edit_mut(name).text.push_str(text);
    }

    pub fn control_edit_clear(&mut self, name: &str) {
        self.edit_mut(name).text.clear();
    }

    pub fn control_edit_set(&mut self, name: &str, text: &str) {
        self.control_edit_clear(name);
        self.control_edit_add(name, text);
    }

    pub fn control_edit_backspace(&mut self, name: &str) {
        self.edit_mut(name).text.pop();
    }

    pub fn control_button_name(&mut self, name: &str, id: i32) {
        let text = self.sysstrings.get(&(id as u32)).cloned().unwrap_or_default();
        self.controls.button_names.insert(name.to_string(), (id, text));
    }

    pub fn control_set_window_title(&mut self, name: &str, title: &str) {
        self.controls.window_titles.insert(name.to_string(), title.to_string());
    }

    pub fn control_set_style(&mut self, name: &str, style: u32, on: bool) {
        self.style_mut(name).set(style, on);
    }

    pub fn control_style(&self, name: &str, style: u32) -> Option<bool> {
        self.controls.window_styles.get(name).and_then(|styles| styles.get(style))
    }

    pub fn control_text_align(&mut self, name: &str, align: i32) {
        self.controls.text_aligns.insert(name.to_string(), align);
    }

    pub fn control_status_icon_size(&mut self, name: &str, size: i32) {
        self.controls.status_icon_sizes.insert(name.to_string(), size);
        if let Some(grid) = self.status_grids.get_mut(name) {
            for cell in grid.iter_mut().flatten() {
                cell.size = size;
            }
        }
    }

    fn window_parent_name(&self, name: &str) -> String {
        name.rsplit_once('.').map(|(parent, _)| parent.to_string()).unwrap_or_default()
    }

    pub fn control_native(&mut self, class: &str, function: &str, args: &mut [Value]) -> Option<Value> {
        let name = arg_str(args, 0);
        let value = match (class, function) {
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "AddItem") => {
                let info = item_info(&args.get(1).cloned().unwrap_or_default());
                self.control_item_add(&name, info);
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "SetItem") => {
                let info = item_info(&args.get(2).cloned().unwrap_or_default());
                Value::Bool(self.control_item_set(&name, arg_int(args, 1), info))
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "DeleteItem") => {
                self.control_item_delete(&name, arg_int(args, 1));
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "Clear") => {
                self.control_item_clear(&name);
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "ClearSelect") => {
                self.control_item_clear_select(&name);
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "SwapItems") => {
                self.control_item_swap(&name, arg_int(args, 1), arg_int(args, 2));
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "GetItem") => {
                match self.control_item_get(&name, arg_int(args, 1)) {
                    Some(info) => {
                        set_out(args, 2, item_info_value(&info));
                        Value::Bool(true)
                    }
                    None => Value::Bool(false),
                }
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "GetSelectedItem") => match self.control_item_get_selected(&name)
            {
                Some(info) => {
                    set_out(args, 1, item_info_value(&info));
                    Value::Bool(true)
                }
                None => Value::Bool(false),
            },
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "GetSelectedItemID") => {
                Value::Int(self.controls.item_window(&name).map_or(-1, ItemWindowState::selected_item_id))
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "GetSelectedNum") => {
                Value::Int(self.control_item_selected_num(&name))
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "GetItemNum") => Value::Int(self.control_item_num(&name)),
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "FindItemWithServerID") => {
                Value::Int(self.control_item_find_server_id(&name, arg_int(args, 1)))
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "FindItemWithClassID") => {
                Value::Int(self.control_item_find_class_id(&name, arg_int(args, 1)))
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "GetIndexAt") => Value::Int(-1),
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "SetCol") => {
                self.item_window_mut(&name).cols = Some(arg_int(args, 1));
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "SetRow") => {
                self.item_window_mut(&name).rows = Some(arg_int(args, 1));
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "SetFaded") => {
                self.item_window_mut(&name).faded = arg_bool(args, 1);
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "SetDisableTex") => {
                self.item_window_mut(&name).disable_tex = arg_str(args, 1);
                Value::None
            }
            ("UIAPI_ITEMWINDOW" | "ItemWindowHandle", "ShowScrollBar") => {
                let show = arg_bool(args, 1);
                self.item_window_mut(&name).scroll_bar = Some(show);
                self.controls.scroll_bars.insert(name.clone(), show);
                Value::None
            }
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "InsertRecord") => {
                let record = list_record(&args.get(1).cloned().unwrap_or_default());
                let max_rows = self.controls.list_limits.get(&name).map(|limits| limits.0);
                self.list_ctrl_mut(&name).insert(record, max_rows);
                Value::None
            }
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "ModifyRecord") => {
                let record = list_record(&args.get(2).cloned().unwrap_or_default());
                Value::Bool(self.list_ctrl_mut(&name).modify(arg_int(args, 1), &record))
            }
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "DeleteRecord") => {
                self.list_ctrl_mut(&name).delete(arg_int(args, 1));
                Value::None
            }
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "DeleteAllItem") => {
                self.list_ctrl_mut(&name).delete_all();
                Value::None
            }
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "GetRecord") => self
                .controls
                .list_ctrl(&name)
                .and_then(|list| list.get(arg_int(args, 1)))
                .map_or_else(|| list_record_value(&ListRecord::default()), list_record_value),
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "GetSelectedRecord") => self
                .controls
                .list_ctrl(&name)
                .and_then(ListCtrlState::get_selected)
                .map_or_else(|| list_record_value(&ListRecord::default()), list_record_value),
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "GetRecordCount") => {
                Value::Int(self.controls.list_ctrl(&name).map_or(0, ListCtrlState::count))
            }
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "GetSelectedIndex") => {
                Value::Int(self.controls.list_ctrl(&name).map_or(-1, ListCtrlState::selected_index))
            }
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "SetSelectedIndex") => {
                let show_rows = self.controls.list_limits.get(&name).map(|limits| limits.1);
                self.list_ctrl_mut(&name).set_selected(arg_int(args, 1), arg_bool(args, 2), show_rows);
                Value::None
            }
            ("UIAPI_LISTCTRL" | "ListCtrlHandle", "ShowScrollBar") => {
                let show = arg_bool(args, 1);
                self.list_ctrl_mut(&name).scroll_bar = Some(show);
                self.controls.scroll_bars.insert(name.clone(), show);
                Value::None
            }
            ("UIAPI_TREECTRL", "InsertNode") => {
                let info = tree_node_info(&args.get(2).cloned().unwrap_or_default());
                let parent = arg_str(args, 1);
                Value::Str(self.tree_mut(&name).insert_node(&parent, info))
            }
            ("UIAPI_TREECTRL", "InsertNodeItem") => {
                let item = tree_item_state(&args.get(2).cloned().unwrap_or_default());
                let node = arg_str(args, 1);
                self.tree_mut(&name).insert_node_item(&node, item);
                Value::None
            }
            ("UIAPI_TREECTRL", "DeleteNode") => {
                let node = arg_str(args, 1);
                Value::Bool(self.tree_mut(&name).delete_node(&node))
            }
            ("UIAPI_TREECTRL", "Clear") => {
                self.tree_mut(&name).nodes.clear();
                Value::None
            }
            ("UIAPI_TREECTRL", "SetExpandedNode") => {
                let node = arg_str(args, 1);
                self.tree_mut(&name).set_expanded(&node, arg_bool(args, 2));
                Value::None
            }
            ("UIAPI_TREECTRL", "IsExpandedNode") => {
                Value::Bool(self.controls.tree(&name).is_some_and(|tree| tree.is_expanded(&arg_str(args, 1))))
            }
            ("UIAPI_TREECTRL", "GetExpandedNode") => Value::Str(
                self.controls.tree(&name).map(|tree| tree.expanded_nodes(&arg_str(args, 1))).unwrap_or_default(),
            ),
            ("UIAPI_TREECTRL", "GetChildNode") => Value::Str(
                self.controls.tree(&name).map(|tree| tree.child_nodes(&arg_str(args, 1))).unwrap_or_default(),
            ),
            ("UIAPI_TREECTRL", "GetParentNode") => Value::Str(
                self.controls.tree(&name).map(|tree| tree.parent_node(&arg_str(args, 1))).unwrap_or_default(),
            ),
            ("UIAPI_TREECTRL", "IsNodeNameExist") => {
                Value::Bool(self.controls.tree(&name).is_some_and(|tree| tree.find(&arg_str(args, 1)).is_some()))
            }
            ("UIAPI_TREECTRL", "SetNodeItemText") => {
                let node = arg_str(args, 1);
                let text = arg_str(args, 3);
                self.tree_mut(&name).set_node_item_text(&node, arg_int(args, 2), &text);
                Value::None
            }
            ("UIAPI_TREECTRL", "ShowScrollBar") => {
                let show = arg_bool(args, 1);
                self.tree_mut(&name).scroll_bar = Some(show);
                self.controls.scroll_bars.insert(name.clone(), show);
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "AddString") => {
                let text = arg_str(args, 1);
                if !text.is_empty() {
                    self.control_combo_add(&name, ComboEntry { sysstring: -1, sysmsg: -1, text, reserved: 0 });
                }
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "AddStringWithReserved") => {
                let text = arg_str(args, 1);
                if !text.is_empty() {
                    self.control_combo_add(
                        &name,
                        ComboEntry { sysstring: -1, sysmsg: -1, text, reserved: arg_int(args, 2) },
                    );
                }
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "SYS_AddString") => {
                let id = arg_int(args, 1);
                if id >= 0 {
                    self.control_combo_add(
                        &name,
                        ComboEntry { sysstring: id, sysmsg: -1, text: String::new(), reserved: 0 },
                    );
                }
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "SYS_AddStringWithReserved") => {
                let id = arg_int(args, 1);
                if id >= 0 {
                    self.control_combo_add(
                        &name,
                        ComboEntry { sysstring: id, sysmsg: -1, text: String::new(), reserved: arg_int(args, 2) },
                    );
                }
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "Clear") => {
                self.control_combo_clear(&name);
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "SetSelectedNum") => {
                self.control_combo_select(&name, arg_int(args, 1));
                Value::None
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "GetSelectedNum") => {
                Value::Int(self.control_combo_selected_num(&name))
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "GetNumOfItems") => {
                Value::Int(self.controls.combo(&name).map_or(0, ComboState::count))
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "GetString") => {
                Value::Str(self.control_combo_string(&name, arg_int(args, 1)))
            }
            ("UIAPI_COMBOBOX" | "ComboBoxHandle", "GetReserved") => {
                Value::Int(self.control_combo_reserved(&name, arg_int(args, 1)))
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle" | "UIAPI_MULTIEDITBOX" | "MultiEditHandle", "GetString") => {
                Value::Str(self.control_edit_string(&name))
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle" | "UIAPI_MULTIEDITBOX" | "MultiEditHandle", "SetString") => {
                let text = arg_str(args, 1);
                self.control_edit_set(&name, &text);
                Value::None
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "AddString") => {
                let text = arg_str(args, 1);
                self.control_edit_add(&name, &text);
                Value::None
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "Clear") => {
                self.control_edit_clear(&name);
                Value::None
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "SimulateBackspace") => {
                self.control_edit_backspace(&name);
                Value::None
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "SetMaxLength") => {
                let length = arg_int(args, 1);
                if length >= 0 {
                    self.edit_mut(&name).max_length = length;
                }
                Value::None
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "GetMaxLength") => {
                Value::Int(self.controls.edit(&name).map_or(0, |edit| edit.max_length))
            }
            ("UIAPI_EDITBOX" | "EditBoxHandle", "SetHighLight") => {
                self.edit_mut(&name).highlight = arg_bool(args, 1);
                Value::None
            }
            ("UIAPI_BUTTON" | "ButtonHandle", "SetButtonName") => {
                self.control_button_name(&name, arg_int(args, 1));
                Value::None
            }
            ("UIAPI_BUTTON" | "ButtonHandle", "GetButtonName") => {
                Value::Str(self.controls.button_names.get(&name).map(|(_, text)| text.clone()).unwrap_or_default())
            }
            ("UIAPI_BUTTON" | "ButtonHandle", "SetTexture") => {
                let textures =
                    ButtonTextures { fore: arg_str(args, 1), back: arg_str(args, 2), highlight: arg_str(args, 3) };
                self.controls.button_textures.insert(name, textures);
                Value::None
            }
            ("UIAPI_TEXTURECTRL" | "TextureHandle", "SetUV") => {
                self.controls.texture_uv.insert(name, (arg_int(args, 1), arg_int(args, 2)));
                Value::None
            }
            ("UIAPI_TEXTURECTRL" | "TextureHandle", "SetTextureWithClanCrest") => {
                self.controls.clan_crests.insert(name, arg_int(args, 1));
                Value::None
            }
            ("UIAPI_TEXTURECTRL" | "TextureHandle", "SetTextureWithObject") => {
                self.controls.object_textures.push(name);
                Value::None
            }
            ("UIAPI_ANIMTEXTURECTRL" | "AnimTextureHandle", "Play") => {
                let anim = self.controls.anims.entry(name).or_default();
                anim.playing = true;
                anim.paused = false;
                Value::None
            }
            ("UIAPI_ANIMTEXTURECTRL" | "AnimTextureHandle", "Pause") => {
                self.controls.anims.entry(name).or_default().paused = true;
                Value::None
            }
            ("UIAPI_ANIMTEXTURECTRL" | "AnimTextureHandle", "Stop") => {
                let anim = self.controls.anims.entry(name).or_default();
                anim.playing = false;
                anim.paused = false;
                Value::None
            }
            ("UIAPI_ANIMTEXTURECTRL" | "AnimTextureHandle", "SetLoopCount") => {
                self.controls.anims.entry(name).or_default().loop_count = Some(arg_int(args, 1));
                Value::None
            }
            ("UIAPI_HTMLCTRL" | "HtmlHandle", "LoadHtml") => {
                let file = arg_str(args, 1);
                let html = self.html_mut(&name);
                html.file = file;
                html.source.clear();
                Value::None
            }
            ("UIAPI_HTMLCTRL" | "HtmlHandle", "LoadHtmlFromString") => {
                let source = arg_str(args, 1);
                let html = self.html_mut(&name);
                html.file.clear();
                html.source = source;
                Value::None
            }
            ("UIAPI_HTMLCTRL" | "HtmlHandle", "SetHtmlBuffData") => {
                self.html_mut(&name).buffer = arg_str(args, 1);
                Value::None
            }
            ("UIAPI_HTMLCTRL" | "HtmlHandle", "SetPageLock") => {
                self.html_mut(&name).page_lock = arg_bool(args, 1);
                Value::None
            }
            ("UIAPI_HTMLCTRL" | "HtmlHandle", "IsPageLock") => {
                Value::Bool(self.controls.html.get(&name).is_some_and(|html| html.page_lock))
            }
            ("UIAPI_HTMLCTRL" | "HtmlHandle", "Clear") => {
                let html = self.html_mut(&name);
                html.file.clear();
                html.source.clear();
                Value::None
            }
            ("UIAPI_HTMLCTRL" | "HtmlHandle", "ControllerExecution") => {
                self.html_mut(&name).bypasses.push(arg_str(args, 1));
                Value::Bool(false)
            }
            ("UIAPI_HTMLCTRL" | "HtmlHandle", "GetFrameMaxHeight") => Value::Int(0),
            ("UIAPI_PROGRESSCTRL" | "ProgressCtrlHandle", "SetProgressTime") => {
                self.controls.progress.entry(name).or_default().time = arg_int(args, 1);
                Value::None
            }
            ("UIAPI_PROGRESSCTRL" | "ProgressCtrlHandle", "SetPos") => {
                self.controls.progress.entry(name).or_default().pos = arg_int(args, 1);
                Value::None
            }
            ("UIAPI_PROGRESSCTRL" | "ProgressCtrlHandle", "Start") => {
                let progress = self.controls.progress.entry(name).or_default();
                progress.running = true;
                progress.starts += 1;
                Value::None
            }
            ("UIAPI_PROGRESSCTRL" | "ProgressCtrlHandle", "Resume") => {
                self.controls.progress.entry(name).or_default().running = true;
                Value::None
            }
            ("UIAPI_PROGRESSCTRL" | "ProgressCtrlHandle", "Stop") => {
                self.controls.progress.entry(name).or_default().running = false;
                Value::None
            }
            ("UIAPI_PROGRESSCTRL" | "ProgressCtrlHandle", "Reset") => {
                let progress = self.controls.progress.entry(name).or_default();
                progress.running = false;
                progress.pos = 0;
                Value::None
            }
            ("UIAPI_SLIDERCTRL" | "SliderHandle", "SetCurrentTick") => {
                self.controls.sliders.insert(name, arg_int(args, 1));
                Value::None
            }
            ("UIAPI_SLIDERCTRL" | "SliderHandle", "GetCurrentTick") => {
                Value::Int(self.controls.sliders.get(&name).copied().unwrap_or(0))
            }
            ("UIAPI_INVENWEIGHT" | "InvenWeightHandle", "AddWeight") => {
                *self.controls.weights.entry(name).or_default() += arg_int(args, 1);
                Value::None
            }
            ("UIAPI_INVENWEIGHT" | "InvenWeightHandle", "ReduceWeight") => {
                *self.controls.weights.entry(name).or_default() -= arg_int(args, 1);
                Value::None
            }
            ("UIAPI_INVENWEIGHT" | "InvenWeightHandle", "ZeroWeight") => {
                self.controls.weights.insert(name, 0);
                Value::None
            }
            ("UIAPI_DRAWPANEL" | "DrawPanelHandle", "InsertDrawItem") => {
                let item = draw_item_info(&args.get(1).cloned().unwrap_or_default());
                self.controls.draw_panels.entry(name).or_default().push(item);
                Value::None
            }
            ("UIAPI_DRAWPANEL" | "DrawPanelHandle", "Clear") => {
                self.controls.draw_panels.entry(name).or_default().clear();
                Value::None
            }
            ("UIAPI_DRAWPANEL" | "DrawPanelHandle", "PreCheckPanelSize") => {
                set_out(args, 1, Value::Int(0));
                set_out(args, 2, Value::Int(0));
                Value::None
            }
            ("UIAPI_SCROLLWND" | "ScrollWndHandle" | "WindowHandle" | "UIAPI_WINDOW", "SetScrollPosition") => {
                self.controls.scroll_positions.insert(name, arg_int(args, 1));
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "GetWindowName") => Value::Str(name),
            ("UIAPI_WINDOW" | "WindowHandle", "GetParentWindowName") => Value::Str(self.window_parent_name(&name)),
            ("UIAPI_WINDOW" | "WindowHandle", "IsChildOf") => {
                let other = arg_str(args, 1);
                Value::Bool(!other.is_empty() && name.starts_with(&format!("{other}.")))
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetDraggable") => {
                self.control_set_style(&name, STYLE_DRAGGABLE, arg_bool(args, 1));
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "IsDraggable") => {
                Value::Bool(self.control_style(&name, STYLE_DRAGGABLE).unwrap_or(false))
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetAlwaysOnTop") => {
                self.control_set_style(&name, STYLE_ALWAYS_ON_TOP, arg_bool(args, 1));
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetSettledWnd") => {
                self.control_set_style(&name, STYLE_SETTLED, arg_bool(args, 1));
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "GetAlpha") => {
                Value::Int(self.overlay.alphas.get(&name).copied().unwrap_or(255))
            }
            ("UIAPI_WINDOW" | "WindowHandle", "IsAnchored") => Value::Bool(self.overlay.anchors.contains_key(&name)),
            ("UIAPI_WINDOW" | "WindowHandle", "IsMinimizedWindow") => Value::Bool(false),
            ("UIAPI_WINDOW" | "WindowHandle", "NotifyAlarm") => {
                self.controls.alarms.push(name);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "Iconize") => {
                self.controls.icons.insert(name, (arg_str(args, 1), arg_int(args, 2)));
                Value::None
            }
            ("UIAPI_WINDOW", "SetWindowTitle") => {
                let title = self.sysstrings.get(&(arg_int(args, 1) as u32)).cloned().unwrap_or_default();
                self.control_set_window_title(&name, &title);
                Value::None
            }
            ("WindowHandle", "SetWindowTitle") | ("UIAPI_WINDOW" | "WindowHandle", "SetWindowTitleByText") => {
                let title = arg_str(args, 1);
                self.control_set_window_title(&name, &title);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetResizeFrameSize") => {
                self.controls.resize_frames.insert(name, (arg_int(args, 1), arg_int(args, 2)));
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetWindowSizeRel") => {
                self.controls
                    .size_rels
                    .insert(name, (arg_float(args, 1), arg_float(args, 2), arg_int(args, 3), arg_int(args, 4)));
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "SetTooltipCustomType") => {
                let tooltip = custom_tooltip(&args.get(1).cloned().unwrap_or_default());
                self.controls.custom_tooltips.insert(name, tooltip);
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "GetTooltipCustomType") => {
                let tooltip = self.controls.custom_tooltips.get(&name).cloned().unwrap_or_default();
                set_out(args, 1, custom_tooltip_value(&tooltip));
                Value::None
            }
            ("UIAPI_WINDOW" | "WindowHandle", "GetSelectedRadioButtonName") => Value::Str(String::new()),
            ("UIAPI_TABCTRL" | "TabHandle", "MergeTab") => {
                self.controls.tab_merges.push((name, arg_int(args, 1)));
                Value::None
            }
            ("UIAPI_TEXTBOX" | "TextBoxHandle", "SetAlign") => {
                self.control_text_align(&name, arg_int(args, 1));
                Value::None
            }
            ("UIAPI_TEXTBOX" | "TextBoxHandle", "SetTextColor") => {
                let rgb = color(&args.get(1).cloned().unwrap_or_default());
                self.overlay.text_colors.insert(name, Argb(0xFF00_0000 | (rgb.0 & 0x00FF_FFFF)));
                Value::None
            }
            ("UIAPI_TEXTBOX" | "TextBoxHandle", "GetTextColor") => {
                color_value(self.overlay.text_colors.get(&name).copied().unwrap_or(Argb(0xFFFF_FFFF)))
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "SetIconSize") => {
                self.control_status_icon_size(&name, arg_int(args, 1));
                Value::None
            }
            ("UIAPI_STATUSICONCTRL" | "StatusIconHandle", "SetItem") => {
                let info = super::vm_host::status_icon_info(&args.get(3).cloned().unwrap_or_default());
                if let Some(cell) = self
                    .status_grids
                    .get_mut(&name)
                    .and_then(|grid| grid.get_mut(usize::try_from(arg_int(args, 1)).unwrap_or(usize::MAX)))
                    .and_then(|row| row.get_mut(usize::try_from(arg_int(args, 2)).unwrap_or(usize::MAX)))
                {
                    *cell = info;
                }
                Value::None
            }
            ("UIAPI_EFFECTBUTTON" | "EffectButtonHandle", "BeginEffect") => {
                self.controls.effect_buttons.insert(name, arg_int(args, 1));
                Value::None
            }
            ("UIAPI_TEXTLISTBOX" | "TextListBoxHandle" | "UIAPI_LISTBOX" | "ListBoxHandle", "Clear") => {
                self.chat_lines.remove(&name);
                Value::None
            }
            _ => return None,
        };
        Some(value)
    }
}

pub fn draw_item_value(item: &DrawItemInfo) -> Value {
    let i = |name: &str, value: i32| (name.to_string(), Value::Int(value));
    Value::Struct(Box::new(StructValue {
        name: "DrawItemInfo".into(),
        fields: vec![
            (
                "eType".into(),
                Value::Byte(match item.kind {
                    DrawItemType::Blank => TREE_ITEM_BLANK as u8,
                    DrawItemType::Text => TREE_ITEM_TEXT as u8,
                    DrawItemType::Texture => TREE_ITEM_TEXTURE as u8,
                }),
            ),
            i("nOffSetX", item.offset_x),
            i("nOffSetY", item.offset_y),
            ("bLineBreak".into(), Value::Bool(item.line_break)),
            i("b_nHeight", item.height),
            i("t_ID", item.text_id),
            ("t_strText".into(), Value::Str(item.text.clone())),
            ("t_color".into(), color_value(item.color.unwrap_or(Argb(0)))),
            ("t_bDrawOneLine".into(), Value::Bool(item.draw_one_line)),
            i("u_nTextureWidth", item.texture_width),
            i("u_nTextureHeight", item.texture_height),
            i("u_nTextureUWidth", item.texture_u_width),
            i("u_nTextureUHeight", item.texture_u_height),
            ("u_strTexture".into(), Value::Str(item.texture.clone())),
            ("Condition".into(), Value::Str(item.condition.clone())),
        ],
    }))
}

pub fn custom_tooltip_value(tooltip: &CustomTooltip) -> Value {
    Value::Struct(Box::new(StructValue {
        name: "CustomTooltip".into(),
        fields: vec![
            ("MinimumWidth".into(), Value::Int(tooltip.minimum_width)),
            ("SimpleLineCount".into(), Value::Int(tooltip.simple_line_count)),
            ("DrawList".into(), Value::Array(tooltip.draw_list.iter().map(draw_item_value).collect())),
        ],
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn host() -> ScriptHost {
        let mut host = ScriptHost::default();
        host.sysstrings.insert(7, "Sete".into());
        host.sysstrings.insert(9, "Nove".into());
        host
    }

    fn call(host: &mut ScriptHost, class: &str, function: &str, args: &mut [Value]) -> Value {
        host.control_native(class, function, args).unwrap()
    }

    fn item(server_id: i32, class_id: i32, num: i32) -> Value {
        Value::Struct(Box::new(StructValue {
            name: "ItemInfo".into(),
            fields: vec![
                ("IconName".into(), Value::Str(format!("icon.{class_id}"))),
                ("ServerID".into(), Value::Int(server_id)),
                ("ClassID".into(), Value::Int(class_id)),
                ("ItemNum".into(), Value::Int(num)),
            ],
        }))
    }

    #[test]
    fn item_window_follows_ncitemwnd() {
        let mut host = host();
        let name = || Value::Str("InventoryWnd.InventoryItemWnd".into());
        call(&mut host, "UIAPI_ITEMWINDOW", "AddItem", &mut [name(), item(10, 57, 1)]);
        call(&mut host, "ItemWindowHandle", "AddItem", &mut [name(), item(11, 1835, 300)]);
        assert_eq!(call(&mut host, "ItemWindowHandle", "GetItemNum", &mut [name()]), Value::Int(2));
        assert_eq!(call(&mut host, "ItemWindowHandle", "GetSelectedNum", &mut [name()]), Value::Int(-1));
        assert_eq!(call(&mut host, "ItemWindowHandle", "GetSelectedItemID", &mut [name()]), Value::Int(-1));
        assert_eq!(
            call(&mut host, "ItemWindowHandle", "FindItemWithServerID", &mut [name(), Value::Int(11)]),
            Value::Int(1)
        );
        assert_eq!(
            call(&mut host, "ItemWindowHandle", "FindItemWithClassID", &mut [name(), Value::Int(57)]),
            Value::Int(0)
        );
        assert_eq!(
            call(&mut host, "ItemWindowHandle", "FindItemWithClassID", &mut [name(), Value::Int(1)]),
            Value::Int(-1)
        );
        let mut args = [name(), Value::Int(1), Value::None];
        assert_eq!(call(&mut host, "ItemWindowHandle", "GetItem", &mut args), Value::Bool(true));
        assert_eq!(args[2].field("ClassID"), Value::Int(1835));
        assert_eq!(args[2].field("ItemNum"), Value::Int(300));
        assert_eq!(
            call(&mut host, "ItemWindowHandle", "GetItem", &mut [name(), Value::Int(2), Value::None]),
            Value::Bool(false)
        );
        assert_eq!(
            call(&mut host, "ItemWindowHandle", "SetItem", &mut [name(), Value::Int(5), item(1, 1, 1)]),
            Value::Bool(false)
        );
        assert_eq!(
            call(&mut host, "ItemWindowHandle", "SetItem", &mut [name(), Value::Int(0), item(12, 2, 4)]),
            Value::Bool(true)
        );
        call(&mut host, "ItemWindowHandle", "SwapItems", &mut [name(), Value::Int(0), Value::Int(1)]);
        let cells = host.controls.item_window("InventoryWnd.InventoryItemWnd").unwrap().cells();
        assert_eq!(cells.len(), 2);
        assert_eq!(cells[0].icon, "icon.1835");
        assert_eq!(cells[0].count, Some(300));
        assert_eq!(cells[1].icon, "icon.2");
        call(&mut host, "ItemWindowHandle", "DeleteItem", &mut [name(), Value::Int(0)]);
        assert_eq!(call(&mut host, "ItemWindowHandle", "GetItemNum", &mut [name()]), Value::Int(1));
        call(&mut host, "ItemWindowHandle", "SetFaded", &mut [name(), Value::Bool(true)]);
        call(&mut host, "ItemWindowHandle", "SetCol", &mut [name(), Value::Int(8)]);
        assert!(host.controls.item_window("InventoryWnd.InventoryItemWnd").unwrap().faded);
        call(&mut host, "ItemWindowHandle", "Clear", &mut [name()]);
        let window = host.controls.item_window("InventoryWnd.InventoryItemWnd").unwrap();
        assert!(window.items.is_empty() && !window.faded && window.cols == Some(8));
    }

    fn record(texts: &[&str], reserved: i32) -> Value {
        Value::Struct(Box::new(StructValue {
            name: "LVDataRecord".into(),
            fields: vec![
                (
                    "LVDataList".into(),
                    Value::Array(
                        texts
                            .iter()
                            .map(|text| {
                                Value::Struct(Box::new(StructValue {
                                    name: "LVData".into(),
                                    fields: vec![
                                        ("szData".into(), Value::Str((*text).to_string())),
                                        ("bUseTextColor".into(), Value::Bool(true)),
                                        (
                                            "TextColor".into(),
                                            Value::Struct(Box::new(StructValue {
                                                name: "Color".into(),
                                                fields: vec![
                                                    ("R".into(), Value::Byte(255)),
                                                    ("G".into(), Value::Byte(0)),
                                                    ("B".into(), Value::Byte(0)),
                                                    ("A".into(), Value::Byte(255)),
                                                ],
                                            })),
                                        ),
                                    ],
                                }))
                            })
                            .collect(),
                    ),
                ),
                ("szReserved".into(), Value::Str("r".into())),
                ("nReserved1".into(), Value::Int(reserved)),
            ],
        }))
    }

    #[test]
    fn list_ctrl_follows_nclistctrl() {
        let mut host = host();
        host.controls.list_limits.insert("PartyMatchWnd.ListCtrl".into(), (2, 1));
        let name = || Value::Str("PartyMatchWnd.ListCtrl".into());
        call(&mut host, "UIAPI_LISTCTRL", "InsertRecord", &mut [name(), record(&["a", "b"], 1)]);
        call(&mut host, "ListCtrlHandle", "InsertRecord", &mut [name(), record(&["c", "d"], 2)]);
        call(&mut host, "ListCtrlHandle", "InsertRecord", &mut [name(), record(&["e", "f"], 3)]);
        assert_eq!(call(&mut host, "ListCtrlHandle", "GetRecordCount", &mut [name()]), Value::Int(2));
        assert_eq!(call(&mut host, "ListCtrlHandle", "GetSelectedIndex", &mut [name()]), Value::Int(-1));
        call(&mut host, "ListCtrlHandle", "SetSelectedIndex", &mut [name(), Value::Int(1), Value::Bool(true)]);
        assert_eq!(call(&mut host, "ListCtrlHandle", "GetSelectedIndex", &mut [name()]), Value::Int(1));
        assert_eq!(host.controls.list_ctrl("PartyMatchWnd.ListCtrl").unwrap().top, 1);
        let selected = call(&mut host, "ListCtrlHandle", "GetSelectedRecord", &mut [name()]);
        assert_eq!(selected.field("nReserved1"), Value::Int(2));
        assert_eq!(
            call(&mut host, "ListCtrlHandle", "ModifyRecord", &mut [name(), Value::Int(0), record(&["x", "y"], 9)]),
            Value::Bool(true)
        );
        assert_eq!(
            call(&mut host, "ListCtrlHandle", "ModifyRecord", &mut [name(), Value::Int(7), record(&["x"], 9)]),
            Value::Bool(false)
        );
        let first = call(&mut host, "ListCtrlHandle", "GetRecord", &mut [name(), Value::Int(0)]);
        let Value::Array(cells) = first.field("LVDataList") else { panic!() };
        assert_eq!(cells[1].field("szData"), Value::Str("y".into()));
        let records = host.controls.list_ctrl("PartyMatchWnd.ListCtrl").unwrap().painter_records();
        assert_eq!(records[0][0].text, "x");
        assert_eq!(records[0][0].color, Argb(0xFFFF_0000));
        call(&mut host, "ListCtrlHandle", "DeleteRecord", &mut [name(), Value::Int(0)]);
        assert_eq!(call(&mut host, "ListCtrlHandle", "GetRecordCount", &mut [name()]), Value::Int(1));
        call(&mut host, "ListCtrlHandle", "DeleteAllItem", &mut [name()]);
        let list = host.controls.list_ctrl("PartyMatchWnd.ListCtrl").unwrap();
        assert!(list.records.is_empty() && list.selected.is_none() && list.top == 0);
    }

    fn node(name: &str) -> Value {
        Value::Struct(Box::new(StructValue {
            name: "XMLTreeNodeInfo".into(),
            fields: vec![("strName".into(), Value::Str(name.into()))],
        }))
    }

    #[test]
    fn tree_follows_ncxmltreectrl() {
        let mut host = host();
        let name = || Value::Str("QuestWnd.Tree".into());
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "InsertNode", &mut [name(), Value::Str(String::new()), node("root")]),
            Value::Str("root".into())
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "InsertNode", &mut [name(), Value::Str("root".into()), node("a")]),
            Value::Str("root.a".into())
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "InsertNode", &mut [name(), Value::Str("root".into()), node("a")]),
            Value::Str(String::new())
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "InsertNode", &mut [name(), Value::Str("root".into()), node("b")]),
            Value::Str("root.b".into())
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "InsertNode", &mut [name(), Value::Str("root".into()), node("")]),
            Value::Str(String::new())
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "IsExpandedNode", &mut [name(), Value::Str("root".into())]),
            Value::Bool(true)
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "IsExpandedNode", &mut [name(), Value::Str("root.a".into())]),
            Value::Bool(false)
        );
        call(
            &mut host,
            "UIAPI_TREECTRL",
            "SetExpandedNode",
            &mut [name(), Value::Str("root.b".into()), Value::Bool(true)],
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "GetChildNode", &mut [name(), Value::Str("root".into())]),
            Value::Str("root.a|root.b".into())
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "GetExpandedNode", &mut [name(), Value::Str("root".into())]),
            Value::Str("root.b".into())
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "GetParentNode", &mut [name(), Value::Str("root.a".into())]),
            Value::Str("root".into())
        );
        let item = Value::Struct(Box::new(StructValue {
            name: "XMLTreeNodeItemInfo".into(),
            fields: vec![
                ("eType".into(), Value::Byte(1)),
                ("t_nTextID".into(), Value::Int(3)),
                ("t_strText".into(), Value::Str("old".into())),
            ],
        }));
        call(&mut host, "UIAPI_TREECTRL", "InsertNodeItem", &mut [name(), Value::Str("root.a".into()), item]);
        call(
            &mut host,
            "UIAPI_TREECTRL",
            "SetNodeItemText",
            &mut [name(), Value::Str("root.a".into()), Value::Int(3), Value::Str("new".into())],
        );
        assert_eq!(host.controls.tree("QuestWnd.Tree").unwrap().find("root.a").unwrap().items[0].info.text, "new");
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "DeleteNode", &mut [name(), Value::Str("root.a".into())]),
            Value::Bool(true)
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "IsNodeNameExist", &mut [name(), Value::Str("root.a".into())]),
            Value::Bool(false)
        );
        assert_eq!(
            call(&mut host, "UIAPI_TREECTRL", "DeleteNode", &mut [name(), Value::Str("root.a".into())]),
            Value::Bool(false)
        );
        call(&mut host, "UIAPI_TREECTRL", "Clear", &mut [name()]);
        assert!(host.controls.tree("QuestWnd.Tree").unwrap().nodes.is_empty());
    }

    #[test]
    fn combo_and_edit_follow_nccombobox_and_nceditbox() {
        let mut host = host();
        let combo = || Value::Str("OptionWnd.Combo".into());
        call(&mut host, "UIAPI_COMBOBOX", "AddString", &mut [combo(), Value::Str("texto".into())]);
        call(&mut host, "ComboBoxHandle", "AddString", &mut [combo(), Value::Str(String::new())]);
        call(&mut host, "ComboBoxHandle", "SYS_AddString", &mut [combo(), Value::Int(7)]);
        call(&mut host, "ComboBoxHandle", "SYS_AddStringWithReserved", &mut [combo(), Value::Int(9), Value::Int(42)]);
        call(
            &mut host,
            "ComboBoxHandle",
            "AddStringWithReserved",
            &mut [combo(), Value::Str("r".into()), Value::Int(5)],
        );
        assert_eq!(call(&mut host, "UIAPI_COMBOBOX", "GetNumOfItems", &mut [combo()]), Value::Int(4));
        assert_eq!(
            call(&mut host, "ComboBoxHandle", "GetString", &mut [combo(), Value::Int(1)]),
            Value::Str("Sete".into())
        );
        assert_eq!(
            call(&mut host, "ComboBoxHandle", "GetString", &mut [combo(), Value::Int(4)]),
            Value::Str(String::new())
        );
        assert_eq!(call(&mut host, "ComboBoxHandle", "GetReserved", &mut [combo(), Value::Int(2)]), Value::Int(42));
        assert_eq!(call(&mut host, "ComboBoxHandle", "GetReserved", &mut [combo(), Value::Int(8)]), Value::Int(-1));
        assert_eq!(call(&mut host, "ComboBoxHandle", "GetSelectedNum", &mut [combo()]), Value::Int(-1));
        call(&mut host, "ComboBoxHandle", "SetSelectedNum", &mut [combo(), Value::Int(3)]);
        let sysstrings = host.sysstrings.clone();
        let (items, selected) =
            host.controls.combo_items("OptionWnd.Combo", &|id| sysstrings.get(&(id as u32)).cloned()).unwrap();
        assert_eq!(items, vec!["texto", "Sete", "Nove", "r"]);
        assert_eq!(selected, Some(3));
        call(&mut host, "ComboBoxHandle", "Clear", &mut [combo()]);
        assert_eq!(call(&mut host, "ComboBoxHandle", "GetSelectedNum", &mut [combo()]), Value::Int(-1));
        assert!(host.controls.combo_items("OptionWnd.Combo", &|_| None).unwrap().0.is_empty());

        let edit = || Value::Str("ChatWnd.Edit".into());
        assert_eq!(call(&mut host, "EditBoxHandle", "GetMaxLength", &mut [edit()]), Value::Int(0));
        call(&mut host, "EditBoxHandle", "SetMaxLength", &mut [edit(), Value::Int(30)]);
        call(&mut host, "EditBoxHandle", "SetMaxLength", &mut [edit(), Value::Int(-1)]);
        assert_eq!(call(&mut host, "EditBoxHandle", "GetMaxLength", &mut [edit()]), Value::Int(30));
        call(&mut host, "UIAPI_EDITBOX", "SetString", &mut [edit(), Value::Str("ab".into())]);
        call(&mut host, "EditBoxHandle", "AddString", &mut [edit(), Value::Str("cd".into())]);
        call(&mut host, "EditBoxHandle", "SimulateBackspace", &mut [edit()]);
        assert_eq!(call(&mut host, "EditBoxHandle", "GetString", &mut [edit()]), Value::Str("abc".into()));
        call(&mut host, "EditBoxHandle", "Clear", &mut [edit()]);
        assert_eq!(call(&mut host, "UIAPI_MULTIEDITBOX", "GetString", &mut [edit()]), Value::Str(String::new()));
    }

    #[test]
    fn window_button_texture_and_misc_natives() {
        let mut host = host();
        let wnd = || Value::Str("InventoryWnd".into());
        call(&mut host, "UIAPI_WINDOW", "SetWindowTitle", &mut [wnd(), Value::Int(9)]);
        assert_eq!(host.controls.window_titles["InventoryWnd"], "Nove");
        call(&mut host, "WindowHandle", "SetWindowTitle", &mut [wnd(), Value::Str("Título".into())]);
        assert_eq!(host.controls.window_titles["InventoryWnd"], "Título");
        call(&mut host, "WindowHandle", "SetDraggable", &mut [wnd(), Value::Bool(true)]);
        assert_eq!(call(&mut host, "WindowHandle", "IsDraggable", &mut [wnd()]), Value::Bool(true));
        call(&mut host, "WindowHandle", "SetDraggable", &mut [wnd(), Value::Bool(false)]);
        assert_eq!(call(&mut host, "WindowHandle", "IsDraggable", &mut [wnd()]), Value::Bool(false));
        call(&mut host, "WindowHandle", "SetAlwaysOnTop", &mut [wnd(), Value::Bool(true)]);
        assert_eq!(host.control_style("InventoryWnd", STYLE_ALWAYS_ON_TOP), Some(true));
        let child = Value::Str("InventoryWnd.Tab".into());
        assert_eq!(
            call(&mut host, "WindowHandle", "GetParentWindowName", &mut [child.clone()]),
            Value::Str("InventoryWnd".into())
        );
        assert_eq!(call(&mut host, "WindowHandle", "GetParentWindowName", &mut [wnd()]), Value::Str(String::new()));
        assert_eq!(call(&mut host, "WindowHandle", "IsChildOf", &mut [child.clone(), wnd()]), Value::Bool(true));
        assert_eq!(call(&mut host, "WindowHandle", "IsChildOf", &mut [wnd(), child.clone()]), Value::Bool(false));
        assert_eq!(
            call(&mut host, "WindowHandle", "GetWindowName", &mut [child]),
            Value::Str("InventoryWnd.Tab".into())
        );
        assert_eq!(call(&mut host, "WindowHandle", "GetAlpha", &mut [wnd()]), Value::Int(255));

        let button = || Value::Str("InventoryWnd.BtnOk".into());
        call(&mut host, "ButtonHandle", "SetButtonName", &mut [button(), Value::Int(7)]);
        assert_eq!(call(&mut host, "ButtonHandle", "GetButtonName", &mut [button()]), Value::Str("Sete".into()));
        call(
            &mut host,
            "ButtonHandle",
            "SetTexture",
            &mut [button(), Value::Str("f".into()), Value::Str("b".into()), Value::Str("h".into())],
        );
        assert_eq!(host.controls.button_textures["InventoryWnd.BtnOk"].highlight, "h");

        let texture = || Value::Str("InventoryWnd.Tex".into());
        call(&mut host, "TextureHandle", "SetUV", &mut [texture(), Value::Int(3), Value::Int(4)]);
        assert_eq!(host.controls.texture_uv["InventoryWnd.Tex"], (3, 4));

        let weight = || Value::Str("InventoryWnd.Weight".into());
        call(&mut host, "UIAPI_INVENWEIGHT", "AddWeight", &mut [weight(), Value::Int(10)]);
        call(&mut host, "UIAPI_INVENWEIGHT", "ReduceWeight", &mut [weight(), Value::Int(3)]);
        assert_eq!(host.controls.weights["InventoryWnd.Weight"], 7);
        call(&mut host, "UIAPI_INVENWEIGHT", "ZeroWeight", &mut [weight()]);
        assert_eq!(host.controls.weights["InventoryWnd.Weight"], 0);

        let slider = || Value::Str("OptionWnd.Slider".into());
        call(&mut host, "UIAPI_SLIDERCTRL", "SetCurrentTick", &mut [slider(), Value::Int(6)]);
        assert_eq!(call(&mut host, "UIAPI_SLIDERCTRL", "GetCurrentTick", &mut [slider()]), Value::Int(6));

        let html = || Value::Str("NpcWnd.Html".into());
        call(&mut host, "UIAPI_HTMLCTRL", "LoadHtmlFromString", &mut [html(), Value::Str("<html/>".into())]);
        call(&mut host, "HtmlHandle", "SetPageLock", &mut [html(), Value::Bool(true)]);
        assert_eq!(call(&mut host, "HtmlHandle", "IsPageLock", &mut [html()]), Value::Bool(true));
        assert_eq!(host.controls.html["NpcWnd.Html"].source, "<html/>");

        let color = Value::Struct(Box::new(StructValue {
            name: "Color".into(),
            fields: vec![
                ("R".into(), Value::Byte(1)),
                ("G".into(), Value::Byte(2)),
                ("B".into(), Value::Byte(3)),
                ("A".into(), Value::Byte(0)),
            ],
        }));
        let text = || Value::Str("StatusWnd.Txt".into());
        call(&mut host, "TextBoxHandle", "SetTextColor", &mut [text(), color]);
        assert_eq!(host.overlay.text_colors["StatusWnd.Txt"], Argb(0xFF01_0203));
        assert!(host.control_native("UIAPI_WINDOW", "ShowWindow", &mut [wnd()]).is_none());
    }
}
