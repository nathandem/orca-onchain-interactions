use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
};
use spl_associated_token_account;
use spl_token::{instruction::approve as token_approve, state::Account as TokenAccount};

use crate::instruction::CreditInstruction;

use crate::whirlpool::Whirlpool;

// Unique PDAs, created once and used as support to the program
pub const CREDIT_SIGNING_PDA_SEED: &[u8] = b"CREDIT_SIGNING_PDA";

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = CreditInstruction::unpack(instruction_data)?;
    match instruction {
        CreditInstruction::Swap { usdc_amount } => {
            swap_usdc_for_bono(program_id, accounts, usdc_amount)
        }
        CreditInstruction::ReadBonoPrice { bono_amount } => {
            read_bono_price(program_id, accounts, bono_amount)
        }
    }
}

/*
 * "Swap" USDCs for BONOs on an orca pool.
 *
 * Steps:
 * 1. check args, including enough USDC owned by user
 * 2. delegate USDC amount to be exchanged to smart-contract's signing PDA
 * 3. swap USDC for BONO
 *
 * Questions:
 * - how to get the 3 tick arrays?
 *   If it's with the utils, then
 *   how to know the value of `startTick` when using `getTickArray` (sdk/src/utils/public/pda-utils.ts#L85)?
 *   how to know the current `tickCurrentIndex` when using `getTickArrayPublicKeys` (sdk/src/utils/public/swap-utils.ts#L93)?
 *
 * Note: in reality, there are other steps after smart-contract receives the BONOs but for the purpose of
 * this example/sample, I just concentrate on the interaction with orca.
 */
pub fn swap_usdc_for_bono(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    usdc_amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // 1. Check accounts, including enough USDC owner by user
    let signer_info = next_account_info(account_info_iter)?;
    let system_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?; // for transfers
    let associated_token_account_program_info = next_account_info(account_info_iter)?;
    let credit_signing_pda_info = next_account_info(account_info_iter)?;
    let signer_usdc_ata_info = next_account_info(account_info_iter)?;
    let credit_signing_pda_bono_ata_info = next_account_info(account_info_iter)?;

    // note: during the first call, `credit_signing_pda_info` and `credit_signing_pda_bono_ata_info`
    // would have to be created, I'm just not doing it here to save space.

    // TODO: add the accounts needed to perform the swap
    // note: the whirlpool to use is https://v1.orca.so/liquidity/browse?tokenMint=CzYSquESBM4qVQiFas6pSMgeFRG4JLiYyNYHQUcNxudc

    // Check signer USDCs balance and delegated amount to program's signing PDA
    let signer_usdc_ata =
        TokenAccount::unpack_from_slice(&signer_usdc_ata_info.data.borrow()).unwrap();
    if signer_usdc_ata.amount < usdc_amount {
        msg!("Not enough USDCs owned by signer to perform the swap asked");
        return Err(ProgramError::InsufficientFunds);
    }

    // 2. Delegate USDC amount to program's signing PDA
    invoke(
        &token_approve(
            token_program_info.key,
            signer_usdc_ata_info.key,
            credit_signing_pda_info.key,
            signer_info.key,
            &[],
            usdc_amount,
        )?,
        &[
            signer_usdc_ata_info.clone(),
            credit_signing_pda_info.clone(),
            signer_info.clone(),
        ],
    )?;

    // 3. Swap USDC for BONO
    // note: all the USDC passed should be used, market price is okay for the swap (no limit)
    // the recipient of the BONO should be `credit_signing_pda_bono_ata_info``

    // TODO: code to perform the swap on the orca pool

    Ok(())
}

/*
 * Read the price of a BONO amount in USDC.
 *
 * Note: in reality, there are other steps after smart-contract receives the BONOs but for the purpose of
 * this example/sample, I just concentrate on the interaction with orca.
 */
pub fn read_bono_price(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    bono_amount: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let whirlpool_account_info = next_account_info(account_info_iter)?;
    let whirlpool = Whirlpool::try_from_slice(whirlpool_account_info.data.borrow().as_ref())?;

    msg!("Whirlpool account: {:?}", whirlpool);

    Ok(())
}
