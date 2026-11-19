use crate::font::{BitmapFont, GlyphRect, Glyphs};
use crate::ini::IniFile;
use l2_dat::swf::{Font, Outline, Segment, SwfError};
use std::collections::HashMap;

pub const ATLAS_WIDTH: u32 = 2048;
pub const SMALL_PPEM: u32 = 12;
pub const LARGE_PPEM: u32 = 13;
pub const TEXTURE_PREFIX: &str = "gfxfont:";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GfxFontSet {
    pub library: String,
    pub normal: FontFace,
    pub bold: FontFace,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FontFace {
    pub family: String,
    pub bold: bool,
}

impl GfxFontSet {
    pub fn from_localization(ini: &IniFile) -> Option<GfxFontSet> {
        let language = ini.get("LanguageSet", "Language")?;
        let country = ini.get("LanguageSet", language)?;
        let library = ini.get(country, &format!("{country}_FontLib"))?.to_string();
        let map = format!("{country}_FontMap");
        let face = |key: &str| parse_face(ini.get(&map, key)?);
        Some(GfxFontSet { library, normal: face("$NormalFont")?, bold: face("$NormalFont_Bold")? })
    }

    pub fn export_name(&self) -> &str {
        self.library.strip_suffix(".gfx").unwrap_or(&self.library)
    }
}

fn parse_face(value: &str) -> Option<FontFace> {
    let rest = value.trim().strip_prefix('"')?;
    let (family, style) = rest.split_once('"')?;
    let bold = match style.trim().to_ascii_lowercase().as_str() {
        "normal" => false,
        "bold" => true,
        _ => return None,
    };
    Some(FontFace { family: family.to_string(), bold })
}

pub fn select_font<'a>(fonts: &'a [Font], face: &FontFace) -> Option<&'a Font> {
    fonts.iter().find(|font| font.name.eq_ignore_ascii_case(&face.family) && font.bold == face.bold)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GfxAtlas {
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub alpha: Vec<u8>,
}

