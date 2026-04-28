pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BMSDHjY74HZAknfMBvNF6fjVStM1pPe5hiVhQqDwSFfw");

#[program]
pub mod dummy {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn drain(ctx: Context<Drain>, amount: u64) -> Result<()> {
        instructions::drain::handler(ctx, amount)
    }
}
