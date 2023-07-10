extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};
use std::time::Instant;

fn main() {
    let password = "my password";
    dbg!(DEFAULT_COST);
    let cost = 5;
    // Measure time taken to hash a password
    let hash_start_time = Instant::now();
    let hashed_password = hash(password, cost).unwrap();
    let hash_elapsed = hash_start_time.elapsed();
    println!("Hashed password: {}", hashed_password);
    println!("Time taken to hash password: {:?}", hash_elapsed);

    // Measure time taken to verify a password
    let verify_start_time = Instant::now();
    let valid = verify(password, &hashed_password).unwrap();
    let verify_elapsed = verify_start_time.elapsed();
    println!("Password is valid: {}", valid);
    println!("Time taken to verify password: {:?}", verify_elapsed);
}
