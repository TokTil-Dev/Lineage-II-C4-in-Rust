use killer_assets::client::Client;
use l2_dat::Chronicle;
use std::path::PathBuf;

const FOOTER_LEN: usize = 13;

fn exact<T>(client: &mut Client, name: &str, read: impl Fn(&[u8]) -> Result<Vec<T>, l2_dat::Error>, rows: usize) {
    let body = client.read_system_file(name).unwrap();
    let tail = &body[body.len() - FOOTER_LEN..];
    assert_eq!(tail, b"\x0cSafePackage\x00", "{name}: rodapé");
    let parsed = read(&body[..body.len() - FOOTER_LEN]).unwrap_or_else(|error| panic!("{name}: {error}"));
    assert_eq!(parsed.len(), rows, "{name}: linhas");
    assert!(read(&body[..body.len() - FOOTER_LEN - 1]).is_err(), "{name}: sobra antes do rodapé");
}

#[test]
#[ignore]
fn high_five_tables_end_exactly_at_the_footer() {
    let root = PathBuf::from(std::env::var_os("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR"));
    let mut client = Client::open(&root).unwrap();
    let c = Chronicle::HighFive;
    assert_eq!(client.chronicle(), c);
    exact(&mut client, "weapongrp.dat", |b| l2_dat::weapongrp::read_as(b, c), 4060);
    exact(&mut client, "armorgrp.dat", |b| l2_dat::armorgrp::read_as(b, c), 3650);
    exact(&mut client, "etcitemgrp.dat", |b| l2_dat::etcitemgrp::read_as(b, c), 11487);
    exact(&mut client, "npcgrp.dat", |b| l2_dat::npcgrp::read_as(b, c), 10461);
    exact(&mut client, "itemname-e.dat", |b| l2_dat::itemname::read_as(b, c), 19197);
    exact(&mut client, "skillgrp.dat", |b| l2_dat::skillgrp::read_as(b, c), 65871);
    exact(&mut client, "skillsoundgrp.dat", |b| l2_dat::skillsoundgrp::read_as(b, c), 5314);
    exact(&mut client, "questname-e.dat", |b| l2_dat::questname::read_as(b, c), 3379);
    exact(&mut client, "castlename-e.dat", |b| l2_dat::castlename::read_as(b, c), 83);
    exact(&mut client, "Recipe-c.dat", |b| l2_dat::recipe::read_as(b, c), 1001);
    exact(&mut client, "chargrp.dat", l2_dat::chargrp::read_high_five, 17);
    exact(&mut client, "systemmsg-e.dat", |b| l2_dat::systemmsg::read_as(b, c), 3258);
    let zones = client.read_system_file("zonename-e.dat").unwrap();
    assert_eq!(l2_dat::zonename::read_as(&zones, c).unwrap().len(), 497);
    assert!(l2_dat::zonename::read_as(&zones[..zones.len() - 1], c).is_err());
    let eula = client.read_system_file("eula-e.dat").unwrap();
    let (_, extra) = l2_dat::eula::read_high_five(&eula).unwrap();
    assert!(extra.iter().all(String::is_empty));
    assert!(l2_dat::eula::read_high_five(&eula[..eula.len() - 1]).is_err());
    let logon = client.read_system_file("logongrp.dat").unwrap();
    assert_eq!(logon.len(), 8 * 16 + FOOTER_LEN);
    assert_eq!(l2_dat::logongrp::read_as(&logon, c).unwrap().slots[0].x, 150995.0);
    let obscene = client.read_system_file("obscene-e.dat").unwrap();
    assert!(l2_dat::obscene::read_as(&obscene, c).unwrap().is_empty());
}

#[test]
#[ignore]
fn high_five_item_groups_agree_with_their_icon_tables() {
    let root = PathBuf::from(std::env::var_os("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR"));
    let mut client = Client::open(&root).unwrap();
    let c = Chronicle::HighFive;
    for (name, kind, rows) in [
        ("weapongrp.dat", l2_dat::ItemKind::Weapon, 4060),
        ("armorgrp.dat", l2_dat::ItemKind::Armor, 3650),
        ("etcitemgrp.dat", l2_dat::ItemKind::Etc, 11487),
    ] {
        let body = client.read_system_file(name).unwrap();
        let icons = l2_dat::itemicons::read_as(&body, kind, c).unwrap();
        assert_eq!(icons.len(), rows, "{name}");
        assert!(icons.iter().all(|icon| icon.icons[0].is_empty() || icon.icons[0].contains('.')), "{name}");
    }
}

#[test]
#[ignore]
fn high_five_tables_load_and_chargrp_rows_are_chosen_by_their_own_fields() {
    let root = PathBuf::from(std::env::var_os("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR"));
    let mut client = Client::open(&root).unwrap();
    let tables = client.tables().unwrap();
    assert_eq!(tables.chargrp_high_five.len(), 17);
    assert!(tables.chargrp.is_empty() && tables.hairgrp.is_empty() && tables.helmetgrp.is_empty());
    for (race, sex, class_id, name) in [
        (0, 0, 0, "MFighter"),
        (0, 1, 10, "FMagic"),
        (1, 0, 18, "MElf"),
        (2, 1, 31, "FDarkElf"),
        (3, 0, 49, "MShaman"),
        (4, 1, 53, "FDwarf"),
        (5, 0, 123, "MKamael"),
        (5, 1, 124, "FKamael"),
    ] {
        assert_eq!(tables.char_row_high_five(race, sex, class_id).map(|row| row.name.as_str()), Some(name));
    }
    for row in &tables.chargrp_high_five[..16] {
        assert!(tables.char_row_high_five(row.race as i32, row.sex as i32, row.class_id as i32).is_some_and(|found| found.name == row.name));
    }
}
