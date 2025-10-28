use crate::VmConfig;
use crate::rt::constant_pool::rt_cp_deprecated::RuntimeConstantPoolDeprecated;
use crate::rt::method_deprecated::MethodDeprecated;
use common::error::{JavaExceptionFromJvm, JvmError};
use common::jtype::{HeapAddr, Value};
use log::debug;
use std::sync::Arc;

pub struct FrameStack {
    max_size: usize,
    max_operand_stack_size: usize,
    frames: Vec<FrameType>,
}

impl FrameStack {
    pub fn new(vm_config: &VmConfig) -> Self {
        let max_size = vm_config.frame_stack_size;
        Self {
            max_size,
            max_operand_stack_size: vm_config.operand_stack_size,
            frames: Vec::with_capacity(max_size),
        }
    }

    pub fn frames(&self) -> &Vec<FrameType> {
        &self.frames
    }

    fn debug_fn_parameters(frame: &JavaFrame) -> String {
        frame
            .locals
            .iter()
            .flatten()
            .map(|v| format!("{:?}", v))
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub fn push_frame(&mut self, frame: FrameType) -> Result<(), JvmError> {
        match &frame {
            FrameType::JavaFrame(f) => debug!(
                "🚀 Executing {}.{}({})",
                f.method.class()?.name(),
                f.method.name(),
                Self::debug_fn_parameters(&f)
            ),
            FrameType::NativeFrame(f) => {
                debug!(
                    "🚀 Executing native {}.{}",
                    f.method.class()?.name(),
                    f.method.name()
                )
            }
        }
        if self.frames.len() >= self.max_size {
            return Err(JvmError::StackOverflow);
        }
        self.frames.push(frame);
        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<FrameType, JvmError> {
        let res = self.frames.pop().ok_or(JvmError::FrameStackIsEmpty)?;
        match &res {
            FrameType::JavaFrame(f) => debug!(
                "🏁 Execution finished of {}.{}",
                f.method.class()?.name(),
                f.method.name()
            ),
            FrameType::NativeFrame(f) => debug!(
                "🏁 Execution finished of native {}.{}",
                f.method.class()?.name(),
                f.method.name()
            ),
        }
        if let Some(frame) = self.frames.last() {
            match frame {
                FrameType::JavaFrame(f) => debug!(
                    "🔄 Resuming execution of {}.{}",
                    f.method.class()?.name(),
                    f.method.name()
                ),
                FrameType::NativeFrame(f) => debug!(
                    "🔄 Resuming execution of native {}.{}",
                    f.method.class()?.name(),
                    f.method.name()
                ),
            }
        }
        Ok(res)
    }

    pub fn pop_java_frame(&mut self) -> Result<JavaFrame, JvmError> {
        match self.pop_frame()? {
            FrameType::JavaFrame(f) => Ok(f),
            FrameType::NativeFrame(_) => Err(JvmError::UnexpectedType(
                "Expected Java frame on top of the stack".to_string(),
            )),
        }
    }

    pub fn pop_native_frame(&mut self) -> Result<NativeFrame, JvmError> {
        match self.pop_frame()? {
            FrameType::NativeFrame(f) => Ok(f),
            FrameType::JavaFrame(_) => Err(JvmError::UnexpectedType(
                "Expected Native frame on top of the stack".to_string(),
            )),
        }
    }

    fn cur_java_frame_mut(&mut self) -> Result<&mut JavaFrame, JvmError> {
        match self.frames.last_mut().ok_or(JvmError::FrameStackIsEmpty)? {
            FrameType::JavaFrame(f) => Ok(f),
            FrameType::NativeFrame(_) => Err(JvmError::UnexpectedType(
                "Expected Java frame on top of the stack".to_string(),
            )),
        }
    }

    fn cur_java_frame(&self) -> Result<&JavaFrame, JvmError> {
        match self.frames.last().ok_or(JvmError::FrameStackIsEmpty)? {
            FrameType::JavaFrame(f) => Ok(f),
            FrameType::NativeFrame(_) => Err(JvmError::UnexpectedType(
                "Expected Java frame on top of the stack".to_string(),
            )),
        }
    }

    pub fn pc(&self) -> Result<&usize, JvmError> {
        self.cur_java_frame().map(|v| &v.pc)
    }

    pub fn pc_mut(&mut self) -> Result<&mut usize, JvmError> {
        self.cur_java_frame_mut().map(|v| &mut v.pc)
    }

    fn get_local(&self, index: u8) -> Result<&Value, JvmError> {
        self.cur_java_frame()?.get_local(index)
    }

    pub fn get_local_long(&self, index: u8) -> Result<&Value, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Long(_) => Ok(local),
            _ => Err(JvmError::UnexpectedType(
                "Expected Long in local variable".to_string(),
            )),
        }
    }

