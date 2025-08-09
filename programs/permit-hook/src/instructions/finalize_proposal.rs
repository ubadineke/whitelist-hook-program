use anchor_lang::prelude::*;
use crate::states::*;
use crate::errors::*;
use crate::events::*;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct FinalizeProposal<'info> {
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(
        mut,
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, HookProposal>,
}

    /// Finalize a proposal and add approved hook to whitelist
    impl FinalizeProposal<'_> {
      pub fn finalize_proposal(ctx: Context<FinalizeProposal>, proposal_id: u64) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let whitelist = &mut ctx.accounts.whitelist;
        require!(proposal.active, WhitelistHookError::ProposalInactive);
        require!(
            Clock::get()?.unix_timestamp >= proposal.created_at + 7 * 24 * 3600,
            WhitelistHookError::VotingPeriodActive
        );
  
        if proposal.votes_for > proposal.votes_against &&
           proposal.votes_for >= whitelist.vote_threshold {
            whitelist.hooks.push(proposal.hook_id);
            emit!(HookApproved {
                hook_id: proposal.hook_id,
                proposal_id,
                votes_for: proposal.votes_for,
                votes_against: proposal.votes_against,
            });
        }
        proposal.active = false;
        Ok(())
    }
  }
