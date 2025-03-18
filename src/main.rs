use rocket::{response::Redirect, serde::json::Json};

#[macro_use] extern crate rocket;

const ENTRY_FILE: &str = "entries.json";

mod entries;
use entries::message::Message;
use entries::json::{create_empty_json, entry_exist_file, find_entry, load_entries, save_entry, save_file, serialize_entry, Entry};

#[get("/")]
fn home() -> Json<Message> {
    Json::from(Message::new("ようこそ！".to_string()))
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
fn path_inexistent(p: &str) -> Json<Message> {
    Json::from(Message::new(format!("Path '{p}' does not exist")))
}

#[post("/new", format="application/json", data="<entry>")]
fn register_url(entry: Json<Entry>) -> Json<Message> {
    let e = entry.0;

    if entry_exist_file(ENTRY_FILE, &e.get_shortcut()) {
        return Json::from(Message::new("The provided shortcut already exist".to_string()))
    }

    save_entry(ENTRY_FILE, e).unwrap();

    Json::from(Message::new("OK".to_string()))
}

#[delete("/delete/<shortcut>")]
fn delete_url(shortcut: String) -> Json<Message> {
    
    if !entry_exist_file(ENTRY_FILE, &shortcut.to_string()) {
        return Json::from(Message::new("The provided shortcut does not exist".to_string()))
    }

    let mut data = load_entries(ENTRY_FILE);

    let pos = data.iter().position(|e: &Entry| e.get_shortcut().eq(&shortcut)).unwrap();
    
    data.remove(pos);

    println!("{:#?}", data);

    create_empty_json(ENTRY_FILE);
    save_file(ENTRY_FILE, &serialize_entry(data));

    Json::from(Message::new("OK".to_string()))
}

#[get("/list")]
fn list_urls() -> String {
    serialize_entry(load_entries(ENTRY_FILE))
}

#[launch]
fn rocket() -> _ {
    println!("Creating empty JSON");
    create_empty_json(ENTRY_FILE);
    rocket::build().mount("/", routes![register_url, home, redirect, path_inexistent, list_urls, delete_url])
}
