use std::sync::Arc;

use super::model::{GlobalRef, ValueKind};
use super::{Error, Host, ObjId, StructValue, Value, Vm};
use crate::bytecode::{Disassembly, Expr};
use ue2_package::ObjectRef;

pub struct Frame {
    pub this: ObjId,
    pub function: GlobalRef,
    pub package: usize,
    pub locals: Vec<Value>,
    pub switch: Option<Value>,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub enum Place {
    Local(usize),
    Instance(ObjId, usize),
    Default(GlobalRef, usize),
    Field(Box<Place>, String),
    Element(Box<Place>, usize),
    Temp(Value),
}

enum Flow {
    Next,
    Jump(usize),
    Return(Value),
}

fn fail(frame: &Frame, vm: &Vm, message: impl Into<String>) -> Error {
    Error { message: message.into(), function: vm.function_name(frame.function), offset: frame.offset }
}

pub const STEP_BUDGET: usize = 2_000_000;

pub fn invoke(
    vm: &mut Vm,
    host: &mut dyn Host,
    this: ObjId,
    function: GlobalRef,
    args: Vec<Value>,
) -> Result<Value, Error> {
    let meta = vm.functions.get(&function).ok_or_else(|| Error::new("função sem metadados"))?.clone();
    if meta.is_native() {
        let mut values = args;
        let this_value = Value::Object(this);
        let class = vm.class_name(meta.class).to_string();
        return host.call_native(&class, &meta.name, Some(&this_value), &mut values);
    }
    if vm.depth >= super::MAX_DEPTH {
        return Err(Error { message: "recursão profunda".into(), function: vm.function_name(function), offset: 0 });
    }
    let mut locals: Vec<Value> = meta.locals.iter().map(|local| vm.prop_default(*local)).collect();
    for (index, param) in meta.params.iter().enumerate() {
        if let Some(value) = args.get(index) {
            let kind = vm.props.get(&meta.locals[param.local]).map(|p| p.kind.clone()).unwrap_or(ValueKind::Other);
            locals[param.local] = coerce(value.clone(), &kind);
        }
    }
    let mut frame = Frame { this, function, package: function.package, locals, switch: None, offset: 0 };
    vm.depth += 1;
    let result = run(vm, host, &mut frame, &meta.body);
    vm.depth -= 1;
    let returned = result?;
    Ok(match (returned, meta.return_local) {
        (Some(value), _) => value,
        (None, Some(index)) => frame.locals.get(index).cloned().unwrap_or_default(),
        (None, None) => Value::None,
    })
}

pub fn coerce(value: Value, kind: &ValueKind) -> Value {
    match kind {
        ValueKind::Bool => Value::Bool(value.as_bool()),
        ValueKind::Byte => match value {
            Value::Byte(_) => value,
            other => Value::Byte(other.as_byte()),
        },
        ValueKind::Int => match value {
            Value::Int(_) => value,
            other => Value::Int(other.as_int()),
        },
        ValueKind::Float => match value {
            Value::Float(_) => value,
            other => Value::Float(other.as_float()),
        },
        ValueKind::Str => match value {
            Value::Str(_) => value,
            other => Value::Str(other.as_str()),
        },
        ValueKind::Name => match value {
            Value::Name(_) => value,
            other => Value::Name(other.as_name()),
        },
        _ => value,
    }
}

fn run(vm: &mut Vm, host: &mut dyn Host, frame: &mut Frame, body: &Arc<Disassembly>) -> Result<Option<Value>, Error> {
    let mut pc = 0;
    while let Some(instruction) = body.instructions.get(pc) {
        frame.offset = instruction.offset;
        vm.steps += 1;
        if vm.steps > STEP_BUDGET {
            return Err(fail(frame, vm, "laço sem fim"));
        }
        match step(vm, host, frame, &instruction.expr)? {
            Flow::Next => pc += 1,
            Flow::Jump(target) => {
                let meta = vm.functions.get(&frame.function).ok_or_else(|| fail(frame, vm, "função sumiu"))?;
                pc = *meta.offsets.get(&target).ok_or_else(|| fail(frame, vm, format!("salto para {target:#x}")))?;
            }
            Flow::Return(value) => return Ok(Some(value)),
        }
    }
    Ok(None)
}

fn step(vm: &mut Vm, host: &mut dyn Host, frame: &mut Frame, expr: &Expr) -> Result<Flow, Error> {
    Ok(match expr {
        Expr::Return(value) => Flow::Return(eval(vm, host, frame, value)?),
        Expr::ReturnNothing | Expr::Stop | Expr::EndOfScript => Flow::Return(Value::None),
        Expr::Jump(target) => Flow::Jump(usize::from(*target)),
        Expr::JumpIfNot { target, condition } => {
            if eval(vm, host, frame, condition)?.as_bool() {
                Flow::Next
            } else {
                Flow::Jump(usize::from(*target))
            }
        }
        Expr::Switch { value, .. } => {
            frame.switch = Some(eval(vm, host, frame, value)?);
            Flow::Next
        }
        Expr::Case { target, value: Some(value) } => {
            let candidate = eval(vm, host, frame, value)?;
            let current = frame.switch.clone().unwrap_or_default();
            if values_equal(&current, &candidate) { Flow::Next } else { Flow::Jump(usize::from(*target)) }
        }
        Expr::Case { value: None, .. } | Expr::Nothing | Expr::EndFunctionParms => Flow::Next,
        Expr::LabelTable(_) => Flow::Next,
        Expr::GotoLabel(_) | Expr::Iterator { .. } | Expr::IteratorPop | Expr::IteratorNext => {
            return Err(fail(frame, vm, "token sem suporte (labels/iterator)"));
        }
        Expr::Assert { condition, .. } => {
            eval(vm, host, frame, condition)?;
            Flow::Next
        }
        other => {
            eval(vm, host, frame, other)?;
            Flow::Next
        }
    })
}

pub fn values_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Str(a), Value::Str(b)) | (Value::Name(a), Value::Name(b)) => a.eq_ignore_ascii_case(b),
        (Value::Float(a), Value::Float(b)) => a == b,
        (Value::Int(_) | Value::Byte(_) | Value::Bool(_), Value::Int(_) | Value::Byte(_) | Value::Bool(_)) => {
            left.as_int() == right.as_int()
        }
        (Value::Float(_), Value::Int(_) | Value::Byte(_)) | (Value::Int(_) | Value::Byte(_), Value::Float(_)) => {
            left.as_float() == right.as_float()
        }
        (Value::None, Value::None) => true,
        (Value::None, Value::Object(_)) | (Value::Object(_), Value::None) => false,
        _ => left == right,
    }
}

