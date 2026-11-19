use killer_assets::client::Client;
use l2_dat::DatReader;
use std::collections::BTreeMap;
use std::path::Path;

fn histogram(name: &str, values: &[u32]) {
    let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
    for value in values {
        *counts.entry(*value).or_default() += 1;
    }
    let distinct = counts.len();
    let in_icon_range = values.iter().filter(|v| (0..=7).contains(*v) || (11..=16).contains(*v)).count();
    println!("{name}: {distinct} valores distintos, {}/{} dentro de 0-7 ou 11-16", in_icon_range, values.len());
    let shown: Vec<String> = counts.iter().take(24).map(|(v, c)| format!("{v}x{c}")).collect();
    println!("  {}", shown.join(" "));
}

fn main() {
    let client_dir = std::env::args().nth(1).unwrap();
    let mut client = Client::open(Path::new(&client_dir)).unwrap();
    let body = client.read_system_file("skillgrp.dat").unwrap();
    let mut r = DatReader::new(&body);
    let count = r.u32().unwrap();
    let mut third = Vec::new();
    let mut after_icon_panel = Vec::new();
    let mut tail = Vec::new();
    let mut samples = Vec::new();
    for index in 0..count {
        let id = r.u32().unwrap();
        let level = r.u32().unwrap();
        let unnamed_third = r.u32().unwrap();
        let _oper_type = r.u32().unwrap();
        let _mp = r.u32().unwrap();
        let _range = r.i32().unwrap();
        let _style = r.u32().unwrap();
        let _hit = r.f32().unwrap();
        let _magic = r.i32().unwrap();
        let _ani = r.unicode().unwrap();
        let _preset = r.unicode().unwrap();
        let icon = r.unicode().unwrap();
        let icon_panel = r.unicode().unwrap();
        let extra_eff = r.u32().unwrap();
        let _is_ench = r.u32().unwrap();
        let _ench_level = r.u32().unwrap();
        let ench_icon = r.ascf().unwrap();
        let _hp = r.u32().unwrap();
        let _rumble_self = r.i32().unwrap();
        let _rumble_target = r.i32().unwrap();
        let unnamed_tail = r.u32().unwrap();
        let _trailing = r.ascf().unwrap();
        third.push(unnamed_third);
        after_icon_panel.push(extra_eff);
        tail.push(unnamed_tail);
        if index < 6 {
            samples.push(format!(
                "id={id} lvl={level} terceiro={unnamed_third} icon={icon:?} icon_panel={icon_panel:?} apos_panel={extra_eff} ench_icon={ench_icon:?} cauda={unnamed_tail}"
            ));
        }
    }
    println!("registros: {count}\n");
    for line in samples {
        println!("{line}");
    }
    println!();
    histogram("coluna 3 (sem nome, logo apos level)", &third);
    histogram("coluna apos icon_panel (hoje chamada extra_eff)", &after_icon_panel);
    histogram("coluna sem nome perto do fim", &tail);
}
