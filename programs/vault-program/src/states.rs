use {
    anchor_lang::{prelude::*, solana_program::hash::Hash},
    crate::{constants::*, errors::ErrorCode}
};

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


impl Bunkr {
    pub fn authenticate(&mut self, authentication_object: AuthenticationObject) -> Result<()> {
        let root = self.root;
    let current_time_interval = Clock::get()?.unix_timestamp / 30;
    let password_hash = authentication_object.password_hash;
    let otp_hash = authentication_object.otp_hash;
    let proof_path= authentication_object.proof_path;

    validate_checked(self)?;
    
    validate_password(self.current_hash, &password_hash)?;

    let used_time_interval = validate_root(self.init_time, &current_time_interval, root.to_vec(), proof_path, otp_hash)?;

    validate_time(self.last_accessed_interval , &used_time_interval)?;

    self.last_accessed_interval = used_time_interval as u32;
    self.current_hash = password_hash.as_slice().try_into().unwrap();
    
    msg!("Authentication successful");
    Ok(())
    }
}


pub fn validate_root(init_time: u32, current_time_interval: &i64, root: Vec<u8>, proof_path: Vec<HashTuple>, hash: [u8; 32]) -> Result<i64> {
    assert!(root.len() == 32, "Root must be 32 bytes");
    assert!(hash.len() == 32, "Hash must be 32 bytes");
    assert!(proof_path.len() == 19, "Proof path must be 32 hashes");
    for hash_tuple in proof_path.iter() {
        assert!(hash_tuple.hash.len() == 32, "Hash must be 32 bytes");
    }


    let root = Hash::new_from_array(root.try_into().unwrap());
    let hash: Hash = anchor_lang::solana_program::hash::Hash::new_from_array(hash.try_into().unwrap());
    let time_interval = current_time_interval - (init_time / 30) as i64;
    msg!("Time Interval: {}", time_interval.to_string());

    let mut extended_hashes = vec![];

    for i in -1..3 {
        extended_hashes.push(anchor_lang::solana_program::hash::extend_and_hash(&hash, (&time_interval + i).to_string().as_bytes()));
    }
    let mut calculated_roots: Vec<Hash> = vec![];

    for hash in extended_hashes {

        let mut calculated_hash: Hash = hash.clone();

        for hash_tuple in proof_path.iter() {
            match hash_tuple.sibling_index {
                0 => {
                    calculated_hash = anchor_lang::solana_program::hash::extend_and_hash(&Hash::new_from_array(hash_tuple.hash.try_into().unwrap()), &calculated_hash.to_bytes());
                },
                1 => {
                    calculated_hash = anchor_lang::solana_program::hash::extend_and_hash(&calculated_hash, &hash_tuple.hash);
                },
                _ => {
                    panic!("Invalid Sibling Index")
                }
            }
        }
        calculated_roots.push(calculated_hash);
    }
    msg!("Calculated Roots: {:?}", calculated_roots);

    assert!(calculated_roots.contains(&root), "Root Mismatch");
    let root_integer_index = calculated_roots.iter().position(|x| *x == root).unwrap();
    let used_time_interval = time_interval + (root_integer_index as i64 - 1);


    Ok(used_time_interval)
}

fn validate_time(last_accessed: u32, current_time_interval: &i64) -> Result<()> {
    let current_time_interval = current_time_interval.clone();
    assert!(current_time_interval > last_accessed as i64, "Account has been accessed in the last 30 seconds");
    Ok(())
}

pub fn validate_password(on_chain_password: [u8; 32], password: &[u8; 32]) -> Result<()> {

    let password_to_check = anchor_lang::solana_program::hash::hash(password.as_slice());
    assert!(on_chain_password == password_to_check.to_bytes(), "Password Mismatch");
    Ok(())
}

fn validate_checked(bunkr: &Bunkr) -> Result<()> {
    assert!(bunkr.activated, "Bunkr is not activated");
    Ok(())
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct HashTuple {
    pub hash: [u8; 32],
    pub sibling_index: u8
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AuthenticationObject {
    pub password_hash: [u8; 32],
    pub otp_hash: [u8; 32],
    pub proof_path: Vec<HashTuple>
}
