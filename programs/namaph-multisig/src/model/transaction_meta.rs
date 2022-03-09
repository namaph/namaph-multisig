use anchor_lang::prelude::*;

#[account]
pub struct TransactionMeta {
    pub proposer: Pubkey,
    pub timestamp: i64,
    pub bump: u8, 
}

impl TransactionMeta {
    pub const SIZE: usize = 8 + 32 + 8 + 1;
}
