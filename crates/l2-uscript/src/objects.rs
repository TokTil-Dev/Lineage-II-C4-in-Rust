use crate::Error;
use crate::bytecode::{self, Disassembly};
use ue2_core::Reader;
use ue2_package::{ObjectRef, Package};

pub const RF_HAS_STACK: u32 = 0x0200_0000;
pub const FUNC_NET: u32 = 0x40;
pub const CPF_NET: u32 = 0x20;
pub const ADDED_HIDE_CATEGORIES: u16 = 99;
pub const ADDED_CPP_TEXT: u16 = 120;
pub const MOVED_FRIENDLY_NAME: u16 = 160;
pub const CLASS_DEPENDENCIES_DEPRECATED: u16 = 186;
pub const PACKAGE_IMPORTS_DEPRECATED: u16 = 208;

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub export: usize,
    pub name: String,
    pub class: String,
    pub path: String,
    pub flags: u32,
    pub serial_offset: usize,
    pub serial_size: usize,
    pub consumed: usize,
    pub kind: Kind,
    pub error: Option<Error>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Class(Class),
    State(State),
    Function(Function),
    Struct(Struct),
    Property(Property),
    Const { value: String, next: ObjectRef },
    Enum { names: Vec<String>, next: ObjectRef },
    TextBuffer(TextBuffer),
    Other(Vec<PropertyTag>),
    Unparsed,
}

impl Kind {
    pub fn structure(&self) -> Option<&Struct> {
        match self {
            Self::Class(class) => Some(&class.state.body),
            Self::State(state) => Some(&state.body),
            Self::Function(function) => Some(&function.body),
            Self::Struct(body) => Some(body),
            _ => None,
        }
    }

