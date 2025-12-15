use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{state::Wallet, error::ErrorCode};
use super::transfer_token_pda;

#[derive(Accounts)]
pub struct SendToVault<'info> {
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

pub fn send_from_wallet_a_to_vault(ctx: &Context<SendToVault>) -> Result<()> {
    let seeds = &[
        b"wallet_seed_a",
        ctx.accounts.vault.to_account_info().key.as_ref(),
    ];
    let signer_seeds = [&seeds[..]];
    transfer_token_pda(
        &ctx.accounts.wallet_a_token_account,
        &ctx.accounts.vault_token_account,
        ctx.accounts.wallet_a.amount,
        &ctx.accounts.token_mint,
        &ctx.accounts.wallet_a,
        &ctx.accounts.token_program,
        &signer_seeds,
    )
}

pub fn save_finish_data(ctx: Context<SendToVault>) -> Result<()> {
    if ctx.accounts.wallet_a.amount < ctx.accounts.wallet_a.received {
        return Err(error!(ErrorCode::SlippageError));
    }

    let profit = ctx.accounts.wallet_a.amount - ctx.accounts.wallet_a.received;

    if profit < ctx.accounts.wallet_a.min_profit {
        return Err(error!(ErrorCode::MinProfitError));
    } else {
        ctx.accounts.wallet_a.set_inner(Wallet {
            amount: 0,
            received: 0,
            min_profit: 0,
        })
    };

    Ok(())
}