use std::path::Path;

use l2_uscript::vm::{ObjId, Value, Vm};

use super::host_impl::{HostEvent, ScriptHost};
use super::{UiApi, WindowScript};

pub const SCRIPT_PACKAGES: [&str; 4] = ["Core", "Engine", "NWindow", "interface"];

enum Backend {
    Hand(Option<Box<dyn WindowScript + Send>>),
    Vm(ObjId),
}

pub const WINDOW_SCRIPTS: [&str; 13] = [
    "ChatWnd",
    "SystemMsgWnd",
    "StatusWnd",
    "TargetStatusWnd",
    "PartyWnd",
    "AbnormalStatusWnd",
    "DetailStatusWnd",
    "InventoryWnd",
    "MenuWnd",
    "ShortcutWnd",
    "RadarWnd",
    "RadarContainerWnd",
    "MinimapWnd",
];

pub struct Runtime {
    pub host: ScriptHost,
    scripts: Vec<(String, Backend)>,
    vm: Option<Vm>,
    pub loaded: bool,
    pub errors: Vec<String>,
}

fn script_for(name: &str) -> Option<Box<dyn WindowScript + Send>> {
    Some(match name {
        "ChatWnd" => Box::new(super::chat_wnd::Script::default()),
        "SystemMsgWnd" => Box::new(super::system_msg_wnd::Script),
        "StatusWnd" => Box::new(super::status_wnd::Script::default()),
        "TargetStatusWnd" => Box::new(super::target_status_wnd::Script::default()),
        "PartyWnd" => Box::new(super::party_wnd::Script::default()),
        "AbnormalStatusWnd" => Box::new(super::abnormal_status_wnd::Script::default()),
        "DetailStatusWnd" => Box::new(super::detail_status_wnd::Script::default()),
        "InventoryWnd" => Box::new(super::inventory_wnd::Script::default()),
        "MenuWnd" => Box::new(super::menu_wnd::Script::default()),
        "ShortcutWnd" => Box::new(super::shortcut_wnd::Script::default()),
        "RadarWnd" => Box::new(super::radar_wnd::Script::default()),
        "RadarContainerWnd" => Box::new(super::radar_container_wnd::Script),
        "MinimapWnd" => Box::new(super::minimap_wnd::Script::default()),
        _ => return None,
    })
}

impl Default for Runtime {
    fn default() -> Self {
        Self::standard()
    }
}

impl Runtime {
    pub fn standard() -> Runtime {
        let scripts = WINDOW_SCRIPTS.iter().filter_map(|name| script_for(name).map(|script| (name.to_string(), Backend::Hand(Some(script))))).collect();
        Runtime { host: ScriptHost::default(), scripts, vm: None, loaded: false, errors: Vec::new() }
    }

    pub fn load_packages(system: &Path) -> Result<Vm, String> {
        let mut packages = Vec::new();
        for name in SCRIPT_PACKAGES {
            let path = system.join(format!("{name}.u"));
            let raw = std::fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
            let plain = l2_crypto::decrypt(&raw, &path).map_err(|error| format!("{}: {error}", path.display()))?;
            let script = l2_uscript::Script::parse(&plain).map_err(|error| format!("{}: {error}", path.display()))?;
            packages.push((name.to_string(), script));
        }
        Ok(Vm::load(packages))
    }

    pub fn from_vm(mut vm: Vm, windows: &[(String, String)]) -> Runtime {
        let mut scripts = Vec::new();
        let mut errors = Vec::new();
        for (window, class) in windows {
            match vm.instantiate(class) {
                Ok(object) => scripts.push((window.clone(), Backend::Vm(object))),
                Err(error) => errors.push(format!("{window}: {class}: {}", error.message)),
            }
        }
        let mut host = ScriptHost::default();
        for (window, backend) in &scripts {
            if let Backend::Vm(object) = backend {
                host.script_objects.insert(window.clone(), object.0);
            }
        }
        for (window, class) in windows {
            if let Some(object) = host.script_objects.get(window).copied() {
                host.script_objects.entry(class.clone()).or_insert(object);
            }
        }
        Runtime { host, scripts, vm: Some(vm), loaded: false, errors }
    }

