use l2_uscript::Script;
use l2_uscript::vm::{Error, Host, Value, Vm};
use std::path::Path;

struct PrintingHost;

impl Host for PrintingHost {
    fn call_native(
        &mut self,
        class: &str,
        function: &str,
        this: Option<&Value>,
        args: &mut [Value],
    ) -> Result<Value, Error> {
        let args_text: Vec<String> = args.iter().map(Value::as_str).collect();
        match this {
            Some(this) => println!("  {class}.{function} [{}] ({})", this.as_str(), args_text.join(", ")),
            None => println!("  {class}.{function} ({})", args_text.join(", ")),
        }
        Ok(match function {
            "GetHandle" => Value::Handle(args.first().map_or_else(String::new, Value::as_str)),
            _ => Value::None,
        })
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let (Some(dir), Some(target)) = (args.next(), args.next()) else {
        eprintln!("uso: run <pasta dos .u> Classe.Funcao [args]");
        std::process::exit(2);
    };
    let dir = Path::new(&dir);
    let mut packages = Vec::new();
    for name in ["Core", "Engine", "NWindow", "interface"] {
        let data = std::fs::read(dir.join(format!("{name}.u"))).expect("pacote");
        packages.push((name.to_string(), Script::parse(&data).expect("pacote")));
    }
    let mut vm = Vm::load(packages);
    let (class, function) = target.split_once('.').expect("Classe.Funcao");
    let object = vm.instantiate(class).expect("classe");
    let values: Vec<Value> = args
        .map(|arg| match arg.parse::<i32>() {
            Ok(number) => Value::Int(number),
            Err(_) => Value::Str(arg),
        })
        .collect();
    let mut host = PrintingHost;
    match vm.call(&mut host, object, function, values) {
        Ok(value) => println!("= {value:?}"),
        Err(error) => println!("erro: {error}"),
    }
    for line in &vm.log {
        println!("log: {line}");
    }
}
