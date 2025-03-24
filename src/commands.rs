//! # Commands Module
//! Contains structures and options related to interact with the server.
//! It contains structures for all the commands, value types and options.

use prost::Message;
use std::fmt::Display;

use crate::errors::{CommandError, StreamError};

mod wire {
    tonic::include_proto!("wire");
}

/// A special input type for the DEL oeration.
#[derive(Debug, Clone, PartialEq)]
pub enum DelInput<'a> {
    /// A single key to delete.
    Single(&'a str),
    /// Multiple keys to delete.
    Multiple(Vec<&'a str>),
}

/// A special input type for the HSET operation.
/// The type is a convenience type that allows users to specify either a single key-value pair or
/// multiple key-value pairs.
#[derive(Debug, Clone, PartialEq)]
pub enum HSetInput<'a> {
    /// A single key-value pair.
    Single(&'a str, &'a str),
    /// Multiple key-value pairs.
    Multiple(Vec<(&'a str, &'a str)>),
}

/// Valid values that can be used with the SET operation.
#[derive(Debug, Clone, PartialEq)]
pub enum SetValue {
    /// A string value.
    Str(String),
    /// An integer value.
    Int(i64),
    /// A floating point value.
    Float(f64),
}

impl Into<Value> for SetValue {
    fn into(self) -> Value {
        match self {
            SetValue::Str(s) => Value::VStr(s),
            SetValue::Int(i) => Value::VInt(i),
            SetValue::Float(f) => Value::VFloat(f),
        }
    }
}

impl TryInto<SetValue> for Value {
    type Error = String;

    fn try_into(self) -> Result<SetValue, Self::Error> {
        match self {
            Value::VStr(s) => Ok(SetValue::Str(s)),
            Value::VInt(i) => Ok(SetValue::Int(i)),
            Value::VFloat(f) => Ok(SetValue::Float(f)),
            Value::VBool(_) => Err("Cannot convert Value::VBool to SetValue".to_string()),
            Value::VNull => Err("Cannot convert Value::VNull to SetValue".to_string()),
        }
    }
}

macro_rules! impl_vint_setvalue_for_int {
    ($($t:ty),*) => {
        $(
            impl From<$t> for SetValue {
                fn from(value: $t) -> Self {
                    SetValue::Int(value as i64)
                }
            }
        )*
    };
}

macro_rules! impl_vint_value_for_int {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Value {
                fn from(value: $t) -> Self {
                    Value::VInt(value as i64)
                }
            }
        )*
    };
}

macro_rules! impl_vint_setvalue_for_float {
    ($($t:ty),*) => {
        $(
            impl From<$t> for SetValue {
                fn from(value: $t) -> Self {
                    SetValue::Float(value as f64)
                }
            }
        )*
    };
}

macro_rules! impl_vint_value_for_float {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Value {
                fn from(value: $t) -> Self {
                    Value::VFloat(value as f64)
                }
            }
        )*
    };
}

impl_vint_setvalue_for_int!(i64, i32, i16, i8, u64, u32, u16, u8);
impl_vint_value_for_int!(i64, i32, i16, i8, u64, u32, u16, u8);
impl_vint_setvalue_for_float!(f64, f32);
impl_vint_value_for_float!(f64, f32);

impl Into<Value> for &str {
    fn into(self) -> Value {
        Value::VStr(self.to_string())
    }
}

impl Into<SetValue> for &str {
    fn into(self) -> SetValue {
        SetValue::Str(self.to_string())
    }
}

/// A value received from the server.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    /// A string value.
    VStr(String),
    /// An integer value.
    VInt(i64),
    /// A floating point value.
    VFloat(f64),
    /// A boolean value.
    VBool(bool),
    /// A null value. A null value is not indicative of failure, but just the absence of a value.
    VNull,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::VStr(s) => write!(f, "{}", s),
            Value::VInt(i) => write!(f, "{}", i),
            Value::VFloat(fl) => write!(f, "{}", fl),
            Value::VBool(b) => write!(f, "{}", b),
            Value::VNull => write!(f, "null"),
        }
    }
}

