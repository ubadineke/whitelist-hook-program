use anchor_lang::prelude::*;
use crate::states::Whitelist;

#[derive(Accounts)]
pub struct InitializeWhitelist<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        space = 8 + Whitelist::INIT_SPACE,
        seeds = [b"whitelist"],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    pub system_program: Program<'info, System>,
}

impl InitializeWhitelist<'_> {
  pub fn initialize_whitelist(ctx: Context<InitializeWhitelist>) -> Result<()> {
    let whitelist = &mut ctx.accounts.whitelist;
    whitelist.admin = ctx.accounts.admin.key();
    whitelist.hooks = vec![];
    whitelist.proposals = vec![];
    whitelist.vote_threshold = 1_000_000_000; // Example: 1M RAY tokens (9 decimals)
    Ok(())
  }
}



