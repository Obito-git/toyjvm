use crate::ClassFileErr;
use crate::constant_pool::{
    ClassReference, ConstantInfo, FieldReference, MethodReference, NameAndTypeReference,
    StringReference,
};
use crate::descriptor::MethodDescriptor;
use crate::jtype::Type;
use dashmap::DashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

#[derive(Debug)]
pub struct RuntimeConstantPool {
    entries: Vec<ConstantInfo>,
    method_descriptors: DashMap<u16, Rc<MethodDescriptor>>,
}

impl RuntimeConstantPool {
    pub fn new(entries: Vec<ConstantInfo>) -> Self {
        Self {
            entries,
            method_descriptors: DashMap::new(),
        }
    }

    fn entry(&self, idx: u16) -> Result<&ConstantInfo, ClassFileErr> {
        self.entries
            .get(idx as usize)
            .ok_or(ClassFileErr::TypeError) //TODO: Err
    }

    pub fn get_method_descriptor(&self, idx: u16) -> Result<Rc<MethodDescriptor>, ClassFileErr> {
        if let Some(method_descriptor) = self.method_descriptors.get(&idx) {
            return Ok(method_descriptor.clone());
        }
        let descriptor = self.get_utf8(idx)?;
        let method_descriptor = Rc::new(MethodDescriptor::try_from(descriptor.as_str())?);
        self.method_descriptors
            .insert(idx, method_descriptor.clone());
        Ok(method_descriptor)
    }

    pub fn get_utf8(&self, idx: u16) -> Result<&Rc<String>, ClassFileErr> {
        match self.entry(idx)? {
            ConstantInfo::Utf8(string) => Ok(string),
            _ => Err(ClassFileErr::TypeError),
        }
    }

    pub fn get_string(&self, idx: u16) -> Result<&Rc<StringReference>, ClassFileErr> {
        match self.entry(idx)? {
            ConstantInfo::String(string_ref) => {
                string_ref.value.get_or_try_init::<_, ClassFileErr>(|| {
                    Ok(self.get_utf8(string_ref.string_index)?.clone())
                })?;
                Ok(string_ref)
            }
            _ => Err(ClassFileErr::TypeError),
        }
    }

    pub fn get_class(&self, idx: u16) -> Result<&Rc<ClassReference>, ClassFileErr> {
        match self.entry(idx)? {
            ConstantInfo::Class(class_ref) => {
                class_ref.name.get_or_try_init::<_, ClassFileErr>(|| {
                    Ok(self.get_utf8(class_ref.name_index)?.clone())
                })?;
                Ok(class_ref)
            }
            _ => Err(ClassFileErr::TypeError),
        }
    }

    pub fn get_method_nat(&self, idx: u16) -> Result<&Rc<NameAndTypeReference>, ClassFileErr> {
        match self.entry(idx)? {
            ConstantInfo::NameAndType(method_nat) => {
                method_nat.name.get_or_try_init::<_, ClassFileErr>(|| {
                    Ok(self.get_utf8(method_nat.name_index)?.clone())
                })?;
                method_nat
                    .raw_descriptor
                    .get_or_try_init::<_, ClassFileErr>(|| {
                        Ok(self.get_utf8(method_nat.descriptor_index)?.clone())
                    })?;
                method_nat
                    .resolved_method
                    .get_or_try_init::<_, ClassFileErr>(|| {
                        Ok(self.get_method_descriptor(method_nat.descriptor_index)?)
                    })?;
                Ok(method_nat)
            }
            _ => Err(ClassFileErr::TypeError),
        }
    }

    pub fn get_field_nat(&self, idx: u16) -> Result<&Rc<NameAndTypeReference>, ClassFileErr> {
        match self.entry(idx)? {
            ConstantInfo::NameAndType(field_nat) => {
                field_nat.name.get_or_try_init(|| {
                    Ok::<_, ClassFileErr>(self.get_utf8(field_nat.name_index)?.clone())
                })?;
                let descriptor = field_nat.raw_descriptor.get_or_try_init(|| {
                    Ok::<_, ClassFileErr>(self.get_utf8(field_nat.descriptor_index)?.clone())
                })?;
                field_nat.resolved_field.get_or_try_init(|| {
                    Ok::<_, ClassFileErr>(Rc::new(Type::try_from(descriptor.as_str())?))
                })?;
                Ok(field_nat)
            }
            _ => Err(ClassFileErr::TypeError),
        }
    }

