use std::os::unix::process;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, address_lookup_table::instruction,  entrypoint,
    entrypoint::ProgramResult, instruction::Instruction, msg, program::invoke, pubkey::Pubkey, rent::Rent, system_instruction::create_account, sysvar::Sysvar,
};

//Tutorial from 
//https://solana.com/developers/guides/getstarted/intro-to-native-rust 

// https://github.com/near/borsh-rs/blob/master/CHANGELOG.md 1.0.0-alpha.6 - 2023-10-02 

entrypoint!(process_instruction);


pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8],
) -> ProgramResult {
    let instruction = Instructions::try_from_slice(instruction_data)?;
    match instruction {
        Instructions::Initialize { data } => process_initialize(program_id, accounts, data)
    }
}

pub fn process_initialize(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    data:u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let new_account = next_account_info(accounts_iter)?;
    let signer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let account_data = NewAccount {data};
    let size = account_data;
    let lamports = (Rent::get()?).minimum_balance(size);

    invoke(
        &create_account(
            signer.key, 
            new_account.key, 
            lamports,
            size as u64, 
            program_id
        ), 
        &[signer.clone(), new_account.clone(), system_program.clone()])?;

        account_data.serialize(&mut *new_account.data.borrow_mut())?;
        msg!("Changed data to {:?}",data);
        Ok(())
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Instructions {
    Initialize { data : u64}
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NewAccount {
    pub data: u64
}