use log::error;

use reqwest::{Error, Response};

pub async fn check_server(address: String) -> bool {
    let client = reqwest::Client::new();
    let res: Result<Response, Error> = client.get(address).send().await;

    match res {
        Ok(r) => r.status() == 200,
        Err(e) => {
            if e.is_connect() {
                error!("[ Connection error ] {e}");
            }
            false
        }
    }
}
