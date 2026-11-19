use killer_assets::client::Client;
use std::path::Path;

fn main() {
    let client_dir = std::env::args().nth(1).unwrap();
    let mut client = Client::open(Path::new(&client_dir)).unwrap();
    let names = [
        "L2UI_ct1.Windows_DF_Large_Bg",
        "L2UI_CT1.Windows.Windows_DF_Small_Vertical_SizeControl_Bg",
        "L2UI_CT1.Windows.Windows_DF_TooltipWnd",
    ];
    for name in names {
        match client.catalog().try_texture_raw(name) {
            Ok(raw) => {
                let mip = raw.mips.first().unwrap();
                println!("{name}: {}x{} format={:?} mips={}", mip.width, mip.height, raw.format, raw.mips.len());
            }
            Err(why) => println!("{name}: ERRO {why}"),
        }
    }
}
