use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Entry {
    long_url: String,
    short_url: String,
    expiration: Option<u64>,
}

impl Entry {
    pub fn new(long_url: String, short_url: String) -> Self {
        Entry {
            long_url,
            short_url,
            expiration: None,
        }
    }

    pub fn long_url(&self) -> &String {
        &self.long_url
    }

    pub fn short_url(&self) -> &String {
        &self.short_url
    }

    pub fn has_expiration(&self) -> bool {
        match self.expiration {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_expiration(&self) -> u64 {
        if self.has_expiration() {
            return self.expiration.unwrap();
        }
        0
    }

    pub fn set_expiration(&mut self, exp: u64) {
        self.expiration = Some(exp);
    }
}
