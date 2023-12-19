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
    instruction::{Instruction, AccountMeta},
};
use spl_associated_token_account::instruction::create_associated_token_account_idempotent;
use spl_token::{instruction::approve as token_approve, state::Account as TokenAccount};

use crate::{instruction::CreditInstruction, whirlpool::{pricemath_sqrt_price_x64_to_price, u64_to_decimal}};

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
        CreditInstruction::Swap { usdc_amount, bono_amount_threshold } => {
            swap_usdc_for_bono(program_id, accounts, usdc_amount, bono_amount_threshold)
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
    bono_amount_threshold: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // 1. Check accounts, including enough USDC owner by user
    let system_program_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?; // for transfers
    let associated_token_account_program_info = next_account_info(account_info_iter)?;
    let whirlpool_program_info = next_account_info(account_info_iter)?;

    let bono_mint_info = next_account_info(account_info_iter)?;

    let signer_info = next_account_info(account_info_iter)?;
    let signer_usdc_ata_info = next_account_info(account_info_iter)?;

    let credit_signing_pda_info = next_account_info(account_info_iter)?;
    let credit_signing_pda_bono_ata_info = next_account_info(account_info_iter)?;

    let whirlpool_info = next_account_info(account_info_iter)?;
    let vault_a_bono_info = next_account_info(account_info_iter)?;
    let vault_b_usdc_info = next_account_info(account_info_iter)?;
    let tick_array_0_info = next_account_info(account_info_iter)?;
    let tick_array_1_info = next_account_info(account_info_iter)?;
    let tick_array_2_info = next_account_info(account_info_iter)?;
    let oracle_info = next_account_info(account_info_iter)?;

    // derive bump
    let (credit_signing_pda_key, bump) = Pubkey::find_program_address(
        &[CREDIT_SIGNING_PDA_SEED],
        program_id,
    );
    assert!(credit_signing_pda_key == *credit_signing_pda_info.key);

    // note: during the first call, `credit_signing_pda_info` and `credit_signing_pda_bono_ata_info`
    // would have to be created, I'm just not doing it here to save space.
    invoke(
        &create_associated_token_account_idempotent(
            signer_info.key,
            credit_signing_pda_info.key,
            bono_mint_info.key,
            token_program_info.key,
        ),
        &[
            associated_token_account_program_info.clone(),
            signer_info.clone(), // funder
            credit_signing_pda_bono_ata_info.clone(), // ata
            credit_signing_pda_info.clone(), // owner
            bono_mint_info.clone(), // mint
            system_program_info.clone(),
            token_program_info.clone(),
        ],
    )?;

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
            token_program_info.clone(),
            signer_usdc_ata_info.clone(),
            credit_signing_pda_info.clone(),
            signer_info.clone(),
        ],
    )?;

    // 3. Swap USDC for BONO
    // note: all the USDC passed should be used, market price is okay for the swap (no limit)
    // the recipient of the BONO should be `credit_signing_pda_bono_ata_info``
    let mut swap_data: Vec<u8> = vec![0xf8, 0xc6, 0x9e, 0x91, 0xe1, 0x75, 0x87, 0xc8]; // ix code
    swap_data.extend_from_slice(&usdc_amount.to_le_bytes()); // amount
    swap_data.extend_from_slice(&bono_amount_threshold.to_le_bytes()); // other_amount_threshold
    swap_data.extend_from_slice(&79226673515401279992447579055u128.to_le_bytes()); // sqrt_price_limit
    swap_data.extend_from_slice(&1u8.to_le_bytes()); // amount_specified_is_input (true, amount is USDC input)
    swap_data.extend_from_slice(&0u8.to_le_bytes()); // a_to_b (false, we want to swap B(USDC) for A(BONO))
    invoke_signed(
        &Instruction {
            program_id: whirlpool_program_info.key.clone(),
            accounts: vec![
                AccountMeta::new_readonly(token_program_info.key.clone(), false),
                AccountMeta::new_readonly(credit_signing_pda_info.key.clone(), true),
                AccountMeta::new(whirlpool_info.key.clone(), false),
                AccountMeta::new(credit_signing_pda_bono_ata_info.key.clone(), false),
                AccountMeta::new(vault_a_bono_info.key.clone(), false),
                AccountMeta::new(signer_usdc_ata_info.key.clone(), false),
                AccountMeta::new(vault_b_usdc_info.key.clone(), false),
                AccountMeta::new(tick_array_0_info.key.clone(), false),
                AccountMeta::new(tick_array_1_info.key.clone(), false),
                AccountMeta::new(tick_array_2_info.key.clone(), false),
                AccountMeta::new_readonly(oracle_info.key.clone(), false),
            ],
            data: swap_data,
        },
        &[
            whirlpool_program_info.clone(),
            token_program_info.clone(),
            credit_signing_pda_info.clone(),
            whirlpool_info.clone(),
            credit_signing_pda_bono_ata_info.clone(),
            vault_a_bono_info.clone(),
            signer_usdc_ata_info.clone(),
            vault_b_usdc_info.clone(),
            tick_array_0_info.clone(),
            tick_array_1_info.clone(),
            tick_array_2_info.clone(),
            oracle_info.clone(),
        ],
        &[&[CREDIT_SIGNING_PDA_SEED, &[bump]]],
    )?;

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

    let sqrt_price_x64 = whirlpool.sqrt_price;
    let decimals_a = 9; // BONO
    let decimals_b = 6; // USDC
    let ui_price = pricemath_sqrt_price_x64_to_price(sqrt_price_x64, decimals_a, decimals_b);

    msg!("Whirlpool account sqrt_price_x64: {:?}", sqrt_price_x64);
    msg!("Whirlpool account ui_price: {:?}", ui_price.trunc_with_scale(6));

    msg!("BONO amount in u64: {:?}", bono_amount);
    
    // to avoid precision loss, it is nice to use integer math only (using Decimal is just sample code to show what the data is)
    let usdc_value = ui_price * u64_to_decimal(bono_amount, decimals_a);
    msg!("BONO amount in USDC: {:?}", usdc_value.trunc_with_scale(6));

    Ok(())
}
