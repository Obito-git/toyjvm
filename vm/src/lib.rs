use thiserror::Error;
use class_file::ClassFileErr;
use common::CursorError;

pub mod execution_engine;
pub mod runtime;

#[derive(Debug, Error)]
pub enum JvmError {
    #[error(transparent)]
    ClassFile(#[from] ClassFileErr),
    #[error(transparent)]
    Cursor(#[from] CursorError),
    #[error("")]
    MissingAttributeInConstantPoll,
    #[error("")]
    ConstantNotFoundInRuntimePool,
    #[error("")]
    TrailingBytes,
    #[error("")]
    TypeError
}
pub fn run(_main: Vec<u8>) {
}
