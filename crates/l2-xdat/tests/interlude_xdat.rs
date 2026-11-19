use l2_xdat::{Alignment, Kind, Xdat};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn interlude_xdat() -> Xdat {
    let dir = PathBuf::from(std::env::var_os("KILLER_INTERLUDE_DIR").expect("KILLER_INTERLUDE_DIR"));
    let path = dir.join("system/interface.xdat");
    let data = std::fs::read(&path).expect("interface.xdat");
    l2_xdat::read_file(&data, &path).expect("interface.xdat inteiro")
}

#[test]
#[ignore]
fn the_whole_interlude_interface_xdat_is_consumed_window_by_window() {
    let xdat = interlude_xdat();
    let mut by_class = BTreeMap::new();
    let mut nodes = 0usize;
    for window in &xdat.windows {
        for node in window.nodes() {
            nodes += 1;
            *by_class.entry(node.kind.class_name()).or_insert(0usize) += 1;
        }
    }
    eprintln!(
        "janelas {} controles {} atalhos {} posições padrão {} cauda {:?}",
        xdat.windows.len(),
        nodes,
        xdat.shortcuts.len(),
        xdat.wnd_def_pos().count(),
        xdat.tail.len()
    );
    for (class, count) in &by_class {
        eprintln!("  {count:5}  {class}");
    }
    assert_eq!(xdat.windows.len(), 172);
    assert_eq!(xdat.default_positions.len(), 1);
    assert_eq!(nodes, 4102);
    assert_eq!(xdat.shortcuts.len(), 8);
    assert_eq!(xdat.wnd_def_pos().count(), 51);
    assert_eq!(xdat.tail.len(), 20);
    assert_eq!(by_class.len(), 30, "{by_class:?}");
    assert!(!by_class.contains_key("ScrollArea"));

    let first = &xdat.windows[0];
    assert_eq!(first.base.name, "AbnormalDebuffsStatusWnd");
    let size = first.base.size.as_ref().unwrap();
    assert_eq!((size.width, size.height), (38, 26));
    let anchor = first.base.anchor.as_ref().unwrap();
    assert_eq!((anchor.parent, anchor.this, anchor.x, anchor.y), (Alignment::TopLeft, Alignment::TopLeft, 176, 0));
    assert_eq!(first.script, "AbnormalDebuffsStatusWnd");
    assert_eq!(first.state, "GamingState");
    assert_eq!(first.working_configuration, "Game");
    assert_eq!(first.children.len(), 2);
    assert_eq!(first.children[0].base.name, "StatusIcon");
    assert!(matches!(first.children[0].kind, Kind::StatusIconCtrl(_)));

    let recipe = xdat.window("RecipeBookWnd").expect("RecipeBookWnd");
    let trash = recipe.find("btnTrash").expect("btnTrash");
    let Kind::Button(trash) = &trash.kind else { panic!("{:?}", trash.kind) };
    assert!(trash.normal_tex.ends_with("Inventory_trash"), "{}", trash.normal_tex);
    eprintln!(
        "RecipeBookWnd.btnTrash: {} / {} / {} / {}",
        trash.normal_tex, trash.pushed_tex, trash.highlight_tex, trash.drop_tex
    );
    assert!(trash.drop_tex.ends_with("Inventory_trash_Drag"), "{}", trash.drop_tex);
    let count = recipe.find("txtCount").expect("txtCount");
    let Kind::TextBox(count) = &count.kind else { panic!("{:?}", count.kind) };
    eprintln!("RecipeBookWnd.txtCount: {:?} sysstring {} fonte {}", count.text, count.sysstring, count.font_type);
    assert_eq!(count.text, "(0/0)");
    assert!(
        recipe.nodes().iter().any(|node| matches!(node.kind, Kind::ItemWindow(_)) && node.base.name == "RecipeItem")
    );

    let defaults: Vec<_> = xdat.wnd_def_pos().map(|p| p.wnd.as_str()).collect();
    assert!(defaults.contains(&"InventoryWnd"), "{defaults:?}");
    assert!(xdat.shortcuts.iter().any(|s| s.actions.iter().any(|a| a.action.contains("Inventory"))));
}
