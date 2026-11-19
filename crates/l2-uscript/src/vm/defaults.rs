use super::model::{GlobalRef, ValueKind};
use super::{Error, StructValue, Value, Vm};
use crate::objects::{PropertyTag, TagValue};
use ue2_core::Reader;

pub fn apply_tag(
    vm: &Vm,
    class: GlobalRef,
    owner: GlobalRef,
    tag: &PropertyTag,
    values: &mut [Value],
) -> Result<(), Error> {
    let meta = vm.classes.get(&class).ok_or_else(|| Error::new("classe sem layout"))?;
    let index = meta
        .layout
        .iter()
        .rposition(|prop| vm.props.get(prop).is_some_and(|p| p.name.eq_ignore_ascii_case(&tag.name)))
        .ok_or_else(|| Error::new("propriedade desconhecida"))?;
    let prop = vm.props.get(&meta.layout[index]).ok_or_else(|| Error::new("propriedade sem metadados"))?;
    let kind = prop.kind.clone();
    let value = match &tag.value {
        TagValue::Bool(b) => Value::Bool(*b),
        TagValue::Bytes(bytes) => {
            let mut reader = Reader::at(bytes, 0);
            decode(vm, owner.package, &kind, tag.kind, &mut reader)?
        }
    };
    let slot = values.get_mut(index).ok_or_else(|| Error::new("índice fora do layout"))?;
    if prop.array_dim > 1 {
        let element = usize::try_from(tag.array_index).unwrap_or(0);
        if let Value::Array(items) = slot
            && let Some(item) = items.get_mut(element)
        {
            *item = value;
        }
    } else {
        *slot = value;
    }
    Ok(())
}

fn decode(vm: &Vm, package: usize, kind: &ValueKind, tag_kind: u8, reader: &mut Reader<'_>) -> Result<Value, Error> {
    let read = |error: ue2_core::ReadError| Error::new(format!("leitura do valor padrão: {error}"));
    Ok(match kind {
        ValueKind::Bool => Value::Bool(reader.u8().map_err(read)? != 0),
        ValueKind::Byte => Value::Byte(reader.u8().map_err(read)?),
        ValueKind::Int => Value::Int(reader.i32().map_err(read)?),
        ValueKind::Float => Value::Float(reader.f32().map_err(read)?),
        ValueKind::Str => Value::Str(reader.string().map_err(read)?),
        ValueKind::Name => {
            let raw = reader.compact().map_err(read)?;
            Value::Name(
                vm.packages[package].script.package.name_at(raw).map_err(|e| Error::new(e.to_string()))?.to_string(),
            )
        }
        ValueKind::Object | ValueKind::Class | ValueKind::Delegate | ValueKind::Other => {
            let raw = reader.compact().map_err(read)?;
            let object = vm.packages[package].script.package.object_at(raw).map_err(|e| Error::new(e.to_string()))?;
            match vm.resolve(package, object) {
                Some(global) if vm.classes.contains_key(&global) => Value::Class(global),
                Some(global) => Value::Name(vm.object_name(global).to_string()),
                None => Value::None,
            }
        }
        ValueKind::Struct(structure) => match tag_kind {
            11 => Value::vector(reader.f32().map_err(read)?, reader.f32().map_err(read)?, reader.f32().map_err(read)?),
            12 => Value::rotator(reader.i32().map_err(read)?, reader.i32().map_err(read)?, reader.i32().map_err(read)?),
            _ => decode_struct(vm, package, *structure, reader)?,
        },
        ValueKind::Array(inner) => {
            let count = usize::try_from(reader.compact().map_err(read)?).unwrap_or(0);
            let inner_meta = inner.and_then(|inner| vm.props.get(&inner));
            let mut items = Vec::with_capacity(count.min(4096));
            for _ in 0..count {
                let Some(inner_meta) = inner_meta else { break };
                items.push(decode(vm, package, &inner_meta.kind, 0, reader)?);
            }
            Value::Array(items)
        }
    })
}

fn decode_struct(
    vm: &Vm,
    package: usize,
    structure: Option<GlobalRef>,
    reader: &mut Reader<'_>,
) -> Result<Value, Error> {
    let Some(structure) = structure else { return Ok(Value::Struct(Box::default())) };
    let name = vm.structs.get(&structure).map(|meta| meta.name.clone()).unwrap_or_default();
    let mut fields = Vec::new();
    for field in vm.struct_fields(structure) {
        let Some(prop) = vm.props.get(&field) else { continue };
        let value = if prop.array_dim > 1 {
            let mut items = Vec::new();
            for _ in 0..prop.array_dim {
                items.push(decode(vm, package, &prop.kind, 0, reader)?);
            }
            Value::Array(items)
        } else {
            decode(vm, package, &prop.kind, 0, reader)?
        };
        fields.push((prop.name.clone(), value));
    }
    Ok(Value::Struct(Box::new(StructValue { name, fields })))
}
