pub mod core;
pub mod defaults;
pub mod exec;
pub mod model;

use std::collections::HashMap;
use std::fmt;

use crate::Script;
use crate::objects::PropertyTag;
pub use model::{ClassMeta, FunctionMeta, GlobalRef, ParamMeta, PropMeta, StateMeta, StructMeta, ValueKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjId(pub usize);

#[derive(Debug, Clone, PartialEq, Default)]
pub struct StructValue {
    pub name: String,
    pub fields: Vec<(String, Value)>,
}

impl StructValue {
    pub fn get(&self, field: &str) -> Option<&Value> {
        self.fields.iter().find(|(name, _)| name.eq_ignore_ascii_case(field)).map(|(_, value)| value)
    }

    pub fn get_mut(&mut self, field: &str) -> Option<&mut Value> {
        self.fields.iter_mut().find(|(name, _)| name.eq_ignore_ascii_case(field)).map(|(_, value)| value)
    }

    pub fn set(&mut self, field: &str, value: Value) {
        match self.get_mut(field) {
            Some(slot) => *slot = value,
            None => self.fields.push((field.to_string(), value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    #[default]
    None,
    Bool(bool),
    Byte(u8),
    Int(i32),
    Float(f32),
    Str(String),
    Name(String),
    Object(ObjId),
    Class(GlobalRef),
    Handle(String),
    Struct(Box<StructValue>),
    Array(Vec<Value>),
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            Value::None => false,
            Value::Bool(b) => *b,
            Value::Byte(b) => *b != 0,
            Value::Int(i) => *i != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) | Value::Name(s) | Value::Handle(s) => !s.is_empty() && s != "None",
            Value::Object(_) | Value::Class(_) => true,
            Value::Struct(_) => true,
            Value::Array(items) => !items.is_empty(),
        }
    }

    pub fn as_int(&self) -> i32 {
        match self {
            Value::Bool(b) => i32::from(*b),
            Value::Byte(b) => i32::from(*b),
            Value::Int(i) => *i,
            Value::Float(f) => *f as i32,
            Value::Str(s) => app_atoi(s),
            _ => 0,
        }
    }

    pub fn as_float(&self) -> f32 {
        match self {
            Value::Bool(b) => f32::from(u8::from(*b)),
            Value::Byte(b) => f32::from(*b),
            Value::Int(i) => *i as f32,
            Value::Float(f) => *f,
            Value::Str(s) => app_atof(s),
            _ => 0.0,
        }
    }

    pub fn as_byte(&self) -> u8 {
        match self {
            Value::Bool(b) => u8::from(*b),
            Value::Byte(b) => *b,
            Value::Int(i) => *i as u8,
            Value::Float(f) => *f as u8,
            Value::Str(s) => app_atoi(s) as u8,
            _ => 0,
        }
    }

    pub fn as_str(&self) -> String {
        match self {
            Value::None => String::new(),
            Value::Bool(b) => {
                if *b {
                    "True".to_string()
                } else {
                    "False".to_string()
                }
            }
            Value::Byte(b) => b.to_string(),
            Value::Int(i) => i.to_string(),
            Value::Float(f) => format_float(*f),
            Value::Str(s) | Value::Name(s) | Value::Handle(s) => s.clone(),
            Value::Object(id) => format!("Object{}", id.0),
            Value::Class(class) => format!("Class{}.{}", class.package, class.export),
            Value::Struct(s) => s.fields.iter().map(|(_, v)| v.as_str()).collect::<Vec<_>>().join(","),
            Value::Array(items) => items.iter().map(Value::as_str).collect::<Vec<_>>().join(","),
        }
    }

    pub fn as_name(&self) -> String {
        match self {
            Value::None => "None".to_string(),
            other => other.as_str(),
        }
    }

    pub fn struct_ref(&self) -> Option<&StructValue> {
        match self {
            Value::Struct(s) => Some(s),
            _ => None,
        }
    }

    pub fn field(&self, name: &str) -> Value {
        self.struct_ref().and_then(|s| s.get(name)).cloned().unwrap_or_default()
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Value {
        Value::Struct(Box::new(StructValue {
            name: "Vector".into(),
            fields: vec![("X".into(), Value::Float(x)), ("Y".into(), Value::Float(y)), ("Z".into(), Value::Float(z))],
        }))
    }

    pub fn rotator(pitch: i32, yaw: i32, roll: i32) -> Value {
        Value::Struct(Box::new(StructValue {
            name: "Rotator".into(),
            fields: vec![
                ("Pitch".into(), Value::Int(pitch)),
                ("Yaw".into(), Value::Int(yaw)),
                ("Roll".into(), Value::Int(roll)),
            ],
        }))
    }

    pub fn xyz(&self) -> (f32, f32, f32) {
        (self.field("X").as_float(), self.field("Y").as_float(), self.field("Z").as_float())
    }
}

pub fn format_float(value: f32) -> String {
    format!("{value:.2}")
}

pub fn app_atoi(text: &str) -> i32 {
    let trimmed = text.trim_start();
    let (negative, digits) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, trimmed.strip_prefix('+').unwrap_or(trimmed)),
    };
    let mut value: i64 = 0;
    for c in digits.chars().take_while(char::is_ascii_digit) {
        value = (value * 10 + i64::from(c as u8 - b'0')).min(i64::from(i32::MAX) + 1);
    }
    let value = if negative { -value } else { value };
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

pub fn app_atof(text: &str) -> f32 {
    let trimmed = text.trim_start();
    let end = trimmed
        .char_indices()
        .find(|(index, c)| !(c.is_ascii_digit() || *c == '.' || (*index == 0 && (*c == '-' || *c == '+'))))
        .map_or(trimmed.len(), |(index, _)| index);
    trimmed[..end].parse().unwrap_or(0.0)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub message: String,
    pub function: String,
    pub offset: usize,
}

impl Error {
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into(), function: String::new(), offset: 0 }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.function.is_empty() {
            write!(f, "{}", self.message)
        } else {
            write!(f, "{} em {}+{:#x}", self.message, self.function, self.offset)
        }
    }
}

