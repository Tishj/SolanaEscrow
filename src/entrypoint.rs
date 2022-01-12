use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    msg,
    pubkey::Pubkey,
	entrypoint
};

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
	Ok(())
}
