use crate::model::Membership;
use crate::{error::NamaphError, model::TransactionAccountCpi};
use anchor_lang::prelude::*;
use serum_multisig::{
    cpi::{
        accounts::{CreateMultisig, CreateTransaction},
        create_transaction,
    },
    program::SerumMultisig,
    TransactionAccount,
};

#[derive(Accounts)]
pub struct CreateTransactionCpi<'info> {
    // the membership is the 'prposer'
    #[account(
        seeds = [b"membership", membership.wallet.as_ref()],
        bump = membership.bump,
        )]
    membership: Account<'info, Membership>,
    /// CHECK: this goes straight to the cpi
    multisig: UncheckedAccount<'info>,
    /// CHECK: this goes straight to the cpi
    transaction: UncheckedAccount<'info>,
    multisig_program: Program<'info, SerumMultisig>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateTransactionCpi>,
    pid: Pubkey,
    accs: Vec<TransactionAccountCpi>,
    data: Vec<u8>,
) -> Result<()> {
    // init the membership
    let membership = &ctx.accounts.membership;

    // cpi
    let program = ctx.accounts.multisig_program.to_account_info();
    let accounts = CreateTransaction {
        multisig: ctx.accounts.multisig.to_account_info(),
        transaction: ctx.accounts.transaction.to_account_info(),
        // we need this to be the signer
        // but we are signing thourgh PDA's
        proposer: membership.to_account_info(),
    };

    let seeds = &[
        &b"membership"[..],
        membership.wallet.as_ref(),
        &[membership.bump],
    ];
    let signer = &[&seeds[..]];
    let ctx_cpi = CpiContext::new_with_signer(program, accounts, signer);

    let accs: Vec<TransactionAccount> = accs.iter().map(Into::into).collect();

    create_transaction(ctx_cpi, pid, accs, data)
}
