use killer_assets::client::Client;
use killer_assets::textures::write_png;
use std::path::Path;

fn main() {
    let client_dir = std::env::args().nth(1).unwrap();
    let out_dir = std::env::args().nth(2).unwrap();
    let mut client = Client::open(Path::new(&client_dir)).unwrap();
    let names = [
        "l2ui_ct1.Gauges.Gauge_DF_Large_HP_Center",
        "l2ui_ct1.Gauges.Gauge_DF_Large_HP_Left",
        "l2ui_ct1.Gauges.Gauge_DF_Large_HP_Right",
        "l2ui_ct1.Gauges.Gauge_DF_Large_HP_bg_Center",
        "l2ui_ct1.Gauges.Gauge_DF_Large_HP_bg_Left",
        "l2ui_ct1.Gauges.Gauge_DF_Large_HP_bg_Right",
    ];
    for name in names {
        let raw = client.catalog().try_texture_raw(name);
        match raw {
            Ok(raw) => {
                let mip = raw.mips.first().unwrap();
                println!("{name}: {}x{} format={:?} mips={}", mip.width, mip.height, raw.format, raw.mips.len());
                let rgba = ue2_assets::decode_rgba(raw.format, &ue2_assets::Mip { width: mip.width, height: mip.height, data: &mip.data }, raw.palette.as_deref()).unwrap();
                let alpha_sum: u64 = rgba.chunks(4).map(|p| p[3] as u64).sum();
                let pixel_count = (mip.width * mip.height) as u64;
                println!("  alpha médio: {:.1} / 255 ({} px)", alpha_sum as f64 / pixel_count as f64, pixel_count);
                let out = format!("{out_dir}/{}.png", name.replace('.', "_"));
                write_png(Path::new(&out), mip.width, mip.height, &rgba).unwrap();
                println!("  salvo em {out}");
            }
            Err(why) => println!("{name}: ERRO {why}"),
        }
    }
}
