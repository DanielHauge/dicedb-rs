#![warn(
    missing_docs,
    missing_copy_implementations,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::restriction,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness,
    clippy::suspicious,
    clippy::restriction,
    clippy::wildcard_imports,
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
    clippy::string_lit_as_bytes,
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::mem_forget,
    clippy::todo,
    clippy::unimplemented,
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    missing_debug_implementations,
    rust_2018_idioms
)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # DiceDB Unofficial Rust SDK
//! DiceDB is an open-source, fast, reactive, in-memory database optimized for modern hardware.
//!
//! [DiceDB](https://dicedb.io/);
//! - is [reactive](https://dicedb.io/get-started/hello-world-reactive/)
//! - is [fast](https://dicedb.io/benchmarks/) and optimized for modern hardware
//! - is [familiar](https://dicedb.io/commands/get/) and easy to use
//! - is [open-source](https://github.com/dicedb/dice)
//!
//! This is a 1.0 feature complete unofficial Rust SDK for DiceDB.
//! To get started, you can create a new client and start interacting with the database with the
//! following example:
//! ```rust
//!
//! use dicedb_rs::{client::Client, errors::ClientError};
//!
//! fn main() -> Result<(), ClientError> {
//!     // Create a new client
//!     let mut client = Client::new("localhost".to_string(), 7379).unwrap();
//!
//!     // Set a key
//!     client.set("Hello", "World")?;
//!
//!     // Get a key
//!     let value = client.get("Hello")?;
//!     println!("Hello: {}", value);
//!
//!     Ok(())
//! }
//! ```
//! This SDK is a work in progress and is not yet stable. Please report any issues you encounter.

pub mod client;
pub(crate) mod commandrpc;
pub mod commands;
pub(crate) mod commandstream;
pub mod errors;
mod stream;
pub(crate) mod watchrpc;
pub mod watchstream;
