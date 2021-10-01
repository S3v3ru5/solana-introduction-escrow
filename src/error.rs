use thiserror:Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Expected Amount Mismatch")]
    ExpectedAmountMismatch,

    #[error("Amount Overflowed")]
    AmountOverflow
}

impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}