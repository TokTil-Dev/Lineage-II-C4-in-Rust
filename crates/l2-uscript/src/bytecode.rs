use crate::Error;
use crate::objects::Ctx;
use std::fmt;
use ue2_package::ObjectRef;

pub const EXTENDED_NATIVE: u8 = 0x60;
pub const FIRST_NATIVE: u8 = 0x70;
pub const MEM_OBJECT: usize = 4;
pub const MEM_NAME: usize = 4;

const CP1252_HIGH: [char; 32] = [
    '\u{20AC}', '\u{FFFD}', '\u{201A}', '\u{0192}', '\u{201E}', '\u{2026}', '\u{2020}', '\u{2021}', '\u{02C6}',
    '\u{2030}', '\u{0160}', '\u{2039}', '\u{0152}', '\u{FFFD}', '\u{017D}', '\u{FFFD}', '\u{FFFD}', '\u{2018}',
    '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}', '\u{2014}', '\u{02DC}', '\u{2122}', '\u{0161}',
    '\u{203A}', '\u{0153}', '\u{FFFD}', '\u{017E}', '\u{0178}',
];

pub fn cp1252(byte: u8) -> char {
    match byte {
        0x80..=0x9F => CP1252_HIGH[usize::from(byte - 0x80)],
        other => char::from(other),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub offset: usize,
    pub file_offset: usize,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Disassembly {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    LocalVariable(ObjectRef),
    InstanceVariable(ObjectRef),
    DefaultVariable(ObjectRef),
    Return(Box<Expr>),
    Switch { size: u8, value: Box<Expr> },
    Jump(u16),
    JumpIfNot { target: u16, condition: Box<Expr> },
    Stop,
    Assert { line: u16, condition: Box<Expr> },
    Case { target: u16, value: Option<Box<Expr>> },
    Nothing,
    LabelTable(Vec<(String, i32)>),
    GotoLabel(Box<Expr>),
    EatString(Box<Expr>),
    Let { target: Box<Expr>, value: Box<Expr> },
    DynArrayElement { index: Box<Expr>, array: Box<Expr> },
    New { outer: Box<Expr>, name: Box<Expr>, flags: Box<Expr>, class: Box<Expr> },
    ClassContext { object: Box<Expr>, skip: u16, size: u8, member: Box<Expr> },
    MetaCast { class: ObjectRef, expr: Box<Expr> },
    LetBool { target: Box<Expr>, value: Box<Expr> },
    LineNumber { line: u16, expr: Box<Expr> },
    EndFunctionParms,
    SelfRef,
    Skip { size: u16, expr: Box<Expr> },
    Context { object: Box<Expr>, skip: u16, size: u8, member: Box<Expr> },
    ArrayElement { index: Box<Expr>, array: Box<Expr> },
    VirtualFunction { name: String, args: Vec<Expr> },
    FinalFunction { function: ObjectRef, args: Vec<Expr> },
    IntConst(i32),
    FloatConst(f32),
    StringConst(String),
    ObjectConst(ObjectRef),
    NameConst(String),
    RotationConst(i32, i32, i32),
    VectorConst(f32, f32, f32),
    ByteConst(u8),
    IntZero,
    IntOne,
    True,
    False,
    NativeParm(ObjectRef),
    NoObject,
    IntConstByte(u8),
    BoolVariable(Box<Expr>),
    DynamicCast { class: ObjectRef, expr: Box<Expr> },
    Iterator { expr: Box<Expr>, target: u16 },
    IteratorPop,
    IteratorNext,
    StructCmpEq { structure: ObjectRef, left: Box<Expr>, right: Box<Expr> },
    StructCmpNe { structure: ObjectRef, left: Box<Expr>, right: Box<Expr> },
    UnicodeStringConst(String),
    RangeConst(f32, f32),
    StructMember { property: ObjectRef, expr: Box<Expr> },
    DynArrayLength(Box<Expr>),
    GlobalFunction { name: String, args: Vec<Expr> },
    PrimitiveCast { cast: u8, expr: Box<Expr> },
    ReturnNothing,
    DelegateCmpEq { left: Box<Expr>, right: Box<Expr> },
    DelegateCmpNe { left: Box<Expr>, right: Box<Expr> },
    DelegateFunctionCmpEq { left: Box<Expr>, right: Box<Expr> },
    DelegateFunctionCmpNe { left: Box<Expr>, right: Box<Expr> },
    EmptyDelegate,
    DynArrayInsert { array: Box<Expr>, index: Box<Expr>, count: Box<Expr> },
    DynArrayRemove { array: Box<Expr>, index: Box<Expr>, count: Box<Expr> },
    DebugInfo { version: i32, line: i32, text_pos: i32, opcode: u8 },
    DelegateFunction { property: ObjectRef, name: String, args: Vec<Expr> },
    DelegateProperty(String),
    LetDelegate { target: Box<Expr>, value: Box<Expr> },
    EndOfScript,
    Conditional { condition: Box<Expr>, skip_true: u16, if_true: Box<Expr>, skip_false: u16, if_false: Box<Expr> },
    Native { index: u16, args: Vec<Expr> },
}

struct Decoder<'c, 'a> {
    ctx: &'c mut Ctx<'a>,
    memory: usize,
}

impl Decoder<'_, '_> {
    fn u8(&mut self) -> Result<u8, Error> {
        self.memory += 1;
        self.ctx.u8()
    }

    fn u16(&mut self) -> Result<u16, Error> {
        self.memory += 2;
        self.ctx.u16()
    }

    fn i32(&mut self) -> Result<i32, Error> {
        self.memory += 4;
        self.ctx.i32()
    }

    fn f32(&mut self) -> Result<f32, Error> {
        self.memory += 4;
        self.ctx.f32()
    }

    fn object(&mut self) -> Result<ObjectRef, Error> {
        self.memory += MEM_OBJECT;
        self.ctx.object()
    }

    fn name(&mut self) -> Result<String, Error> {
        self.memory += MEM_NAME;
        self.ctx.name()
    }

    fn ansi(&mut self) -> Result<String, Error> {
        let mut bytes = Vec::new();
        loop {
            let byte = self.ctx.u8()?;
            self.memory += 1;
            if byte == 0 {
                break;
            }
            bytes.push(byte);
        }
        Ok(bytes.iter().map(|&b| cp1252(b)).collect())
    }

    fn unicode(&mut self) -> Result<String, Error> {
        let mut units = Vec::new();
        loop {
            let unit = self.ctx.u16()?;
            self.memory += 2;
            if unit == 0 {
                break;
            }
            units.push(unit);
        }
        Ok(String::from_utf16_lossy(&units))
    }

    fn boxed(&mut self) -> Result<Box<Expr>, Error> {
        Ok(Box::new(self.expr()?))
    }

    fn args(&mut self) -> Result<Vec<Expr>, Error> {
        let mut args = Vec::new();
        loop {
            let arg = self.expr()?;
            if arg == Expr::EndFunctionParms {
                return Ok(args);
            }
            args.push(arg);
        }
    }

    fn expr(&mut self) -> Result<Expr, Error> {
        let file_offset = self.ctx.pos();
        let token = self.u8()?;
        if token >= FIRST_NATIVE {
            return Ok(Expr::Native { index: u16::from(token), args: self.args()? });
        }
        if token >= EXTENDED_NATIVE {
            let low = self.u8()?;
            let index = (u16::from(token - EXTENDED_NATIVE) << 8) | u16::from(low);
            return Ok(Expr::Native { index, args: self.args()? });
        }
        Ok(match token {
            0x00 => Expr::LocalVariable(self.object()?),
            0x01 => Expr::InstanceVariable(self.object()?),
            0x02 => Expr::DefaultVariable(self.object()?),
            0x04 => Expr::Return(self.boxed()?),
            0x05 => Expr::Switch { size: self.u8()?, value: self.boxed()? },
            0x06 => Expr::Jump(self.u16()?),
            0x07 => Expr::JumpIfNot { target: self.u16()?, condition: self.boxed()? },
            0x08 => Expr::Stop,
            0x09 => Expr::Assert { line: self.u16()?, condition: self.boxed()? },
            0x0A => {
                let target = self.u16()?;
                let value = if target == u16::MAX { None } else { Some(self.boxed()?) };
                Expr::Case { target, value }
            }
            0x0B => Expr::Nothing,
            0x0C => {
                let mut labels = Vec::new();
                loop {
                    let name = self.name()?;
                    let offset = self.i32()?;
                    if name == "None" {
                        break;
                    }
                    labels.push((name, offset));
                }
                Expr::LabelTable(labels)
            }
            0x0D => Expr::GotoLabel(self.boxed()?),
            0x0E => Expr::EatString(self.boxed()?),
            0x0F => Expr::Let { target: self.boxed()?, value: self.boxed()? },
            0x10 => Expr::DynArrayElement { index: self.boxed()?, array: self.boxed()? },
            0x11 => Expr::New { outer: self.boxed()?, name: self.boxed()?, flags: self.boxed()?, class: self.boxed()? },
            0x12 => {
                let object = self.boxed()?;
                let skip = self.u16()?;
                let size = self.u8()?;
                Expr::ClassContext { object, skip, size, member: self.boxed()? }
            }
            0x13 => Expr::MetaCast { class: self.object()?, expr: self.boxed()? },
            0x14 => Expr::LetBool { target: self.boxed()?, value: self.boxed()? },
            0x15 => Expr::LineNumber { line: self.u16()?, expr: self.boxed()? },
            0x16 => Expr::EndFunctionParms,
            0x17 => Expr::SelfRef,
            0x18 => Expr::Skip { size: self.u16()?, expr: self.boxed()? },
            0x19 => {
                let object = self.boxed()?;
                let skip = self.u16()?;
                let size = self.u8()?;
                Expr::Context { object, skip, size, member: self.boxed()? }
            }
            0x1A => Expr::ArrayElement { index: self.boxed()?, array: self.boxed()? },
            0x1B => Expr::VirtualFunction { name: self.name()?, args: self.args()? },
            0x1C => Expr::FinalFunction { function: self.object()?, args: self.args()? },
            0x1D => Expr::IntConst(self.i32()?),
            0x1E => Expr::FloatConst(self.f32()?),
            0x1F => Expr::StringConst(self.ansi()?),
            0x20 => Expr::ObjectConst(self.object()?),
            0x21 => Expr::NameConst(self.name()?),
            0x22 => Expr::RotationConst(self.i32()?, self.i32()?, self.i32()?),
            0x23 => Expr::VectorConst(self.f32()?, self.f32()?, self.f32()?),
            0x24 => Expr::ByteConst(self.u8()?),
            0x25 => Expr::IntZero,
            0x26 => Expr::IntOne,
            0x27 => Expr::True,
            0x28 => Expr::False,
            0x29 => Expr::NativeParm(self.object()?),
            0x2A => Expr::NoObject,
            0x2C => Expr::IntConstByte(self.u8()?),
            0x2D => Expr::BoolVariable(self.boxed()?),
            0x2E => Expr::DynamicCast { class: self.object()?, expr: self.boxed()? },
            0x2F => {
                let expr = self.boxed()?;
                Expr::Iterator { expr, target: self.u16()? }
            }
            0x30 => Expr::IteratorPop,
            0x31 => Expr::IteratorNext,
            0x32 => Expr::StructCmpEq { structure: self.object()?, left: self.boxed()?, right: self.boxed()? },
            0x33 => Expr::StructCmpNe { structure: self.object()?, left: self.boxed()?, right: self.boxed()? },
            0x34 => Expr::UnicodeStringConst(self.unicode()?),
            0x35 => Expr::RangeConst(self.f32()?, self.f32()?),
            0x36 => Expr::StructMember { property: self.object()?, expr: self.boxed()? },
            0x37 => Expr::DynArrayLength(self.boxed()?),
            0x38 => Expr::GlobalFunction { name: self.name()?, args: self.args()? },
            0x39 => Expr::PrimitiveCast { cast: self.u8()?, expr: self.boxed()? },
            0x3A => Expr::ReturnNothing,
            0x3B => Expr::DelegateCmpEq { left: self.boxed()?, right: self.boxed()? },
            0x3C => Expr::DelegateCmpNe { left: self.boxed()?, right: self.boxed()? },
            0x3D => Expr::DelegateFunctionCmpEq { left: self.boxed()?, right: self.boxed()? },
            0x3E => Expr::DelegateFunctionCmpNe { left: self.boxed()?, right: self.boxed()? },
            0x3F => Expr::EmptyDelegate,
            0x40 => Expr::DynArrayInsert { array: self.boxed()?, index: self.boxed()?, count: self.boxed()? },
            0x41 => Expr::DynArrayRemove { array: self.boxed()?, index: self.boxed()?, count: self.boxed()? },
            0x42 => {
                Expr::DebugInfo { version: self.i32()?, line: self.i32()?, text_pos: self.i32()?, opcode: self.u8()? }
            }
            0x43 => Expr::DelegateFunction { property: self.object()?, name: self.name()?, args: self.args()? },
            0x44 => Expr::DelegateProperty(self.name()?),
            0x45 => Expr::LetDelegate { target: self.boxed()?, value: self.boxed()? },
            0x47 => Expr::EndOfScript,
            0x48 => {
                let condition = self.boxed()?;
                let skip_true = self.u16()?;
                let if_true = self.boxed()?;
                let skip_false = self.u16()?;
                Expr::Conditional { condition, skip_true, if_true, skip_false, if_false: self.boxed()? }
            }
            _ => return Err(Error::UnknownToken { export: self.ctx.export, file_offset, token }),
        })
    }
}

pub fn read_script(ctx: &mut Ctx<'_>, script_size: usize) -> Result<Disassembly, Error> {
    let mut decoder = Decoder { ctx, memory: 0 };
    let mut instructions = Vec::new();
    while decoder.memory < script_size {
        let offset = decoder.memory;
        let file_offset = decoder.ctx.pos();
        let expr = decoder.expr()?;
        instructions.push(Instruction { offset, file_offset, expr });
    }
    if decoder.memory != script_size {
        return Err(Error::ScriptOverrun {
            export: decoder.ctx.export,
            expected: script_size,
            consumed: decoder.memory,
        });
    }
    Ok(Disassembly { instructions })
}

pub trait Resolver {
    fn object(&self, object: ObjectRef) -> String;
    fn native(&self, index: u16) -> Option<(String, bool)>;
}

pub struct Pretty<'a, R: Resolver + ?Sized> {
    pub expr: &'a Expr,
    pub resolver: &'a R,
}

impl<R: Resolver + ?Sized> fmt::Display for Pretty<'_, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write(f, self.expr, self.resolver)
    }
}