impl AsArg for Value {
    fn as_arg(&self) -> String {
        match self {
            Value::VStr(s) => s.clone(),
            Value::VInt(i) => i.to_string(),
            Value::VFloat(f) => f.to_string(),
            Value::VBool(b) => b.to_string(),
            Value::VNull => "".to_string(),
        }
    }
}

impl Into<Value> for wire::response::Value {
    fn into(self) -> Value {
        match self {
            wire::response::Value::VNil(_) => Value::VNull,
            wire::response::Value::VInt(i) => Value::VInt(i),
            wire::response::Value::VStr(s) => Value::VStr(s),
            wire::response::Value::VFloat(f) => Value::VFloat(f),
            wire::response::Value::VBytes(b) => {
                Value::VStr(String::from_utf8_lossy(&b).to_string())
            }
        }
    }
}

/// A watch value is a value that originates from a GET.WATCH command.
#[derive(Debug)]
pub struct WatchValue {
    /// The value from the watch session, it indicates a change in a watched key.
    pub value: Value,
    /// The fingerprint of the value, which is a unique identifier for the value.
    pub fingerprint: String,
}

impl Into<Value> for WatchValue {
    fn into(self) -> Value {
        self.value
    }
}

impl WatchValue {
    pub(crate) fn decode_watchvalue(bytes: &[u8]) -> Result<Self, CommandError> {
        match wire::Response::decode(bytes) {
            Ok(v) => {
                if v.err == "" {
                    let fingerprint = match v
                        .attrs
                        .ok_or(CommandError::WatchValueExpectationError(
                            "Missing attributes from response".to_string(),
                        ))?
                        .fields
                        .get("fingerprint")
                        .ok_or(CommandError::WatchValueExpectationError(
                            "Missing fingerprint from attributes".to_string(),
                        ))?
                        .kind
                        .clone()
                        .ok_or(CommandError::WatchValueExpectationError(
                            "Missing kind from fingerprint attribute".to_string(),
                        ))? {
                        prost_types::value::Kind::StringValue(s) => s,
                        _ => {
                            return Err(CommandError::WatchValueExpectationError(
                                "Fingerprint is not a string".to_string(),
                            ))
                        }
                    };
                    let value = v
                        .value
                        .ok_or(CommandError::WatchValueExpectationError(
                            "Missing value from response".to_string(),
                        ))?
                        .into();

                    Ok(WatchValue { value, fingerprint })
                } else {
                    Err(CommandError::ServerError(v.err))
                }
            }
            Err(e) => Err(CommandError::DecodeError(e)),
        }
    }
}

impl Value {
    pub(crate) fn decode_value(bytes: &[u8]) -> Result<Self, CommandError> {
        let decoded = match wire::Response::decode(bytes) {
            Ok(v) => {
                if v.err == "" {
                    match v.value {
                        Some(value) => Ok(value.into()),
                        None => Ok(Value::VNull),
                    }
                } else {
                    Err(CommandError::ServerError(v.err))
                }
            }
            Err(e) => Err(CommandError::DecodeError(e)),
        };
        eprintln!("Decoded value: {:?}", decoded);

        decoded
    }
}

trait AsArg {
    fn as_arg(&self) -> String;
}

trait AsArgs {
    fn as_args(&self) -> Vec<String>;
}

pub(crate) trait CommandExecutor {
    fn execute_command(&mut self, command: Command) -> Result<Value, StreamError>;
}

/// Expire options for the EXPIRE command
#[derive(Debug, Clone, Copy)]
pub enum ExpireOption {
    /// Don't overwrite existing expiration time
    NX,
    /// Only set the expiration time if it already exists
    XX,
    /// Always set the expiration time
    None,
}

impl AsArg for ExpireOption {
    fn as_arg(&self) -> String {
        match self {
            ExpireOption::NX => "NX".to_string(),
            ExpireOption::XX => "XX".to_string(),
            ExpireOption::None => "".to_string(),
        }
    }
}

