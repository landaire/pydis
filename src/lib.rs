pub mod error;
pub mod opcode;
pub mod prelude;

use num_traits::FromPrimitive;
use opcode::Python27;
use std::io::Read;

use crate::error::DecodeError;
use crate::opcode::{Instruction, Opcode};

/// Decodes a single instruction from a source and returns its result or an error
pub fn decode<O: Opcode + FromPrimitive, R: Read>(
    source: &mut R,
) -> Result<Instruction<O>, DecodeError> {
    let mut opcode_buffer = [0u8];
    let bytes_read = source.read(&mut opcode_buffer)?;
    if bytes_read != opcode_buffer.len() {
        return Err(error::DecodeError::InvalidBytesRead);
    }

    let opcode = O::from_u8(opcode_buffer[0])
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

/// Convenience wrapper around [`decode`] for decoding Python 2.7 instructions
pub fn decode_py27(source: &mut impl Read) -> Result<Instruction<Python27>, DecodeError> {
    decode::<Python27, _>(source)
}
