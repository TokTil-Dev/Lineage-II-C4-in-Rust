use crate::{Chronicle, DatReader};

const HIGH_FIVE: Chronicle = Chronicle::HighFive;
const FOOTER: &[u8] = b"\x0cSafePackage\x00";

fn u(value: u32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

fn i(value: i32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

fn f(value: f32) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

fn uni(text: &str) -> Vec<u8> {
    let units: Vec<u8> = text.encode_utf16().flat_map(u16::to_le_bytes).collect();
    let mut out = u(units.len() as u32);
    out.extend(units);
    out
}

fn ascf(text: &str) -> Vec<u8> {
    let mut out = vec![(text.len() + 1) as u8];
    out.extend(text.bytes());
    out.push(0);
    out
}

fn cat(parts: Vec<Vec<u8>>) -> Vec<u8> {
    parts.concat()
}

fn array(items: &[&str]) -> Vec<u8> {
    let mut out = u(items.len() as u32);
    for item in items {
        out.extend(uni(item));
    }
    out
}

fn head(tag: u32, id: u32, icon: &str, ids: &[u32]) -> Vec<u8> {
    let mut out = cat(vec![u(tag), u(id), u(0), u(3), u(8), u(4), u(0)]);
    for name in ["drop.mesh", "", ""] {
        out.extend(uni(name));
    }
    for (index, name) in ["drop.tex", "", "", "", "", "", "", "", ""].iter().enumerate() {
        out.extend(uni(if index == 0 { name } else { "" }));
    }
    out.extend(cat(vec![u(1), u(0), u(0)]));
    for name in [icon, "", "", "", ""] {
        out.extend(uni(name));
    }
    out.extend(cat(vec![i(-1), u(6), u(13), u(1), u(7)]));
    out.extend(u(ids.len() as u32));
    for id in ids {
        out.extend(u(*id));
    }
    out.extend(u(1));
    out.extend(uni("icon.panel_2"));
    out
}

fn with_footer(count: u32, rows: Vec<Vec<u8>>) -> Vec<u8> {
    let mut out = u(count);
    for row in rows {
        out.extend(row);
    }
    out.extend(FOOTER);
    out
}

fn etc_record(id: u32) -> Vec<u8> {
    let mut out = head(2, id, "icon.etc_wooden_quiver_i00", &[335]);
    out.extend(array(&["Weapons.arrow"]));
    out.extend(array(&["WeaponsTex.arrow"]));
    out.extend(uni("ItemSound.drop"));
    out.extend(uni(""));
    out.extend(cat(vec![u(2), u(2), u(0)]));
    out
}

#[test]
fn etcitemgrp_reads_the_extended_head_with_nine_materials_and_the_panel_lists() {
    let body = with_footer(2, vec![etc_record(17), etc_record(18)]);
    let items = crate::etcitemgrp::read_as(&body[..body.len() - FOOTER.len()], HIGH_FIVE).unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!((items[0].id, items[1].id), (17, 18));
    assert_eq!(items[0].icons[0], "icon.etc_wooden_quiver_i00");
    assert_eq!((items[0].weight, items[0].type1), (6, 7));
    assert_eq!(items[0].meshes, ["Weapons.arrow"]);
    assert_eq!((items[0].stackable, items[0].family, items[0].grade), (2, 2, 0));
    assert!(crate::etcitemgrp::read_as(&body[..body.len() - FOOTER.len() - 1], HIGH_FIVE).is_err());
    assert!(crate::etcitemgrp::read_as(&body, Chronicle::C4).is_err());
}

#[test]
fn item_head_keeps_the_first_three_of_the_nine_drop_materials() {
    let mut bytes = head(2, 17, "icon.x", &[]);
    bytes.truncate(bytes.len());
    let mut reader = DatReader::new(&bytes);
    let head = crate::armorgrp::item_head_as(&mut reader, HIGH_FIVE).unwrap();
    assert_eq!(reader.remaining(), 0);
    assert_eq!(head.drop.meshes, ["drop.mesh", "", ""]);
    assert_eq!(head.drop.materials, ["drop.tex", "", ""]);
    assert_eq!((head.drop.style, head.drop.radius, head.drop.height), (0, 8, 4));
}

fn weapon_record(id: u32, meshes: &[&str], textures: &[&str]) -> Vec<u8> {
    let mut out = head(0, id, "icon.weapon_small_sword_i00", &[]);
    out.extend(cat(vec![u(27), u(1)]));
    out.extend(array(meshes));
    for _ in meshes {
        out.extend(u(textures.len() as u32));
    }
    out.extend(array(textures));
    out.extend(array(&["ItemSound.sword_small_1", "ItemSound.sword_big_7"]));
    out.extend(cat(vec![uni("ItemSound.itemdrop_sword"), uni("ItemSound.itemequip_sword"), uni("")]));
    out.extend(cat(vec![i(10), i(8), i(6), u(1)]));
    for value in 0..10 {
        out.extend(i(value));
    }
    out.extend(cat(vec![u(1000), u(0), i(-1), u(0)]));
    for _ in meshes {
        out.extend(uni(""));
    }
    for _ in meshes {
        out.extend(cat(vec![f(0.0), f(0.0), f(0.0)]));
    }
    for _ in meshes {
        out.extend(f(1.0));
    }
    for _ in meshes {
        out.extend(f(2.0));
    }
    for _ in meshes {
        out.extend(uni("Lineage.enchant"));
    }
    for _ in meshes {
        out.extend(cat(vec![f(0.95), f(0.55), f(0.55)]));
    }
    for _ in meshes {
        out.extend(cat(vec![f(11.0), f(0.0), f(0.0)]));
    }
    for _ in 0..6 {
        out.extend(i(-1));
    }
    for _ in 0..4 {
        out.extend(uni(""));
    }
    out
}

#[test]
fn weapongrp_reads_per_mesh_texture_counts_and_a_six_value_appendix() {
    let body = with_footer(2, vec![weapon_record(1, &["Lineage.sword"], &["Tex.sword"]), weapon_record(52, &["Lineage.a", "Lineage.b"], &["Tex.a0", "Tex.a1", "Tex.b0"])]);
    let weapons = crate::weapongrp::read_as(&body[..body.len() - FOOTER.len()], HIGH_FIVE).unwrap();
    assert_eq!(weapons.len(), 2);
    assert_eq!((weapons[0].id, weapons[0].body_part, weapons[0].handness), (1, 27, 1));
    assert_eq!(weapons[0].meshes, ["Lineage.sword"]);
    assert_eq!(weapons[0].textures, ["Tex.sword"]);
    assert_eq!(weapons[0].attack_sounds.len(), 2);
    assert_eq!(weapons[1].meshes.len(), 2);
    assert_eq!(weapons[1].textures.len(), 3);
    assert_eq!(weapons[1].enchant_meshes[1].vector_124, [0.95, 0.55, 0.55]);
    assert_eq!(weapons[0].curvature, 1000);
    assert!(crate::weapongrp::read_as(&body[..body.len() - FOOTER.len() - 1], HIGH_FIVE).is_err());
    assert!(crate::weapongrp::read_as(&body[..body.len() - FOOTER.len()], Chronicle::Interlude).is_err());
}

fn armor_row(mesh: &str, ex_meshes: &[&str]) -> Vec<u8> {
    let mut out = array(&[mesh]);
    out.extend(array(&["Tex.row"]));
    out.extend(u(ex_meshes.len() as u32));
    for ex in ex_meshes {
        out.extend(uni(ex));
        out.extend(0xff00u16.to_le_bytes());
    }
    out.extend(array(&[""]));
    out.extend(uni(""));
    out
}

fn armor_record(id: u32) -> Vec<u8> {
    let mut out = head(1, id, "icon.armor_t02_u_i00", &[]);
    out.extend(u(21));
    for row in 0..crate::armorgrp::ROWS_HIGH_FIVE {
        out.extend(armor_row(&format!("Race.row{row}"), if row == 14 { &["Ex.a", "Ex.b"] } else { &[""] }));
    }
    out.extend(array(&[""]));
    out.extend(array(&[""]));
    out.extend(cat(vec![u(0), u(0), u(0)]));
    out.extend(uni("LineageEffect.p_u002_a"));
    out.extend(array(&["MonSound.Hit_normal_3"]));
    out.extend(cat(vec![uni("ItemSound.drop"), uni("ItemSound.equip")]));
    out.extend(array(&[""]));
    for value in [1, 0, 0, 36, 1, 0, 48] {
        out.extend(i(value));
    }
    out
}

#[test]
fn armorgrp_reads_sixteen_rows_with_ex_meshes_followed_by_a_u16() {
    let body = with_footer(2, vec![armor_record(21), armor_record(22)]);
    let armors = crate::armorgrp::read_as(&body[..body.len() - FOOTER.len()], HIGH_FIVE).unwrap();
    assert_eq!(armors.len(), 2);
    assert_eq!((armors[0].id, armors[0].body_part), (21, 21));
    assert_eq!(armors[0].rows.len(), 16);
    assert_eq!(armors[0].mesh(14, 0), Some("Race.row14"));
    assert_eq!(armors[0].rows[14].ex_meshes, ["Ex.a", "Ex.b"]);
    assert_eq!(armors[0].rows[15].meshes, ["Race.row15"]);
    assert_eq!(armors[0].damage_effect, "LineageEffect.p_u002_a");
    assert_eq!(armors[0].attributes_58c, [1, 0, 0, 36, 1, 0]);
    assert_eq!(armors[1].equip_sound, "ItemSound.equip");
    assert!(crate::armorgrp::read_as(&body[..body.len() - FOOTER.len() - 1], HIGH_FIVE).is_err());
}

fn npc_record(id: u32, greetings: &[&str]) -> Vec<u8> {
    let mut out = cat(vec![u(id), uni("LineageMonster.gremlin"), uni("LineageMonsters.gremlin_m00"), array(&["Tex.t00"]), array(&[])]);
    out.push(2);
    out.extend(cat(vec![u(4416), u(13), f(1.0), u(0)]));
    out.extend(cat(vec![array(&[""]), array(&["MonSound.hit"]), array(&["MonSound.dmg"]), u(0)]));
    out.extend(cat(vec![vec![1], u(0), vec![1], u(0)]));
    out.extend(cat(vec![uni("LineageEffect.p_u002_a"), u(0), f(50.0), f(250.0), f(70.0), u(0), u(1)]));
    out.extend(u(greetings.len() as u32));
    for greeting in greetings {
        out.extend(ascf(greeting));
    }
    out.extend(u(0));
    out
}

#[test]
fn npcgrp_has_two_counted_lists_before_the_effect_and_greetings_after_the_bars_flag() {
    let body = with_footer(2, vec![npc_record(20001, &[]), npc_record(20855, &["iason_haine_greeting_1", "iason_haine_greeting_2", "iason_haine_greeting_3"])]);
    let npcs = crate::npcgrp::read_as(&body[..body.len() - FOOTER.len()], HIGH_FIVE).unwrap();
    assert_eq!(npcs.len(), 2);
    assert_eq!((npcs[0].id, npcs[1].id), (20001, 20855));
    assert_eq!(npcs[0].skills, [4416, 13]);
    assert_eq!(npcs[0].effect, "LineageEffect.p_u002_a");
    assert_eq!((npcs[0].sound_radius, npcs[0].sound_volume, npcs[0].sound_random), (50.0, 250.0, 70.0));
    assert_eq!(npcs[1].shows_bars, 1);
    assert!(crate::npcgrp::read_as(&body[..body.len() - FOOTER.len() - 1], HIGH_FIVE).is_err());
    assert!(crate::npcgrp::read_as(&body[..body.len() - FOOTER.len()], Chronicle::Interlude).is_err());
}

fn item_name_record(id: u32, name: &str, set: &[&str], effect: &str, enchant: (u32, &str)) -> Vec<u8> {
    let mut out = cat(vec![u(id), uni(name), uni(""), ascf("An arrow."), i(-1)]);
    out.extend(u(set.len() as u32));
    out.extend(array(set));
    out.extend(ascf(effect));
    for _ in 0..2 {
        out.extend(cat(vec![u(0), array(&[]), ascf("")]));
    }
    out.extend(cat(vec![u(enchant.0), ascf(enchant.1), u(2)]));
    out
}

#[test]
fn itemname_reads_three_id_lists_and_the_enchant_text() {
    let body = with_footer(
        2,
        vec![
            item_name_record(17, "Wooden Arrow", &[], "", (0, "")),
            item_name_record(23, "Wooden Breastplate", &["23", "2386", "43"], "P. Def. +2%.", (6, "When enchanted by 6.")),
        ],
    );
    let rows = crate::itemname::read_high_five(&body[..body.len() - FOOTER.len()]).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!((rows[0].0.id, rows[0].0.name.as_str(), rows[0].0.popup), (17, "Wooden Arrow", -1));
    assert!(rows[0].1.set_ids.is_empty());
    assert_eq!(rows[1].1.set_ids, "23,2386,43");
    assert_eq!(rows[1].1.set_effect, "P. Def. +2%.");
    assert_eq!((rows[1].1.enchant_level, rows[1].1.enchant_text.as_str()), (6, "When enchanted by 6."));
    let plain = crate::itemname::read_as(&body[..body.len() - FOOTER.len()], HIGH_FIVE).unwrap();
    assert_eq!(plain[1].name, "Wooden Breastplate");
    assert!(crate::itemname::read_as(&body[..body.len() - FOOTER.len() - 1], HIGH_FIVE).is_err());
}

#[test]
fn eula_has_four_texts_before_the_footer() {
    let mut file = cat(vec![ascf("one"), ascf("two"), ascf("three"), ascf("four")]);
    file.extend(ascf("SafePackage"));
    let (eula, extra) = crate::eula::read_high_five(&file).unwrap();
    assert_eq!((eula.eula.as_str(), eula.test_server.as_str()), ("one", "two"));
    assert_eq!(extra, ["three".to_string(), "four".to_string()]);
    assert_eq!(crate::eula::read_as(&file, HIGH_FIVE).unwrap(), eula);
    assert!(crate::eula::read_as(&file, Chronicle::Interlude).is_err());
    assert!(crate::eula::read_high_five(&file[..file.len() - 1]).is_err());
}

#[test]
fn castlename_has_four_extra_texts_per_record() {
    let mut body = u(2);
    for (index, name) in ["Gludio Castle", "Gludio Territory"].iter().enumerate() {
        body.extend(cat(vec![u(index as u32), u(1), u(index as u32 + 1), ascf(name), ascf("Gludio"), ascf("")]));
        for text in ["icon.a", "icon.b", "icon.c", "Guardian"] {
            body.extend(ascf(if index == 0 { "" } else { text }));
        }
    }
    body.extend(FOOTER);
    let (rows, tail) = crate::castlename::read_high_five_with_tail(&body).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!((rows[1].id, rows[1].name.as_str()), (2, "Gludio Territory"));
    assert_eq!(tail, FOOTER);
    assert_eq!(crate::castlename::read_as(&body, HIGH_FIVE).unwrap().len(), 2);
    assert!(crate::castlename::read_as(&body[..body.len() - FOOTER.len() - 1], HIGH_FIVE).is_err());
}

#[test]
fn logongrp_has_seven_slots_and_the_center_without_stage_spots() {
    let mut body = Vec::new();
    for spot in 0..8 {
        body.extend(cat(vec![f(150000.0 + spot as f32), f(-246000.0), f(-8117.0), f(-26408.0)]));
    }
    body.extend(FOOTER);
    let table = crate::logongrp::read_as(&body, HIGH_FIVE).unwrap();
    assert_eq!(table.slots[6].x, 150006.0);
    assert_eq!(table.center.x, 150007.0);
    assert_eq!(table.stage[0], crate::Spot::default());
    assert!(crate::logongrp::read_as(&body[..body.len() - 40], HIGH_FIVE).is_err());
}

#[test]
fn recipe_has_one_more_value_after_the_material_count() {
    let mut body = u(1);
    body.extend(ascf("mk_wooden_arrow"));
    for value in [1, 1666, 1, 17, 500, 30, 100, 2, 1] {
        body.extend(i(value));
    }
    for value in [1864, 4, 1869, 2] {
        body.extend(i(value));
    }
    let rows = crate::recipe::read_as(&body, HIGH_FIVE).unwrap();
    assert_eq!(rows[0].materials.len(), 2);
    assert_eq!((rows[0].materials[1].item_id, rows[0].materials[1].count), (1869, 2));
    assert_ne!(crate::recipe::read_as(&body, Chronicle::C4).map(|rows| rows[0].materials[0].item_id).ok(), Some(1864));
}

#[test]
fn skillgrp_has_two_more_values_a_label_and_a_panel_icon() {
    let mut body = u(1);
    body.extend(cat(vec![u(3), u(1), u(0), u(0), u(9), i(40), u(3), f(1.08), i(0), uni("S"), uni("3"), uni("icon.skill0003"), uni("icon.panel_2")]));
    body.extend(cat(vec![u(1), u(1), u(37), ascf("cost"), u(499), i(9), i(11), u(0), ascf("none")]));
    let rows = crate::skillgrp::read_as(&body, HIGH_FIVE).unwrap();
    assert_eq!((rows[0].id, rows[0].level, rows[0].mp_consume, rows[0].cast_range), (3, 1, 9, 40));
    assert_eq!(rows[0].icon, "icon.skill0003");
    assert_eq!((rows[0].extra_eff, rows[0].is_ench, rows[0].ench_skill_id, rows[0].hp_consume), (1, 1, 37, 499));
    assert_eq!((rows[0].rumble_self, rows[0].rumble_target), (9, 11));
    assert_eq!(rows[0].icon_type, Some(0));
    assert_eq!(rows[0].icon_group(), Some((false, 0)));
    assert!(crate::skillgrp::read_as(&body[..body.len() - 1], HIGH_FIVE).is_err());
}

#[test]
fn skillsoundgrp_has_seventeen_race_names() {
    let mut body = u(1);
    body.extend(cat(vec![i(1), i(1)]));
    for slot in 0..3 {
        for text in ["", "", ""] {
            body.extend(uni(if slot == 0 && text.is_empty() { "SkillSound4.cast" } else { text }));
        }
        for _ in 0..6 {
            body.extend(f(0.0));
        }
    }
    for group in 0..2 {
        for race in 0..crate::skillsoundgrp::RACES_HIGH_FIVE {
            body.extend(uni(if race == 16 { if group == 0 { "chrsound.f_kamael_sub" } else { "chrsound.f_kamael_throw" } } else { "" }));
        }
    }
    body.extend(cat(vec![f(250.0), f(50.0)]));
    body.extend(FOOTER);
    let (rows, tail) = crate::skillsoundgrp::read_with_tail_as(&body, HIGH_FIVE).unwrap();
    assert_eq!(rows[0].cast.len(), 17);
    assert_eq!(rows[0].cast[16], "chrsound.f_kamael_sub");
    assert_eq!(rows[0].magic[16], "chrsound.f_kamael_throw");
    assert_eq!(tail, FOOTER);
    assert!(crate::skillsoundgrp::read_as(&body[..body.len() - FOOTER.len() - 9], HIGH_FIVE).is_err());
}

fn quest_record(id: i32, state: i32, steps: i32) -> Vec<u8> {
    let mut out = cat(vec![i(1), i(id), i(state), ascf("Letters of Love"), ascf("Delivery"), ascf("Text"), vec![0], vec![0]]);
    out.extend(cat(vec![f(1.0), f(2.0), f(3.0), i(2), i(5), i(3), ascf("Roxxy"), i(1), i(1), i(1), i(30048), f(4.0), f(5.0), f(6.0), ascf("No Requirements"), ascf("Intro")]));
    out.extend(cat(vec![vec![0], vec![0], i(2), i(31), i(0), i(30), i(0)]));
    out.push(2);
    out.extend(cat(vec![i(15623), i(57)]));
    out.push(1);
    out.extend(i(5672));
    out.push(1);
    out.extend(i(steps));
    out
}

#[test]
fn questname_reads_the_class_lists_the_level_limit_and_the_reward_lists() {
    let body = with_footer(2, vec![quest_record(1, 1, 0), quest_record(1, 2, 1)]);
    let rows = crate::questname::read_high_five(&body[..body.len() - FOOTER.len()]).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!((rows[1].0.id, rows[1].0.state, rows[1].0.start_npc_id), (1, 2, 30048));
    assert_eq!(rows[0].1.extra, [2, 31, 0]);
    assert_eq!(rows[0].1.level_limit, 30);
    assert_eq!(rows[0].1.reward_items, [15623, 57]);
    assert_eq!(rows[1].1.steps, [1]);
    assert!(crate::questname::read_as(&body[..body.len() - FOOTER.len() - 1], HIGH_FIVE).is_err());
}

#[test]
fn zonename_has_one_more_value_after_the_map_texture() {
    let mut body = u(2);
    for (id, texture, extra) in [(1u32, "", 0u32), (20, "map_gludio_castle_town", 1)] {
        body.extend(cat(vec![u(id), u(255), u(19), u(21), f(30000.0), f(-30000.0), ascf("Town of Gludio")]));
        body.extend(cat(vec![i(756), i(1932), i(-19968), i(117504), i(1024), i(1024), f(0.05), ascf(texture), u(extra)]));
    }
    body.extend(ascf("SafePackage"));
    let rows = crate::zonename::read_high_five(&body).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[1].1.texture, "map_gludio_castle_town");
    assert_eq!(crate::zonename::read_as(&body, HIGH_FIVE).unwrap()[1].id, 20);
    assert!(crate::zonename::read_as(&body[..body.len() - 1], HIGH_FIVE).is_err());
    assert!(crate::zonename::read_as(&body, Chronicle::Interlude).is_err());
}

fn char_row(name: &str, race: u32, class_id: u32, sex: u32) -> Vec<u8> {
    let mut out = Vec::new();
    for slot in 0..300 {
        out.extend(uni(if slot == 6 { "Fighter.MFighter_m001_m00_bh" } else { "" }));
    }
    out.extend(array(&["Fighter.MFighter_m000_f"; 3]));
    out.extend(array(&["MFighter.MFighter_m000_t00_f", "MFighter.MFighter_m000_t01_f", "MFighter.MFighter_m000_t02_f"]));
    for _ in 0..90 {
        out.extend(uni(""));
    }
    for (index, mesh) in ["g", "u", "l", "b"].iter().enumerate() {
        out.extend(array(&[&format!("Fighter.MFighter_m001_{mesh}")]));
        out.extend(array(&[&format!("MFighter.MFighter_m001_t02_{mesh}")]));
        let ex: &[&str] = if index == 1 { &["Kamael.w_ad00", "Kamael.l_ad00"] } else { &[] };
        out.extend(array(ex));
        out.extend(array(ex));
        out.extend(vec![0u8; 2 + 2 * ex.len()]);
    }
    out.extend(vec![0u8; 90]);
    out.extend(cat(vec![uni("LineageEffect.p_u002_a"), u(12), u(3), u(5), u(3)]));
    out.extend(cat(vec![uni("a1"), uni("a2"), uni("a3")]));
    for index in 0..5 {
        out.extend(uni(&format!("d{index}")));
    }
    out.extend(cat(vec![uni("m1"), uni("m2"), uni("m3")]));
    for group in 0..crate::chargrp::VOICE_GROUPS_HIGH_FIVE {
        out.extend(array(if group % 2 == 0 { &["ChrSound.x"] } else { &[] }));
    }
    out.extend(u(16));
    out.extend(ascf(name));
    out.extend(cat(vec![u(race), u(class_id), u(sex)]));
    out.extend(array(&[]));
    out.extend(array(&["FFighter.extra"]));
    out
}

#[test]
fn chargrp_rows_have_three_hundred_hair_slots_four_body_parts_and_eleven_voice_groups() {
    let mut body = char_row("MFighter", 0, 0, 0);
    body.extend(char_row("MKamael", 0, 123, 0));
    body.extend(FOOTER);
    let rows = crate::chargrp::read_high_five(&body).unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].hair.len(), 300);
    assert_eq!(rows[0].hair(0, 1, 2), Some("Fighter.MFighter_m001_m00_bh"));
    assert_eq!(rows[0].hair(0, 1, 3), None);
    assert_eq!(rows[0].hair(5, 0, 0), None);
    assert_eq!(rows[0].face_meshes.len(), 3);
    assert_eq!(rows[0].body_parts[1].ex_meshes, ["Kamael.w_ad00", "Kamael.l_ad00"]);
    assert_eq!(rows[0].body_parts[3].meshes, ["Fighter.MFighter_m001_b"]);
    assert_eq!((rows[0].walk_anim_frame, rows[0].attack_sounds.len(), rows[0].defense_sounds.len(), rows[0].damage_sounds.len()), (12, 3, 5, 3));
    assert_eq!(rows[0].voice_sounds[10], ["ChrSound.x"]);
    assert_eq!(rows[0].voice_extra, 16);
    assert_eq!((rows[1].name.as_str(), rows[1].class_id), ("MKamael", 123));
    assert_eq!(rows[1].extra_names[1], ["FFighter.extra"]);
    assert!(crate::chargrp::read_high_five(&body[..body.len() - FOOTER.len() - 1]).is_err());
}

