use class_file::ClassFile;
use std::collections::HashMap;

mod method_descriptor;

/// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.4
pub struct MethodArea {
    pub classes: HashMap<String, ClassFile>,
}