    pub fn get_local_int(&self, index: u8) -> Result<&Value, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Integer(_) => Ok(local),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer in local variable".to_string(),
            )),
        }
    }

    pub fn get_local_int_val(&self, index: u8) -> Result<i32, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Integer(v) => Ok(*v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer in local variable".to_string(),
            )),
        }
    }

    pub fn get_local_float(&self, index: u8) -> Result<&Value, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Float(_) => Ok(local),
            _ => Err(JvmError::UnexpectedType(
                "Expected Float in local variable".to_string(),
            )),
        }
    }

    pub fn get_local_ref(&self, index: u8) -> Result<&Value, JvmError> {
        let local = self.get_local(index)?;
        match local {
            Value::Ref(_) | Value::Null => Ok(local),
            _ => Err(JvmError::UnexpectedType(
                "Expected Object or Array in local variable".to_string(),
            )),
        }
    }

    // TODO: check index bounds
    pub fn set_local(&mut self, idx: usize, value: Value) -> Result<(), JvmError> {
        self.cur_java_frame_mut()?.locals[idx] = Some(value);
        Ok(())
    }

    pub fn push_operand(&mut self, value: Value) -> Result<(), JvmError> {
        if self.cur_java_frame()?.operands.len() >= self.max_operand_stack_size {
            return Err(JvmError::StackOverflow);
        }
        self.cur_java_frame_mut()?.operands.push(value);
        Ok(())
    }

    pub fn pop_int(&mut self) -> Result<Value, JvmError> {
        match self.pop_operand()? {
            Value::Integer(v) => Ok(Value::Integer(v)),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_int_val(&mut self) -> Result<i32, JvmError> {
        match self.pop_operand()? {
            Value::Integer(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Integer on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_long(&mut self) -> Result<Value, JvmError> {
        match self.pop_operand()? {
            Value::Long(v) => Ok(Value::Long(v)),
            _ => Err(JvmError::UnexpectedType(
                "Expected Long on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_long_val(&mut self) -> Result<i64, JvmError> {
        match self.pop_operand()? {
            Value::Long(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Long on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_double(&mut self) -> Result<Value, JvmError> {
        match self.pop_operand()? {
            Value::Double(v) => Ok(Value::Double(v)),
            _ => Err(JvmError::UnexpectedType(
                "Expected Double on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_double_val(&mut self) -> Result<f64, JvmError> {
        match self.pop_operand()? {
            Value::Double(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Double on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_float(&mut self) -> Result<Value, JvmError> {
        match self.pop_operand()? {
            Value::Float(v) => Ok(Value::Float(v)),
            _ => Err(JvmError::UnexpectedType(
                "Expected Float on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_float_val(&mut self) -> Result<f32, JvmError> {
        match self.pop_operand()? {
            Value::Float(v) => Ok(v),
            _ => Err(JvmError::UnexpectedType(
                "Expected Float on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_nullable_ref(&mut self) -> Result<Value, JvmError> {
        let value = self.pop_operand()?;
        match &value {
            Value::Ref(_) | Value::Null => Ok(value),
            _ => Err(JvmError::UnexpectedType(
                "Expected Object on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_nullable_ref_val(&mut self) -> Result<Option<HeapAddr>, JvmError> {
        match self.pop_operand()? {
            Value::Ref(v) => Ok(Some(v)),
            Value::Null => Ok(None),
            _ => Err(JvmError::UnexpectedType(
                "Expected Object on operand stack".to_string(),
            )),
        }
    }

    pub fn pop_obj_val(&mut self) -> Result<HeapAddr, JvmError> {
        self.pop_nullable_ref_val()?.ok_or(JvmError::JavaException(
            JavaExceptionFromJvm::NullPointerException(None),
        ))
    }

    pub fn pop_operand(&mut self) -> Result<Value, JvmError> {
        self.cur_java_frame_mut()?.pop_operand()
    }

    pub fn cp(&self) -> Result<&RuntimeConstantPoolDeprecated, JvmError> {
        self.cur_java_frame().map(|v| v.cp.as_ref())
    }

    pub fn peek(&self) -> Result<&Value, JvmError> {
        self.cur_java_frame()?.peek()
    }
}

pub enum FrameType {
    JavaFrame(JavaFrame),
    NativeFrame(NativeFrame),
}

impl FrameType {
    pub fn method(&self) -> &Arc<MethodDeprecated> {
        match self {
            FrameType::JavaFrame(f) => &f.method,
            FrameType::NativeFrame(f) => &f.method,
        }
    }
}

#[derive(Clone)]
pub struct NativeFrame {
    method: Arc<MethodDeprecated>,
}

impl NativeFrame {
    pub fn new(method: Arc<MethodDeprecated>) -> Self {
        Self { method }
    }

    pub fn method(&self) -> &Arc<MethodDeprecated> {
        &self.method
    }
}

/// https://docs.oracle.com/javase/specs/jvms/se24/html/jvms-2.html#jvms-2.6
#[derive(Clone)]
pub struct JavaFrame {
    locals: Vec<Option<Value>>,
    operands: Vec<Value>,
    cp: Arc<RuntimeConstantPoolDeprecated>,
    pc: usize,
    method: Arc<MethodDeprecated>,
}

impl JavaFrame {
    pub fn new(
        cp: Arc<RuntimeConstantPoolDeprecated>,
        method: Arc<MethodDeprecated>,
        locals: Vec<Option<Value>>,
    ) -> Result<Self, JvmError> {
        let max_stack = method.max_stack()?;
        Ok(Self {
            locals,
            operands: Vec::with_capacity(max_stack),
            cp,
            pc: 0,
            method,
        })
    }

    pub fn method(&self) -> &Arc<MethodDeprecated> {
        &self.method
    }

    pub fn get_local(&self, index: u8) -> Result<&Value, JvmError> {
        self.locals
            .get(index as usize)
            .and_then(|v| v.as_ref())
            .ok_or(JvmError::LocalVariableNotInitialized(index))
    }

    pub fn peek(&self) -> Result<&Value, JvmError> {
        self.operands.last().ok_or(JvmError::OperandStackIsEmpty)
    }

    pub fn pop_operand(&mut self) -> Result<Value, JvmError> {
        self.operands.pop().ok_or(JvmError::OperandStackIsEmpty)
    }

    pub fn pc(&self) -> usize {
        self.pc
    }
}
