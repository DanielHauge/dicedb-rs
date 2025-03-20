use dicedb_rs::{client::Client, errors::ClientError};

fn main() -> Result<(), ClientError> {
    // Create a new client
    let mut client = Client::new("localhost".to_string(), 7379).unwrap();

    // Set a key
    client.set("Hello", "World")?;

    // Get a key
    let value = client.get("Hello")?;
    println!("Hello: {}", value);

    Ok(())
}
