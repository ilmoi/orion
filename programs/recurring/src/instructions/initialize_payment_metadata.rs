use crate::{error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
#[instruction(amount_delegated: u64)]
pub struct InitializePaymentMetadata<'info> {
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        seeds = [b"payment_metadata", payer.key().as_ref(), payment_config.key().as_ref()],
        bump
    )]
    pub payment_metadata: Account<'info, PaymentMetadata>,

    pub payment_config: Account<'info, PaymentConfig>,

    #[account(
        constraint = owner_payment_account.mint == payment_config.payment_mint @ ErrorCode::IncorrectMint,
        constraint = owner_payment_account.amount >= amount_delegated @ ErrorCode::InsufficientBalanceToDelegate
    )]
    pub owner_payment_account: Account<'info, TokenAccount>,

    #[account(seeds = [b"program", b"signer"], bump)]
    pub program_as_signer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializePaymentMetadata>, amount_delegated: u64) -> ProgramResult {
    let bump = *ctx.bumps.get("payment_metadata").unwrap();
    let payment_metadata = &mut ctx.accounts.payment_metadata;
    let payment_config = &mut ctx.accounts.payment_config;

    require!(
        amount_delegated > payment_config.minimum_amount_to_delegate,
        ErrorCode::AmountToDelegateIsSmallerThanMinimum
    );

    payment_metadata.owner = ctx.accounts.payer.key();
    payment_metadata.payment_config = ctx.accounts.payment_config.key();
    payment_metadata.owner_payment_account = ctx.accounts.owner_payment_account.key();
    payment_metadata.amount_delegated = amount_delegated;
    payment_metadata.bump = bump;

    Ok(())
}
