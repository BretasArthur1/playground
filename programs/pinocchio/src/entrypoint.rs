// use pinocchio::error::ProgramError;
use pinocchio::{
    account::AccountView, no_allocator, nostd_panic_handler, program_entrypoint, Address,
    ProgramResult,
};
// use solana_program_log::log;
//use pinocchio_system::instructions::CreateAccount;

// Declares the entrypoint of the program.
program_entrypoint!(process_instruction);
nostd_panic_handler!();
no_allocator!();

fn read_u64_unaligned(data: &[u8]) -> u64 {
    if data.len() < 9 {
        return 1;
    }
    unsafe { core::ptr::read_unaligned(data.as_ptr().add(1) as *const u64) }
}

/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    // (1) run_accounts
    // core::hint::black_box(Ok(()))
    // end of 1

    // (2) run_cpi
    /*
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
    */
    // end of 2

    // (3) run_log
    // let [account] = accounts else {
    //     return Err(ProgramError::NotEnoughAccountKeys);
    // };

    // log!("lamports={}", account.lamports());

    // Ok(())
    // end of 3

    //(4) read unaligned
    let v = read_u64_unaligned(instruction_data);
    core::hint::black_box(v);
    core::hint::black_box(Ok(()))
}
