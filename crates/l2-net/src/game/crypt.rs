use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[error("KeyPacket inválido")]
pub struct KeyError;

pub const HIGH_FIVE_KEY_TAIL: [u8; 8] = [0xc8, 0x27, 0x93, 0x01, 0xa1, 0x6c, 0x31, 0x97];
pub const HIGH_FIVE_KEY_PACKET_LEN: usize = 23;
pub const OP_KEY_PACKET_HIGH_FIVE: u8 = 0x2e;

enum Keys {
    Narrow { tx: [u8; 8], rx: [u8; 8] },
    Wide { tx: [u8; 16], rx: [u8; 16] },
}

pub struct GameCrypt {
    keys: Keys,
}

impl GameCrypt {
    pub fn from_key_packet(body: &[u8]) -> Result<Self, KeyError> {
        if body.len() != 14 || body[0] != 0 || body[1] != 1 || body[6..10] != [1, 0, 0, 0] {
            return Err(KeyError);
        }
        let seed = u32::from_le_bytes([body[2], body[3], body[4], body[5]]);
        let tail: u32 = if seed & 0x8000_0000 != 0 { 0xffff_ffff } else { 0x8754_6ca1 };
        let mut key = [0u8; 8];
        key[..4].copy_from_slice(&seed.to_le_bytes());
        key[4..].copy_from_slice(&tail.to_le_bytes());
        Ok(Self { keys: Keys::Narrow { tx: key, rx: key } })
    }

    pub fn from_key_packet_high_five(body: &[u8]) -> Result<Self, KeyError> {
        if body.len() != HIGH_FIVE_KEY_PACKET_LEN || body[0] != OP_KEY_PACKET_HIGH_FIVE || body[1] != 1 {
            return Err(KeyError);
        }
        let mut key = [0u8; 16];
        key[..8].copy_from_slice(&body[2..10]);
        key[8..].copy_from_slice(&HIGH_FIVE_KEY_TAIL);
        Ok(Self { keys: Keys::Wide { tx: key, rx: key } })
    }

    pub fn encrypt(&mut self, data: &mut [u8]) {
        match &mut self.keys {
            Keys::Narrow { tx, .. } => {
                let mut previous = 0u8;
                for (i, byte) in data.iter_mut().enumerate() {
                    *byte ^= tx[i & 7] ^ previous;
                    previous = *byte;
                }
                advance(tx, data.len());
            }
            Keys::Wide { tx, .. } => {
                let mut previous = 0u8;
                for (i, byte) in data.iter_mut().enumerate() {
                    *byte ^= tx[i & 15] ^ previous;
                    previous = *byte;
                }
                advance_wide(tx, data.len());
            }
        }
    }

    pub fn decrypt(&mut self, data: &mut [u8]) {
        match &mut self.keys {
            Keys::Narrow { rx, .. } => {
                let mut previous = 0u8;
                for (i, byte) in data.iter_mut().enumerate() {
                    let cipher = *byte;
                    *byte ^= rx[i & 7] ^ previous;
                    previous = cipher;
                }
                advance(rx, data.len());
            }
            Keys::Wide { rx, .. } => {
                let mut previous = 0u8;
                for (i, byte) in data.iter_mut().enumerate() {
                    let cipher = *byte;
                    *byte ^= rx[i & 15] ^ previous;
                    previous = cipher;
                }
                advance_wide(rx, data.len());
            }
        }
    }
}

fn advance(key: &mut [u8; 8], len: usize) {
    let value = u64::from_le_bytes(*key).wrapping_add(len as u64);
    *key = value.to_le_bytes();
}

