/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.3.2
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JavaType {
    Void,
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Instance(String), // obj or interface
    Short,
    Boolean,
    Array(Box<JavaType>),
}

impl TryFrom<char> for JavaType {
    type Error = (); // todo

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'V' => Ok(JavaType::Void),
            'B' => Ok(JavaType::Byte),
            'C' => Ok(JavaType::Char),
            'D' => Ok(JavaType::Double),
            'F' => Ok(JavaType::Float),
            'I' => Ok(JavaType::Int),
            'J' => Ok(JavaType::Long),
            'S' => Ok(JavaType::Short),
            'Z' => Ok(JavaType::Boolean),
            _ => Err(()),
        }
    }
}
