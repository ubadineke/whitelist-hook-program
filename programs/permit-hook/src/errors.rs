use anchor_lang::prelude::*;

#[error_code]
pub enum WhitelistHookError {
    #[msg("Unauthorized access")]
    Unauthorized,
    #[msg("Hook not whitelisted")]
    HookNotWhitelisted,
    #[msg("Proposal is inactive")]
    ProposalInactive,
    #[msg("Voting period has ended")]
    VotingPeriodEnded,
    #[msg("Voting period is still active")]
    VotingPeriodActive,
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    #[msg("Invalid token owner")]
    InvalidTokenOwner,
  }