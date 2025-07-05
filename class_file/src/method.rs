use std::fmt;

use crate::{attribute::AttributeInfo, constant_pool::ConstantInfo, Cursor, ParseError};

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes: Vec<AttributeInfo>,
}

impl<'a> MethodInfo {
    pub(crate) fn read(
        constant_pool: &Vec<ConstantInfo>,
        cursor: &mut Cursor<'a>,
    ) -> Result<Self, ParseError> {
        let access_flags = cursor.u16()?;
        let name_index = cursor.u16()?;
        let descriptor_index = cursor.u16()?;
        let attribute_count = cursor.u16()?;
        let mut attributes = Vec::with_capacity(attribute_count as usize);
        for _ in 0..attribute_count {
            attributes.push(AttributeInfo::read(constant_pool, cursor)?);
        }
        Ok(Self {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        })
    }
}

impl fmt::Display for MethodInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MethodInfo {} name_index={} descriptor_index={} attributes=[",
            self.access_flags, self.name_index, self.descriptor_index
        )?;

        for (i, attr) in self.attributes.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", attr)?;
        }

        write!(f, "]")
    }
}
