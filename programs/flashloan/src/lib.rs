pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FsLos6rnXoPrhjTrW4qfiypcy47WrJRibZXBJGMDy8iD");

#[program]
pub mod flashloan {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn flash_loan(ctx: Context<FlashLoan>, amount: u64) -> Result<()> {
        instructions::flash_loan::handler(ctx, amount)
    }

    pub fn flash_repay(ctx: Context<FlashRepay>) -> Result<()> {
        instructions::flash_repay::handler(ctx)
    }
}