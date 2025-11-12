use log::error;
use std::env::vars;

#[derive(Default)]
pub struct Config {
    pub redis_address: Option<String>,
    pub api_address: Option<String>,
    pub api_port: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn set_redis_address(&mut self, redis_address: String) {
        self.redis_address = Some(redis_address);
    }

    pub fn set_api_address(&mut self, api_address: String) {
        self.api_address = Some(api_address);
    }

    pub fn set_api_port(&mut self, api_port: String) {
        self.api_port = Some(api_port);
    }

    pub fn validate(&self) -> bool {
        self.redis_address != None && self.api_address != None && self.api_port != None
    }
}

pub fn get_config() -> Config {
    let mut ret = Config::new();

    let settings = vars();

    for (key, val) in settings {
        match key.as_str() {
            "REDIS_ADDRESS" => ret.set_redis_address(val),
            "API_ADDRESS" => ret.set_api_address(val),
            "API_PORT" => ret.set_api_port(val),
            _ => {}
        };
    }

    if !ret.validate() {
        let base: String = String::from("Missing:");

        if ret.api_address == None {
            error!("{base} API_ADDRESS");
        }

        if ret.api_port == None {
            error!("{base} API_PORT");
        }

        if ret.redis_address == None {
            error!("{base} REDIS_ADDRESS");
        }

        std::process::exit(1);
    }
    ret
}
