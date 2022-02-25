use anchor_lang::prelude::*;
use serum_multisig::{cpi::{accounts::Approve, approve}, program::SerumMultisig};
use crate::model::Membership;

#[derive(Accounts)]
pub struct ApproveCpi<'info> {
    /// CHECK: 
    multisig: UncheckedAccount<'info>,
    /// CHECK:
    #[account(mut)]
    transaction: UncheckedAccount<'info>,
        #[account(
        has_one = wallet,
        seeds = [b"membership", membership.wallet.as_ref()],
        bump = membership.bump,
        )]
    membership: Account<'info, Membership>,
    wallet: Signer<'info>,
    multisig_program: Program<'info, SerumMultisig>
}

pub fn handler(ctx: Context<ApproveCpi>) -> Result<()> {

    let program = ctx.accounts.multisig_program.to_account_info();

    let membership = &ctx.accounts.membership;

    let accounts = Approve {
        multisig: ctx.accounts.multisig.to_account_info(),
        transaction: ctx.accounts.transaction.to_account_info(),
        owner: ctx.accounts.membership.to_account_info(),
    };

    let seeds = &[
        &b"membership"[..],
        membership.wallet.as_ref(),
        &[membership.bump],
    ];

    let signer = &[&seeds[..]];
    let ctx_cpi = CpiContext::new_with_signer(program, accounts, signer);
    
    approve(ctx_cpi)

}
