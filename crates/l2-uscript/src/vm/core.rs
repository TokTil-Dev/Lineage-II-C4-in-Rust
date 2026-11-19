use super::exec::{self, Frame, place, read_place, write_place};
use super::{Error, Host, Value, Vm};
use crate::bytecode::Expr;

fn fail(frame: &Frame, vm: &Vm, message: impl Into<String>) -> Error {
    Error { message: message.into(), function: vm.function_name(frame.function), offset: frame.offset }
}

fn int(args: &[Value], index: usize) -> i32 {
    args.get(index).map_or(0, Value::as_int)
}

fn float(args: &[Value], index: usize) -> f32 {
    args.get(index).map_or(0.0, Value::as_float)
}

fn text(args: &[Value], index: usize) -> String {
    args.get(index).map_or_else(String::new, Value::as_str)
}

fn chars_len(s: &str) -> i32 {
    s.chars().count() as i32
}

fn substring(s: &str, start: i32, count: Option<i32>) -> String {
    let start = start.max(0) as usize;
    let iter = s.chars().skip(start);
    match count {
        Some(count) => iter.take(count.max(0) as usize).collect(),
        None => iter.collect(),
    }
}

fn instr(haystack: &str, needle: &str) -> i32 {
    if needle.is_empty() {
        return 0;
    }
    let hay: Vec<char> = haystack.chars().collect();
    let needle: Vec<char> = needle.chars().collect();
    if needle.len() > hay.len() {
        return -1;
    }
    (0..=hay.len() - needle.len()).find(|&i| hay[i..i + needle.len()] == needle[..]).map_or(-1, |i| i as i32)
}

fn vec_op(a: &Value, b: &Value, op: impl Fn(f32, f32) -> f32) -> Value {
    let (ax, ay, az) = a.xyz();
    let (bx, by, bz) = b.xyz();
    Value::vector(op(ax, bx), op(ay, by), op(az, bz))
}

fn vec_scale(a: &Value, s: f32) -> Value {
    let (x, y, z) = a.xyz();
    Value::vector(x * s, y * s, z * s)
}

fn rot_op(a: &Value, b: &Value, op: impl Fn(i32, i32) -> i32) -> Value {
    Value::rotator(
        op(a.field("Pitch").as_int(), b.field("Pitch").as_int()),
        op(a.field("Yaw").as_int(), b.field("Yaw").as_int()),
        op(a.field("Roll").as_int(), b.field("Roll").as_int()),
    )
}