fn args_list<R: Resolver + ?Sized>(f: &mut fmt::Formatter<'_>, args: &[Expr], r: &R) -> fmt::Result {
    for (index, arg) in args.iter().enumerate() {
        if index > 0 {
            f.write_str(", ")?;
        }
        write(f, arg, r)?;
    }
    Ok(())
}

fn write<R: Resolver + ?Sized>(f: &mut fmt::Formatter<'_>, expr: &Expr, r: &R) -> fmt::Result {
    match expr {
        Expr::LocalVariable(o) | Expr::InstanceVariable(o) | Expr::NativeParm(o) => f.write_str(&r.object(*o)),
        Expr::DefaultVariable(o) => write!(f, "default.{}", r.object(*o)),
        Expr::Return(e) => {
            f.write_str("return ")?;
            write(f, e, r)
        }
        Expr::Switch { value, .. } => {
            f.write_str("switch (")?;
            write(f, value, r)?;
            f.write_str(")")
        }
        Expr::Jump(t) => write!(f, "goto {t:#x}"),
        Expr::JumpIfNot { target, condition } => {
            f.write_str("if (!(")?;
            write(f, condition, r)?;
            write!(f, ")) goto {target:#x}")
        }
        Expr::Stop => f.write_str("stop"),
        Expr::Assert { line, condition } => {
            write!(f, "assert@{line}(")?;
            write(f, condition, r)?;
            f.write_str(")")
        }
        Expr::Case { target, value: Some(v) } => {
            f.write_str("case ")?;
            write(f, v, r)?;
            write!(f, ": (next {target:#x})")
        }
        Expr::Case { .. } => f.write_str("default:"),
        Expr::Nothing => f.write_str("nothing"),
        Expr::LabelTable(labels) => {
            f.write_str("labels")?;
            for (name, offset) in labels {
                write!(f, " {name}@{offset:#x}")?;
            }
            Ok(())
        }
        Expr::GotoLabel(e) => {
            f.write_str("goto ")?;
            write(f, e, r)
        }
        Expr::EatString(e) | Expr::BoolVariable(e) | Expr::Skip { expr: e, .. } | Expr::LineNumber { expr: e, .. } => {
            write(f, e, r)
        }
        Expr::Let { target, value } | Expr::LetBool { target, value } | Expr::LetDelegate { target, value } => {
            write(f, target, r)?;
            f.write_str(" = ")?;
            write(f, value, r)
        }
        Expr::DynArrayElement { index, array } | Expr::ArrayElement { index, array } => {
            write(f, array, r)?;
            f.write_str("[")?;
            write(f, index, r)?;
            f.write_str("]")
        }
        Expr::New { outer, name, flags, class } => {
            f.write_str("new(")?;
            args_list(f, &[(**outer).clone(), (**name).clone(), (**flags).clone()], r)?;
            f.write_str(") ")?;
            write(f, class, r)
        }
        Expr::ClassContext { object, member, .. } => {
            write(f, object, r)?;
            f.write_str(".static.")?;
            write(f, member, r)
        }
        Expr::Context { object, member, .. } => {
            write(f, object, r)?;
            f.write_str(".")?;
            write(f, member, r)
        }
        Expr::MetaCast { class, expr } | Expr::DynamicCast { class, expr } => {
            write!(f, "{}(", r.object(*class))?;
            write(f, expr, r)?;
            f.write_str(")")
        }
        Expr::EndFunctionParms => f.write_str("<end>"),
        Expr::SelfRef => f.write_str("self"),
        Expr::VirtualFunction { name, args } | Expr::GlobalFunction { name, args } => {
            write!(f, "{name}(")?;
            args_list(f, args, r)?;
            f.write_str(")")
        }
        Expr::FinalFunction { function, args } => {
            write!(f, "{}(", r.object(*function))?;
            args_list(f, args, r)?;
            f.write_str(")")
        }
        Expr::DelegateFunction { name, args, .. } => {
            write!(f, "{name}(")?;
            args_list(f, args, r)?;
            f.write_str(")")
        }
        Expr::IntConst(v) => write!(f, "{v}"),
        Expr::FloatConst(v) => write!(f, "{v:?}"),
        Expr::StringConst(s) | Expr::UnicodeStringConst(s) => write!(f, "{s:?}"),
        Expr::ObjectConst(o) => write!(f, "'{}'", r.object(*o)),
        Expr::NameConst(n) | Expr::DelegateProperty(n) => write!(f, "'{n}'"),
        Expr::RotationConst(a, b, c) => write!(f, "rot({a},{b},{c})"),
        Expr::VectorConst(a, b, c) => write!(f, "vect({a:?},{b:?},{c:?})"),
        Expr::ByteConst(v) | Expr::IntConstByte(v) => write!(f, "{v}"),
        Expr::IntZero => f.write_str("0"),
        Expr::IntOne => f.write_str("1"),
        Expr::True => f.write_str("True"),
        Expr::False => f.write_str("False"),
        Expr::NoObject => f.write_str("None"),
        Expr::Iterator { expr, target } => {
            f.write_str("foreach ")?;
            write(f, expr, r)?;
            write!(f, " (end {target:#x})")
        }
        Expr::IteratorPop => f.write_str("iterator pop"),
        Expr::IteratorNext => f.write_str("iterator next"),
        Expr::StructCmpEq { left, right, .. }
        | Expr::DelegateCmpEq { left, right }
        | Expr::DelegateFunctionCmpEq { left, right } => {
            write(f, left, r)?;
            f.write_str(" == ")?;
            write(f, right, r)
        }
        Expr::StructCmpNe { left, right, .. }
        | Expr::DelegateCmpNe { left, right }
        | Expr::DelegateFunctionCmpNe { left, right } => {
            write(f, left, r)?;
            f.write_str(" != ")?;
            write(f, right, r)
        }
        Expr::RangeConst(a, b) => write!(f, "range({a:?},{b:?})"),
        Expr::StructMember { property, expr } => {
            write(f, expr, r)?;
            write!(f, ".{}", r.object(*property))
        }
        Expr::DynArrayLength(e) => {
            write(f, e, r)?;
            f.write_str(".Length")
        }
        Expr::PrimitiveCast { cast, expr } => {
            write!(f, "cast{cast:#04x}(")?;
            write(f, expr, r)?;
            f.write_str(")")
        }
        Expr::ReturnNothing => f.write_str("return"),
        Expr::EmptyDelegate => f.write_str("None"),
        Expr::DynArrayInsert { array, index, count } => {
            write(f, array, r)?;
            f.write_str(".Insert(")?;
            args_list(f, &[(**index).clone(), (**count).clone()], r)?;
            f.write_str(")")
        }
        Expr::DynArrayRemove { array, index, count } => {
            write(f, array, r)?;
            f.write_str(".Remove(")?;
            args_list(f, &[(**index).clone(), (**count).clone()], r)?;
            f.write_str(")")
        }
        Expr::DebugInfo { line, opcode, .. } => write!(f, "debug line {line} op {opcode:#04x}"),
        Expr::EndOfScript => f.write_str("<end of script>"),
        Expr::Conditional { condition, if_true, if_false, .. } => {
            write(f, condition, r)?;
            f.write_str(" ? ")?;
            write(f, if_true, r)?;
            f.write_str(" : ")?;
            write(f, if_false, r)
        }
        Expr::Native { index, args } => match r.native(*index) {
            Some((name, true)) if args.len() == 2 => {
                f.write_str("(")?;
                write(f, &args[0], r)?;
                write!(f, " {name} ")?;
                write(f, &args[1], r)?;
                f.write_str(")")
            }
            Some((name, true)) if args.len() == 1 => {
                write!(f, "{name}(")?;
                write(f, &args[0], r)?;
                f.write_str(")")
            }
            Some((name, _)) => {
                write!(f, "{name}(")?;
                args_list(f, args, r)?;
                f.write_str(")")
            }
            None => {
                write!(f, "native{index}(")?;
                args_list(f, args, r)?;
                f.write_str(")")
            }
        },
    }
}

