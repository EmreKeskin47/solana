use solana_program::{program_error, program_error::ProgramError};
use std::convert::{try_into, TryInto};

#[derive(Debug)]
pub enum HelloInstruction {
    Increment,
    Decrement,
}

impl HelloInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag,rest) = input.split_first().ok_or(program_error::InvalidInsturctionData)?;

        match tag {
            0 => return Ok(HelloInstruction::Increment),
            1 => return Ok(HelloInstruction::Decrement),
            _ => return Err(ProgramError::InvalidInstructionData)
        }
        

    }
}