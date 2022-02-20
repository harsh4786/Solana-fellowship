use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod voting {
    use super::*;
    //giving the right to vote and starting the voting process
    pub fn initialize_voting(ctx: Context<Initialize>, chairperson: Pubkey) -> ProgramResult {
        let newBallot = &mut ctx.accounts.ballot;
        newBallot.ballot_authority = chairperson;
        newBallot.winner = Proposal::default();

        Ok(())
    }
    pub fn delegate(ctx: Context<Delegate>, from_voter: Pubkey, to_voter: Pubkey) -> ProgramResult {
        if &ctx.accounts.from_voter.to_account_info().key == &ctx.accounts.to_voter.to_account_info().key {
            return Err(ErrorCode::CannotSelfDelegate.into());
        }
        else if ctx.accounts.from_voter.has_voted == true{
            return Err(ErrorCode::AlreadyDelegated.into());
        }
        else { 
        let from_voter = &mut ctx.accounts.from_voter.to_account_info().key;
        let to_voter = &mut ctx.accounts.to_voter; 
        ctx.accounts.from_voter.has_voted = true;
        }

        Ok(())
    }
    pub fn vote(ctx: Context<Vote>, voter_address: Pubkey, proposal_ID: u8) -> ProgramResult {
    
        let voting_status =  ctx.accounts.voter.has_voted;
        if voting_status == true {
            return Err(ErrorCode::AlreadyVoted.into());
        }
        if voter_address != ctx.accounts.voter.voter_address {
            return Err(ErrorCode::InvalidVoter.into());
        }

        else {
            ctx.accounts.voter.has_voted = true;
            let votes = ctx.accounts.voter.calculate_votes();
            ctx.accounts.proposal.votes += votes;
        }
        Ok(())
    }
    pub fn create_proposal(ctx: Context<CreateProposal>, proposal_ID: u8) -> ProgramResult {
        if ctx.accounts.ballot.ballot_authority != ctx.accounts.chairperson.key() {
            return Err(ErrorCode::NotAuthorizedToAddProposals.into());
        }
        if ctx.accounts.proposal.is_initialized == true{
            return Err(ErrorCode::ProposalAlreadyExists.into());
        }
        let mut newProposal = Proposal::default();
        newProposal.ID = proposal_ID;
        newProposal.votes = 0;
        newProposal.is_initialized = true;
        ctx.accounts.ballot.proposal_list.push(newProposal);

        Ok(())
    }
    pub fn winning_proposal(ctx: Context<Winner>) -> ProgramResult {
        let mut winningVotes = 0;
        //let mut winningProposalName = &mut ctx.accounts.ballot.winner.name;
        //let winningProposal = Proposal::default();
        for proposal in ctx.accounts.ballot.proposal_list.iter() {
            if proposal.votes > winningVotes {
                winningVotes = proposal.votes;
               // winningProposalName = proposal.name;
               // winningProposal = Proposal::new(winningProposalName, winningVotes);
                
            }
        }
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Initialize<'info>{
    pub chairperson: Signer<'info>,
    pub voter: Account<'info, Voter>,
    #[account(init, payer = voter, space = 8 + 4000 + 8)]
    pub ballot: Account<'info, BallotBox>,
    pub system_program: Program<'info, System>,

}
#[derive(Accounts)]
pub struct Delegate<'info>{
    #[account(mut, signer)]
    pub from_voter: Account<'info, Voter>,
    #[account(mut)]
    pub to_voter: Account<'info, Voter>,
    pub ballot: Account<'info, BallotBox>,
}
#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(signer)]
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub ballot: Account<'info, BallotBox>,
    #[account(mut)]
    pub proposal: Account<'info, Proposal>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    pub chairperson: Signer<'info>,
    #[account(init, payer = voter, space = 8 + 8 + 200)]
    pub proposal: Account<'info, Proposal>,
    pub voter: Account<'info, Voter>,
    #[account(mut)]
    pub ballot: Account<'info, BallotBox>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Winner<'info>{
    #[account(mut)]
    pub ballot: Account<'info, BallotBox>,
}

//#[derive(AnchorSerialize, AnchorDeserialize, PartialEq)]
#[account]
pub struct Voter{
    pub voter_address: Pubkey,
    pub votes: u64,
    pub has_voted: bool,
    pub vote_weight: u64,
}
impl Voter{
    pub fn calculate_votes(&self) -> u64 {
        self.votes * self.vote_weight
    }
}
#[account]
#[derive( PartialEq)]
pub struct BallotBox {
    pub ballot_authority: Pubkey,
    //limited to 20 proposals atm
    pub proposal_list: Box<Vec<Proposal>>,
    pub votes: u64,
    pub winner: Proposal,
}
#[account]
#[derive(PartialEq)]
pub struct Proposal {
    pub ID: u8,
    pub votes: u64,
    pub is_initialized: bool,
}
impl Default for Proposal {
    fn default() -> Self {
        Proposal {
            ID: 0,
            votes: 0,
            is_initialized: false,
        }
    }
}
impl Proposal{
    fn new(&self, name: String) -> Self {
        Proposal {
            ID: 0,
            votes:0,
            is_initialized: true,
        }
    }
}
#[error]
pub enum ErrorCode{
    #[msg("You have already voted")]
    AlreadyVoted,
    #[msg("The proposal you entered does not exist in the ballot")]
    InvalidProposal,
    #[msg("This account has already delegated their vote")]
    AlreadyDelegated,
    #[msg("You are not authorized to add proposals")]
    NotAuthorizedToAddProposals,
    #[msg("The proposal you entered already exists")]
    ProposalAlreadyExists,
    #[msg("Cannot self delegate")]
    CannotSelfDelegate,
    #[msg("You are not authorized to vote")]
    InvalidVoter,

}
