use crate::draw::DrawList;
use crate::window::Rect;

pub const EQUIP_OUTLINE: &str = "l2ui_ch3.InventoryWnd.Inventory_OutLine";
pub const EQUIP_OUTLINE_DOWN: &str = "l2ui_ch3.InventoryWnd.Inventory_OutLine_down";
pub const EQUIP_CLICK: &str = "l2ui.NWindow.item_click";
pub const EQUIP_DUALCAP: &str = "L2UI.InventoryWnd.Icon_dualcap";
pub const EQUIP_MARK: &str = "L2UI_CH3.InventoryWnd.Inventory_Mark";
pub const ETC_DISABLED: &str = "Default.ChatBack";
pub const SKILL_OUTLINE: &str = "l2ui_ch3.InventoryWnd.inventory_outline";
pub const SKILL_OUTLINE_DOWN: &str = "l2ui_ch3.InventoryWnd.inventory_outline_down";
pub const SKILL_TOGGLE: &str = "L2_SkillTime.ToggleEffect.ToggleEffect001";
pub const COOLDOWN_FRAME: u32 = 0xFF008080;
pub const EQUIP_COLUMNS: [i32; 5] = [7, 0x2e, 0x55, 0x82, 0xa9];
pub const EQUIP_ROWS: [i32; 3] = [7, 0x2d, 0x53];
pub const EQUIP_SLOTS: usize = 15;
pub const EQUIP_OUTLINE_SIZE: i32 = 0x22;
pub const EQUIP_MARK_SIZE: i32 = 0x23;
pub const ETC_COLUMN_X: i32 = 0xd7;
pub const ETC_ROW_PITCH: i32 = 0x1b;
pub const ETC_ROW_Y: i32 = 8;
pub const ETC_BACK_SIZE: i32 = 0x1a;
pub const ETC_ICON_SIZE: i32 = 0x18;
pub const SKILL_GAP_X: i32 = 5;
pub const SKILL_GAP_Y: i32 = 3;
pub const SKILL_OVERLAY_SIZE: i32 = 0x20;
pub const LEGACY_INVENTORY_BACK: Rect = Rect { x: 0, y: 0x14, w: 0x100, h: 0x17d };
pub const LEGACY_ADENA_ICON: Rect = Rect { x: 0x62, y: 0x163, w: 0x10, h: 0xc };
pub const LEGACY_WEIGHT_ICON: Rect = Rect { x: 0x62, y: 0x174, w: 0x10, h: 0xc };
pub const LEGACY_COUNT_TEXT_Y: i32 = 0xa2;
pub const LEGACY_COUNT_TEXT_COLOR: u32 = 0xFFB09B79;

pub fn equip_slot_origin(slot: usize) -> Option<(i32, i32)> {
    (slot < EQUIP_SLOTS).then(|| (EQUIP_COLUMNS[slot % 5], EQUIP_ROWS[slot / 5]))
}

