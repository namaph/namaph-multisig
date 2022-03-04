use anchor_lang::prelude::*;
use crate::util::fit_string;
use crate::model::{Topology, Membership};
use serum_multisig::{cpi::{accounts::CreateMultisig, create_multisig}, program::SerumMultisig};
use crate::error::NamaphError;

#[derive(Accounts)]
#[instruction(username:String, map_name: String, capacity: u8, nonce: u8)]
pub struct Init<'info> {
    #[account(init, 
              payer=payer, 
              space=Topology::size(capacity),
              seeds=[b"topology", fit_string(&map_name)],
              bump
              )]
    topology: Account<'info, Topology>,
    multisig: Signer<'info>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(
        init, 
        space = Membership::SIZE,
        payer = payer, 
        seeds = [b"membership", payer.key().as_ref()],
        bump
        )]
    membership: Account<'info, Membership>,
    multisig_program: Program<'info, SerumMultisig>,
    system_program: Program<'info, System>
}

pub fn handler(ctx: Context<Init>, username:String, map_name: String, capacity:u8, nonce: u8) -> Result<()> {
    require!(map_name.chars().count() < Topology::MAX_NAME_BYTES, NamaphError::StringTooLong);
    
    require!(username.chars().count() < Membership::MAX_NAME_BYTES, NamaphError::StringTooLong);
    
    let payer = & ctx.accounts.payer;

    let membership = &mut ctx.accounts.membership;
    membership.wallet = payer.key();
    membership.bump = *ctx.bumps.get("membership").ok_or(NamaphError::BumpMismatch)?;
    membership.username = username;
    
    let program = ctx.accounts.multisig_program.to_account_info();
    let multisig = ctx.accounts.multisig.to_account_info();
    let multisig_pubkey = multisig.key();
            
    let seeds = &[multisig_pubkey.as_ref()];
    let (multisig_signer, _) = Pubkey::find_program_address(seeds, &program.key());
    
    // cpi
    let accounts = CreateMultisig { 
        multisig: multisig.to_account_info()
    };
    
    let ctx_cpi = CpiContext::new(program, accounts);
    let owners = [membership.key()].to_vec();
    
    create_multisig(ctx_cpi, owners, 1, nonce)?;
    
    let topology = &mut ctx.accounts.topology;
    topology.authority = multisig_signer; 
    topology.capacity = capacity;
    topology.multisig = multisig.key();
    topology.map_name = map_name;
    topology.bump = *ctx.bumps.get("topology").ok_or(NamaphError::BumpMismatch)?;
    topology.values = (0..capacity).map(|_|0).collect();
    
    Ok(())
}

