use solana_program::{
    account_info::AccountInfo, entrypoint_deprecated::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

solana_program::entrypoint_deprecated!(entry);
fn entry(_program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let account = accounts.get(0).ok_or(ProgramError::NotEnoughAccountKeys)?;
    if instruction_data.len() != 32 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let expected_owner = match Pubkey::try_from(instruction_data) {
        Ok(key) => key,
        Err(_) => return Err(ProgramError::InvalidInstructionData),
    };
    if expected_owner != *account.owner {
        solana_program::log::msg!("Account owner mismatch");
        return Err(ProgramError::Custom(0x100));
    }
    Ok(())
}