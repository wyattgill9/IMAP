use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: u64,
    pub payload: String,
    pub headers: HashMap<String, String>,
}

impl Message {
    pub fn new(id: u64, payload: String) -> Self {
        Self {
            id,
            payload,
            headers: HashMap::new(),
        }
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }
}
