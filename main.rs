use solana_program::hash;
use bs58;

fn main () {

    let my_data = String::from("hello");
    let mut my_hash= hash::hash(my_data.as_bytes());
    let mut counter = 0;
    let hash_str = bs58::encode(my_hash).into_string();
    println!("{:?}", hash_str);

    while counter < 10 {
        my_hash = hash::hash(&my_hash.to_bytes());
        counter += 1;
        println!("{}", my_hash)
    }

    println!("{}", my_hash.to_string())
}


