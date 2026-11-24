//! Decryption of Lineage 2 client files protected by a `Lineage2VerXXX` header.
//!
//! Supports C4/High Five XOR containers and C2+ RSA/zlib data-table containers.
//! Pure: bytes in, bytes out.

mod cafef00d;
mod ffedface;
mod ver413;

use std::borrow::Cow;
use std::fmt;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

/// `Lineage2VerXXX` as UTF-16LE.
const HEADER_LEN: usize = 28;
/// Optional footer `[0, ?, ?, crc32, 0]` as little-endian u32s.
pub const TRAILER_LEN: usize = 20;
const MAGIC: &str = "Lineage2Ver";
pub const PROPRIETARY_CONTAINER_MAGIC: [u8; 4] = [0xCA, 0xFE, 0xF0, 0x0D];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Scheme {
    /// No `Lineage2Ver` header; the file is used as-is.
    Plain,
    Ver111,
    Ver121,
    Ver411,
    Ver412,
    Ver413,
    CafeF00d,
    FfEdFaCe,
}

#[derive(Debug)]
pub enum Error {
    UnsupportedVersion(u16),
    ProprietaryContainer,
    Truncated,
    MisalignedRsaPayload(usize),
    NoMatchingRsaKey,
    InvalidRsaBlock(usize),
    Inflate(std::io::Error),
    SizeMismatch { declared: usize, actual: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedVersion(v) => write!(f, "unsupported Lineage2Ver{v}"),
            Self::ProprietaryContainer => f.write_str("CAFEF00D container: known, not decodable by this port"),
            Self::Truncated => f.write_str("file is truncated"),
            Self::MisalignedRsaPayload(len) => write!(f, "RSA payload of {len} bytes is not a multiple of 128"),
            Self::NoMatchingRsaKey => f.write_str("no known Lineage2Ver411/412/413 RSA key opens this file"),
            Self::InvalidRsaBlock(i) => write!(f, "RSA block {i} is malformed"),
            Self::Inflate(e) => write!(f, "zlib stream is corrupt: {e}"),
            Self::SizeMismatch { declared, actual } => {
                write!(f, "declared {declared} decompressed bytes, got {actual}")
            }
        }
    }
}

impl std::error::Error for Error {}

/// Identifies the protection scheme from the file header.
pub fn detect(data: &[u8]) -> Result<Scheme, Error> {
    if cafef00d::matches(data) {
        return Ok(Scheme::CafeF00d);
    }
    if ffedface::matches(data) {
        return Ok(Scheme::FfEdFaCe);
    }
    match header_version(data) {
        None => Ok(Scheme::Plain),
        Some(111) => Ok(Scheme::Ver111),
        Some(121) => Ok(Scheme::Ver121),
        Some(411) => Ok(Scheme::Ver411),
        Some(412) => Ok(Scheme::Ver412),
        Some(413) => Ok(Scheme::Ver413),
        Some(other) => Err(Error::UnsupportedVersion(other)),
    }
}

/// Decrypts a client file. `path` is only used for its file name, which keys `Ver121`.
pub fn decrypt<'a>(data: &'a [u8], path: &Path) -> Result<Cow<'a, [u8]>, Error> {
    Ok(Cow::Owned(match detect(data)? {
        Scheme::Plain => return Ok(Cow::Borrowed(data)),
        Scheme::Ver111 => xor(xor_payload(data)?, 0xAC),
        Scheme::Ver121 => xor(xor_payload(data)?, ver121_key(path)),
        Scheme::Ver411 | Scheme::Ver412 | Scheme::Ver413 => {
            ver413::decrypt(data.get(HEADER_LEN..).ok_or(Error::Truncated)?)?
        }
        Scheme::CafeF00d => cafef00d::decrypt(data)?,
        Scheme::FfEdFaCe => ffedface::decrypt(data)?,
    }))
}

