# dicedb-rs

Unofficial Rust SDK for DiceDB.

This project is a work in progress, and are missing ```GET.WATCH``` and ```UNWATCH``` features, quality of life features, a crate deploy and examples.

## Usage

Add as a dependency in your Rust project.

```sh
cargo add dicedb-rs
```

### Example usage

Some initial simple examples of how to use the sdk.

```rust
fn main() -> Result<(), client::ClientError> {
    // Create a new client
    let mut client = Client::new("localhost".to_string(), 7379).unwrap();

    // Set a key
    client.set("Hello", "World")?;

    // Get a key
    let value = client.get("Hello")?;
    println!("Hello: {}", value);

    // set a key
    client.set("my_int", 1)?;

    // Increment a key
    client.incrby("my_int", 5)?;

    // Decrement a key
    client.decr("my_int")?;

    // Get an int
    let int_value = client.get("my_int")?;
    match int_value {
        Value::VInt(int_value) => println!("my_int: {}", int_value),
        _ => println!("my_int is not an int? oh nouh!, someone changed my int!"),
    }

    // Delete a key
    client.del(vec!["my_int", "Hello"])?;

    Ok(())
}
```

## Build

Pre-requisites:

- Install the protobuf compiler (protoc) - ex. [Protobuf](https://archlinux.org/packages/extra/x86_64/protobuf/) for arch.
- [Rustup](https://www.rust-lang.org/tools/install)

```bash
cargo build
```

## Test

```bash
cargo test
```
