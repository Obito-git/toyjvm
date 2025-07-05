use core::fmt;

use num_enum::TryFromPrimitive;

use crate::{Cursor, ParseError};

#[derive(Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum ConstantTag {
    Utf8 = 1,
    Integer = 3,
    Float = 4,
    Long = 5,
    Double = 6,
    Class = 7,
    String = 8,
    FieldRef = 9,
    MethodRef = 10,
    InterfaceMethodRef = 11,
    NameAndType = 12,
    MethodHandle = 15,
    MethodType = 16,
    Dynamic = 17,
    InvokeDynamic = 18,
    Module = 19,
    Package = 20,
}

#[derive(Debug)]
pub enum ConstantInfo {
    Utf8(Vec<u8>),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(u16),
    String(u16),
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    InterfaceRef {
        class_index: u16,
        name_and_type_index: u16,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
}

impl<'a> ConstantInfo {
    pub(crate) fn read(cursor: &mut Cursor<'a>) -> Result<Self, ParseError> {
        let raw_tag = cursor.u8()?;
        let tag = ConstantTag::try_from_primitive(raw_tag)
            .map_err(|_| ParseError::UnknownTag(raw_tag))?;
        match tag {
            ConstantTag::Utf8 => {
                let len = cursor.u16()?;
                let bytes = cursor.bytes(len as usize)?;
                Ok(ConstantInfo::Utf8(bytes))
            }
            ConstantTag::Integer => todo!(),
            ConstantTag::Float => todo!(),
            ConstantTag::Long => todo!(),
            ConstantTag::Double => todo!(),
            ConstantTag::Class => Ok(ConstantInfo::Class(cursor.u16()?)),
            ConstantTag::String => Ok(ConstantInfo::String(cursor.u16()?)),
            ConstantTag::FieldRef => Ok(ConstantInfo::FieldRef {
                class_index: cursor.u16()?,
                name_and_type_index: cursor.u16()?,
            }),
            ConstantTag::MethodRef => Ok(ConstantInfo::MethodRef {
                class_index: cursor.u16()?,
                name_and_type_index: cursor.u16()?,
            }),
            ConstantTag::InterfaceMethodRef => Ok(ConstantInfo::InterfaceRef {
                class_index: cursor.u16()?,
                name_and_type_index: cursor.u16()?,
            }),
            ConstantTag::NameAndType => Ok(ConstantInfo::NameAndType {
                name_index: cursor.u16()?,
                descriptor_index: cursor.u16()?,
            }),
            ConstantTag::MethodHandle => todo!(),
            ConstantTag::MethodType => todo!(),
            ConstantTag::Dynamic => todo!(),
            ConstantTag::InvokeDynamic => todo!(),
            ConstantTag::Module => todo!(),
            ConstantTag::Package => todo!(),
        }
    }
}

impl fmt::Display for ConstantInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantInfo::Utf8(bytes) => match std::str::from_utf8(bytes) {
                Ok(s) => write!(f, "Utf8(\"{}\")", s),
                Err(_) => write!(f, "Utf8(<invalid utf8: {:?}>)", bytes),
            },
            ConstantInfo::Integer(i) => write!(f, "Integer({})", i),
            ConstantInfo::Float(fl) => write!(f, "Float({})", fl),
            ConstantInfo::Long(l) => write!(f, "Long({})", l),
            ConstantInfo::Double(d) => write!(f, "Double({})", d),
            ConstantInfo::Class(index) => write!(f, "Class(index: {})", index),
            ConstantInfo::String(index) => write!(f, "String(index: {})", index),
            ConstantInfo::MethodRef {
                class_index,
                name_and_type_index,
            } => {
                write!(
                    f,
                    "MethodRef(class: {}, name_and_type: {})",
                    class_index, name_and_type_index
                )
            }
            ConstantInfo::FieldRef {
                class_index,
                name_and_type_index,
            } => {
                write!(
                    f,
                    "FieldRef(class: {}, name_and_type: {})",
                    class_index, name_and_type_index
                )
            }
            ConstantInfo::InterfaceRef {
                class_index,
                name_and_type_index,
            } => {
                write!(
                    f,
                    "InterfaceRef(class: {}, name_and_type: {})",
                    class_index, name_and_type_index
                )
            }
            ConstantInfo::NameAndType {
                name_index,
                descriptor_index,
            } => {
                write!(
                    f,
                    "NameAndType(name: {}, descriptor: {})",
                    name_index, descriptor_index
                )
            }
        }
    }
}
