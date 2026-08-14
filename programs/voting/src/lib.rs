use anchor_lang::prelude::{borsh::de, *};

declare_id!("JBmRNtAuk4adsXxhQaSj1LVCPBj1pWRpnX1E1D1tQcm5");

#[program]
pub mod voting {
    use super::*;

    pub fn init_poll(ctx: Context<InitPoll>, _poll_id: u64, start: u64, end: u64, name: String, description: String) -> Result<()> {
        let mut pool = &mut ctx.accounts.poll_account;
        pool.poll_name = name;
        pool.poll_description = description;
        pool.poll_voting_start = start;
        pool.poll_voting_end = end;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitPoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"pool".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>
}


#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name : String,
    #[max_len(280)]
    pub poll_description : String,
    pub poll_voting_start : u64,
    pub poll_voting_end : u64,
    pub poll_options_index: u64,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name : String,
    pub candidate_votes: u64,
}
