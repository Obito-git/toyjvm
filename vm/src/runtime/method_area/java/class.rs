use crate::runtime::method_area::java::access::class::ClassAccessFlag;
use crate::runtime::method_area::java::field::Field;
use crate::runtime::method_area::java::method::Method;
use std::rc::Rc;
use class_file::ClassFile;
use class_file::constant_pool::{ClassReference, ConstantInfo};
use crate::JvmError;

#[derive(Debug)]
pub struct Class {
    this: Rc<ClassReference>,
    access: ClassAccessFlag,
    super_class: Rc<ClassReference>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    cp: Vec<ConstantInfo>,
    initialized: bool,
}

impl Class {
    pub fn new(cf: ClassFile) -> Result<Self, JvmError> {
        let cp = &cf.constant_pool;
        let this = cp.get_class(cf.this_class)?.clone();
        let super_class = cp.get_class(cf.super_class)?.clone();
        let access = ClassAccessFlag(cf.access_flags);
        let methods = cf.methods.iter().map(|method| {
            Method::new(method, &cp)
        }).collect::<Result<Vec<_>, _>>()?;

        Ok(Class {
            this,
            access,
            super_class,
            fields: vec![],
            methods,
            cp: vec![],
            initialized: false,
        })
    }
}
