use crate::{Error, HEADER_LEN};

pub(crate) const MAGIC: [u8; 4] = [0xFF, 0xED, 0xFA, 0xCE];
const VERSION_PACKAGE: [u8; 2] = [0x01, 0x00];
const VERSION_XDAT: [u8; 2] = [0x02, 0x00];
const PACKAGE_MASK: u8 = 0x3D;
const KEY_START: usize = 6;
const KEY_LEN: usize = 6;
const TABLE_MASK: usize = 0x7F;
const SIGN_BIT: usize = 0x80;

const TABLE: [u8; 128] = [
    0x71, 0x49, 0x6d, 0x41, 0xeb, 0xa2, 0x9b, 0xe9, 0x4f, 0x15, 0x24, 0xc5, 0xfb, 0x70, 0x3a, 0x8a, 0xa1, 0xb9, 0x1d,
    0x11, 0xbb, 0xd2, 0x4b, 0xd9, 0x1f, 0x25, 0x74, 0xb5, 0xcb, 0x20, 0xea, 0x5a, 0x11, 0x29, 0x8d, 0xe1, 0x0b, 0x42,
    0x3b, 0x49, 0xef, 0xb5, 0xc4, 0x25, 0x5b, 0x90, 0x5a, 0x2a, 0x41, 0x19, 0xbd, 0xb1, 0xdb, 0x72, 0xeb, 0xb9, 0xbf,
    0xc5, 0x14, 0x15, 0xab, 0x40, 0x0a, 0xfa, 0xb1, 0x89, 0x2d, 0x01, 0xab, 0xe2, 0x5b, 0x29, 0x0f, 0x55, 0x64, 0x85,
    0x3b, 0x30, 0xfa, 0x4a, 0xe1, 0xf9, 0xdd, 0xd1, 0x7b, 0x12, 0x0b, 0x19, 0xdf, 0x65, 0xb4, 0x75, 0x0b, 0xe0, 0xaa,
    0x1a, 0x51, 0x69, 0x4d, 0xa1, 0xcb, 0x82, 0xfb, 0x89, 0xaf, 0xf5, 0x04, 0xe5, 0x9b, 0x50, 0x1a, 0xea, 0x81, 0x59,
    0x7d, 0x71, 0x9b, 0xb2, 0xab, 0xf9, 0x7f, 0x05, 0x54, 0xd5, 0xeb, 0x00, 0xca, 0xba,
];

pub(crate) fn matches(data: &[u8]) -> bool {
    data.starts_with(&MAGIC)
}

pub(crate) fn version(data: &[u8]) -> Option<u16> {
    data.get(MAGIC.len()..HEADER_LEN.min(data.len()))?.first_chunk::<2>().map(|v| u16::from_le_bytes(*v))
}

fn mask(data: &[u8]) -> Result<u8, Error> {
    match [data[MAGIC.len()], data[MAGIC.len() + 1]] {
        VERSION_PACKAGE => Ok(PACKAGE_MASK),
        VERSION_XDAT => Ok(0),
        _ => Err(Error::UnsupportedVersion(version(data).unwrap_or(0))),
    }
}

pub(crate) fn decrypt(data: &[u8]) -> Result<Vec<u8>, Error> {
    if data.len() < HEADER_LEN {
        return Err(Error::Truncated);
    }
    let mask = mask(data)?;
    let key = &data[KEY_START..KEY_START + KEY_LEN];
    Ok(data
        .iter()
        .enumerate()
        .skip(HEADER_LEN)
        .map(|(offset, &byte)| {
            byte ^ TABLE[offset & TABLE_MASK] ^ (offset & SIGN_BIT) as u8 ^ key[offset % KEY_LEN] ^ mask
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encode(plain: &[u8], key: [u8; KEY_LEN], version: [u8; 2], mask: u8) -> Vec<u8> {
        let mut file = vec![0; HEADER_LEN];
        file[..MAGIC.len()].copy_from_slice(&MAGIC);
        file[MAGIC.len()..KEY_START].copy_from_slice(&version);
        file[KEY_START..KEY_START + KEY_LEN].copy_from_slice(&key);
        for (index, &byte) in plain.iter().enumerate() {
            let offset = HEADER_LEN + index;
            file.push(byte ^ TABLE[offset & TABLE_MASK] ^ (offset & SIGN_BIT) as u8 ^ key[offset % KEY_LEN] ^ mask);
        }
        file
    }

    #[test]
    fn round_trips_both_versions_across_the_whole_key_period() {
        let plain: Vec<u8> = (0..3000u32).map(|i| (i * 7 + i / 5) as u8).collect();
        for key in [[0; KEY_LEN], [0xD0, 0x96, 0x29, 0xC8, 0xF9, 0xA9]] {
            assert_eq!(decrypt(&encode(&plain, key, VERSION_XDAT, 0)).unwrap(), plain);
            assert_eq!(decrypt(&encode(&plain, key, VERSION_PACKAGE, PACKAGE_MASK)).unwrap(), plain);
        }
    }

    #[test]
    fn rejects_truncated_files_and_other_versions() {
        assert!(matches!(decrypt(&MAGIC), Err(Error::Truncated)));
        let mut file = encode(&[1, 2, 3], [0; KEY_LEN], VERSION_XDAT, 0);
        file[MAGIC.len()] = 0x03;
        assert!(matches!(decrypt(&file), Err(Error::UnsupportedVersion(3))));
    }
}
