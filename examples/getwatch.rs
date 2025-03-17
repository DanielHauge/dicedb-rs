use dicedb_rs::{
    self,
    client::{self, Client},
};

fn main() -> Result<(), client::ClientError> {
    // Create a new client
    let mut client = Client::new("localhost".to_string(), 7379).unwrap();

    // Set a key
    client.set("Hello", "World")?;

    // Setup a watch
    let (hello_changes, first_value) = client.get_watch("Hello").unwrap();
    eprintln!("First value was: {:?}", first_value);

    // Listen for changes
    for change in hello_changes {
        eprintln!("There was a change: {:?}", change);
    }

    Ok(())
}