#[test]
fn chargrp_row_choice_uses_the_class_sex_and_race_stored_in_the_record() {
    let mut body = Vec::new();
    for (name, race, class_id, sex) in [
        ("MFighter", 0, 0, 0),
        ("FFighter", 0, 0, 1),
        ("MElf", 1, 18, 0),
        ("FElf", 1, 18, 1),
        ("MMagic", 0, 10, 0),
        ("FMagic", 0, 10, 1),
        ("MKamael", 0, 123, 0),
        ("FKamael", 0, 124, 1),
        ("", u32::MAX, u32::MAX, u32::MAX),
    ] {
        body.extend(char_row(name, race, class_id, sex));
    }
    body.extend(FOOTER);
    let rows = crate::chargrp::read_high_five(&body).unwrap();
    let pick = |race, sex, class_id| crate::chargrp::index_for_high_five(&rows, race, sex, class_id);
    assert_eq!(pick(0, 0, 0), Some(0));
    assert_eq!(pick(0, 1, 10), Some(5));
    assert_eq!(pick(5, 0, 123), Some(6));
    assert_eq!(pick(5, 1, 124), Some(7));
    assert_eq!(pick(1, 1, 25), Some(3));
    assert_eq!(pick(1, 0, 18), Some(2));
    assert_eq!(pick(0, 0, 88), None);
    assert_eq!(pick(9, 0, 1), None);
    assert_eq!(pick(-1, 0, 0), None);
}
