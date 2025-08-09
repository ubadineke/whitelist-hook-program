use anchor_lang::prelude::*;
use anchor_spl::token_interface::TokenAccount;

use crate::states::*;
use crate::consts::*;
use crate::errors::*;

#[derive(Accounts)]
pub struct CheckHook<'info> {
    pub whitelist: Account<'info, Whitelist>,
}

   /// Check if a hook is whitelisted (called by AMM)
   pub fn check_hook(ctx: Context<CheckHook>, hook_id: Pubkey) -> Result<()> {
    let whitelist = &ctx.accounts.whitelist;
    require!(whitelist.hooks.contains(&hook_id), WhitelistHookError::HookNotWhitelisted);
    Ok(())
}