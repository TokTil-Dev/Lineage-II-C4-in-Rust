pub mod bytecode;
pub mod natives;
pub mod objects;
pub mod vm;

use std::fmt;

pub use bytecode::{Disassembly, Expr, Instruction};
pub use natives::{NativeInfo, Natives};
pub use objects::{
    Class, Dependency, Function, Kind, Object, Property, PropertyKind, PropertyTag, State, StateFrame, Struct,
    TagValue, TextBuffer,
};
pub use ue2_package::{Export, ObjectRef, Package};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Package(String),
    Read(ue2_core::ReadError),
    UnknownToken { export: usize, file_offset: usize, token: u8 },
    ScriptOverrun { export: usize, expected: usize, consumed: usize },
    SerialMismatch { export: usize, expected: usize, consumed: usize },
    UnknownCastOrSize { export: usize, file_offset: usize, value: u8 },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Package(error) => write!(f, "pacote: {error}"),
            Self::Read(error) => write!(f, "leitura: {error}"),
            Self::UnknownToken { export, file_offset, token } => {
                write!(f, "export {export}: token {token:#04x} desconhecido em {file_offset:#x}")
            }
            Self::ScriptOverrun { export, expected, consumed } => {
                write!(f, "export {export}: script com {consumed} bytes de memória, esperado {expected}")
            }
            Self::SerialMismatch { export, expected, consumed } => {
                write!(f, "export {export}: consumidos {consumed} de {expected} bytes")
            }
            Self::UnknownCastOrSize { export, file_offset, value } => {
                write!(f, "export {export}: valor {value:#04x} inesperado em {file_offset:#x}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<ue2_package::Error> for Error {
    fn from(error: ue2_package::Error) -> Self {
        Self::Package(error.to_string())
    }
}

impl From<ue2_core::ReadError> for Error {
    fn from(error: ue2_core::ReadError) -> Self {
        Self::Read(error)
    }
}

pub struct Script {
    pub package: Package,
    pub objects: Vec<Object>,
}

impl Script {
    pub fn parse(data: &[u8]) -> Result<Self, Error> {
        let package = Package::parse(data)?;
        let objects = (0..package.exports().len()).map(|index| objects::read_object(&package, data, index)).collect();
        Ok(Self { package, objects })
    }

    pub fn object_name(&self, object: ObjectRef) -> &str {
        self.package.object_name(object)
    }

    pub fn find(&self, path: &str) -> Option<&Object> {
        self.objects.iter().find(|object| object.path == path)
    }

    pub fn class(&self, name: &str) -> Option<&Object> {
        self.objects.iter().find(|object| matches!(object.kind, Kind::Class(_)) && object.name == name)
    }

    pub fn children(&self, owner: ObjectRef) -> Vec<&Object> {
        let mut out = Vec::new();
        let ObjectRef::Export(index) = owner else { return out };
        let mut next = self.objects.get(index).and_then(|object| object.kind.structure()).map(|s| s.children);
        let mut guard = 0;
        while let Some(ObjectRef::Export(child)) = next {
            let Some(object) = self.objects.get(child) else { break };
            out.push(object);
            next = object.kind.next_field();
            guard += 1;
            if guard > 100_000 {
                break;
            }
        }
        out
    }

    pub fn functions(&self, class: ObjectRef) -> Vec<&Object> {
        self.children(class).into_iter().filter(|object| matches!(object.kind, Kind::Function(_))).collect()
    }

    pub fn errors(&self) -> impl Iterator<Item = (&Object, &Error)> {
        self.objects.iter().filter_map(|object| object.error.as_ref().map(|error| (object, error)))
    }
}
