//! # Error Module
//! This module contains the error types for the client and the server.
//! The error types are used to handle errors that occur during the execution of the client and
//! server.
use crate::commands::Value;
use prost::DecodeError;
use std::io::Error;

/// The errors that originates from handling commands.
#[derive(Debug)]
pub enum CommandError {
    /// A server side error occured. This might be caused by a bug in the SDK, or uninteded usage.
    ServerError(String),
    /// The server returned an unexpected response, this can be caused by running on an
    /// incompatible server version.
    DecodeError(DecodeError),
    /// The server returned an unexpected watch response, this can be caused by running on an
    /// incompatible server version.
    WatchValueExpectationError(String),
}

/// The errors that originates from the command stream.
#[derive(Debug)]
pub enum CommandStreamError {
    /// An error occured while reading from the stream. This is caused by the underlying IO to the
    /// server. Connection to server could be lost, or the server could have closed the connection.
    ReadError(Error),
    /// An error occured while decoding the response from the server. This can be caused by an
    /// incompatible server version.
    DecodeError(prost::DecodeError),
    /// An unexpected value was received from the server during handshake. This can be caused by
    /// incompatible server version.
    HandshakeError(Value),
    /// An error occured in the command stream, this can be caused by an unexpected response from
    /// the server.
    CommandError(String),
}

impl From<Error> for CommandStreamError {
    fn from(error: Error) -> Self {
        CommandStreamError::ReadError(error)
    }
}
impl From<prost::DecodeError> for CommandStreamError {
    fn from(error: prost::DecodeError) -> Self {
        CommandStreamError::DecodeError(error)
    }
}

/// The errors that originates from the Client.
#[derive(Debug)]
pub enum ClientError {
    /// An error occured with the command stream
    CommandStreamError(CommandStreamError),
    /// An error occured with the watch stream
    WatchStreamError(WatchStreamError),
    /// An error occured in the clients stream
    StreamError(StreamError),
}

impl From<CommandStreamError> for ClientError {
    fn from(error: CommandStreamError) -> Self {
        ClientError::CommandStreamError(error)
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

/// The errors that originates from base functionality of a stream, either command stream or watch
/// stream.
#[derive(Debug)]
pub enum StreamError {
    /// An error occured with the IO, this could be caused by the underlying IO to the server.
    /// Connection to server could be lost, or the server could have closed the connection.
    IoError(Error),
    /// An error occured while decoding the response from the server. This can be caused by an
    /// incompatible server version.
    DecodeError(DecodeError),
    /// An error occured while handling a command.
    /// This can be caused by an unexpected response from the server.
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

/// The errors that originates from the watch stream.
#[derive(Debug)]
pub enum WatchStreamError {
    /// An error occured with the IO, this could be caused by the underlying IO to the server.
    /// Connection to server could be lost, or the server could have closed the connection.
    IoError(Error),
    /// An error occured while decoding the response from the server. This can be caused by an
    /// incompatible server version.
    UnexpectedResponse(Value),
    /// An error occured while handling a command.
    StreamError(StreamError),
}

impl From<Error> for WatchStreamError {
    fn from(error: Error) -> Self {
        WatchStreamError::IoError(error)
    }
}

impl From<StreamError> for WatchStreamError {
    fn from(error: StreamError) -> Self {
        WatchStreamError::StreamError(error)
    }
}
