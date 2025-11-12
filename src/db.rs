use log::{debug, info};
use r2d2::{Pool, PooledConnection};
use redis::{Client, TypedCommands};

pub fn get_redis_connection_pool(address: String) -> Pool<Client> {
    debug!("Attepting to connect to Redis");

    let client = redis::Client::open(format!("redis://{address}")).unwrap();
    let pool = r2d2::Pool::builder().build(client).unwrap();

    info!("Connected to Redis successfully");
    pool
}

pub fn key_exists<'a>(client: &mut PooledConnection<Client>, key: &'a str) -> bool {
    let ret = client.exists(key).unwrap();
    debug!(
        "[ Redis Client {} ] Checking if {} exists -> {ret}",
        client.client_id().unwrap(),
        key
    );
    ret
}
