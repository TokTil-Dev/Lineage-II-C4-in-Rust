use l2_dat::xdat::{self, Props};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn interface_file(root_var: &str) -> Option<Vec<u8>> {
    let Some(root) = std::env::var_os(root_var).map(PathBuf::from) else {
        eprintln!("{root_var} not set; skipping");
        return None;
    };
    let path = root.join("system").join("interface.xdat");
    let raw = std::fs::read(&path).unwrap();
    Some(l2_crypto::decrypt(&raw, &path).unwrap().into_owned())
}

#[test]
#[ignore]
fn high_five_interface_xdat_is_consumed_exactly() {
    let bytes = interface_file("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR");
    let interface = xdat::read_interface(&bytes).unwrap();
    assert_eq!(interface.shortcut_sets.len(), 17);
    assert_eq!(interface.shortcut_sets.iter().map(|set| set.keys.len()).sum::<usize>(), 341);
    assert_eq!(interface.windows.len(), 263);
    assert_eq!(interface.control_count(), 4790);
    assert_eq!(interface.layouts.len(), 91);
    assert_eq!(interface.fonts.fonts.len(), 17);
    assert_eq!(interface.palette.len(), 24);
    assert_eq!(interface.trailer, [0, 67, 216, -1_733_207_327, 0]);
    let mut kinds: BTreeMap<&str, usize> = BTreeMap::new();
    fn walk<'a>(controls: &'a [xdat::Control], kinds: &mut BTreeMap<&'a str, usize>) {
        for control in controls {
            *kinds.entry(control.kind.as_str()).or_default() += 1;
            walk(&control.children, kinds);
        }
    }
    for window in &interface.windows {
        walk(&window.children, &mut kinds);
    }
    assert_eq!(kinds.len(), 35);
    assert_eq!(kinds["TextBox"], 1269);
    assert_eq!(kinds["Texture"], 1372);
    assert_eq!(kinds["Button"], 856);
    assert_eq!(kinds["Window"], 286);
    for name in ["StatusWnd", "InventoryWnd", "ChatWnd"] {
        assert!(interface.window(name).is_some(), "{name}");
    }
    let names: std::collections::HashSet<_> = interface.windows.iter().map(|w| w.head.name.as_str()).collect();
    assert_eq!(names.len(), 263);
    assert_eq!(interface.layouts[0].window, "MainWnd");
    let shop = interface.layouts.iter().find(|layout| layout.window == "ShopWnd").unwrap();
    assert_eq!(shop.values, [1, 200, 150, 0, -9999, -9999]);
    let button = interface
        .windows
        .iter()
        .flat_map(|w| w.children.iter())
        .find_map(|c| if let Props::Button(b) = &c.props { Some(b) } else { None })
        .unwrap();
    assert!(!button.textures[0].is_empty());
}

#[test]
#[ignore]
fn high_five_dump_of_known_windows() {
    let bytes = interface_file("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR");
    let interface = xdat::read_interface(&bytes).unwrap();
    for name in ["StatusWnd", "InventoryWnd", "ChatWnd"] {
        let window = interface.window(name).unwrap();
        println!("{}", xdat::dump_window(window));
    }
}

#[test]
#[ignore]
fn interlude_interface_xdat_header_decrypts() {
    let Some(bytes) = interface_file("KILLER_INTERLUDE_DIR") else { return };
    assert!(bytes.len() > 1_000_000);
    assert_eq!(u32::from_le_bytes(bytes[..4].try_into().unwrap()), 172);
}

#[test]
#[ignore]
fn high_five_dump_of_lobby_windows() {
    let bytes = interface_file("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR");
    let interface = xdat::read_interface(&bytes).unwrap();
    for window in &interface.windows {
        if window.head.name.contains("Lobby") || window.head.name.contains("Login") || window.head.name.contains("Server") || window.head.name.contains("Create") {
            println!("{}", xdat::dump_window(window));
        }
    }
}

#[test]
#[ignore]
fn high_five_window_states_table() {
    let bytes = interface_file("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR");
    let interface = xdat::read_interface(&bytes).unwrap();
    for window in &interface.windows {
        let part = &window.part;
        println!(
            "{}\tstate={}\twindow_parent={}\tworking_configuration={}\tadditional_state={:?}",
            window.head.name, part.state, part.window_parent, part.working_configuration, part.additional_state
        );
    }
}

#[test]
#[ignore]
fn high_five_dump_of_named_windows() {
    let Some(names) = std::env::var_os("XDAT_WINDOWS") else { return };
    let names = names.to_string_lossy().into_owned();
    let bytes = interface_file("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR");
    let interface = xdat::read_interface(&bytes).unwrap();
    for name in names.split(',') {
        match interface.window(name) {
            Some(window) => println!("{}", xdat::dump_window(window)),
            None => println!("Window {name} ausente"),
        }
    }
}

#[test]
#[ignore]
fn high_five_root_window_anchor_targets() {
    let bytes = interface_file("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR");
    let interface = xdat::read_interface(&bytes).unwrap();
    let names: std::collections::HashSet<&str> = interface.windows.iter().map(|w| w.head.name.as_str()).collect();
    let layouts: std::collections::HashSet<&str> = interface.layouts.iter().map(|l| l.window.as_str()).collect();
    for window in &interface.windows {
        if let Some(anchor) = &window.head.anchor
            && !anchor.target.is_empty()
        {
            let has_layout = layouts.contains(window.head.name.as_str());
            let target_exists = names.contains(anchor.target.as_str());
            println!("{} anchor target={:?} tem_layout={has_layout} alvo_existe={target_exists}", window.head.name, anchor.target);
        }
    }
}

#[test]
#[ignore]
fn high_five_dump_of_layouts() {
    let bytes = interface_file("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR");
    let interface = xdat::read_interface(&bytes).unwrap();
    for layout in &interface.layouts {
        println!("{} {:?}", layout.window, layout.values);
    }
    for set in &interface.shortcut_sets {
        println!("SET {} state={} mode={} keys={}", set.name, set.state, set.mode, set.keys.len());
    }
}

#[test]
#[ignore]
fn high_five_dump_of_shortcut_keys() {
    let bytes = interface_file("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR");
    let interface = xdat::read_interface(&bytes).unwrap();
    for set in interface.shortcut_sets.iter().filter(|set| set.state == "GamingState") {
        println!("SET {}", set.name);
        for key in &set.keys {
            println!("  key={:#x} a={} b={} edge={} {}", key.key, key.modifier_a, key.modifier_b, key.edge, key.command);
        }
    }
}

#[test]
#[ignore]
fn high_five_item_window_props() {
    let bytes = interface_file("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR");
    let interface = xdat::read_interface(&bytes).unwrap();
    fn walk(controls: &[xdat::Control]) {
        for control in controls {
            if let Props::ItemWindow(props) = &control.props
                && (control.head.name == "InventoryItem" || control.head.name == "EquipItem_Head" || control.head.name == "WarehouseItem")
            {
                println!("{} head={:?} groups={:?} values={:?} fill={:?} extras={:?}", control.head.name, props.head, props.groups, props.values, props.fill, props.extras);
            }
            walk(&control.children);
        }
    }
    for window in &interface.windows {
        walk(&window.children);
    }
}