impl Disassembly {
    pub fn pretty<R: Resolver + ?Sized>(&self, resolver: &R) -> String {
        self.instructions
            .iter()
            .map(|instruction| format!("{:#06x}: {}", instruction.offset, Pretty { expr: &instruction.expr, resolver }))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn walk<'a>(&'a self, mut visit: impl FnMut(&'a Expr)) {
        for instruction in &self.instructions {
            walk_expr(&instruction.expr, &mut visit);
        }
    }
}

pub fn walk_expr<'a>(expr: &'a Expr, visit: &mut impl FnMut(&'a Expr)) {
    visit(expr);
    let children: Vec<&Expr> = match expr {
        Expr::Return(e)
        | Expr::GotoLabel(e)
        | Expr::EatString(e)
        | Expr::BoolVariable(e)
        | Expr::DynArrayLength(e)
        | Expr::Skip { expr: e, .. }
        | Expr::LineNumber { expr: e, .. }
        | Expr::MetaCast { expr: e, .. }
        | Expr::DynamicCast { expr: e, .. }
        | Expr::StructMember { expr: e, .. }
        | Expr::PrimitiveCast { expr: e, .. }
        | Expr::Iterator { expr: e, .. }
        | Expr::Switch { value: e, .. }
        | Expr::JumpIfNot { condition: e, .. }
        | Expr::Assert { condition: e, .. } => vec![e],
        Expr::Case { value: Some(e), .. } => vec![e],
        Expr::Let { target, value } | Expr::LetBool { target, value } | Expr::LetDelegate { target, value } => {
            vec![target, value]
        }
        Expr::DynArrayElement { index, array } | Expr::ArrayElement { index, array } => vec![index, array],
        Expr::New { outer, name, flags, class } => vec![outer, name, flags, class],
        Expr::ClassContext { object, member, .. } | Expr::Context { object, member, .. } => vec![object, member],
        Expr::VirtualFunction { args, .. }
        | Expr::FinalFunction { args, .. }
        | Expr::GlobalFunction { args, .. }
        | Expr::DelegateFunction { args, .. }
        | Expr::Native { args, .. } => args.iter().collect(),
        Expr::StructCmpEq { left, right, .. }
        | Expr::StructCmpNe { left, right, .. }
        | Expr::DelegateCmpEq { left, right }
        | Expr::DelegateCmpNe { left, right }
        | Expr::DelegateFunctionCmpEq { left, right }
        | Expr::DelegateFunctionCmpNe { left, right } => vec![left, right],
        Expr::DynArrayInsert { array, index, count } | Expr::DynArrayRemove { array, index, count } => {
            vec![array, index, count]
        }
        Expr::Conditional { condition, if_true, if_false, .. } => vec![condition, if_true, if_false],
        _ => Vec::new(),
    };
    for child in children {
        walk_expr(child, visit);
    }
}
