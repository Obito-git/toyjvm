use crate::access_flag::MethodAccessFlag;

///https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.6
pub struct JavaMethod {
    name: String,
    flags: MethodAccessFlag,
}
