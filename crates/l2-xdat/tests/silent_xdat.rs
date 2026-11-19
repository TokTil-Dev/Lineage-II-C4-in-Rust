use std::path::PathBuf;

fn silent_dir() -> PathBuf {
    PathBuf::from(std::env::var_os("KILLER_SILENT_DIR").expect("KILLER_SILENT_DIR"))
}

#[test]
#[ignore]
fn the_silent_classic_xdat_decrypts_with_ffedface_and_is_consumed_whole() {
    let path = silent_dir().join("System/interface.xdat");
    let data = std::fs::read(&path).expect("interface.xdat");
    assert_eq!(l2_crypto::detect(&data).unwrap(), l2_crypto::Scheme::FfEdFaCe);
    let xdat = l2_xdat::read_file(&data, &path).expect("interface.xdat inteiro");
    let nodes: usize = xdat.windows.iter().map(|window| window.nodes().len()).sum();
    eprintln!(
        "janelas {} controles {} atalhos {} posições padrão {} cauda {}",
        xdat.windows.len(),
        nodes,
        xdat.shortcuts.len(),
        xdat.wnd_def_pos().count(),
        xdat.tail.len()
    );
    assert_eq!(xdat.windows.len(), 171);
    let decrypted = silent_dir().join("../Compilador/XDAT Descriptografado/interface.xdat");
    if let Ok(theirs) = std::fs::read(&decrypted) {
        let plain = l2_crypto::decrypt(&data, &path).expect("decifrar");
        assert_eq!(&plain[..theirs.len()], &theirs[..], "o decifrado do autor do pacote difere do nosso");
        assert_eq!(plain.len() - theirs.len(), 20);
        eprintln!("decifrado do pacote conferido: {} bytes iguais", theirs.len());
    }
    assert_eq!(xdat.tail.len(), 40);
    assert_eq!(xdat.windows[0].base.name, "AbnormalDebuffsStatusWnd");
    let inventory = xdat.window("InventoryWnd").expect("InventoryWnd");
    let size = inventory.base.size.as_ref().unwrap();
    assert_eq!((size.width, size.height), (566, 400));
    assert!(inventory.find("BtnClose").is_some());
    let textures: Vec<String> = inventory
        .nodes()
        .iter()
        .filter_map(|node| match &node.kind {
            l2_xdat::Kind::Texture(texture) => Some(texture.file.clone()),
            l2_xdat::Kind::Button(button) => Some(button.normal_tex.clone()),
            _ => None,
        })
        .collect();
    assert!(textures.iter().any(|name| name.starts_with("Silent_Tex.")), "{textures:?}");
}

#[test]
#[ignore]
fn the_silent_classic_packages_decrypt_into_ue2_packages() {
    for (name, version) in [("interface.u", 126u16), ("Silent_Tex.u", 123)] {
        let path = silent_dir().join("System").join(name);
        let data = std::fs::read(&path).expect(name);
        assert_eq!(l2_crypto::detect(&data).unwrap(), l2_crypto::Scheme::FfEdFaCe);
        let plain = l2_crypto::decrypt(&data, &path).expect(name);
        assert_eq!(&plain[..4], &[0xC1, 0x83, 0x2A, 0x9E], "{name}");
        assert_eq!(u16::from_le_bytes([plain[4], plain[5]]), version, "{name}");
    }
}
