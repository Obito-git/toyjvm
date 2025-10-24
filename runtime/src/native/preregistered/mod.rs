use crate::native::preregistered::java_io::do_register_java_io_preregistered_natives;
use crate::native::preregistered::java_lang::do_register_java_lang_preregistered_natives;
use crate::native::preregistered::jdk_internal::do_register_jdk_internal_preregistered_natives;
use crate::native::preregistered::vm_internal::do_register_vm_internal_preregistered_natives;

mod java_io;
mod java_lang;
mod jdk_internal;
mod vm_internal;

pub(super) fn preregister_natives(native_registry: &mut crate::native::NativeRegistry) {
    do_register_jdk_internal_preregistered_natives(native_registry);
    do_register_java_lang_preregistered_natives(native_registry);
    do_register_java_io_preregistered_natives(native_registry);
    do_register_vm_internal_preregistered_natives(native_registry);
}
