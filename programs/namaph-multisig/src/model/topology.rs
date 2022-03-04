use anchor_lang::prelude::*;

#[account]
pub struct Topology {
   pub multisig: Pubkey,
   pub map_name: String,
   pub capacity: u8,
   pub values: Vec<u8>,
   pub authority: Pubkey,
   pub bump: u8
}

impl Topology {
    pub const MAX_NAME_BYTES:usize = 32;
    pub fn size(capacity: u8) -> usize{
        8 + // disc
        32 + // multisig
        4 + Self::MAX_NAME_BYTES + // in ascii
        1 +  // cap
        4 + capacity as usize +
        32 + // authority 
        1
    }
}


