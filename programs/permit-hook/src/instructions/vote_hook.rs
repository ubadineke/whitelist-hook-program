use anchor_lang::prelude::*;
use anchor_spl::token_interface:: TokenAccount;
use crate::states::*;
use crate::consts::*;
use crate::errors::*;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct VoteHook<'info> {
    #[account(
        mut,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, HookProposal>,
    #[account(
        constraint = voter_token_account.mint == RAY_TOKEN_MINT @ WhitelistHookError::InvalidTokenAccount,
        constraint = voter_token_account.owner == voter.key() @ WhitelistHookError::InvalidTokenOwner,
    )]
    pub voter_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(signer)]
    pub voter: Signer<'info>,
} 
 
  /// Vote on a hook proposal using RAY token balance
  pub fn vote_hook(ctx: Context<VoteHook>, proposal_id: u64, vote_for: bool) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    require!(proposal.active, WhitelistHookError::ProposalInactive);
    require!(
        Clock::get()?.unix_timestamp < proposal.created_at + 7 * 24 * 3600,
        WhitelistHookError::VotingPeriodEnded
    );

    let voter_balance = ctx.accounts.voter_token_account.amount;
    if vote_for {
        proposal.votes_for += voter_balance;
    } else {
        proposal.votes_against += voter_balance;
    }
    Ok(())
}