    pub fn is_vm(&self) -> bool {
        self.vm.is_some()
    }

    pub fn windows(&self) -> impl Iterator<Item = &str> {
        self.scripts.iter().map(|(name, _)| name.as_str())
    }

    fn with(&mut self, window: &str, call: impl FnOnce(&mut Box<dyn WindowScript + Send>, &mut dyn UiApi)) -> bool {
        let Some(index) = self.scripts.iter().position(|(name, _)| name == window) else { return false };
        let Backend::Hand(slot) = &mut self.scripts[index].1 else { return false };
        let Some(mut script) = slot.take() else { return false };
        let previous = std::mem::replace(&mut self.host.current, window.to_string());
        call(&mut script, &mut self.host);
        self.host.current = previous;
        if let Backend::Hand(slot) = &mut self.scripts[index].1 {
            *slot = Some(script);
        }
        true
    }

    fn run(&mut self, window: &str, function: &str, args: Vec<Value>) -> bool {
        let Some(index) = self.scripts.iter().position(|(name, _)| name == window) else { return false };
        let object = match self.scripts[index].1 {
            Backend::Vm(object) => object,
            Backend::Hand(_) => return false,
        };
        let Some(vm) = self.vm.as_mut() else { return false };
        let previous = std::mem::replace(&mut self.host.current, window.to_string());
        if let Err(error) = vm.event(&mut self.host, object, function, args) {
            self.errors.push(format!("{window}.{function}: {} em {:?} +{:#x}", error.message, error.function, error.offset));
        }
        self.host.current = previous;
        true
    }

    fn call_script(&mut self, window: &str, function: &str, args: Vec<Value>, hand: impl FnOnce(&mut Box<dyn WindowScript + Send>, &mut dyn UiApi)) -> bool {
        if self.vm.is_some() { self.run(window, function, args) } else { self.with(window, hand) }
    }

    pub fn load(&mut self) {
        if self.loaded {
            return;
        }
        let names: Vec<String> = self.scripts.iter().map(|(name, _)| name.clone()).collect();
        for name in names {
            self.call_script(&name, "OnLoad", Vec::new(), |script, api| script.on_load(api));
        }
        self.loaded = true;
    }

    pub fn show(&mut self, window: &str) -> bool {
        self.call_script(window, "OnShow", Vec::new(), |script, api| script.on_show(api))
    }

    pub fn hide(&mut self, window: &str) -> bool {
        self.call_script(window, "OnHide", Vec::new(), |script, api| script.on_hide(api))
    }

    pub fn default_position(&mut self, window: &str) -> bool {
        self.call_script(window, "OnDefaultPosition", Vec::new(), |script, api| script.on_default_position(api))
    }

    pub fn click(&mut self, window: &str, button: &str) -> bool {
        self.call_script(window, "OnClickButton", vec![Value::Str(button.to_string())], |script, api| script.on_click_button(api, button))
    }

    pub fn timer(&mut self, window: &str, timer_id: i32) -> bool {
        self.call_script(window, "OnTimer", vec![Value::Int(timer_id)], |script, api| script.on_timer(api, timer_id))
    }

    pub fn tick(&mut self) {
        let names: Vec<String> = self.scripts.iter().map(|(name, _)| name.clone()).collect();
        for name in names {
            self.call_script(&name, "OnTick", Vec::new(), |script, api| script.on_tick(api));
        }
    }

    pub fn enter_state(&mut self, window: &str, previous: &str) -> bool {
        self.call_script(window, "OnEnterState", vec![Value::Name(previous.to_string())], |script, api| script.on_enter_state(api, previous))
    }

    pub fn event(&mut self, event_id: i32, param: &str) -> usize {
        let listeners: Vec<String> = self.host.events.listeners(event_id).to_vec();
        let mut delivered = 0;
        for window in listeners {
            let args = vec![Value::Int(event_id), Value::Str(param.to_string())];
            if self.call_script(&window, "OnEvent", args, |script, api| script.on_event(api, event_id, param)) {
                delivered += 1;
            }
        }
        delivered
    }

