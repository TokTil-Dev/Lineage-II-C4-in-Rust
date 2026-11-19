use l2_xdat::{Kind, Node, Window};
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = PathBuf::from(args.next().expect("caminho do interface.xdat"));
    let data = std::fs::read(&path).expect("ler o arquivo");
    let xdat = l2_xdat::read_file(&data, &path).expect("ler o xdat");
    let wanted: Vec<String> = args.collect();
    if wanted.is_empty() {
        for window in &xdat.windows {
            println!(
                "{}  filhos {}  {:?}",
                window.base.name,
                window.nodes().len(),
                window.base.size.as_ref().map(|s| (s.width, s.height))
            );
        }
        println!("atalhos:");
        for shortcut in &xdat.shortcuts {
            println!("  {} {} ({} ações)", shortcut.unk0, shortcut.unk1, shortcut.actions.len());
        }
        println!("posições padrão:");
        for p in xdat.wnd_def_pos() {
            println!(
                "  {} {:?} ({}, {}) {}x{} mover pai {}",
                p.wnd, p.alignment, p.x, p.y, p.width, p.height, p.move_parent
            );
        }
        return;
    }
    for name in wanted {
        match xdat.window(&name) {
            Some(window) => dump_window(window),
            None => println!("{name}: não existe"),
        }
    }
}

fn dump_window(window: &Window) {
    println!("== {} ==", window.base.name);
    println!("base: {:?}", window.base);
    println!("{window:#?}");
    for child in &window.children {
        dump_node(child, 1);
    }
}

fn dump_node(node: &Node, depth: usize) {
    let pad = "  ".repeat(depth);
    let b = &node.base;
    let size = b
        .size
        .as_ref()
        .map(|s| match &s.percent {
            Some(rate) => format!(
                "{}x{} ({}*pai+{}, {}*pai+{} de {:?})",
                s.width, s.height, rate.width, s.width, rate.height, s.height, rate.relative_to
            ),
            None => format!("{}x{}", s.width, s.height),
        })
        .unwrap_or_default();
    let anchor = b
        .anchor
        .as_ref()
        .map(|a| format!("{:?}/{:?} {:?} ({}, {})", a.parent, a.this, a.ctrl, a.x, a.y))
        .unwrap_or_default();
    println!(
        "{pad}{} {} {size} {anchor} top/back ({}, {}) pai {:?} tab ({:?}, {:?}) anchored {} virtual/pass/unk68 ({}, {}, {}) tooltip {:?}/{}",
        node.kind.class_name(),
        b.name,
        b.always_on_top,
        b.always_on_back,
        b.parent_name,
        b.tab_pre_order,
        b.tab_post_order,
        b.anchored,
        b.virtual_,
        b.pass_through,
        b.unk68,
        b.tooltip_type,
        b.tooltip_text
    );
    match &node.kind {
        Kind::Window(w) => {
            println!("{pad}  {w:?}");
            for child in &w.children {
                dump_node(child, depth + 1);
            }
        }
        Kind::ScrollArea(a) => {
            for child in &a.children {
                dump_node(child, depth + 1);
            }
        }
        other => println!("{pad}  {other:?}"),
    }
}
