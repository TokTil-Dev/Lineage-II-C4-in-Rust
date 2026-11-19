use killer_assets::client::Client;
use l2_dat::xdat::{self, Control, Props};
use std::collections::BTreeSet;
use std::path::PathBuf;

fn collect(controls: &[Control], out: &mut BTreeSet<String>) {
    for control in controls {
        match &control.props {
            Props::Button(b) => out.extend(b.textures.iter().cloned()),
            Props::Texture(t) => {
                out.insert(t.texture.clone());
            }
            Props::StatusBar(g) | Props::InvenWeight(g) => out.extend(g.textures.iter().cloned()),
            Props::BarCtrl { textures, .. } => out.extend(textures.iter().cloned()),
            _ => {}
        }
        collect(&control.children, out);
    }
}

#[test]
#[ignore]
fn high_five_lobby_textures_resolve_in_the_catalog() {
    let root = PathBuf::from(std::env::var_os("KILLER_CLIENT_DIR").expect("KILLER_CLIENT_DIR"));
    let mut client = Client::open(&root).unwrap();
    let body = client.read_system_file("interface.xdat").unwrap();
    let interface = xdat::read_interface(&body).unwrap();
    let mut names = BTreeSet::new();
    for window in &interface.windows {
        if window.head.name.contains("Lobby") || window.head.name.contains("Login") || window.head.name.starts_with("CharacterCreate") {
            names.insert(window.part.back_tex.clone());
            collect(&window.children, &mut names);
        }
    }
    for name in &names {
        if name == "undefined" || name.is_empty() {
            continue;
        }
        println!("{} {name}", if client.catalog().texture_size(name).is_some() { "ok     " } else { "MISSING" });
    }
    for name in [
        "L2UI_CH3.LoginWnd.logon_back", "L2UI_CH3.Etc.inputbox1", "L2UI_CH3.BUTTON.bigbutton", "L2UI_CH3.LobbyWnd.lobby_menuback", "L2UI_CH3.LobbyWnd.select1_back",
        "L2UI_CH3.LobbyWnd.select2_back", "L2UI_CH3.Etc.textbackline", "L2UI_CH3.Etc.textbox1", "L2UI_CH3.Serverselectwnd.Serverselect_back", "L2UI_CH3.ListCtrl.TextSelect",
        "L2UI_CH3.Button.Btn1_normal", "L2UI_CH3.ListCtrl.tab1", "L2UI_ch3.LobbyWnd.lobby3_back", "L2UI_ch3.button.btn1_normal", "L2UI_ch3.playerstatuswnd.ps_hpbar",
        "l2ui.LobbyWnd.left", "L2ui_ch3.Button.PREV1", "L2UI.NWindow.Number", "L2UI.SquareWhite", "L2UI_CH3.Etc.XLargeNumber",
        "L2UI_CT1.Windows.Windows_DF_Drawer_Bg", "L2UI_CT1.Button.Button_DF", "L2UI_CT1.EditBox.EditBox_DF", "L2UI_CT1.Windows.Windows_DF_Small_Vertical_SizeControl_Bg",
    ] {
        println!("C4 {} {name}", if client.catalog().texture_size(name).is_some() { "ok     " } else { "MISSING" });
    }
    for name in ["L2UI_CT1.Windows.Windows_DF_Drawer_Bg", "L2UI_CT1.Button.Button_DF", "L2UI_CT1.Button.Button_DF_Large", "L2UI_CT1.Gauges.Gauge_DF_Large_HP_Left", "L2UI_CT1.Gauges.Gauge_DF_Large_HP_Center"] {
        println!("size {name} {:?}", client.catalog().texture_size(name));
    }
}