pub fn two_hand_origins() -> [(i32, i32); 2] {
    [(EQUIP_COLUMNS[0], EQUIP_ROWS[1]), (EQUIP_COLUMNS[2], EQUIP_ROWS[1])]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EquipCell {
    pub slot: usize,
    pub icon: String,
    pub clicked: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EtcCell {
    pub icon: Option<String>,
    pub disabled: bool,
}

pub fn paint_equip_slots(list: &mut DrawList, origin: (i32, i32), icon_size: i32, cells: &[EquipCell], alpha: f32) {
    for cell in cells {
        let Some((sx, sy)) = equip_slot_origin(cell.slot) else { continue };
        let (x, y) = (origin.0 + sx, origin.1 + sy);
        list.draw_texture(x, y, icon_size, icon_size, 0, 0, icon_size, icon_size, &cell.icon, alpha);
        if cell.clicked {
            list.draw_texture(
                x - 1,
                y - 1,
                EQUIP_OUTLINE_SIZE,
                EQUIP_OUTLINE_SIZE,
                0,
                0,
                EQUIP_OUTLINE_SIZE,
                EQUIP_OUTLINE_SIZE,
                EQUIP_CLICK,
                alpha,
            );
        } else {
            list.draw_texture(
                x - 1,
                y - 1,
                EQUIP_OUTLINE_SIZE,
                EQUIP_OUTLINE_SIZE,
                0,
                0,
                EQUIP_OUTLINE_SIZE,
                EQUIP_OUTLINE_SIZE,
                EQUIP_OUTLINE_DOWN,
                alpha,
            );
            list.draw_texture(
                x - 1,
                y - 1,
                EQUIP_MARK_SIZE,
                EQUIP_MARK_SIZE,
                0,
                0,
                EQUIP_MARK_SIZE,
                EQUIP_MARK_SIZE,
                EQUIP_OUTLINE,
                alpha,
            );
        }
    }
}

pub fn etc_cell_rect(index: i32) -> Rect {
    Rect::new(ETC_COLUMN_X, index * ETC_ROW_PITCH + ETC_ROW_Y, ETC_BACK_SIZE, ETC_BACK_SIZE)
}

pub fn paint_etc_column(list: &mut DrawList, origin: (i32, i32), cells: &[EtcCell], alpha: f32) {
    for (index, cell) in cells.iter().enumerate() {
        let back = etc_cell_rect(index as i32);
        list.draw_texture(
            origin.0 + back.x,
            origin.1 + back.y,
            back.w,
            back.h,
            0,
            0,
            back.w,
            back.h,
            EQUIP_MARK,
            alpha,
        );
        if let Some(icon) = &cell.icon {
            let (x, y) = (origin.0 + back.x + 1, origin.1 + back.y + 1);
            list.draw_texture(x, y, ETC_ICON_SIZE, ETC_ICON_SIZE, 0, 0, ETC_ICON_SIZE, ETC_ICON_SIZE, icon, alpha);
            if cell.disabled {
                list.draw_texture(
                    x + 1,
                    y,
                    ETC_ICON_SIZE,
                    ETC_ICON_SIZE,
                    0,
                    0,
                    ETC_ICON_SIZE,
                    ETC_ICON_SIZE,
                    ETC_DISABLED,
                    alpha,
                );
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillCell {
    pub icon: Option<String>,
    pub learned: bool,
    pub selected: bool,
    pub toggled: bool,
    pub disabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkillGrid {
    pub columns: i32,
    pub rows: i32,
    pub page: i32,
    pub cell: (i32, i32),
}

pub fn skill_cell_origin(grid: SkillGrid, index: i32) -> Option<(i32, i32)> {
    let first = grid.page * grid.columns;
    if index < first || grid.columns <= 0 {
        return None;
    }
    let local = index - first;
    let (column, row) = (local % grid.columns, local / grid.columns);
    let y = 1 + row * (grid.cell.1 + SKILL_GAP_Y);
    ((grid.cell.1 + 1) * grid.rows > y).then(|| (1 + column * (grid.cell.0 + SKILL_GAP_X), y))
}

pub fn paint_skill_grid(
    list: &mut DrawList,
    origin: (i32, i32),
    grid: SkillGrid,
    cells: &[SkillCell],
    back: Option<&str>,
    alpha: f32,
) {
    for (index, cell) in cells.iter().enumerate() {
        let Some((cx, cy)) = skill_cell_origin(grid, index as i32) else { continue };
        let (x, y) = (origin.0 + cx, origin.1 + cy);
        let (w, h) = grid.cell;
        if let Some(back) = back {
            list.draw_texture(x, y, w, h, 0, 0, w, h, back, alpha);
        }
        if let Some(icon) = &cell.icon {
            list.draw_texture(x, y, w, h, 0, 0, w, h, icon, alpha);
        }
        if !cell.learned {
            list.draw_texture(
                x - 1,
                y - 1,
                EQUIP_OUTLINE_SIZE,
                EQUIP_OUTLINE_SIZE,
                0,
                0,
                EQUIP_OUTLINE_SIZE,
                EQUIP_OUTLINE_SIZE,
                SKILL_OUTLINE,
                alpha,
            );
            if cell.selected {
                list.draw_texture(
                    x - 1,
                    y - 1,
                    EQUIP_OUTLINE_SIZE,
                    EQUIP_OUTLINE_SIZE,
                    0,
                    0,
                    EQUIP_OUTLINE_SIZE,
                    EQUIP_OUTLINE_SIZE,
                    SKILL_OUTLINE_DOWN,
                    alpha,
                );
            }
        }
        if cell.toggled {
            list.draw_texture(
                x,
                y,
                SKILL_OVERLAY_SIZE,
                SKILL_OVERLAY_SIZE,
                0,
                0,
                SKILL_OVERLAY_SIZE,
                SKILL_OVERLAY_SIZE,
                SKILL_TOGGLE,
                alpha,
            );
        }
        if cell.disabled {
            list.draw_texture(
                x,
                y,
                SKILL_OVERLAY_SIZE,
                SKILL_OVERLAY_SIZE,
                0,
                0,
                SKILL_OVERLAY_SIZE,
                SKILL_OVERLAY_SIZE,
                ETC_DISABLED,
                alpha,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equip_slots_sit_on_the_five_by_three_table_of_the_dll() {
        assert_eq!(equip_slot_origin(0), Some((7, 7)));
        assert_eq!(equip_slot_origin(4), Some((0xa9, 7)));
        assert_eq!(equip_slot_origin(5), Some((7, 0x2d)));
        assert_eq!(equip_slot_origin(12), Some((0x55, 0x53)));
        assert_eq!(equip_slot_origin(14), Some((0xa9, 0x53)));
        assert_eq!(equip_slot_origin(15), None);
        assert_eq!(two_hand_origins(), [(7, 0x2d), (0x55, 0x2d)]);
        assert_eq!(etc_cell_rect(0), Rect::new(0xd7, 8, 0x1a, 0x1a));
        assert_eq!(etc_cell_rect(2), Rect::new(0xd7, 8 + 2 * 0x1b, 0x1a, 0x1a));
    }

    #[test]
    fn equip_cells_draw_icon_then_outline_and_mark_at_minus_one() {
        let mut list = DrawList::default();
        let cells = vec![
            EquipCell { slot: 6, icon: "Icon.a".into(), clicked: false },
            EquipCell { slot: 1, icon: "Icon.b".into(), clicked: true },
        ];
        paint_equip_slots(&mut list, (10, 20), 32, &cells, 1.0);
        let quads: Vec<(&str, f32, f32, f32)> =
            list.quads.iter().map(|q| (q.texture.as_str(), q.x, q.y, q.w)).collect();
        assert_eq!(quads[0], ("Icon.a", 10.0 + 0x2e as f32, 20.0 + 0x2d as f32, 32.0));
        assert_eq!(quads[1], (EQUIP_OUTLINE_DOWN, 10.0 + 0x2d as f32, 20.0 + 0x2c as f32, 34.0));
        assert_eq!(quads[2], (EQUIP_OUTLINE, 10.0 + 0x2d as f32, 20.0 + 0x2c as f32, 35.0));
        assert_eq!(quads[3], ("Icon.b", 10.0 + 0x2e as f32, 27.0, 32.0));
        assert_eq!(quads[4], (EQUIP_CLICK, 10.0 + 0x2d as f32, 26.0, 34.0));
        let mut list = DrawList::default();
        paint_etc_column(&mut list, (0, 0), &[EtcCell { icon: Some("Icon.c".into()), disabled: true }], 1.0);
        let quads: Vec<(&str, f32, f32, f32)> =
            list.quads.iter().map(|q| (q.texture.as_str(), q.x, q.y, q.w)).collect();
        assert_eq!(quads[0], (EQUIP_MARK, 0xd7 as f32, 8.0, 0x1a as f32));
        assert_eq!(quads[1], ("Icon.c", 0xd8 as f32, 9.0, 0x18 as f32));
        assert_eq!(quads[2], (ETC_DISABLED, 0xd9 as f32, 9.0, 0x18 as f32));
    }

    #[test]
    fn skill_cells_step_by_cell_plus_five_and_three_from_one_one() {
        let grid = SkillGrid { columns: 6, rows: 10, page: 0, cell: (32, 32) };
        assert_eq!(skill_cell_origin(grid, 0), Some((1, 1)));
        assert_eq!(skill_cell_origin(grid, 1), Some((38, 1)));
        assert_eq!(skill_cell_origin(grid, 6), Some((1, 36)));
        assert_eq!(skill_cell_origin(grid, 59), Some((1 + 5 * 37, 1 + 9 * 35)));
        assert_eq!(skill_cell_origin(grid, 60), None);
        let paged = SkillGrid { page: 1, ..grid };
        assert_eq!(skill_cell_origin(paged, 5), None);
        assert_eq!(skill_cell_origin(paged, 6), Some((1, 1)));
        let mut list = DrawList::default();
        let cell =
            SkillCell { icon: Some("Icon.s".into()), learned: false, selected: true, toggled: true, disabled: false };
        paint_skill_grid(&mut list, (3, 4), grid, &[cell], None, 1.0);
        let quads: Vec<(&str, f32, f32, f32)> =
            list.quads.iter().map(|q| (q.texture.as_str(), q.x, q.y, q.w)).collect();
        assert_eq!(quads[0], ("Icon.s", 4.0, 5.0, 32.0));
        assert_eq!(quads[1], (SKILL_OUTLINE, 3.0, 4.0, 34.0));
        assert_eq!(quads[2], (SKILL_OUTLINE_DOWN, 3.0, 4.0, 34.0));
        assert_eq!(quads[3], (SKILL_TOGGLE, 4.0, 5.0, 32.0));
    }
}
