use crate::client::Client;
use crate::commands::Command;
use crate::commands::CommandExecutor;
use crate::commands::DelInput;
use crate::commands::ExpireAtOption;
use crate::commands::ExpireOption;
use crate::commands::GetexOption;
use crate::commands::HSetInput;
use crate::commands::HSetValue;
use crate::commands::ScalarValue;
use crate::commands::SetInput;
use crate::commands::SetOption;
use crate::errors::StreamError;

type Result<T> = std::result::Result<T, StreamError>;

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

impl<'a> Into<HSetInput<'a>> for (&'a str, &'a str) {
    fn into(self) -> HSetInput<'a> {
        HSetInput::Single(self.0, self.1)
    }
}

impl<'a> Into<HSetInput<'a>> for Vec<(&'a str, &'a str)> {
    fn into(self) -> HSetInput<'a> {
        HSetInput::Multiple(self)
    }
}

impl Client {
    /// Decrements the integer at `key` by one. Creates `key` as -1 if absent. Errors on wrong type
    /// or non-integer string. Limited to 64-bit signed integers.
    ///
    /// # Arguments
    /// * `key` - The key to decrement.
    /// # Returns
    /// * [`Value`] - The new value of `key`.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn decr(&mut self, key: &str) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::DECR {
            key: key.to_string(),
        })?;
        Ok(resp)
    }
    // DECRBY command decrements the integer at ‘key’ by the delta specified. Creates ‘key’ with value (-delta) if absent. Errors on wrong type or non-integer string. Limited to 64-bit signed integers.
    /// Decrements the integer at `key` by `delta`. Creates `key` as `-delta` if absent. Errors on
    /// wrong type
    /// or non-integer string. Limited to 64-bit signed integers.
    /// # Arguments
    /// * `key` - The key to decrement.
    /// * `delta` - The amount to decrement by.
    /// # Returns
    /// * [`Value`] - The new value of `key`.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn decrby(&mut self, key: &str, delta: i64) -> Result<ScalarValue> {
        let resp = self
            .command_client
            .execute_scalar_command(Command::DECRBY {
                key: key.to_string(),
                delta,
            })?;
        Ok(resp)
    }

    // DEL command deletes all the specified keys and returns the number of keys deleted on success. &
    /// Deletes all the specified keys and returns the number of keys deleted on success.
    /// # Arguments
    /// * `keys` - The keys to delete, either a single key or multiple keys.
    /// # Returns
    /// * [`Value`] - The number of keys deleted.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn del<'a, T: Into<DelInput<'a>>>(&mut self, keys: T) -> Result<ScalarValue> {
        let del_input: DelInput<'_> = keys.into();
        let keys = match del_input {
            DelInput::Single(key) => vec![key].iter().map(|&x| x.to_string()).collect(),
            DelInput::Multiple(keys) => keys.iter().map(|&x| x.to_string()).collect(),
        };
        let resp = self
            .command_client
            .execute_scalar_command(Command::DEL { keys })?;
        Ok(resp)
    }

    /// Echos a message with the server, ie. returns the message passed to it.
    /// # Arguments
    /// * `message` - The message to return.
    /// # Returns
    /// * [`Value`] - The message.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn echo(&mut self, message: &str) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::ECHO {
            message: message.to_string(),
        })?;
        Ok(resp)
    }

    /// Checks if the specified keys exist.
    /// # Arguments
    /// * `key` - The key to check.
    /// * `additional_keys` - Additional keys to check. If empty, only `key` is checked.
    /// # Returns
    /// * [`Value`] - The number of keys that exist.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn exists(&mut self, key: &str, additional_keys: Vec<&str>) -> Result<ScalarValue> {
        let resp = self
            .command_client
            .execute_scalar_command(Command::EXISTS {
                key: key.to_string(),
                additional_keys: additional_keys.iter().map(|&x| x.to_string()).collect(),
            })?;
        Ok(resp)
    }
    // EXPIRE sets an expiry (in seconds) on a specified key. After the expiry time has elapsed, the key will be automatically deleted.
    //
    //     If you want to delete the expirtation time on the key, you can use the PERSIST command.
    //
    // The command returns 1 if the expiry was set, and 0 if the key already had an expiry set. The command supports the following options:
    //
    //     NX: Set the expiration only if the key does not already have an expiration time.
    //     XX: Set the expiration only if the key already has an expiration time.
    //
    /// Sets an expiry (in seconds) on a specified key. After the expiry time has elapsed, the key
    /// will be automatically deleted.
    /// # Arguments
    /// * `key` - The key to set the expiry on.
    /// * `seconds` - The number of seconds until the key expires.
    /// * `option`: [`ExpireOption`] - The option to specify conditions for setting the expiry.
    /// # Returns
    /// * [`Value`] - 1 if the expiry was set, 0 if expire was not set.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn expire(&mut self, key: &str, seconds: i64, option: ExpireOption) -> Result<ScalarValue> {
        let resp = self
            .command_client
            .execute_scalar_command(Command::EXPIRE {
                key: key.to_string(),
                seconds,
                option,
            })?;
        Ok(resp)
    }

    /// Sets the expiration time of a key as an absolute Unix timestamp (in seconds). After the
    /// expiry
    /// time has elapsed, the key will be automatically deleted.
    /// # Arguments
    /// * `key` - The key to set the expiry on.
    /// * `timestamp` - The Unix timestamp in seconds.
    /// * `option`: [`ExpireAtOption`] - The option to specify conditions for setting the expiry.
    /// # Returns
    /// * [`Value`] - 1 if the expiry was set or updated, 0 if the expiration time was not changed.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn expireat(
        &mut self,
        key: &str,
        timestamp: i64,
        option: ExpireAtOption,
    ) -> Result<ScalarValue> {
        let resp = self
            .command_client
            .execute_scalar_command(Command::EXPIREAT {
                key: key.to_string(),
                timestamp,
                option,
            })?;
        Ok(resp)
    }

    /// Returns the absolute Unix timestamp in seconds at which the given key will expire.
    /// # Arguments
    /// * `key` - The key to get the expiry time of.
    /// # Returns
    /// * [`Value`] - The Unix timestamp in seconds.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn expiretime(&mut self, key: &str) -> Result<ScalarValue> {
        let resp = self
            .command_client
            .execute_scalar_command(Command::EXPIRETIME {
                key: key.to_string(),
            })?;
        Ok(resp)
    }

    /// Deletes all keys present in the database.
    pub fn flushdb(&mut self) -> Result<ScalarValue> {
        let resp = self
            .command_client
            .execute_scalar_command(Command::FLUSHDB)?;
        Ok(resp)
    }
    // GET returns the value for the key in args.
    //
    // The command returns (nil) if the key does not exist.
    /// Returns the value for the given key.
    /// # Arguments
    /// * `key` - The key to get the value of.
    /// # Returns
    /// * [`Value`] - The value of the key. Returns a valid  [`Value::VNull`] variant if the key does not exist.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn get(&mut self, key: &str) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::GET {
            key: key.to_string(),
        })?;
        Ok(resp)
    }
    /// Returns the value for the given key and then deletes the key.
    /// # Arguments
    /// * `key` - The key to get the value of and delete.
    /// # Returns
    /// * [`Value`] - The value of the key. Returns a valid  [`Value::VNull`] variant if the key
    /// does not exist.
    pub fn getdel(&mut self, key: &str) -> Result<ScalarValue> {
        let resp = self
            .command_client
            .execute_scalar_command(Command::GETDEL {
                key: key.to_string(),
            })?;
        Ok(resp)
    }

    /// Returns the value for the given key and optionally sets its expiration.
    /// # Arguments
    /// * `key` - The key to get the value of.
    /// * `option`: [`GetexOption`] - The option to specify conditions for setting the expiry.
    /// # Returns
    /// * [`Value`] - The value of the key. Returns a valid  [`Value::VNull`] variant if the key
    /// does not exist.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn getex(&mut self, key: &str, option: GetexOption) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::GETEX {
            key: key.to_string(),
            ex: option,
        })?;
        Ok(resp)
    }
    /// Increments the integer at `key` by one. Creates `key` as 1 if absent.    
    /// /// # Arguments
    /// * `key` - The key to increment.
    /// # Returns
    /// * [`Value`] - The new value of `key`.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream, or if the key is not
    /// an integer.
    pub fn incr(&mut self, key: &str) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::INCR {
            key: key.to_string(),
        })?;
        Ok(resp)
    }
    /// Increments the integer at `key` by `delta`. Creates `key` as `delta` if absent.
    /// # Arguments
    /// * `key` - The key to increment.
    /// * `delta` - The amount to increment by.
    /// # Returns
    /// * [`Value`] - The new value of `key`, or an error if the key is not an integer.
    pub fn incrby(&mut self, key: &str, delta: i64) -> Result<ScalarValue> {
        let resp = self
            .command_client
            .execute_scalar_command(Command::INCRBY {
                key: key.to_string(),
                delta,
            })?;
        Ok(resp)
    }
    /// Returns PONG if no argument is provided, otherwise it returns PONG with the message
    /// argument.
    /// # Returns
    /// * [`Value`] - The response from the server, with PONG if no argument is provided.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn ping(&mut self) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::PING)?;
        Ok(resp)
    }
    /// Sets the value of a key.
    /// # Arguments
    /// * `key` - The key to set the value of.
    /// * `value` - The value to set.
    /// # Returns
    /// * [`Value`] - A response from the server with an OK if succes.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn set<T: Into<SetInput>>(&mut self, key: &str, value: T) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::SET {
            key: key.to_string(),
            value: value.into(),
            option: crate::commands::SetOption::None,
            get: false,
        })?;
        Ok(resp)
    }

    /// Sets the value of a key and returns the previous value.
    /// # Arguments
    /// * `key` - The key to set the value of.
    /// * `value` - The value to set.
    /// # Returns
    /// * [`Value`] - The previous value of the key.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn setget<T: Into<SetInput>>(&mut self, key: &str, value: T) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::SET {
            key: key.to_string(),
            value: value.into(),
            option: crate::commands::SetOption::None,
            get: true,
        })?;
        Ok(resp)
    }

    /// Sets the value of a field in a set for a key.
    /// Yields a OK result if operation went okay, and an integer value for number of fields
    /// updated.
    ///
    /// # Arguments
    /// * `key` - The key to set the value of.
    /// * `fields` - The fields to set.
    /// # Returns
    /// * [`Value`] - A response from the server with an OK if succes and the number of updated
    /// fields.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn hset<'a, T: Into<HSetInput<'a>>>(
        &mut self,
        key: &str,
        fields: T,
    ) -> Result<ScalarValue> {
        let hset_input: HSetInput<'_> = fields.into();
        let fields: Vec<(String, String)> = match hset_input {
            HSetInput::Single(field, value) => vec![(field.to_string(), value.to_owned())],
            HSetInput::Multiple(fields) => fields
                .iter()
                .map(|(f, v)| (f.to_string(), v.to_string()))
                .collect(),
        };
        let resp = self.command_client.execute_scalar_command(Command::HSET {
            key: key.to_string(),
            fields,
        })?;
        Ok(resp)
    }

    /// Gets the value of a field in a set for a key.
    /// # Arguments
    /// * `key` - The key to get the value of.
    /// * `field` - The field to get the value of.
    /// # Returns
    /// * [`Value`] - The value of the field, VNull if the field does not exist.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn hget(&mut self, key: &str, field: &str) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::HGET {
            key: key.to_string(),
            field: field.to_string(),
        })?;
        Ok(resp)
    }

    /// Gets all fields for a set for a key.
    /// # Arguments
    /// * `key` - The key to get the fields of.
    /// # Returns
    /// * [`Value`] - A list of fields and their values. TODO: Probalby wrong
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn hgetall(&mut self, key: &str) -> Result<HSetValue> {
        let resp = self.command_client.execute_hset_command(Command::HGETALL {
            key: key.to_string(),
        })?;
        Ok(resp)
    }

    /// Sets the value of a key with an expiration time.
    /// # Arguments
    /// * `key` - The key to set the value of.
    /// * `value` - The value to set.
    /// * `option`: [`SetOption`] - The option to specify conditions for setting the expiry.
    /// # Returns
    /// * [`Value`] - A response from the server with an OK if succes.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn setex<T: Into<SetInput>>(
        &mut self,
        key: &str,
        value: T,
        option: SetOption,
    ) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::SET {
            key: key.to_string(),
            value: value.into(),
            option,
            get: false,
        })?;
        Ok(resp)
    }
    /// Returns the remaining time to live (in seconds) of a key that has an expiration set.
    /// # Arguments
    /// * `key` - The key to get the time to live of.
    /// # Returns
    /// * [`Value`] - The remaining time to live in seconds.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn ttl(&mut self, key: &str) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::TTL {
            key: key.to_string(),
        })?;
        Ok(resp)
    }

    /// Returns the type of the value stored at `key` as a string.
    /// # Arguments
    /// * `key` - The key to get the type of.
    /// # Returns
    /// * [`Value`] - The type of the value stored at `key`, as a [`Value::VStr`] variant.
    /// # Errors
    /// * [`StreamError`] - If an error occured in the communication stream.
    pub fn dtype(&mut self, key: &str) -> Result<ScalarValue> {
        let resp = self.command_client.execute_scalar_command(Command::TYPE {
            key: key.to_string(),
        })?;
        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use uuid::Uuid;

    use super::*;
    const HOST: &str = "localhost";
    const PORT: u16 = 7379;

    #[test]
    fn test_key_w_spaces() {
        // NOTE: Today this is legal, but should it?
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test ilegal key";
        let value = SetInput::Str("ilegal key?".to_string());
        let result = client.set(key, value.clone());
        assert!(result.is_ok());
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VStr("ilegal key?".to_string()));
    }

    #[test]
    fn test_key_w_underscores() {
        // NOTE: Today this is legal, but should it?
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_ilegal_key";
        let value = SetInput::Str("ilegal key with underscores?".to_string());
        let result = client.set(key, value.clone());
        assert!(result.is_ok());
        let value_get = client.get(key).unwrap();
        assert_eq!(
            value_get,
            ScalarValue::VStr("ilegal key with underscores?".to_string())
        );
    }

    #[test]
    fn test_key_w_newline() {
        // NOTE: Today this is legal, but should it?
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test\nilegal\nkey";
        let value = SetInput::Str("ilegal key with newlines?".to_string());
        let result = client.set(key, value.clone());
        assert!(result.is_ok());
        let value_get = client.get(key).unwrap();
        assert_eq!(
            value_get,
            ScalarValue::VStr("ilegal key with newlines?".to_string())
        );
    }

    #[test]
    fn test_key_w_weird_symbols() {
        // NOTE: Today this is legal, but should it?
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test!@#$«»%^&*()_+\t";
        let value = SetInput::Str("ilegal key with weird symbols?".to_string());
        let result = client.set(key, value.clone());
        assert!(result.is_ok());
        let value_get = client.get(key).unwrap();
        assert_eq!(
            value_get,
            ScalarValue::VStr("ilegal key with weird symbols?".to_string())
        );
    }

    #[test]
    fn test_key_w_underscores_cause_problems_with_exists() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_ilegal_key_exists";
        let value = SetInput::Str("ilegal key with underscores?".to_string());
        let result = client.set(key, value.clone());
        assert!(result.is_ok());
        let value_get = client.exists(key, vec![key, key]).unwrap();
        assert_eq!(value_get, ScalarValue::VInt(9)); // BUG: There is probably a bug with how additional
                                                     // keys are handled in the exists command.
    }

    #[test]
    fn test_case_sensitive_keys() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "UPPERcase";
        let value = SetInput::Str("case sensitive key?".to_string());
        let result = client.set(key, value.clone());
        assert!(result.is_ok());
        let get = client.get("uppercase").unwrap();
        assert_eq!(get, ScalarValue::VNull);
        let value_get = client.get(key).unwrap();
        assert_eq!(
            value_get,
            ScalarValue::VStr("case sensitive key?".to_string())
        );
    }

    #[test]
    fn test_hgetset_single() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();

        let key = "testhsetint";
        let field_string = Uuid::new_v4().to_string();
        let field = field_string.as_str();

        let set_value = "Some value";
        let result = client.hset(key, (field, set_value)).unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let value_get = client.hget(key, field).unwrap();
        assert_eq!(value_get, ScalarValue::VStr(set_value.to_string()));
    }

    #[test]
    fn test_hgetset_multi() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();

        let key = "testhsetint";
        let field_string = Uuid::new_v4().to_string();
        let field = field_string.as_str();

        let field_string2 = Uuid::new_v4().to_string();
        let field2 = field_string2.as_str();

        let set_value = "Some value";
        let set_value2 = "Some value 2";
        let result = client
            .hset(key, vec![(field, set_value), (field2, set_value2)])
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(2));

        let value_get = client.hget(key, field).unwrap();
        assert_eq!(value_get, ScalarValue::VStr(set_value.to_string()));

        let value_get2 = client.hget(key, field2).unwrap();
        assert_eq!(value_get2, ScalarValue::VStr(set_value2.to_string()));
    }

    #[test]
    fn test_hgetall() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();

        let randomness = Uuid::new_v4().to_string();
        let key = format!("testhgetall{}", randomness);
        let kv = vec![
            ("somefield1", "Some  value1"),
            ("somefield2", "Some value2"),
            ("somefield3", "Some value3"),
        ];
        let set_result = client.hset(&key, kv).unwrap();
        assert_eq!(set_result, ScalarValue::VInt(3));

        let hset: HashMap<String, String> = client.hgetall(&key).unwrap().into();

        assert_eq!(hset.len(), 3);
        assert_eq!(hset.get("somefield1").unwrap(), "Some  value1");
        assert_eq!(hset.get("somefield2").unwrap(), "Some value2");
        assert_eq!(hset.get("somefield3").unwrap(), "Some value3");
    }

    #[test]
    fn test_hgetlallnil() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();

        let key = "testhgetallnil";
        let hset: HashMap<String, String> = client.hgetall(key).unwrap().into();
        assert_eq!(hset.len(), 0);
    }

    #[test]
    fn test_decr() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testdecr";
        let value = SetInput::Int(1);
        client.set(key, value.clone()).unwrap();
        let result = client.decr(key).unwrap();
        assert_eq!(result, ScalarValue::VInt(0));
    }

    #[test]
    fn test_decrby() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testdecrby";
        let value = SetInput::Int(3);
        client.set(key, value.clone()).unwrap();
        let result = client.decrby(key, 2).unwrap();
        assert_eq!(result, ScalarValue::VInt(1));
    }

    #[test]
    fn test_decrby_overflow() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testdecrbyoverflow";
        let value = SetInput::Int(i64::MIN);
        client.set(key, value.clone()).unwrap();
        let result = client.decrby(key, 1).unwrap();
        assert_eq!(result, ScalarValue::VInt(i64::MAX));
    }

    #[test]
    fn test_decr_min_underflow() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testdecrmin";
        let value = SetInput::Int(i64::MIN);
        client.set(key, value.clone()).unwrap();
        let result = client.decr(key).unwrap();
        assert_eq!(result, ScalarValue::VInt(i64::MAX));
    }

    #[test]
    fn test_del() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testdel";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.del(vec![key]).unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_expire() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testexpire";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.expire(key, 1, ExpireOption::None).unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        std::thread::sleep(std::time::Duration::from_secs(2));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_expire_nx() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testexpirenx";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.expire(key, 1, ExpireOption::NX).unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let result = client.expire(key, 100, ExpireOption::NX).unwrap();
        assert_eq!(result, ScalarValue::VInt(0));

        std::thread::sleep(std::time::Duration::from_secs(2));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_expire_xx() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testexpirexx";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();

        let result = client.expire(key, 100, ExpireOption::XX).unwrap();
        assert_eq!(result, ScalarValue::VInt(0));

        let result = client.expire(key, 100, ExpireOption::None).unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let result = client.expire(key, 1, ExpireOption::XX).unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        std::thread::sleep(std::time::Duration::from_secs(3));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_existsmany() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key1 = "testexistsmany1";
        client.set(key1, "test").unwrap();
        let key2 = "testexistsmany2";
        client.set(key2, "test").unwrap();
        let key3 = "testexistsmany3";
        let result = client.exists(key1, vec![key2, key3]).unwrap();
        assert_eq!(result, ScalarValue::VInt(3));
    }

    #[test]
    fn test_exists_one() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key1 = "testexists1";
        client.set(key1, "test").unwrap();
        let result = client.exists(key1, vec![]).unwrap();
        assert_eq!(result, ScalarValue::VInt(1));
    }

    #[test]
    fn test_exists_two() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key1 = "testexiststwo1";
        client.set(key1, "test").unwrap();
        let key2 = "testexiststwo2";
        client.set(key2, "test").unwrap();
        let result = client.exists(key1, vec![key2]).unwrap();
        assert_eq!(result, ScalarValue::VInt(2));
    }

    #[test]
    fn test_expireat() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testexpireat";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 1;

        let result = client
            .expireat(key, timestamp as i64, ExpireAtOption::None)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        std::thread::sleep(std::time::Duration::from_secs(2));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_expireat_nx() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testexpireatnx";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 1;

        let result = client
            .expireat(key, timestamp as i64, ExpireAtOption::NX)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let result = client
            .expireat(key, timestamp as i64, ExpireAtOption::NX)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(0));

        std::thread::sleep(std::time::Duration::from_secs(2));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_expireat_xx() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testexpireatxx";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 1;

        let result = client
            .expireat(key, timestamp as i64, ExpireAtOption::XX)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(0));

        let result = client
            .expireat(key, timestamp as i64, ExpireAtOption::None)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let result = client
            .expireat(key, timestamp as i64, ExpireAtOption::XX)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        std::thread::sleep(std::time::Duration::from_secs(2));
        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_expireat_gt() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testexpireatgt";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();

        let timestamp_2sec = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 2;

        let timestamp_1sec = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 1;

        let result = client
            .expireat(key, timestamp_2sec as i64, ExpireAtOption::GT)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(0));

        let result = client
            .expireat(key, timestamp_1sec as i64, ExpireAtOption::None)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let result = client
            .expireat(key, timestamp_2sec as i64, ExpireAtOption::GT)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let result = client
            .expireat(key, timestamp_1sec as i64, ExpireAtOption::GT)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(0));
    }

    #[test]
    fn test_expireat_lt() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testexpireatlt";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();

        let timestamp_2sec = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 2;

        let timestamp_1sec = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 1;

        let result = client
            .expireat(key, timestamp_1sec as i64, ExpireAtOption::LT)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(0));

        let result = client
            .expireat(key, timestamp_2sec as i64, ExpireAtOption::None)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let result = client
            .expireat(key, timestamp_1sec as i64, ExpireAtOption::LT)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(1));

        let result = client
            .expireat(key, timestamp_2sec as i64, ExpireAtOption::LT)
            .unwrap();
        assert_eq!(result, ScalarValue::VInt(0));
    }

    #[test]
    fn test_expiretime() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testexpiretime";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let expire_result = client.expire(key, 1, ExpireOption::None).unwrap();
        let expire_time = client.expiretime(key).unwrap();
        assert_eq!(expire_result, ScalarValue::VInt(1));
        let now_epoch = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 1;
        assert_eq!(expire_time, ScalarValue::VInt(now_epoch as i64));
    }

    #[test]
    #[ignore] // We ignore this test, as it will flush the database and cause other tests to fail
    fn test_flushdb() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testflushdb";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.flushdb().unwrap();
        assert_eq!(result, ScalarValue::VStr("OK".to_string()));

        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_get_set() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testgetset";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.get(key).unwrap();
        assert_eq!(result, value.into());
    }

    #[test]
    fn test_set_with_get() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testsetwithget";
        let value = SetInput::Str("test".to_string());
        let result = client.set(key, value.clone()).unwrap();
        assert_eq!(result, ScalarValue::VStr("OK".to_string()));
        let new_value = SetInput::Str("new test".to_string());
        let result = client.setget(key, new_value.clone()).unwrap();
        assert_eq!(result, value.into());
    }

    #[test]
    fn test_ping_pong() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let result = client.ping().unwrap();
        assert_eq!(result, ScalarValue::VStr("PONG".to_string()));
    }

    #[test]
    fn test_echo() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let message = "hello";
        let result = client.echo(message).unwrap();
        assert_eq!(result, ScalarValue::VStr(message.to_string()));
    }

    #[test]
    fn test_getdel() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testgetdel";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.getdel(key).unwrap();
        assert_eq!(result, value.into());

        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_getex() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testgetex";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.getex(key, GetexOption::EX(1)).unwrap();
        assert_eq!(result, value.into());

        std::thread::sleep(std::time::Duration::from_secs(2));

        let value_get = client.get(key).unwrap();
        assert_eq!(value_get, ScalarValue::VNull);
    }

    #[test]
    fn test_incr() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testincr";
        let value = SetInput::Int(1);
        client.set(key, value.clone()).unwrap();
        let result = client.incr(key).unwrap();
        assert_eq!(result, ScalarValue::VInt(2));
    }

    #[test]
    fn test_incrby() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testincrby";
        let value = SetInput::Int(1);
        client.set(key, value.clone()).unwrap();
        let result = client.incrby(key, 2).unwrap();
        assert_eq!(result, ScalarValue::VInt(3));
    }

    #[test]
    fn test_incr_overflow() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testincroverflow";
        let value = SetInput::Int(i64::MAX);
        client.set(key, value.clone()).unwrap();
        let result = client.incr(key).unwrap();
        assert_eq!(result, ScalarValue::VInt(i64::MIN));
    }

    #[test]
    fn test_ttl() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testttl";
        let value = SetInput::Str("test".to_string());
        let result = client.setex(key, value.clone(), SetOption::EX(1)).unwrap();
        assert_eq!(result, ScalarValue::VStr("OK".to_string()));
        let ttl = client.ttl(key).unwrap();
        // This test is susceptible to failing for timing reasons if not given a acceptable range
        let withinacceptable = match ttl {
            ScalarValue::VInt(v) if v <= 2 && v >= 0 => true,
            _ => false,
        };
        assert_eq!(withinacceptable, true);
    }

    #[test]
    fn test_type_str() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testtypestr";
        let value = SetInput::Str("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.dtype(key).unwrap();
        assert_eq!(result, ScalarValue::VStr("string".to_string()));
    }

    #[test]
    fn test_type_int() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testtypeint";
        let value = SetInput::Int(1);
        client.set(key, value.clone()).unwrap();
        let result = client.dtype(key).unwrap();
        assert_eq!(result, ScalarValue::VStr("int".to_string()));
    }

    #[test]
    fn test_type_null() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testtypenull";
        let result = client.dtype(key).unwrap();
        assert_eq!(result, ScalarValue::VStr("none".to_string()));
    }

    #[test]
    fn test_type_float() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testtypefloat";
        let value = SetInput::Float(1.3);
        client.set(key, value.clone()).unwrap();
        let result = client.dtype(key).unwrap();
        assert_eq!(result, ScalarValue::VStr("float".to_string()));
    }

    #[test]
    fn test_get_set_float() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "testgetsetfloat";
        let value = SetInput::Float(1.3);
        client.set(key, value.clone()).unwrap();
        let result = client.get(key);
        assert!(result.is_err()); // BUG: Known bug, cant get float values atm.
    }
}
