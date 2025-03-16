use crate::commands::Value;
use crate::commands::{Command, ExecutionMode};

use std::io::Write;
use std::io::{self, Read};
use uuid::Uuid;

pub(crate) type Result<T> = std::result::Result<T, ClientError>;

pub(crate) struct WatchClient {
    pub id: String,
    pub stream: std::net::TcpStream,
}

pub(crate) struct CommandClient {
    pub id: String,
    pub stream: std::net::TcpStream,
}

#[derive(Debug)]
pub enum ClientError {
    ReadError(io::Error),
    DecodeError(prost::DecodeError),
    HandshakeError(Value),
    CommandError(String),
    MissingValue,
}

impl From<io::Error> for ClientError {
    fn from(error: io::Error) -> Self {
        ClientError::ReadError(error)
    }
}
impl From<prost::DecodeError> for ClientError {
    fn from(error: prost::DecodeError) -> Self {
        ClientError::DecodeError(error)
    }
}

pub struct Client {
    host: String,
    port: u16,
    pub(crate) command_client: CommandClient,
}

impl Client {
    pub fn new(host: String, port: u16) -> Result<Self> {
        let mut command_client = Self::new_command_client(host.clone(), port);
        command_client.handshake()?;
        Ok(Client {
            host,
            port,
            command_client,
        })
    }

    fn new_command_client(host: String, port: u16) -> CommandClient {
        let stream = std::net::TcpStream::connect(format!("{}:{}", host, port)).unwrap();
        let id = Uuid::new_v4().to_string();
        CommandClient { stream, id }
    }

    fn new_watch_client(host: String, port: u16) -> WatchClient {
        let stream = std::net::TcpStream::connect(format!("{}:{}", host, port)).unwrap();
        let id = Uuid::new_v4().to_string();

        WatchClient { stream, id }
    }
}

const MAX_REQUEST_SIZE: usize = 32 * 1024 * 1024;
const BUFFER_SIZE: usize = 16 * 1024;

impl CommandClient {
    fn handshake(&mut self) -> Result<()> {
        let handshake = Command::HANDSHAKE {
            client_id: self.id.clone(),
            execution_mode: ExecutionMode::Command,
        };
        let reply = self.request_reply(handshake)?;
        match reply {
            Value::VStr(v) if v == "OK" => Ok(()),
            value => Err(ClientError::HandshakeError(value)),
        }
    }

    pub fn request_reply(&mut self, command: Command) -> Result<Value> {
        let serialized_command = command.encode();
        self.stream.write_all(&serialized_command)?;
        let mut buffer = vec![0; BUFFER_SIZE];
        let size = self.stream.read(&mut buffer)?;
        let reply_slice = &buffer[..size];
        let val = Value::decode_response(reply_slice)?;
        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const HOST: &str = "localhost";
    const PORT: u16 = 7379;

    #[test]
    fn test_client() {
        let d = Client::new(HOST.to_string(), PORT);
        assert!(d.is_ok());
    }
}
