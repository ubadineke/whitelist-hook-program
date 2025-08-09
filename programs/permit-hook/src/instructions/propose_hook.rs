use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
#[instruction(proposal_id: u64)]
pub struct ProposeHook<'info> {
    #[account(
        init,
        payer = proposer,
        space = HookProposal::INIT_SPACE, // HookProposal size
        seeds = [b"proposal", proposal_id.to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, HookProposal>,
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub proposer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

  /// Propose a new hook program with an audit hash
  impl ProposeHook<'_>{
  pub fn propose_hook(
    ctx: Context<ProposeHook>,
    proposal_id: u64,
    hook_id: Pubkey,
    audit_hash: [u8; 32],
) -> Result<()> {
    let proposal = &mut ctx.accounts.proposal;
    proposal.hook_id = hook_id;
    proposal.audit_hash = audit_hash;
    proposal.votes_for = 0;
    proposal.votes_against = 0;
    proposal.proposer = ctx.accounts.proposer.key();
    proposal.active = true;
    proposal.id = proposal_id;
    proposal.created_at = Clock::get()?.unix_timestamp;

    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.proposals.push(proposal_id);
    Ok(())
}
  }