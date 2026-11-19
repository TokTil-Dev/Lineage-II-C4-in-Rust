use l2_uscript::natives::ScriptResolver;
use l2_uscript::{Kind, Natives, ObjectRef, Script};
use std::path::{Path, PathBuf};

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("uso: dump <pacote.u> [Classe[.Funcao]] [--natives <Core.u> ...]");
        std::process::exit(2);
    };
    let wanted: Vec<String> = args.collect();
    let data = std::fs::read(&path).expect("leitura do pacote");
    let script = Script::parse(&data).expect("pacote");
    let mut natives = Natives::default();
    natives.add(&script, &stem(&path));
    let dir = Path::new(&path).parent().map(Path::to_path_buf).unwrap_or_else(|| PathBuf::from("."));
    for sibling in ["Core.u", "Engine.u", "NWindow.u"] {
        let candidate = dir.join(sibling);
        if candidate.exists()
            && candidate.file_name() != Path::new(&path).file_name()
            && let Ok(bytes) = std::fs::read(&candidate)
            && let Ok(other) = Script::parse(&bytes)
        {
            natives.add(&other, &stem(&candidate.to_string_lossy()));
        }
    }
    let resolver = ScriptResolver { script: &script, natives: Some(&natives) };
    let filter = wanted.first().cloned();
    let (class_filter, function_filter) = match filter.as_deref().and_then(|f| f.split_once('.')) {
        Some((class, function)) => (Some(class.to_string()), Some(function.to_string())),
        None => (filter.clone(), None),
    };
    let errors: Vec<_> = script.errors().collect();
    println!("{} exports, {} com erro", script.objects.len(), errors.len());
    for (object, error) in errors.iter().take(20) {
        println!("  erro {} ({}): {error}", object.path, object.class);
    }
    for object in &script.objects {
        let Kind::Class(class) = &object.kind else { continue };
        if class_filter.as_deref().is_some_and(|name| name != object.name) {
            continue;
        }
        println!(
            "class {} extends {} (flags {:#x}, {} bytes de script)",
            object.name,
            resolver_name(&script, class.state.body.super_field),
            class.flags,
            class.state.body.script_size
        );
        for child in script.children(ObjectRef::Export(object.export)) {
            match &child.kind {
                Kind::Property(property) if function_filter.is_none() => {
                    println!(
                        "  var {:?} {} x{} ({} bytes, flags {:#x})",
                        property.kind, child.name, property.array_dim, property.element_size, property.flags
                    );
                }
                Kind::Function(function) => {
                    if function_filter.as_deref().is_some_and(|name| name != child.name) {
                        continue;
                    }
                    println!(
                        "  function {} (flags {:#x}, native {}, prec {}, script {} bytes mem / {} bytes arquivo)",
                        child.name,
                        function.flags,
                        function.native_index,
                        function.operator_precedence,
                        function.body.script_size,
                        function.body.script_file_size
                    );
                    for param in script.children(ObjectRef::Export(child.export)) {
                        if let Kind::Property(property) = &param.kind {
                            println!("    local/parm {:?} {} (flags {:#x})", property.kind, param.name, property.flags);
                        }
                    }
                    for line in function.body.script.pretty(&resolver).lines() {
                        println!("    {line}");
                    }
                }
                Kind::State(state) if function_filter.is_none() => {
                    println!("  state {} (flags {:#x}, {} bytes)", child.name, state.flags, state.body.script_size);
                }
                Kind::Const { value, .. } if function_filter.is_none() => {
                    println!("  const {} = {value}", child.name);
                }
                Kind::Enum { names, .. } if function_filter.is_none() => {
                    println!("  enum {} {{ {} }}", child.name, names.join(", "));
                }
                Kind::Struct(body) if function_filter.is_none() => {
                    println!("  struct {} ({} bytes)", child.name, body.script_size);
                }
                _ => {}
            }
        }
    }
    if wanted.iter().any(|w| w == "--natives") {
        for info in natives.sorted() {
            println!(
                "native {} = {}::{} ({}) flags {:#x} prec {}",
                info.index,
                info.class,
                info.name,
                info.display_name(),
                info.flags,
                info.operator_precedence
            );
        }
    }
}

fn stem(path: &str) -> String {
    Path::new(path).file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
}

fn resolver_name(script: &Script, object: ObjectRef) -> String {
    match object {
        ObjectRef::Null => "None".to_string(),
        other => script.object_name(other).to_string(),
    }
}
