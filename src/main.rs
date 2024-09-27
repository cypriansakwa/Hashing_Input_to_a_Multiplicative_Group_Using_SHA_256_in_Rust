extern crate sha2;
use sha2::{Sha256, Digest};
use num_bigint::BigUint;
use num_traits::Num;

fn hash_to_multiplicative_group(input: &[u8], prime: &BigUint) -> BigUint {
    // Create a SHA-256 object
    let mut hasher = Sha256::new();
    
    // Write input data
    hasher.update(input);
    
    // Read the hash digest (output)
    let hash_result = hasher.finalize();

    // Convert the hash (which is a byte array) into a large integer
    let hash_hex = hex::encode(hash_result);
    let hash_int = BigUint::from_str_radix(&hash_hex, 16).expect("Failed to convert hash to integer");

    // Ensure the result is in the range of [1, prime-1] (for a multiplicative group)
    let group_element = (hash_int % (prime - 1u32)) + 1u32;

    group_element
}

fn main() {
    // Example input
    let input = b"Hello, Multiplicative Group!";
    
    // Prime number for the multiplicative group (a large prime)
    let prime = BigUint::parse_bytes(b"340282366920938463463374607431768211507", 10)
        .expect("Failed to parse prime");

    // Compute the hash to the multiplicative group
    let group_element = hash_to_multiplicative_group(input, &prime);

    // Print the resulting group element
    println!("Hashed to multiplicative group: {}", group_element);
}

