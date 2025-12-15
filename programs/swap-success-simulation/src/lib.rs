mod instructions;
mod state;
mod error;

use anchor_lang::prelude::*;

pub use instructions::*;

declare_id!("14iYe7T2Tm3fdr22x6gRfY2yJ5yTYaco9vhWR4H1oKyk");

#[program]
pub mod swap_success_simulation {
    use super::*;

    pub fn send_to_wallet(ctx: Context<SendToWallet>, amount: u64, min_profit: u64) -> Result<()> {
        instructions::send_to_wallet::send_from_vault_to_wallet(&ctx, amount)?;
        instructions::send_to_wallet::save_init_data(ctx, amount, min_profit)
    }
    
    pub fn swap_simulate(ctx: Context<SwapSimulate>, success: bool) -> Result<()> {
        instructions::swap_simulate::send_from_b_to_a(&ctx, success)?;
        instructions::swap_simulate::save_simulate_data(ctx, success)
    }
    
    pub fn send_to_vault(ctx: Context<SendToVault>) -> Result<()> {
        instructions::send_to_vault::send_from_wallet_a_to_vault(&ctx)?;
        instructions::send_to_vault::save_finish_data(ctx)
    }
}
