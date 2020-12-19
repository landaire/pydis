use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("unknown opcode: 0x{0:X} ({0})")]
    UnknownOpcode(u8),
    #[error("an IO error occurred while reading data: {0}")]
    IoError(#[from] std::io::Error),
    #[error("could not read required number of bytes")]
    InvalidBytesRead,
}
