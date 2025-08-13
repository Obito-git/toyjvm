use std::fmt::{self, Display, Formatter};
use common::ByteCursor;
use crate::ClassFileErr;
use crate::runtime_constant_pool::RuntimeConstantPool;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LineNumberEntry {
    start_pc: u16,
    line_number: u16,
}
#[derive(Debug, Clone, PartialEq, Eq)]
struct LocalVariableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.7

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassAttribute {
    SourceFile { sourcefile_index: u16 },
    Unknown { name_index: u16, info: Vec<u8> },
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodAttribute {
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<CodeAttribute>,
    },
    Unknown {
        name_index: u16,
        info: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeAttribute {
    LineNumberTable(Vec<LineNumberEntry>),
    LocalVariableTable(Vec<LocalVariableEntry>),
    Unknown { name_index: u16, info: Vec<u8> },
}
//attribute_name_index: u16,
//info: Vec<u8>,

//TODO: where to put?
pub const ATTR_CODE: &[u8] = b"Code";
pub const ATTR_LOCAL_VARIABLE_TABLE: &[u8] = b"LocalVariableTable";
pub const ATTR_LINE_NUMBER_TABLE: &[u8] = b"LineNumberTable";
pub const ATTR_SOURCE_FILE: &[u8] = b"SourceFile";

impl<'a> ClassAttribute {
    pub(crate) fn read(
        constant_pool: &RuntimeConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let attribute_length = cursor.u32()? as usize;

        let utf8 = constant_pool.get_utf8(attribute_name_index)?.as_bytes();
        match utf8 {
            ATTR_SOURCE_FILE => Ok(ClassAttribute::SourceFile {
                sourcefile_index: cursor.u16()?,
            }),
            _ => {
                let mut buf = vec![0u8; attribute_length];
                cursor.read_exact(&mut buf)?;
                Ok(ClassAttribute::Unknown {
                    name_index: attribute_name_index,
                    info: buf,
                })
            }
        }
    }
}

impl<'a> MethodAttribute {
    pub(crate) fn read(
        constant_pool: &RuntimeConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let attribute_length = cursor.u32()? as usize;

        let utf8 = constant_pool.get_utf8(attribute_name_index)?.as_bytes();
        match utf8 {
            ATTR_CODE => {
                let max_stack = cursor.u16()?;
                let max_locals = cursor.u16()?;
                let code_length = cursor.u32()? as usize;

                let mut code = vec![0u8; code_length];
                cursor.read_exact(&mut code)?;

                let exception_table_length = cursor.u16()? as usize;
                let mut exception_table = Vec::with_capacity(exception_table_length);
                for _ in 0..exception_table_length {
                    exception_table.push(ExceptionTableEntry {
                        start_pc: cursor.u16()?,
                        end_pc: cursor.u16()?,
                        handler_pc: cursor.u16()?,
                        catch_type: cursor.u16()?,
                    });
                }

                let attributes_count = cursor.u16()? as usize;
                let mut attributes = Vec::with_capacity(attributes_count);
                for _ in 0..attributes_count {
                    attributes.push(CodeAttribute::read(constant_pool, cursor)?);
                }

                Ok(MethodAttribute::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                })
            }
            _ => {
                let mut buf = vec![0u8; attribute_length];
                cursor.read_exact(&mut buf)?;
                Ok(MethodAttribute::Unknown {
                    name_index: attribute_name_index,
                    info: buf,
                })
            }
        }
    }
}

impl<'a> CodeAttribute {
    pub(crate) fn read(
        constant_pool: &RuntimeConstantPool,
        cursor: &mut ByteCursor<'a>,
    ) -> Result<Self, ClassFileErr> {
        let attribute_name_index = cursor.u16()?;
        let attribute_length = cursor.u32()? as usize;

        let utf8 = constant_pool.get_utf8(attribute_name_index)?.as_bytes();
        match utf8 {
            ATTR_LINE_NUMBER_TABLE => {
                let line_number_table_length = cursor.u16()? as usize;
                let mut line_number_table = Vec::with_capacity(line_number_table_length);
                for _ in 0..line_number_table_length {
                    line_number_table.push(LineNumberEntry {
                        start_pc: cursor.u16()?,
                        line_number: cursor.u16()?,
                    });
                }
                Ok(CodeAttribute::LineNumberTable(line_number_table))
            }
            ATTR_LOCAL_VARIABLE_TABLE => {
                let local_variable_table_length = cursor.u16()?;
                let mut local_variable_table =
                    Vec::with_capacity(local_variable_table_length as usize);
                for _ in 0..local_variable_table_length {
                    let start_pc = cursor.u16()?;
                    let length = cursor.u16()?;
                    let name_index = cursor.u16()?;
                    let descriptor_index = cursor.u16()?;
                    let index = cursor.u16()?;
                    local_variable_table.push(LocalVariableEntry {
                        start_pc,
                        length,
                        name_index,
                        descriptor_index,
                        index,
                    });
                }
                Ok(CodeAttribute::LocalVariableTable(local_variable_table))
            }
            _ => {
                let mut buf = vec![0u8; attribute_length];
                cursor.read_exact(&mut buf)?;
                Ok(CodeAttribute::Unknown {
                    name_index: attribute_name_index,
                    info: buf,
                })
            }
        }
    }
}

impl Display for ClassAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ClassAttribute::SourceFile { sourcefile_index } => {
                write!(f, "SourceFile(sourcefile_index: {})", sourcefile_index)
            }
            ClassAttribute::Unknown { name_index, info } => write!(
                f,
                "Unsupported(name_index: {}, data: {} bytes)",
                name_index,
                info.len()
            ),
        }
    }
}

impl Display for MethodAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MethodAttribute::Code {
                max_stack,
                max_locals,
                code,
                exception_table,
                attributes,
            } => {
                let code_str = code
                    .iter()
                    .map(|b| format!("{:02X}", b))
                    .collect::<Vec<_>>()
                    .join(" ");

                write!(
                    f,
                    "Code(max_stack: {}, max_locals: {}, code: \"{}\"",
                    max_stack, max_locals, code_str
                )?;

                if !exception_table.is_empty() {
                    write!(f, ", exception_table: {:?} ", exception_table)?;
                }
                if !attributes.is_empty() {
                    write!(f, ", attributes: [")?;
                    for (i, attr) in attributes.iter().enumerate() {
                        if i > 0 {
                            write!(f, ", ")?;
                        }
                        write!(f, "{}", attr)?;
                    }
                    write!(f, "]")?;
                }
                write!(f, ")")
            }
            MethodAttribute::Unknown { name_index, info } => write!(
                f,
                "Unsupported(name_index: {}, data: {} bytes)",
                name_index,
                info.len()
            ),
        }
    }
}

impl Display for CodeAttribute {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            CodeAttribute::LineNumberTable(table) => {
                write!(f, "LineNumberTable{:?}", table)
            }
            CodeAttribute::LocalVariableTable(table) => {
                write!(f, "LocalVariableTable{:?}", table)
            }
            CodeAttribute::Unknown { name_index, info } => write!(
                f,
                "Unsupported(name_index: {}, data: {} bytes)",
                name_index,
                info.len()
            ),
        }
    }
}
