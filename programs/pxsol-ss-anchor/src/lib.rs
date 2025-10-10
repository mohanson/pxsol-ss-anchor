use anchor_lang::prelude::*;

declare_id!("GS5XPyzsXRec4sQzxJSpeDYHaTnZyYt5BtpeNXYuH1SM");

#[program]
pub mod pxsol_ss_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
