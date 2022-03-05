use crate::error::NamaphError;
use crate::model::{Membership, TransactionAccountCpi};
use anchor_lang::prelude::*;
use serum_multisig::TransactionAccount;
use serum_multisig::{
    cpi::{accounts::CreateTransaction, create_transaction},
    program::SerumMultisig,
};

#[derive(Accounts)]
#[instruction(
    username: String, 
    user: Pubkey, 
    pid: Pubkey, 
    accs: Vec<TransactionAccountCpi>,
    data: Vec<u8>)]
pub struct AddMembershipAndCreateTransactionCpi<'info> {
    #[account(
        seeds = [b"membership", multisig.key().as_ref(), wallet.key().as_ref()],
        bump = proposer.bump,
        has_one = wallet
        )]
    proposer: Account<'info, Membership>,
    #[account(mut)]
    wallet: Signer<'info>,
    /// CHECK: this goes straight to the cpi
    multisig: UncheckedAccount<'info>,
    /// CHECK: this goes straight to the cpi
    transaction: UncheckedAccount<'info>,
    multisig_program: Program<'info, SerumMultisig>,
    #[account(
        init,
        payer = wallet,
        space= Membership::SIZE,
        seeds = [b"membership", multisig.key().as_ref(), user.as_ref()],
        bump,
        )]
    membership: Account<'info, Membership>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<AddMembershipAndCreateTransactionCpi>,
    username: String,
    user: Pubkey,
    pid: Pubkey,
    accs: Vec<TransactionAccountCpi>,
    data: Vec<u8>,
) -> Result<()> {
    require!(
        username.chars().count() < Membership::MAX_NAME_BYTES,
        NamaphError::StringTooLong
    );

    let membership = &mut ctx.accounts.membership;
    membership.wallet = user;
    membership.bump = *ctx
        .bumps
        .get("membership")
        .ok_or(NamaphError::BumpMismatch)?;
    membership.username = username;

    let proposer = &ctx.accounts.proposer;

    // cpi
    let program = ctx.accounts.multisig_program.to_account_info();
    let accounts = CreateTransaction {
        multisig: ctx.accounts.multisig.to_account_info(),
        transaction: ctx.accounts.transaction.to_account_info(),
        // we need this to be the signer
        // but we are signing thourgh PDA's
        proposer: ctx.accounts.proposer.to_account_info(),
    };

    let multisig_key = &ctx.accounts.multisig.key();

    let seeds = &[
        &b"membership"[..],
        multisig_key.as_ref(),
        proposer.wallet.as_ref(),
        &[proposer.bump],
    ];
    let signer = &[&seeds[..]];
    let ctx_cpi = CpiContext::new_with_signer(program, accounts, signer);
    let accs: Vec<TransactionAccount> = accs.iter().map(Into::into).collect();

    create_transaction(ctx_cpi, pid, accs, data)
}
