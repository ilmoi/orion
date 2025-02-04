use crate::{error::*, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TransferMerchantAuthority<'info> {
    #[account(constraint = payer.key() == merchant_authority.current_authority @ ErrorCode::IncorrectAuthority)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [b"merchant_authority", merchant_authority.key().as_ref(), init_authority.key().as_ref()], bump)]
    pub merchant_authority: Account<'info, MerchantAuthority>,

    #[account(constraint = init_authority.key() == merchant_authority.init_authority @ ErrorCode::IncorrectInitAuthority)]
    pub init_authority: UncheckedAccount<'info>,

    pub proposed_authority: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<TransferMerchantAuthority>) -> ProgramResult {
    let merchant_authority = &mut ctx.accounts.merchant_authority;
    let proposed_authority = &mut ctx.accounts.proposed_authority;

    merchant_authority.pending_authority = proposed_authority.key();

    Ok(())
}
