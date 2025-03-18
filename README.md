<h4 align="center">
  <a href="#getting-started">Getting Started</a>
  ·
  <a href="./examples/">Examples</a>
  ·
  <a href="#">Docs</a>
</h4>

<div align="center"><p>
    <a href="https://github.com/DanielHauge/dicedb-rs/actions/workflows/rust.yml">
        <img alt="Tests" src="https://img.shields.io/github/actions/workflow/status/DanielHauge/dicedb-rs/rust.yml?style=flat-square">
    </a>
    <a href="https://crates.io/crates/dicedb-rs">
      <img alt="Crate.io" src="https://img.shields.io/crates/v/dicedb-rs.svg?style=flat-square" />
    </a>
    <a href="https://dicedb.io">
        <img alt="DiceDB" src="https://img.shields.io/badge/site-dicedb.io-00A1FF?style=flat-square" />
    </a>
    <a href="https://discord.gg/6r8uXWtXh7">
        <img alt="Discord" src="https://dcbadge.limes.pink/api/server/6r8uXWtXh7?style=flat" />
    </a>
    <a href="LICENSE">
        <img alt="License" src="https://img.shields.io/badge/license-BSD--3--Clause-blue.svg">
    </a>
</div>

# DiceDB Unofficial Rust SDK

DiceDB is an open-source, fast, reactive, in-memory database optimized for modern hardware.
The source code for DiceDB can be found in the [DiceDB Github](https://github.com/DiceDB/dice) repository.

> [!WARNING]
> This SDK project is under active development without any stable API yet. A base implementation is made to support all operations of DiceDB.

## Getting Started

DiceDB must be running to use the SDK, read more at the [DiceDB Getting Started](https://github.com/dicedb/dice?tab=readme-ov-file#get-started).

A local container can be started with:

```sh
docker run -p 7379:7379 dicedb/dicedb
```

Add the SDK as depedency to your Rust project.

```sh
cargo add dicedb-rs
```

A Simple examples of how to use the sdk:

```rust
fn main() -> Result<(), client::ClientError> {
    // Create a new client
    let mut client = Client::new("localhost".to_string(), 7379)?;

    // Set a key
    client.set("Hello", "World")?;

    // Get a key
    let value = client.get("Hello")?;
    println!("Hello: {}", value);

    // Subscribe to changes in the Hello key
    let (hello_changes, _) = client.get_watch("Hello")?;

    // Listen for changes
    for change in hello_changes {
        eprintln!("There was a change: {:?}", change);
    }

    Ok(())
}
```

More examples of programs using the SDK can be found in the [examples](./examples).

## Development

To contribute and develop the SDK, the following pre-requisites are needed to build and test the project:

- Install protobuf compiler - [Protoc](https://grpc.io/docs/protoc-installation/)
- Install Rust - [Rustup](https://www.rust-lang.org/tools/install)

Clone the repository and remember to checkout the protos submodule.

Build with:

```bash
cargo build
```

Run tests with:

```bash
cargo test
```
