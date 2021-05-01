extern crate solana_core;
extern crate solana_sdk;

use solana_core::gen_keys::GenKeys;
use solana_sdk::pubkey::Pubkey;

fn main() {
    loop {
        let mut seed = [0u8; 32];
        for x in seed.iter_mut() {
            *x = rand::random();
        }

        let keypair = GenKeys::new(seed).gen_keypair();
        let keypair_bytes = keypair.to_bytes();

        let pubkey = Pubkey::new(&keypair_bytes[0..32]);
        if &pubkey.to_string()[0..3] == "PSG" {
            println!("{:?}", pubkey.to_string());
            println!("{:?}", keypair.to_base58_string());
            println!("{:?}", keypair_bytes);
            break;
        }
    }
}
