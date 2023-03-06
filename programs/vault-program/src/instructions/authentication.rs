use {
    anchor_lang::{prelude::*, solana_program::hash::Hash},
    crate::{constants::*, errors::ErrorCode, states::*}
};




pub fn handler(bunkr: &mut Bunkr, authentication_object: AuthenticationObject) -> Result<()> {
    let root = bunkr.root;
    let current_time = Clock::get()?.unix_timestamp;
    let password_hash = authentication_object.password_hash;
    let otp_hash = authentication_object.otp_hash;
    let proof_path= authentication_object.proof_path;
    validate_time(bunkr.last_accessed, &current_time)?;
    validate_password(bunkr.current_hash, &password_hash)?;
    validate_root(bunkr.init_time, &current_time, root.to_vec(), proof_path, otp_hash)?;
    bunkr.last_accessed = current_time as u32;
    bunkr.current_hash = password_hash.as_slice().try_into().unwrap();
    msg!("Authentication successful");
    Ok(())
}

fn validate_root(init_time: u32, current_time: &i64, root: Vec<u8>, proof_path: Vec<HashTuple>, hash: [u8; 32]) -> Result<()> {
    assert!(root.len() == 32, "Root must be 32 bytes");
    assert!(hash.len() == 32, "Hash must be 32 bytes");
    assert!(proof_path.len() == 20, "Proof path must be 32 hashes");
    for hash_tuple in proof_path.iter() {
        assert!(hash_tuple.hash.len() == 32, "Hash must be 32 bytes");
    }


    let root = Hash::new_from_array(root.try_into().unwrap());
    let hash: Hash = anchor_lang::solana_program::hash::Hash::new_from_array(hash.try_into().unwrap());
    let time_interval = (current_time - init_time as i64) / 30;
    msg!("Time Interval: {}", time_interval.to_string());
    let extended_hashes: [Hash; 2] = [anchor_lang::solana_program::hash::extend_and_hash(&hash, &time_interval.to_string().as_bytes()), anchor_lang::solana_program::hash::extend_and_hash(&hash, &(time_interval - 1).to_string().as_bytes())];
    msg!("Extended Hashes: {:?}", extended_hashes);
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

    Ok(())
}

fn validate_time(last_accessed: u32, current_time: &i64) -> Result<()> {
    assert!(current_time - last_accessed as i64 > 30, "Account has been accessed in the last 30 seconds");
    Ok(())
}

fn validate_password(on_chain_password: [u8; 32], password: &[u8; 32]) -> Result<()> {

    let password_to_check = anchor_lang::solana_program::hash::hash(password.as_slice());
    assert!(on_chain_password == password_to_check.to_bytes(), "Password Mismatch");
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