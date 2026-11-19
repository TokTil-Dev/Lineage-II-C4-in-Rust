use l2_uscript::natives::ScriptResolver;
use l2_uscript::{Expr, Kind, Natives, ObjectRef, Script};
use std::path::PathBuf;

const SCRATCH: &str = "/private/tmp/claude-501/-Users-wender-projetos-lineage2-killer-web/e7251fea-f786-4501-b8bd-6aa35ac88ddc/scratchpad/u";

fn package_dir() -> Option<PathBuf> {
    let candidates = [std::env::var("KILLER_USCRIPT_DIR").ok(), Some(SCRATCH.to_string())];
    candidates.into_iter().flatten().map(PathBuf::from).find(|dir| dir.join("interface.u").is_file())
}

fn load(name: &str) -> Option<Script> {
    let path = package_dir()?.join(name);
    let data = std::fs::read(path).ok()?;
    Some(Script::parse(&data).expect("pacote"))
}

fn script_errors(script: &Script) -> Vec<String> {
    script
        .errors()
        .filter(|(object, _)| {
            !matches!(
                object.class.as_str(),
                "Palette" | "Texture" | "Font" | "Sound" | "Music" | "StaticMesh" | "Mesh" | "Model" | "Polys"
            )
        })
        .map(|(object, error)| format!("{} ({}): {error}", object.path, object.class))
        .collect()
}

#[test]
fn every_script_object_consumes_exactly_its_serial_size() {
    let Some(dir) = package_dir() else { return };
    for name in ["Core.u", "Engine.u", "NWindow.u", "interface.u"] {
        let Ok(data) = std::fs::read(dir.join(name)) else { continue };
        let script = Script::parse(&data).expect("pacote");
        let errors = script_errors(&script);
        assert!(errors.is_empty(), "{name}: {} objetos com erro, por exemplo {:?}", errors.len(), errors.first());
        let scripted = script.objects.iter().filter(|object| matches!(object.kind, Kind::Function(_))).count();
        assert!(scripted > 0, "{name} sem funções");
    }
}

#[test]
fn status_wnd_on_load_registers_the_eight_events_in_order() {
    let Some(script) = load("interface.u") else { return };
    let class = script.class("StatusWnd").expect("StatusWnd");
    let on_load =
        script.functions(ObjectRef::Export(class.export)).into_iter().find(|f| f.name == "OnLoad").expect("OnLoad");
    let Kind::Function(function) = &on_load.kind else { panic!() };
    let mut ids = Vec::new();
    for instruction in &function.body.script.instructions {
        let (name, args) = match &instruction.expr {
            Expr::FinalFunction { function: callee, args } => (script.object_name(*callee).to_string(), args),
            Expr::VirtualFunction { name, args } => (name.clone(), args),
            Expr::ReturnNothing | Expr::Return(_) => continue,
            other => panic!("{other:?}"),
        };
        assert_eq!(name, "RegisterEvent");
        let id = match args.as_slice() {
            [Expr::IntConst(id)] => *id,
            [Expr::IntConstByte(id)] => i32::from(*id),
            other => panic!("{other:?}"),
        };
        ids.push(id);
    }
    assert_eq!(ids, [70, 180, 190, 200, 210, 220, 230, 240]);
    assert_eq!(function.body.script_size, 66);
    assert_eq!(function.body.script_file_size, 42);
}

#[test]
fn status_wnd_sets_the_user_name_with_type_zero_and_align_one() {
    let Some(script) = load("interface.u") else { return };
    let class = script.class("StatusWnd").expect("StatusWnd");
    let mut found = Vec::new();
    for function in script.functions(ObjectRef::Export(class.export)) {
        let Kind::Function(body) = &function.kind else { continue };
        body.body.script.walk(|expr| {
            if let Expr::ClassContext { member, .. } = expr {
                let call = match member.as_ref() {
                    Expr::FinalFunction { function: callee, args } => {
                        Some((script.object_name(*callee).to_string(), args))
                    }
                    Expr::VirtualFunction { name, args } => Some((name.clone(), args)),
                    _ => None,
                };
                if let Some((name, args)) = call
                    && name == "SetName"
                {
                    found.push((function.name.clone(), args.clone()));
                }
            }
        });
    }
    let (owner, args) = found.first().expect("SetName");
    assert_eq!(owner, "UpdateUserInfo");
    assert_eq!(args[0], Expr::StringConst("StatusWnd.UserName".into()));
    assert_eq!(args[2], Expr::ByteConst(0));
    assert_eq!(args[3], Expr::ByteConst(1));
}

#[test]
fn chat_wnd_has_the_functions_of_the_shipped_script() {
    let Some(script) = load("interface.u") else { return };
    let class = script.class("ChatWnd").expect("ChatWnd");
    let names: Vec<&str> = script.functions(ObjectRef::Export(class.export)).iter().map(|f| f.name.as_str()).collect();
    assert_eq!(names.len(), 29, "{names:?}");
    for expected in [
        "OnLoad",
        "OnShow",
        "OnEvent",
        "OnClickButton",
        "HandleTabClick",
        "OnTabSplit",
        "OnTabMerge",
        "InitScrollBarPosition",
        "OnCompleteEditBox",
    ] {
        assert!(names.contains(&expected), "falta {expected} em {names:?}");
    }
}

#[test]
fn core_natives_carry_the_unrealscript_operators_by_index() {
    let Some(core) = load("Core.u") else { return };
    let mut natives = Natives::default();
    assert!(natives.add(&core, "Core") > 100);
    let expect = [
        (112, "$", true),
        (130, "&&", true),
        (132, "||", true),
        (146, "+", true),
        (150, "<", true),
        (154, "==", true),
        (155, "!=", true),
        (129, "!", true),
        (125, "Len", false),
        (126, "InStr", false),
        (127, "Mid", false),
        (128, "Left", false),
        (168, "@", true),
        (122, "==", true),
    ];
    for (index, name, operator) in expect {
        let info = natives.get(index).unwrap_or_else(|| panic!("native {index}"));
        assert_eq!(info.display_name(), name, "native {index}");
        assert_eq!(info.is_operator() || info.is_pre_operator(), operator, "native {index}");
    }
}

#[test]
fn pretty_printing_resolves_natives_and_objects() {
    let (Some(core), Some(script)) = (load("Core.u"), load("interface.u")) else { return };
    let mut natives = Natives::default();
    natives.add(&core, "Core");
    let class = script.class("StatusWnd").expect("StatusWnd");
    let on_load =
        script.functions(ObjectRef::Export(class.export)).into_iter().find(|f| f.name == "OnLoad").expect("OnLoad");
    let Kind::Function(function) = &on_load.kind else { panic!() };
    let text = function.body.script.pretty(&ScriptResolver { script: &script, natives: Some(&natives) });
    assert!(text.starts_with("0x0000: RegisterEvent(70)\n0x0008: RegisterEvent(180)"), "{text}");
}
