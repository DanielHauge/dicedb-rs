//! # WatchStream Module
//! The watchstream module contains the WatchStream struct and its implementation.
use std::io;

use uuid::Uuid;

use crate::{
    commands::{Command, CommandExecutor, ExecutionMode, ScalarValue, WatchValue},
    errors::{StreamError, WatchStreamError},
    stream::{Stream, WatchValueReceiver},
};

/// WatchStream is a stream that is used to watch for changes in a key.
/// It is build from the [`Client`](crate::client::Client) using the
/// [`get_watch`](crate::client::Client::get_watch) method.
///
/// The stream implements the [`Iterator`] trait
/// and will yield [`WatchValue`] values.
///
/// Therefore to use the stream, you can use it in a for loop like this:
///
/// ```rust
/// use dicedb_rs::client::Client;
/// fn main() -> Result<(), dicedb_rs::errors::ClientError> {
///     let mut client = Client::new("localhost".to_string(), 7379)?;
///     let (watch_stream, first_value) = client.get_watch("key").unwrap();
///     eprintln!("First value: {:?}", first_value);
///     // watch stream is an iterator:
///     // for value in watch_stream {
///        // println!("Value: {:?}", value);
///        // Do something with the value
///        // ...
///    // }
/// Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct WatchStream {
    host: String,
    port: u16,
    pub(crate) fingerprint: Option<String>,
    pub(crate) id: String,
    pub(crate) stream: std::net::TcpStream,
}

impl WatchStream {
    pub(crate) fn new(host: String, port: u16) -> Result<Self, WatchStreamError> {
        let stream = std::net::TcpStream::connect(format!("{}:{}", host, port))?;
        let id = Uuid::new_v4().to_string();
        let fingerprint = None;
        Ok(WatchStream {
            stream,
            id,
            fingerprint,
            host,
            port,
        })
    }
}

impl Drop for WatchStream {
    fn drop(&mut self) {
        match &self.fingerprint {
            Some(f) => _ = self.execute_scalar_command(Command::UNWATCH { key: f.to_string() }),
            None => {}
        }
    }
}

impl Iterator for WatchStream {
    type Item = WatchValue;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.recieve_watchvalue();
        match value {
            Ok(val) => Some(val),
            Err(_) => None,
        }
    }
}

impl Stream for WatchStream {
    fn host(&self) -> &str {
        self.host.as_str()
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn set_stream(&mut self, stream: std::net::TcpStream) {
        self.stream = stream;
    }

    fn tcp_stream(&mut self) -> &std::net::TcpStream {
        &self.stream
    }

    fn handshake(&mut self) -> Result<(), StreamError> {
        let handshake = Command::HANDSHAKE {
            client_id: self.id.clone(),
            execution_mode: ExecutionMode::Watch,
        };
        let reply = self.execute_scalar_command(handshake)?;
        match reply {
            ScalarValue::VStr(v) if v == "OK" => Ok(()),
            value => Err(StreamError::IoError(io::Error::new(
                io::ErrorKind::Other,
                format!("Handshake error: {:?}", value),
            ))),
        }
    }
}
