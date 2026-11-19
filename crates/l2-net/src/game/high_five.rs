use super::packets::{CharSelectEntry, NewCharacter};
use crate::login::Authorization;
use crate::wire::{Reader, Result, Writer};

pub const OP_PROTOCOL_VERSION: u8 = 0x0e;
pub const OP_AUTH_LOGIN: u8 = 0x2b;
pub const OP_CHARACTER_SELECTED: u8 = 0x12;
pub const OP_CHARACTER_CREATE: u8 = 0x0c;
pub const OP_CHARACTER_DELETE: u8 = 0x0d;
pub const OP_NEW_CHARACTER: u8 = 0x13;
pub const OP_CHARACTER_RESTORE: u8 = 0x7b;
pub const OP_LOGOUT: u8 = 0x00;
pub const OP_EXTENDED: u8 = 0xd0;
pub const EX_REQUEST_CHARACTER_SELECTION: u16 = 0x36;

pub const OP_CHAR_SELECT_INFO: u8 = 0x09;
pub const OP_AUTH_LOGIN_FAIL: u8 = 0x0a;
pub const OP_CHAR_SELECTED: u8 = 0x0b;
pub const OP_CHAR_TEMPLATES: u8 = 0x0d;
pub const OP_CHAR_CREATE_OK: u8 = 0x0f;
pub const OP_CHAR_CREATE_FAIL: u8 = 0x10;
pub const OP_CHAR_DELETE_OK: u8 = 0x1d;
pub const OP_CHAR_DELETE_FAIL: u8 = 0x1e;
pub const OP_SERVER_CLOSE: u8 = 0x20;

pub const PROTOCOL_VERSION: i32 = 273;
pub const ACCEPTED_PROTOCOL_VERSIONS: [i32; 4] = [267, 268, 271, 273];
pub const LANGUAGE_ENGLISH: i32 = 1;

pub const PAPERDOLL_SLOTS: usize = 26;
pub const CHAR_SELECT_PAPERDOLL_C4_SLOTS: usize = 16;

pub fn protocol_version(version: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_PROTOCOL_VERSION);
    w.write_d(version);
    w.into_bytes()
}

pub fn auth_login(name: &str, auth: &Authorization, language: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_AUTH_LOGIN);
    w.write_s(name);
    w.write_d(auth.play_key[1] as i32);
    w.write_d(auth.play_key[0] as i32);
    w.write_d(auth.login_key[0] as i32);
    w.write_d(auth.login_key[1] as i32);
    w.write_d(language);
    w.into_bytes()
}

pub fn character_selected(slot: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_CHARACTER_SELECTED);
    w.write_d(slot);
    w.write_h(0);
    w.write_d(0);
    w.write_d(0);
    w.write_d(0);
    w.into_bytes()
}

pub fn request_character_selection() -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_EXTENDED);
    w.write_h(EX_REQUEST_CHARACTER_SELECTION);
    w.into_bytes()
}

pub fn logout() -> Vec<u8> {
    vec![OP_LOGOUT]
}

pub fn new_character() -> Vec<u8> {
    vec![OP_NEW_CHARACTER]
}

pub fn character_create(c: &NewCharacter) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_CHARACTER_CREATE);
    w.write_s(&c.name);
    for v in [c.race, c.sex, c.class_id, c.int_, c.str_, c.con, c.men, c.dex, c.wit, c.hair_style, c.hair_color, c.face]
    {
        w.write_d(v);
    }
    w.into_bytes()
}

pub fn character_delete(slot: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_CHARACTER_DELETE);
    w.write_d(slot);
    w.into_bytes()
}

