use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

// PAYLOADS

#[derive(BorshDeserialize)]
struct SwapPayload {
    usdc_amount: u64,
    bono_amount_threshold: u64,
}

#[derive(BorshDeserialize)]
struct ReadBonoPricePayload {
    bono_amount: u64,
}

// PROGRAM INSTRUCTIONS

pub enum CreditInstruction {
    Swap { usdc_amount: u64, bono_amount_threshold: u64 },
    ReadBonoPrice { bono_amount: u64 },
}

impl CreditInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match variant {
            0 => {
                let payload =
                    SwapPayload::try_from_slice(rest).expect("Invalid data payload for variant 0");
                Self::Swap {
                    usdc_amount: payload.usdc_amount,
                    bono_amount_threshold: payload.bono_amount_threshold,
                }
            }
            1 => {
                let payload = ReadBonoPricePayload::try_from_slice(rest)
                    .expect("Invalid data payload for variant 1");
                Self::ReadBonoPrice {
                    bono_amount: payload.bono_amount,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