impl std::error::Error for Error {}

pub trait Host {
    fn call_native(
        &mut self,
        class: &str,
        function: &str,
        this: Option<&Value>,
        args: &mut [Value],
    ) -> Result<Value, Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScriptObject {
    pub class: GlobalRef,
    pub name: String,
    pub props: Vec<Value>,
    pub state: Option<String>,
}

pub struct LoadedPackage {
    pub name: String,
    pub script: Script,
}

pub struct Vm {
    pub packages: Vec<LoadedPackage>,
    pub by_path: HashMap<String, GlobalRef>,
    pub import_cache: Vec<Vec<Option<GlobalRef>>>,
    pub classes: HashMap<GlobalRef, ClassMeta>,
    pub functions: HashMap<GlobalRef, FunctionMeta>,
    pub structs: HashMap<GlobalRef, StructMeta>,
    pub props: HashMap<GlobalRef, PropMeta>,
    pub enums: HashMap<GlobalRef, Vec<String>>,
    pub core: HashMap<u16, GlobalRef>,
    pub objects: Vec<ScriptObject>,
    pub log: Vec<String>,
    pub default_errors: Vec<String>,
    pub steps: usize,
    pub handle_classes: HashMap<String, String>,
    pub depth: usize,
    rng: u32,
}

pub const MAX_DEPTH: usize = 256;

impl Vm {
    pub fn load(packages: Vec<(String, Script)>) -> Self {
        let packages = packages.into_iter().map(|(name, script)| LoadedPackage { name, script }).collect();
        let mut vm = Vm {
            packages,
            by_path: HashMap::new(),
            import_cache: Vec::new(),
            classes: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            props: HashMap::new(),
            enums: HashMap::new(),
            core: HashMap::new(),
            objects: Vec::new(),
            log: Vec::new(),
            default_errors: Vec::new(),
            steps: 0,
            handle_classes: HashMap::new(),
            depth: 0,
            rng: 0x1234_5678,
        };
        model::build(&mut vm);
        vm
    }

    pub fn class_by_name(&self, name: &str) -> Option<GlobalRef> {
        self.classes.iter().find(|(_, class)| class.name.eq_ignore_ascii_case(name)).map(|(global, _)| *global)
    }