pub fn character_restore(slot: i32) -> Vec<u8> {
    let mut w = Writer::new();
    w.write_c(OP_CHARACTER_RESTORE);
    w.write_d(slot);
    w.into_bytes()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PetSummary {
    pub id: i32,
    pub level: i32,
    pub max_food: i32,
    pub food: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CharSelectEntryHighFive {
    pub entry: CharSelectEntry,
    pub session_key: i32,
    pub exp_percent: f64,
    pub pk_kills: i32,
    pub pvp_kills: i32,
    pub paperdoll_items: [i32; PAPERDOLL_SLOTS],
    pub augmentation: i32,
    pub pet: PetSummary,
    pub pet_max_hp: f64,
    pub pet_max_mp: f64,
    pub vitality: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CharSelectionInfoHighFive {
    pub max_slots: i32,
    pub characters: Vec<CharSelectEntryHighFive>,
}

const CHARACTER_FIXED_BYTES: usize = 209 + PAPERDOLL_SLOTS * 4;

pub fn read_char_select_info(body: &[u8]) -> Result<CharSelectionInfoHighFive> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let declared = r.read_d()?;
    let max_slots = r.read_d()?;
    r.read_c()?;
    let count = usize::try_from(declared).unwrap_or(0);
    if count.saturating_mul(CHARACTER_FIXED_BYTES) > r.remaining() {
        return Err(crate::wire::WireError::Short);
    }
    let mut characters = Vec::with_capacity(count);
    for _ in 0..count {
        let name = r.read_s()?;
        let char_id = r.read_d()?;
        let login = r.read_s()?;
        let session_key = r.read_d()?;
        let clan_id = r.read_d()?;
        r.read_d()?;
        let sex = r.read_d()?;
        let race = r.read_d()?;
        let base_class_id = r.read_d()?;
        let active = r.read_d()?;
        let x = r.read_d()?;
        let y = r.read_d()?;
        let z = r.read_d()?;
        let hp = r.read_f()?;
        let mp = r.read_f()?;
        let sp = r.read_d()?;
        let exp = r.read_q()?;
        let exp_percent = r.read_f()?;
        let level = r.read_d()?;
        let karma = r.read_d()?;
        let pk_kills = r.read_d()?;
        let pvp_kills = r.read_d()?;
        for _ in 0..7 {
            r.read_d()?;
        }
        let mut paperdoll_items = [0i32; PAPERDOLL_SLOTS];
        for slot in &mut paperdoll_items {
            *slot = r.read_d()?;
        }
        let hair_style = r.read_d()?;
        let hair_color = r.read_d()?;
        let face = r.read_d()?;
        let max_hp = r.read_f()?;
        let max_mp = r.read_f()?;
        let delete_days = r.read_d()?;
        let class_id = r.read_d()?;
        let last_used = r.read_d()?;
        let enchant_effect = r.read_c()?;
        let augmentation = r.read_d()?;
        r.read_d()?;
        let pet = PetSummary { id: r.read_d()?, level: r.read_d()?, max_food: r.read_d()?, food: r.read_d()? };
        let pet_max_hp = r.read_f()?;
        let pet_max_mp = r.read_f()?;
        let vitality = r.read_d()?;
        let mut visible = [0i32; CHAR_SELECT_PAPERDOLL_C4_SLOTS];
        visible.copy_from_slice(&paperdoll_items[..CHAR_SELECT_PAPERDOLL_C4_SLOTS]);
        let entry = CharSelectEntry {
            name,
            char_id,
            login,
            clan_id,
            sex,
            race,
            base_class_id,
            active,
            x,
            y,
            z,
            hp,
            mp,
            sp,
            exp,
            level,
            karma,
            paperdoll_objects: [0; CHAR_SELECT_PAPERDOLL_C4_SLOTS],
            paperdoll_items: visible,
            hair_style,
            hair_color,
            face,
            max_hp,
            max_mp,
            delete_days,
            class_id,
            last_used,
            enchant_effect,
        };
        characters.push(CharSelectEntryHighFive {
            entry,
            session_key,
            exp_percent,
            pk_kills,
            pvp_kills,
            paperdoll_items,
            augmentation,
            pet,
            pet_max_hp,
            pet_max_mp,
            vitality,
        });
    }
    Ok(CharSelectionInfoHighFive { max_slots, characters })
}

#[derive(Debug, Clone, PartialEq)]
pub struct CharacterSelectedHighFive {
    pub name: String,
    pub char_id: i32,
    pub title: String,
    pub session_id: i32,
    pub clan_id: i32,
    pub sex: i32,
    pub race: i32,
    pub class_id: i32,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub hp: f64,
    pub mp: f64,
    pub sp: i32,
    pub exp: i64,
    pub level: i32,
    pub karma: i32,
    pub pk_kills: i32,
    pub attributes: [i32; 6],
    pub game_minutes: i32,
    pub base_class_id: i32,
}

pub fn read_character_selected(body: &[u8]) -> Result<CharacterSelectedHighFive> {
    let mut r = Reader::new(body);
    r.read_c()?;
    let name = r.read_s()?;
    let char_id = r.read_d()?;
    let title = r.read_s()?;
    let session_id = r.read_d()?;
    let clan_id = r.read_d()?;
    r.read_d()?;
    let sex = r.read_d()?;
    let race = r.read_d()?;
    let class_id = r.read_d()?;
    r.read_d()?;
    let x = r.read_d()?;
    let y = r.read_d()?;
    let z = r.read_d()?;
    let hp = r.read_f()?;
    let mp = r.read_f()?;
    let sp = r.read_d()?;
    let exp = r.read_q()?;
    let level = r.read_d()?;
    let karma = r.read_d()?;
    let pk_kills = r.read_d()?;
    let mut attributes = [0i32; 6];
    for value in &mut attributes {
        *value = r.read_d()?;
    }
    let game_minutes = r.read_d()?;
    r.read_d()?;
    let base_class_id = r.read_d()?;
    r.read_b(80)?;
    r.read_d()?;
    Ok(CharacterSelectedHighFive {
        name,
        char_id,
        title,
        session_id,
        clan_id,
        sex,
        race,
        class_id,
        x,
        y,
        z,
        hp,
        mp,
        sp,
        exp,
        level,
        karma,
        pk_kills,
        attributes,
        game_minutes,
        base_class_id,
    })
}

pub fn key_packet_is_protocol_refusal(body: &[u8]) -> bool {
    body.len() == super::crypt::HIGH_FIVE_KEY_PACKET_LEN
        && body[0] == super::crypt::OP_KEY_PACKET_HIGH_FIVE
        && body[1] == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn character_body(name: &str, login: &str, id: i32) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_s(name);
        w.write_d(id);
        w.write_s(login);
        for v in [0x1234, 7, 0, 0, 3, 1] {
            w.write_d(v);
        }
        w.write_d(1);
        for v in [-71000, 258000, -3000] {
            w.write_d(v);
        }
        w.write_f(500.5);
        w.write_f(250.25);
        w.write_d(1234);
        w.write_q(5_000_000_000);
        w.write_f(33.5);
        for v in [40, -9, 2, 5] {
            w.write_d(v);
        }
        for _ in 0..7 {
            w.write_d(0);
        }
        for slot in 0..PAPERDOLL_SLOTS as i32 {
            w.write_d(1000 + slot);
        }
        for v in [2, 1, 0] {
            w.write_d(v);
        }
        w.write_f(900.0);
        w.write_f(400.0);
        w.write_d(86400);
        w.write_d(88);
        w.write_d(1);
        w.write_c(16);
        w.write_d(77);
        w.write_d(0);
        for v in [11, 12, 13, 14] {
            w.write_d(v);
        }
        w.write_f(1.5);
        w.write_f(2.5);
        w.write_d(20000);
        w.into_bytes()
    }

    fn selection(characters: &[(&str, i32)]) -> Vec<u8> {
        let mut w = Writer::new();
        w.write_c(OP_CHAR_SELECT_INFO);
        w.write_d(characters.len() as i32);
        w.write_d(7);
        w.write_c(0);
        for (name, id) in characters {
            w.write_b(&character_body(name, "conta", *id));
        }
        w.into_bytes()
    }

    #[test]
    fn character_record_has_the_size_the_oracle_declares() {
        let body = character_body("Luz", "conta", 1);
        let strings = (("Luz".len() + 1) + ("conta".len() + 1)) * 2;
        assert_eq!(body.len(), strings + CHARACTER_FIXED_BYTES);
    }

    #[test]
    fn char_select_info_reads_every_field_and_ends_exactly() {
        let body = selection(&[("Luz", 268_480_001), ("Sombra", 268_480_002)]);
        let info = read_char_select_info(&body).unwrap();
        assert_eq!(info.max_slots, 7);
        assert_eq!(info.characters.len(), 2);
        let first = &info.characters[0];
        assert_eq!(first.session_key, 0x1234);
        assert_eq!(first.exp_percent, 33.5);
        assert_eq!((first.pk_kills, first.pvp_kills), (2, 5));
        assert_eq!(first.augmentation, 77);
        assert_eq!(first.pet, PetSummary { id: 11, level: 12, max_food: 13, food: 14 });
        assert_eq!((first.pet_max_hp, first.pet_max_mp), (1.5, 2.5));
        assert_eq!(first.vitality, 20000);
        assert_eq!(first.paperdoll_items[25], 1025);
        let entry = &first.entry;
        assert_eq!(entry.name, "Luz");
        assert_eq!(entry.login, "conta");
        assert_eq!(entry.char_id, 268_480_001);
        assert_eq!((entry.sex, entry.race, entry.base_class_id, entry.active), (0, 3, 1, 1));
        assert_eq!((entry.x, entry.y, entry.z), (-71000, 258000, -3000));
        assert_eq!((entry.hp, entry.mp, entry.sp), (500.5, 250.25, 1234));
        assert_eq!(entry.exp, 5_000_000_000);
        assert_eq!((entry.level, entry.karma), (40, -9));
        assert_eq!(entry.paperdoll_items[0], 1000);
        assert_eq!(entry.paperdoll_items[15], 1015);
        assert_eq!(entry.paperdoll_objects, [0; 16]);
        assert_eq!((entry.hair_style, entry.hair_color, entry.face), (2, 1, 0));
        assert_eq!((entry.max_hp, entry.max_mp, entry.delete_days), (900.0, 400.0, 86400));
        assert_eq!((entry.class_id, entry.last_used, entry.enchant_effect), (88, 1, 16));
        assert_eq!(info.characters[1].entry.name, "Sombra");
    }

    #[test]
    fn char_select_info_rejects_truncation_and_absurd_counts() {
        let body = selection(&[("Luz", 1)]);
        assert!(read_char_select_info(&body[..body.len() - 1]).is_err());
        let mut empty = Writer::new();
        empty.write_c(OP_CHAR_SELECT_INFO);
        empty.write_d(0);
        empty.write_d(7);
        empty.write_c(0);
        assert!(read_char_select_info(&empty.into_bytes()).unwrap().characters.is_empty());
        let mut huge = Writer::new();
        huge.write_c(OP_CHAR_SELECT_INFO);
        huge.write_d(i32::MAX);
        huge.write_d(7);
        huge.write_c(0);
        assert!(read_char_select_info(&huge.into_bytes()).is_err());
    }

    #[test]
    fn character_selected_reads_the_oracle_layout() {
        let mut w = Writer::new();
        w.write_c(OP_CHAR_SELECTED);
        w.write_s("Luz");
        w.write_d(5);
        w.write_s("Titulo");
        for v in [0x1234, 9, 0, 1, 3, 4, 1, 10, 20, 30] {
            w.write_d(v);
        }
        w.write_f(100.0);
        w.write_f(50.0);
        w.write_d(99);
        w.write_q(1 << 40);
        for v in [20, 0, 3, 10, 11, 12, 13, 14, 15, 600, 0, 4] {
            w.write_d(v);
        }
        w.write_b(&[0; 80]);
        w.write_d(0);
        let body = w.into_bytes();
        let selected = read_character_selected(&body).unwrap();
        assert_eq!((selected.name.as_str(), selected.title.as_str(), selected.char_id), ("Luz", "Titulo", 5));
        assert_eq!((selected.sex, selected.race, selected.class_id), (1, 3, 4));
        assert_eq!((selected.x, selected.y, selected.z), (10, 20, 30));
        assert_eq!((selected.sp, selected.exp, selected.level), (99, 1 << 40, 20));
        assert_eq!(selected.attributes, [10, 11, 12, 13, 14, 15]);
        assert_eq!((selected.game_minutes, selected.base_class_id), (600, 4));
        assert!(read_character_selected(&body[..body.len() - 1]).is_err());
    }

    #[test]
    fn client_packets_use_the_high_five_opcodes() {
        assert_eq!(protocol_version(PROTOCOL_VERSION), [0x0e, 0x11, 0x01, 0, 0]);
        assert!(ACCEPTED_PROTOCOL_VERSIONS.contains(&PROTOCOL_VERSION));
        let auth = Authorization {
            server: crate::login::GameServer {
                id: 1,
                ip: [127, 0, 0, 1],
                port: 7777,
                players: 0,
                max_players: 0,
                online: true,
                extra: Default::default(),
            },
            login_key: [3, 4],
            play_key: [1, 2],
        };
        let body = auth_login("ab", &auth, LANGUAGE_ENGLISH);
        assert_eq!(body[0], 0x2b);
        assert_eq!(&body[1..7], &[b'a', 0, b'b', 0, 0, 0]);
        let numbers: Vec<i32> = body[7..].chunks(4).map(|c| i32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
        assert_eq!(numbers, [2, 1, 3, 4, 1]);
        assert_eq!(character_selected(2), [0x12, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(request_character_selection(), [0xd0, 0x36, 0x00]);
        assert_eq!(logout(), [0x00]);
        assert_eq!(new_character(), [0x13]);
        assert_eq!(character_delete(3), [0x0d, 3, 0, 0, 0]);
        assert_eq!(character_restore(3), [0x7b, 3, 0, 0, 0]);
        let create = NewCharacter {
            name: "A".into(),
            race: 1,
            sex: 0,
            class_id: 18,
            int_: 2,
            str_: 3,
            con: 4,
            men: 5,
            dex: 6,
            wit: 7,
            hair_style: 1,
            hair_color: 2,
            face: 0,
        };
        let body = character_create(&create);
        assert_eq!(body[0], 0x0c);
        assert_eq!(body.len(), 1 + 4 + 12 * 4);
    }

    #[test]
    fn protocol_refusal_key_packet_is_recognised() {
        let mut body = vec![super::super::crypt::OP_KEY_PACKET_HIGH_FIVE, 0];
        body.extend_from_slice(&[0; 21]);
        assert!(key_packet_is_protocol_refusal(&body));
        body[1] = 1;
        assert!(!key_packet_is_protocol_refusal(&body));
    }
}