fn local_slot(vm: &Vm, frame: &Frame, object: ObjectRef) -> Option<usize> {
    let global = vm.resolve(frame.package, object)?;
    vm.functions.get(&frame.function)?.local_index.get(&global).copied()
}

fn instance_slot(vm: &Vm, this: ObjId, package: usize, object: ObjectRef) -> Option<usize> {
    let global = vm.resolve(package, object)?;
    let class = vm.objects.get(this.0)?.class;
    vm.classes.get(&class)?.layout_index.get(&global).copied()
}

fn default_slot(vm: &Vm, class: GlobalRef, package: usize, object: ObjectRef) -> Option<usize> {
    let global = vm.resolve(package, object)?;
    vm.classes.get(&class)?.layout_index.get(&global).copied()
}

fn prop_name(vm: &Vm, package: usize, object: ObjectRef) -> String {
    vm.resolve(package, object).map_or_else(String::new, |global| vm.object_name(global).to_string())
}

pub fn place(vm: &mut Vm, host: &mut dyn Host, frame: &mut Frame, expr: &Expr) -> Result<Place, Error> {
    Ok(match expr {
        Expr::LocalVariable(object) | Expr::NativeParm(object) => {
            Place::Local(local_slot(vm, frame, *object).ok_or_else(|| fail(frame, vm, "local desconhecida"))?)
        }
        Expr::InstanceVariable(object) => {
            let slot = instance_slot(vm, frame.this, frame.package, *object).ok_or_else(|| {
                fail(frame, vm, format!("propriedade {} desconhecida", prop_name(vm, frame.package, *object)))
            })?;
            Place::Instance(frame.this, slot)
        }
        Expr::DefaultVariable(object) => {
            let class = vm.objects.get(frame.this.0).map(|o| o.class).ok_or_else(|| fail(frame, vm, "self sumiu"))?;
            let slot = default_slot(vm, class, frame.package, *object)
                .ok_or_else(|| fail(frame, vm, "propriedade padrão desconhecida"))?;
            Place::Default(class, slot)
        }
        Expr::BoolVariable(inner) | Expr::Skip { expr: inner, .. } | Expr::LineNumber { expr: inner, .. } => {
            place(vm, host, frame, inner)?
        }
        Expr::StructMember { property, expr } => {
            let base = place(vm, host, frame, expr)?;
            Place::Field(Box::new(base), prop_name(vm, frame.package, *property))
        }
        Expr::ArrayElement { index, array } | Expr::DynArrayElement { index, array } => {
            let index = eval(vm, host, frame, index)?.as_int();
            let base = place(vm, host, frame, array)?;
            Place::Element(Box::new(base), usize::try_from(index).unwrap_or(usize::MAX))
        }
        Expr::Context { object, member, .. } => {
            let target = eval(vm, host, frame, object)?;
            match target {
                Value::Object(id) => {
                    let saved = frame.this;
                    frame.this = id;
                    let result = place(vm, host, frame, member);
                    frame.this = saved;
                    result?
                }
                _ => Place::Temp(Value::None),
            }
        }
        Expr::ClassContext { object, member, .. } => {
            let class = match eval(vm, host, frame, object)? {
                Value::Class(class) => class,
                _ => return Ok(Place::Temp(Value::None)),
            };
            match member.as_ref() {
                Expr::DefaultVariable(object) | Expr::InstanceVariable(object) => {
                    let slot = default_slot(vm, class, frame.package, *object)
                        .ok_or_else(|| fail(frame, vm, "propriedade padrão de classe desconhecida"))?;
                    Place::Default(class, slot)
                }
                other => Place::Temp(eval(vm, host, frame, other)?),
            }
        }
        other => Place::Temp(eval(vm, host, frame, other)?),
    })
}

