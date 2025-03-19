use crate::client::Client;
use crate::commands::Command;
use crate::commands::CommandExecutor;
use crate::commands::ExpireAtOption;
use crate::commands::ExpireOption;
use crate::commands::GetexOption;
use crate::commands::SetOption;
use crate::commands::SetValue;
use crate::commands::Value;
use crate::stream::StreamError;

type Result<T> = std::result::Result<T, StreamError>;

pub enum DelInput<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

impl<'a> Into<DelInput<'a>> for Vec<&'a str> {
    fn into(self) -> DelInput<'a> {
        DelInput::Multiple(self)
    }
}

impl<'a> Into<DelInput<'a>> for &'a str {
    fn into(self) -> DelInput<'a> {
        DelInput::Single(self)
    }
}

impl Client {
    pub fn decr(&mut self, key: &str) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::DECR {
            key: key.to_string(),
        })?;
        Ok(resp)
    }

    pub fn decrby(&mut self, key: &str, delta: i64) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::DECRBY {
            key: key.to_string(),
            delta,
        })?;
        Ok(resp)
    }

    pub fn del<'a, T: Into<DelInput<'a>>>(&mut self, keys: T) -> Result<Value> {
        let del_input: DelInput = keys.into();
        let keys = match del_input {
            DelInput::Single(key) => vec![key].iter().map(|&x| x.to_string()).collect(),
            DelInput::Multiple(keys) => keys.iter().map(|&x| x.to_string()).collect(),
        };
        let resp = self.command_client.execute_command(Command::DEL { keys })?;
        Ok(resp)
    }

    pub fn echo(&mut self, message: &str) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::ECHO {
            message: message.to_string(),
        })?;
        Ok(resp)
    }

    pub fn exists(&mut self, key: &str, additional_keys: Vec<&str>) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::EXISTS {
            key: key.to_string(),
            additional_keys: additional_keys.iter().map(|&x| x.to_string()).collect(),
        })?;
        Ok(resp)
    }

    pub fn expire(&mut self, key: &str, seconds: i64, option: ExpireOption) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::EXPIRE {
            key: key.to_string(),
            seconds,
            option,
        })?;
        Ok(resp)
    }

    pub fn expireat(&mut self, key: &str, timestamp: i64, option: ExpireAtOption) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::EXPIREAT {
            key: key.to_string(),
            timestamp,
            option,
        })?;
        Ok(resp)
    }

    pub fn expiretime(&mut self, key: &str) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::EXPIRETIME {
            key: key.to_string(),
        })?;
        Ok(resp)
    }

    pub fn flushdb(&mut self) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::FLUSHDB)?;
        Ok(resp)
    }

    pub fn get(&mut self, key: &str) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::GET {
            key: key.to_string(),
        })?;
        Ok(resp)
    }

    pub fn getdel(&mut self, key: &str) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::GETDEL {
            key: key.to_string(),
        })?;
        Ok(resp)
    }

    pub fn getex(&mut self, key: &str, option: GetexOption) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::GETEX {
            key: key.to_string(),
            ex: option,
        })?;
        Ok(resp)
    }

    pub fn incr(&mut self, key: &str) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::INCR {
            key: key.to_string(),
        })?;
        Ok(resp)
    }

    pub fn incrby(&mut self, key: &str, delta: i64) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::INCRBY {
            key: key.to_string(),
            delta,
        })?;
        Ok(resp)
    }

    pub fn ping(&mut self) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::PING)?;
        Ok(resp)
    }

    pub fn set<T: Into<SetValue>>(&mut self, key: &str, value: T) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::SET {
            key: key.to_string(),
            value: value.into(),
            option: crate::commands::SetOption::None,
            get: false,
        })?;
        Ok(resp)
    }

    pub fn setex<T: Into<SetValue>>(
        &mut self,
        key: &str,
        value: T,
        option: SetOption,
    ) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::SET {
            key: key.to_string(),
            value: value.into(),
            option,
            get: false,
        })?;
        Ok(resp)
    }

    pub fn ttl(&mut self, key: &str) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::TTL {
            key: key.to_string(),
        })?;
        Ok(resp)
    }

    pub fn dtype(&mut self, key: &str) -> Result<Value> {
        let resp = self.command_client.execute_command(Command::TYPE {
            key: key.to_string(),
        })?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const HOST: &str = "localhost";
    const PORT: u16 = 7379;

    #[test]
    fn test_decr() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_decr";
        let value = SetValue::Int(1);
        client.set(key, value.clone()).unwrap();
        let result = client.decr(key).unwrap();
        assert_eq!(result, Value::VInt(0));
    }

    #[test]
    fn test_decr_min_underflow() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_decr_min";
        let value = SetValue::Int(i64::MIN);
        client.set(key, value.clone()).unwrap();
        let result = client.decr(key).unwrap();
        assert_eq!(result, Value::VInt(i64::MAX));
    }

    #[test]
    fn test_del() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_del";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.del(vec![key]).unwrap();
        assert_eq!(result, Value::VInt(1));

        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, Value::VNull);
    }

    #[test]
    fn test_expire() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_expire";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.expire(key, 1, ExpireOption::None).unwrap();
        assert_eq!(result, Value::VInt(1));

        std::thread::sleep(std::time::Duration::from_secs(2));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, Value::VNull);
    }

    #[test]
    fn test_expire_nx() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_expire_nx";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.expire(key, 1, ExpireOption::NX).unwrap();
        assert_eq!(result, Value::VInt(1));

        let result = client.expire(key, 100, ExpireOption::NX).unwrap();
        assert_eq!(result, Value::VInt(0));

        std::thread::sleep(std::time::Duration::from_secs(2));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, Value::VNull);
    }

    #[test]
    fn test_expire_xx() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_expire_xx";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();

        let result = client.expire(key, 100, ExpireOption::XX).unwrap();
        assert_eq!(result, Value::VInt(0));

        let result = client.expire(key, 100, ExpireOption::None).unwrap();
        assert_eq!(result, Value::VInt(1));

        let result = client.expire(key, 1, ExpireOption::XX).unwrap();
        assert_eq!(result, Value::VInt(1));

        std::thread::sleep(std::time::Duration::from_secs(3));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, Value::VNull);
    }

    #[test]
    fn test_expireat() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_expireat";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 1;

        let result = client
            .expireat(key, timestamp as i64, ExpireAtOption::None)
            .unwrap();
        assert_eq!(result, Value::VInt(1));

        std::thread::sleep(std::time::Duration::from_secs(2));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, Value::VNull);
    }

    #[test]
    fn test_expiretime() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_expiretime";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let expire_result = client.expire(key, 1, ExpireOption::None).unwrap();
        let expire_time = client.expiretime(key).unwrap();
        assert_eq!(expire_result, Value::VInt(1));
        let now_epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 1;
        assert_eq!(expire_time, Value::VInt(now_epoch as i64));
    }

    #[test]
    #[ignore] // We ignore this test, as it will flush the database and cause other tests to fail
    fn test_flushdb() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_flushdb";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.flushdb().unwrap();
        assert_eq!(result, Value::VStr("OK".to_string()));

        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, Value::VNull);
    }

    #[test]
    fn test_get_set() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_get_set";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.get(key).unwrap();
        assert_eq!(result, value.into());
    }

    #[test]
    fn test_ping_pong() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let result = client.ping().unwrap();
        assert_eq!(result, Value::VStr("PONG".to_string()));
    }

    #[test]
    fn test_echo() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let message = "hello";
        let result = client.echo(message).unwrap();
        assert_eq!(result, Value::VStr(message.to_string()));
    }

    #[test]
    fn test_getdel() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_getdel";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.getdel(key).unwrap();
        assert_eq!(result, value.into());

        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, Value::VNull);
    }

    #[test]
    fn test_getex() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_getex";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.getex(key, GetexOption::EX(1)).unwrap();
        assert_eq!(result, value.into());

        std::thread::sleep(std::time::Duration::from_secs(2));

        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, Value::VNull);
    }

    #[test]
    fn test_incr() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_incr";
        let value = SetValue::Int(1);
        client.set(key, value.clone()).unwrap();
        let result = client.incr(key).unwrap();
        assert_eq!(result, Value::VInt(2));
    }

    #[test]
    fn test_incrby() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_incrby";
        let value = SetValue::Int(1);
        client.set(key, value.clone()).unwrap();
        let result = client.incrby(key, 2).unwrap();
        assert_eq!(result, Value::VInt(3));
    }

    #[test]
    fn test_incr_overflow() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_incr_overflow";
        let value = SetValue::Int(i64::MAX);
        client.set(key, value.clone()).unwrap();
        let result = client.incr(key).unwrap();
        assert_eq!(result, Value::VInt(i64::MIN));
    }

    #[test]
    fn test_ttl() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_ttl";
        let value = SetValue::Str("test".to_string());
        let result = client.setex(key, value.clone(), SetOption::EX(1)).unwrap();
        assert_eq!(result, Value::VStr("OK".to_string()));
        let ttl = client.ttl(key).unwrap();
        // This test is susceptible to failing for timing reasons if not given a acceptable range
        let withinacceptable = match ttl {
            Value::VInt(v) if v <= 2 && v >= 0 => true,
            _ => false,
        };
        assert_eq!(withinacceptable, true);
    }

    #[test]
    fn test_type_str() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_type_str";
        let value = SetValue::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.dtype(key).unwrap();
        assert_eq!(result, Value::VStr("string".to_string()));
    }

    #[test]
    fn test_type_int() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_type_int";
        let value = SetValue::Int(1);
        client.set(key, value.clone()).unwrap();
        let result = client.dtype(key).unwrap();
        assert_eq!(result, Value::VStr("int".to_string()));
    }

    #[test]
    fn test_type_null() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_type_null";
        let result = client.dtype(key).unwrap();
        assert_eq!(result, Value::VStr("none".to_string()));
    }

    #[test]
    fn test_type_float() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_type_float";
        let value = SetValue::Float(1.3);
        client.set(key, value.clone()).unwrap();
        let result = client.dtype(key).unwrap();
        assert_eq!(result, Value::VStr("float".to_string()));
    }
}
