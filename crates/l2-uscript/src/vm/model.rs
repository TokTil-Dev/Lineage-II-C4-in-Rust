use std::collections::HashMap;

use super::{Value, Vm};
use crate::objects::{Kind, PropertyKind, PropertyTag};
use ue2_package::ObjectRef;

pub const CPF_PARM: u32 = 0x80;
pub const CPF_OPTIONAL: u32 = 0x10;
pub const CPF_OUT: u32 = 0x100;
pub const CPF_RETURN: u32 = 0x400;
pub const FUNC_NATIVE: u32 = 0x400;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GlobalRef {
    pub package: usize,
    pub export: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueKind {
    Bool,
    Byte,
    Int,
    Float,
    Str,
    Name,
    Object,
    Class,
    Struct(Option<GlobalRef>),
    Array(Option<GlobalRef>),
    Delegate,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropMeta {
    pub global: GlobalRef,
    pub name: String,
    pub kind: ValueKind,
    pub array_dim: u16,
    pub flags: u32,
    pub enumeration: Option<GlobalRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamMeta {
    pub local: usize,
    pub out: bool,
    pub optional: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionMeta {
    pub global: GlobalRef,
    pub name: String,
    pub class: GlobalRef,
    pub flags: u32,
    pub native_index: u16,
    pub params: Vec<ParamMeta>,
    pub return_local: Option<usize>,
    pub locals: Vec<GlobalRef>,
    pub local_index: HashMap<GlobalRef, usize>,
    pub offsets: HashMap<usize, usize>,
    pub body: std::sync::Arc<crate::bytecode::Disassembly>,
}

impl FunctionMeta {
    pub fn is_native(&self) -> bool {
        self.flags & FUNC_NATIVE != 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct StateMeta {
    pub name: String,
    pub functions: HashMap<String, GlobalRef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassMeta {
    pub global: GlobalRef,
    pub name: String,
    pub super_class: Option<GlobalRef>,
    pub own_props: Vec<GlobalRef>,
    pub functions: HashMap<String, GlobalRef>,
    pub states: HashMap<String, StateMeta>,
    pub defaults: Vec<PropertyTag>,
    pub layout: Vec<GlobalRef>,
    pub layout_index: HashMap<GlobalRef, usize>,
    pub default_values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructMeta {
    pub global: GlobalRef,
    pub name: String,
    pub super_struct: Option<GlobalRef>,
    pub own_fields: Vec<GlobalRef>,
    pub fields: Vec<GlobalRef>,
}

pub fn lower(name: &str) -> String {
    name.to_ascii_lowercase()
}

impl Vm {
    pub fn export_path(&self, package: usize, export: usize) -> String {
        let script = &self.packages[package].script;
        let mut parts = vec![script.package.object_name(ObjectRef::Export(export)).to_string()];
        let mut current = script.package.outer(ObjectRef::Export(export));
        let mut guard = 0;
        while current != ObjectRef::Null && guard < 64 {
            parts.push(script.package.object_name(current).to_string());
            current = script.package.outer(current);
            guard += 1;
        }
        parts.push(self.packages[package].name.clone());
        parts.reverse();
        lower(&parts.join("."))
    }

    pub fn import_path(&self, package: usize, import: usize) -> String {
        let script = &self.packages[package].script;
        let mut parts = Vec::new();
        let mut current = ObjectRef::Import(import);
        let mut guard = 0;
        while let ObjectRef::Import(index) = current {
            if guard > 64 {
                break;
            }
            parts.push(script.package.object_name(current).to_string());
            current = script.package.imports()[index].outer;
            guard += 1;
        }
        parts.reverse();
        lower(&parts.join("."))
    }

    pub fn resolve(&self, package: usize, object: ObjectRef) -> Option<GlobalRef> {
        match object {
            ObjectRef::Null => None,
            ObjectRef::Export(export) => Some(GlobalRef { package, export }),
            ObjectRef::Import(import) => {
                self.import_cache.get(package).and_then(|cache| cache.get(import)).copied().flatten()
            }
        }
    }

    pub fn object_name(&self, global: GlobalRef) -> &str {
        self.packages
            .get(global.package)
            .map_or("?", |package| package.script.package.object_name(ObjectRef::Export(global.export)))
    }

    pub fn object_kind(&self, global: GlobalRef) -> Option<&Kind> {
        self.packages.get(global.package).and_then(|p| p.script.objects.get(global.export)).map(|object| &object.kind)
    }

    pub fn class_chain(&self, class: GlobalRef) -> Vec<GlobalRef> {
        let mut chain = Vec::new();
        let mut current = Some(class);
        while let Some(global) = current {
            if chain.contains(&global) || chain.len() > 64 {
                break;
            }
            chain.push(global);
            current = self.classes.get(&global).and_then(|meta| meta.super_class);
        }
        chain
    }

    pub fn is_child_of(&self, class: GlobalRef, parent: GlobalRef) -> bool {
        self.class_chain(class).contains(&parent)
    }

    pub fn find_function(&self, class: GlobalRef, name: &str, state: Option<&str>) -> Option<GlobalRef> {
        let key = lower(name);
        for global in self.class_chain(class) {
            let meta = self.classes.get(&global)?;
            if let Some(state) = state
                && let Some(state_meta) = meta.states.get(&lower(state))
                && let Some(function) = state_meta.functions.get(&key)
            {
                return Some(*function);
            }
        }
        for global in self.class_chain(class) {
            let meta = self.classes.get(&global)?;
            if let Some(function) = meta.functions.get(&key) {
                return Some(*function);
            }
        }
        None
    }

    pub fn find_global_function(&self, class: GlobalRef, name: &str) -> Option<GlobalRef> {
        self.find_function(class, name, None)
    }

    pub fn struct_fields(&self, structure: GlobalRef) -> Vec<GlobalRef> {
        self.structs.get(&structure).map(|meta| meta.fields.clone()).unwrap_or_default()
    }

    pub fn default_value(&self, kind: &ValueKind, array_dim: u16) -> Value {
        if array_dim > 1 {
            return Value::Array((0..array_dim).map(|_| self.default_value(kind, 1)).collect());
        }
        match kind {
            ValueKind::Bool => Value::Bool(false),
            ValueKind::Byte => Value::Byte(0),
            ValueKind::Int => Value::Int(0),
            ValueKind::Float => Value::Float(0.0),
            ValueKind::Str => Value::Str(String::new()),
            ValueKind::Name => Value::Name("None".into()),
            ValueKind::Object | ValueKind::Class | ValueKind::Delegate | ValueKind::Other => Value::None,
            ValueKind::Struct(structure) => self.default_struct(*structure),
            ValueKind::Array(_) => Value::Array(Vec::new()),
        }
    }

    pub fn default_struct(&self, structure: Option<GlobalRef>) -> Value {
        let Some(structure) = structure else { return Value::Struct(Box::default()) };
        let name = self.structs.get(&structure).map(|meta| meta.name.clone()).unwrap_or_default();
        let fields = self
            .struct_fields(structure)
            .into_iter()
            .filter_map(|field| self.props.get(&field))
            .map(|prop| (prop.name.clone(), self.default_value(&prop.kind, prop.array_dim)))
            .collect();
        Value::Struct(Box::new(super::StructValue { name, fields }))
    }

    pub fn prop_default(&self, prop: GlobalRef) -> Value {
        self.props.get(&prop).map_or(Value::None, |meta| self.default_value(&meta.kind, meta.array_dim))
    }
}

pub fn build(vm: &mut Vm) {
    for package in 0..vm.packages.len() {
        for export in 0..vm.packages[package].script.objects.len() {
            let path = vm.export_path(package, export);
            vm.by_path.entry(path).or_insert(GlobalRef { package, export });
        }
    }
    let mut cache = Vec::new();
    for package in 0..vm.packages.len() {
        let count = vm.packages[package].script.package.imports().len();
        let resolved = (0..count).map(|import| vm.by_path.get(&vm.import_path(package, import)).copied()).collect();
        cache.push(resolved);
    }
    vm.import_cache = cache;
    build_props(vm);
    build_structs(vm);
    build_classes(vm);
    build_functions(vm);
    build_layouts(vm);
    defaults(vm);
}

fn prop_kind(vm: &Vm, package: usize, kind: &PropertyKind) -> ValueKind {
    match kind {
        PropertyKind::Byte { .. } => ValueKind::Byte,
        PropertyKind::Int => ValueKind::Int,
        PropertyKind::Bool => ValueKind::Bool,
        PropertyKind::Float => ValueKind::Float,
        PropertyKind::Object { .. } => ValueKind::Object,
        PropertyKind::Class { .. } => ValueKind::Class,
        PropertyKind::Name => ValueKind::Name,
        PropertyKind::Str => ValueKind::Str,
        PropertyKind::Struct { structure } => ValueKind::Struct(vm.resolve(package, *structure)),
        PropertyKind::Array { inner } => ValueKind::Array(vm.resolve(package, *inner)),
        PropertyKind::Delegate { .. } => ValueKind::Delegate,
        _ => ValueKind::Other,
    }
}

fn build_props(vm: &mut Vm) {
    let mut props = HashMap::new();
    let mut enums = HashMap::new();
    for package in 0..vm.packages.len() {
        for (export, object) in vm.packages[package].script.objects.iter().enumerate() {
            let global = GlobalRef { package, export };
            match &object.kind {
                Kind::Property(property) => {
                    let enumeration = match &property.kind {
                        PropertyKind::Byte { enumeration } => vm.resolve(package, *enumeration),
                        _ => None,
                    };
                    props.insert(
                        global,
                        PropMeta {
                            global,
                            name: object.name.clone(),
                            kind: prop_kind(vm, package, &property.kind),
                            array_dim: property.array_dim,
                            flags: property.flags,
                            enumeration,
                        },
                    );
                }
                Kind::Enum { names, .. } => {
                    enums.insert(global, names.clone());
                }
                _ => {}
            }
        }
    }
    vm.props = props;
    vm.enums = enums;
}

fn children_of(vm: &Vm, global: GlobalRef) -> Vec<GlobalRef> {
    let script = &vm.packages[global.package].script;
    script
        .children(ObjectRef::Export(global.export))
        .iter()
        .map(|child| GlobalRef { package: global.package, export: child.export })
        .collect()
}

fn build_structs(vm: &mut Vm) {
    let mut structs = HashMap::new();
    for package in 0..vm.packages.len() {
        for (export, object) in vm.packages[package].script.objects.iter().enumerate() {
            let Kind::Struct(body) = &object.kind else { continue };
            let global = GlobalRef { package, export };
            let own_fields: Vec<GlobalRef> =
                children_of(vm, global).into_iter().filter(|child| vm.props.contains_key(child)).collect();
            structs.insert(
                global,
                StructMeta {
                    global,
                    name: object.name.clone(),
                    super_struct: vm.resolve(package, body.super_field),
                    own_fields,
                    fields: Vec::new(),
                },
            );
        }
    }
    let keys: Vec<GlobalRef> = structs.keys().copied().collect();
    for key in keys {
        let mut chain = Vec::new();
        let mut current = Some(key);
        while let Some(global) = current {
            if chain.contains(&global) {
                break;
            }
            chain.push(global);
            current = structs.get(&global).and_then(|meta| meta.super_struct);
        }
        let mut fields = Vec::new();
        for global in chain.iter().rev() {
            if let Some(meta) = structs.get(global) {
                fields.extend(meta.own_fields.iter().copied());
            }
        }
        if let Some(meta) = structs.get_mut(&key) {
            meta.fields = fields;
        }
    }
    vm.structs = structs;
}

fn build_classes(vm: &mut Vm) {
    let mut classes = HashMap::new();
    for package in 0..vm.packages.len() {
        for (export, object) in vm.packages[package].script.objects.iter().enumerate() {
            let Kind::Class(class) = &object.kind else { continue };
            let global = GlobalRef { package, export };
            let mut own_props = Vec::new();
            let mut functions = HashMap::new();
            let mut states = HashMap::new();
            for child in children_of(vm, global) {
                match vm.object_kind(child) {
                    Some(Kind::Property(_)) => own_props.push(child),
                    Some(Kind::Function(_)) => {
                        functions.insert(lower(vm.object_name(child)), child);
                    }
                    Some(Kind::State(_)) => {
                        let mut state =
                            StateMeta { name: vm.object_name(child).to_string(), functions: HashMap::new() };
                        for member in children_of(vm, child) {
                            if matches!(vm.object_kind(member), Some(Kind::Function(_))) {
                                state.functions.insert(lower(vm.object_name(member)), member);
                            }
                        }
                        states.insert(lower(&state.name), state);
                    }
                    _ => {}
                }
            }
            classes.insert(
                global,
                ClassMeta {
                    global,
                    name: object.name.clone(),
                    super_class: vm.resolve(package, class.state.body.super_field),
                    own_props,
                    functions,
                    states,
                    defaults: class.defaults.clone(),
                    layout: Vec::new(),
                    layout_index: HashMap::new(),
                    default_values: Vec::new(),
                },
            );
        }
    }
    vm.classes = classes;
}

fn build_functions(vm: &mut Vm) {
    let mut functions = HashMap::new();
    for package in 0..vm.packages.len() {
        for (export, object) in vm.packages[package].script.objects.iter().enumerate() {
            let Kind::Function(function) = &object.kind else { continue };
            let global = GlobalRef { package, export };
            let mut owner = vm.resolve(package, vm.packages[package].script.package.outer(ObjectRef::Export(export)));
            if let Some(state) = owner
                && matches!(vm.object_kind(state), Some(Kind::State(_)))
            {
                owner = vm.resolve(package, vm.packages[package].script.package.outer(ObjectRef::Export(state.export)));
            }
            let Some(class) = owner else { continue };
            let locals: Vec<GlobalRef> =
                children_of(vm, global).into_iter().filter(|child| vm.props.contains_key(child)).collect();
            let local_index: HashMap<GlobalRef, usize> =
                locals.iter().enumerate().map(|(index, global)| (*global, index)).collect();
            let mut params = Vec::new();
            let mut return_local = None;
            for (index, local) in locals.iter().enumerate() {
                let Some(prop) = vm.props.get(local) else { continue };
                if prop.flags & CPF_PARM == 0 {
                    continue;
                }
                if prop.flags & CPF_RETURN != 0 {
                    return_local = Some(index);
                    continue;
                }
                params.push(ParamMeta {
                    local: index,
                    out: prop.flags & CPF_OUT != 0,
                    optional: prop.flags & CPF_OPTIONAL != 0,
                });
            }
            let offsets = function
                .body
                .script
                .instructions
                .iter()
                .enumerate()
                .map(|(index, instruction)| (instruction.offset, index))
                .collect();
            functions.insert(
                global,
                FunctionMeta {
                    global,
                    name: object.name.clone(),
                    class,
                    flags: function.flags,
                    native_index: function.native_index,
                    params,
                    return_local,
                    locals,
                    local_index,
                    offsets,
                    body: std::sync::Arc::new(function.body.script.clone()),
                },
            );
            if function.native_index != 0 {
                vm.core.entry(function.native_index).or_insert(global);
            }
        }
    }
    vm.functions = functions;
}

fn build_layouts(vm: &mut Vm) {
    let keys: Vec<GlobalRef> = vm.classes.keys().copied().collect();
    for key in keys {
        let chain = vm.class_chain(key);
        let mut layout = Vec::new();
        for global in chain.iter().rev() {
            if let Some(meta) = vm.classes.get(global) {
                layout.extend(meta.own_props.iter().copied());
            }
        }
        let layout_index = layout.iter().enumerate().map(|(index, global)| (*global, index)).collect();
        let default_values = layout.iter().map(|prop| vm.prop_default(*prop)).collect();
        if let Some(meta) = vm.classes.get_mut(&key) {
            meta.layout = layout;
            meta.layout_index = layout_index;
            meta.default_values = default_values;
        }
    }
}

fn defaults(vm: &mut Vm) {
    let mut order: Vec<GlobalRef> = vm.classes.keys().copied().collect();
    order.sort_by_key(|key| vm.class_chain(*key).len());
    for key in order {
        let chain = vm.class_chain(key);
        let mut values = vm.classes.get(&key).map(|meta| meta.default_values.clone()).unwrap_or_default();
        for global in chain.iter().rev() {
            let tags = vm.classes.get(global).map(|meta| meta.defaults.clone()).unwrap_or_default();
            for tag in &tags {
                match super::defaults::apply_tag(vm, key, *global, tag, &mut values) {
                    Ok(()) => {}
                    Err(error) => vm.default_errors.push(format!("{}.{}: {error}", vm.class_name(key), tag.name)),
                }
            }
        }
        if let Some(meta) = vm.classes.get_mut(&key) {
            meta.default_values = values;
        }
    }
}
