use crate::client::AssetError;
use l2_crypto::Window;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;
use ue2_package::Package;

pub const KIND_GFX: u8 = 1;
pub const KIND_IMAGE: u8 = 2;

pub struct DecryptReader<R> {
    inner: R,
    window: Window,
    position: u64,
    length: u64,
}

impl<R: Read + Seek> DecryptReader<R> {
    pub fn new(mut inner: R, path: &Path) -> Result<Self, AssetError> {
        let mut head = [0u8; 64];
        let read = inner.read(&mut head)?;
        let window = Window::of(&head[..read], path).map_err(|e| AssetError::Crypto(format!("{e:?}")))?.ok_or_else(|| AssetError::Crypto("esquema sem leitura por faixa".into()))?;
        let length = window.payload_len(&mut inner)?;
        Ok(Self { inner, window, position: 0, length })
    }

    pub fn len(&self) -> u64 {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<R: Read + Seek> Read for DecryptReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.position >= self.length {
            return Ok(0);
        }
        let want = buf.len().min((self.length - self.position) as usize);
        self.inner.seek(SeekFrom::Start(self.window.file_offset(self.position)))?;
        let count = self.inner.read(&mut buf[..want])?;
        self.window.decrypt_in_place(&mut buf[..count]);
        self.position += count as u64;
        Ok(count)
    }
}

impl<R: Read + Seek> Seek for DecryptReader<R> {
    fn seek(&mut self, to: SeekFrom) -> io::Result<u64> {
        let target = match to {
            SeekFrom::Start(at) => at as i64,
            SeekFrom::Current(delta) => self.position as i64 + delta,
            SeekFrom::End(delta) => self.length as i64 + delta,
        };
        self.position = u64::try_from(target).map_err(|_| io::ErrorKind::InvalidInput)?;
        Ok(self.position)
    }
}

pub fn read_export(path: &Path, export: &str) -> Result<Vec<u8>, AssetError> {
    let mut reader = DecryptReader::new(File::open(path)?, path)?;
    let size = reader.len() as usize;
    let package = Package::read(&mut reader, size).map_err(|e| AssetError::Package(format!("{e:?}")))?;
    let entry = package
        .exports()
        .iter()
        .find(|entry| package.name(entry.name).eq_ignore_ascii_case(export) && package.class_name(entry) == "GFxFlash")
        .ok_or_else(|| AssetError::NotFound(format!("{}: export GFxFlash {export}", path.display())))?;
    reader.seek(SeekFrom::Start(entry.serial_offset as u64))?;
    let mut serial = vec![0u8; entry.serial_size];
    reader.read_exact(&mut serial)?;
    let (kind, start) = payload_start(&serial).ok_or_else(|| AssetError::Package(format!("{export}: serial do GFxFlash inválido")))?;
    if kind != KIND_GFX {
        return Err(AssetError::Package(format!("{export}: o export é do tipo {kind}, não um GFX")));
    }
    Ok(serial.split_off(start))
}

pub fn payload_start(serial: &[u8]) -> Option<(u8, usize)> {
    let (&none, rest) = serial.split_first()?;
    if none != 0 {
        return None;
    }
    let (&kind, rest) = rest.split_first()?;
    let mut value: i64 = 0;
    let mut used = 0;
    for (i, byte) in rest.iter().enumerate() {
        used = i + 1;
        if i == 0 {
            value = i64::from(byte & 0x3f);
            if byte & 0x40 == 0 {
                break;
            }
        } else {
            value |= i64::from(byte & 0x7f) << (6 + 7 * (i - 1));
            if byte & 0x80 == 0 {
                break;
            }
        }
    }
    let start = 2 + used;
    (start as i64 + value == serial.len() as i64).then_some((kind, start))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serial_prefix_is_none_kind_and_compact_length() {
        let mut serial = vec![0, 1, 0x42, 0x45];
        serial.extend(vec![7u8; 4418]);
        assert_eq!(payload_start(&serial), Some((1, 4)));
        assert_eq!(payload_start(&serial[..100]), None);
        assert_eq!(payload_start(&[1, 1, 0]), None);
        assert_eq!(payload_start(&[0, 2, 0x03, 9, 9, 9]), Some((2, 3)));
    }
}