pub fn read_place(vm: &Vm, frame: &Frame, place: &Place) -> Value {
    match place {
        Place::Local(index) => frame.locals.get(*index).cloned().unwrap_or_default(),
        Place::Instance(id, index) => {
            vm.objects.get(id.0).and_then(|object| object.props.get(*index)).cloned().unwrap_or_default()
        }
        Place::Default(class, index) => {
            vm.classes.get(class).and_then(|meta| meta.default_values.get(*index)).cloned().unwrap_or_default()
        }
        Place::Field(base, field) => read_place(vm, frame, base).field(field),
        Place::Element(base, index) => match read_place(vm, frame, base) {
            Value::Array(items) => items.get(*index).cloned().unwrap_or_default(),
            _ => Value::None,
        },
        Place::Temp(value) => value.clone(),
    }
}

pub fn write_place(vm: &mut Vm, frame: &mut Frame, place: &Place, value: Value) {
    match place {
        Place::Local(index) => {
            if let Some(slot) = frame.locals.get_mut(*index) {
                *slot = value;
            }
        }
        Place::Instance(id, index) => {
            if let Some(slot) = vm.objects.get_mut(id.0).and_then(|object| object.props.get_mut(*index)) {
                *slot = value;
            }
        }
        Place::Default(class, index) => {
            if let Some(slot) = vm.classes.get_mut(class).and_then(|meta| meta.default_values.get_mut(*index)) {
                *slot = value;
            }
        }
        Place::Field(base, field) => {
            let mut container = read_place(vm, frame, base);
            match &mut container {
                Value::Struct(structure) => structure.set(field, value),
                other => {
                    let mut structure = StructValue::default();
                    structure.set(field, value);
                    *other = Value::Struct(Box::new(structure));
                }
            }
            write_place(vm, frame, base, container);
        }
        Place::Element(base, index) => {
            let mut container = read_place(vm, frame, base);
            match &mut container {
                Value::Array(items) => {
                    if *index < items.len() {
                        items[*index] = value;
                    } else if *index < 65_536 {
                        items.resize(*index + 1, Value::None);
                        items[*index] = value;
                    }
                }
                other => {
                    if *index < 65_536 {
                        let mut items = vec![Value::None; *index + 1];
                        items[*index] = value;
                        *other = Value::Array(items);
                    }
                }
            }
            write_place(vm, frame, base, container);
        }
        Place::Temp(_) => {}
    }
}