/// Expire options for the EXPIREAT command
#[derive(Debug, Clone, Copy)]
pub enum ExpireAtOption {
    /// Don't overwrite existing expiration time
    NX,
    /// Only set the expiration time if it already exists
    XX,
    /// Set the expiration time only if it's greater than the existing expiration time
    GT,
    /// Set the expiration time only if it's less than the existing expiration time
    LT,
    /// Always set the expiration time
    None,
}

impl AsArg for ExpireAtOption {
    fn as_arg(&self) -> String {
        match self {
            ExpireAtOption::NX => "NX".to_string(),
            ExpireAtOption::XX => "XX".to_string(),
            ExpireAtOption::GT => "GT".to_string(),
            ExpireAtOption::LT => "LT".to_string(),
            ExpireAtOption::None => "".to_string(),
        }
    }
}

/// Options for the GETEX command
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GetexOption {
    /// Set the expiration to seconds from now.
    EX(u64),
    /// Set the expiration to milliseconds from now.
    PX(u64),
    /// Set the expiration to a Unix timestamp.
    EXAT(u64),
    /// Set the expiration to a Unix timestamp in milliseconds.
    PXAT(u64),
    /// Remove the expiration from the key.
    PERSIST,
}

impl AsArgs for GetexOption {
    fn as_args(&self) -> Vec<String> {
        match self {
            GetexOption::EX(seconds) => vec!["EX".to_string(), seconds.to_string()],
            GetexOption::PX(milliseconds) => vec!["PX".to_string(), milliseconds.to_string()],
            GetexOption::EXAT(timestamp) => vec!["EXAT".to_string(), timestamp.to_string()],
            GetexOption::PXAT(timestamp) => vec!["PXAT".to_string(), timestamp.to_string()],
            GetexOption::PERSIST => vec!["PERSIST".to_string()],
        }
    }
}

/// Options for the SET command
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SetOption {
    /// Set the expiration time in seconds
    EX(u64),
    /// Set the expiration time in milliseconds
    PX(u64),
    /// Set the expiration time in seconds since epoch
    EXAT(u64),
    /// Set the expiration time in milliseconds since epoch
    PXAT(u64),
    /// Only set the key if it already exists
    XX,
    /// Only set the key if it does not already exist
    NX,
    /// Keep the existing TTL of the key
    KEEPTTL,
    /// No special option, default
    None,
}

impl AsArgs for SetOption {
    fn as_args(&self) -> Vec<String> {
        match self {
            SetOption::EX(seconds) => vec!["EX".to_string(), seconds.to_string()],
            SetOption::PX(milliseconds) => vec!["PX".to_string(), milliseconds.to_string()],
            SetOption::EXAT(timestamp) => vec!["EXAT".to_string(), timestamp.to_string()],
            SetOption::PXAT(timestamp) => vec!["PXAT".to_string(), timestamp.to_string()],
            SetOption::XX => vec!["XX".to_string()],
            SetOption::NX => vec!["NX".to_string()],
            SetOption::KEEPTTL => vec!["KEEPTTL".to_string()],
            SetOption::None => vec![],
        }
    }
}

impl AsArg for SetValue {
    fn as_arg(&self) -> String {
        match self {
            SetValue::Str(s) => s.clone(),
            SetValue::Int(i) => i.to_string(),
            SetValue::Float(f) => f.to_string(),
        }
    }
}

impl AsArg for String {
    fn as_arg(&self) -> String {
        self.clone()
    }
}

impl AsArgs for Vec<(String, SetValue)> {
    fn as_args(&self) -> Vec<String> {
        let mut args = vec![];
        for (field, value) in self {
            args.push(field.clone());
            args.push(value.as_arg());
        }
        args
    }
}

#[derive(Debug)]
pub(crate) enum ExecutionMode {
    Command,
    Watch,
}

