use killer_assets::client::Client;
use killer_assets::gfx_lib::read_export;
use killer_ui::font::{BitmapFont, Glyphs};
use killer_ui::gfx_font::{self, GfxAtlas, GfxFontSet, LARGE_PPEM, SMALL_PPEM};
use killer_ui::ini::parse_ini;
use l2_dat::swf::Swf;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(std::env::var_os("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR"))
}

fn blit(canvas: &mut [u8], canvas_w: usize, font: &BitmapFont, atlas: &GfxAtlas, x: usize, y: usize, text: &str) -> usize {
    let mut pen = x;
    for c in text.chars() {
        let Some(g) = font.glyph(c) else { continue };
        for row in 0..g.height as usize {
            for col in 0..g.width as usize {
                let a = u32::from(atlas.alpha[(g.y as usize + row) * atlas.width as usize + g.x as usize + col]);
                let at = ((y + row) * canvas_w + pen + col) * 4;
                for channel in 0..3 {
                    canvas[at + channel] = ((u32::from(canvas[at + channel]) * (255 - a) + 255 * a) / 255) as u8;
                }
            }
        }
        pen += g.width as usize;
    }
    pen
}

#[test]
#[ignore]
fn the_english_gfx_library_holds_tahoma_normal_and_bold_and_renders() {
    let root = root();
    let mut client = Client::open(&root).unwrap();
    let localization = parse_ini(&client.read_system_text("localization.ini").unwrap());
    let set = GfxFontSet::from_localization(&localization).unwrap();
    assert_eq!(set.library, "fonts_en.gfx");
    assert_eq!((set.normal.family.as_str(), set.normal.bold, set.bold.bold), ("Tahoma", false, true));
    let bytes = read_export(&root.join("SysTextures/ui.ugx"), set.export_name()).unwrap();
    assert_eq!(&bytes[..3], b"CFX");
    let swf = Swf::parse(&bytes).unwrap();
    assert_eq!((swf.version, swf.frame_count), (8, 1));
    let fonts = swf.fonts().unwrap();
    assert_eq!(fonts.len(), 2);
    for font in &fonts {
        assert_eq!((font.glyph_count(), font.codes.len(), font.advances.len(), font.bounds.len()), (54048, 54048, 54048, 54048));
        assert_eq!((font.ascent, font.descent, font.leading, font.em), (20500, 4240, 4260, 20480));
    }
    let normal = gfx_font::select_font(&fonts, &set.normal).unwrap();
    let bold = gfx_font::select_font(&fonts, &set.bold).unwrap();
    assert_eq!((normal.id, bold.id), (4, 1));

    let small_gly = Glyphs::parse(&client.read_system_file("SmallFont-e.gly").unwrap()).unwrap();
    let mut same = 0;
    let mut total = 0;
    for code in 0x21u16..0x7f {
        let c = char::from_u32(u32::from(code)).unwrap();
        let index = normal.index_of(code).unwrap();
        let metrics = gfx_font::cell_metrics(normal, 12);
        let width = (f32::from(normal.advance(index).unwrap()) * metrics.scale).round() as u32;
        total += 1;
        same += u32::from(small_gly.glyphs[&c].width == width);
    }
    assert!(same * 2 < total, "o .gly bateu em {same}/{total}");

    let started = std::time::Instant::now();
    let (small, small_atlas) = gfx_font::build(normal, SMALL_PPEM, normal.codes.iter().filter_map(|c| char::from_u32(u32::from(*c)))).unwrap();
    let (large, large_atlas) = gfx_font::build(bold, LARGE_PPEM, bold.codes.iter().filter_map(|c| char::from_u32(u32::from(*c)))).unwrap();
    eprintln!("atlas pequeno {}x{} ({} glifos), grande {}x{}, {:?}", small_atlas.width, small_atlas.height, small.glyphs.glyphs.len(), large_atlas.width, large_atlas.height, started.elapsed());
    assert!(small.has_glyph('A') && small.has_glyph(' ') && large.has_glyph('g'));
    assert!(small.measure("Lineage") > 30);

    let strings = l2_dat::sysstring::read(&client.read_system_file("Sysstring-e.dat").unwrap()).unwrap();
    let phrase = strings.iter().map(|s| s.text.as_str()).find(|t| t.len() > 40 && t.len() < 70 && t.is_ascii() && !t.contains('\\') && !t.contains('$')).unwrap();
    let lines = [
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string(),
        "abcdefghijklmnopqrstuvwxyz 0123456789".to_string(),
        "!\"#%&'()*+,-./:;<=>?@[]_ \u{00e1}\u{00e9}\u{00ed}\u{00f3}\u{00fa}\u{00e7}\u{00f1}\u{00fc} \u{0416}\u{0434}\u{03a9}".to_string(),
        phrase.to_string(),
    ];
    let (w, h) = (640usize, 8 + lines.len() * 2 * 20);
    let mut canvas = vec![0u8; w * h * 4];
    for px in canvas.chunks_exact_mut(4) {
        px.copy_from_slice(&[24, 28, 40, 255]);
    }
    for (i, line) in lines.iter().enumerate() {
        blit(&mut canvas, w, &small, &small_atlas, 8, 8 + i * 40, line);
        blit(&mut canvas, w, &large, &large_atlas, 8, 8 + i * 40 + 20, line);
    }
    let out = std::env::var_os("KILLER_GFX_PNG").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/renders/h5-gfx-fonts.png"));
    killer_assets::textures::write_png(&out, w as u32, h as u32, &canvas).unwrap();
    let lit = canvas.chunks_exact(4).filter(|p| p[0] > 120).count();
    assert!(lit > 2000, "pixels acesos: {lit}");
}
