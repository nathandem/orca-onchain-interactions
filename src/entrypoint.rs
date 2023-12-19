use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey, declare_id,
};

use crate::processor;

declare_id!("82XBkYcPfaevmCNDJwV4EPcDrhWbvonN9iCUJaorfCRj");

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    processor::process_instruction(program_id, accounts, instruction_data)?;

    Ok(())
}