pub fn eval(vm: &mut Vm, host: &mut dyn Host, frame: &mut Frame, expr: &Expr) -> Result<Value, Error> {
    Ok(match expr {
        Expr::LocalVariable(_)
        | Expr::NativeParm(_)
        | Expr::InstanceVariable(_)
        | Expr::DefaultVariable(_)
        | Expr::StructMember { .. }
        | Expr::ArrayElement { .. }
        | Expr::DynArrayElement { .. } => {
            let target = place(vm, host, frame, expr)?;
            read_place(vm, frame, &target)
        }
        Expr::BoolVariable(inner) => Value::Bool(eval(vm, host, frame, inner)?.as_bool()),
        Expr::Skip { expr, .. } | Expr::LineNumber { expr, .. } | Expr::EatString(expr) => eval(vm, host, frame, expr)?,
        Expr::Let { target, value } | Expr::LetBool { target, value } | Expr::LetDelegate { target, value } => {
            let value = eval(vm, host, frame, value)?;
            if let Expr::DynArrayLength(array) = target.as_ref() {
                let wanted = value.as_int().max(0) as usize;
                let target = place(vm, host, frame, array)?;
                let filler = array_filler(vm, frame, array);
                let mut current = read_place(vm, frame, &target);
                match &mut current {
                    Value::Array(items) => items.resize(wanted.min(65_536), filler),
                    other => *other = Value::Array(vec![filler; wanted.min(65_536)]),
                }
                write_place(vm, frame, &target, current);
                return Ok(value);
            }
            let target = place(vm, host, frame, target)?;
            let value = coerce_to_place(vm, frame, &target, value);
            write_place(vm, frame, &target, value.clone());
            value
        }
        Expr::Nothing
        | Expr::EndFunctionParms
        | Expr::NoObject
        | Expr::EmptyDelegate
        | Expr::IteratorPop
        | Expr::IteratorNext => Value::None,
        Expr::SelfRef => Value::Object(frame.this),
        Expr::IntConst(v) => Value::Int(*v),
        Expr::FloatConst(v) => Value::Float(*v),
        Expr::StringConst(s) | Expr::UnicodeStringConst(s) => Value::Str(s.clone()),
        Expr::ObjectConst(object) => match vm.resolve(frame.package, *object) {
            Some(global) if vm.classes.contains_key(&global) => Value::Class(global),
            Some(global) => Value::Name(vm.object_name(global).to_string()),
            None => Value::Name(vm.packages[frame.package].script.package.object_name(*object).to_string()),
        },
        Expr::NameConst(n) | Expr::DelegateProperty(n) => Value::Name(n.clone()),
        Expr::RotationConst(a, b, c) => Value::rotator(*a, *b, *c),
        Expr::VectorConst(a, b, c) => Value::vector(*a, *b, *c),
        Expr::RangeConst(a, b) => Value::vector(*a, *b, 0.0),
        Expr::ByteConst(v) => Value::Byte(*v),
        Expr::IntConstByte(v) => Value::Int(i32::from(*v)),
        Expr::IntZero => Value::Int(0),
        Expr::IntOne => Value::Int(1),
        Expr::True => Value::Bool(true),
        Expr::False => Value::Bool(false),
        Expr::DynArrayLength(inner) => match eval(vm, host, frame, inner)? {
            Value::Array(items) => Value::Int(items.len() as i32),
            _ => Value::Int(0),
        },
        Expr::DynArrayInsert { array, index, count } => {
            let index = eval(vm, host, frame, index)?.as_int().max(0) as usize;
            let count = eval(vm, host, frame, count)?.as_int().max(0) as usize;
            let target = place(vm, host, frame, array)?;
            let mut value = read_place(vm, frame, &target);
            let filler = array_filler(vm, frame, array);
            if let Value::Array(items) = &mut value {
                let at = index.min(items.len());
                for _ in 0..count.min(65_536) {
                    items.insert(at, filler.clone());
                }
            } else {
                value = Value::Array(vec![filler; count.min(65_536)]);
            }
            write_place(vm, frame, &target, value);
            Value::None
        }
        Expr::DynArrayRemove { array, index, count } => {
            let index = eval(vm, host, frame, index)?.as_int().max(0) as usize;
            let count = eval(vm, host, frame, count)?.as_int().max(0) as usize;
            let target = place(vm, host, frame, array)?;
            let mut value = read_place(vm, frame, &target);
            if let Value::Array(items) = &mut value {
                let end = index.saturating_add(count).min(items.len());
                if index < end {
                    items.drain(index..end);
                }
            }
            write_place(vm, frame, &target, value);
            Value::None
        }
        Expr::StructCmpEq { left, right, .. }
        | Expr::DelegateCmpEq { left, right }
        | Expr::DelegateFunctionCmpEq { left, right } => {
            let a = eval(vm, host, frame, left)?;
            let b = eval(vm, host, frame, right)?;
            Value::Bool(values_equal(&a, &b))
        }
        Expr::StructCmpNe { left, right, .. }
        | Expr::DelegateCmpNe { left, right }
        | Expr::DelegateFunctionCmpNe { left, right } => {
            let a = eval(vm, host, frame, left)?;
            let b = eval(vm, host, frame, right)?;
            Value::Bool(!values_equal(&a, &b))
        }
        Expr::PrimitiveCast { cast, expr } => {
            let value = eval(vm, host, frame, expr)?;
            cast_value(vm, *cast, value)
        }
        Expr::DynamicCast { class, expr } => {
            let value = eval(vm, host, frame, expr)?;
            let target = vm.resolve(frame.package, *class);
            match (&value, target) {
                (Value::Object(id), Some(target)) => {
                    let class = vm.objects.get(id.0).map(|o| o.class);
                    if class.is_some_and(|class| vm.is_child_of(class, target)) { value } else { Value::None }
                }
                (Value::Handle(handle), Some(target)) => {
                    let class = vm.class_name(target).to_string();
                    vm.handle_classes.insert(handle.clone(), class);
                    value
                }
                (Value::Handle(_), None) => value,
                _ => Value::None,
            }
        }
        Expr::MetaCast { class, expr } => {
            let value = eval(vm, host, frame, expr)?;
            let target = vm.resolve(frame.package, *class);
            match (&value, target) {
                (Value::Class(actual), Some(target)) => {
                    if vm.is_child_of(*actual, target) {
                        value
                    } else {
                        Value::None
                    }
                }
                _ => Value::None,
            }
        }
        Expr::Conditional { condition, if_true, if_false, .. } => {
            if eval(vm, host, frame, condition)?.as_bool() {
                eval(vm, host, frame, if_true)?
            } else {
                eval(vm, host, frame, if_false)?
            }
        }
        Expr::Context { object, member, .. } => {
            let target = eval(vm, host, frame, object)?;
            match target {
                Value::Object(id) => {
                    let saved = frame.this;
                    frame.this = id;
                    let result = eval(vm, host, frame, member);
                    frame.this = saved;
                    result?
                }
                Value::Handle(_) => call_on_handle(vm, host, frame, &target, member)?,
                Value::Struct(_) => match member.as_ref() {
                    Expr::StructMember { property, .. } => target.field(&prop_name(vm, frame.package, *property)),
                    _ => Value::None,
                },
                _ => Value::None,
            }
        }
        Expr::ClassContext { object, member, .. } => {
            let class = eval(vm, host, frame, object)?;
            let Value::Class(class) = class else { return Ok(Value::None) };
            match member.as_ref() {
                Expr::FinalFunction { function, args } => {
                    let Some(target) = vm.resolve(frame.package, *function) else {
                        return Err(fail(frame, vm, "função estática não resolvida"));
                    };
                    call_function(vm, host, frame, None, target, args)?
                }
                Expr::VirtualFunction { name, args } | Expr::GlobalFunction { name, args } => {
                    match vm.find_function(class, name, None) {
                        Some(target) => call_function(vm, host, frame, None, target, args)?,
                        None => {
                            let class_name = vm.class_name(class).to_string();
                            call_unknown(vm, host, frame, &class_name, name, None, args)?
                        }
                    }
                }
                Expr::DefaultVariable(object) | Expr::InstanceVariable(object) => {
                    let slot = default_slot(vm, class, frame.package, *object)
                        .ok_or_else(|| fail(frame, vm, "propriedade padrão de classe desconhecida"))?;
                    read_place(vm, frame, &Place::Default(class, slot))
                }
                other => eval(vm, host, frame, other)?,
            }
        }
        Expr::FinalFunction { function, args } => {
            let Some(target) = vm.resolve(frame.package, *function) else {
                let name = vm.packages[frame.package].script.package.object_name(*function).to_string();
                let class =
                    vm.objects.get(frame.this.0).map(|o| vm.class_name(o.class).to_string()).unwrap_or_default();
                return call_unknown(vm, host, frame, &class, &name, Some(Value::Object(frame.this)), args);
            };
            call_function(vm, host, frame, Some(frame.this), target, args)?
        }
        Expr::VirtualFunction { name, args } => call_by_name(vm, host, frame, name, args, true)?,
        Expr::GlobalFunction { name, args } => call_by_name(vm, host, frame, name, args, false)?,
        Expr::DelegateFunction { name, args, .. } => call_by_name(vm, host, frame, name, args, true)?,
        Expr::Native { index, args } => super::core::call_core(vm, host, frame, *index, args)?,
        Expr::New { .. } => return Err(fail(frame, vm, "new sem suporte")),
        Expr::Iterator { .. } | Expr::GotoLabel(_) | Expr::LabelTable(_) => {
            return Err(fail(frame, vm, "token sem suporte (labels/iterator)"));
        }
        Expr::Return(_)
        | Expr::ReturnNothing
        | Expr::Stop
        | Expr::EndOfScript
        | Expr::Jump(_)
        | Expr::JumpIfNot { .. }
        | Expr::Switch { .. }
        | Expr::Case { .. }
        | Expr::Assert { .. }
        | Expr::DebugInfo { .. } => Value::None,
    })
}