    pub fn next_field(&self) -> Option<ObjectRef> {
        match self {
            Self::Property(property) => Some(property.next),
            Self::Const { next, .. } | Self::Enum { next, .. } => Some(*next),
            other => other.structure().map(|body| body.next),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateFrame {
    pub node: ObjectRef,
    pub state_node: ObjectRef,
    pub probe_mask: u64,
    pub latent_action: i32,
    pub offset: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagValue {
    Bool(bool),
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PropertyTag {
    pub name: String,
    pub kind: u8,
    pub struct_name: Option<String>,
    pub array_index: i32,
    pub value: TagValue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub frame: Option<StateFrame>,
    pub tags: Vec<PropertyTag>,
    pub super_field: ObjectRef,
    pub next: ObjectRef,
    pub script_text: ObjectRef,
    pub children: ObjectRef,
    pub friendly_name: String,
    pub cpp_text: ObjectRef,
    pub line: i32,
    pub text_pos: i32,
    pub script_size: usize,
    pub script_file_offset: usize,
    pub script_file_size: usize,
    pub script: Disassembly,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub body: Struct,
    pub native_index: u16,
    pub operator_precedence: u8,
    pub flags: u32,
    pub rep_offset: Option<u16>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct State {
    pub body: Struct,
    pub probe_mask: u64,
    pub ignore_mask: u64,
    pub label_table_offset: u16,
    pub flags: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    pub class: ObjectRef,
    pub deep: u32,
    pub script_text_crc: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub state: State,
    pub flags: u32,
    pub guid: [u8; 16],
    pub dependencies: Vec<Dependency>,
    pub package_imports: Vec<String>,
    pub within: ObjectRef,
    pub config_name: String,
    pub hide_categories: Vec<String>,
    pub defaults: Vec<PropertyTag>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyKind {
    Byte { enumeration: ObjectRef },
    Int,
    Bool,
    Float,
    Object { class: ObjectRef },
    Class { class: ObjectRef, meta: ObjectRef },
    Name,
    Str,
    Struct { structure: ObjectRef },
    Array { inner: ObjectRef },
    Delegate { function: ObjectRef },
    FixedArray { inner: ObjectRef, count: i32 },
    Map { key: ObjectRef, value: ObjectRef },
    Pointer,
    Other(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    pub frame: Option<StateFrame>,
    pub tags: Vec<PropertyTag>,
    pub super_field: ObjectRef,
    pub next: ObjectRef,
    pub array_dim: u16,
    pub element_size: u16,
    pub flags: u32,
    pub category: String,
    pub rep_offset: Option<u16>,
    pub kind: PropertyKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextBuffer {
    pub pos: u32,
    pub top: u32,
    pub text: String,
}

pub struct Ctx<'a> {
    pub package: &'a Package,
    pub export: usize,
    pub reader: Reader<'a>,
}

impl Ctx<'_> {
    pub fn name(&mut self) -> Result<String, Error> {
        let raw = self.reader.compact()?;
        Ok(self.package.name_at(raw)?.to_string())
    }

    pub fn object(&mut self) -> Result<ObjectRef, Error> {
        let raw = self.reader.compact()?;
        Ok(self.package.object_at(raw)?)
    }

    pub fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.reader.u8()?)
    }

    pub fn u16(&mut self) -> Result<u16, Error> {
        Ok(self.reader.u16()?)
    }

    pub fn u32(&mut self) -> Result<u32, Error> {
        Ok(self.reader.u32()?)
    }

    pub fn i32(&mut self) -> Result<i32, Error> {
        Ok(self.reader.i32()?)
    }

    pub fn f32(&mut self) -> Result<f32, Error> {
        Ok(self.reader.f32()?)
    }

    pub fn u64(&mut self) -> Result<u64, Error> {
        let low = u64::from(self.reader.u32()?);
        let high = u64::from(self.reader.u32()?);
        Ok(low | (high << 32))
    }

    pub fn string(&mut self) -> Result<String, Error> {
        Ok(self.reader.string()?)
    }

    pub fn count(&mut self) -> Result<usize, Error> {
        let raw = self.reader.compact()?;
        Ok(usize::try_from(raw).unwrap_or(0))
    }

    pub fn pos(&self) -> usize {
        self.reader.pos()
    }
}

pub fn read_object(package: &Package, data: &[u8], index: usize) -> Object {
    let export = &package.exports()[index];
    let name = package.object_name(ObjectRef::Export(index)).to_string();
    let class = package.class_name(export).to_string();
    let path = package.object_path(ObjectRef::Export(index));
    let mut object = Object {
        export: index,
        name,
        class,
        path,
        flags: export.flags,
        serial_offset: export.serial_offset,
        serial_size: export.serial_size,
        consumed: 0,
        kind: Kind::Unparsed,
        error: None,
    };
    if export.serial_size == 0 {
        object.kind = Kind::Other(Vec::new());
        return object;
    }
    let window = &data[export.serial_offset..export.serial_offset + export.serial_size];
    let mut ctx = Ctx { package, export: index, reader: Reader::at(window, 0) };
    match read_kind(&mut ctx, &object.class, export.flags, export.class == ObjectRef::Null) {
        Ok(kind) => {
            object.kind = kind;
            object.consumed = ctx.pos();
            if object.consumed != export.serial_size {
                object.error = Some(Error::SerialMismatch {
                    export: index,
                    expected: export.serial_size,
                    consumed: object.consumed,
                });
            }
        }
        Err(error) => {
            object.consumed = ctx.pos();
            object.error = Some(error);
        }
    }
    object
}

fn read_kind(ctx: &mut Ctx<'_>, class: &str, flags: u32, is_class: bool) -> Result<Kind, Error> {
    if is_class {
        return read_class(ctx).map(Kind::Class);
    }
    Ok(match class {
        "Function" => Kind::Function(read_function(ctx, flags)?),
        "State" => Kind::State(read_state(ctx, flags)?),
        "Struct" => Kind::Struct(read_struct(ctx, flags)?),
        "Const" => {
            let (frame, tags) = read_header(ctx, flags)?;
            let _ = (frame, tags);
            let _super = ctx.object()?;
            let next = ctx.object()?;
            Kind::Const { value: ctx.string()?, next }
        }
        "Enum" => {
            let (frame, tags) = read_header(ctx, flags)?;
            let _ = (frame, tags);
            let _super = ctx.object()?;
            let next = ctx.object()?;
            let count = ctx.count()?;
            let names = (0..count).map(|_| ctx.name()).collect::<Result<Vec<_>, _>>()?;
            Kind::Enum { names, next }
        }
        "TextBuffer" => {
            let (frame, tags) = read_header(ctx, flags)?;
            let _ = (frame, tags);
            Kind::TextBuffer(TextBuffer { pos: ctx.u32()?, top: ctx.u32()?, text: ctx.string()? })
        }
        name if name.ends_with("Property") => Kind::Property(read_property(ctx, name, flags)?),
        _ => {
            let (_frame, tags) = read_header(ctx, flags)?;
            Kind::Other(tags)
        }
    })
}

fn read_frame(ctx: &mut Ctx<'_>) -> Result<StateFrame, Error> {
    let node = ctx.object()?;
    let state_node = ctx.object()?;
    let probe_mask = ctx.u64()?;
    let latent_action = ctx.i32()?;
    let offset = if node == ObjectRef::Null { 0 } else { ctx.reader.compact()? };
    Ok(StateFrame { node, state_node, probe_mask, latent_action, offset })
}

fn read_header(ctx: &mut Ctx<'_>, flags: u32) -> Result<(Option<StateFrame>, Vec<PropertyTag>), Error> {
    let frame = if flags & RF_HAS_STACK != 0 { Some(read_frame(ctx)?) } else { None };
    let tags = read_tags(ctx)?;
    Ok((frame, tags))
}

pub fn read_tags(ctx: &mut Ctx<'_>) -> Result<Vec<PropertyTag>, Error> {
    let mut tags = Vec::new();
    loop {
        let name = ctx.name()?;
        if name == "None" {
            return Ok(tags);
        }
        let info = ctx.u8()?;
        let kind = info & 0x0F;
        let struct_name = if kind == 10 { Some(ctx.name()?) } else { None };
        let size = match info & 0x70 {
            0x00 => 1,
            0x10 => 2,
            0x20 => 4,
            0x30 => 12,
            0x40 => 16,
            0x50 => usize::from(ctx.u8()?),
            0x60 => usize::from(ctx.u16()?),
            _ => usize::try_from(ctx.i32()?).unwrap_or(0),
        };
        let mut array_index = 0;
        let value = if kind == 3 {
            TagValue::Bool(info & 0x80 != 0)
        } else {
            if info & 0x80 != 0 {
                let first = ctx.u8()?;
                array_index = if first & 0x80 == 0 {
                    i32::from(first)
                } else if first & 0xC0 == 0x80 {
                    (i32::from(first & 0x7F) << 8) + i32::from(ctx.u8()?)
                } else {
                    let c = i32::from(ctx.u8()?);
                    let d = i32::from(ctx.u8()?);
                    let e = i32::from(ctx.u8()?);
                    (i32::from(first & 0x3F) << 24) + (c << 16) + (d << 8) + e
                };
            }
            TagValue::Bytes(ctx.reader.bytes(size)?.to_vec())
        };
        tags.push(PropertyTag { name, kind, struct_name, array_index, value });
    }
}

fn read_struct(ctx: &mut Ctx<'_>, flags: u32) -> Result<Struct, Error> {
    let (frame, tags) = read_header(ctx, flags)?;
    read_struct_body(ctx, frame, tags)
}

fn read_struct_body(ctx: &mut Ctx<'_>, frame: Option<StateFrame>, tags: Vec<PropertyTag>) -> Result<Struct, Error> {
    let version = ctx.package.version;
    let super_field = ctx.object()?;
    let next = ctx.object()?;
    let script_text = ctx.object()?;
    let children = ctx.object()?;
    let friendly_name = if version < MOVED_FRIENDLY_NAME { ctx.name()? } else { String::new() };
    let cpp_text = if version >= ADDED_CPP_TEXT { ctx.object()? } else { ObjectRef::Null };
    let line = ctx.i32()?;
    let text_pos = ctx.i32()?;
    let script_size = usize::try_from(ctx.i32()?).unwrap_or(0);
    let script_file_offset = ctx.pos();
    let script = bytecode::read_script(ctx, script_size)?;
    let script_file_size = ctx.pos() - script_file_offset;
    Ok(Struct {
        frame,
        tags,
        super_field,
        next,
        script_text,
        children,
        friendly_name,
        cpp_text,
        line,
        text_pos,
        script_size,
        script_file_offset,
        script_file_size,
        script,
    })
}

fn read_function(ctx: &mut Ctx<'_>, flags: u32) -> Result<Function, Error> {
    let body = read_struct(ctx, flags)?;
    let native_index = ctx.u16()?;
    let operator_precedence = ctx.u8()?;
    let function_flags = ctx.u32()?;
    let rep_offset = if function_flags & FUNC_NET != 0 { Some(ctx.u16()?) } else { None };
    Ok(Function { body, native_index, operator_precedence, flags: function_flags, rep_offset })
}

fn read_state_body(ctx: &mut Ctx<'_>, body: Struct) -> Result<State, Error> {
    let probe_mask = ctx.u64()?;
    let ignore_mask = ctx.u64()?;
    let label_table_offset = ctx.u16()?;
    let state_flags = ctx.u32()?;
    Ok(State { body, probe_mask, ignore_mask, label_table_offset, flags: state_flags })
}

fn read_state(ctx: &mut Ctx<'_>, flags: u32) -> Result<State, Error> {
    let body = read_struct(ctx, flags)?;
    read_state_body(ctx, body)
}

fn read_class(ctx: &mut Ctx<'_>) -> Result<Class, Error> {
    let version = ctx.package.version;
    let body = read_struct_body(ctx, None, Vec::new())?;
    let state = read_state_body(ctx, body)?;
    let class_flags = ctx.u32()?;
    let mut guid = [0u8; 16];
    guid.copy_from_slice(ctx.reader.bytes(16)?);
    let mut dependencies = Vec::new();
    if version < CLASS_DEPENDENCIES_DEPRECATED {
        let count = ctx.count()?;
        for _ in 0..count {
            dependencies.push(Dependency { class: ctx.object()?, deep: ctx.u32()?, script_text_crc: ctx.u32()? });
        }
    }
    let mut package_imports = Vec::new();
    if version < PACKAGE_IMPORTS_DEPRECATED {
        let count = ctx.count()?;
        for _ in 0..count {
            package_imports.push(ctx.name()?);
        }
    }
    let within = ctx.object()?;
    let config_name = ctx.name()?;
    let mut hide_categories = Vec::new();
    if version >= ADDED_HIDE_CATEGORIES {
        let count = ctx.count()?;
        for _ in 0..count {
            hide_categories.push(ctx.name()?);
        }
    }
    let defaults = read_tags(ctx)?;
    Ok(Class {
        state,
        flags: class_flags,
        guid,
        dependencies,
        package_imports,
        within,
        config_name,
        hide_categories,
        defaults,
    })
}

fn read_property(ctx: &mut Ctx<'_>, class: &str, flags: u32) -> Result<Property, Error> {
    let (frame, tags) = read_header(ctx, flags)?;
    let super_field = ctx.object()?;
    let next = ctx.object()?;
    let dims = ctx.u32()?;
    let array_dim = (dims & 0xFFFF) as u16;
    let element_size = (dims >> 16) as u16;
    let property_flags = ctx.u32()?;
    let category = ctx.name()?;
    let rep_offset = if property_flags & CPF_NET != 0 { Some(ctx.u16()?) } else { None };
    let kind = match class {
        "ByteProperty" => PropertyKind::Byte { enumeration: ctx.object()? },
        "IntProperty" => PropertyKind::Int,
        "BoolProperty" => PropertyKind::Bool,
        "FloatProperty" => PropertyKind::Float,
        "ObjectProperty" => PropertyKind::Object { class: ctx.object()? },
        "ClassProperty" => PropertyKind::Class { class: ctx.object()?, meta: ctx.object()? },
        "NameProperty" => PropertyKind::Name,
        "StrProperty" => PropertyKind::Str,
        "StructProperty" => PropertyKind::Struct { structure: ctx.object()? },
        "ArrayProperty" => PropertyKind::Array { inner: ctx.object()? },
        "DelegateProperty" => PropertyKind::Delegate { function: ctx.object()? },
        "FixedArrayProperty" => PropertyKind::FixedArray { inner: ctx.object()?, count: ctx.i32()? },
        "MapProperty" => PropertyKind::Map { key: ctx.object()?, value: ctx.object()? },
        "PointerProperty" => PropertyKind::Pointer,
        other => PropertyKind::Other(other.to_string()),
    };
    Ok(Property {
        frame,
        tags,
        super_field,
        next,
        array_dim,
        element_size,
        flags: property_flags,
        category,
        rep_offset,
        kind,
    })
}
