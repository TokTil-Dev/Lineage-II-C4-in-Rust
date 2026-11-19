use std::path::PathBuf;

fn system_file(root_var: &str, name: &str) -> Option<Vec<u8>> {
    let Some(root) = std::env::var_os(root_var).map(PathBuf::from) else {
        eprintln!("{root_var} not set; skipping");
        return None;
    };
    let path = root.join("system").join(name);
    let raw = std::fs::read(&path).unwrap();
    Some(l2_crypto::decrypt(&raw, &path).unwrap().into_owned())
}

#[test]
#[ignore]
fn every_high_five_skill_has_an_icon_type_inside_the_active_or_passive_group_ranges() {
    let Some(bytes) = system_file("KILLER_CLIENT_DIR", "skillgrp.dat") else { return };
    let skills = l2_dat::skillgrp::read_as(&bytes, l2_dat::Chronicle::HighFive).unwrap();
    assert!(!skills.is_empty());
    let mut active = 0usize;
    let mut passive = 0usize;
    for skill in &skills {
        let icon_type = skill.icon_type.expect("crônica High Five traz IconType");
        let (is_passive, group) = skill
            .icon_group()
            .unwrap_or_else(|| panic!("skill {} nível {} com IconType {icon_type} fora das faixas do MagicSkillWnd.uc", skill.id, skill.level));
        if is_passive {
            assert!(group <= 5);
            passive += 1;
        } else {
            assert!(group <= 7);
            active += 1;
        }
    }
    assert_eq!(active + passive, skills.len());
    assert!(active > 0 && passive > 0);
}

#[test]
#[ignore]
fn classic_skillgrp_has_no_icon_type_column() {
    let Some(bytes) = system_file("KILLER_BASE_CLIENT_DIR", "skillgrp.dat") else { return };
    let skills = l2_dat::skillgrp::read_as(&bytes, l2_dat::Chronicle::C4).unwrap();
    assert!(!skills.is_empty());
    assert!(skills.iter().all(|skill| skill.icon_type.is_none()));
}
