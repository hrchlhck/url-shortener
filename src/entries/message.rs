use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message{
    message: String
}

impl Message {
    pub fn new(message: String) -> Message {
        Message{message}
    }
}