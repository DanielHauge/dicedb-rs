use std::io;

use uuid::Uuid;

use crate::{
    commands::{Command, CommandExecutor, ExecutionMode, Value, WatchValue},
    stream::{Stream, StreamError, ValueReceiver, WatchValueReceiver},
};

#[derive(Debug)]
pub enum WatchStreamError {
    IoError(io::Error),
    UnexpectedResponse(Value),
    StreamError(StreamError),
}

impl From<io::Error> for WatchStreamError {
    fn from(error: io::Error) -> Self {
        WatchStreamError::IoError(error)
    }
}

impl From<StreamError> for WatchStreamError {
    fn from(error: StreamError) -> Self {
        WatchStreamError::StreamError(error)
    }
}

pub(crate) struct WatchStream {
    host: String,
    port: u16,
    pub fingerprint: Option<String>,
    pub id: String,
    pub stream: std::net::TcpStream,
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
            Some(f) => _ = self.execute_command(Command::UNWATCH { key: f.to_string() }),
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
        let reply = self.execute_command(handshake)?;
        match reply {
            Value::VStr(v) if v == "OK" => Ok(()),
            value => Err(StreamError::IoError(io::Error::new(
                io::ErrorKind::Other,
                format!("Handshake error: {:?}", value),
            ))),
        }
    }
}
