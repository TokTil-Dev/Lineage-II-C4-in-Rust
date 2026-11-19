use std::path::Path;

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(input), Some(output)) = (args.next(), args.next()) else {
        eprintln!("uso: decrypt <entrada> <saida>");
        std::process::exit(2);
    };
    let data = std::fs::read(&input).expect("leitura");
    let plain = l2_crypto::decrypt(&data, Path::new(&input)).expect("decifragem");
    std::fs::write(&output, plain.as_ref()).expect("escrita");
    println!("{} bytes", plain.len());
}
