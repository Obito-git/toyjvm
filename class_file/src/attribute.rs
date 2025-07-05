use std::fmt::{self, Display, Formatter};

use crate::{constant_pool::ConstantInfo, cursor::Cursor, ParseError};

#[derive(Debug)]
struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug)]
struct LineNumberEntry {
    start_pc: u16,
    line_number: u16,
}
#[derive(Debug)]
struct LocalVariableEntry {
    start_pc: u16,
    length: u16,
    name_index: u16,
    descriptor_index: u16,
    index: u16,
}

#[derive(Debug)]
pub enum AttributeInfo {
    Code {
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTableEntry>,
        attributes: Vec<AttributeInfo>,
    },
    LineNumberTable(Vec<LineNumberEntry>),
    LocalVariableTable(Vec<LocalVariableEntry>),
    Unsupported {
        name_index: u16,
        data: Vec<u8>,
    },
    SourceFile {
        sourcefile_index: u16,
    },
}
//attribute_name_index: u16,
//info: Vec<u8>,

//TODO: where to put?
pub const ATTR_CODE: &[u8] = b"Code";
pub const ATTR_LOCAL_VARIABLE_TABLE: &[u8] = b"LocalVariableTable";
pub const ATTR_LINE_NUMBER_TABLE: &[u8] = b"LineNumberTable";
pub const ATTR_SOURCE_FILE: &[u8] = b"SourceFile";
//const CODE_ATTR_NAME: &[u8] = &[b'C', b'o', b'd', b'e'];

impl<'a> AttributeInfo {
    pub(crate) fn read(
        constant_pool: &[ConstantInfo],
        cursor: &mut Cursor<'a>,
    ) -> Result<Self, ParseError> {
        let attribute_name_index = cursor.u16()?;
        let attribute_length = cursor.u32()? as usize;

        let utf8 = match constant_pool.get(attribute_name_index as usize - 1) {
            Some(ConstantInfo::Utf8(bytes)) => bytes.as_slice(),
            _ => return Err(ParseError::AttributeShouldBeUtf8),
        };

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
                Ok(AttributeInfo::LineNumberTable(line_number_table))
            }
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
                    attributes.push(AttributeInfo::read(constant_pool, cursor)?);
                }

                Ok(AttributeInfo::Code {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                })
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
                Ok(AttributeInfo::LocalVariableTable(local_variable_table))
            }
            ATTR_SOURCE_FILE => Ok(AttributeInfo::SourceFile {
                sourcefile_index: cursor.u16()?,
            }),
            _ => {
                let mut buf = vec![0u8; attribute_length];
                cursor.read_exact(&mut buf)?;
                Ok(AttributeInfo::Unsupported {
                    name_index: attribute_name_index,
                    data: buf,
                })
            }
        }
    }
}

impl Display for AttributeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AttributeInfo::Code {
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

            AttributeInfo::LineNumberTable(table) => {
                write!(f, "LineNumberTable{:?}", table)
            }

            AttributeInfo::LocalVariableTable(table) => {
                write!(f, "LocalVariableTable{:?}", table)
            }

            AttributeInfo::SourceFile { sourcefile_index } => {
                write!(f, "SourceFile(sourcefile_index: {})", sourcefile_index)
            }

            AttributeInfo::Unsupported { name_index, data } => write!(
                f,
                "Unsupported(name_index: {}, data: {} bytes)",
                name_index,
                data.len()
            ),
        }
    }
}
