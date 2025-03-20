//! # Client Module
//! The client module contains the main client struct and its implementation.
//! The SDK is centered around the `Client` struct, which is used to interact with the DiceDB
//! server.
use crate::commandstream::CommandStream;
use crate::errors::ClientError;
use crate::stream::Stream;

/// The main client struct used to interact with the DiceDB server.
/// Create a new client with `Client::new(host: String, port: u16)`.
#[derive(Debug)]
pub struct Client {
    pub(crate) port: u16,
    pub(crate) host: String,
    pub(crate) command_client: CommandStream,
}

impl Client {
    /// Create a new client with the given host and port.
    /// # Example
    /// ```
    /// use dice_db::client::Client;
    /// use dice_db::errors::ClientError;
    /// fn main() -> Result<(), ClientError> {
    ///    // Create a new client
    ///    let mut client = Client::new("localhost".to_string(), 7379)?;
    ///    Ok(())
    /// }
    /// ```
    /// # Errors
    /// Returns a [`ClientError`] if the connection to the server fails.
    pub fn new(host: String, port: u16) -> Result<Self, ClientError> {
        let mut command_client = CommandStream::new(host.clone(), port)?;
        command_client.handshake()?;
        Ok(Client {
            command_client,
            host,
            port,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::watchstream::WatchStream;

    use super::*;
    const HOST: &str = "localhost";
    const PORT: u16 = 7379;

    #[test]
    fn test_client() {
        let d = Client::new(HOST.to_string(), PORT);
        assert!(d.is_ok());
    }

    #[test]
    fn test_client_error() {
        let d = Client::new(HOST.to_string(), 0); // invalid port
        assert!(d.is_err());
    }

    #[test]
    fn test_client_error2() {
        let wc = WatchStream::new(HOST.to_string(), 0); // invalid port
        assert!(wc.is_err());
    }
}
