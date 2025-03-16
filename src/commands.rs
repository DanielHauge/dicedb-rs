use prost::{DecodeError, Message};

mod wire {
    tonic::include_proto!("wire");
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    VStr(String),
    VInt(i64),
    VFloat(f64),
    VBool(bool),
    VNull,
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

pub(crate) enum ExpireAtOption {
    NX, // Don't overwrite existing expiration time
    XX, // Only set the expiration time if it already exists
    GT, // Set the expiration time only if it's greater than the existing expiration time
    LT, // Set the expiration time only if it's less than the existing expiration time
}

impl AsArg for ExpireAtOption {
    fn as_arg(&self) -> String {
        match self {
            ExpireAtOption::NX => "NX".to_string(),
            ExpireAtOption::XX => "XX".to_string(),
            ExpireAtOption::GT => "GT".to_string(),
            ExpireAtOption::LT => "LT".to_string(),
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

impl AsArg for GetexOption {
    fn as_arg(&self) -> String {
        match self {
            GetexOption::EX(seconds) => format!("EX {seconds}"),
            GetexOption::PX(milliseconds) => format!("PX {milliseconds}"),
            GetexOption::EXAT(timestamp) => format!("EXAT {timestamp}"),
            GetexOption::PXAT(timestamp) => format!("PXAT {timestamp}"),
            GetexOption::PERSIST => "PERSIST".to_string(),
        }
    }
}

pub(crate) enum SetOption {
    EX(u64),
    PX(u64),
    EXAT(u64),
    PXAT(u64),
    XX,
    NX,
    KEEPTTL,
    None,
}

impl AsArg for SetOption {
    fn as_arg(&self) -> String {
        match self {
            SetOption::EX(seconds) => format!("EX {seconds}"),
            SetOption::PX(milliseconds) => format!("PX {milliseconds}"),
            SetOption::EXAT(timestamp) => format!("EXAT {timestamp}"),
            SetOption::PXAT(timestamp) => format!("PXAT {timestamp}"),
            SetOption::XX => "XX".to_string(),
            SetOption::NX => "NX".to_string(),
            SetOption::KEEPTTL => "KEEPTTL".to_string(),
            SetOption::None => "".to_string(),
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
        value: Value,
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
            } => wire::Command {
                cmd: "EXPIRE".to_string(),
                args: vec![key, seconds.to_string(), option.as_arg()],
            },
            Command::EXPIREAT {
                key,
                timestamp,
                option: option_arg,
            } => wire::Command {
                cmd: "EXPIREAT".to_string(),
                args: vec![key, timestamp.to_string(), option_arg.as_arg()],
            },
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
            Command::GETEX { key, ex } => wire::Command {
                cmd: "GETEX".to_string(),
                args: vec![key, ex.as_arg()],
            },
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
            } => wire::Command {
                cmd: "SET".to_string(),
                args: vec![
                    key,
                    value.as_arg(),
                    option.as_arg(),
                    if get {
                        "GET".to_string()
                    } else {
                        "".to_string()
                    },
                ],
            },
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
