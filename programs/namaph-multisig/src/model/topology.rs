use anchor_lang::prelude::*;

#[account]
pub struct Topology {
   pub authority: Pubkey,
   pub map_name: String,
   pub capacity: u8,
   pub values: Vec<u8>,
   pub bump: u8
}

impl Topology {
    pub const MAX_NAME_BYTES:usize = 32;
    pub fn size(capacity: u8) -> usize{
        8 +
        32 +
        4 + Self::MAX_NAME_BYTES + // in ascii
        32 +
        4 + capacity as usize +
        1
    }
}


