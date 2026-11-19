use crate::{DatReader, Error};

pub const ROWS: usize = 15;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharTemplate {
    pub face_icon: String,
    pub hair_meshes: Vec<String>,
    pub hair_textures: Vec<String>,
    pub face_meshes: Vec<String>,
    pub face_textures: Vec<String>,
    pub body_meshes: [String; 4],
    pub body_textures: [String; 4],
    pub damage_effect: String,
    pub walk_anim_frame: u32,
    pub attack_sounds: Vec<String>,
    pub defense_sounds: Vec<String>,
    pub damage_sounds: Vec<String>,
    pub voice_sounds: [Vec<String>; VOICE_GROUPS],
}

pub const VOICE_GROUPS: usize = 8;

pub const HAIR_BLOCKS_HIGH_FIVE: usize = 5;
pub const HAIR_STYLES_HIGH_FIVE: usize = 15;
pub const HAIR_SLOTS_HIGH_FIVE: usize = 4;
pub const VOICE_GROUPS_HIGH_FIVE: usize = 11;
pub const BODY_PARTS_HIGH_FIVE: usize = 4;
const UNUSED_NAMES_HIGH_FIVE: usize = 90;
const UNUSED_BYTES_HIGH_FIVE: usize = 90;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BodyPartHighFive {
    pub meshes: Vec<String>,
    pub textures: Vec<String>,
    pub ex_meshes: Vec<String>,
    pub ex_textures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharRowHighFive {
    pub hair: Vec<String>,
    pub face_meshes: Vec<String>,
    pub face_textures: Vec<String>,
    pub body_parts: [BodyPartHighFive; BODY_PARTS_HIGH_FIVE],
    pub damage_effect: String,
    pub walk_anim_frame: u32,
    pub attack_sounds: Vec<String>,
    pub defense_sounds: Vec<String>,
    pub damage_sounds: Vec<String>,
    pub voice_sounds: [Vec<String>; VOICE_GROUPS_HIGH_FIVE],
    pub voice_extra: u32,
    pub name: String,
    pub race: u32,
    pub class_id: u32,
    pub sex: u32,
    pub extra_names: [Vec<String>; 2],
}

impl CharRowHighFive {
    pub fn hair(&self, block: usize, style: usize, slot: usize) -> Option<&str> {
        if block >= HAIR_BLOCKS_HIGH_FIVE || style >= HAIR_STYLES_HIGH_FIVE || slot >= HAIR_SLOTS_HIGH_FIVE {
            return None;
        }
        let index = (block * HAIR_STYLES_HIGH_FIVE + style) * HAIR_SLOTS_HIGH_FIVE + slot;
        self.hair.get(index).map(String::as_str).filter(|name| !name.is_empty())
    }
}

pub fn read_high_five(body: &[u8]) -> Result<Vec<CharRowHighFive>, Error> {
    let mut reader = DatReader::new(body);
    let mut rows = Vec::new();
    while reader.remaining() > FOOTER_LEN {
        rows.push(record_high_five(&mut reader)?);
    }
    Ok(rows)
}

const FOOTER_LEN: usize = 13;

fn record_high_five(r: &mut DatReader<'_>) -> Result<CharRowHighFive, Error> {
    let hair = (0..HAIR_BLOCKS_HIGH_FIVE * HAIR_STYLES_HIGH_FIVE * HAIR_SLOTS_HIGH_FIVE).map(|_| r.unicode()).collect::<Result<Vec<_>, _>>()?;
    let face_meshes = r.unicode_array()?;
    let face_textures = r.unicode_array()?;
    for _ in 0..UNUSED_NAMES_HIGH_FIVE {
        r.unicode()?;
    }
    let mut body_parts: [BodyPartHighFive; BODY_PARTS_HIGH_FIVE] = Default::default();
    for part in &mut body_parts {
        part.meshes = r.unicode_array()?;
        part.textures = r.unicode_array()?;
        part.ex_meshes = r.unicode_array()?;
        part.ex_textures = r.unicode_array()?;
        r.skip(2 + 2 * part.ex_meshes.len())?;
    }
    r.skip(UNUSED_BYTES_HIGH_FIVE)?;
    let damage_effect = r.unicode()?;
    let walk_anim_frame = r.u32()?;
    let [attacks, defenses, damages] = [r.u32()?, r.u32()?, r.u32()?];
    let attack_sounds = strings(r, attacks)?;
    let defense_sounds = strings(r, defenses)?;
    let damage_sounds = strings(r, damages)?;
    let mut voice_sounds: [Vec<String>; VOICE_GROUPS_HIGH_FIVE] = Default::default();
    for group in &mut voice_sounds {
        *group = r.unicode_array()?;
    }
    let voice_extra = r.u32()?;
    let name = r.ascf()?;
    let race = r.u32()?;
    let class_id = r.u32()?;
    let sex = r.u32()?;
    let extra_names = [r.unicode_array()?, r.unicode_array()?];
    Ok(CharRowHighFive { hair, face_meshes, face_textures, body_parts, damage_effect, walk_anim_frame, attack_sounds, defense_sounds, damage_sounds, voice_sounds, voice_extra, name, race, class_id, sex, extra_names })
}

pub fn index_for_high_five(rows: &[CharRowHighFive], race: i32, sex: i32, class_id: i32) -> Option<usize> {
    let (race, sex, class_id) = (u32::try_from(race).ok()?, u32::try_from(sex).ok()?, u32::try_from(class_id).ok()?);
    if let Some(index) = rows.iter().position(|row| row.class_id == class_id && row.sex == sex) {
        return Some(index);
    }
    let mut same_race = rows.iter().enumerate().filter(|(_, row)| row.race == race && row.sex == sex);
    let first = same_race.next()?;
    same_race.next().is_none().then_some(first.0)
}

pub fn read(body: &[u8]) -> Result<Vec<CharTemplate>, Error> {
    let mut reader = DatReader::new(body);
    let mut rows = Vec::with_capacity(ROWS);
    while rows.len() < ROWS && reader.remaining() > 0 {
        rows.push(record(&mut reader)?);
    }
    Ok(rows)
}

fn record(r: &mut DatReader<'_>) -> Result<CharTemplate, Error> {
    let face_icon = r.unicode()?;
    let [hairs, hair_skins, faces, face_skins] = [r.u32()?, r.u32()?, r.u32()?, r.u32()?];
    let hair_meshes = strings(r, hairs)?;
    let hair_textures = strings(r, hair_skins)?;
    let face_meshes = strings(r, faces)?;
    let face_textures = strings(r, face_skins)?;
    let body_meshes = fixed(r)?;
    let body_textures = fixed(r)?;
    let damage_effect = r.unicode()?;
    let walk_anim_frame = r.u32()?;
    let [attacks, defenses, damages] = [r.u32()?, r.u32()?, r.u32()?];
    let attack_sounds = strings(r, attacks)?;
    let defense_sounds = strings(r, defenses)?;
    let damage_sounds = strings(r, damages)?;
    let mut voice_sounds: [Vec<String>; VOICE_GROUPS] = Default::default();
    for group in &mut voice_sounds {
        *group = r.unicode_array()?;
    }
    Ok(CharTemplate {
        face_icon,
        hair_meshes,
        hair_textures,
        face_meshes,
        face_textures,
        body_meshes,
        body_textures,
        damage_effect,
        walk_anim_frame,
        attack_sounds,
        defense_sounds,
        damage_sounds,
        voice_sounds,
    })
}

fn strings(r: &mut DatReader<'_>, count: u32) -> Result<Vec<String>, Error> {
    (0..count).map(|_| r.unicode()).collect()
}

fn fixed(r: &mut DatReader<'_>) -> Result<[String; 4], Error> {
    Ok([r.unicode()?, r.unicode()?, r.unicode()?, r.unicode()?])
}

pub fn index_for(race: i32, sex: i32, class_id: i32) -> Option<usize> {
    let pair = match (race, class_id) {
        (0, 0..=9 | 88..=93) => 0,
        (0, 10..=17 | 94..=98) => 8,
        (1, _) => 6,
        (2, _) => 2,
        (3, 44..=48 | 113 | 114) => 10,
        (3, 49..=52 | 115 | 116) => 12,
        (4, _) => 4,
        _ => return None,
    };
    Some(pair + usize::from(sex != 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chargrp_rows_and_index() {
        let rows = read(include_bytes!("../tests/fixtures/chargrp-head.bin")).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].face_icon, "SEK.cbui29");
        assert_eq!(rows[0].hair_meshes, ["DarkElf.FDarkElf_m000_m00_ah"]);
        assert_eq!(rows[0].hair_textures, ["FdarkElf.FDarkElf_m000_t00_m00_ah"]);
        assert_eq!(rows[0].face_meshes, ["Fighter.MFighter_m000_f"]);
        assert_eq!(rows[0].face_textures.len(), 3);
        assert_eq!(rows[0].face_textures[1], "MFighter.MFighter_m000_t01_f");
        assert_eq!(rows[0].body_meshes[0], "fighter.MFighter_m000_u");
        assert_eq!(rows[0].body_meshes[3], "fighter.MFighter_m000_b");
        assert_eq!(rows[0].body_textures[0], "MFighter.MFighter_m000_t1000_u");
        assert_eq!(rows[0].body_textures[3], "MFighter.MFighter_m000_t1000_b");
        assert_eq!(rows[0].walk_anim_frame, 12);
        assert_eq!(rows[0].damage_effect, "LineageEffect.p_u002_a");
        assert_eq!(rows[1].damage_effect, "LineageEffect.p_u002_a");
        assert_eq!(rows[1].face_icon, "SEK.cbui22");
        assert_eq!(rows[1].face_meshes, ["Fighter.FFighter_m000_f"]);
        assert_eq!(rows[1].body_textures[2], "FFighter.FFighter_m000_t1000_g");
    }

    #[test]
    fn index_follows_get_mesh_type() {
        assert_eq!(index_for(0, 0, 0), Some(0));
        assert_eq!(index_for(0, 1, 9), Some(1));
        assert_eq!(index_for(0, 0, 88), Some(0));
        assert_eq!(index_for(0, 1, 93), Some(1));
        assert_eq!(index_for(0, 0, 10), Some(8));
        assert_eq!(index_for(0, 1, 17), Some(9));
        assert_eq!(index_for(0, 0, 94), Some(8));
        assert_eq!(index_for(0, 1, 98), Some(9));
        assert_eq!(index_for(0, 0, 18), None);
        assert_eq!(index_for(0, 0, 99), None);
        assert_eq!(index_for(1, 0, 18), Some(6));
        assert_eq!(index_for(1, 1, 109), Some(7));
        assert_eq!(index_for(2, 0, 31), Some(2));
        assert_eq!(index_for(2, 1, 43), Some(3));
        assert_eq!(index_for(3, 0, 44), Some(10));
        assert_eq!(index_for(3, 1, 48), Some(11));
        assert_eq!(index_for(3, 0, 113), Some(10));
        assert_eq!(index_for(3, 1, 114), Some(11));
        assert_eq!(index_for(3, 0, 49), Some(12));
        assert_eq!(index_for(3, 1, 52), Some(13));
        assert_eq!(index_for(3, 0, 115), Some(12));
        assert_eq!(index_for(3, 1, 116), Some(13));
        assert_eq!(index_for(3, 0, 53), None);
        assert_eq!(index_for(4, 0, 53), Some(4));
        assert_eq!(index_for(4, 2, 57), Some(5));
        assert_eq!(index_for(5, 0, 0), None);
        assert_eq!(index_for(-1, 0, 0), None);
    }

    #[test]
    fn truncated_row_is_an_error() {
        let body = include_bytes!("../tests/fixtures/chargrp-head.bin");
        assert!(read(&body[..body.len() - 3]).is_err());
        assert!(read(&[]).unwrap().is_empty());
    }
}
