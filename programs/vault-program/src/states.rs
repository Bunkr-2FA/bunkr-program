use {crate::constants::*, anchor_lang::prelude::*, anchor_lang::solana_program::pubkey::Pubkey};


#[account()]
pub struct Bunkr {
    pub name: String,
    pub withdraw_address: Pubkey,
    pub activated: bool,
    pub init_time: u32,
    pub last_accessed_interval: u32,
    pub root: [u8; 32],
    pub current_hash: [u8; 32],
    pub final_hash: [u8; 32],
    pub current_reset_hash: [u8; 32],
    pub final_reset_hash: [u8; 32],
    pub shadow_drive_space: String
}