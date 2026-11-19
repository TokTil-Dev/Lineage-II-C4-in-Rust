use num_bigint::BigUint;

pub const BLOCK_LEN: usize = 128;
pub const USER_OFFSET: usize = 0x5e;
pub const USER_LEN: usize = 14;
pub const PASSWORD_OFFSET: usize = 0x6c;
pub const PASSWORD_LEN: usize = 16;
const EXPONENT: u32 = 0x10001;

pub fn unscramble_modulus(n: &mut [u8; BLOCK_LEN]) {
    for i in 0..0x40 {
        n[0x40 + i] ^= n[i];
    }
    for i in 0..4 {
        n[0x0d + i] ^= n[0x34 + i];
    }
    for i in 0..0x40 {
        n[i] ^= n[0x40 + i];
    }
    for i in 0..4 {
        n.swap(i, 0x4d + i);
    }
}

pub fn credentials_block(user: &[u8], password: &[u8]) -> Option<[u8; BLOCK_LEN]> {
    if user.len() > USER_LEN || password.len() > PASSWORD_LEN {
        return None;
    }
    let mut block = [0u8; BLOCK_LEN];
    block[USER_OFFSET..USER_OFFSET + user.len()].copy_from_slice(user);
    block[PASSWORD_OFFSET..PASSWORD_OFFSET + password.len()].copy_from_slice(password);
    Some(block)
}

pub fn encrypt_no_padding(modulus: &[u8], block: &[u8]) -> Option<Vec<u8>> {
    let n = BigUint::from_bytes_be(modulus);
    let m = BigUint::from_bytes_be(block);
    if n.bits() == 0 || m >= n {
        return None;
    }
    let bytes = m.modpow(&BigUint::from(EXPONENT), &n).to_bytes_be();
    let mut out = vec![0u8; modulus.len()];
    let start = out.len().checked_sub(bytes.len())?;
    out[start..].copy_from_slice(&bytes);
    Some(out)
}

#[cfg(test)]
pub fn scramble_modulus(n: &mut [u8; BLOCK_LEN]) {
    for i in 0..4 {
        n.swap(i, 0x4d + i);
    }
    for i in 0..0x40 {
        n[i] ^= n[0x40 + i];
    }
    for i in 0..4 {
        n[0x0d + i] ^= n[0x34 + i];
    }
    for i in 0..0x40 {
        n[0x40 + i] ^= n[i];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;

    fn modular_inverse(a: &BigUint, m: &BigUint) -> BigUint {
        let (mut old_r, mut r) = (BigInt::from(a.clone()), BigInt::from(m.clone()));
        let (mut old_s, mut s) = (BigInt::from(1), BigInt::from(0));
        while r != BigInt::from(0) {
            let q = &old_r / &r;
            (old_r, r) = (r.clone(), &old_r - &q * &r);
            (old_s, s) = (s.clone(), &old_s - &q * &s);
        }
        let m = BigInt::from(m.clone());
        (((old_s % &m) + &m) % &m).to_biguint().unwrap()
    }

    #[test]
    fn scramble_and_unscramble_are_inverse_for_every_position() {
        let mut original = [0u8; BLOCK_LEN];
        for (i, b) in original.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(37).wrapping_add(11);
        }
        let mut n = original;
        scramble_modulus(&mut n);
        assert_ne!(n, original);
        unscramble_modulus(&mut n);
        assert_eq!(n, original);
    }

    #[test]
    fn unscramble_matches_a_hand_computed_vector() {
        let mut n = [0u8; BLOCK_LEN];
        n[0x40] = 0xaa;
        n[0] = 0x55;
        n[0x4d] = 0x01;
        let mut expected = [0u8; BLOCK_LEN];
        expected[0] = 0x01;
        expected[0x0d] = 0x01;
        expected[0x40] = 0xff;
        expected[0x4d] = 0xaa;
        unscramble_modulus(&mut n);
        assert_eq!(n, expected);
    }

    #[test]
    fn credentials_block_places_user_and_password_at_the_client_offsets() {
        let block = credentials_block(b"conta", b"senha").unwrap();
        assert_eq!(&block[0x5e..0x63], b"conta");
        assert_eq!(&block[0x6c..0x71], b"senha");
        assert!(block[..0x5e].iter().all(|b| *b == 0));
        assert!(block[0x63..0x6c].iter().all(|b| *b == 0));
        assert!(block[0x71..].iter().all(|b| *b == 0));
        assert!(credentials_block(&[b'a'; 15], b"x").is_none());
        assert!(credentials_block(b"x", &[b'a'; 17]).is_none());
        let full = credentials_block(&[b'u'; 14], &[b'p'; 16]).unwrap();
        assert_eq!(&full[0x5e..0x6c], &[b'u'; 14]);
        assert_eq!(&full[0x6c..0x7c], &[b'p'; 16]);
    }

    #[test]
    fn no_padding_rsa_round_trips_with_a_known_private_exponent() {
        let p = (BigUint::from(1u8) << 521u32) - 1u8;
        let q = (BigUint::from(1u8) << 607u32) - 1u8;
        let n = &p * &q;
        let phi = (&p - 1u8) * (&q - 1u8);
        let d = modular_inverse(&BigUint::from(EXPONENT), &phi);
        let modulus = n.to_bytes_be();
        let block = credentials_block(b"conta", b"senha").unwrap();
        let sealed = encrypt_no_padding(&modulus, &block).unwrap();
        assert_eq!(sealed.len(), modulus.len());
        let plain = BigUint::from_bytes_be(&sealed).modpow(&d, &n).to_bytes_be();
        let mut padded = vec![0u8; BLOCK_LEN - plain.len()];
        padded.extend_from_slice(&plain);
        assert_eq!(padded, block);
    }

    #[test]
    fn message_not_below_the_modulus_is_refused() {
        assert!(encrypt_no_padding(&[0x80; 128], &[0xff; 128]).is_none());
        assert!(encrypt_no_padding(&[], &[1]).is_none());
    }
}
