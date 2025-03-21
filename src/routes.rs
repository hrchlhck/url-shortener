use rocket::{response::Redirect, serde::json::Json};
use crate::entries::message::Message;
use crate::entries::json::{create_empty_json, entry_exist_file, find_entry, load_entries, save_entry, save_file, serialize_entry, Entry};
use std::env::var;

#[get("/")]
pub fn home() -> Json<Message> {
    Json::from(Message::new("ようこそ！".to_string()))
}

#[get("/<path>")]
pub fn redirect(path: &str) -> Redirect {
    let entries = load_entries(&var("ENTRY_FILE").unwrap());
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
pub fn path_inexistent(p: &str) -> Json<Message> {
    Json::from(Message::new(format!("Path '{p}' does not exist")))
}

#[post("/new", format="application/json", data="<entry>")]
pub fn register_url(entry: Json<Entry>) -> Json<Message> {
    let e = entry.0;

    if entry_exist_file(&var("ENTRY_FILE").unwrap(), &e.get_shortcut()) {
        return Json::from(Message::new("The provided shortcut already exist".to_string()))
    }

    save_entry(&var("ENTRY_FILE").unwrap(), e).unwrap();

    Json::from(Message::new("OK".to_string()))
}

#[delete("/delete/<shortcut>")]
pub fn delete_url(shortcut: String) -> Json<Message> {
    
    if !entry_exist_file(&var("ENTRY_FILE").unwrap(), &shortcut.to_string()) {
        return Json::from(Message::new("The provided shortcut does not exist".to_string()))
    }

    let mut data = load_entries(&var("ENTRY_FILE").unwrap());

    let pos = data.iter().position(|e: &Entry| e.get_shortcut().eq(&shortcut)).unwrap();
    
    data.remove(pos);

    println!("{:#?}", data);

    create_empty_json(&var("ENTRY_FILE").unwrap());
    save_file(&var("ENTRY_FILE").unwrap(), &serialize_entry(data));

    Json::from(Message::new("OK".to_string()))
}

#[get("/list")]
pub fn list_urls() -> String {
    serialize_entry(load_entries(&var("ENTRY_FILE").unwrap()))
}