impl GfxAtlas {
    pub fn rgba(&self) -> Vec<u8> {
        self.alpha.iter().flat_map(|a| [255, 255, 255, *a]).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellMetrics {
    pub baseline: u32,
    pub height: u32,
    pub scale: f32,
}

pub fn cell_metrics(font: &Font, ppem: u32) -> CellMetrics {
    let scale = ppem as f32 / font.em as f32;
    let baseline = (f32::from(font.ascent) * scale).round().max(0.0) as u32;
    let descent = (f32::from(font.descent) * scale).round().max(0.0) as u32;
    CellMetrics { baseline, height: baseline + descent, scale }
}

pub fn atlas_name(font: &Font, ppem: u32) -> String {
    format!("{TEXTURE_PREFIX}{}:{}:{ppem}", font.name.trim_end_matches('\0').to_ascii_lowercase(), if font.bold { "bold" } else { "normal" })
}

pub fn build(font: &Font, ppem: u32, chars: impl IntoIterator<Item = char>) -> Result<(BitmapFont, GfxAtlas), SwfError> {
    let metrics = cell_metrics(font, ppem);
    let mut cells: Vec<(char, u32, Vec<u8>)> = Vec::new();
    let mut seen = std::collections::BTreeSet::new();
    for c in chars {
        let Ok(code) = u16::try_from(u32::from(c)) else { continue };
        if !seen.insert(code) {
            continue;
        }
        let Some(index) = font.index_of(code) else { continue };
        let Some(advance) = font.advance(index) else { continue };
        let width = (f32::from(advance) * metrics.scale).round().max(0.0) as u32;
        if width == 0 || metrics.height == 0 {
            continue;
        }
        let outline = font.outline(index)?;
        cells.push((c, width, rasterize(&outline, width, metrics)));
    }
    let name = atlas_name(font, ppem);
    let mut glyphs = HashMap::new();
    let (mut x, mut y, mut rows) = (0u32, 0u32, 1u32);
    let mut placed = Vec::with_capacity(cells.len());
    for (c, width, _) in &cells {
        if x + width > ATLAS_WIDTH {
            x = 0;
            y += metrics.height;
            rows += 1;
        }
        placed.push((x, y));
        glyphs.insert(*c, GlyphRect { x: x as i32, width: *width, y, height: metrics.height });
        x += width;
    }
    let height = rows * metrics.height;
    let mut alpha = vec![0u8; (ATLAS_WIDTH * height) as usize];
    for ((_, width, pixels), (px, py)) in cells.iter().zip(&placed) {
        for row in 0..metrics.height {
            let from = (row * width) as usize;
            let to = ((py + row) * ATLAS_WIDTH + px) as usize;
            alpha[to..to + *width as usize].copy_from_slice(&pixels[from..from + *width as usize]);
        }
    }
    let font = BitmapFont { atlas: name.clone(), glyphs: Glyphs { tex_width: ATLAS_WIDTH, tex_height: height, glyphs }, line_height: metrics.height };
    Ok((font, GfxAtlas { name, width: ATLAS_WIDTH, height, alpha }))
}

pub fn rasterize(outline: &Outline, width: u32, metrics: CellMetrics) -> Vec<u8> {
    let (w, h) = (width as usize, metrics.height as usize);
    let pad = 4usize;
    let canvas_w = w + pad * 2;
    let mut acc = vec![0f32; canvas_w * h + canvas_w + 2];
    let scale = metrics.scale;
    let map = |p: (i32, i32)| ((p.0 as f32) * scale + pad as f32, (p.1 as f32) * scale + metrics.baseline as f32);
    for path in &outline.paths {
        let mut from = map(path.start);
        for segment in &path.segments {
            match *segment {
                Segment::Line { to } => {
                    let to = map(to);
                    edge(&mut acc, canvas_w, h, from, to, path.fill0, path.fill1);
                    from = to;
                }
                Segment::Curve { control, to } => {
                    let (control, to) = (map(control), map(to));
                    let dx = from.0 - 2.0 * control.0 + to.0;
                    let dy = from.1 - 2.0 * control.1 + to.1;
                    let steps = ((dx * dx + dy * dy).sqrt() * 2.0).sqrt().ceil().clamp(1.0, 24.0) as usize;
                    let mut last = from;
                    for step in 1..=steps {
                        let t = step as f32 / steps as f32;
                        let u = 1.0 - t;
                        let point = (u * u * from.0 + 2.0 * u * t * control.0 + t * t * to.0, u * u * from.1 + 2.0 * u * t * control.1 + t * t * to.1);
                        edge(&mut acc, canvas_w, h, last, point, path.fill0, path.fill1);
                        last = point;
                    }
                    from = to;
                }
            }
        }
        let start = map(path.start);
        if from != start {
            edge(&mut acc, canvas_w, h, from, start, path.fill0, path.fill1);
        }
    }
    let mut out = vec![0u8; w * h];
    for row in 0..h {
        let mut sum = 0f32;
        for col in 0..canvas_w {
            sum += acc[row * canvas_w + col];
            if col >= pad && col < pad + w {
                out[row * w + col - pad] = (sum.abs().min(1.0) * 255.0 + 0.5) as u8;
            }
        }
    }
    out
}

fn edge(acc: &mut [f32], width: usize, height: usize, from: (f32, f32), to: (f32, f32), fill0: u32, fill1: u32) {
    let weight = f32::from(fill1 != 0) - f32::from(fill0 != 0);
    if weight == 0.0 {
        return;
    }
    let clamp_x = |p: (f32, f32)| (p.0.clamp(0.0, width as f32 - 2.0), p.1);
    line(acc, width, height, clamp_x(from), clamp_x(to), weight);
}

fn line(a: &mut [f32], w: usize, h: usize, p0: (f32, f32), p1: (f32, f32), weight: f32) {
    if p0.1 == p1.1 {
        return;
    }
    let (dir, p0, p1) = if p0.1 < p1.1 { (weight, p0, p1) } else { (-weight, p1, p0) };
    if p1.1 <= 0.0 {
        return;
    }
    let dxdy = (p1.0 - p0.0) / (p1.1 - p0.1);
    let mut x = p0.0;
    if p0.1 < 0.0 {
        x -= p0.1 * dxdy;
    }
    let y0 = p0.1.max(0.0) as usize;
    for y in y0..h.min(p1.1.ceil() as usize) {
        let start = y * w;
        let dy = ((y + 1) as f32).min(p1.1) - (y as f32).max(p0.1);
        let xnext = x + dxdy * dy;
        let d = dy * dir;
        let (x0, x1) = if x < xnext { (x, xnext) } else { (xnext, x) };
        let x0floor = x0.floor();
        let x0i = x0floor as usize;
        let x1ceil = x1.ceil();
        let x1i = x1ceil as usize;
        if x1i <= x0i + 1 {
            let xmf = 0.5 * (x + xnext) - x0floor;
            a[start + x0i] += d - d * xmf;
            a[start + x0i + 1] += d * xmf;
        } else {
            let s = 1.0 / (x1 - x0);
            let x0f = x0 - x0floor;
            let a0 = 0.5 * s * (1.0 - x0f) * (1.0 - x0f);
            let x1f = x1 - x1ceil + 1.0;
            let am = 0.5 * s * x1f * x1f;
            a[start + x0i] += d * a0;
            if x1i == x0i + 2 {
                a[start + x0i + 1] += d * (1.0 - a0 - am);
            } else {
                let a1 = s * (1.5 - x0f);
                a[start + x0i + 1] += d * (a1 - a0);
                for xi in x0i + 2..x1i - 1 {
                    a[start + xi] += d * s;
                }
                let a2 = a1 + (x1i - x0i - 3) as f32 * s;
                a[start + x1i - 1] += d * (1.0 - a2 - am);
            }
            a[start + x1i] += d * am;
        }
        x = xnext;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ini::parse_ini;
    use l2_dat::swf::Path;

    fn square(fill0: u32, fill1: u32) -> Outline {
        let (a, b) = (1024, 8192);
        let segments = vec![
            Segment::Line { to: (b, a) },
            Segment::Line { to: (b, -b) },
            Segment::Line { to: (a, -b) },
            Segment::Line { to: (a, a) },
        ];
        Outline { paths: vec![Path { start: (a, a), fill0, fill1, segments }] }
    }

    #[test]
    fn a_filled_square_covers_exactly_its_pixels_whatever_the_winding_side() {
        let metrics = CellMetrics { baseline: 10, height: 12, scale: 1.0 / 1024.0 };
        for outline in [square(0, 1), square(1, 0)] {
            let pixels = rasterize(&outline, 12, metrics);
            let inside: Vec<u8> = (0..12 * 12).filter(|i| (3..9).contains(&(i / 12)) && (1..8).contains(&(i % 12))).map(|i| pixels[i]).collect();
            assert!(inside.iter().all(|a| *a == 255), "{inside:?}");
            assert!(pixels.iter().filter(|a| **a != 0).count() >= inside.len());
        }
        assert!(rasterize(&square(1, 1), 12, metrics).iter().all(|a| *a == 0));
        assert!(rasterize(&square(0, 0), 12, metrics).iter().all(|a| *a == 0));
    }

    #[test]
    fn half_covered_pixels_get_half_coverage() {
        let metrics = CellMetrics { baseline: 4, height: 4, scale: 1.0 };
        let half = Outline {
            paths: vec![Path {
                start: (0, -2),
                fill0: 0,
                fill1: 1,
                segments: vec![Segment::Line { to: (1, -2) }, Segment::Line { to: (1, 0) }, Segment::Line { to: (0, 0) }],
            }],
        };
        let metrics = CellMetrics { scale: 0.5, ..metrics };
        let pixels = rasterize(&half, 2, metrics);
        assert_eq!(pixels[3 * 2], 128);
        assert_eq!(pixels[3 * 2 + 1], 0);
        assert_eq!(pixels[0], 0);
    }

    #[test]
    fn the_font_set_follows_the_language_and_the_font_map() {
        let text = "[LanguageSet]\nLanguage=1\n0=Korea\n1=English\n[English]\nEnglish_FontLib=fonts_en.gfx\n[English_FontMap]\n$NormalFont=\"Tahoma\" Normal\n$NormalFont_Bold=\"Tahoma\" Bold\n[Korea]\nKorea_FontLib=fonts_kr.gfx\n";
        let set = GfxFontSet::from_localization(&parse_ini(text)).unwrap();
        assert_eq!(set.export_name(), "fonts_en");
        assert_eq!(set.normal, FontFace { family: "Tahoma".into(), bold: false });
        assert!(set.bold.bold);
        assert!(GfxFontSet::from_localization(&parse_ini(&text.replace("Language=1", "Language=0"))).is_none());
        assert!(GfxFontSet::from_localization(&parse_ini("")).is_none());
        assert_eq!(parse_face("\"MS Gothic\" Italic"), None);
    }
}
