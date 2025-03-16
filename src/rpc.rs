use crate::client::Client;
use crate::client::Result;
use crate::commands::Command;
use crate::commands::Value;

impl Client {
    pub fn get(&mut self, key: &str) -> Result<Value> {
        let command = Command::GET {
            key: key.to_string(),
        };
        let gg = self.command_client.request_reply(command)?;
        Ok(gg)
    }

    pub fn set(&mut self, key: &str, value: Value) -> Result<()> {
        let command = Command::SET {
            key: key.to_string(),
            value,
            option: crate::commands::SetOption::None,
            get: false,
        };
        self.command_client.request_reply(command)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const HOST: &str = "localhost";
    const PORT: u16 = 7379;

    #[test]
    fn test_get_set() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "test_get_set";
        let value = Value::VStr("test".to_string());
        client.set(key, value.clone()).unwrap();
        let result = client.get(key).unwrap();
        assert_eq!(result, value);
    }
}
