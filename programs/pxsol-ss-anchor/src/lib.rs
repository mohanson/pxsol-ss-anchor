use anchor_lang::prelude::*;

declare_id!("GS5XPyzsXRec4sQzxJSpeDYHaTnZyYt5BtpeNXYuH1SM");

#[program]
pub mod pxsol_ss_anchor {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        let account_user = &ctx.accounts.user;
        let account_user_pda = &mut ctx.accounts.user_pda;
        account_user_pda.auth = account_user.key();
        account_user_pda.bump = ctx.bumps.user_pda;
        account_user_pda.data = Vec::new();
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: Vec<u8>) -> Result<()> {
        let account_user = &ctx.accounts.user;
        let account_user_pda = &mut ctx.accounts.user_pda;

        // Update the data field with the new data.
        account_user_pda.data = data;

        // If the account was shrunk, Anchor won't automatically refund excess lamports. Refund any surplus (over the
        // new rent-exempt minimum) back to the user.
        let account_user_pda_info = account_user_pda.to_account_info();
        let rent_exemption = Rent::get()?.minimum_balance(account_user_pda_info.data_len());
        let hold = **account_user_pda_info.lamports.borrow();
        if hold > rent_exemption {
            let refund = hold.saturating_sub(rent_exemption);
            **account_user_pda_info.lamports.borrow_mut() = rent_exemption;
            **account_user.lamports.borrow_mut() = account_user.lamports().checked_add(refund).unwrap();
        }
        Ok(())
    }
}

/// Fixed seed prefix for PDA derivation, ensuring a stable namespace and avoiding collisions.
const SEED: &[u8] = b"data";

#[derive(Accounts)]
pub struct Init<'info> {
    /// Instruction caller; must be mutable to pay costs and sign.
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init,                                  // Creates the account on first call.
        payer = user,                          // Account rent/creation cost paid by user.
        seeds = [SEED, user.key().as_ref()],   // Must match initialization to locate the same PDA.
        bump,                                  // Must match initialization to locate the same PDA.
        space = Data::space_for(0)             // Initializes space with empty data.
    )]
    pub user_pda: Account<'info, Data>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(new_data: Vec<u8>)]
pub struct Update<'info> {
    /// The authority allowed to mutate and fund reallocation.
    #[account(mut)]
    pub user: Signer<'info>,
    /// PDA derived from authority; reallocated dynamically to fit `new_data`.
    #[account(
        mut,                                                                 // Will write new data.
        seeds = [SEED, user.key().as_ref()],                                 // Must match initialization to locate the same PDA.
        bump = user_pda.bump,                                                // Must match initialization to locate the same PDA.
        realloc = Data::space_for(new_data.len()),                           // Reallocates space based on new data length.
        realloc::payer = user,                                               // Authority pays for expansion.
        realloc::zero = false,                                               // Preserves existing fields without zero-filling (only adjusts capacity).
        constraint = user_pda.auth == user.key() @ PxsolError::Unauthorized, // Additional authority equality check; throws `PxsolError::Unauthorized` on failure.
    )]
    pub user_pda: Account<'info, Data>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Data {
    pub auth: Pubkey,  // Authority public key; only this key can update.
    pub bump: u8,      // PDA bump value ensuring unique address.
    pub data: Vec<u8>, // Business data with variable length.
}

impl Data {
    pub fn space_for(data_len: usize) -> usize {
        // 8 (discriminator) + 32 (authority) + 1 (bump) + 4 (vec len) + data_len.
        8 + 32 + 1 + 4 + data_len
    }
}

#[error_code]
pub enum PxsolError {
    #[msg("Unauthorized")]
    Unauthorized,
}
