use crate::error::NamaphError;
use crate::model::TransactionAccountCpi;
use crate::model::url::UrlTopic;
use crate::model::membership::Membership;
use crate::util::fit_string;
use anchor_lang::prelude::*;
use serum_multisig::{Multisig, TransactionAccount};
use serum_multisig::{
    cpi::{accounts::CreateTransaction, create_transaction},
    program::SerumMultisig,
};

#[derive(Accounts)]
#[instruction(title: String, authority: Pubkey, pid: Pubkey, accs: Vec<TransactionAccountCpi>, data: Vec<u8>)]
pub struct CreateUrlTopic<'info> {
    #[account(
        init,
        payer = wallet,
        space = UrlTopic::SIZE,
        seeds = [b"url", multisig.key().as_ref(), fit_string(&title)],
        bump,
              )]
    url_topic: Account<'info, UrlTopic>,
    multisig: Account<'info, Multisig>,
    system_program: Program<'info, System>,
    
    //cpi
    #[account(
        seeds = [b"membership", multisig.key().as_ref(), wallet.key().as_ref()],
        bump = proposer.bump,
        has_one = wallet
        )]
    proposer: Account<'info, Membership>,
    #[account(mut)]
    wallet: Signer<'info>,
    /// CHECK: this goes to the multisig program via CPI
    transaction: UncheckedAccount<'info>,
    multisig_program: Program<'info, SerumMultisig>,
}

pub fn handle(
    ctx: Context<CreateUrlTopic>,
    title: String,
    authority: Pubkey,
    pid: Pubkey,
    accs: Vec<TransactionAccountCpi>,
    data: Vec<u8>
) -> Result<()> {

    require!(
        title.len() < UrlTopic::MAX_TITLE_BYTES,
        NamaphError::StringTooLong
    );

    let multisig = &ctx.accounts.multisig;

    let url_topic = &mut ctx.accounts.url_topic;
    url_topic.title = title;
    url_topic.authority = authority;
    url_topic.bump = *ctx.bumps.get("url_topic").ok_or(NamaphError::BumpMismatch)?;
    url_topic.multisig = multisig.key();

    // cpi
    let proposer = &ctx.accounts.proposer;
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
