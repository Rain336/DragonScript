pub mod error;
pub mod operators;
pub mod registers;
mod writer;

pub use writer::*;

use operators::{BinaryOperator, UnaryOperator};
use registers::Register;

pub type InstructionPointer = usize;

pub type FunctionId = usize;

pub type StructureId = usize;

pub type StringId = ();

pub type Local = u64;

pub enum Instruction {
    Nop,
    LoadThis {
        target: Register,
    },
    LoadNull {
        target: Register,
    },
    LoadBoolean {
        target: Register,
        value: bool,
    },
    LoadNumber {
        target: Register,
        value: f64,
    },
    LoadString {
        target: Register,
        value: StringId,
    },
    LoadLocal {
        target: Register,
        local: Local,
    },
    StoreLocal {
        target: Register,
        local: Local,
        index: Option<Register>,
    },
}
