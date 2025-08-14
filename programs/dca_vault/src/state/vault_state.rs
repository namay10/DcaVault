use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultState {
    pub owner: Pubkey,
    pub total_amount: u64,
    pub periods: u16,
    pub interval_seconds: u64,
    pub staked_at: i64,
    pub curr_balance: u64,
    pub periods_completed: u16,
    pub next_swap_time: i64,
    pub total_sol_received: u64,
    pub bump: u8,
}
