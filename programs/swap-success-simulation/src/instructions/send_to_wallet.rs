use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::Wallet;
use super::transfer_token;

#[derive(Accounts)]
pub struct SendToWallet<'info> {
    #[account(mut)]
    pub vault: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
    mut,
    associated_token::mint = token_mint,
    associated_token::authority = vault,
    associated_token::token_program = token_program,
    )]
    pub vault_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
    init,
    payer = vault,
    space = 8 + Wallet::INIT_SPACE,
    seeds = [b"wallet_seed_a", vault.key().as_ref()],
    bump,
    )]
    pub wallet_a: Account<'info, Wallet>,

    #[account(
    init,
    payer = vault,
    associated_token::mint = token_mint,
    associated_token::authority = wallet_a,
    associated_token::token_program = token_program,
    )]
    pub wallet_a_token_account: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn send_from_vault_to_wallet(ctx: &Context<SendToWallet>, amount: u64) -> Result<()> {
    transfer_token(
        &ctx.accounts.vault_token_account,
        &ctx.accounts.wallet_a_token_account,
        amount,
        &ctx.accounts.token_mint,
        &ctx.accounts.vault,
        &ctx.accounts.token_program,
    )
}

pub fn save_init_data(ctx: Context<SendToWallet>, amount: u64, min_profit: u64) -> Result<()> {
    ctx.accounts.wallet_a.set_inner(Wallet {
        received: amount,
        min_profit,
        bump: ctx.bumps.wallet_a,
    });

    Ok(())
}