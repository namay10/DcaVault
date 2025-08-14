use crate::{error::VaultError, state::VaultState};
use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use jupiter_aggregator::program::Jupiter;
use std::str::FromStr;
declare_program!(jupiter_aggregator);
// Jupiter Program ID (same for mainnet and devnet)
pub fn jupiter_program_id() -> Pubkey {
    Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap()
}

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"dcavault", user.key().as_ref()],
        bump = vault.bump,
        constraint = vault.owner == user.key() @ VaultError::Unauthorized,
    )]
    pub vault: Account<'info, VaultState>,

    // USDC accounts
    pub mint_usdc: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_usdc,
        associated_token::authority = vault,
    )]
    pub vault_usdc_ata: InterfaceAccount<'info, TokenAccount>,

    // SOL accounts (direct to user)
    pub mint_sol: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_sol,
        associated_token::authority = user,
    )]
    pub user_sol_ata: InterfaceAccount<'info, TokenAccount>,

    pub jupiter_program: Program<'info, Jupiter>,

    // Token program
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[event]
pub struct SwapExecuted {
    pub owner: Pubkey,
    pub usdc_amount: u64,
    pub sol_amount: u64,
    pub periods_completed: u16,
    pub next_swap_time: i64,
    pub timestamp: i64,
}

impl<'info> Swap<'info> {
    pub fn execute_swap(
        &mut self,
        usdc_amount: u64,
        jupiter_swap_ix: Vec<u8>,
        remaining_accounts: &[AccountInfo],
    ) -> Result<()> {
        // Step 1: Time validation

        let curr_time = Clock::get()?.unix_timestamp;
        require!(
            curr_time >= self.vault.next_swap_time,
            VaultError::SwapNotDue
        );

        // Step 2: DCA plan validation
        require!(
            self.vault.periods_completed < self.vault.periods,
            VaultError::DcaPlanComplete
        );

        // Step 3: Amount validation
        require!(usdc_amount > 0, VaultError::InvalidAmount);
        require!(
            self.vault_usdc_ata.amount >= usdc_amount,
            VaultError::InsufficientBalance
        );

        // Step 4: Slice amount validation
        let slice_amount = self.vault.total_amount / self.vault.periods as u64;
        require!(usdc_amount == slice_amount, VaultError::InvalidSliceAmount);

        // Step 5: Store initial SOL balance for calculation
        let initial_sol_balance = self.user_sol_ata.amount;

        // Step 6: Execute Jupiter swap via CPI
        // Validate Jupiter program ID
        require_keys_eq!(*self.jupiter_program.key, jupiter_program_id());

        // Create account metas from remaining accounts
        let accounts: Vec<AccountMeta> = remaining_accounts
            .iter()
            .map(|acc| {
                let is_signer = acc.key == &self.vault.key();
                AccountMeta {
                    pubkey: *acc.key,
                    is_signer,
                    is_writable: acc.is_writable,
                }
            })
            .collect();

        // Create account infos from remaining accounts
        let accounts_infos: Vec<AccountInfo> = remaining_accounts
            .iter()
            .map(|acc| AccountInfo { ..acc.clone() })
            .collect();

        // Signer seeds for vault PDA

        let seeds = &[
            b"dcavault",
            self.user.to_account_info().key.as_ref(),
            &[self.vault.bump],
        ];
        let signers_seeds = &[&seeds[..]];

        // Execute Jupiter CPI call using the exact pattern from your example
        invoke_signed(
            &Instruction {
                program_id: self.jupiter_program.key(),
                accounts,
                data: jupiter_swap_ix,
            },
            &accounts_infos,
            signers_seeds,
        )?;

        msg!(
            "Jupiter swap executed successfully for {} USDC",
            usdc_amount
        );

        // Step 7: Calculate SOL received
        let final_sol_balance = self.user_sol_ata.amount;
        let sol_received = final_sol_balance
            .checked_sub(initial_sol_balance)
            .unwrap_or(0);

        // Step 8: Update vault state
        self.vault.curr_balance = self
            .vault
            .curr_balance
            .checked_sub(usdc_amount)
            .ok_or(VaultError::ArithmeticOverflow)?;

        self.vault.periods_completed = self
            .vault
            .periods_completed
            .checked_add(1)
            .ok_or(VaultError::ArithmeticOverflow)?;

        self.vault.next_swap_time = self
            .vault
            .next_swap_time
            .checked_add(self.vault.interval_seconds as i64)
            .ok_or(VaultError::ArithmeticOverflow)?;

        // Update total SOL received
        self.vault.total_sol_received = self
            .vault
            .total_sol_received
            .checked_add(sol_received)
            .ok_or(VaultError::ArithmeticOverflow)?;

        // Step 9: Emit swap event
        emit!(SwapExecuted {
            owner: self.user.key(),
            usdc_amount,
            sol_amount: sol_received,
            periods_completed: self.vault.periods_completed,
            next_swap_time: self.vault.next_swap_time,
            timestamp: curr_time,
        });

        Ok(())
    }
}
