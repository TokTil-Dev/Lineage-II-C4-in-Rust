use std::io::Read;
use std::sync::Arc;

pub const TAG_END: u16 = 0;
pub const TAG_DEFINE_FONT: u16 = 10;
pub const TAG_DEFINE_FONT_INFO: u16 = 13;
pub const TAG_DEFINE_FONT_INFO2: u16 = 62;
pub const TAG_DEFINE_FONT2: u16 = 48;
pub const TAG_DEFINE_FONT_ALIGN_ZONES: u16 = 73;
pub const TAG_DEFINE_FONT3: u16 = 75;
pub const TAG_DEFINE_FONT_NAME: u16 = 88;
pub const TAG_DEFINE_FONT4: u16 = 91;
pub const TAG_GFX_EXPORTER_INFO: u16 = 1000;

pub const EM_FONT1_2: i32 = 1024;
pub const EM_FONT3: i32 = 20480;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwfError {
    Truncated(usize),
    Signature,
    Inflate,
    Length,
    Shape(usize),
    Font(u16),
}

impl std::fmt::Display for SwfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated(at) => write!(f, "SWF truncado em {at:#x}"),
            Self::Signature => write!(f, "assinatura SWF/GFX desconhecida"),
            Self::Inflate => write!(f, "corpo zlib do SWF inválido"),
            Self::Length => write!(f, "tamanho do SWF não bate com o cabeçalho"),
            Self::Shape(at) => write!(f, "forma de glifo inválida em {at:#x}"),
            Self::Font(id) => write!(f, "fonte {id} inválida"),
        }
    }
}

impl std::error::Error for SwfError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagRef {
    pub code: u16,
    pub start: usize,
    pub len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Swf {
    pub signature: [u8; 3],
    pub version: u8,
    pub declared_length: u32,
    pub frame_size: Rect,
    pub frame_rate: u16,
    pub frame_count: u16,
    pub tags: Vec<TagRef>,
    body: Vec<u8>,
}

impl Swf {
    pub fn parse(bytes: &[u8]) -> Result<Swf, SwfError> {
        let head = bytes.get(..8).ok_or(SwfError::Truncated(0))?;
        let signature = [head[0], head[1], head[2]];
        let compressed = match &signature {
            b"FWS" | b"GFX" => false,
            b"CWS" | b"CFX" => true,
            _ => return Err(SwfError::Signature),
        };
        let version = head[3];
        let declared_length = u32::from_le_bytes([head[4], head[5], head[6], head[7]]);
        let rest = &bytes[8..];
        let body = if compressed {
            let mut out = Vec::with_capacity((declared_length as usize).saturating_sub(8));
            flate2::read::ZlibDecoder::new(rest).read_to_end(&mut out).map_err(|_| SwfError::Inflate)?;
            out
        } else {
            rest.to_vec()
        };
        if body.len() + 8 != declared_length as usize {
            return Err(SwfError::Length);
        }
        let mut bits = Bits::new(&body, 0);
        let frame_size = bits.rect()?;
        let mut at = bits.byte_pos();
        let frame_rate = u16_at(&body, at)?;
        let frame_count = u16_at(&body, at + 2)?;
        at += 4;
        let mut tags = Vec::new();
        while at < body.len() {
            let header = u16_at(&body, at)?;
            at += 2;
            let code = header >> 6;
            let mut len = (header & 0x3f) as usize;
            if len == 0x3f {
                len = u32_at(&body, at)? as usize;
                at += 4;
            }
            let end = at.checked_add(len).filter(|end| *end <= body.len()).ok_or(SwfError::Truncated(at))?;
            tags.push(TagRef { code, start: at, len });
            at = end;
            if code == TAG_END {
                break;
            }
        }
        Ok(Swf { signature, version, declared_length, frame_size, frame_rate, frame_count, tags, body })
    }

    pub fn tag(&self, tag: &TagRef) -> &[u8] {
        &self.body[tag.start..tag.start + tag.len]
    }

    pub fn tags_of(&self, code: u16) -> impl Iterator<Item = &TagRef> {
        self.tags.iter().filter(move |tag| tag.code == code)
    }

