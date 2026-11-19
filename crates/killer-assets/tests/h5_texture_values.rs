use killer_assets::client::Client;
use l2_dat::xdat::{self, Control, Props, Size};
use std::collections::BTreeMap;
use std::path::PathBuf;

fn walk(window: &str, controls: &[Control], client: &mut Client, counts: &mut BTreeMap<(i32, i32), usize>, samples: &mut BTreeMap<(i32, i32), Vec<String>>) {
    for control in controls {
        if let Props::Texture(t) = &control.props {
            let key = (t.values[0], t.values[1]);
            *counts.entry(key).or_default() += 1;
            let size = match &control.head.size {
                Some(Size::Fixed { width, height }) => format!("{width}x{height}"),
                Some(Size::Scaled { scale, extent, .. }) => format!("scaled{scale:?}{extent:?}"),
                None => "none".into(),
            };
            let real = client.catalog().texture_size(&t.texture);
            let entry = samples.entry(key).or_default();
            if entry.len() < 14 {
                let f = |v: i32| f32::from_bits(v as u32);
                entry.push(format!(
                    "{window}.{} ctrl={size} tex={:?} {} v2={} v3={} v4={} v5={} v6={} v7={} auto_rotate={} alpha_mask={}",
                    control.head.name,
                    real,
                    t.texture,
                    t.values[2],
                    f(t.values[3]),
                    f(t.values[4]),
                    f(t.values[5]),
                    t.values[6],
                    t.values[7],
                    t.auto_rotate,
                    t.alpha_mask
                ));
            }
        }
        walk(window, &control.children, client, counts, samples);
    }
}

#[test]
#[ignore]
fn high_five_texture_values_against_real_sizes() {
    let root = PathBuf::from(std::env::var_os("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR"));
    let mut client = Client::open(&root).unwrap();
    let body = client.read_system_file("interface.xdat").unwrap();
    let interface = xdat::read_interface(&body).unwrap();
    let mut counts = BTreeMap::new();
    let mut samples = BTreeMap::new();
    for window in &interface.windows {
        walk(&window.head.name, &window.children, &mut client, &mut counts, &mut samples);
    }
    for (key, count) in &counts {
        println!("== v0={} v1={} n={count}", key.0, key.1);
        for line in &samples[key] {
            println!("   {line}");
        }
    }
}
