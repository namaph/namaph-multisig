use crate::model::{TransactionMeta, Membership};
use crate::model::TransactionAccountCpi;
use anchor_lang::prelude::*;
use serum_multisig::{
    cpi::{
        accounts::CreateTransaction,
        create_transaction,
    },
    program::SerumMultisig,
    TransactionAccount,
};
use crate::error::NamaphError;


#[derive(Accounts)]
pub struct CreateTransactionCpi<'info> {
    // the membership is the 'proposer'
    #[account(
        seeds = [b"membership", multisig.key().as_ref(), wallet.key().as_ref()],
        bump = membership.bump,
        has_one = wallet,
        )]
    membership: Account<'info, Membership>,
    #[account(mut)]
    wallet: Signer<'info>,
    /// CHECK: this goes straight to the cpi
    multisig: UncheckedAccount<'info>,
    /// CHECK: this goes straight to the cpi
    transaction: UncheckedAccount<'info>,
    #[account(
        init,
        payer=wallet,
        space = TransactionMeta::SIZE,
        seeds = [b"transaction_meta", transaction.key().as_ref()],
        bump,
        )]
    transaction_meta: Account<'info, TransactionMeta>,
    multisig_program: Program<'info, SerumMultisig>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateTransactionCpi>,
    pid: Pubkey,
    accs: Vec<TransactionAccountCpi>,
    data: Vec<u8>,
) -> Result<()> {

    let membership = &ctx.accounts.membership;

    let tx_meta = &mut ctx.accounts.transaction_meta;
    tx_meta.proposer = membership.key();
    let clock = Clock::get()?;
    tx_meta.timestamp = clock.unix_timestamp;
    tx_meta.bump = *ctx.bumps.get("transaction_meta").ok_or(NamaphError::BumpMismatch)?;

    // cpi
    let program = ctx.accounts.multisig_program.to_account_info();
    let accounts = CreateTransaction {
        multisig: ctx.accounts.multisig.to_account_info(),
        transaction: ctx.accounts.transaction.to_account_info(),
        // we need this to be the signer
        // but we are signing thourgh PDA's
        proposer: membership.to_account_info(),
    };

    let multisig_key = &ctx.accounts.multisig.key();

    let seeds = &[
        &b"membership"[..],
        multisig_key.as_ref(),
        membership.wallet.as_ref(),
        &[membership.bump],
    ];
    let signer = &[&seeds[..]];
    let ctx_cpi = CpiContext::new_with_signer(program, accounts, signer);

    let accs: Vec<TransactionAccount> = accs.iter().map(Into::into).collect();

    create_transaction(ctx_cpi, pid, accs, data)
}
