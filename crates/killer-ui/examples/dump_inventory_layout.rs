use killer_ui::window::Rect;
use killer_ui::xdat_window;

fn main() {
    let client = std::env::args().nth(1).unwrap();
    let path = format!("{client}/system/interface.xdat");
    let body = std::fs::read(&path).unwrap();
    let interface = l2_dat::xdat::read_interface(&body).unwrap();
    let window = interface.windows.iter().find(|w| w.head.name.eq_ignore_ascii_case("InventoryWnd")).unwrap();
    let defaults: &dyn Fn(&str) -> Option<[i32; 3]> =
        &|name| interface.layouts.iter().find(|layout| layout.window == name).map(|layout| [layout.values[0], layout.values[1], layout.values[2]]);
    let viewport = Rect::new(0, 0, 1280, 956);
    let layout = xdat_window::layout_with(window, viewport, &|_| None, defaults);
    println!("window rect = {:?}", layout.rect);
    for name in ["EquipItem_Chest", "EquipItem_Underwear", "EquipItem_Head", "InventoryItem", "QuestItem", "InventoryTab", "AdenaText", "InvenWeight"] {
        match layout.find(name) {
            Some(item) => println!("{name}: rect={:?} kind={}", item.rect, item.control.kind),
            None => println!("{name}: não encontrado no layout"),
        }
    }
}
