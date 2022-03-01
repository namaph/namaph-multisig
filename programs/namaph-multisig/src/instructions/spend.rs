use std::borrow::Borrow;

use crate::model::treasury::Treasury;
use crate::util::fit_string;
use crate::error::NamaphError;
use anchor_lang::prelude::*;


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Spend<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [b"treasury", treasury.multisig.as_ref(), fit_string(&treasury.name)],
        bump = treasury.bump
        )]
    treasury: Account<'info, Treasury>,
    authority: Signer<'info>,
    /// CHECK: it's the proposers responsibility to check this
    #[account(mut)]
    to: UncheckedAccount<'info>
}

pub fn handler(ctx:Context<Spend>, amount: u64) -> Result<()> {
    let from = ctx.accounts.treasury.to_account_info();
    let to = ctx.accounts.to.to_account_info();

    let balance = *from.lamports().borrow();
    require!(balance > amount, NamaphError::NotEnoughBalanceInTreasury);

    // actual paying
    **from.try_borrow_mut_lamports()? -= amount;
    **to.try_borrow_mut_lamports()? += amount;

    Ok(())
}
