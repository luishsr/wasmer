use crate::CodeOffset;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use wasmer_vm::TrapCode;

/// Information about trap.
#[derive(RkyvSerialize, RkyvDeserialize, Archive, Clone, Debug, PartialEq, Eq)]
pub struct TrapInformation {
    /// The offset of the trapping instruction in native code. It is relative to the beginning of the function.
    pub code_offset: CodeOffset,
    /// Code of the trap.
    pub trap_code: TrapCode,
}
