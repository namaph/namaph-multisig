use crate::model::{Membership, TransactionAccountCpi};
use anchor_lang::prelude::*;
use serum_multisig::program::SerumMultisig;
use serum_multisig::TransactionAccount;

use serum_multisig::cpi::{accounts::CreateTransaction, create_transaction};

// TODO: this needs rethinking... As soon as this is invoked, the 'Membership' account will be
// closed, while the process of approving the new owners list will be pending.
// As a result, there will be situations that the dao will be deadlocked by setting the quora too
// high.
// This transaction itself needs to be created.

#[derive(Accounts)]
#[instruction(pid: Pubkey, accs: Vec<TransactionAccountCpi>, data: Vec<u8>)]
pub struct DeleteMembershipAndCreateTransactionCpi<'info> {
    #[account(
        seeds = [b"membership", proposer.wallet.as_ref()],
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
        mut,
        close = user,
        seeds = [b"membership", membership.wallet.as_ref()],
        bump = membership.bump,
        )]
    membership: Account<'info, Membership>,
    /// CHECK: this is checked as constraints
    #[account(mut,
              address = membership.wallet)]
    user: UncheckedAccount<'info>,
    system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<DeleteMembershipAndCreateTransactionCpi>,
    pid: Pubkey,
    accs: Vec<TransactionAccountCpi>,
    data: Vec<u8>,
) -> Result<()> {
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

    let seeds = &[
        &b"membership"[..],
        proposer.wallet.as_ref(),
        &[proposer.bump],
    ];

    let signer = &[&seeds[..]];
    let ctx_cpi = CpiContext::new_with_signer(program, accounts, signer);
    let accs: Vec<TransactionAccount> = accs.iter().map(Into::into).collect();

    create_transaction(ctx_cpi, pid, accs, data)
}
