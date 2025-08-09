use anchor_lang::prelude::*;

pub mod states;
pub mod instructions;
pub mod consts;
pub mod errors;
pub mod events;


declare_id!("FLCeHJtrs6ENYehB6BC3TctxHUPqzsquBGqhJHgQnyE3");


#[program]
pub mod permit_hook {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
