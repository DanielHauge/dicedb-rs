use std::io::{self, ErrorKind};

use uuid::Uuid;

use crate::{
    commands::{Command, CommandExecutor, ExecutionMode, ScalarValue},
    errors::{CommandStreamError, StreamError},
    stream::Stream,
};

#[derive(Debug)]
pub(crate) struct CommandStream {
    host: String,
    port: u16,
    pub id: String,
    pub stream: std::net::TcpStream,
}

impl CommandStream {
    pub(crate) fn new(host: String, port: u16) -> Result<Self, CommandStreamError> {
        let stream = std::net::TcpStream::connect(format!("{}:{}", host, port))?;
        let id = Uuid::new_v4().to_string();
        Ok(CommandStream {
            stream,
            id,
            host,
            port,
        })
    }
}

impl Stream for CommandStream {
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
            execution_mode: ExecutionMode::Command,
        };
        let reply = self.execute_scalar_command(handshake)?;
        match reply {
            ScalarValue::VStr(v) if v == "OK" => Ok(()),
            value => Err(StreamError::IoError(io::Error::new(
                ErrorKind::Other,
                format!("Handshake error: {:?}", value),
            ))),
        }
    }
}
