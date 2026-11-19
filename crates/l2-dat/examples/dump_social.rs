fn main() {
    let client = std::env::args().nth(1).unwrap();
    let path = format!("{client}/system/interface.xdat");
    let body = std::fs::read(&path).unwrap();
    let interface = l2_dat::xdat::read_interface(&body).unwrap();
    let names = ["MenuWnd", "ClanWnd", "ClanDrawerWnd", "QuestTreeWnd", "QuestTreeDrawerWnd", "PartyMatchWnd", "RadarMapWnd"];
    for name in names {
        match interface.windows.iter().find(|window| window.head.name.eq_ignore_ascii_case(name)) {
            Some(window) => println!("{}", l2_dat::xdat::dump_window(window)),
            None => println!("Window {name}: não encontrada\n"),
        }
    }
    for layout in interface.layouts.iter().filter(|layout| names.iter().any(|name| layout.window.eq_ignore_ascii_case(name))) {
        println!("layout {:?} = {:?}", layout.window, layout.values);
    }
}