fn array_filler(vm: &Vm, frame: &Frame, array: &Expr) -> Value {
    let prop = match array {
        Expr::LocalVariable(object) | Expr::InstanceVariable(object) | Expr::DefaultVariable(object) => {
            vm.resolve(frame.package, *object)
        }
        Expr::StructMember { property, .. } => vm.resolve(frame.package, *property),
        _ => None,
    };
    match prop.and_then(|global| vm.props.get(&global)) {
        Some(meta) => match &meta.kind {
            ValueKind::Array(Some(inner)) => vm.prop_default(*inner),
            _ => Value::None,
        },
        None => Value::None,
    }
}

fn coerce_to_place(vm: &Vm, frame: &Frame, target: &Place, value: Value) -> Value {
    let kind = match target {
        Place::Local(index) => vm.functions.get(&frame.function).and_then(|meta| meta.locals.get(*index)).copied(),
        Place::Instance(id, index) => vm
            .objects
            .get(id.0)
            .and_then(|object| vm.classes.get(&object.class))
            .and_then(|meta| meta.layout.get(*index))
            .copied(),
        Place::Default(class, index) => vm.classes.get(class).and_then(|meta| meta.layout.get(*index)).copied(),
        _ => None,
    };
    match kind.and_then(|global| vm.props.get(&global)) {
        Some(meta) if meta.array_dim <= 1 && !matches!(meta.kind, ValueKind::Array(_)) => coerce(value, &meta.kind),
        _ => value,
    }
}

