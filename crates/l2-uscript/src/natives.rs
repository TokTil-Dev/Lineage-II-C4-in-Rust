use crate::Script;
use crate::objects::Kind;
use std::collections::HashMap;

pub const FUNC_PRE_OPERATOR: u32 = 0x10;
pub const FUNC_NATIVE: u32 = 0x400;
pub const FUNC_OPERATOR: u32 = 0x1000;
pub const FUNC_STATIC: u32 = 0x2000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeInfo {
    pub index: u16,
    pub name: String,
    pub friendly_name: String,
    pub class: String,
    pub package: String,
    pub flags: u32,
    pub operator_precedence: u8,
    pub parameters: usize,
}

impl NativeInfo {
    pub fn is_operator(&self) -> bool {
        self.flags & FUNC_OPERATOR != 0
    }

    pub fn is_pre_operator(&self) -> bool {
        self.flags & FUNC_PRE_OPERATOR != 0
    }

    pub fn display_name(&self) -> &str {
        if self.friendly_name.is_empty() { &self.name } else { &self.friendly_name }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Natives {
    pub by_index: HashMap<u16, NativeInfo>,
}

impl Natives {
    pub fn add(&mut self, script: &Script, package_name: &str) -> usize {
        let mut added = 0;
        for object in &script.objects {
            let Kind::Function(function) = &object.kind else { continue };
            if function.native_index == 0 {
                continue;
            }
            let class =
                script.object_name(script.package.outer(ue2_package::ObjectRef::Export(object.export))).to_string();
            let parameters = script
                .children(ue2_package::ObjectRef::Export(object.export))
                .iter()
                .filter(|child| matches!(&child.kind, Kind::Property(property) if property.flags & 0x80 != 0 && property.flags & 0x400 == 0))
                .count();
            self.by_index.entry(function.native_index).or_insert_with(|| {
                added += 1;
                NativeInfo {
                    index: function.native_index,
                    name: object.name.clone(),
                    friendly_name: function.body.friendly_name.clone(),
                    class,
                    package: package_name.to_string(),
                    flags: function.flags,
                    operator_precedence: function.operator_precedence,
                    parameters,
                }
            });
        }
        added
    }

    pub fn get(&self, index: u16) -> Option<&NativeInfo> {
        self.by_index.get(&index)
    }

    pub fn sorted(&self) -> Vec<&NativeInfo> {
        let mut all: Vec<&NativeInfo> = self.by_index.values().collect();
        all.sort_by_key(|info| info.index);
        all
    }
}

pub struct ScriptResolver<'a> {
    pub script: &'a Script,
    pub natives: Option<&'a Natives>,
}

impl crate::bytecode::Resolver for ScriptResolver<'_> {
    fn object(&self, object: ue2_package::ObjectRef) -> String {
        match object {
            ue2_package::ObjectRef::Null => "None".to_string(),
            other => self.script.object_name(other).to_string(),
        }
    }

    fn native(&self, index: u16) -> Option<(String, bool)> {
        let info = self.natives?.get(index)?;
        Some((info.display_name().to_string(), info.is_operator() || info.is_pre_operator()))
    }
}
