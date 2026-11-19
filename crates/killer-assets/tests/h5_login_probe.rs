use killer_assets::client::Client;
use std::path::PathBuf;
use ue2_package::{ObjectRef, Package};

fn root() -> PathBuf {
    PathBuf::from(std::env::var_os("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR"))
}

#[test]
#[ignore]
fn high_five_login_related_textures_have_a_path_and_a_size() {
    let root = root();
    let mut client = Client::open(&root).unwrap();
    for (file, needles) in [
        ("L2UI_CT1.utx", &["Msn_Bg_LogIn", "MISC_DF_SERVERSELECT_BACK", "login_7thAnivText", "SlideShow_DF_Credit_01", "SlideShow_DF_Credit_10", "CharacterPassword_DF_EditBox"][..]),
        ("L2UI.utx", &["serverback", "loginBack", "CreditFont"][..]),
        ("L2UI_CH3.utx", &["bloodhood_logon", "credit_ch3_01", "msnlogin01"][..]),
    ] {
        let path = root.join("SysTextures").join(file);
        let raw = std::fs::read(&path).unwrap();
        let bytes = l2_crypto::decrypt(&raw, &path).unwrap().into_owned();
        let package = Package::parse(&bytes).unwrap();
        for (index, export) in package.exports().iter().enumerate() {
            let name = package.name(export.name);
            if needles.contains(&name) {
                let full = format!("{}.{}", file.trim_end_matches(".utx"), package.object_path(ObjectRef::Export(index)));
                println!("{full} {:?}", client.catalog().texture_size(&full));
            }
        }
    }
}

#[test]
#[ignore]
fn high_five_login_flow_messages_resolve() {
    let mut client = Client::open(&root()).unwrap();
    let tables = client.tables().unwrap();
    for id in [78u32, 80, 81, 82, 83, 91, 204, 366, 1186, 1187, 1555, 3315, 3322, 3376, 3391] {
        println!("systemmsg {id}: {:?}", tables.systemmsg.get(&id).map(|message| message.text.as_str()));
    }
    for id in [158u32, 159, 160, 884] {
        println!("sysstring {id}: {:?}", tables.sysstring.get(&id));
    }
    let mut hits: Vec<(u32, &str)> = tables
        .sysstring
        .iter()
        .filter(|(_, text)| {
            let lower = text.to_lowercase();
            ["password", "account", "eula", "credit", "server list", "select a server", "agreement"].iter().any(|key| lower.contains(key))
        })
        .map(|(id, text)| (*id, text.as_str()))
        .collect();
    hits.sort_unstable();
    for (id, text) in hits.iter().take(40) {
        println!("sysstring-hit {id}: {text:?}");
    }
}