    pub fn fonts(&self) -> Result<Vec<Font>, SwfError> {
        let mut fonts: Vec<Font> = Vec::new();
        for tag in &self.tags {
            match tag.code {
                TAG_DEFINE_FONT => fonts.push(Font::define_font1(self.tag(tag))?),
                TAG_DEFINE_FONT2 | TAG_DEFINE_FONT3 => fonts.push(Font::define_font23(self.tag(tag), tag.code == TAG_DEFINE_FONT3)?),
                TAG_DEFINE_FONT_INFO | TAG_DEFINE_FONT_INFO2 => {
                    let data = self.tag(tag);
                    let id = u16_at(data, 0)?;
                    if let Some(font) = fonts.iter_mut().find(|font| font.id == id) {
                        font.apply_info(data, tag.code == TAG_DEFINE_FONT_INFO2)?;
                    }
                }
                TAG_DEFINE_FONT_NAME => {
                    let data = self.tag(tag);
                    let id = u16_at(data, 0)?;
                    let mut at = 2;
                    let name = cstring(data, &mut at)?;
                    let copyright = cstring(data, &mut at)?;
                    if let Some(font) = fonts.iter_mut().find(|font| font.id == id) {
                        font.display_name = Some(name);
                        font.copyright = Some(copyright);
                    }
                }
                _ => {}
            }
        }
        Ok(fonts)
    }

