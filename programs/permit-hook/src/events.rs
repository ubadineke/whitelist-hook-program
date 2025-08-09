use anchor_lang::prelude::*;

#[event]
pub struct HookApproved {
    pub hook_id: Pubkey,
    pub proposal_id: u64,
    pub votes_for: u64,
    pub votes_against: u64,
}