fn advance_wide(key: &mut [u8; 16], len: usize) {
    let value = u32::from_le_bytes([key[8], key[9], key[10], key[11]]).wrapping_add(len as u32);
    key[8..12].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    impl GameCrypt {
        fn narrow_tx(&self) -> [u8; 8] {
            match &self.keys {
                Keys::Narrow { tx, .. } => *tx,
                Keys::Wide { .. } => unreachable!(),
            }
        }
    }

    #[test]
    fn key_packet_shape_is_checked() {
        assert!(GameCrypt::from_key_packet(&[0; 13]).is_err());
        assert!(GameCrypt::from_key_packet(&[0, 0, 1, 2, 3, 4, 1, 0, 0, 0, 0, 0, 0, 0]).is_err());
        assert!(GameCrypt::from_key_packet(&[0, 1, 1, 2, 3, 4, 1, 0, 0, 0, 0, 0, 0, 0]).is_ok());
    }

    #[test]
    fn negative_seed_uses_all_ones_tail() {
        let c = GameCrypt::from_key_packet(&[0, 1, 0, 0, 0, 0x80, 1, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(c.narrow_tx(), [0, 0, 0, 0x80, 0xff, 0xff, 0xff, 0xff]);
        let c = GameCrypt::from_key_packet(&[0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]).unwrap();
        assert_eq!(c.narrow_tx(), [1, 0, 0, 0, 0xa1, 0x6c, 0x54, 0x87]);
    }

    fn key_packet_high_five(key: [u8; 8]) -> Vec<u8> {
        let mut body = vec![OP_KEY_PACKET_HIGH_FIVE, 1];
        body.extend_from_slice(&key);
        body.extend_from_slice(&[1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0]);
        body
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    #[test]
    fn high_five_key_packet_shape_is_checked() {
        let good = key_packet_high_five([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(good.len(), HIGH_FIVE_KEY_PACKET_LEN);
        assert!(GameCrypt::from_key_packet_high_five(&good).is_ok());
        assert!(GameCrypt::from_key_packet_high_five(&good[..22]).is_err());
        let mut refused = good.clone();
        refused[1] = 0;
        assert!(GameCrypt::from_key_packet_high_five(&refused).is_err());
        let mut wrong_op = good;
        wrong_op[0] = 0;
        assert!(GameCrypt::from_key_packet_high_five(&wrong_op).is_err());
    }

    #[test]
    fn high_five_cipher_matches_the_oracle_vectors_and_advances_the_counter() {
        let mut crypt = GameCrypt::from_key_packet_high_five(&key_packet_high_five([1, 2, 3, 4, 5, 6, 7, 8])).unwrap();
        let mut first: Vec<u8> = (0..20).collect();
        crypt.encrypt(&mut first);
        assert_eq!(hex(&first), "0102030405060708c8e67f75d8b9861e0f1c0d1a");
        let mut second = vec![0x0e, 0x11, 0x01, 0, 0];
        crypt.encrypt(&mut second);
        assert_eq!(hex(&second), "0f1c1e1a1f");
        match &crypt.keys {
            Keys::Wide { tx, rx } => {
                assert_eq!(hex(tx), "0102030405060708e1279301a16c3197");
                assert_eq!(hex(rx), "0102030405060708c8279301a16c3197");
            }
            Keys::Narrow { .. } => unreachable!(),
        }
    }

    #[test]
    fn high_five_decrypt_inverts_encrypt_across_packets() {
        let body = key_packet_high_five([9, 8, 7, 6, 5, 4, 3, 2]);
        let mut client = GameCrypt::from_key_packet_high_five(&body).unwrap();
        let mut server = GameCrypt::from_key_packet_high_five(&body).unwrap();
        for len in [1usize, 5, 16, 17, 300] {
            let plain: Vec<u8> = (0..len).map(|i| (i * 7 + len) as u8).collect();
            let mut wire = plain.clone();
            client.encrypt(&mut wire);
            server.decrypt(&mut wire);
            assert_eq!(wire, plain);
        }
    }

    #[test]
    fn high_five_counter_wraps_as_a_signed_32_bit_word() {
        let mut crypt = GameCrypt { keys: Keys::Wide { tx: [0, 0, 0, 0, 0, 0, 0, 0, 0xff, 0xff, 0xff, 0x7f, 0, 0, 0, 0], rx: [0; 16] } };
        let mut data = [1u8, 2, 3];
        crypt.encrypt(&mut data);
        assert_eq!(data, [1, 3, 0]);
        match &crypt.keys {
            Keys::Wide { tx, .. } => assert_eq!(&tx[8..12], &[2, 0, 0, 0x80]),
            Keys::Narrow { .. } => unreachable!(),
        }
    }
}

