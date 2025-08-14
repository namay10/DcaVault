use crate::{error::VaultError, state::VaultState};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};
#[derive(Accounts)]
pub struct VaultInit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_usdc: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=mint_usdc,
        associated_token::authority=user,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer=user,
        space=8+VaultState::INIT_SPACE,
        seeds=[b"dcavault",user.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, VaultState>,
    #[account(
        init,
        payer=user,
        associated_token::mint=mint_usdc,
        associated_token::authority=vault,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
#[event]
pub struct VaultCreated {
    pub owner: Pubkey,
    pub total_amount: u64,
    pub periods: u16,
    pub interval_seconds: u64,
    pub staked_at: i64,
}
impl<'info> VaultInit<'info> {
    pub fn deposit_and_init_dca_vault(
        &mut self,
        amount: u64,
        periods: u16,
        interval_seconds: u64,
        bumps: &VaultInitBumps,
    ) -> Result<()> {
        require!(amount > 0, VaultError::InvalidAmount);
        require!(periods > 0, VaultError::InvalidPeriods);
        require!(interval_seconds > 0, VaultError::InvalidInterval);
        transfer_checked(
            CpiContext::new(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.user_ata.to_account_info(),
                    mint: self.mint_usdc.to_account_info(),
                    to: self.vault_ata.to_account_info(),
                    authority: self.user.to_account_info(),
                },
            ),
            amount,
            self.mint_usdc.decimals,
        )?;
        let curr_time = Clock::get()?.unix_timestamp;
        self.vault.set_inner(VaultState {
            owner: self.user.key(),
            total_amount: amount,
            periods,
            interval_seconds,
            staked_at: curr_time,
            curr_balance: amount,
            periods_completed: 0,
            next_swap_time: curr_time + interval_seconds as i64,
            total_sol_received: 0,
            bump: bumps.vault,
        });
        emit!(VaultCreated {
            owner: self.user.key(),
            total_amount: amount,
            periods,
            interval_seconds,
            staked_at: curr_time,
        });
        Ok(())
    }
}
