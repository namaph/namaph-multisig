use anchor_lang::prelude::*;
use serum_multisig::{
    cpi::{accounts::Auth, set_owners},
    program::SerumMultisig,
    Multisig,
};

#[derive(Accounts)]
pub struct SetOwnersCpi<'info> {
    #[account(mut)]
    multisig: Box<Account<'info, Multisig>>,
    // #[account(
    //    seeds = [multisig.key().as_ref()],
    //    bump = multisig.nonce
    //    )]
    /// CHECK: 
    multisig_signer: Signer<'info>,
    multisig_program: Program<'info, SerumMultisig>,
}

pub fn handler(ctx: Context<SetOwnersCpi>, owners: Vec<Pubkey>) -> Result<()> {
    let multisig = &ctx.accounts.multisig;

    // we make this a signer
    // let mut ms_account = ctx.accounts.multisig_signer.to_account_info();
    // ms_account.is_signer = true;

    let program = ctx.accounts.multisig_program.to_account_info();
    let accounts = Auth {
        multisig: multisig.to_account_info(),
        multisig_signer: ctx.accounts.multisig_signer.to_account_info(),
    };

    let m_key = multisig.key();
    // let seeds = &[m_key.as_ref(), &[multisig.nonce]];
    let seeds = &[m_key.as_ref()];
    let signer = &[&seeds[..]];

    msg!("cpi, set_owners");

    let ctx_cpi = CpiContext::new_with_signer(program, accounts, signer);
    set_owners(ctx_cpi, owners)
}

// // Sets the owners field on the multisig. The only way this can be invoked
// // is via a recursive call from execute_transaction -> set_owners.
// pub fn set_owners(ctx: Context<Auth>, owners: Vec<Pubkey>) -> Result<()> {
//     assert_unique_owners(&owners)?;
//     require!(!owners.is_empty(), InvalidOwnersLen);
//
//     let multisig = &mut ctx.accounts.multisig;
//
//     if (owners.len() as u64) < multisig.threshold {
//         multisig.threshold = owners.len() as u64;
//     }
//
//     multisig.owners = owners;
//     multisig.owner_set_seqno += 1;
//
//     Ok(())
// }
// #[derive(Accounts)]
// pub struct Auth<'info> {
//     #[account(mut)]
//     multisig: Box<Account<'info, Multisig>>,
//     #[account(
//         seeds = [multisig.key().as_ref()],
//         bump = multisig.nonce,
//     )]
//     multisig_signer: Signer<'info>,
// }
//
