use anchor_lang::prelude::*;

#[account]
pub struct Membership {
    pub wallet: Pubkey,
    pub multisig: Pubkey,
    pub username: String,
    pub bump: u8,
}

impl Membership{
    pub const MAX_NAME_BYTES: usize = 40;
    pub const SIZE: usize = 8 + 32 + 32 + 4 + Self::MAX_NAME_BYTES + 1;
}