pub fn cast_value(vm: &Vm, cast: u8, value: Value) -> Value {
    match cast {
        0x3A | 0x41 => Value::Int(value.as_int()),
        0x3B | 0x3E | 0x45 | 0x47 | 0x48 | 0x4B | 0x4F | 0x51 => Value::Bool(value.as_bool()),
        0x3C | 0x3F | 0x42 | 0x4C => Value::Float(value.as_float()),
        0x3D | 0x40 | 0x43 | 0x49 => Value::Byte(value.as_byte()),
        0x44 | 0x4A => Value::Int(value.as_int()),
        0x52 | 0x53 | 0x54 | 0x55 | 0x57 | 0x58 | 0x59 | 0x5A => Value::Str(value.as_str()),
        0x56 => Value::Str(match &value {
            Value::Object(id) => vm.objects.get(id.0).map(|o| o.name.clone()).unwrap_or_default(),
            Value::Class(class) => vm.class_name(*class).to_string(),
            Value::None => "None".to_string(),
            other => other.as_str(),
        }),
        0x4D => Value::vector(0.0, 0.0, 0.0),
        0x4E | 0x39 | 0x50 => Value::rotator(0, 0, 0),
        0x60 => Value::Name(value.as_name()),
        _ => value,
    }
}

fn call_by_name(
    vm: &mut Vm,
    host: &mut dyn Host,
    frame: &mut Frame,
    name: &str,
    args: &[Expr],
    use_state: bool,
) -> Result<Value, Error> {
    let object = vm.objects.get(frame.this.0).ok_or_else(|| fail(frame, vm, "self sumiu"))?;
    let class = object.class;
    let state = if use_state { object.state.clone() } else { None };
    match vm.find_function(class, name, state.as_deref()) {
        Some(target) => call_function(vm, host, frame, Some(frame.this), target, args),
        None => {
            let class_name = vm.class_name(class).to_string();
            call_unknown(vm, host, frame, &class_name, name, Some(Value::Object(frame.this)), args)
        }
    }
}

