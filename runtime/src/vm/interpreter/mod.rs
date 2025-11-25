mod handlers;

use crate::keys::{ClassId, FieldKey, MethodKey};
use crate::rt::{ClassLike, JvmClass};
use crate::vm::stack::{FrameType, JavaFrame, NativeFrame};
use crate::{MethodId, ThreadId, VirtualMachine, build_exception};
use common::error::{JavaExceptionFromJvm, JvmError};
use common::instruction::Instruction;
use common::{HeapRef, Value};
use jclass::attribute::method::ExceptionTableEntry;
use std::ops::ControlFlow;
use tracing_log::log::warn;

pub struct Interpreter;

#[cfg_attr(feature = "hotpath", hotpath::measure_all)]
impl Interpreter {
    pub(crate) fn branch16(bci: usize, off: i16) -> usize {
        ((bci as isize) + (off as isize)) as usize
    }
    pub(crate) fn branch32(bci: usize, off: i32) -> usize {
        ((bci as isize) + (off as isize)) as usize
    }

    fn interpret_instruction(
        thread_id: ThreadId,
        instruction: Instruction,
        vm: &mut VirtualMachine,
    ) -> Result<ControlFlow<Option<Value>>, JvmError> {
        let is_branch = instruction.is_branch();
        let instruction_byte_size = instruction.byte_size();

        //debug_log_instruction!(&instruction, &thread_id);

        match instruction {
            Instruction::Athrow => {
                handlers::handle_athrow(thread_id, vm)?;
            }
            Instruction::Aaload => {
                handlers::handle_aaload(thread_id, vm)?;
            }
            Instruction::Aastore => {
                handlers::handle_aastore(thread_id, vm)?;
            }
            Instruction::Bastore => {
                handlers::handle_bastore(thread_id, vm)?;
            }
            Instruction::Iaload => {
                handlers::handle_iaload(thread_id, vm)?;
            }
            Instruction::Caload => {
                handlers::handle_caload(thread_id, vm)?;
            }
            Instruction::Baload => {
                handlers::handle_baload(thread_id, vm)?;
            }
            Instruction::Checkcast(idx) => {
                handlers::handle_checkcast(thread_id, idx, vm)?;
            }
            Instruction::AconstNull => {
                handlers::handle_aconst_null(thread_id, vm)?;
            }
            Instruction::Aload0 => {
                handlers::handle_aload0(thread_id, vm)?;
            }
            Instruction::Aload1 => {
                handlers::handle_aload1(thread_id, vm)?;
            }
            Instruction::Aload2 => {
                handlers::handle_aload2(thread_id, vm)?;
            }
            Instruction::Aload3 => {
                handlers::handle_aload3(thread_id, vm)?;
            }
            Instruction::Aload(pos) => {
                handlers::handle_aload(thread_id, pos, vm)?;
            }
            Instruction::Anewarray(idx) => {
                handlers::handle_anewarray(thread_id, idx, vm)?;
            }
            Instruction::ArrayLength => {
                handlers::handle_arraylength(thread_id, vm)?;
            }
            Instruction::Astore0 => {
                handlers::handle_astore0(thread_id, vm)?;
            }
            Instruction::Astore1 => {
                handlers::handle_astore1(thread_id, vm)?;
            }
            Instruction::Astore2 => {
                handlers::handle_astore2(thread_id, vm)?;
            }
            Instruction::Astore3 => {
                handlers::handle_astore3(thread_id, vm)?;
            }
            Instruction::Astore(pos) => {
                handlers::handle_astore(thread_id, pos, vm)?;
            }
            Instruction::Bipush(value) => {
                handlers::handle_bipush(thread_id, value, vm)?;
            }
            Instruction::Castore => {
                handlers::handle_castore(thread_id, vm)?;
            }
            Instruction::Dadd => {
                handlers::handle_dadd(thread_id, vm)?;
            }
            Instruction::Dconst0 => {
                handlers::handle_dconst0(thread_id, vm)?;
            }
            Instruction::Dconst1 => {
                handlers::handle_dconst1(thread_id, vm)?;
            }
            Instruction::Dup => {
                handlers::handle_dup(thread_id, vm)?;
            }
            Instruction::Dup2 => {
                handlers::handle_dup2(thread_id, vm)?;
            }
            Instruction::DupX1 => {
                handlers::handle_dup_x1(thread_id, vm)?;
            }
            Instruction::Fcmpl => {
                handlers::handle_fcmpl(thread_id, vm)?;
            }
            Instruction::Fcmpg => {
                handlers::handle_fcmpg(thread_id, vm)?;
            }
            Instruction::Fconst0 => {
                handlers::handle_fconst0(thread_id, vm)?;
            }
            Instruction::Fconst1 => {
                handlers::handle_fconst1(thread_id, vm)?;
            }
            Instruction::Fload0 => {
                handlers::handle_fload0(thread_id, vm)?;
            }
            Instruction::Fload1 => {
                handlers::handle_fload1(thread_id, vm)?;
            }
            Instruction::Fload2 => {
                handlers::handle_fload2(thread_id, vm)?;
            }
            Instruction::Fload3 => {
                handlers::handle_fload3(thread_id, vm)?;
            }
            Instruction::Fload(n) => {
                handlers::handle_fload(thread_id, n, vm)?;
            }
            Instruction::Fstore(n) => {
                handlers::handle_fstore(thread_id, n, vm)?;
            }
            Instruction::Fstore0 => {
                handlers::handle_fstore0(thread_id, vm)?;
            }
            Instruction::Fstore1 => {
                handlers::handle_fstore1(thread_id, vm)?;
            }
            Instruction::Fstore2 => {
                handlers::handle_fstore2(thread_id, vm)?;
            }
            Instruction::Fstore3 => {
                handlers::handle_fstore3(thread_id, vm)?;
            }
            Instruction::Getfield(idx) => {
                handlers::handle_getfield(thread_id, idx, vm)?;
            }
            Instruction::Getstatic(idx) => {
                handlers::handle_getstatic(thread_id, idx, vm)?;
            }
            Instruction::Goto(offset) => {
                handlers::handle_goto(thread_id, offset, vm)?;
            }
            Instruction::Iadd => {
                handlers::handle_iadd(thread_id, vm)?;
            }
            Instruction::Iconst0 => {
                handlers::handle_iconst0(thread_id, vm)?;
            }
            Instruction::Iconst1 => {
                handlers::handle_iconst1(thread_id, vm)?;
            }
            Instruction::Iconst2 => {
                handlers::handle_iconst2(thread_id, vm)?;
            }
            Instruction::Iconst3 => {
                handlers::handle_iconst3(thread_id, vm)?;
            }
            Instruction::Iconst4 => {
                handlers::handle_iconst4(thread_id, vm)?;
            }
            Instruction::Iconst5 => {
                handlers::handle_iconst5(thread_id, vm)?;
            }
            Instruction::IconstM1 => {
                handlers::handle_iconst_m1(thread_id, vm)?;
            }
            Instruction::Idiv => {
                handlers::handle_idiv(thread_id, vm)?;
            }
            Instruction::IfEq(offset) => {
                handlers::handle_if_eq(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfGe(offset) => {
                handlers::handle_if_ge(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfGt(offset) => {
                handlers::handle_if_gt(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::Lcmp => {
                handlers::handle_lcmp(thread_id, vm)?;
            }
            Instruction::Lconst0 => {
                handlers::handle_lconst0(thread_id, vm)?;
            }
            Instruction::Lconst1 => {
                handlers::handle_lconst1(thread_id, vm)?;
            }
            Instruction::Lookupswitch(ref switch) => {
                handlers::handle_lookupswitch(thread_id, switch, vm)?;
            }
            Instruction::Ifnull(offset) => {
                handlers::handle_ifnull(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfIcmplt(offset) => {
                handlers::handle_if_icmplt(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfLe(offset) => {
                handlers::handle_if_le(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfLt(offset) => {
                handlers::handle_if_lt(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfAcmpEq(offset) => {
                handlers::handle_if_acmpeq(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfAcmpNe(offset) => {
                handlers::handle_if_acmpne(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfIcmpne(offset) => {
                handlers::handle_if_icmpne(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfIcmpge(offset) => {
                handlers::handle_if_icmpge(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfIcmpgt(offset) => {
                handlers::handle_if_icmpgt(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfIcmpeq(offset) => {
                handlers::handle_if_icmpeq(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfIcmple(offset) => {
                handlers::handle_if_icmple(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::Ifnonnull(offset) => {
                handlers::handle_ifnonnull(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::IfNe(offset) => {
                handlers::handle_if_ne(thread_id, offset, instruction_byte_size, vm)?;
            }
            Instruction::Iload0 => {
                handlers::handle_iload0(thread_id, vm)?;
            }
            Instruction::Iload1 => {
                handlers::handle_iload1(thread_id, vm)?;
            }
            Instruction::Iload2 => {
                handlers::handle_iload2(thread_id, vm)?;
            }
            Instruction::Iload3 => {
                handlers::handle_iload3(thread_id, vm)?;
            }
            Instruction::Iload(pos) => {
                handlers::handle_iload(thread_id, pos, vm)?;
            }
            Instruction::InvokeVirtual(idx) => {
                handlers::handle_invokevirtual(thread_id, idx, vm)?;
            }
            Instruction::Instanceof(idx) => {
                handlers::handle_instanceof(thread_id, idx, vm)?;
            }
            Instruction::Fmul => {
                handlers::handle_fmul(thread_id, vm)?;
            }
            Instruction::Fdiv => {
                handlers::handle_fdiv(thread_id, vm)?;
            }
            Instruction::Irem => {
                handlers::handle_irem(thread_id, vm)?;
            }
            Instruction::Ladd => {
                handlers::handle_ladd(thread_id, vm)?;
            }
            Instruction::Ldiv => {
                handlers::handle_ldiv(thread_id, vm)?;
            }
            Instruction::Lmul => {
                handlers::handle_lmul(thread_id, vm)?;
            }
            Instruction::Lrem => {
                handlers::handle_lrem(thread_id, vm)?;
            }
            Instruction::Land => {
                handlers::handle_land(thread_id, vm)?;
            }
            Instruction::Lor => {
                handlers::handle_lor(thread_id, vm)?;
            }
            Instruction::Lxor => {
                handlers::handle_lxor(thread_id, vm)?;
            }
            Instruction::Iand => {
                handlers::handle_iand(thread_id, vm)?;
            }
            Instruction::Ior => {
                handlers::handle_ior(thread_id, vm)?;
            }
            Instruction::Ixor => {
                handlers::handle_ixor(thread_id, vm)?;
            }
            Instruction::L2i => {
                handlers::handle_l2i(thread_id, vm)?;
            }
            Instruction::L2f => {
                handlers::handle_l2f(thread_id, vm)?;
            }
            Instruction::D2l => {
                handlers::handle_d2l(thread_id, vm)?;
            }
            Instruction::F2i => {
                handlers::handle_f2i(thread_id, vm)?;
            }
            Instruction::F2d => {
                handlers::handle_f2d(thread_id, vm)?;
            }
            Instruction::Ineg => {
                handlers::handle_ineg(thread_id, vm)?;
            }
            Instruction::I2s => {
                handlers::handle_i2s(thread_id, vm)?;
            }
            Instruction::I2c => {
                handlers::handle_i2c(thread_id, vm)?;
            }
            Instruction::I2l => {
                handlers::handle_i2l(thread_id, vm)?;
            }
            Instruction::I2f => {
                handlers::handle_i2f(thread_id, vm)?;
            }
            Instruction::I2b => {
                handlers::handle_i2b(thread_id, vm)?;
            }
            Instruction::Istore0 => {
                handlers::handle_istore0(thread_id, vm)?;
            }
            Instruction::Istore1 => {
                handlers::handle_istore1(thread_id, vm)?;
            }
            Instruction::Istore2 => {
                handlers::handle_istore2(thread_id, vm)?;
            }
            Instruction::Istore3 => {
                handlers::handle_istore3(thread_id, vm)?;
            }
            Instruction::Istore(idx) => {
                handlers::handle_istore(thread_id, idx, vm)?;
            }
            Instruction::Isub => {
                handlers::handle_isub(thread_id, vm)?;
            }
            Instruction::Imul => {
                handlers::handle_imul(thread_id, vm)?;
            }
            Instruction::Iinc(index, const_val) => {
                handlers::handle_iinc(thread_id, index, const_val, vm)?;
            }
            Instruction::Ldc(idx) | Instruction::LdcW(idx) | Instruction::Ldc2W(idx) => {
                handlers::handle_ldc(thread_id, idx, vm)?;
            }
            Instruction::New(idx) => {
                handlers::handle_new(thread_id, idx, vm)?;
            }
            Instruction::Newarray(array_type) => {
                handlers::handle_newarray(thread_id, array_type, vm)?;
            }
            Instruction::Pop => {
                handlers::handle_pop(thread_id, vm)?;
            }
            Instruction::Putfield(idx) => {
                handlers::handle_putfield(thread_id, idx, vm)?;
            }
            Instruction::Putstatic(idx) => {
                handlers::handle_putstatic(thread_id, idx, vm)?;
            }
            Instruction::InvokeInterface(idx, count) => {
                handlers::handle_invokeinterface(thread_id, idx, count, vm)?;
            }
            Instruction::InvokeSpecial(idx) => {
                handlers::handle_invokespecial(thread_id, idx, vm)?;
            }
            Instruction::InvokeStatic(idx) => {
                handlers::handle_invokestatic(thread_id, idx, vm)?;
            }
            Instruction::Iushr => {
                handlers::handle_iushr(thread_id, vm)?;
            }
            Instruction::Lload0 => {
                handlers::handle_lload0(thread_id, vm)?;
            }
            Instruction::Lload1 => {
                handlers::handle_lload1(thread_id, vm)?;
            }
            Instruction::Lload2 => {
                handlers::handle_lload2(thread_id, vm)?;
            }
            Instruction::Lload3 => {
                handlers::handle_lload3(thread_id, vm)?;
            }
            Instruction::Lload(pos) => {
                handlers::handle_lload(thread_id, pos, vm)?;
            }
            Instruction::Lshl => {
                handlers::handle_lshl(thread_id, vm)?;
            }
            Instruction::Lstore0 => {
                handlers::handle_lstore0(thread_id, vm)?;
            }
            Instruction::Lstore1 => {
                handlers::handle_lstore1(thread_id, vm)?;
            }
            Instruction::Lstore2 => {
                handlers::handle_lstore2(thread_id, vm)?;
            }
            Instruction::Lstore3 => {
                handlers::handle_lstore3(thread_id, vm)?;
            }
            Instruction::Lstore(idx) => {
                handlers::handle_lstore(thread_id, idx, vm)?;
            }
            Instruction::Iastore => {
                handlers::handle_iastore(thread_id, vm)?;
            }
            Instruction::Ishl => {
                handlers::handle_ishl(thread_id, vm)?;
            }
            Instruction::Ishr => {
                handlers::handle_ishr(thread_id, vm)?;
            }
            Instruction::Saload => {
                handlers::handle_saload(thread_id, vm)?;
            }
            Instruction::Sastore => {
                handlers::handle_sastore(thread_id, vm)?;
            }
            Instruction::Sipush(value) => {
                handlers::handle_sipush(thread_id, value, vm)?;
            }
            Instruction::TableSwitch(ref switch) => {
                handlers::handle_tableswitch(thread_id, switch, vm)?;
            }
            Instruction::Return => {
                return Ok(ControlFlow::Break(handlers::handle_return(thread_id, vm)?));
            }
            Instruction::Ireturn => {
                return Ok(ControlFlow::Break(handlers::handle_ireturn(thread_id, vm)?));
            }
            Instruction::Areturn => {
                return Ok(ControlFlow::Break(handlers::handle_areturn(thread_id, vm)?));
            }
            Instruction::Lreturn => {
                return Ok(ControlFlow::Break(handlers::handle_lreturn(thread_id, vm)?));
            }
            Instruction::Freturn => {
                return Ok(ControlFlow::Break(handlers::handle_freturn(thread_id, vm)?));
            }
            Instruction::Monitorenter => {
                handlers::handle_monitorenter(thread_id, vm)?;
            }
            Instruction::Monitorexit => {
                handlers::handle_monitorexit(thread_id, vm)?;
            }
            instruction => unimplemented!("instruction {:?}", instruction),
        }

        if !is_branch {
            vm.get_stack_mut(&thread_id)?
                .cur_java_frame_mut()?
                .increment_pc(instruction_byte_size);
        }
        Ok(ControlFlow::Continue(()))
    }

    //TODO: need to move it, refactor and it will still probably will not work for catch
    fn map_rust_error_to_java_exception(
        thread_id: ThreadId,
        exception: &JavaExceptionFromJvm,
        vm: &mut VirtualMachine,
    ) -> Result<HeapRef, JvmError> {
        let exception_ref = exception.as_reference();
        let class_id = vm
            .method_area
            .get_class_id_or_load(vm.interner().get_or_intern(exception_ref.class))?;
        let (method_id, instance_size) = {
            let class = vm.method_area.get_instance_class(&class_id)?;
            (
                class.get_special_method_id(
                    &MethodKey {
                        name: vm.interner().get_or_intern(exception_ref.name),
                        desc: vm.interner().get_or_intern(exception_ref.descriptor),
                    },
                )?,
                class.get_instance_size()?,
            )
        };
        let instance = vm.heap.alloc_instance(instance_size, class_id)?;
        let params = if let Some(msg) = exception.get_message() {
            vec![Value::Ref(instance), Value::Ref(vm.heap.alloc_string(msg)?)]
        } else {
            vec![Value::Ref(instance)]
        };
        Self::invoke_method_internal(thread_id, method_id, params, vm)?;
        Ok(instance)
    }

    pub(crate) fn prepare_method_args(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
    ) -> Result<Vec<Value>, JvmError> {
        let mut args_count = vm
            .method_area
            .get_method_descriptor_by_method_id(&method_id)
            .params
            .len();
        if !vm.method_area.get_method(&method_id).is_static() {
            args_count += 1;
        }
        let mut args = Vec::with_capacity(args_count);
        for _ in 0..args_count {
            args.push(vm.get_stack_mut(&thread_id)?.pop_operand()?);
        }
        args.reverse();
        Ok(args)
    }

    fn pc_in_range(pc: usize, entry: &ExceptionTableEntry) -> bool {
        pc >= entry.start_pc as usize && pc < entry.end_pc as usize
    }

    fn is_exception_caught(
        vm: &VirtualMachine,
        entry: &ExceptionTableEntry,
        method_id: &MethodId,
        java_exception: HeapRef,
    ) -> Result<bool, JvmError> {
        let catch_type = entry.catch_type;
        if catch_type == 0 {
            return Ok(true);
        }
        let exception_class_id = vm.heap.get_class_id(java_exception)?;
        let catch_type_sym = vm
            .method_area
            .get_cp_by_method_id(method_id)?
            .get_class_sym(&catch_type, vm.interner())?;
        Ok(vm.method_area.instance_of(exception_class_id, catch_type_sym))
    }

    fn find_exception_handler(
        vm: &mut VirtualMachine,
        method_id: &MethodId,
        java_exception: HeapRef,
        thread_id: &ThreadId,
    ) -> Result<bool, JvmError> {
        let pc = vm.get_stack_mut(thread_id)?.pc()?;
        let exception_table = vm.method_area.get_method(method_id).get_exception_table()?;
        for entry in exception_table.iter() {
            if !Self::pc_in_range(pc, entry) {
                continue;
            }
            if Self::is_exception_caught(vm, entry, method_id, java_exception)? {
                let handler_pc = entry.handler_pc as usize;
                let stack = vm.get_stack_mut(thread_id)?;
                stack.push_operand(Value::Ref(java_exception))?;
                *stack.pc_mut()? = handler_pc;
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn interpret_method(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let code_ptr = vm.method_area.get_method(&method_id).get_code()? as *const [u8];
        loop {
            let code = unsafe { &*code_ptr };
            let pc = vm.get_stack_mut(&thread_id)?.pc()?;
            let instruction = Instruction::new_at(code, pc)?;
            match Self::interpret_instruction(thread_id, instruction, vm) {
                Ok(flow) => {
                    if let ControlFlow::Break(res) = flow {
                        return Ok(res);
                    }
                }
                Err(e) => {
                    let java_exception = match e {
                        JvmError::JavaException(exception) => {
                            Self::map_rust_error_to_java_exception(thread_id, &exception, vm)
                        }
                        JvmError::JavaExceptionThrown(exception_ref) => Ok(exception_ref),
                        e => Err(e),
                    }?;
                    if !Self::find_exception_handler(vm, &method_id, java_exception, &thread_id)? {
                        vm.get_stack_mut(&thread_id)?.pop_java_frame()?;
                        return Err(JvmError::JavaExceptionThrown(java_exception));
                    }
                }
            }
        }
    }

    fn invoke_method_core(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        let method = vm.method_area.get_method(&method_id);
        if method.is_native() {
            let clone_desc = vm.br.clone_desc;
            let object_class_sym = vm.br.java_lang_object_sym;
            let mut method_key = vm
                .method_area
                .build_fully_qualified_native_method_key(&method_id);
            if !method.is_static()
                && vm.heap.is_array(args[0].as_obj_ref()?)?
                && method_key.name == vm.br.clone_sym
                && method_key.desc == clone_desc
                && method_key.class == Some(object_class_sym)
            {
                method_key.class = None;
            }
            let frame = NativeFrame::new(method_id);
            let native = *vm.native_registry.get(&method_key).ok_or(build_exception!(
                NoSuchMethodError,
                vm.pretty_method_not_found_message(&method_id)
            ))?;
            vm.get_stack_mut(&thread_id)?
                .push_frame(FrameType::NativeFrame(frame))?;
            let native_res = match native(vm, thread_id, args.as_slice()) {
                Ok(res) => res,
                Err(JvmError::JavaException(e)) => {
                    let exception_ref = Self::map_rust_error_to_java_exception(thread_id, &e, vm)?;
                    vm.get_stack_mut(&thread_id)?.pop_native_frame()?;
                    return Err(JvmError::JavaExceptionThrown(exception_ref));
                }
                Err(e) => {
                    vm.get_stack_mut(&thread_id)?.pop_native_frame()?;
                    return Err(e);
                }
            };
            vm.get_stack_mut(&thread_id)?.pop_native_frame()?;
            Ok(native_res)
        } else {
            let (max_stack, max_locals) = vm
                .method_area
                .get_method(&method_id)
                .get_frame_attributes()?;
            let frame = JavaFrame::new(method_id, max_stack, max_locals, args);
            vm.get_stack_mut(&thread_id)?
                .push_frame(FrameType::JavaFrame(frame))?;
            let method_ret = Self::interpret_method(thread_id, method_id, vm)?;
            vm.get_stack_mut(&thread_id)?.pop_java_frame()?;
            Ok(method_ret)
        }
    }

    pub(crate) fn invoke_method_internal(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let method_ret = Self::invoke_method_core(thread_id, method_id, args, vm)?;
        if let Some(ret) = method_ret {
            vm.get_stack_mut(&thread_id)?.push_operand(ret)?;
        }
        Ok(())
    }

    pub fn run_method(
        thread_id: ThreadId,
        method_id: MethodId,
        args: Vec<Value>,
        vm: &mut VirtualMachine,
    ) -> Result<Option<Value>, JvmError> {
        Self::invoke_method_core(thread_id, method_id, args, vm)
    }

    fn interface_needs_initialization(
        interface_id: ClassId,
        vm: &VirtualMachine,
    ) -> Result<bool, JvmError> {
        let interface = vm.method_area.get_interface_class(&interface_id)?;
        Ok(interface.has_clinit())
    }

    fn run_clinit_if_exists(
        thread_id: ThreadId,
        class_id: ClassId,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        if let Some(&clinit_method_id) = vm
            .method_area
            .get_class_like(&class_id)?
            .get_clinit_method_id()
        {
            Self::invoke_method_internal(thread_id, clinit_method_id, vec![], vm)?;
        }
        Ok(())
    }

    fn run_init_phase1(
        thread_id: ThreadId,
        class_id: ClassId,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let init_phase1_method_id = vm
            .method_area
            .get_instance_class(&class_id)?
            .get_special_method_id(&vm.br().system_init_phase1_mk)?;
        Self::invoke_method_internal(thread_id, init_phase1_method_id, vec![], vm)?;
        Ok(())
    }

    pub(crate) fn ensure_initialized(
        thread_id: ThreadId,
        class_id: Option<ClassId>,
        vm: &mut VirtualMachine,
    ) -> Result<(), JvmError> {
        let Some(class_id) = class_id else {
            return Ok(());
        };

        {
            let class = vm.method_area.get_class_like(&class_id)?;
            if class.is_initialized_or_initializing() {
                return Ok(());
            }
            class.set_initializing();
        }

        match vm.method_area.get_class(&class_id) {
            JvmClass::Instance(inst) => {
                if let Some(super_id) = inst.get_super() {
                    Self::ensure_initialized(thread_id, Some(super_id), vm)?;
                }
                for interface_id in vm
                    .method_area
                    .get_instance_class(&class_id)?
                    .get_interfaces()?
                    .clone()
                {
                    if Self::interface_needs_initialization(interface_id, vm)? {
                        Self::ensure_initialized(thread_id, Some(interface_id), vm)?;
                    }
                }

                Self::run_clinit_if_exists(thread_id, class_id, vm)?;

                let cur_class_name = vm.method_area.get_instance_class(&class_id)?.name();

                if cur_class_name == vm.br().java_lang_system_sym {
                    Self::run_init_phase1(thread_id, class_id, vm)?;
                }
                if vm.interner().resolve(&cur_class_name) == "jdk/internal/access/SharedSecrets" {
                    warn!(
                        "TODO: Stub: Setting jdk/internal/access/SharedSecrets javaLangRefAccess to non-null value, to avoid NPEs"
                    );
                    let ref_access_fk = FieldKey {
                        name: vm.interner().get_or_intern("javaLangRefAccess"),
                        desc: vm
                            .interner()
                            .get_or_intern("Ljdk/internal/access/JavaLangRefAccess;"),
                    };
                    vm.method_area
                        .get_instance_class(&class_id)?
                        .set_static_field_value(&ref_access_fk, Value::Ref(0))?;
                }
            }
            JvmClass::Interface(interface) => {
                for super_interface_id in interface.get_interfaces()?.clone() {
                    if Self::interface_needs_initialization(super_interface_id, vm)? {
                        Self::ensure_initialized(thread_id, Some(super_interface_id), vm)?;
                    }
                }
                Self::run_clinit_if_exists(thread_id, class_id, vm)?;
            }
            _ => {}
        }

        vm.method_area.get_class_like(&class_id)?.set_initialized();
        Ok(())
    }

    pub fn invoke_static_method(
        thread_id: ThreadId,
        method_id: MethodId,
        vm: &mut VirtualMachine,
        args: Vec<Value>,
    ) -> Result<(), JvmError> {
        let class_id = vm.method_area.get_method(&method_id).class_id();
        Self::ensure_initialized(thread_id, Some(class_id), vm)?;
        Self::invoke_method_internal(thread_id, method_id, args, vm)?;
        Ok(())
    }
}
