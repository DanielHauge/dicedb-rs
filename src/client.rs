use crate::stream::{Stream, StreamError};
use crate::{
    commandstream::{CommandStream, CommandStreamError},
    watchstream::WatchStreamError,
};

pub struct Client {
    pub(crate) port: u16,
    pub(crate) host: String,
    pub(crate) command_client: CommandStream,
}

#[derive(Debug)]
pub enum ClientError {
    IoError(std::io::Error),
    CommandStreamError(CommandStreamError),
    WatchStreamError(WatchStreamError),
    StreamError(StreamError),
}

impl From<CommandStreamError> for ClientError {
    fn from(error: CommandStreamError) -> Self {
        ClientError::CommandStreamError(error)
    }
}

impl From<std::io::Error> for ClientError {
    fn from(error: std::io::Error) -> Self {
        ClientError::IoError(error)
    }
}

impl From<WatchStreamError> for ClientError {
    fn from(error: WatchStreamError) -> Self {
        ClientError::WatchStreamError(error)
    }
}

impl From<StreamError> for ClientError {
    fn from(error: StreamError) -> Self {
        ClientError::StreamError(error)
    }
}

impl Client {
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
    use super::*;
    const HOST: &str = "localhost";
    const PORT: u16 = 7379;

    #[test]
    fn test_client() {
        let d = Client::new(HOST.to_string(), PORT);
        assert!(d.is_ok());
    }
}
