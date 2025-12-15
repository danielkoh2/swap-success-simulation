use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::Wallet;
use super::transfer_token_pda;
use super::transfer_token;

#[derive(Accounts)]
pub struct SwapSimulate<'info> {
    #[account(mut)]
    pub vault: SystemAccount<'info>,

    #[account(mut)]
    pub wallet_b: Signer<'info>,

    pub token_mint: InterfaceAccount<'info, Mint>,

    #[account(
    mut,
    associated_token::mint = token_mint,
    associated_token::authority = wallet_b,
    associated_token::token_program = token_program,
    )]
    pub wallet_b_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
    init_if_needed,
    payer = vault,
    space = 8 + Wallet::INIT_SPACE,
    seeds = [b"wallet_seed_a", vault.key().as_ref()],
    bump,
    )]
    pub wallet_a: Account<'info, Wallet>,

    #[account(
    init_if_needed,
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

pub fn send_from_b_to_a(ctx: &Context<SwapSimulate>, success: bool) -> Result<()> {
    if success {
        msg!("Swap Simulate: success");
        transfer_token(
            &ctx.accounts.wallet_b_token_account,
            &ctx.accounts.wallet_a_token_account,
            &ctx.accounts.wallet_a.received / 10,
            &ctx.accounts.token_mint,
            &ctx.accounts.wallet_b,
            &ctx.accounts.token_program,
        )
    } else {
        msg!("Swap Simulate: not success");
        let seeds = &[
            b"wallet_seed_a",
            ctx.accounts.vault.to_account_info().key.as_ref(),
            &[ctx.accounts.wallet_a.bump],
        ];
        let signer_seeds = [&seeds[..]];
        transfer_token_pda(
            &ctx.accounts.wallet_a_token_account,
            &ctx.accounts.wallet_b_token_account,
            ctx.accounts.wallet_a.received / 10,
            &ctx.accounts.token_mint,
            &ctx.accounts.wallet_a,
            &ctx.accounts.token_program,
            &signer_seeds,
        )
    }
}