use anchor_lang:: {prelude::*, solana_program::hash};

fn main() {
    let mut otps = vec![];
    for i in 0..math.pow(2,20) {
        let word = hello;
        let hash = hash::hash(word.as_bytes());
        let extended_hash = hash::extend_and_hash(&hash, &i.to_bytes());
        vec.push(extended_hash);
    }
}