impl AsArg for ExecutionMode {
    fn as_arg(&self) -> String {
        match self {
            ExecutionMode::Command => "command".to_string(),
            ExecutionMode::Watch => "watch".to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Command {
    DECR {
        key: String,
    },
    DECRBY {
        key: String,
        delta: i64,
    },
    DEL {
        keys: Vec<String>,
    },
    ECHO {
        message: String,
    },
    EXISTS {
        key: String,
        additional_keys: Vec<String>,
    },
    EXPIRE {
        key: String,
        seconds: i64,
        option: ExpireOption,
    },
    EXPIREAT {
        key: String,
        timestamp: i64,
        option: ExpireAtOption,
    },
    EXPIRETIME {
        key: String,
    },
    FLUSHDB,
    GET {
        key: String,
    },
    GETDEL {
        key: String,
    },
    GETEX {
        key: String,
        ex: GetexOption,
    },
    HSET {
        key: String,
        fields: Vec<(String, String)>,
    },
    HGET {
        key: String,
        field: String,
    },
    HGETALL {
        key: String,
    },
    GETWATCH {
        key: String,
    },
    HANDSHAKE {
        client_id: String,
        execution_mode: ExecutionMode,
    },
    INCR {
        key: String,
    },
    INCRBY {
        key: String,
        delta: i64,
    },
    PING,
    SET {
        key: String,
        value: SetValue,
        option: SetOption,
        get: bool,
    },
    TTL {
        key: String,
    },
    TYPE {
        key: String,
    },
    UNWATCH {
        key: String,
    },
}

impl Into<wire::Command> for Command {
    fn into(self) -> wire::Command {
        match self {
            Command::DECR { key } => wire::Command {
                cmd: "DECR".to_string(),
                args: vec![key],
            },
            Command::DECRBY { key, delta } => wire::Command {
                cmd: "DECRBY".to_string(),
                args: vec![key, delta.to_string()],
            },
            Command::DEL { keys } => wire::Command {
                cmd: "DEL".to_string(),
                args: keys,
            },
            Command::ECHO { message } => wire::Command {
                cmd: "ECHO".to_string(),
                args: vec![message],
            },
            Command::EXISTS {
                key,
                additional_keys: keys,
            } => {
                let mut args = vec![key];
                args.extend(keys);
                wire::Command {
                    cmd: "EXISTS".to_string(),
                    args,
                }
            }
            Command::EXPIRE {
                key,
                seconds,
                option,
            } => {
                let mut args = vec![key, seconds.to_string()];
                match option {
                    ExpireOption::NX => args.push("NX".to_string()),
                    ExpireOption::XX => args.push("XX".to_string()),
                    ExpireOption::None => {}
                }
                wire::Command {
                    cmd: "EXPIRE".to_string(),
                    args,
                }
            }
            Command::EXPIREAT {
                key,
                timestamp,
                option,
            } => {
                let mut args = vec![key, timestamp.to_string()];
                match option {
                    ExpireAtOption::None => {}
                    option => args.push(option.as_arg()),
                }
                wire::Command {
                    cmd: "EXPIREAT".to_string(),
                    args,
                }
            }
            Command::EXPIRETIME { key } => wire::Command {
                cmd: "EXPIRETIME".to_string(),
                args: vec![key],
            },
            Command::FLUSHDB => wire::Command {
                cmd: "FLUSHDB".to_string(),
                args: vec![],
            },
            Command::GET { key } => wire::Command {
                cmd: "GET".to_string(),
                args: vec![key],
            },
            Command::GETDEL { key } => wire::Command {
                cmd: "GETDEL".to_string(),
                args: vec![key],
            },
            Command::GETEX { key, ex } => {
                let mut args = vec![key];
                args.extend(ex.as_args());
                wire::Command {
                    cmd: "GETEX".to_string(),
                    args,
                }
            }
            Command::HSET { key, fields } => {
                let mut args = vec![key];
                for (field, value) in fields {
                    args.push(field);
                    args.push(value.as_arg());
                }
                wire::Command {
                    cmd: "HSET".to_string(),
                    args,
                }
            }
            Command::HGET { key, field } => wire::Command {
                cmd: "HGET".to_string(),
                args: vec![key, field],
            },
            Command::HGETALL { key } => wire::Command {
                cmd: "HGETALL".to_string(),
                args: vec![key],
            },
            Command::GETWATCH { key } => wire::Command {
                cmd: "GET.WATCH".to_string(),
                args: vec![key],
            },
            Command::HANDSHAKE {
                client_id,
                execution_mode,
            } => wire::Command {
                cmd: "HANDSHAKE".to_string(),
                args: vec![client_id, execution_mode.as_arg()],
            },
            Command::INCR { key } => wire::Command {
                cmd: "INCR".to_string(),
                args: vec![key],
            },
            Command::INCRBY { key, delta } => wire::Command {
                cmd: "INCRBY".to_string(),
                args: vec![key, delta.to_string()],
            },
            Command::PING => wire::Command {
                cmd: "PING".to_string(),
                args: vec![],
            },
            Command::SET {
                key,
                value,
                option,
                get,
            } => {
                let value: Value = value.into();
                let mut args = vec![key, value.as_arg()];
                args.extend(option.as_args());
                match get {
                    true => args.push("GET".to_string()),
                    false => {}
                }
                wire::Command {
                    cmd: "SET".to_string(),
                    args,
                }
            }
            Command::TTL { key } => wire::Command {
                cmd: "TTL".to_string(),
                args: vec![key],
            },
            Command::TYPE { key } => wire::Command {
                cmd: "TYPE".to_string(),
                args: vec![key],
            },
            Command::UNWATCH { key } => wire::Command {
                cmd: "UNWATCH".to_string(),
                args: vec![key],
            },
        }
    }
}

impl Command {
    pub(crate) fn encode(self) -> Vec<u8> {
        let command: wire::Command = self.into();
        eprintln!("Sending command: {:?}", command);
        command.encode_to_vec()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_try_into() {
        let v: Value = Value::VInt(42);
        let v_setval: SetValue = v.try_into().unwrap();
        assert_eq!(v_setval, SetValue::Int(42));
        let v: Value = Value::VStr("42".to_string());
        let v_setval: SetValue = v.try_into().unwrap();
        assert_eq!(v_setval, SetValue::Str("42".to_string()));
        let v: Value = Value::VFloat(42.0);
        let v_setval: SetValue = v.try_into().unwrap();
        assert_eq!(v_setval, SetValue::Float(42.0));
        let v: Value = Value::VBool(true);
        let v_setval: Result<SetValue, String> = v.try_into();
        assert_eq!(
            v_setval,
            Err("Cannot convert Value::VBool to SetValue".to_string())
        );
        let v: Value = Value::VNull;
        let v_setval: Result<SetValue, String> = v.try_into();
        assert_eq!(
            v_setval,
            Err("Cannot convert Value::VNull to SetValue".to_string())
        );
    }

    #[test]
    fn test_value_can_convert() {
        let v: i64 = 42;
        let v_setval: SetValue = v.into();
        let v_value: Value = v.into();
        assert_eq!(v_setval, SetValue::Int(42));
        assert_eq!(v_value, Value::VInt(42));

        let v_f: f64 = 42.0;
        let v_setval: SetValue = v_f.into();
        let v_value: Value = v_f.into();
        assert_eq!(v_setval, SetValue::Float(42.0));
        assert_eq!(v_value, Value::VFloat(42.0));

        let v_str: &str = "42";
        let v_setval: SetValue = v_str.into();
        let v_value: Value = v_str.into();
        assert_eq!(v_setval, SetValue::Str("42".to_string()));
        assert_eq!(v_value, Value::VStr("42".to_string()));
    }

    #[test]
    fn test_display_for_value() {
        let value = Value::VInt(1);
        assert_eq!(format!("{}", value), "1");
        let value = Value::VStr("test".to_string());
        assert_eq!(format!("{}", value), "test");
        let value = Value::VNull;
        assert_eq!(format!("{}", value), "null");
        let value = Value::VFloat(1.2);
        assert_eq!(format!("{}", value), "1.2");
        let value = Value::VBool(true);
        assert_eq!(format!("{}", value), "true");
    }
}
