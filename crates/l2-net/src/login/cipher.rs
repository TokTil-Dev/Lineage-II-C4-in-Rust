use blowfish::BlowfishLE;
use cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use thiserror::Error;

const C4_KEY: [u8; 21] = [
    0x5f, 0x3b, 0x35, 0x2e, 0x5d, 0x39, 0x34, 0x2d, 0x33, 0x31, 0x3d, 0x3d, 0x2d, 0x25, 0x78, 0x54, 0x21, 0x5e, 0x5b,
    0x24, 0x00,
];

const INIT_KEY: [u8; 16] = [0x6b, 0x60, 0xcb, 0x5b, 0x82, 0xce, 0x90, 0xb1, 0xcc, 0x2b, 0x6c, 0x55, 0x6c, 0x6c, 0x6c, 0x6c];

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CipherError {
    #[error("tamanho não é múltiplo de 8")]
    Length,
    #[error("checksum inválido")]
    Checksum,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Scheme {
    C4,
    Modern,
}

pub struct LoginCipher {
    blowfish: BlowfishLE,
    scheme: Scheme,
}

impl LoginCipher {
    pub fn c4() -> Self {
        let blowfish = BlowfishLE::new_from_slice(&C4_KEY).expect("chave de 21 bytes");
        Self { blowfish, scheme: Scheme::C4 }
    }

    pub fn init() -> Self {
        let blowfish = BlowfishLE::new_from_slice(&INIT_KEY).expect("chave de 16 bytes");
        Self { blowfish, scheme: Scheme::Modern }
    }

    pub fn session(key: &[u8; 16]) -> Self {
        let blowfish = BlowfishLE::new_from_slice(key).expect("chave de 16 bytes");
        Self { blowfish, scheme: Scheme::Modern }
    }

    pub fn open_init(&self, sealed: &[u8]) -> Result<Vec<u8>, CipherError> {
        if sealed.len() < 16 || !sealed.len().is_multiple_of(8) {
            return Err(CipherError::Length);
        }
        let mut out = sealed.to_vec();
        for block in out.as_chunks_mut::<8>().0 {
            self.blowfish.decrypt_block(block.into());
        }
        let len = out.len();
        let mut key = read_word(&out, len - 8);
        let mut position = len - 12;
        while position >= 4 {
            let value = read_word(&out, position) ^ key;
            key = key.wrapping_sub(value);
            out[position..position + 4].copy_from_slice(&value.to_le_bytes());
            position -= 4;
        }
        if read_word(&out, len - 4) != 0 {
            return Err(CipherError::Checksum);
        }
        Ok(out)
    }

    pub fn seal_init(&self, payload: &[u8], key: u32) -> Vec<u8> {
        let aligned = (payload.len() + 8).div_ceil(8) * 8;
        let mut out = vec![0u8; aligned];
        out[..payload.len()].copy_from_slice(payload);
        let mut key = key;
        let mut position = 4;
        while position < aligned - 8 {
            let value = read_word(&out, position);
            key = key.wrapping_add(value);
            out[position..position + 4].copy_from_slice(&(value ^ key).to_le_bytes());
            position += 4;
        }
        out[aligned - 8..aligned - 4].copy_from_slice(&key.to_le_bytes());
        for block in out.as_chunks_mut::<8>().0 {
            self.blowfish.encrypt_block(block.into());
        }
        out
    }

    pub fn seal(&self, payload: &[u8]) -> Vec<u8> {
        if self.scheme == Scheme::Modern {
            let aligned = (payload.len() + 4).div_ceil(8) * 8;
            let mut out = vec![0u8; aligned];
            out[..payload.len()].copy_from_slice(payload);
            let checksum = xor_words(&out[..aligned - 4]);
            out[aligned - 4..].copy_from_slice(&checksum.to_le_bytes());
            for block in out.as_chunks_mut::<8>().0 {
                self.blowfish.encrypt_block(block.into());
            }
            return out;
        }
        let aligned = (payload.len() + 7) & !7;
        let mut out = vec![0u8; aligned + 8];
        out[..payload.len()].copy_from_slice(payload);
        let checksum = xor_words(&out[..aligned]);
        out[aligned..aligned + 4].copy_from_slice(&checksum.to_le_bytes());
        for block in out.as_chunks_mut::<8>().0 {
            self.blowfish.encrypt_block(block.into());
        }
        out
    }

    pub fn open(&self, sealed: &[u8]) -> Result<Vec<u8>, CipherError> {
        if sealed.is_empty() || !sealed.len().is_multiple_of(8) {
            return Err(CipherError::Length);
        }
        let mut out = sealed.to_vec();
        for block in out.as_chunks_mut::<8>().0 {
            self.blowfish.decrypt_block(block.into());
        }
        if self.scheme == Scheme::Modern {
            let last = out.len() - 4;
            if xor_words(&out[..last]) != read_word(&out, last) {
                return Err(CipherError::Checksum);
            }
            out.truncate(last);
            return Ok(out);
        }
        if xor_words(&out[..out.len() - 4]) != 0 {
            return Err(CipherError::Checksum);
        }
        out.truncate(out.len() - 8);
        Ok(out)
    }
}

fn read_word(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

fn xor_words(bytes: &[u8]) -> u32 {
    bytes.as_chunks::<4>().0.iter().map(|w| u32::from_le_bytes(*w)).fold(0, |acc, w| acc ^ w)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_xor_pass_round_trips_through_the_static_blowfish_key() {
        let cipher = LoginCipher::init();
        let mut body = vec![0u8];
        body.extend((1..170u32).map(|i| (i * 7) as u8));
        for key in [0u32, 1, 0xdead_beef, u32::MAX] {
            let sealed = cipher.seal_init(&body, key);
            assert_eq!(sealed.len() % 8, 0);
            let opened = cipher.open_init(&sealed).unwrap();
            assert_eq!(&opened[..body.len()], &body[..]);
            assert!(opened[body.len()..opened.len() - 8].iter().all(|b| *b == 0));
        }
    }

    #[test]
    fn init_xor_pass_leaves_the_first_dword_in_clear_and_stores_the_key() {
        let cipher = LoginCipher::init();
        let body = [0u8; 16];
        let sealed = cipher.seal_init(&body, 0x1122_3344);
        assert_eq!(sealed.len(), 24);
        let mut raw = sealed.clone();
        for block in raw.as_chunks_mut::<8>().0 {
            cipher.blowfish.decrypt_block(block.into());
        }
        assert_eq!(&raw[..4], &[0, 0, 0, 0]);
        assert_ne!(read_word(&raw, 4), 0);
        assert_eq!(read_word(&raw, 16), 0x1122_3344);
        assert_eq!(read_word(&raw, 20), 0);
    }

    #[test]
    fn init_rejects_garbage_and_short_bodies() {
        let cipher = LoginCipher::init();
        assert!(matches!(cipher.open_init(&[0u8; 8]), Err(CipherError::Length)));
        assert!(matches!(cipher.open_init(&[0u8; 20]), Err(CipherError::Length)));
        assert!(matches!(cipher.open_init(&[0x5au8; 32]), Err(CipherError::Checksum)));
    }

    #[test]
    fn session_cipher_checksums_the_last_dword_and_round_trips() {
        let cipher = LoginCipher::session(&[7u8; 16]);
        for len in [1usize, 4, 5, 9, 21, 133] {
            let payload: Vec<u8> = (0..len as u32).map(|i| (i * 3 + 1) as u8).collect();
            let sealed = cipher.seal(&payload);
            assert_eq!(sealed.len() % 8, 0);
            assert!(sealed.len() >= len + 4);
            let opened = cipher.open(&sealed).unwrap();
            assert_eq!(&opened[..len], &payload[..]);
        }
        let mut sealed = cipher.seal(&[1, 2, 3, 4, 5]);
        sealed[0] ^= 1;
        assert!(matches!(cipher.open(&sealed), Err(CipherError::Checksum)));
    }

    #[test]
    fn open_rejects_bad_checksum() {
        let cipher = LoginCipher::c4();
        let mut sealed = cipher.seal(&[1, 2, 3]);
        sealed[0] ^= 1;
        assert!(matches!(cipher.open(&sealed), Err(CipherError::Checksum)));
    }

    #[test]
    fn open_rejects_unaligned_length() {
        let cipher = LoginCipher::c4();
        assert!(matches!(cipher.open(&[0u8; 12]), Err(CipherError::Length)));
    }
}
