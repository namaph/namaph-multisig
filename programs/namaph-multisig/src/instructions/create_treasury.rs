use crate::error::NamaphError;
use crate::model::treasury::Treasury;
use crate::util::fit_string;
use anchor_lang::prelude::*;
use serum_multisig::program::SerumMultisig;
use serum_multisig::Multisig;

#[derive(Accounts)]
#[instruction(treasury_name: String, authority: Pubkey)]
pub struct CreateTreasury<'info> {
    #[account(
        init,
        payer = payer,
        space = Treasury::SIZE,
        seeds = [b"treasury", multisig.key().as_ref(), fit_string(&treasury_name)],
        bump,
              )]
    treasury: Account<'info, Treasury>,
    #[account(mut)]
    payer: Signer<'info>,
    multisig: Account<'info, Multisig>,
    system_program: Program<'info, System>,
}

pub fn handle(ctx: Context<CreateTreasury>, treasury_name: String, authority: Pubkey) -> Result<()> {
    require!(
        treasury_name.chars().count() < Treasury::MAX_NAME_CHAR_COUNT,
        NamaphError::StringTooLong
    );

    let multisig = &ctx.accounts.multisig;

    let m_key = multisig.key();
    let seeds = &[m_key.as_ref()];
    let treasury = &mut ctx.accounts.treasury;
    treasury.name = treasury_name;
    treasury.authority = authority;
    treasury.bump = *ctx.bumps.get("treasury").ok_or(NamaphError::BumpMismatch)?;
    treasury.multisig = multisig.key();

    Ok(())
}
