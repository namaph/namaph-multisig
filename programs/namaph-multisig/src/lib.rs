mod error;
mod instructions;
mod model;
mod util;

use crate::model::TransactionAccountCpi;
use anchor_lang::prelude::*;
use instructions::*;

declare_id!("A13BbQ3UV9CvermetyC5ymZN2gqCLTM8CqpwragH7kCX");

#[program]
pub mod namaph_multisig {

    use super::*;

    pub fn initialize(
        ctx: Context<Init>,
        username: String,
        map_name: String,
        capacity: u8,
        nonce: u8,
    ) -> Result<()> {
        init::handler(ctx, username, map_name, capacity, nonce)
    }

    pub fn update_topology(ctx: Context<UpdateTopology>, id: u8, value: u8) -> Result<()> {
        update_topology::handler(ctx, id, value)
    }

    pub fn add_membership_and_create_transaction(
        ctx: Context<AddMembershipAndCreateTransactionCpi>,
        username: String,
        user: Pubkey,
        pid: Pubkey,
        accs: Vec<TransactionAccountCpi>,
        data: Vec<u8>,
        ) -> Result<()> {
        add_membership::handler(ctx, username, user, pid, accs, data)
    }

    pub fn create_transaction(
        ctx: Context<CreateTransactionCpi>,
        pid: Pubkey,
        accs: Vec<TransactionAccountCpi>,
        data: Vec<u8>,
    ) -> Result<()> {
        create_transaction::handler(ctx, pid, accs, data)
    }

    pub fn approve(ctx: Context<ApproveCpi>) -> Result<()> {
        approve_cpi::handler(ctx)
    }
}
