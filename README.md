# bcrypt-example

This is a Rust command-line application that demonstrates how to use the `bcrypt` library to hash and verify passwords. It uses the `clap` library for command-line argument parsing and the `env_logger` library for logging.

## Usage

To hash a password, run the following command:

```
cargo run --password <password>
```

To verify a password, run the following command:

```
cargo run --verify <hashed_password> --password <password>
```

The `--verify` flag specifies that the password should be verified instead of hashed. The `--password` flag specifies the password to hash or verify. The `--cost` flag specifies the cost factor to use when hashing the password (default is 10). The `--debug` flag enables verbose output.

## Example

To hash the password "password123" with a cost factor of 12, run the following command:

```
cargo run --password password123 --cost 12
```

This will output the hashed password.

To verify the password "password123" against the hashed password "$2a$12$5zVJ9jJzQz5zQJ7VzZJv2uZz9zjL6jJzjzJzjzJzjzjzjzjzjzjz", run the following command:

```
cargo run --verify '$2a$12$5zVJ9jJzQz5zQJ7VzZJv2uZz9zjL6jJzjzJzjzJzjzjzjzjzjzjz' --password password123
```

This will output whether the password is valid or not.

## Dependencies

This project depends on the following Rust libraries:

- `bcrypt` for password hashing and verification
- `clap` for command-line argument parsing
- `env_logger` for logging

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

