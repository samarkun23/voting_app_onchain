use anchor_lang::prelude::*;

declare_id!("JBmRNtAuk4adsXxhQaSj1LVCPBj1pWRpnX1E1D1tQcm5");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
#[derive(InitSpace)]
pub struct VotingAccount {
    #[max_len(32)]
    pub poll_name : String,
    #[max_len(280)]
    pub poll_description : String,
    pub poll_voting_start : u64,
    pub poll_voting_end : u64,
    pub poll_options_index: u64,
}

