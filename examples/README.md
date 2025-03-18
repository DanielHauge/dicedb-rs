# Examples using the SDK

To build the examples, the [pre-requisites](../README.md#Development) must be fulfilled.

```bash
cargo build --examples
```

The binaries for examples will be in the `./target/debug/examples` directory.
All examples can also be run with `cargo run --example <example_name>`.

## [getset.rs](./getset.rs)

A simple example of how to set and get a key from the database.

```bash
# ./target/debug/examples/getset
cargo run --example getset
```

Outputs:

```bash
Hello: World
my_int: 5
```

## [getwatch.rs](./getwatch.rs)

An example of how to watch for changes in a key.

```bash
# ./target/debug/examples/getwatch
cargo run --example getwatch
```

Outputs:

```bash
First value was: VStr("World")
There was a change: WatchValue { value: VStr("World"), fingerprint: "3975712615" }
```
