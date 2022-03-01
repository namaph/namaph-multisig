use anchor_lang::prelude::*;

#[account]
pub struct Treasury {
    pub multisig: Pubkey,
    pub authority: Pubkey, // multisig_signer
    pub name: String,
    pub bump: u8,
}

impl Treasury {
    pub const MAX_NAME_CHAR_COUNT: usize = 32;
    pub const SIZE: usize = 8 + 32 + 32 + 4 + Self::MAX_NAME_CHAR_COUNT;
    // name should be ascii not unicode
}



