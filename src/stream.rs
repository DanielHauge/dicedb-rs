use std::io::{Read, Write};

use crate::{
    commands::{Command, CommandExecutor, ScalarValue, WatchValue},
    errors::StreamError,
};

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

pub trait ScalarValueReceiver {
    fn receive_scalar_value(&mut self) -> Result<ScalarValue, StreamError>;
}

pub trait HsetValueReceiver {
    fn receive_hset_value(&mut self) -> Result<crate::commands::HSetValue, StreamError>;
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

impl<T: Stream> WatchValueReceiver for T {
    fn recieve_watchvalue(&mut self) -> Result<WatchValue, StreamError> {
        let mut buffer = vec![0; MAX_REQUEST_SIZE];
        let size = self.tcp_stream().read(&mut buffer)?;
        let reply_slice = &buffer[..size];
        let val = WatchValue::decode_watchvalue(reply_slice)?;
        Ok(val)
    }
}

impl<T: Stream> ScalarValueReceiver for T {
    fn receive_scalar_value(&mut self) -> Result<ScalarValue, StreamError> {
        let mut buffer = vec![0; MAX_REQUEST_SIZE];
        let size = self.tcp_stream().read(&mut buffer)?;
        let reply_slice = &buffer[..size];
        let val = ScalarValue::decode(reply_slice)?;
        Ok(val)
    }
}

impl<T: Stream> HsetValueReceiver for T {
    fn receive_hset_value(&mut self) -> Result<crate::commands::HSetValue, StreamError> {
        let mut buffer = vec![0; MAX_REQUEST_SIZE];
        let size = self.tcp_stream().read(&mut buffer)?;
        let reply_slice = &buffer[..size];
        let val = crate::commands::HSetValue::decode(reply_slice)?;
        Ok(val)
    }
}

impl<T: Stream> CommandSender for T {
    fn send_command(&mut self, command: Command) -> Result<(), StreamError> {
        eprintln!("Sending command: {:?}", command);
        let serialized_command = command.encode();
        eprintln!("Sending command: {:?}", serialized_command);
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
    fn execute_scalar_command(&mut self, command: Command) -> Result<ScalarValue, StreamError> {
        self.send_command(command)?;
        self.receive_scalar_value()
    }

    fn execute_hset_command(
        &mut self,
        command: Command,
    ) -> Result<crate::commands::HSetValue, StreamError> {
        self.send_command(command)?;
        self.receive_hset_value()
    }
}

#[cfg(test)]
mod tests {

    use crate::commandstream::CommandStream;

    use super::*;

    #[test]
    fn test_reconnect() {
        let mut command_client = CommandStream::new("localhost".to_string(), 7379).unwrap();
        let reconnect_result = command_client.reconnect(10);
        assert!(reconnect_result.is_ok());
    }
}
