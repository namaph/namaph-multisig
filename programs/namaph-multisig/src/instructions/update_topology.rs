use anchor_lang::prelude::*;
use crate::model::Topology;
use crate::model::Membership;

#[derive(Accounts)]
pub struct UpdateTopology<'info>{
    #[account(mut,has_one=authority)]
    topology: Account<'info, Topology>,
    proposer: Account<'info, Membership>,
    authority: Signer<'info>
}

pub fn handler(ctx: Context<UpdateTopology>, id: u8, value: u8) -> Result<()> {
    let topology = &mut ctx.accounts.topology;
    topology.values[id as usize] = value;
    
    Ok(())
}