    pub fn exporter_info(&self) -> Option<&[u8]> {
        self.tags_of(TAG_GFX_EXPORTER_INFO).next().map(|tag| self.tag(tag))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kerning {
    pub first: u16,
    pub second: u16,
    pub adjustment: i16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Font {
    pub id: u16,
    pub tag: u16,
    pub name: String,
    pub display_name: Option<String>,
    pub copyright: Option<String>,
    pub bold: bool,
    pub italic: bool,
    pub small_text: bool,
    pub language: u8,
    pub em: i32,
    pub ascent: i16,
    pub descent: i16,
    pub leading: i16,
    pub has_layout: bool,
    pub codes: Vec<u16>,
    pub advances: Vec<i16>,
    pub bounds: Vec<Rect>,
    pub kerning: Vec<Kerning>,
    data: Arc<[u8]>,
    shapes: Vec<(usize, usize)>,
}

impl Font {
    fn define_font1(data: &[u8]) -> Result<Font, SwfError> {
        let id = u16_at(data, 0)?;
        let first = u16_at(data, 2)? as usize;
        let count = first / 2;
        let mut offsets = Vec::with_capacity(count);
        for index in 0..count {
            offsets.push(u16_at(data, 2 + index * 2)? as usize);
        }
        let shapes = shape_ranges(&offsets, data.len() - 2, 2)?;
        Ok(Font {
            id,
            tag: TAG_DEFINE_FONT,
            name: String::new(),
            display_name: None,
            copyright: None,
            bold: false,
            italic: false,
            small_text: false,
            language: 0,
            em: EM_FONT1_2,
            ascent: 0,
            descent: 0,
            leading: 0,
            has_layout: false,
            codes: Vec::new(),
            advances: Vec::new(),
            bounds: Vec::new(),
            kerning: Vec::new(),
            data: Arc::from(data),
            shapes,
        })
    }

    fn define_font23(data: &[u8], version3: bool) -> Result<Font, SwfError> {
        let id = u16_at(data, 0)?;
        let flags = *data.get(2).ok_or(SwfError::Truncated(2))?;
        let language = *data.get(3).ok_or(SwfError::Truncated(3))?;
        let name_len = *data.get(4).ok_or(SwfError::Truncated(4))? as usize;
        let name_bytes = data.get(5..5 + name_len).ok_or(SwfError::Truncated(5))?;
        let name = String::from_utf8_lossy(name_bytes.split(|b| *b == 0).next().unwrap_or(&[])).into_owned();
        let mut at = 5 + name_len;
        let count = u16_at(data, at)? as usize;
        at += 2;
        let wide_offsets = flags & 0x08 != 0;
        let wide_codes = flags & 0x04 != 0;
        let table = at;
        let step = if wide_offsets { 4 } else { 2 };
        let mut offsets = Vec::with_capacity(count);
        for index in 0..count {
            offsets.push(offset_at(data, table + index * step, wide_offsets)?);
        }
        let code_table = offset_at(data, table + count * step, wide_offsets)?;
        let shapes = shape_ranges(&offsets, code_table, table)?;
        let mut at = table.checked_add(code_table).ok_or(SwfError::Font(id))?;
        let mut codes = Vec::with_capacity(count);
        for _ in 0..count {
            if wide_codes {
                codes.push(u16_at(data, at)?);
                at += 2;
            } else {
                codes.push(*data.get(at).ok_or(SwfError::Truncated(at))? as u16);
                at += 1;
            }
        }
        let has_layout = flags & 0x80 != 0;
        let (mut ascent, mut descent, mut leading) = (0, 0, 0);
        let mut advances = Vec::new();
        let mut bounds = Vec::new();
        let mut kerning = Vec::new();
        if has_layout {
            ascent = i16_at(data, at)?;
            descent = i16_at(data, at + 2)?;
            leading = i16_at(data, at + 4)?;
            at += 6;
            for _ in 0..count {
                advances.push(i16_at(data, at)?);
                at += 2;
            }
            for _ in 0..count {
                let mut bits = Bits::new(data, at);
                bounds.push(bits.rect()?);
                at = bits.byte_pos();
            }
            let pairs = u16_at(data, at)? as usize;
            at += 2;
            for _ in 0..pairs {
                let (first, second) = if wide_codes {
                    let pair = (u16_at(data, at)?, u16_at(data, at + 2)?);
                    at += 4;
                    pair
                } else {
                    let pair = (*data.get(at).ok_or(SwfError::Truncated(at))? as u16, *data.get(at + 1).ok_or(SwfError::Truncated(at))? as u16);
                    at += 2;
                    pair
                };
                kerning.push(Kerning { first, second, adjustment: i16_at(data, at)? });
                at += 2;
            }
        }
        Ok(Font {
            id,
            tag: if version3 { TAG_DEFINE_FONT3 } else { TAG_DEFINE_FONT2 },
            name,
            display_name: None,
            copyright: None,
            bold: flags & 0x01 != 0,
            italic: flags & 0x02 != 0,
            small_text: flags & 0x20 != 0,
            language,
            em: if version3 { EM_FONT3 } else { EM_FONT1_2 },
            ascent,
            descent,
            leading,
            has_layout,
            codes,
            advances,
            bounds,
            kerning,
            data: Arc::from(data),
            shapes,
        })
    }

    fn apply_info(&mut self, data: &[u8], info2: bool) -> Result<(), SwfError> {
        let name_len = *data.get(2).ok_or(SwfError::Truncated(2))? as usize;
        let name_bytes = data.get(3..3 + name_len).ok_or(SwfError::Truncated(3))?;
        self.name = String::from_utf8_lossy(name_bytes).into_owned();
        let mut at = 3 + name_len;
        let flags = *data.get(at).ok_or(SwfError::Truncated(at))?;
        at += 1;
        self.small_text = flags & 0x20 != 0;
        self.italic = flags & 0x04 != 0;
        self.bold = flags & 0x02 != 0;
        let wide = flags & 0x01 != 0;
        if info2 {
            self.language = *data.get(at).ok_or(SwfError::Truncated(at))?;
            at += 1;
        }
        self.codes.clear();
        for _ in 0..self.shapes.len() {
            if wide {
                self.codes.push(u16_at(data, at)?);
                at += 2;
            } else {
                self.codes.push(*data.get(at).ok_or(SwfError::Truncated(at))? as u16);
                at += 1;
            }
        }
        Ok(())
    }

    pub fn glyph_count(&self) -> usize {
        self.shapes.len()
    }

    pub fn index_of(&self, code: u16) -> Option<usize> {
        self.codes.binary_search(&code).ok().or_else(|| self.codes.iter().position(|c| *c == code))
    }

    pub fn advance(&self, index: usize) -> Option<i16> {
        self.advances.get(index).copied()
    }

    pub fn kerning_between(&self, first: u16, second: u16) -> i16 {
        self.kerning.iter().find(|pair| pair.first == first && pair.second == second).map_or(0, |pair| pair.adjustment)
    }

    pub fn outline(&self, index: usize) -> Result<Outline, SwfError> {
        let &(start, end) = self.shapes.get(index).ok_or(SwfError::Font(self.id))?;
        read_shape(&self.data, start, end)
    }
}

fn shape_ranges(offsets: &[usize], end: usize, base: usize) -> Result<Vec<(usize, usize)>, SwfError> {
    let mut ranges = Vec::with_capacity(offsets.len());
    for (index, offset) in offsets.iter().enumerate() {
        let next = offsets.get(index + 1).copied().unwrap_or(end);
        if next < *offset {
            return Err(SwfError::Shape(base + offset));
        }
        ranges.push((base + offset, base + next));
    }
    Ok(ranges)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Segment {
    Line { to: (i32, i32) },
    Curve { control: (i32, i32), to: (i32, i32) },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path {
    pub start: (i32, i32),
    pub fill0: u32,
    pub fill1: u32,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Outline {
    pub paths: Vec<Path>,
}

fn read_shape(data: &[u8], start: usize, end: usize) -> Result<Outline, SwfError> {
    let slice = data.get(..end).ok_or(SwfError::Shape(start))?;
    let mut bits = Bits::new(slice, start);
    let fill_bits = bits.unsigned(4)?;
    let line_bits = bits.unsigned(4)?;
    let mut outline = Outline::default();
    let (mut x, mut y) = (0i32, 0i32);
    let (mut fill0, mut fill1) = (0u32, 0u32);
    let mut open: Option<Path> = None;
    loop {
        if bits.unsigned(1)? == 1 {
            let straight = bits.unsigned(1)? == 1;
            let count = bits.unsigned(4)? + 2;
            let origin = (x, y);
            let segment = if straight {
                let (dx, dy) = if bits.unsigned(1)? == 1 {
                    (bits.signed(count)?, bits.signed(count)?)
                } else if bits.unsigned(1)? == 1 {
                    (0, bits.signed(count)?)
                } else {
                    (bits.signed(count)?, 0)
                };
                x += dx;
                y += dy;
                Segment::Line { to: (x, y) }
            } else {
                let control = (x + bits.signed(count)?, y + bits.signed(count)?);
                x = control.0 + bits.signed(count)?;
                y = control.1 + bits.signed(count)?;
                Segment::Curve { control, to: (x, y) }
            };
            open.get_or_insert_with(|| Path { start: origin, fill0, fill1, segments: Vec::new() }).segments.push(segment);
        } else {
            let new_styles = bits.unsigned(1)? == 1;
            let line_style = bits.unsigned(1)? == 1;
            let fill_style1 = bits.unsigned(1)? == 1;
            let fill_style0 = bits.unsigned(1)? == 1;
            let move_to = bits.unsigned(1)? == 1;
            if !(new_styles || line_style || fill_style1 || fill_style0 || move_to) {
                break;
            }
            if let Some(path) = open.take() {
                outline.paths.push(path);
            }
            if move_to {
                let count = bits.unsigned(5)?;
                x = bits.signed(count)?;
                y = bits.signed(count)?;
            }
            if fill_style0 {
                fill0 = bits.unsigned(fill_bits)?;
            }
            if fill_style1 {
                fill1 = bits.unsigned(fill_bits)?;
            }
            if line_style {
                bits.unsigned(line_bits)?;
            }
            if new_styles {
                return Err(SwfError::Shape(bits.byte_pos()));
            }
            open = Some(Path { start: (x, y), fill0, fill1, segments: Vec::new() });
        }
    }
    if let Some(path) = open.take() {
        outline.paths.push(path);
    }
    outline.paths.retain(|path| !path.segments.is_empty());
    Ok(outline)
}

struct Bits<'a> {
    data: &'a [u8],
    byte: usize,
    bit: u8,
}

impl<'a> Bits<'a> {
    fn new(data: &'a [u8], byte: usize) -> Self {
        Bits { data, byte, bit: 0 }
    }

    fn byte_pos(&self) -> usize {
        self.byte + usize::from(self.bit > 0)
    }

    fn unsigned(&mut self, count: u32) -> Result<u32, SwfError> {
        let mut value = 0u32;
        for _ in 0..count {
            let byte = *self.data.get(self.byte).ok_or(SwfError::Truncated(self.byte))?;
            value = (value << 1) | u32::from((byte >> (7 - self.bit)) & 1);
            self.bit += 1;
            if self.bit == 8 {
                self.bit = 0;
                self.byte += 1;
            }
        }
        Ok(value)
    }

    fn signed(&mut self, count: u32) -> Result<i32, SwfError> {
        if count == 0 {
            return Ok(0);
        }
        let value = self.unsigned(count)?;
        let shift = 32 - count;
        Ok(((value << shift) as i32) >> shift)
    }

    fn rect(&mut self) -> Result<Rect, SwfError> {
        let count = self.unsigned(5)?;
        let rect = Rect { x_min: self.signed(count)?, x_max: self.signed(count)?, y_min: self.signed(count)?, y_max: self.signed(count)? };
        Ok(rect)
    }
}

fn u16_at(data: &[u8], at: usize) -> Result<u16, SwfError> {
    let bytes = data.get(at..at + 2).ok_or(SwfError::Truncated(at))?;
    Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
}

fn i16_at(data: &[u8], at: usize) -> Result<i16, SwfError> {
    u16_at(data, at).map(|v| v as i16)
}

fn u32_at(data: &[u8], at: usize) -> Result<u32, SwfError> {
    let bytes = data.get(at..at + 4).ok_or(SwfError::Truncated(at))?;
    Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

fn offset_at(data: &[u8], at: usize, wide: bool) -> Result<usize, SwfError> {
    if wide { u32_at(data, at).map(|v| v as usize) } else { u16_at(data, at).map(usize::from) }
}

fn cstring(data: &[u8], at: &mut usize) -> Result<String, SwfError> {
    let rest = data.get(*at..).ok_or(SwfError::Truncated(*at))?;
    let end = rest.iter().position(|b| *b == 0).ok_or(SwfError::Truncated(*at))?;
    let text = String::from_utf8_lossy(&rest[..end]).into_owned();
    *at += end + 1;
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct BitWriter {
        bytes: Vec<u8>,
        used: u8,
    }

    impl BitWriter {
        fn put(&mut self, value: u32, count: u32) {
            for shift in (0..count).rev() {
                if self.used == 0 {
                    self.bytes.push(0);
                }
                let last = self.bytes.len() - 1;
                self.bytes[last] |= (((value >> shift) & 1) as u8) << (7 - self.used);
                self.used = (self.used + 1) % 8;
            }
        }

        fn signed(&mut self, value: i32, count: u32) {
            self.put((value as u32) & ((1u64 << count) - 1) as u32, count);
        }
    }

    fn square_shape() -> Vec<u8> {
        let mut w = BitWriter::default();
        w.put(1, 4);
        w.put(0, 4);
        w.put(0, 1);
        w.put(0b00101, 5);
        w.put(12, 5);
        w.signed(100, 12);
        w.signed(-200, 12);
        w.put(1, 1);
        w.put(1, 1);
        w.put(1, 1);
        w.put(10, 4);
        w.put(0, 1);
        w.put(0, 1);
        w.signed(300, 12);
        w.put(1, 1);
        w.put(1, 1);
        w.put(10, 4);
        w.put(0, 1);
        w.put(1, 1);
        w.signed(400, 12);
        w.put(1, 1);
        w.put(0, 1);
        w.put(10, 4);
        w.signed(-150, 12);
        w.signed(0, 12);
        w.signed(-150, 12);
        w.signed(-400, 12);
        w.put(0, 6);
        w.bytes
    }

    fn rect_bytes() -> Vec<u8> {
        let mut w = BitWriter::default();
        w.put(3, 5);
        w.signed(0, 3);
        w.signed(3, 3);
        w.signed(0, 3);
        w.signed(3, 3);
        w.bytes
    }

    fn define_font3() -> Vec<u8> {
        let shape = square_shape();
        let mut d = Vec::new();
        d.extend(7u16.to_le_bytes());
        d.push(0x80 | 0x08 | 0x04 | 0x01);
        d.push(1);
        d.push(4);
        d.extend(b"Abc\0");
        d.extend(1u16.to_le_bytes());
        let table = d.len();
        let first = 8u32;
        d.extend(first.to_le_bytes());
        d.extend((first + shape.len() as u32).to_le_bytes());
        assert_eq!(d.len() - table, 8);
        d.extend(&shape);
        d.extend(0x41u16.to_le_bytes());
        d.extend(20480i16.to_le_bytes());
        d.extend(4000i16.to_le_bytes());
        d.extend(100i16.to_le_bytes());
        d.extend(12000i16.to_le_bytes());
        d.extend(rect_bytes());
        d.extend(1u16.to_le_bytes());
        d.extend(0x41u16.to_le_bytes());
        d.extend(0x41u16.to_le_bytes());
        d.extend((-5i16).to_le_bytes());
        d
    }

    fn swf_with(tag: u16, body: &[u8], signature: &[u8; 3]) -> Vec<u8> {
        let mut rect = BitWriter::default();
        rect.put(1, 5);
        for _ in 0..4 {
            rect.signed(0, 1);
        }
        let mut payload = rect.bytes;
        payload.extend(0x0c00u16.to_le_bytes());
        payload.extend(1u16.to_le_bytes());
        payload.extend(((tag << 6) | 0x3f).to_le_bytes());
        payload.extend((body.len() as u32).to_le_bytes());
        payload.extend(body);
        payload.extend(0u16.to_le_bytes());
        let mut file = signature.to_vec();
        file.push(8);
        file.extend((payload.len() as u32 + 8).to_le_bytes());
        file.extend(payload);
        file
    }

    #[test]
    fn define_font3_reads_names_metrics_codes_kerning_and_the_outline() {
        let swf = Swf::parse(&swf_with(TAG_DEFINE_FONT3, &define_font3(), b"GFX")).unwrap();
        assert_eq!((swf.version, swf.frame_rate, swf.frame_count), (8, 0x0c00, 1));
        let fonts = swf.fonts().unwrap();
        let font = &fonts[0];
        assert_eq!((font.id, font.name.as_str(), font.bold, font.em, font.language), (7, "Abc", true, EM_FONT3, 1));
        assert_eq!((font.ascent, font.descent, font.leading), (20480, 4000, 100));
        assert_eq!((font.codes.clone(), font.advances.clone()), (vec![0x41], vec![12000]));
        assert_eq!(font.bounds[0], Rect { x_min: 0, x_max: 3, y_min: 0, y_max: 3 });
        assert_eq!(font.kerning_between(0x41, 0x41), -5);
        let outline = font.outline(font.index_of(0x41).unwrap()).unwrap();
        assert_eq!(outline.paths.len(), 1);
        let path = &outline.paths[0];
        assert_eq!((path.start, path.fill0, path.fill1), ((100, -200), 0, 1));
        assert_eq!(path.segments, vec![
            Segment::Line { to: (400, -200) },
            Segment::Line { to: (400, 200) },
            Segment::Curve { control: (250, 200), to: (100, -200) },
        ]);
    }

    #[test]
    fn compressed_signature_and_bad_input_are_handled() {
        let plain = swf_with(TAG_DEFINE_FONT3, &define_font3(), b"GFX");
        let mut encoder = flate2_encode(&plain[8..]);
        let mut packed = b"CFX".to_vec();
        packed.push(8);
        packed.extend(&plain[4..8]);
        packed.append(&mut encoder);
        assert_eq!(Swf::parse(&packed).unwrap().tags, Swf::parse(&plain).unwrap().tags);
        assert_eq!(Swf::parse(b"ABC\x08\0\0\0\0"), Err(SwfError::Signature));
        assert_eq!(Swf::parse(&plain[..plain.len() - 3]), Err(SwfError::Length));
        assert!(Swf::parse(&plain[..5]).is_err());
    }

    fn flate2_encode(body: &[u8]) -> Vec<u8> {
        use std::io::Write;
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::fast());
        encoder.write_all(body).unwrap();
        encoder.finish().unwrap()
    }
}
