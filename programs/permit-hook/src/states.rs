use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Whitelist {
    pub admin: Pubkey,
    #[max_len(10)]
    pub hooks: Vec<Pubkey>,
    #[max_len(20)]
    pub proposals: Vec<u64>,
    pub vote_threshold: u64,
}

#[account]
#[derive(InitSpace)]
pub struct HookProposal {
    pub hook_id: Pubkey,
    pub audit_hash: [u8; 32],
    pub votes_for: u64,
    pub votes_against: u64,
    pub proposer: Pubkey,
    pub active: bool,
    pub id: u64,
    pub created_at: i64,
}
