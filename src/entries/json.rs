use serde::{Deserialize, Serialize};
use std::{fs, io::{Read, Write}, process::exit};

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct Entry {
    shortcut_url: String,
    original_url: String,
}

impl Entry {
    pub fn new(shortcut_url: String, original_url: String) -> Entry {
        Entry { shortcut_url, original_url }
    }

    pub fn get_shortcut(&self) -> String {
        self.shortcut_url.clone()
    }

    pub fn get_original(&self) -> String {
        self.original_url.clone()
    }
}


fn check_exist(filename: &str) -> bool {
    match fs::exists(filename) {
        Ok(ok) => ok,
        Err(_) => false
    }
}

fn read_file(filename: &str) -> String {
    let mut file_content = fs::File::open(filename).unwrap();
    let mut buf: String = String::new();
    file_content.read_to_string(&mut buf).unwrap();
    buf
}

fn deserialize_entry(file_content: &String) -> Vec<Entry> {
    let json: Vec<Entry> = match serde_json::from_str(file_content) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Unable to load data");
            // Exit the program with exit code `1`.
            exit(1)
        }
    };

    json
}

fn save_file(filename: &str, contents: &String) {
    let mut file = fs::OpenOptions::new().write(true).open(filename).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}

fn serialize_entry(entries: Vec<Entry>) -> String {
    let x = match serde_json::to_string_pretty(&entries) {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Unable to load data");
            // Exit the program with exit code `1`.
            exit(1)
        }
    };

    x
}

pub fn save_entry(filename: &str, entry: Entry) -> Result<(), String> {

    if !check_exist(filename) {
        return Err(format!("File '{filename}' does not exist."));
    }

    let contents = read_file(filename);
    let mut json_entries = deserialize_entry(&contents);

    json_entries.push(entry);

    save_file(filename, &serialize_entry(json_entries));

    Ok(())
}


pub fn entry_exist(short: &String, entries: &Vec<Entry>) -> bool {
    let f = |x: &&Entry| {
        x.get_shortcut().eq(short)
    };
    entries.iter().filter(f).collect::<Vec<&Entry>>().len() == 1
}

pub fn entry_exist_file(filename: &str, short: &String) -> bool {
    let contents = read_file(filename);
    let contents = deserialize_entry(&contents);
    entry_exist(short, &contents)
}

pub fn find_entry<'a>(short: &String, entries: &'a Vec<Entry>) -> Option<&'a Entry> {
    if !entry_exist(short, entries) {
        return None
    }

    let f = |x: &&Entry| {
        x.get_shortcut().eq(short)
    };
    Some(&entries.iter().filter(f).nth(0).unwrap())
}

pub fn load_entries(filename: &str) -> Vec<Entry> {
    let content = read_file(filename);
    deserialize_entry(&content)
}

pub fn create_empty_json(filename: &str) {
    let mut file = fs::File::create(filename).unwrap();
    file.write_all(b"[]").unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file_exist() {
        let res = check_exist("Cargo.toml");

        assert_eq!(res, true);
    }

    #[test]
    fn test_file_not_exist() {
        let res = check_exist("Carg1o.toml");

        assert_eq!(res, false);
    }
    

    #[test]
    fn test_file_not_exist_save_entry() {
        let filename = "Car1go.toml";
        let entry = Entry::new(String::new(),String::new());
        let res = save_entry(filename, entry);

        assert_eq!(res, Err(format!("File '{filename}' does not exist.")));
    }

    #[test]
    fn test_read_file_has_content() {
        let filename = "mock/entries_read.json";
        let file_buf = read_file(filename);

        let data = r#"[
  {
    "shortcut_url": "nothing",
    "original_url": "not.thing.com"
  }
]"#;

        assert_eq!(data, file_buf);
    }

    #[test]
    fn test_save_file() {
        let filename = "mock/entries.json";
        
       
        let data = r#"[
  {
    "shortcut_url": "nothing",
    "original_url": "not.thing.com"
  },
  {
    "shortcut_url": "abc",
    "original_url": "a.b.c"
  }
]"#;
        save_file(filename, &data.to_string());
        let file_buf = read_file(filename);

        assert_eq!(data, file_buf);
    }
    #[test]
    fn test_read_json() {
        let expected = vec![
            Entry::new(String::from("nothing"), String::from("not.thing.com"))
        ];

        let content = read_file("mock/entries_read.json");
        let got = deserialize_entry(&content);
        assert_eq!(expected, got);
    }

    #[test]
    fn test_serialize() {
        let expected = vec![
            Entry::new(String::from("nothing"), String::from("not.thing.com")),
            Entry::new(String::from("abc"), String::from("a.b.c"))
        ];

        let data = r#"[
  {
    "shortcut_url": "nothing",
    "original_url": "not.thing.com"
  },
  {
    "shortcut_url": "abc",
    "original_url": "a.b.c"
  }
]"#;

        let got = serialize_entry(expected);
        assert_eq!(got, data);
    }

    #[test]
    fn test_serialize_save() {
        let expected = r#"[
  {
    "shortcut_url": "nothing",
    "original_url": "not.thing.com"
  },
  {
    "shortcut_url": "abc",
    "original_url": "a.b.c"
  },
  {
    "shortcut_url": "ggl",
    "original_url": "google.com"
  }
]"#;
    let data = vec![
        Entry::new(String::from("nothing"), String::from("not.thing.com")),
        Entry::new(String::from("abc"), String::from("a.b.c")),
        Entry::new(String::from("ggl"), String::from("google.com")),
    ];

    let filename = "mock/entries_serialized.json";
    let ser = serialize_entry(data);
    save_file(filename, &ser);

    let content = read_file(filename);
    assert_eq!(expected.to_string(), content);
    }

    #[test]
    fn test_entry_exist() {
        let entries = vec![
            Entry::new("g".to_string(), "google.com".to_string()),
            Entry::new("g1".to_string(), "google.com".to_string()),
        ];
        let short = String::from("g");
        assert_eq!(entry_exist(&short, &entries), true);
    }

    #[test]
    fn test_entry_not_exist() {
        let entries = vec![
            Entry::new("g".to_string(), "google.com".to_string()),
            Entry::new("g1".to_string(), "google.com".to_string()),
        ];
        let short = String::from("g3");
        assert_eq!(entry_exist(&short, &entries), false);
    }

    #[test]
    fn test_entry_exist_file() {
        assert_eq!(entry_exist_file("mock/entries.json", &"abc".to_string()), true);
    }

    #[test]
    fn test_find_entry() {
        let content = read_file("mock/entries.json");
        let entries = deserialize_entry(&content);

        let e = find_entry(&"nothing".to_string(), &entries);

        match e {
            Some(x) => {
                assert_eq!(x.get_shortcut(), "nothing")
            }
            None => assert!(false)
        };
    }

    #[test]
    fn test_create_empty_json() {
        let filename = "mock/empty.json";
        create_empty_json(filename);
        let data = read_file(&filename);

        assert_eq!(data, "[]");
    }
}