    pub fn drain(&mut self) -> Vec<HostEvent> {
        let mut external = Vec::new();
        let mut guard = 0;
        while !self.host.queue.is_empty() && guard < 64 {
            guard += 1;
            let pending = std::mem::take(&mut self.host.queue);
            for item in pending {
                match item {
                    HostEvent::ShowWindow(ref window) => {
                        self.show(window);
                        external.push(item);
                    }
                    HostEvent::HideWindow(ref window) => {
                        self.hide(window);
                        external.push(item);
                    }
                    HostEvent::ExecuteEvent(id, ref param) => {
                        self.event(id, param);
                    }
                    HostEvent::TabTop(..) => external.push(item),
                    other => external.push(other),
                }
            }
        }
        external
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interlude_script::UserInfo;

    fn packages_dir() -> Option<std::path::PathBuf> {
        let dir = std::env::var_os("KILLER_USCRIPT_DIR")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| std::path::PathBuf::from("/private/tmp/claude-501/-Users-wender-projetos-lineage2-killer-web/e7251fea-f786-4501-b8bd-6aa35ac88ddc/scratchpad/u"));
        dir.join("interface.u").is_file().then_some(dir)
    }

    #[test]
    fn the_bytecode_backend_runs_the_real_status_and_chat_scripts_through_the_same_host() {
        let Some(dir) = packages_dir() else { return };
        let vm = Runtime::load_packages(&dir).unwrap();
        let windows = vec![("StatusWnd".to_string(), "StatusWnd".to_string()), ("ChatWnd".to_string(), "ChatWnd".to_string())];
        let mut runtime = Runtime::from_vm(vm, &windows);
        assert!(runtime.errors.is_empty(), "{:?}", runtime.errors);
        runtime.load();
        assert_eq!(runtime.host.events.listeners(70), ["StatusWnd"]);
        assert!(runtime.host.events.listeners(540).contains(&"ChatWnd".to_string()));
        assert_eq!(runtime.host.chat_scroll_bars.get("ChatWnd.NormalChat"), Some(&(5, 10, -25)));
        runtime.host.player = Some(UserInfo { id: 7, cur_hp: 400, max_hp: 900, ..Default::default() });
        runtime.event(200, "ServerID=7 MaxHP=900 ");
        assert_eq!(runtime.host.overlay.point("StatusWnd.HPBar"), Some((400, 900)));
        runtime.event(540, "Type=0 Msg=\u{7f}Killer: oi\u{7f} ColorR=255 ColorG=255 ColorB=255 ");
        assert!(runtime.host.chat_lines.values().any(|lines| lines.iter().any(|(_, text)| text == "Killer: oi")), "{:?} {:?} {:?}", runtime.host.chat_lines, runtime.errors, runtime.host.unhandled_natives);
        assert!(runtime.errors.is_empty(), "{:?}", runtime.errors);
    }

    #[test]
    fn loading_registers_every_window_in_script_order_and_events_reach_them_in_that_order() {
        let mut runtime = Runtime::standard();
        runtime.load();
        assert_eq!(runtime.host.events.listeners(70), ["StatusWnd"]);
        let listeners_180 = runtime.host.events.listeners(180).to_vec();
        assert_eq!(listeners_180.first().map(String::as_str), Some("StatusWnd"));
        assert!(listeners_180.contains(&"DetailStatusWnd".to_string()));
        assert!(runtime.host.events.listeners(540).contains(&"ChatWnd".to_string()));
        runtime.host.player = Some(UserInfo { id: 7, cur_hp: 400, max_hp: 900, cur_mp: 200, max_mp: 300, cur_cp: 100, max_cp: 100, ..Default::default() });
        let delivered = runtime.event(200, "ServerID=7 MaxHP=900 ");
        assert!(delivered >= 1);
        assert_eq!(runtime.host.overlay.point("StatusWnd.HPBar"), Some((400, 900)));
        runtime.host.queue.push(HostEvent::ExecuteEvent(200, "ServerID=7 MaxHP=950 ".into()));
        let external = runtime.drain();
        assert!(!external.iter().any(|item| matches!(item, HostEvent::ExecuteEvent(..))));
        assert!(runtime.host.queue.is_empty());
        assert_eq!(runtime.host.overlay.point("StatusWnd.HPBar"), Some((400, 900)));
    }
}
