use std::fmt;

use attribute::AttributeInfo;
use constant_pool::ConstantInfo;
use cursor::Cursor;
use field::FieldInfo;
use method::MethodInfo;

mod attribute;
mod constant_pool;
mod cursor;
mod field;
mod method;

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html
#[derive(Debug)]
pub struct ClassFile {
    minor_version: u16,
    major_version: u16,
    constant_pool: Vec<ConstantInfo>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<FieldInfo>,
    methods: Vec<MethodInfo>,
    attributes: Vec<AttributeInfo>,
}

#[derive(Debug)]
pub enum ParseError {
    WrongMagic,
    UnexpectedEof,
    UnknownTag(u8),
    TrailingBytes,
    MissingAttributeInConstantPoll,
    AttributeShouldBeUtf8,
}

impl ClassFile {
    const MAGIC: u32 = 0xCAFEBABE;
    fn validate_magic(val: u32) -> Result<(), ParseError> {
        (val == ClassFile::MAGIC)
            .then_some(())
            .ok_or(ParseError::WrongMagic)
    }
}

impl TryFrom<Vec<u8>> for ClassFile {
    type Error = ParseError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&value);
        let magic = cursor.u32()?;
        ClassFile::validate_magic(magic)?;
        let minor_version = cursor.u16()?;
        let major_version = cursor.u16()?;
        let constant_pool_count = cursor.u16()?;
        let mut constant_pool = Vec::with_capacity(constant_pool_count as usize);
        for _ in 1..constant_pool_count {
            constant_pool.push(ConstantInfo::read(&mut cursor)?);
        }
        let access_flags = cursor.u16()?;
        let this_class = cursor.u16()?;
        let super_class = cursor.u16()?;
        let interfaces_count = cursor.u16()?;
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(cursor.u16()?);
        }
        let fields_count = cursor.u16()?;
        let mut fields = Vec::with_capacity(fields_count as usize);
        for _ in 0..fields_count {
            fields.push(FieldInfo::read(&constant_pool, &mut cursor)?);
        }
        let methods_count = cursor.u16()?;
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(MethodInfo::read(&constant_pool, &mut cursor)?);
        }
        let attributes_count = cursor.u16()?;
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(AttributeInfo::read(&constant_pool, &mut cursor)?);
        }

        if cursor.u8().is_ok() {
            Err(ParseError::TrailingBytes)
        } else {
            Ok(Self {
                minor_version,
                major_version,
                constant_pool,
                access_flags,
                this_class,
                super_class,
                interfaces,
                fields,
                methods,
                attributes,
            })
        }
    }
}

impl fmt::Display for ClassFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ClassFile {{")?;
        writeln!(
            f,
            "  version: {}.{}",
            self.major_version, self.minor_version
        )?;
        writeln!(f, "  access_flags: {}", self.access_flags)?;
        writeln!(f, "  this_class: {}", self.this_class)?;
        writeln!(f, "  super_class: {}", self.super_class)?;

        writeln!(f, "  constant_pool ({}):", self.constant_pool.len())?;
        for (i, item) in self.constant_pool.iter().enumerate() {
            writeln!(f, "    {}: {}", i + 1, item)?;
        }

        writeln!(
            f,
            "  interfaces ({}): {:?}",
            self.interfaces.len(),
            self.interfaces
        )?;

        writeln!(f, "  fields ({}):", self.fields.len())?;
        for field in &self.fields {
            writeln!(f, "    {}", field)?;
        }

        writeln!(f, "  methods ({}):", self.methods.len())?;
        for method in &self.methods {
            writeln!(f, "    {}", method)?;
        }

        writeln!(f, "  attributes ({}):", self.attributes.len())?;
        for attr in &self.attributes {
            writeln!(f, "    {:?}", attr)?;
        }

        write!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
