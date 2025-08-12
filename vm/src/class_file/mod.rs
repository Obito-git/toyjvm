use crate::class_file::attribute::ClassAttribute;
use crate::runtime::data::runtime_constant_pool::RuntimeConstantPool;
use constant_pool::ConstantInfo;
use cursor::Cursor;
use field::FieldInfo;
use method::MethodInfo;
use std::fmt;

pub mod attribute;
pub mod constant_pool;
pub mod cursor;
mod field;
pub mod method;

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html
#[derive(Debug)]
pub struct ClassFile {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: RuntimeConstantPool,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>,
    pub fields: Vec<FieldInfo>,
    pub methods: Vec<MethodInfo>,
    pub attributes: Vec<ClassAttribute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodDescriptorErr {
    ShouldStartWithParentheses,
    MissingClosingParenthesis,
    UnexpectedEnd,
    InvalidType,
    TrailingCharacters,
}

#[derive(Debug)]
pub enum JvmError {
    WrongMagic,
    UnexpectedEof,
    UnknownTag(u8),
    TrailingBytes,
    MissingAttributeInConstantPoll,
    TypeError,
    MethodDescriptor(MethodDescriptorErr),
    ConstantNotFoundInRuntimePool,
}

impl ClassFile {
    const MAGIC: u32 = 0xCAFEBABE;
    fn validate_magic(val: u32) -> Result<(), JvmError> {
        (val == ClassFile::MAGIC)
            .then_some(())
            .ok_or(JvmError::WrongMagic)
    }
}

impl TryFrom<Vec<u8>> for ClassFile {
    type Error = JvmError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let mut cursor = Cursor::new(&value);
        let magic = cursor.u32()?;
        ClassFile::validate_magic(magic)?;
        let minor_version = cursor.u16()?;
        let major_version = cursor.u16()?;
        let constant_pool_count = cursor.u16()?;
        let mut constant_pool_entries = Vec::with_capacity((constant_pool_count + 1) as usize);
        constant_pool_entries.push(ConstantInfo::Dummy);
        for _ in 1..constant_pool_count {
            constant_pool_entries.push(ConstantInfo::read(&mut cursor)?);
        }
        let constant_pool = RuntimeConstantPool::new(constant_pool_entries);
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
            attributes.push(ClassAttribute::read(&constant_pool, &mut cursor)?);
        }

        if cursor.u8().is_ok() {
            Err(JvmError::TrailingBytes)
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
        writeln!(f, "  minor version: {}", self.minor_version)?;
        writeln!(f, "  major version: {}", self.major_version)?;
        writeln!(f, "  access_flags: 0x{:04X}", self.access_flags)?;
        writeln!(f, "  this_class: #{}", self.this_class)?;
        writeln!(f, "  super_class: #{}", self.super_class)?;

        writeln!(f, "  constant_pool:\n{}", self.constant_pool)?;

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
    #[test]
    fn it_works() {}
}
