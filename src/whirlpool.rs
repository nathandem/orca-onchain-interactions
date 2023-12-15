use solana_program::pubkey::Pubkey;
use borsh::BorshDeserialize;

// copied from: https://github.com/orca-so/whirlpools/blob/main/programs/whirlpool/src/state/whirlpool.rs
// replace anchor macro by Borsh

// Number of rewards supported by Whirlpools
pub const NUM_REWARDS: usize = 3;

#[derive(Copy, Clone, BorshDeserialize, Default, Debug, PartialEq)]
pub struct Whirlpool {
  // ANCHOR ACCOUNT DISCRIMINATOR
  pub discriminator: [u8; 8], // 8

  pub whirlpools_config: Pubkey, // 32
  pub whirlpool_bump: [u8; 1],   // 1

  pub tick_spacing: u16,          // 2
  pub tick_spacing_seed: [u8; 2], // 2

  // Stored as hundredths of a basis point
  // u16::MAX corresponds to ~6.5%
  pub fee_rate: u16, // 2

  // Portion of fee rate taken stored as basis points
  pub protocol_fee_rate: u16, // 2

  // Maximum amount that can be held by Solana account
  pub liquidity: u128, // 16

  // MAX/MIN at Q32.64, but using Q64.64 for rounder bytes
  // Q64.64
  pub sqrt_price: u128,        // 16
  pub tick_current_index: i32, // 4

  pub protocol_fee_owed_a: u64, // 8
  pub protocol_fee_owed_b: u64, // 8

  pub token_mint_a: Pubkey,  // 32
  pub token_vault_a: Pubkey, // 32

  // Q64.64
  pub fee_growth_global_a: u128, // 16

  pub token_mint_b: Pubkey,  // 32
  pub token_vault_b: Pubkey, // 32

  // Q64.64
  pub fee_growth_global_b: u128, // 16

  pub reward_last_updated_timestamp: u64, // 8

  pub reward_infos: [WhirlpoolRewardInfo; NUM_REWARDS], // 384
}

#[derive(Copy, Clone, BorshDeserialize, Default, Debug, PartialEq)]
pub struct WhirlpoolRewardInfo {
    /// Reward token mint.
    pub mint: Pubkey,
    /// Reward vault token account.
    pub vault: Pubkey,
    /// Authority account that has permission to initialize the reward and set emissions.
    pub authority: Pubkey,
    /// Q64.64 number that indicates how many tokens per second are earned per unit of liquidity.
    pub emissions_per_second_x64: u128,
    /// Q64.64 number that tracks the total tokens earned per unit of liquidity since the reward
    /// emissions were turned on.
    pub growth_global_x64: u128,
}


use rust_decimal::prelude::*;
use rust_decimal::MathematicalOps;

// https://orca-so.github.io/whirlpools/classes/PriceMath.html#sqrtPriceX64ToPrice
// https://github.com/orca-so/whirlpools/blob/main/sdk/src/utils/public/price-math.ts#L22
pub fn pricemath_sqrt_price_x64_to_price(sqrt_price_x64: u128, decimals_a: i8, decimals_b: i8) -> Decimal {
  let sqrt_price_x64_decimal = Decimal::from_str(&sqrt_price_x64.to_string()).unwrap();

  let price = sqrt_price_x64_decimal
    .checked_div(Decimal::TWO.powu(64)).unwrap()
    .powu(2)
    .checked_mul(Decimal::TEN.powi((decimals_a - decimals_b) as i64)).unwrap();
  
  return price;
}

pub fn u64_to_decimal(amount: u64, decimals: i8) -> Decimal {
  let amount_as_decimal = Decimal::from_str(&amount.to_string()).unwrap();
  let decimal_adjusted_amount = amount_as_decimal.checked_div(Decimal::TEN.powi(decimals as i64)).unwrap();
  return decimal_adjusted_amount;
}
