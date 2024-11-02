use rocket::response::Redirect;

#[macro_use] extern crate rocket;

const ENTRY_FILE: &str = "entries.json";

mod entries;

use entries::json::{create_empty_json, entry_exist_file, find_entry, load_entries, save_entry, Entry};

#[get("/")]
fn home() -> String {
    "ようこそ！".to_string()
}

#[get("/<path>")]
fn redirect(path: &str) -> Redirect {
    let entries = load_entries(ENTRY_FILE);
    let e = find_entry(&path.to_string(), &entries);

    match e {
        Some(p) => {
            return Redirect::to(format!("https://{}", p.get_original()))
        },
        None => {
            return Redirect::to(uri!(path_inexistent(path)))
        }
    }
}

#[get("/error/<p>")]
fn path_inexistent(p: &str) -> String {
    format!("Path '{p}' does not exist")
}

#[post("/new?<url>&<short>")]
fn register_url(url: &str, short: &str) -> &'static str {
    
    if url.len() == 0 || short.len() == 0 {
       return "URL MUST NO BE EMPTY"
    }

    let e = Entry::new(short.to_string(), url.to_string());

    if entry_exist_file(ENTRY_FILE, &e.get_shortcut()) {
        return "URL ALREADY EXIST"
    }

    save_entry(ENTRY_FILE, e).unwrap();

    "OK"
}

#[launch]
fn rocket() -> _ {
    println!("Creating empty JSON");
    create_empty_json(ENTRY_FILE);
    rocket::build().mount("/", routes![register_url, home, redirect, path_inexistent])
}
