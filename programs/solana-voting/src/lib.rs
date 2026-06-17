use anchor_lang::prelude::*;
use std::str::pattern::StrSearcher;

declare_id!("BTzpSv33D4FcRcGT8urd1MJF9SkN3sneRD21e7hB8v86");

#[program]
pub mod solana_voting {
    use super::*;

    pub fn init_poll(ctx: Context<InitPoll>, poll_id: u64, start: u64, end: u64, name: String, description: String) -> Result<()> {
        let mut poll = ctx.accounts.poll_account;
        poll.poll_description = description;
        poll.poll_voting_start = start;
        poll.poll_voting_end = end;
        poll.poll_name = name;
        poll.poll_option_index = 0;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitPoll {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()], bump,
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
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
