use bytecode::registers::Register;
use swc_ecma_visit::swc_ecma_ast::{ThisExpr, ArrayLit};
use swc_ecma_visit::Visit;

pub struct RegisterTracker {
    
}

pub enum ConstantOrRegister {
    ConstantUndefined,
    ConstantNull,
    ConstantTrue,
    ConstantFalse,
    ConstantNumber(f64),
    Register(Register),
}

struct ExpressionWriter {
    result: ConstantOrRegister,
}

impl ExpressionWriter {
    pub fn new() -> Self {
        ExpressionWriter {
            result: ConstantOrRegister::ConstantUndefined,
        }
    }

    pub fn into_result(self) -> ConstantOrRegister {
        self.result
    }
}

impl Visit for ExpressionWriter {
    fn visit_this_expr(&mut self, _: &ThisExpr) {
        self.result = ConstantOrRegister::Register(Register::X0)
    }

    fn visit_array_lit(&mut self, array: &ArrayLit) {

    }
}
