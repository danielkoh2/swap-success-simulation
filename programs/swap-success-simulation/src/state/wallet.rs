use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Wallet {
    pub amount: u64,
    pub received: u64,
    pub min_profit: u64,
}