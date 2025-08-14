pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AZDprYt6ksZxH1nUFQdSEp84GYu9WpvG68M4oNmscTFF");

#[program]
pub mod dca_vault {
    use super::*;
    pub fn vault_initialize(
        ctx: Context<VaultInit>,
        amount: u64,
        periods: u16,
        interval_seconds: u64,
    ) -> Result<()> {
        ctx.accounts
            .deposit_and_init_dca_vault(amount, periods, interval_seconds, &ctx.bumps)?;
        Ok(())
    }
    pub fn swap(ctx: Context<Swap>, usdc_amount: u64, jupiter_swap_ix: Vec<u8>) -> Result<()> {
        ctx.accounts
            .execute_swap(usdc_amount, jupiter_swap_ix, &ctx.remaining_accounts)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        ctx.accounts.withdraw()?;
        Ok(())
    }
}
