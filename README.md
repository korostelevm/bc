# Bcrypt Example

This is a Rust command-line application that demonstrates how to use the `bcrypt` crate to hash and verify passwords.

## Prerequisites

To build and run this application, you need to have Rust and Cargo installed on your system. You can download Rust and Cargo from the official website: https://www.rust-lang.org/tools/install

## Installation

To install the application, clone the repository and run the following command:

```
cargo build --release
```

This will build the application in release mode and create an executable file in the `target/release` directory.

## Usage

To use the application, run the following command:

```
./target/release/bcrypt-example <password> [options]
```

Replace `<password>` with the password you want to hash or verify.

The following options are available:

- `-c, --cost <cost>`: The cost factor to use when hashing the password (default: 10).
- `-v, --verify <hash>`: Verify the password instead of hashing it. Provide the hash to verify as an argument.
- `-d, --debug`: Enable verbose output for debugging.

## Examples

To hash a password with the default cost factor (10), run the following command:

```
./target/release/bcrypt-example mypassword
```

To hash a password with a custom cost factor (12), run the following command:

```
./target/release/bcrypt-example mypassword -c 12
```

To verify a password hash, run the following command:

```
./target/release/bcrypt-example mypassword -v '$2a$10$1....'
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

