use anchor_lang::prelude::*;

declare_id!("BTzpSv33D4FcRcGT8urd1MJF9SkN3sneRD21e7hB8v86");

#[program]
pub mod solana_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
