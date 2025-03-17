use std::io::{Error, Read, Write};

use prost::DecodeError;

use crate::commands::{Command, CommandError, CommandExecutor, Value, WatchValue};

pub trait Stream {
    fn host(&self) -> &str;
    fn port(&self) -> u16;
    fn set_stream(&mut self, stream: std::net::TcpStream);
    fn tcp_stream(&mut self) -> &std::net::TcpStream;
    fn handshake(&mut self) -> Result<(), StreamError>;
}

pub trait Reconnectable {
    fn reconnect(&mut self, max_tries: u64) -> Result<(), StreamError>;
}

pub trait ValueReceiver {
    fn receive_value(&mut self) -> Result<Value, StreamError>;
}

pub trait WatchValueReceiver {
    fn recieve_watchvalue(&mut self) -> Result<WatchValue, StreamError>;
}

pub trait CommandSender {
    fn send_command(&mut self, command: Command) -> Result<(), StreamError>;
}

impl<T: Stream> Reconnectable for T {
    fn reconnect(&mut self, max_tries: u64) -> Result<(), StreamError> {
        let mut tries = 0;
        while tries < max_tries {
            tries += 1;
            let stream = std::net::TcpStream::connect(format!("{}:{}", self.host(), self.port()));
            match stream {
                Ok(stream) => {
                    self.set_stream(stream);
                    self.handshake()?;
                    return Ok(());
                }
                Err(_) => {
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    continue;
                }
            }
        }
        Err(StreamError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Max attempts reached",
        )))
    }
}

const MAX_REQUEST_SIZE: usize = 32 * 1024 * 1024; // 32MB per session, meeh probably too much, fi.

#[derive(Debug)]
pub enum StreamError {
    IoError(Error),
    DecodeError(DecodeError),
    CommandError(CommandError),
}

impl From<Error> for StreamError {
    fn from(error: Error) -> Self {
        StreamError::IoError(error)
    }
}

impl From<DecodeError> for StreamError {
    fn from(error: DecodeError) -> Self {
        StreamError::DecodeError(error)
    }
}
impl From<CommandError> for StreamError {
    fn from(error: CommandError) -> Self {
        StreamError::CommandError(error)
    }
}

impl<T: Stream> WatchValueReceiver for T {
    fn recieve_watchvalue(&mut self) -> Result<WatchValue, StreamError> {
        let mut buffer = vec![0; MAX_REQUEST_SIZE];
        let size = self.tcp_stream().read(&mut buffer)?;
        let reply_slice = &buffer[..size];
        let val = WatchValue::decode_watchvalue(reply_slice)?;
        Ok(val)
    }
}

impl<T: Stream> ValueReceiver for T {
    fn receive_value(&mut self) -> Result<Value, StreamError> {
        let mut buffer = vec![0; MAX_REQUEST_SIZE];
        let size = self.tcp_stream().read(&mut buffer)?;
        let reply_slice = &buffer[..size];
        let val = Value::decode_value(reply_slice)?;
        Ok(val)
    }
}

impl<T: Stream> CommandSender for T {
    fn send_command(&mut self, command: Command) -> Result<(), StreamError> {
        eprint!("Sending command: {:?} -> ", command);
        let serialized_command = command.encode();
        match self.tcp_stream().write_all(&serialized_command) {
            Ok(_) => Ok(()),
            Err(_) => {
                self.reconnect(10)?;
                self.tcp_stream().write_all(&serialized_command)?;
                Ok(())
            }
        }
    }
}

impl<T: Stream> CommandExecutor for T {
    fn execute_command(&mut self, command: Command) -> Result<Value, StreamError> {
        self.send_command(command)?;
        self.receive_value()
    }
}