pub fn call_core(
    vm: &mut Vm,
    host: &mut dyn Host,
    frame: &mut Frame,
    index: u16,
    args: &[Expr],
) -> Result<Value, Error> {
    let Some(global) = vm.core.get(&index).copied() else {
        return Err(fail(frame, vm, format!("native {index} do Core sem definição")));
    };
    let name = vm.functions.get(&global).map(|meta| meta.name.clone()).unwrap_or_default();
    let is_compound = name.starts_with("MultiplyEqual_")
        || name.starts_with("DivideEqual_")
        || name.starts_with("AddEqual_")
        || name.starts_with("SubtractEqual_")
        || name.starts_with("AddAdd_")
        || name.starts_with("SubtractSubtract_");
    if is_compound {
        let target = place(vm, host, frame, args.first().ok_or_else(|| fail(frame, vm, "operador sem operando"))?)?;
        let current = read_place(vm, frame, &target);
        let operand = match args.get(1) {
            Some(expr) => exec::eval(vm, host, frame, expr)?,
            None => Value::None,
        };
        let (updated, result) = compound(&name, &current, &operand);
        write_place(vm, frame, &target, updated);
        return Ok(result);
    }
    match name.as_str() {
        "AndAnd_BoolBool" => {
            let a = exec::eval(vm, host, frame, args.first().ok_or_else(|| fail(frame, vm, "&& sem operando"))?)?;
            if !a.as_bool() {
                return Ok(Value::Bool(false));
            }
            let b = exec::eval(vm, host, frame, args.get(1).ok_or_else(|| fail(frame, vm, "&& sem operando"))?)?;
            return Ok(Value::Bool(b.as_bool()));
        }
        "OrOr_BoolBool" => {
            let a = exec::eval(vm, host, frame, args.first().ok_or_else(|| fail(frame, vm, "|| sem operando"))?)?;
            if a.as_bool() {
                return Ok(Value::Bool(true));
            }
            let b = exec::eval(vm, host, frame, args.get(1).ok_or_else(|| fail(frame, vm, "|| sem operando"))?)?;
            return Ok(Value::Bool(b.as_bool()));
        }
        _ => {}
    }
    let mut values = Vec::with_capacity(args.len());
    for arg in args {
        values.push(exec::eval(vm, host, frame, arg)?);
    }
    let a = values.first().cloned().unwrap_or_default();
    let b = values.get(1).cloned().unwrap_or_default();
    Ok(match name.as_str() {
        "Concat_StrStr" => Value::Str(format!("{}{}", text(&values, 0), text(&values, 1))),
        "At_StrStr" => Value::Str(format!("{} {}", text(&values, 0), text(&values, 1))),
        "EqualEqual_ObjectObject"
        | "EqualEqual_NameName"
        | "EqualEqual_BoolBool"
        | "EqualEqual_RotatorRotator"
        | "EqualEqual_VectorVector" => Value::Bool(exec::values_equal(&a, &b)),
        "NotEqual_ObjectObject"
        | "NotEqual_NameName"
        | "NotEqual_BoolBool"
        | "NotEqual_RotatorRotator"
        | "NotEqual_VectorVector" => Value::Bool(!exec::values_equal(&a, &b)),
        "Less_StrStr" => Value::Bool(text(&values, 0) < text(&values, 1)),
        "Greater_StrStr" => Value::Bool(text(&values, 0) > text(&values, 1)),
        "LessEqual_StrStr" => Value::Bool(text(&values, 0) <= text(&values, 1)),
        "GreaterEqual_StrStr" => Value::Bool(text(&values, 0) >= text(&values, 1)),
        "EqualEqual_StrStr" => Value::Bool(text(&values, 0) == text(&values, 1)),
        "NotEqual_StrStr" => Value::Bool(text(&values, 0) != text(&values, 1)),
        "ComplementEqual_StrStr" => Value::Bool(text(&values, 0).eq_ignore_ascii_case(&text(&values, 1))),
        "Len" => Value::Int(chars_len(&text(&values, 0))),
        "InStr" => Value::Int(instr(&text(&values, 0), &text(&values, 1))),
        "Mid" => Value::Str(substring(&text(&values, 0), int(&values, 1), values.get(2).map(Value::as_int))),
        "Left" => Value::Str(substring(&text(&values, 0), 0, Some(int(&values, 1)))),
        "Right" => {
            let s = text(&values, 0);
            let count = int(&values, 1).max(0);
            let start = (chars_len(&s) - count).max(0);
            Value::Str(substring(&s, start, None))
        }
        "Caps" => Value::Str(text(&values, 0).to_uppercase()),
        "Chr" => Value::Str(char::from_u32(int(&values, 0) as u32).map(String::from).unwrap_or_default()),
        "Asc" => Value::Int(text(&values, 0).chars().next().map_or(0, |c| c as i32)),
        "Not_PreBool" => Value::Bool(!a.as_bool()),
        "XorXor_BoolBool" => Value::Bool(a.as_bool() ^ b.as_bool()),
        "Complement_PreInt" => Value::Int(!a.as_int()),
        "Subtract_PreInt" => Value::Int(a.as_int().wrapping_neg()),
        "Multiply_IntInt" => Value::Int(a.as_int().wrapping_mul(b.as_int())),
        "Divide_IntInt" => Value::Int(if b.as_int() == 0 { 0 } else { a.as_int().wrapping_div(b.as_int()) }),
        "Add_IntInt" => Value::Int(a.as_int().wrapping_add(b.as_int())),
        "Subtract_IntInt" => Value::Int(a.as_int().wrapping_sub(b.as_int())),
        "LessLess_IntInt" => Value::Int(a.as_int().wrapping_shl(b.as_int() as u32)),
        "GreaterGreater_IntInt" => Value::Int(a.as_int().wrapping_shr(b.as_int() as u32)),
        "GreaterGreaterGreater_IntInt" => Value::Int(((a.as_int() as u32).wrapping_shr(b.as_int() as u32)) as i32),
        "Less_IntInt" => Value::Bool(a.as_int() < b.as_int()),
        "Greater_IntInt" => Value::Bool(a.as_int() > b.as_int()),
        "LessEqual_IntInt" => Value::Bool(a.as_int() <= b.as_int()),
        "GreaterEqual_IntInt" => Value::Bool(a.as_int() >= b.as_int()),
        "EqualEqual_IntInt" => Value::Bool(a.as_int() == b.as_int()),
        "NotEqual_IntInt" => Value::Bool(a.as_int() != b.as_int()),
        "And_IntInt" => Value::Int(a.as_int() & b.as_int()),
        "Xor_IntInt" => Value::Int(a.as_int() ^ b.as_int()),
        "Or_IntInt" => Value::Int(a.as_int() | b.as_int()),
        "Rand" => {
            let max = a.as_int();
            Value::Int(if max <= 0 { 0 } else { (vm.next_rand() % max as u32) as i32 })
        }
        "FRand" => Value::Float((vm.next_rand() % 10_000) as f32 / 10_000.0),
        "Subtract_PreFloat" => Value::Float(-a.as_float()),
        "MultiplyMultiply_FloatFloat" => Value::Float(a.as_float().powf(b.as_float())),
        "Multiply_FloatFloat" => Value::Float(a.as_float() * b.as_float()),
        "Divide_FloatFloat" => Value::Float(if b.as_float() == 0.0 { 0.0 } else { a.as_float() / b.as_float() }),
        "Percent_FloatFloat" => Value::Float(if b.as_float() == 0.0 { 0.0 } else { a.as_float() % b.as_float() }),
        "Add_FloatFloat" => Value::Float(a.as_float() + b.as_float()),
        "Subtract_FloatFloat" => Value::Float(a.as_float() - b.as_float()),
        "Less_FloatFloat" => Value::Bool(a.as_float() < b.as_float()),
        "Greater_FloatFloat" => Value::Bool(a.as_float() > b.as_float()),
        "LessEqual_FloatFloat" => Value::Bool(a.as_float() <= b.as_float()),
        "GreaterEqual_FloatFloat" => Value::Bool(a.as_float() >= b.as_float()),
        "EqualEqual_FloatFloat" => Value::Bool(a.as_float() == b.as_float()),
        "NotEqual_FloatFloat" => Value::Bool(a.as_float() != b.as_float()),
        "ComplementEqual_FloatFloat" => Value::Bool((a.as_float() - b.as_float()).abs() < 0.0001),
        "Abs" => Value::Float(a.as_float().abs()),
        "Sin" => Value::Float(a.as_float().sin()),
        "Cos" => Value::Float(a.as_float().cos()),
        "Tan" => Value::Float(a.as_float().tan()),
        "Atan" => Value::Float(a.as_float().atan2(float(&values, 1))),
        "Exp" => Value::Float(a.as_float().exp()),
        "Loge" => Value::Float(a.as_float().ln()),
        "Sqrt" => Value::Float(a.as_float().sqrt()),
        "Square" => Value::Float(a.as_float() * a.as_float()),
        "FMin" => Value::Float(a.as_float().min(b.as_float())),
        "FMax" => Value::Float(a.as_float().max(b.as_float())),
        "FClamp" => Value::Float(a.as_float().clamp(b.as_float(), float(&values, 2).max(b.as_float()))),
        "Lerp" => Value::Float(b.as_float() + a.as_float() * (float(&values, 2) - b.as_float())),
        "Smerp" => {
            let t = a.as_float();
            let t = t * t * (3.0 - 2.0 * t);
            Value::Float(b.as_float() + t * (float(&values, 2) - b.as_float()))
        }
        "Min" => Value::Int(a.as_int().min(b.as_int())),
        "Max" => Value::Int(a.as_int().max(b.as_int())),
        "Clamp" => Value::Int(a.as_int().clamp(b.as_int(), int(&values, 2).max(b.as_int()))),
        "Subtract_PreVector" => vec_scale(&a, -1.0),
        "Multiply_VectorFloat" => vec_scale(&a, b.as_float()),
        "Multiply_FloatVector" => vec_scale(&b, a.as_float()),
        "Divide_VectorFloat" => vec_scale(&a, if b.as_float() == 0.0 { 0.0 } else { 1.0 / b.as_float() }),
        "Add_VectorVector" => vec_op(&a, &b, |x, y| x + y),
        "Subtract_VectorVector" => vec_op(&a, &b, |x, y| x - y),
        "Multiply_VectorVector" => vec_op(&a, &b, |x, y| x * y),
        "Dot_VectorVector" => {
            let (ax, ay, az) = a.xyz();
            let (bx, by, bz) = b.xyz();
            Value::Float(ax * bx + ay * by + az * bz)
        }
        "Cross_VectorVector" => {
            let (ax, ay, az) = a.xyz();
            let (bx, by, bz) = b.xyz();
            Value::vector(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
        }
        "VSize" => {
            let (x, y, z) = a.xyz();
            Value::Float((x * x + y * y + z * z).sqrt())
        }
        "Normal" => {
            let (x, y, z) = a.xyz();
            let size = (x * x + y * y + z * z).sqrt();
            if size == 0.0 { Value::vector(0.0, 0.0, 0.0) } else { Value::vector(x / size, y / size, z / size) }
        }
        "Add_RotatorRotator" => rot_op(&a, &b, |x, y| x.wrapping_add(y)),
        "Subtract_RotatorRotator" => rot_op(&a, &b, |x, y| x.wrapping_sub(y)),
        "Multiply_RotatorFloat" => rot_op(&a, &Value::None, |x, _| (x as f32 * b.as_float()) as i32),
        "Multiply_FloatRotator" => rot_op(&b, &Value::None, |x, _| (x as f32 * a.as_float()) as i32),
        "Divide_RotatorFloat" => {
            rot_op(&a, &Value::None, |x, _| if b.as_float() == 0.0 { 0 } else { (x as f32 / b.as_float()) as i32 })
        }
        "VRand" | "RotRand" => Value::vector(0.0, 0.0, 0.0),
        "Log" | "Warn" => {
            let line = text(&values, 0);
            vm.log.push(line.clone());
            let mut forwarded = vec![Value::Str(line)];
            host.call_native("Core.Object", &name, Some(&Value::Object(frame.this)), &mut forwarded)?;
            Value::None
        }
        "GotoState" => {
            let state = values.first().map(Value::as_name).unwrap_or_else(|| "None".to_string());
            goto_state(vm, host, frame, &state)?;
            Value::None
        }
        "IsInState" => {
            let wanted = a.as_name();
            let current = vm.objects.get(frame.this.0).and_then(|o| o.state.clone());
            Value::Bool(current.is_some_and(|state| state.eq_ignore_ascii_case(&wanted)))
        }
        "GetStateName" => Value::Name(
            vm.objects.get(frame.this.0).and_then(|o| o.state.clone()).unwrap_or_else(|| "None".to_string()),
        ),
        "Enable" | "Disable" | "SaveConfig" => Value::None,
        "ClassIsChildOf" => match (&a, &b) {
            (Value::Class(child), Value::Class(parent)) => Value::Bool(vm.is_child_of(*child, *parent)),
            _ => Value::Bool(false),
        },
        "IsA" => {
            let wanted = a.as_name();
            let class = vm.objects.get(frame.this.0).map(|o| o.class);
            Value::Bool(class.is_some_and(|class| {
                vm.class_chain(class).iter().any(|global| vm.class_name(*global).eq_ignore_ascii_case(&wanted))
            }))
        }
        other => return Err(fail(frame, vm, format!("native {index} ({other}) do Core sem implementação"))),
    })
}

fn compound(name: &str, current: &Value, operand: &Value) -> (Value, Value) {
    let is_float = name.ends_with("_FloatFloat")
        || name.ends_with("_IntFloat") && name.starts_with("Multiply")
        || name.ends_with("_IntFloat") && name.starts_with("Divide");
    let is_vector = name.ends_with("_VectorFloat") || name.ends_with("_VectorVector");
    let is_rotator = name.ends_with("_RotatorFloat") || name.ends_with("_RotatorRotator");
    let is_byte = name.ends_with("_ByteByte") || name.ends_with("Byte");
    let pre = name.contains("_Pre");
    if is_vector {
        let updated = match name {
            "MultiplyEqual_VectorFloat" => vec_scale(current, operand.as_float()),
            "DivideEqual_VectorFloat" => {
                vec_scale(current, if operand.as_float() == 0.0 { 0.0 } else { 1.0 / operand.as_float() })
            }
            "AddEqual_VectorVector" => vec_op(current, operand, |x, y| x + y),
            "SubtractEqual_VectorVector" => vec_op(current, operand, |x, y| x - y),
            "MultiplyEqual_VectorVector" => vec_op(current, operand, |x, y| x * y),
            _ => current.clone(),
        };
        return (updated.clone(), updated);
    }
    if is_rotator {
        let updated = match name {
            "AddEqual_RotatorRotator" => rot_op(current, operand, |x, y| x.wrapping_add(y)),
            "SubtractEqual_RotatorRotator" => rot_op(current, operand, |x, y| x.wrapping_sub(y)),
            "MultiplyEqual_RotatorFloat" => {
                rot_op(current, &Value::None, |x, _| (x as f32 * operand.as_float()) as i32)
            }
            "DivideEqual_RotatorFloat" => rot_op(current, &Value::None, |x, _| {
                if operand.as_float() == 0.0 { 0 } else { (x as f32 / operand.as_float()) as i32 }
            }),
            _ => current.clone(),
        };
        return (updated.clone(), updated);
    }
    if name.ends_with("_IntFloat") {
        let value = current.as_int() as f32;
        let updated = if name.starts_with("Multiply") {
            value * operand.as_float()
        } else if operand.as_float() == 0.0 {
            0.0
        } else {
            value / operand.as_float()
        };
        let updated = Value::Int(updated as i32);
        return (updated.clone(), updated);
    }
    if is_float {
        let value = current.as_float();
        let other = operand.as_float();
        let updated = if name.starts_with("MultiplyEqual") {
            value * other
        } else if name.starts_with("DivideEqual") {
            if other == 0.0 { 0.0 } else { value / other }
        } else if name.starts_with("AddEqual") {
            value + other
        } else {
            value - other
        };
        let updated = Value::Float(updated);
        return (updated.clone(), updated);
    }
    let value = if is_byte { i32::from(current.as_byte()) } else { current.as_int() };
    let other = operand.as_int();
    let updated = if name.starts_with("MultiplyEqual") {
        value.wrapping_mul(other)
    } else if name.starts_with("DivideEqual") {
        if other == 0 { 0 } else { value.wrapping_div(other) }
    } else if name.starts_with("AddEqual") {
        value.wrapping_add(other)
    } else if name.starts_with("SubtractEqual") {
        value.wrapping_sub(other)
    } else if name.starts_with("AddAdd") {
        value.wrapping_add(1)
    } else {
        value.wrapping_sub(1)
    };
    let wrap = |v: i32| if is_byte { Value::Byte(v as u8) } else { Value::Int(v) };
    let result = if name.starts_with("AddAdd") || name.starts_with("SubtractSubtract") {
        if pre { wrap(updated) } else { wrap(value) }
    } else {
        wrap(updated)
    };
    (wrap(updated), result)
}

fn goto_state(vm: &mut Vm, host: &mut dyn Host, frame: &mut Frame, state: &str) -> Result<(), Error> {
    let this = frame.this;
    let class = vm.objects.get(this.0).map(|o| o.class).ok_or_else(|| fail(frame, vm, "self sumiu"))?;
    let previous = vm.objects.get(this.0).and_then(|o| o.state.clone());
    let next = if state.eq_ignore_ascii_case("None") || state.is_empty() { None } else { Some(state.to_string()) };
    if previous.as_deref().map(str::to_ascii_lowercase) == next.as_deref().map(str::to_ascii_lowercase) {
        return Ok(());
    }
    if previous.is_some()
        && let Some(end) = vm.find_function(class, "EndState", previous.as_deref())
    {
        exec::invoke(vm, host, this, end, Vec::new())?;
    }
    if let Some(object) = vm.objects.get_mut(this.0) {
        object.state = next.clone();
    }
    if next.is_some()
        && let Some(begin) = vm.find_function(class, "BeginState", next.as_deref())
    {
        exec::invoke(vm, host, this, begin, Vec::new())?;
    }
    Ok(())
}
