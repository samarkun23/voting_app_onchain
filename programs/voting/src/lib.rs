use anchor_lang::prelude::{borsh::de, *};

declare_id!("JBmRNtAuk4adsXxhQaSj1LVCPBj1pWRpnX1E1D1tQcm5");

#[program]
pub mod voting {
    use super::*;

    pub fn init_poll(ctx: Context<InitPoll>, _poll_id: u64, start: u64, end: u64, name: String, description: String) -> Result<()> {
        let pool = &mut ctx.accounts.poll_account;
        pool.poll_name = name;
        pool.poll_description = description;
        pool.poll_voting_start = start;
        pool.poll_voting_end = end;
        Ok(())
    }

    pub fn initialize_candidate(ctx: Context<InitCandidate>, _poll_id: u64, name: String) -> Result<()> {
        ctx.accounts.candidate_account.candidate_name = name;
        ctx.accounts.poll_account.poll_options_index += 1;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()>{
        let candidate = &mut ctx.accounts.candidate_account;

        let current_time = Clock::get()?.unix_timestamp;  // this is u64 type that we save the start and end time u64

        if current_time > (ctx.accounts.poll_account.poll_voting_end as i64) {
            return Err(ErrorCode::VotingEnded.into());
        }

        if current_time <= (ctx.accounts.poll_account.poll_voting_end as i64) {
            return Err(ErrorCode::VotingNotStarted.into());
        }

        candidate.candidate_votes += 1;

        Ok(())
    }
}

// initialize candidate account
#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitCandidate<'info> {
    #[account(mut)]
    pub singer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"pool".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = singer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [b"candidate".as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub singer: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"pool".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [b"candidate".as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,
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

#[error_code]
pub enum ErrorCode{
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded
}