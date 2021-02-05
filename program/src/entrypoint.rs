//! Program entrypoint

use crate::processor::Processor;

use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
    msg,
};

use hex;

entrypoint!(process_instruction);

// Program entrypoint's implementation
fn process_instruction<'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("instruction: {}", hex::encode(instruction_data));
    // if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
    //     // catch the error so we can print it
    //     // error.print::<Error>();
    //     // msg!("{:?}", error);
    //     return Err(error);
    // }
    Ok(())
}
