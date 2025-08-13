use core::fmt;
use std::rc::Rc;
use num_enum::TryFromPrimitive;
use common::ByteCursor;
use crate::descriptor::MethodDescriptor;
use crate::jtype::Type;
use crate::ClassFileErr;

type OnceCell<I> = once_cell::unsync::OnceCell<I>;
//type OnceCell<I> = once_cell::sync::OnceCell<I>;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
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

#[derive(Debug, Clone)]
pub struct ReferenceInfo {
    pub class_index: u16,
    pub name_and_type_index: u16,
}

impl ReferenceInfo {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            class_index,
            name_and_type_index,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NameAndTypeInfo {
    pub name_index: u16,
    pub descriptor_index: u16,
}

impl NameAndTypeInfo {
    pub fn new(name_index: u16, descriptor_index: u16) -> Self {
        Self {
            name_index,
            descriptor_index,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ClassReference {
    pub name_index: u16,
    pub name: OnceCell<Rc<String>>,
}

impl ClassReference {
    pub fn new(name_index: u16) -> Self {
        Self {
            name_index,
            name: OnceCell::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct StringReference {
    pub string_index: u16,
    pub value: OnceCell<Rc<String>>,
}

impl StringReference {
    pub fn new(string_index: u16) -> Self {
        Self {
            string_index,
            value: OnceCell::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MethodReference {
    pub class_index: u16,
    pub name_and_type_index: u16,
    pub class: OnceCell<Rc<ClassReference>>,
    pub name_and_type: OnceCell<Rc<NameAndTypeReference>>,
}

impl MethodReference {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            class_index,
            name_and_type_index,
            class: OnceCell::new(),
            name_and_type: OnceCell::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FieldReference {
    pub class_index: u16,
    pub name_and_type_index: u16,
    pub class: OnceCell<Rc<ClassReference>>,
    pub name_and_type: OnceCell<Rc<NameAndTypeReference>>,
}

impl FieldReference {
    pub fn new(class_index: u16, name_and_type_index: u16) -> Self {
        Self {
            class_index,
            name_and_type_index,
            class: OnceCell::new(),
            name_and_type: OnceCell::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct NameAndTypeReference {
    pub name_index: u16,
    pub descriptor_index: u16,
    pub name: OnceCell<Rc<String>>,
    pub raw_descriptor: OnceCell<Rc<String>>,
    pub resolved_field: OnceCell<Rc<Type>>,
    pub resolved_method: OnceCell<Rc<MethodDescriptor>>,
}

impl NameAndTypeReference {
    pub fn new(name_index: u16, descriptor_index: u16) -> Self {
        Self {
            name_index,
            descriptor_index,
            name: OnceCell::new(),
            raw_descriptor: OnceCell::new(),
            resolved_field: OnceCell::new(),
            resolved_method: OnceCell::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConstantInfo {
    Dummy,
    Utf8(Rc<String>),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(Rc<ClassReference>),
    String(Rc<StringReference>),
    MethodRef(Rc<MethodReference>),
    FieldRef(Rc<FieldReference>),
    InterfaceRef(ReferenceInfo),
    NameAndType(Rc<NameAndTypeReference>),
}

impl<'a> ConstantInfo {
    pub(crate) fn read(cursor: &mut ByteCursor<'a>) -> Result<Self, ClassFileErr> {
        let raw_tag = cursor.u8()?;
        let tag =
            ConstantTag::try_from_primitive(raw_tag).map_err(|_| ClassFileErr::UnknownTag(raw_tag))?;
        let const_info = match tag {
            ConstantTag::Utf8 => {
                let len = cursor.u16()?;
                let bytes = cursor.bytes(len as usize)?;
                ConstantInfo::Utf8(Rc::new(String::from_utf8(bytes).unwrap()))
            }
            ConstantTag::Integer => todo!(),
            ConstantTag::Float => todo!(),
            ConstantTag::Long => todo!(),
            ConstantTag::Double => todo!(),
            ConstantTag::Class => ConstantInfo::Class(Rc::new(ClassReference::new(cursor.u16()?))),
            ConstantTag::String => {
                ConstantInfo::String(Rc::new(StringReference::new(cursor.u16()?)))
            }
            ConstantTag::FieldRef => {
                ConstantInfo::FieldRef(Rc::new(FieldReference::new(cursor.u16()?, cursor.u16()?)))
            }
            ConstantTag::MethodRef => {
                ConstantInfo::MethodRef(Rc::new(MethodReference::new(cursor.u16()?, cursor.u16()?)))
            }
            ConstantTag::InterfaceMethodRef => {
                ConstantInfo::InterfaceRef(ReferenceInfo::new(cursor.u16()?, cursor.u16()?))
            }
            ConstantTag::NameAndType => ConstantInfo::NameAndType(Rc::new(
                NameAndTypeReference::new(cursor.u16()?, cursor.u16()?),
            )),
            ConstantTag::Dynamic => todo!(),
            ConstantTag::InvokeDynamic => todo!(),
            ConstantTag::Module => todo!(),
            ConstantTag::Package => todo!(),
            ConstantTag::MethodHandle | ConstantTag::MethodType => todo!(),
        };
        Ok(const_info)
    }
}

impl fmt::Display for ConstantInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConstantInfo::Utf8(s) => write!(f, "Utf8(\"{}\")", s),
            ConstantInfo::Integer(i) => write!(f, "Integer({})", i),
            ConstantInfo::Float(fl) => write!(f, "Float({})", fl),
            ConstantInfo::Long(l) => write!(f, "Long({})", l),
            ConstantInfo::Double(d) => write!(f, "Double({})", d),
            ConstantInfo::Class(index) => write!(f, "Class(index: {})", index.name_index),
            ConstantInfo::String(index) => write!(f, "String(index: {})", index.string_index),
            ConstantInfo::MethodRef(ref_info) => {
                write!(
                    f,
                    "MethodRef(class: {}, name_and_type: {})",
                    ref_info.class_index, ref_info.name_and_type_index
                )
            }
            ConstantInfo::FieldRef(ref_info) => {
                write!(
                    f,
                    "FieldRef(class: {}, name_and_type: {})",
                    ref_info.class_index, ref_info.name_and_type_index
                )
            }
            ConstantInfo::InterfaceRef(ref_info) => {
                write!(
                    f,
                    "InterfaceRef(class: {}, name_and_type: {})",
                    ref_info.class_index, ref_info.name_and_type_index
                )
            }
            ConstantInfo::NameAndType(name_and_type_info) => {
                write!(
                    f,
                    "NameAndType(name: {}, descriptor: {})",
                    name_and_type_info.name_index, name_and_type_info.descriptor_index
                )
            }
            _ => {write!(f,"")},
        }
    }
}
