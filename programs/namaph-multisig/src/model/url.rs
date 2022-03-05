use anchor_lang::prelude::*;

#[account]
pub struct UrlTopic {
   pub multisig: Pubkey,
   pub title: String,
   pub url: String,
   pub authority: Pubkey,
   pub bump: u8
}

impl UrlTopic {
    pub const MAX_TITLE_BYTES:usize = 64;
    pub const MAX_URL_BYTES: usize = 256;
    pub const SIZE:usize = 
        8 + // disc
        32 + // multisig
        4 + Self::MAX_TITLE_BYTES + // in ascii
        4 + Self::MAX_URL_BYTES + // in ascii
        32 + // authority 
        1;
}


