#[macro_use] extern crate rocket;

mod entries;
mod routes;

use entries::json::create_empty_json;
use crate::routes::*;
use std::env::var;

#[launch]
fn rocket() -> _ {
    let ef;

    match var("ENTRY_FILE") {
        Ok(x) => ef = x,
        Err(_) => {
            println!("Environment variable not found");
            std::process::exit(1);
        }
    }

    println!("Creating empty JSON");
    create_empty_json(&ef);
    rocket::build().mount("/", routes![register_url, home, redirect, path_inexistent, list_urls, delete_url])
}
