pub mod membership;
pub mod topology;
pub mod treasury;
pub mod url;
pub mod text;

pub use membership::*;
pub use topology::*;
pub use url::*;
pub use text::*;
use anchor_lang::prelude::*;

use serum_multisig::TransactionAccount;

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TransactionAccountCpi {
    pubkey: Pubkey,
    is_writable: bool,
    is_signer: bool,
}

impl From<&TransactionAccountCpi> for TransactionAccount {
    fn from(tacpi: &TransactionAccountCpi) -> Self {
        Self{
            pubkey: tacpi.pubkey,
            is_writable: tacpi.is_writable,
            is_signer: tacpi.is_signer,
        }
    }
}

