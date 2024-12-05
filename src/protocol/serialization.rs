use super::message::Message;
use super::error::Result;
use bincode;

pub fn serialize_message(msg: &Message) -> Result<Vec<u8>> {
    Ok(bincode::serialize(msg)?)
}

pub fn deserialize_message(data: &[u8]) -> Result<Message> {
    Ok(bincode::deserialize(data)?)
}
