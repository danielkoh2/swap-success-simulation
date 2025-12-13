use anchor_lang::prelude::*;

declare_id!("14iYe7T2Tm3fdr22x6gRfY2yJ5yTYaco9vhWR4H1oKyk");

#[program]
pub mod swap_success_simulation {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
