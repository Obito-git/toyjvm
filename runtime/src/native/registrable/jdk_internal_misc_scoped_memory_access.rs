use crate::VirtualMachineDeprecated;
use crate::native::NativeRet;
use crate::stack_deprecated::FrameStackDeprecated;
use common::jtype::Value;
use log::debug;

pub(super) fn jdk_internal_misc_scoped_memory_access_register_natives(
    _vm: &mut VirtualMachineDeprecated,
    _frame_stack: &FrameStackDeprecated,

    _args: &[Value],
) -> NativeRet {
    debug!("TODO: Stub: jdk.internal.misc.ScopedMemoryAccess.registerNatives");
    Ok(None)
}