    pub fn get_methodref(&self, idx: u16) -> Result<&Rc<MethodReference>, ClassFileErr> {
        match self.entry(idx)? {
            ConstantInfo::MethodRef(method_ref) => {
                method_ref.class.get_or_try_init(|| {
                    Ok::<_, ClassFileErr>(self.get_class(method_ref.class_index)?.clone())
                })?;
                method_ref
                    .name_and_type
                    .get_or_try_init::<_, ClassFileErr>(|| {
                        Ok(self.get_method_nat(method_ref.name_and_type_index)?.clone())
                    })?;
                Ok(method_ref)
            }
            _ => Err(ClassFileErr::TypeError),
        }
    }

    pub fn get_fieldref(&self, idx: u16) -> Result<&Rc<FieldReference>, ClassFileErr> {
        match self.entry(idx)? {
            ConstantInfo::FieldRef(field_ref) => {
                field_ref.class.get_or_try_init::<_, ClassFileErr>(|| {
                    Ok(self.get_class(field_ref.class_index)?.clone())
                })?;
                field_ref
                    .name_and_type
                    .get_or_try_init::<_, ClassFileErr>(|| {
                        Ok(self.get_method_nat(field_ref.name_and_type_index)?.clone())
                    })?;
                Ok(field_ref)
            }
            _ => Err(ClassFileErr::TypeError),
        }
    }
}

impl Display for RuntimeConstantPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let width = ((self.entries.len().saturating_sub(1)).to_string().len()).max(1) + 1;

        macro_rules! try_map {
            ($e:expr) => {
                $e.map_err(|_| fmt::Error)
            };
        }

        for (pos, entry) in self.entries.iter().enumerate().skip(1) {
            write!(f, "{p:>w$} = ", p = format_args!("#{}", pos), w = width)?;
            match entry {
                ConstantInfo::Utf8(s) => {
                    writeln!(f, "{:<16}\t{}", "Utf8", s)?;
                }
                ConstantInfo::Class(class) => {
                    let name = try_map!(self.get_utf8(class.name_index))?;
                    writeln!(
                        f,
                        "{:<16}\t#{}\t\t // {}",
                        "Class", class.name_index, &**name
                    )?;
                }
                ConstantInfo::String(sr) => {
                    let val = try_map!(self.get_utf8(sr.string_index))?;
                    writeln!(
                        f,
                        "{:<16}\t#{}\t\t // {}",
                        "String", sr.string_index, &**val
                    )?;
                }
                ConstantInfo::MethodRef(mr) => {
                    let class = try_map!(self.get_class(mr.class_index))?;
                    let nat = try_map!(self.get_method_nat(mr.name_and_type_index))?;
                    let cls_name = class.name.get().ok_or(fmt::Error)?;
                    let name = try_map!(self.get_utf8(nat.name_index))?;
                    let desc = nat.raw_descriptor.get().ok_or(fmt::Error)?;
                    writeln!(
                        f,
                        "{:<16}\t#{}.#{}\t\t // {}.\"{}\":{}",
                        "Methodref",
                        mr.class_index,
                        mr.name_and_type_index,
                        &**cls_name,
                        &**name,
                        &**desc
                    )?;
                }
                ConstantInfo::FieldRef(fr) => {
                    let class = try_map!(self.get_class(fr.class_index))?;
                    let nat = try_map!(self.get_field_nat(fr.name_and_type_index))?;
                    let cls_name = class.name.get().ok_or(fmt::Error)?;
                    let name = try_map!(self.get_utf8(nat.name_index))?;
                    let desc = nat.raw_descriptor.get().ok_or(fmt::Error)?;
                    writeln!(
                        f,
                        "{:<16}\t#{}.#{}\t\t // {}.\"{}\":{}",
                        "Fieldref",
                        fr.class_index,
                        fr.name_and_type_index,
                        &**cls_name,
                        &**name,
                        &**desc
                    )?;
                }
                ConstantInfo::NameAndType(nat) => {
                    let name = try_map!(self.get_utf8(nat.name_index))?;
                    let desc = try_map!(self.get_utf8(nat.descriptor_index))?;
                    writeln!(
                        f,
                        "{:<16}\t#{}.#{}\t\t // \"{}\":{}",
                        "NameAndType", nat.name_index, nat.descriptor_index, &**name, &**desc
                    )?;
                }
                other => {
                    writeln!(f, "{:<16}\t{:?}", "/* TODO */", other)?;
                }
            }
        }
        Ok(())
    }
}