fn call_on_handle(
    vm: &mut Vm,
    host: &mut dyn Host,
    frame: &mut Frame,
    handle: &Value,
    member: &Expr,
) -> Result<Value, Error> {
    match member {
        Expr::FinalFunction { function, args } => match vm.resolve(frame.package, *function) {
            Some(target) => {
                let meta = vm.functions.get(&target).cloned().ok_or_else(|| fail(frame, vm, "função sem metadados"))?;
                let class = vm.class_name(meta.class).to_string();
                call_host(vm, host, frame, &class, &meta.name, Some(handle.clone()), args, Some(&meta))
            }
            None => {
                let name = vm.packages[frame.package].script.package.object_name(*function).to_string();
                let class = match handle {
                    Value::Handle(control) => vm.handle_class_for(control, &name),
                    _ => "WindowHandle".to_string(),
                };
                call_unknown(vm, host, frame, &class, &name, Some(handle.clone()), args)
            }
        },
        Expr::VirtualFunction { name, args } | Expr::GlobalFunction { name, args } => {
            let class = match handle {
                Value::Handle(control) => vm.handle_class_for(control, name),
                _ => "WindowHandle".to_string(),
            };
            call_unknown(vm, host, frame, &class, name, Some(handle.clone()), args)
        }
        _ => Ok(Value::None),
    }
}

fn call_unknown(
    vm: &mut Vm,
    host: &mut dyn Host,
    frame: &mut Frame,
    class: &str,
    name: &str,
    this: Option<Value>,
    args: &[Expr],
) -> Result<Value, Error> {
    call_host(vm, host, frame, class, name, this, args, None)
}

