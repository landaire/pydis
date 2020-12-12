pub mod error;
pub mod opcode;

use num_traits::FromPrimitive;
use std::io::Read;

use crate::error::DecodeError;
use crate::opcode::{Instruction, Opcode};

/// Decodes a single instruction from a source and returns its result or an error
pub fn decode<R: Read>(source: &mut R) -> Result<Instruction, DecodeError> {
    let mut opcode_buffer = [0u8];
    let bytes_read = source.read(&mut opcode_buffer)?;
    if bytes_read != opcode_buffer.len() {
        return Err(error::DecodeError::InvalidBytesRead);
    }

    let opcode = Opcode::from_u8(opcode_buffer[0])
        .map_or(Err(DecodeError::UnknownOpcode(opcode_buffer[0])), Ok)?;

    let arg = if opcode.has_arg() {
        let mut argument_buffer = [0u8, 0u8];
        let bytes_read = source.read(&mut argument_buffer)?;
        if bytes_read != argument_buffer.len() {
            return Err(error::DecodeError::InvalidBytesRead);
        }

        Some(u16::from_le_bytes(argument_buffer))
    } else {
        None
    };

    Ok(Instruction { opcode, arg })
}
