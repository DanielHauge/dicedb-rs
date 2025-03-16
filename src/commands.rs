use std::fmt::Display;

use prost::{DecodeError, Message};

mod wire {
    tonic::include_proto!("wire");
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetValue {
    Str(String),
    Int(i64),
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

impl Into<SetValue> for &str {
    fn into(self) -> SetValue {
        SetValue::Str(self.to_string())
    }
}

impl Into<SetValue> for String {
    fn into(self) -> SetValue {
        SetValue::Str(self)
    }
}

impl Into<SetValue> for i64 {
    fn into(self) -> SetValue {
        SetValue::Int(self)
    }
}

impl Into<SetValue> for f64 {
    fn into(self) -> SetValue {
        SetValue::Float(self)
    }
}

impl Into<SetValue> for i32 {
    fn into(self) -> SetValue {
        SetValue::Int(self as i64)
    }
}

impl Into<SetValue> for i16 {
    fn into(self) -> SetValue {
        SetValue::Int(self as i64)
    }
}

impl Into<SetValue> for i8 {
    fn into(self) -> SetValue {
        SetValue::Int(self as i64)
    }
}

impl Into<SetValue> for u64 {
    fn into(self) -> SetValue {
        SetValue::Int(self as i64)
    }
}

impl Into<SetValue> for u32 {
    fn into(self) -> SetValue {
        SetValue::Int(self as i64)
    }
}

impl Into<SetValue> for u16 {
    fn into(self) -> SetValue {
        SetValue::Int(self as i64)
    }
}

impl Into<SetValue> for u8 {
    fn into(self) -> SetValue {
        SetValue::Int(self as i64)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    VStr(String),
    VInt(i64),
    VFloat(f64),
    VBool(bool),
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

impl Value {
    pub fn decode_response(bytes: &[u8]) -> Result<Value, DecodeError> {
        match wire::Response::decode(bytes) {
            Ok(v) => match v.value {
                Some(value) => Ok(value.into()),
                None => Ok(Value::VNull),
            },
            Err(e) => Err(e),
        }
    }
}

trait AsArg {
    fn as_arg(&self) -> String;
}

trait AsArgs {
    fn as_args(&self) -> Vec<String>;
}

pub enum ExpireOption {
    NX, // Don't overwrite existing expiration time
    XX, // Only set the expiration time if it already exists
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

pub enum ExpireAtOption {
    NX, // Don't overwrite existing expiration time
    XX, // Only set the expiration time if it already exists
    GT, // Set the expiration time only if it's greater than the existing expiration time
    LT, // Set the expiration time only if it's less than the existing expiration time
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

pub enum GetexOption {
    EX(u64),
    PX(u64),
    EXAT(u64),
    PXAT(u64),
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

pub enum SetOption {
    EX(u64),
    PX(u64),
    EXAT(u64),
    PXAT(u64),
    XX,
    NX,
    KEEPTTL,
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
            Command::GETWATCH { key } => wire::Command {
                cmd: "GETWATCH".to_string(),
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
        command.encode_to_vec()
    }
}
