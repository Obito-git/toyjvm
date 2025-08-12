use std::collections::HashMap;
use crate::class_file::ClassFile;

pub mod java;

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.4
pub struct MethodArea {
    pub classes: HashMap<String, ClassFile>,
}
