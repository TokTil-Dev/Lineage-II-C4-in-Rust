use l2_uscript::vm::{Error, Host, Value, Vm};
use l2_uscript::{Kind, ObjectRef, Script};
use std::collections::BTreeMap;
use std::path::PathBuf;

const SCRATCH: &str = "/private/tmp/claude-501/-Users-wender-projetos-lineage2-killer-web/e7251fea-f786-4501-b8bd-6aa35ac88ddc/scratchpad/u";
const PARAM_DELIMITER: char = '\u{7f}';

fn package_dir() -> Option<PathBuf> {
    let candidates = [std::env::var("KILLER_USCRIPT_DIR").ok(), Some(SCRATCH.to_string())];
    candidates.into_iter().flatten().map(PathBuf::from).find(|dir| dir.join("interface.u").is_file())
}

fn load_vm() -> Option<Vm> {
    let dir = package_dir()?;
    let mut packages = Vec::new();
    for name in ["Core", "Engine", "NWindow", "interface"] {
        let data = std::fs::read(dir.join(format!("{name}.u"))).ok()?;
        packages.push((name.to_string(), Script::parse(&data).expect("pacote")));
    }
    Some(Vm::load(packages))
}

fn parse_value<'a>(param: &'a str, name: &str) -> Option<&'a str> {
    if name.is_empty() || param.is_empty() {
        return None;
    }
    let mut from = 0;
    loop {
        let found = from + param[from..].find(name)?;
        let after = found + name.len();
        if param[after..].starts_with('=') {
            let rest = &param[after + 1..];
            let (body, delimiter) = match rest.strip_prefix(PARAM_DELIMITER) {
                Some(body) => (body, PARAM_DELIMITER),
                None => (rest, ' '),
            };
            let end = body.find(delimiter).unwrap_or(body.len());
            return Some(&body[..end]);
        }
        from = after;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Call {
    class: String,
    function: String,
    this: Option<Value>,
    args: Vec<Value>,
}

#[derive(Default)]
struct RecordingHost {
    calls: Vec<Call>,
    unknown: BTreeMap<String, usize>,
    options: BTreeMap<(String, String), Value>,
}

impl RecordingHost {
    fn names(&self) -> Vec<String> {
        self.calls.iter().map(|call| call.function.clone()).collect()
    }

    fn calls_named(&self, function: &str) -> Vec<&Call> {
        self.calls.iter().filter(|call| call.function == function).collect()
    }
}

impl Host for RecordingHost {
    fn call_native(
        &mut self,
        class: &str,
        function: &str,
        this: Option<&Value>,
        args: &mut [Value],
    ) -> Result<Value, Error> {
        self.calls.push(Call {
            class: class.to_string(),
            function: function.to_string(),
            this: this.cloned(),
            args: args.to_vec(),
        });
        Ok(match function {
            "GetHandle" => Value::Handle(args.first().map_or_else(String::new, Value::as_str)),
            "ParseInt" => {
                let found = parse_value(&args[0].as_str(), &args[1].as_str()).map(|v| v.to_string());
                match found {
                    Some(text) => {
                        args[2] = Value::Int(l2_uscript::vm::app_atoi(&text));
                        Value::Bool(true)
                    }
                    None => Value::Bool(false),
                }
            }
            "ParseString" => {
                let found = parse_value(&args[0].as_str(), &args[1].as_str()).map(|v| v.to_string());
                match found {
                    Some(text) => {
                        args[2] = Value::Str(text);
                        Value::Bool(true)
                    }
                    None => Value::Bool(false),
                }
            }
            "ParseFloat" => {
                let found = parse_value(&args[0].as_str(), &args[1].as_str()).map(|v| v.to_string());
                match found {
                    Some(text) => {
                        args[2] = Value::Float(l2_uscript::vm::app_atof(&text));
                        Value::Bool(true)
                    }
                    None => Value::Bool(false),
                }
            }
            "GetOptionBool" | "GetOptionInt" | "GetOptionString" => {
                let key = (args[0].as_str(), args[1].as_str());
                self.options.get(&key).cloned().unwrap_or_default()
            }
            "GetSystemString" => Value::Str(format!("sys{}", args.first().map_or(0, Value::as_int))),
            "GetSystemMessage" => Value::Str(format!("msg{}", args.first().map_or(0, Value::as_int))),
            "IsShowWindow" | "IsChecked" | "IsEnableWindow" | "IsFocused" => Value::Bool(false),
            "GetPlayerInfo" | "GetTargetInfo" | "GetUserInfo" | "GetPetInfo" | "GetSkillInfo" | "GetItemInfo" => {
                Value::Bool(false)
            }
            _ => {
                if class != "UIAPI_WINDOW"
                    && !(class.starts_with("UIAPI_")
                        || class.ends_with("Handle")
                        || class.ends_with("API")
                        || class == "UIScript"
                        || class == "UIEventManager"
                        || class == "Core.Object")
                {
                    *self.unknown.entry(format!("{class}.{function}")).or_default() += 1;
                }
                Value::None
            }
        })
    }
}

fn arg_ints(call: &Call) -> Vec<i32> {
    call.args.iter().map(Value::as_int).collect()
}

#[test]
fn status_wnd_on_load_registers_the_eight_events_in_order() {
    let Some(mut vm) = load_vm() else { return };
    let mut host = RecordingHost::default();
    let status = vm.instantiate("StatusWnd").expect("StatusWnd");
    vm.call(&mut host, status, "OnLoad", Vec::new()).expect("OnLoad");
    let ids: Vec<i32> = host.calls_named("RegisterEvent").iter().map(|call| call.args[0].as_int()).collect();
    assert_eq!(ids, [70, 180, 190, 200, 210, 220, 230, 240]);
    assert_eq!(host.calls.len(), 8);
    assert!(host.calls.iter().all(|call| call.class == "UIEventManager"), "{:?}", host.calls[0]);
}

#[test]
fn chat_wnd_on_load_registers_events_takes_handles_and_places_the_scroll_bars() {
    let Some(mut vm) = load_vm() else { return };
    let mut host = RecordingHost::default();
    let chat = vm.instantiate("ChatWnd").expect("ChatWnd");
    vm.call(&mut host, chat, "OnLoad", Vec::new()).expect("OnLoad");
    let ids: Vec<i32> = host.calls_named("RegisterEvent").iter().map(|call| call.args[0].as_int()).collect();
    assert_eq!(ids, [540, 1500, 550, 560, 570, 571, 572]);
    let handles: Vec<String> = host.calls_named("GetHandle").iter().map(|call| call.args[0].as_str()).collect();
    assert!(handles.contains(&"ChatWnd.NormalChat".to_string()), "{handles:?}");
    assert!(handles.contains(&"ChatWnd.ChatTabCtrl".to_string()), "{handles:?}");
    let bars: Vec<(Option<Value>, Vec<i32>)> =
        host.calls_named("SetScrollBarPosition").iter().map(|call| (call.this.clone(), arg_ints(call))).collect();
    assert_eq!(bars.len(), 5, "{bars:?}");
    assert_eq!(bars[0], (Some(Value::Handle("ChatWnd.NormalChat".into())), vec![5, 10, -25]));
    assert!(bars.iter().all(|(_, args)| args == &[5, 10, -25]), "{bars:?}");
    let state = host.calls_named("RegisterState");
    assert_eq!(state[0].args.iter().map(Value::as_str).collect::<Vec<_>>(), ["ChatWnd", "OlympiadObserverState"]);
    let sections = vm.get_prop(chat, "m_sectionName").cloned().expect("m_sectionName");
    assert_eq!(
        sections,
        Value::Array(
            ["entire_tab", "pledge_tab", "party_tab", "market_tab", "ally_tab"].map(|s| Value::Str(s.into())).to_vec()
        )
    );
    let Value::Array(filters) = vm.get_prop(chat, "m_filterInfo").cloned().expect("m_filterInfo") else { panic!() };
    assert_eq!(filters.len(), 6);
}

#[test]
fn chat_message_540_is_parsed_and_added_to_the_windows_whose_filter_accepts_it() {
    let Some(mut vm) = load_vm() else { return };
    let mut host = RecordingHost::default();
    let chat = vm.instantiate("ChatWnd").expect("ChatWnd");
    vm.call(&mut host, chat, "OnLoad", Vec::new()).expect("OnLoad");
    host.calls.clear();
    let param = format!("Type=1 Msg={PARAM_DELIMITER}oi mundo{PARAM_DELIMITER} ColorR=10 ColorG=20 ColorB=30 ");
    vm.call(&mut host, chat, "OnEvent", vec![Value::Int(540), Value::Str(param)]).expect("OnEvent");
    let parsed: Vec<String> =
        host.calls.iter().filter(|c| c.function.starts_with("Parse")).map(|c| c.args[1].as_str()).collect();
    assert_eq!(parsed, ["Type", "Msg", "ColorR", "ColorG", "ColorB"]);
    let added: Vec<(Option<Value>, String, Value)> = host
        .calls_named("AddString")
        .iter()
        .map(|call| (call.this.clone(), call.args[0].as_str(), call.args[1].clone()))
        .collect();
    assert!(!added.is_empty(), "{:?}", host.names());
    for (this, text, color) in &added {
        assert!(matches!(this, Some(Value::Handle(_))), "{this:?}");
        assert_eq!(text, "oi mundo");
        assert_eq!(color.field("R"), Value::Byte(10));
        assert_eq!(color.field("G"), Value::Byte(20));
        assert_eq!(color.field("B"), Value::Byte(30));
        assert_eq!(color.field("A"), Value::Byte(255));
    }
    assert!(!host.names().contains(&"HandleIMEStatusChange".to_string()));
    host.calls.clear();
    vm.call(&mut host, chat, "OnEvent", vec![Value::Int(1500), Value::Str(String::new())]).expect("OnEvent 1500");
    assert!(
        host.names()
            .iter()
            .any(|n| n == "GetCurrentIMELang" || n == "SetTexture" || n == "GetHandle" || n == "GetOptionBool"),
        "{:?}",
        host.names()
    );
}

#[test]
fn status_wnd_events_parse_the_server_values_and_drive_the_bars() {
    let Some(mut vm) = load_vm() else { return };
    let mut host = RecordingHost::default();
    let status = vm.instantiate("StatusWnd").expect("StatusWnd");
    vm.call(&mut host, status, "OnLoad", Vec::new()).expect("OnLoad");
    host.calls.clear();
    vm.call(
        &mut host,
        status,
        "OnEvent",
        vec![Value::Int(70), Value::Str("Type=1 Duration=100 Ticks=5 Amount=1.5 ".into())],
    )
    .expect("OnEvent 70");
    let parsed: Vec<String> =
        host.calls.iter().filter(|c| c.function.starts_with("Parse")).map(|c| c.args[1].as_str()).collect();
    assert_eq!(parsed, ["Type", "Duration", "Ticks", "Amount"], "{:?}", host.names());
    let regen = host.calls_named("SetRegenInfo");
    assert_eq!(regen.len(), 1, "{:?}", host.names());
    assert_eq!(regen[0].args.iter().map(Value::as_str).collect::<Vec<_>>(), ["StatusWnd.HPBar", "100", "5", "1.50"]);
    host.calls.clear();
    vm.call(&mut host, status, "OnEvent", vec![Value::Int(200), Value::Str("ServerID=7 MaxHP=900 ".into())])
        .expect("OnEvent 200");
    assert!(host.calls.iter().any(|c| c.function == "ParseInt"), "{:?}", host.names());
}

#[test]
fn inventory_wnd_loads_and_reacts_to_its_events_without_vm_errors() {
    let Some(mut vm) = load_vm() else { return };
    let mut host = RecordingHost::default();
    let inventory = vm.instantiate("InventoryWnd").expect("InventoryWnd");
    vm.call(&mut host, inventory, "OnLoad", Vec::new()).expect("OnLoad");
    let ids: Vec<i32> = host.calls_named("RegisterEvent").iter().map(|call| call.args[0].as_int()).collect();
    assert!(!ids.is_empty(), "{:?}", host.names());
    for id in ids {
        vm.call(&mut host, inventory, "OnEvent", vec![Value::Int(id), Value::Str("ServerID=1 Type=1 ".into())])
            .unwrap_or_else(|error| panic!("evento {id}: {error}"));
    }
    vm.call(&mut host, inventory, "OnShow", Vec::new()).expect("OnShow");
    vm.call(&mut host, inventory, "OnHide", Vec::new()).expect("OnHide");
}

#[test]
fn every_parameterless_function_of_every_interface_class_runs() {
    let Some(mut vm) = load_vm() else { return };
    assert!(vm.default_errors.is_empty(), "{:?}", &vm.default_errors[..vm.default_errors.len().min(10)]);
    let interface = vm.packages.iter().position(|p| p.name == "interface").expect("interface");
    let classes: Vec<(String, usize)> = vm.packages[interface]
        .script
        .objects
        .iter()
        .filter(|object| matches!(object.kind, Kind::Class(_)))
        .map(|object| (object.name.clone(), object.export))
        .collect();
    let mut host = RecordingHost::default();
    let mut ran = 0usize;
    let mut failed: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut total_functions = 0usize;
    for (name, export) in classes {
        let id = vm.instantiate(&name).unwrap_or_else(|error| panic!("{name}: {error}"));
        let functions: Vec<(String, bool)> = vm.packages[interface]
            .script
            .functions(ObjectRef::Export(export))
            .iter()
            .filter_map(|object| match &object.kind {
                Kind::Function(function) => Some((object.name.clone(), function.flags & 0x400 == 0)),
                _ => None,
            })
            .collect();
        for (function, scripted) in functions {
            if !scripted {
                continue;
            }
            let global = l2_uscript::vm::GlobalRef { package: interface, export };
            let Some(target) = vm.find_function(global, &function, None) else { continue };
            let params = vm.functions.get(&target).map_or(0, |meta| meta.params.len());
            if params != 0 {
                continue;
            }
            total_functions += 1;
            match vm.call(&mut host, id, &function, Vec::new()) {
                Ok(_) => ran += 1,
                Err(error) => failed.entry(error.message.clone()).or_default().push(format!("{name}.{function}")),
            }
        }
    }
    let summary: Vec<String> = failed
        .iter()
        .map(|(message, owners)| format!("{message}: {} ({})", owners.len(), owners[..owners.len().min(3)].join(", ")))
        .collect();
    eprintln!("funções sem parâmetros: {total_functions}, executadas: {ran}, falhas: {}", total_functions - ran);
    for line in &summary {
        eprintln!("  {line}");
    }
    eprintln!("nativas do host desconhecidas (classe fora de UIAPI/Handle/API): {}", host.unknown.len());
    for (name, count) in host.unknown.iter().take(40) {
        eprintln!("  {name} ×{count}");
    }
    let core_missing: Vec<&String> = failed
        .keys()
        .filter(|message| {
            message.contains("sem implementação")
                || message.contains("sem suporte")
                || message.contains("sem definição")
        })
        .collect();
    assert!(core_missing.is_empty(), "{core_missing:?}");
    assert!(ran * 10 >= total_functions * 9, "{summary:?}");
}

#[test]
fn survey_of_natives_used_by_the_interface_package() {
    let Some(vm) = load_vm() else { return };
    let interface = vm.packages.iter().position(|p| p.name == "interface").expect("interface");
    let mut host_natives: BTreeMap<String, usize> = BTreeMap::new();
    let mut core_natives: BTreeMap<String, usize> = BTreeMap::new();
    let mut unresolved: BTreeMap<String, usize> = BTreeMap::new();
    let mut tokens: BTreeMap<String, usize> = BTreeMap::new();
    for object in &vm.packages[interface].script.objects {
        let Kind::Function(function) = &object.kind else { continue };
        function.body.script.walk(|expr| {
            let label = format!("{expr:?}");
            let token = label.split(['(', ' ', '{']).next().unwrap_or("").to_string();
            *tokens.entry(token).or_default() += 1;
            match expr {
                l2_uscript::Expr::FinalFunction { function: callee, .. } => match vm.resolve(interface, *callee) {
                    Some(global) => {
                        if let Some(meta) = vm.functions.get(&global) {
                            if meta.native_index != 0 {
                                *core_natives.entry(format!("{} {}", meta.native_index, meta.name)).or_default() += 1;
                            } else if meta.is_native() {
                                *host_natives
                                    .entry(format!("{}.{}", vm.class_name(meta.class), meta.name))
                                    .or_default() += 1;
                            }
                        }
                    }
                    None => {
                        let name = vm.packages[interface].script.package.object_name(*callee).to_string();
                        *unresolved.entry(name).or_default() += 1;
                    }
                },
                l2_uscript::Expr::Native { index, .. } => {
                    let name = vm
                        .core
                        .get(index)
                        .and_then(|g| vm.functions.get(g))
                        .map(|m| m.name.clone())
                        .unwrap_or_default();
                    *core_natives.entry(format!("{index} {name}")).or_default() += 1;
                }
                _ => {}
            }
        });
    }
    eprintln!("tokens distintos: {}", tokens.len());
    for (token, count) in &tokens {
        eprintln!("  token {token} ×{count}");
    }
    eprintln!("natives do Core usadas: {}", core_natives.len());
    for (name, count) in &core_natives {
        eprintln!("  core {name} ×{count}");
    }
    eprintln!("natives do host usadas: {}", host_natives.len());
    for (name, count) in &host_natives {
        eprintln!("  host {name} ×{count}");
    }
    eprintln!("chamadas não resolvidas: {}", unresolved.len());
    for (name, count) in &unresolved {
        eprintln!("  ? {name} ×{count}");
    }
    assert!(unresolved.is_empty(), "{unresolved:?}");
}