    pub fn instantiate(&mut self, class: &str) -> Result<ObjId, Error> {
        let global = self.class_by_name(class).ok_or_else(|| Error::new(format!("classe {class} não encontrada")))?;
        self.instantiate_ref(global, class)
    }

    pub fn instantiate_ref(&mut self, class: GlobalRef, name: &str) -> Result<ObjId, Error> {
        let meta = self.classes.get(&class).ok_or_else(|| Error::new("classe sem metadados"))?;
        let props = meta.default_values.clone();
        let id = ObjId(self.objects.len());
        self.objects.push(ScriptObject { class, name: name.to_string(), props, state: None });
        Ok(id)
    }

    pub fn object(&self, id: ObjId) -> Option<&ScriptObject> {
        self.objects.get(id.0)
    }

    pub fn object_by_name(&self, name: &str) -> Option<ObjId> {
        self.objects.iter().position(|object| object.name.eq_ignore_ascii_case(name)).map(ObjId)
    }

    pub fn get_prop(&self, id: ObjId, name: &str) -> Option<&Value> {
        let object = self.objects.get(id.0)?;
        let class = self.classes.get(&object.class)?;
        let index = class
            .layout
            .iter()
            .position(|prop| self.props.get(prop).is_some_and(|p| p.name.eq_ignore_ascii_case(name)))?;
        object.props.get(index)
    }

    pub fn set_prop(&mut self, id: ObjId, name: &str, value: Value) -> bool {
        let Some(object) = self.objects.get(id.0) else { return false };
        let Some(class) = self.classes.get(&object.class) else { return false };
        let Some(index) = class
            .layout
            .iter()
            .position(|prop| self.props.get(prop).is_some_and(|p| p.name.eq_ignore_ascii_case(name)))
        else {
            return false;
        };
        if let Some(slot) = self.objects.get_mut(id.0).and_then(|object| object.props.get_mut(index)) {
            *slot = value;
            true
        } else {
            false
        }
    }

    pub fn handle_class_for(&self, handle: &str, function: &str) -> String {
        if let Some(class) = self.handle_classes.get(handle) {
            return class.clone();
        }
        let mut declaring: Vec<&ClassMeta> = self
            .classes
            .values()
            .filter(|meta| meta.name.ends_with("Handle") && meta.functions.contains_key(function))
            .collect();
        declaring.sort_by(|a, b| a.name.cmp(&b.name));
        match declaring.as_slice() {
            [only] => only.name.clone(),
            [] => "WindowHandle".to_string(),
            many => many.iter().find(|meta| meta.name == "WindowHandle").map_or_else(|| many[0].name.clone(), |meta| meta.name.clone()),
        }
    }

    pub fn class_name(&self, class: GlobalRef) -> &str {
        self.classes.get(&class).map_or("?", |meta| meta.name.as_str())
    }

    pub fn function_name(&self, function: GlobalRef) -> String {
        match self.functions.get(&function) {
            Some(meta) => format!("{}.{}", self.class_name(meta.class), meta.name),
            None => "?".to_string(),
        }
    }

    pub fn has_function(&self, id: ObjId, name: &str) -> bool {
        self.object(id).and_then(|object| self.find_function(object.class, name, None)).is_some()
    }

    pub fn call(&mut self, host: &mut dyn Host, this: ObjId, function: &str, args: Vec<Value>) -> Result<Value, Error> {
        self.steps = 0;
        let object = self.object(this).ok_or_else(|| Error::new("objeto inexistente"))?;
        let class = object.class;
        let state = object.state.clone();
        let target = self
            .find_function(class, function, state.as_deref())
            .ok_or_else(|| Error::new(format!("{}.{function} não existe", self.class_name(class))))?;
        exec::invoke(self, host, this, target, args)
    }

    pub fn event(
        &mut self,
        host: &mut dyn Host,
        this: ObjId,
        function: &str,
        args: Vec<Value>,
    ) -> Result<Value, Error> {
        if !self.has_function(this, function) {
            return Ok(Value::None);
        }
        self.call(host, this, function, args)
    }

    pub fn next_rand(&mut self) -> u32 {
        self.rng = self.rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        self.rng >> 8
    }

    pub fn default_tags(&self, class: GlobalRef) -> &[PropertyTag] {
        self.classes.get(&class).map_or(&[], |meta| meta.defaults.as_slice())
    }
}
