extern crate redis;
use redis::{Commands, Client};

pub struct PersistedEvent<T> {
    location: i64,
    data: T
}

pub trait EventStorage {
    fn get_events<T>(&self, stream_id: String) -> Result<Vec<PersistedEvent<T>>, String>;

    fn append_event<T>(&self, stream_id: String, expected_location: i64) -> Result<(), String>;
}

pub struct RedisEventStore {
    client: Client
}

impl EventStorage for RedisEventStore {
    fn get_events<T>(&self, stream_id: String) -> Result<Vec<PersistedEvent<T>>, String> {
        match self.client.get_connection() {
            Ok(mut connection) => connection.xrange_all::<String, String>(stream_id),
            Err(_) => return Err("Failed to get redis connection.".to_string())
        };

        Ok(vec![])
    }

    fn append_event<T>(&self, stream_id: String, expected_location: i64) -> Result<(), String> {
        Ok(())
    }
}
