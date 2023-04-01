use crate::error::ByteCodeError;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
}

impl TryFrom<u8> for Register {
    type Error = ByteCodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register::X0),
            1 => Ok(Register::X1),
            2 => Ok(Register::X2),
            3 => Ok(Register::X3),
            4 => Ok(Register::X4),
            5 => Ok(Register::X5),
            6 => Ok(Register::X6),
            7 => Ok(Register::X7),
            8 => Ok(Register::X8),
            9 => Ok(Register::X9),
            10 => Ok(Register::X10),
            11 => Ok(Register::X11),
            12 => Ok(Register::X12),
            13 => Ok(Register::X13),
            _ => Err(ByteCodeError::InvaildRegister(value)),
        }
    }
}
