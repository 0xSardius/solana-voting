use anchor_lang::prelude::*;

declare_id!("BTzpSv33D4FcRcGT8urd1MJF9SkN3sneRD21e7hB8v86");

#[program]
pub mod solana_voting {
    use super::*;

    pub fn init_poll(ctx: Context<InitPoll>) -> Result<()> {
        init_poll::handler(ctx);
    }
}

#[derive(Accounts)]
pub struct InitPoll {
    #[account(mut)]
    pub signer: Signer<'info>
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount(
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
)

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}
