use crate::{
    client::Client,
    commands::{Command, CommandExecutor, ScalarValue},
    errors::ClientError,
    stream::Stream,
    watchstream::WatchStream,
};

type Result<T> = std::result::Result<T, ClientError>;

impl Client {
    /// Get a watch stream for a key.
    /// >[!WARNING]
    /// > This operation is non deterministic, but will at best effort yield changes.
    /// # Arguments
    /// * `key` - The key to watch
    /// # Returns
    /// * A watch stream and the first value of the key
    /// # Errors
    /// * If the watch stream could not be created
    pub fn get_watch(&mut self, key: &str) -> Result<(WatchStream, ScalarValue)> {
        let mut new_watch_stream = WatchStream::new(self.host.clone(), self.port)?;
        new_watch_stream.handshake()?;
        let get_watch = Command::GETWATCH {
            key: key.to_string(),
        };
        let reply = new_watch_stream.execute_scalar_command(get_watch)?;
        new_watch_stream.fingerprint = Some(key.to_string());
        Ok((new_watch_stream, reply))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        thread,
    };

    use super::*;
    const HOST: &str = "localhost";
    const PORT: u16 = 7379;

    // BUG: When keys contain underscores, it seems to give inconsistent behaviors
    #[allow(dead_code)]
    const BUGGY_KEYS: [&str; 4] = [
        "watch_key",
        "watch_key_first_value",
        "watch_key_first_val_int",
        "watch_key_iter",
    ];

    const GOOD_KEYS: [&str; 4] = [
        "watchkey",
        "watchkeyfirstvalue",
        "watchkeyfirstvalint",
        "watchkeyiter",
    ];

    const KEYS: [&str; 4] = GOOD_KEYS;

    #[test]
    fn test_create_watcher() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = KEYS[0];
        let watch_stream = client.get_watch(key);
        assert!(watch_stream.is_ok());
    }

    #[test]
    fn test_get_watch_first_value_null() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = KEYS[1];
        let watch_stream = client.get_watch(key).unwrap();
        let (_, first_value) = watch_stream;
        assert_eq!(first_value, ScalarValue::VNull);
    }

    #[test]
    fn test_get_watch_first_val_int() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = KEYS[2];
        client.set(key, 1).unwrap();
        let watch_stream = client.get_watch(key).unwrap();
        let (_, first_value) = watch_stream;
        assert_eq!(first_value, ScalarValue::VInt(1));
    }

    #[test]
    #[ignore] // BUG: Flaky test
    fn test_get_watch_iter() {
        let key = KEYS[3];
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        client.del(key).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
        let (watch_stream, _) = client.get_watch(key).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
        let empty_value_vec: Vec<ScalarValue> = vec![];
        let changed = Arc::new(Mutex::new(empty_value_vec));
        let changed_clone = changed.clone();
        thread::spawn(move || {
            let watch_stream = watch_stream;
            for change in watch_stream {
                changed.lock().unwrap().push(change.into());
            }
        });
        for i in 0..=5 {
            client.set(key, i).unwrap();
        }

        thread::sleep(std::time::Duration::from_secs(1));
        let changed = changed_clone.lock().unwrap();
        assert_eq!(
            *changed,
            vec![
                ScalarValue::VNull, // WARN: Sometimes this is omitted, sometimes not
                ScalarValue::VInt(0),
                ScalarValue::VInt(1),
                ScalarValue::VInt(2),
                ScalarValue::VInt(3),
                ScalarValue::VInt(4),
                ScalarValue::VInt(5),
            ]
        );
    }
}
