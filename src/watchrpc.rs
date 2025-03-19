use crate::{
    client::{Client, ClientError},
    commands::{Command, CommandExecutor, Value},
    stream::Stream,
    watchstream::WatchStream,
};

type Result<T> = std::result::Result<T, ClientError>;

impl Client {
    #[deprecated(note = "This operation is unstable.")]
    pub fn get_watch(&mut self, key: &str) -> Result<(WatchStream, Value)> {
        let mut new_watch_stream = WatchStream::new(self.host.clone(), self.port)?;
        new_watch_stream.handshake()?;
        let get_watch = Command::GETWATCH {
            key: key.to_string(),
        };
        let reply = new_watch_stream.execute_command(get_watch)?;
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

    #[test]
    #[ignore] // TODO: Most likely flaky test, watcher client seems to be unstable. Problem peraps
              // due to spawning to many watch clients.
    fn test_create_watcher() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let watch_stream = client.get_watch("watch_key");
        assert!(watch_stream.is_ok());
    }

    #[test]
    #[ignore] // TODO: Flaky test (Handshake respone with VNull, rarely)
    fn test_get_watch_first_value_null() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "watch_key_first_value";
        let watch_stream = client.get_watch(key).unwrap();
        let (_, first_value) = watch_stream;
        assert_eq!(first_value, Value::VNull);
    }

    #[test]
    #[ignore] // TODO: Flaky test  (Connection reset by peers, rarely)
    fn test_get_watch_first_val_int() {
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        let key = "watch_key_first_val_int";
        client.set(key, 1).unwrap();
        let watch_stream = client.get_watch(key).unwrap();
        let (_, first_value) = watch_stream;
        assert_eq!(first_value, Value::VInt(1));
    }

    #[test]
    #[ignore] // TODO: Flaky test
    fn test_get_watch_iter() {
        let key = "watch_key_iter";
        let mut client = Client::new(HOST.to_string(), PORT).unwrap();
        client.del(key).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
        let (watch_stream, _) = client.get_watch(key).unwrap();
        thread::sleep(std::time::Duration::from_secs(1));
        let empty_value_vec: Vec<Value> = vec![];
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
                Value::VNull, // WARNING: This sometimes is omitted, sometimes not
                Value::VInt(0),
                Value::VInt(1),
                Value::VInt(2),
                Value::VInt(3),
                Value::VInt(4),
                Value::VInt(5),
            ]
        );
    }
}
