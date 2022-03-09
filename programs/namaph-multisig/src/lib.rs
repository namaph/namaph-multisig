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

    pub fn delete_membership_and_create_transaction(
        ctx: Context<DeleteMembershipAndCreateTransactionCpi>,
        pid: Pubkey,
        accs: Vec<TransactionAccountCpi>,
        data: Vec<u8>,
    ) -> Result<()> {
        delete_membership::handler(ctx, pid, accs, data)
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

    pub fn create_treasury(
        ctx: Context<CreateTreasury>,
        treasury_name: String,
        authority: Pubkey,
    ) -> Result<()> {
        create_treasury::handle(ctx, treasury_name, authority)
    }

    pub fn spend(ctx: Context<Spend>, amount: u64) -> Result<()> {
        spend::handler(ctx, amount)
    }

    pub fn create_url_topic(
        ctx: Context<CreateUrlTopic>,
        title: String,
        authority: Pubkey,
        pid: Pubkey,
        accs: Vec<TransactionAccountCpi>,
        data: Vec<u8>
    ) -> Result<()> {
        create_url_topic::handle(ctx, title, authority, pid, accs, data)
    }

    pub fn update_url_topic(
        ctx: Context<UpdateUrlTopic>,
        proposer: Pubkey,
        title: String,
        url: String,
        ) -> Result<()> {
        update_url_topic::handle(ctx, proposer, title, url)
    }

    pub fn create_text_topic(
        ctx: Context<CreateTextTopic>,
        title: String,
        authority: Pubkey,
        pid: Pubkey,
        accs: Vec<TransactionAccountCpi>,
        data: Vec<u8>
    ) -> Result<()> {
        create_text_topic::handle(ctx, title, authority, pid, accs, data)
    }

    pub fn update_text_topic(
        ctx: Context<UpdateTextTopic>,
        proposer: Pubkey,
        title: String, 
        body: String,
        ) -> Result<()> {
        update_text_topic::handle(ctx, proposer, title, body)
    }

}
