use pinocchio::error::ProgramError;
use pinocchio::{
    account::AccountView, no_allocator, nostd_panic_handler, program_entrypoint, Address,
    ProgramResult,
};
//use pinocchio_log::log;
use pinocchio_system::instructions::CreateAccount;

// Declares the entrypoint of the program.
program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

/// Instruction processor
pub fn process_instruction(
    program_id: &Address,
    accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    // (1) run_accounts
    //core::hint::black_box(Ok(()))
    // end of 1

    // (2) run_cpi
    let [from, to, _system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    CreateAccount {
        from,
        to,
        lamports: 1_000_000_000,
        space: 10,
        owner: program_id,
    }
    .invoke()

    // end of 2

    // (3) run_log
    /*
    let [account] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    log!("lamports={}", &[account.lamports(), account.lamports()]);

    Ok(())
    */
    // end of 3
}
