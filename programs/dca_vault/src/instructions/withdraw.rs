use crate::{error::VaultError, state::VaultState};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

const EARLY_EXIT_FEE_BPS: u64 = 50;
#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_usdc: InterfaceAccount<'info, Mint>,
    #[account(
       mut,
        associated_token::mint=mint_usdc,
        associated_token::authority=user,
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        close=user,
        seeds=[b"dcavault",user.key().as_ref()],
        bump,
    )]
    pub vault: Account<'info, VaultState>,
    #[account(
        mut,
        associated_token::mint=mint_usdc,
        associated_token::authority=vault,
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
#[event]
pub struct WithdrawEvent {
    pub owner: Pubkey,
    pub amount_withdrawn: u64,
    pub fee_charged: u64,
    pub is_early_exit: bool,
    pub timestamp: i64,
}

//withdraw simply means taking all the remaining usdc out from the vault and closing the vault account
impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        // Step 1: Authorization check
        require!(
            self.vault.owner == self.user.key(),
            VaultError::Unauthorized
        );

        // Step 2: Get current USDC balance in vault
        let remaining_usdc = self.vault_ata.amount;
        require!(remaining_usdc > 0, VaultError::InsufficientBalance);

        // Step 3: Determine if this is an early exit
        let is_early_exit = self.vault.periods_completed < self.vault.periods;

        // Step 4: Calculate fee if early exit
        let fee_amount = if is_early_exit {
            (remaining_usdc * EARLY_EXIT_FEE_BPS) / 10000
        } else {
            0
        };

        // Step 5: Calculate amount to transfer to user
        let transfer_amount = remaining_usdc
            .checked_sub(fee_amount)
            .ok_or(VaultError::ArithmeticOverflow)?;

        // Step 6: Transfer USDC from vault to user
        let seeds = &[
            b"dcavault",
            self.user.to_account_info().key.as_ref(),
            &[self.vault.bump],
        ];
        let signers_seeds = &[&seeds[..]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                TransferChecked {
                    from: self.vault_ata.to_account_info(),
                    mint: self.mint_usdc.to_account_info(),
                    to: self.user_ata.to_account_info(),
                    authority: self.vault.to_account_info(),
                },
                signers_seeds,
            ),
            transfer_amount,
            self.mint_usdc.decimals,
        )?;

        // Step 7: Emit withdraw event
        let curr_time = Clock::get()?.unix_timestamp;
        emit!(WithdrawEvent {
            owner: self.user.key(),
            amount_withdrawn: transfer_amount,
            fee_charged: fee_amount,
            is_early_exit,
            timestamp: curr_time,
        });
        Ok(())
    }
}
