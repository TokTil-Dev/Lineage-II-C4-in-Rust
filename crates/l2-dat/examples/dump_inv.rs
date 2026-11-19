use std::collections::BTreeSet;
fn main() {
    let client = std::env::args().nth(1).unwrap();
    let path = format!("{client}/system/interface.xdat");
    let body = std::fs::read(&path).unwrap();
    let interface = l2_dat::xdat::read_interface(&body).unwrap();
    let mut wanted: BTreeSet<String> = BTreeSet::new();
    for set in interface.shortcut_sets.iter().filter(|s| s.state == "GamingState") {
        for key in &set.keys {
            if let Some(rest) = key.command.strip_prefix("ShowWindow Name=") {
                wanted.insert(rest.split_whitespace().next().unwrap_or(rest).to_string());
            }
        }
    }
    println!("=== janelas abertas por atalho (GamingState) ===");
    for name in &wanted {
        match interface.windows.iter().find(|w| w.head.name.eq_ignore_ascii_case(name)) {
            Some(w) => println!("{name}: {} controles, size {:?}", w.control_count(), w.head.size),
            None => println!("{name}: SEM DEFINIÇÃO no xdat (função nativa do engine)"),
        }
    }
    println!("\n=== botões do SystemMenuWnd ===");
    if let Some(menu) = interface.windows.iter().find(|w| w.head.name.eq_ignore_ascii_case("SystemMenuWnd")) {
        fn walk(c: &l2_dat::xdat::Control, out: &mut Vec<String>) {
            if c.kind == "Button" { out.push(c.head.name.clone()); }
            for k in &c.children { walk(k, out); }
        }
        let mut names = Vec::new();
        for c in &menu.children { walk(c, &mut names); }
        println!("{}", names.join(", "));
    }
}
