use dicedb_rs::{
    self,
    client::{self, Client},
    commands::Value,
};

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
