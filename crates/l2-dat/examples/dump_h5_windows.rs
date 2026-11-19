fn main() {
    let client = std::env::args().nth(1).unwrap();
    let path = format!("{client}/system/interface.xdat");
    let body = std::fs::read(&path).unwrap();
    let interface = l2_dat::xdat::read_interface(&body).unwrap();
    for name in std::env::args().skip(2) {
        match interface.windows.iter().find(|w| w.head.name.eq_ignore_ascii_case(&name)) {
            Some(window) => println!("{}", l2_dat::xdat::dump_window(window)),
            None => println!("Window {name}: não encontrada\n"),
        }
    }
    for layout in interface.layouts.iter() {
        println!("layout {:?} = {:?}", layout.window, layout.values);
    }
}
