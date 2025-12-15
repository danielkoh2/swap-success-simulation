use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Wallet {
    pub received: u64,
    pub min_profit: u64,
    pub bump: u8
}