/// Decrypts a client file in place, without a second copy of it.
pub fn decrypt_owned(mut data: Vec<u8>, path: &Path) -> Result<Vec<u8>, Error> {
    let key = match detect(&data)? {
        Scheme::Plain => return Ok(data),
        Scheme::Ver111 => 0xAC,
        Scheme::Ver121 => ver121_key(path),
        Scheme::Ver411 | Scheme::Ver412 | Scheme::Ver413 | Scheme::CafeF00d | Scheme::FfEdFaCe => {
            return decrypt(&data, path).map(Cow::into_owned);
        }
    };
    let payload = xor_payload(&data)?.len();
    data.truncate(HEADER_LEN + payload);
    data.drain(..HEADER_LEN);
    for byte in &mut data {
        *byte ^= key;
    }
    Ok(data)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    header: usize,
    key: u8,
}

impl Window {
    pub fn payload_len(self, input: &mut (impl Read + Seek)) -> io::Result<u64> {
        let size = input.seek(SeekFrom::End(0))?;
        let body = size.checked_sub(self.header as u64).ok_or(io::ErrorKind::UnexpectedEof)?;
        if self.header == 0 || body < TRAILER_LEN as u64 {
            return Ok(body);
        }
        let signed = size - TRAILER_LEN as u64;
        input.seek(SeekFrom::Start(signed + 12))?;
        let mut crc = [0; 4];
        input.read_exact(&mut crc)?;
        input.seek(SeekFrom::Start(0))?;
        let mut hasher = flate2::Crc::new();
        let mut buffer = [0; 65536];
        let mut remaining = signed;
        while remaining > 0 {
            let count = remaining.min(buffer.len() as u64) as usize;
            input.read_exact(&mut buffer[..count])?;
            hasher.update(&buffer[..count]);
            remaining -= count as u64;
        }
        Ok(if hasher.sum() == u32::from_le_bytes(crc) { body - TRAILER_LEN as u64 } else { body })
    }

    pub fn of(data: &[u8], path: &Path) -> Result<Option<Self>, Error> {
        Ok(match detect(data)? {
            Scheme::Plain => Some(Self { header: 0, key: 0 }),
            Scheme::Ver111 => Some(Self { header: HEADER_LEN, key: 0xAC }),
            Scheme::Ver121 => Some(Self { header: HEADER_LEN, key: ver121_key(path) }),
            Scheme::Ver411 | Scheme::Ver412 | Scheme::Ver413 | Scheme::CafeF00d | Scheme::FfEdFaCe => None,
        })
    }

    pub fn file_offset(self, offset: u64) -> u64 {
        self.header as u64 + offset
    }

    pub fn decrypt_in_place(self, bytes: &mut [u8]) {
        for byte in bytes {
            *byte ^= self.key;
        }
    }
}

fn xor_payload(data: &[u8]) -> Result<&[u8], Error> {
    let body = data.get(HEADER_LEN..).ok_or(Error::Truncated)?;
    let Some((signed, trailer)) = data.split_last_chunk::<TRAILER_LEN>() else {
        return Ok(body);
    };
    let [.., c0, c1, c2, c3, _, _, _, _] = *trailer;
    let crc = u32::from_le_bytes([c0, c1, c2, c3]);
    let mut hasher = flate2::Crc::new();
    hasher.update(signed);
    match signed.get(HEADER_LEN..) {
        Some(payload) if hasher.sum() == crc => Ok(payload),
        _ => Ok(body),
    }
}

fn header_version(data: &[u8]) -> Option<u16> {
    let units = data.get(..HEADER_LEN)?.as_chunks::<2>().0.iter().map(|&unit| u16::from_le_bytes(unit));
    let text: String = char::decode_utf16(units).collect::<Result<_, _>>().ok()?;
    text.strip_prefix(MAGIC)?.parse().ok()
}

fn ver121_key(path: &Path) -> u8 {
    let name = path.file_name().map(|n| n.to_string_lossy().to_lowercase()).unwrap_or_default();
    let [low, ..] = name.encode_utf16().fold(0u32, |sum, unit| sum.wrapping_add(unit.into())).to_le_bytes();
    low
}

fn xor(payload: &[u8], key: u8) -> Vec<u8> {
    payload.iter().map(|b| b ^ key).collect()
}