#[allow(clippy::too_many_arguments)]
fn call_host(
    vm: &mut Vm,
    host: &mut dyn Host,
    frame: &mut Frame,
    class: &str,
    name: &str,
    this: Option<Value>,
    args: &[Expr],
    meta: Option<&super::model::FunctionMeta>,
) -> Result<Value, Error> {
    let mut values = Vec::with_capacity(args.len());
    let mut places = Vec::with_capacity(args.len());
    for (index, arg) in args.iter().enumerate() {
        let out = meta.and_then(|meta| meta.params.get(index)).is_some_and(|param| param.out);
        if out {
            let target = place(vm, host, frame, arg)?;
            values.push(read_place(vm, frame, &target));
            places.push(Some(target));
        } else {
            values.push(eval(vm, host, frame, arg)?);
            places.push(None);
        }
    }
    if let Some(meta) = meta {
        for (index, param) in meta.params.iter().enumerate() {
            if index >= values.len() {
                values.push(vm.prop_default(meta.locals[param.local]));
                places.push(None);
            }
        }
    }
    let result = host.call_native(class, name, this.as_ref(), &mut values).map_err(|error| Error {
        message: error.message,
        function: vm.function_name(frame.function),
        offset: frame.offset,
    })?;
    for (value, target) in values.into_iter().zip(places) {
        if let Some(target) = target {
            write_place(vm, frame, &target, value);
        }
    }
    Ok(result)
}

pub fn call_function(
    vm: &mut Vm,
    host: &mut dyn Host,
    frame: &mut Frame,
    this: Option<ObjId>,
    target: GlobalRef,
    args: &[Expr],
) -> Result<Value, Error> {
    let meta = vm.functions.get(&target).cloned().ok_or_else(|| fail(frame, vm, "função sem metadados"))?;
    if meta.native_index != 0 {
        return super::core::call_core(vm, host, frame, meta.native_index, args);
    }
    if meta.is_native() {
        let class = vm.class_name(meta.class).to_string();
        let this_value = this.map(Value::Object);
        return call_host(vm, host, frame, &class, &meta.name, this_value, args, Some(&meta));
    }
    let mut values = Vec::with_capacity(args.len());
    let mut places = Vec::with_capacity(args.len());
    for (index, arg) in args.iter().enumerate() {
        let out = meta.params.get(index).is_some_and(|param| param.out);
        if out {
            let target = place(vm, host, frame, arg)?;
            values.push(read_place(vm, frame, &target));
            places.push(Some(target));
        } else {
            values.push(eval(vm, host, frame, arg)?);
            places.push(None);
        }
    }
    let callee_this = this.unwrap_or(frame.this);
    let mut locals: Vec<Value> = meta.locals.iter().map(|local| vm.prop_default(*local)).collect();
    for (index, param) in meta.params.iter().enumerate() {
        if let Some(value) = values.get(index) {
            let kind = vm.props.get(&meta.locals[param.local]).map(|p| p.kind.clone()).unwrap_or(ValueKind::Other);
            locals[param.local] = coerce(value.clone(), &kind);
        }
    }
    if vm.depth >= super::MAX_DEPTH {
        return Err(fail(frame, vm, "recursão profunda"));
    }
    let mut callee =
        Frame { this: callee_this, function: target, package: target.package, locals, switch: None, offset: 0 };
    vm.depth += 1;
    let result = run(vm, host, &mut callee, &meta.body);
    vm.depth -= 1;
    let returned = result?;
    for (index, param) in meta.params.iter().enumerate() {
        if let Some(Some(target)) = places.get(index) {
            let value = callee.locals.get(param.local).cloned().unwrap_or_default();
            write_place(vm, frame, target, value);
        }
    }
    Ok(match (returned, meta.return_local) {
        (Some(value), _) => value,
        (None, Some(index)) => callee.locals.get(index).cloned().unwrap_or_default(),
        (None, None) => Value::None,
    })
}
