use thiserror::Error;

#[derive(Debug, Error)]
pub enum ByteCodeError {
    #[error("Invalid register number: {0}")]
    InvaildRegister(u8),
}

pub type Result<T> = std::result::Result<T, ByteCodeError>;
