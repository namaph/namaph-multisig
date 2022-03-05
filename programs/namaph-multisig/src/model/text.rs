use anchor_lang::prelude::*;

#[account]
pub struct TextTopic {
   pub multisig: Pubkey,
   pub title: String,
   pub body: String,
   pub authority: Pubkey,
   pub bump: u8
}

impl TextTopic {
    pub const MAX_TITLE_BYTES: usize = 64;
    pub const MAX_BODY_BYTES: usize = 1600; // 400 characters in 
    pub const SIZE:usize =
        8 + // disc
        32 + // multisig
        4 + Self::MAX_TITLE_BYTES + // in ascii
        4 + Self::MAX_BODY_BYTES + // in ascii
        32 + // authority 
        1;
}


