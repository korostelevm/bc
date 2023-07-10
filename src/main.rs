use std::time::Instant;
use bcrypt::{hash, verify};
use clap::{App, Arg};
use log::{LevelFilter, debug, error,};


fn main() {


    let matches = App::new("bc")
        .arg(Arg::with_name("password")
            .required(true)
            .index(1)
            .help("The password to hash or verify"))
        .arg(Arg::with_name("cost")
            .short('c')
            .long("cost")
            .takes_value(true)
            .default_value("10")
            .help("The cost factor to use when hashing the password"))
        .arg(Arg::with_name("verify")
            .short('v')
            .long("verify")
            .takes_value(true)
            .help("Verify the password instead of hashing it ex.: -v '$2a$10$1....'"))
        .arg(Arg::with_name("debug")
            .short('d')
            .long("debug")
            .help("Verbose output"))
        .get_matches();

    let password = matches.value_of("password").unwrap();
    let debug = matches.is_present("debug");
    let level = if debug { LevelFilter::Debug } else { LevelFilter::Info };
    env_logger::builder()
    .filter_level(level)
    .format_timestamp(None)
    .init();


    let verify_password = matches.is_present("verify");
    let hashed_password = matches.value_of("verify");
    let cost:u32 = matches.value_of("cost").unwrap_or("10").parse().unwrap_or(10);


    if verify_password {
        let hashed_password = hashed_password.unwrap_or("");
        dbg!(hashed_password);
        let verify_start_time = Instant::now();
        let valid = verify(password, hashed_password).unwrap_or_else(|error| {
            error!("Error verifying password: {}", error);
            error!("Invalid hash provided. Provided a valid hash with flag '-v' : -v '$2a$1....'");
            false
        });
        // let valid = verify(password, hashed_password).unwrap_or(false);
        let verify_elapsed = verify_start_time.elapsed();
        debug!("Time taken to verify password: {:?}", verify_elapsed);
        println!("Password is valid: {}", valid);
    } else {
        // Measure time taken to hash a password
        let hash_start_time = Instant::now();
        let hashed_password = hash(password, cost).unwrap();
        let hash_elapsed = hash_start_time.elapsed();
        debug!("Time taken to hash password: {:?}", hash_elapsed);
        print!("{}", hashed_password);
    }
}