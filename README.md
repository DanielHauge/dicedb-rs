# dicedb-rs

Unofficial Rust SDK for DiceDB.
This project is a work in progress, but support all operations of DiceDB.

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

    // Subscribe to changes in the Hello key
    let (hello_changes, _) = client.get_watch("Hello").unwrap();

    // Listen for changes
    for change in hello_changes {
        eprintln!("There was a change: {:?}", change);
    }

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
