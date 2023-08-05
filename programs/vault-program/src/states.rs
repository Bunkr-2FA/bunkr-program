use {
    crate::{constants::*, errors::ErrorCode},
    anchor_lang::{prelude::*, solana_program::hash::Hash},
};

#[account()]
pub struct Bunkr {
    pub raw_id: Vec<u8>,
    pub public_key: [u8; 64],
    pub withdraw_address: Pubkey,
    pub current_reset_hash: [u8; 32],
    pub final_reset_hash: [u8; 32],
}

pub fn validate_password(on_chain_password: [u8; 32], password: &[u8; 32]) -> Result<()> {
    let password_to_check = Hash::new(password.as_slice());
    assert!(
        on_chain_password == password_to_check.to_bytes(),
        "Password Mismatch"
    );
    Ok(())
}

pub struct Memo;

impl anchor_lang::Id for Memo {
    fn id() -> Pubkey {
        spl_memo::ID
    }
}
