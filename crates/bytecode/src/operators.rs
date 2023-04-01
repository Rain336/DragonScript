#[repr(u8)]
#[derive(Clone, Copy)]
pub enum UnaryOperator {
    Minus,
    Plus,
    Bang,
    Tilde,
    TypeOf,
    Void,
    Delete,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum BinaryOperator {
    LooseEquals,
    LooseNotEquals,
    StrictEquals,
    StrictNotEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    LeftShift,
    RightShift,
    ZeroFillRightShift,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    LogicalOr,
    LogicalAnd,
    In,
    InstanceOf,
    Exponent,
    NullishCoalescing,
}