use std::{env::temp_dir, io::Read};

use serde_json::Value;

pub fn display_info() {
    let tmp_dir = format!("{}/jcgd/data.json", temp_dir().to_str().unwrap());
    // Load JSON
    let mut json_file = match std::fs::File::open(tmp_dir) {
        Ok(json) => {json},
        Err(e) => {
            return;
        },
    };
    let mut json_string: String = String::new();
    match json_file.read_to_string(&mut json_string) {
        Ok(_) => {},
        Err(e) => {panic!("Error converting JSON file to string!\n{}", e)},
    };
    let json: Value = match serde_json::from_str(&json_string) {
        Ok(json) => json,
        Err(e) => {panic!("Error parsing string to JSON!\n{}", e)},
    };
    let image = &json["images"][0];
    let tags = &image["tags"];
    for i in tags.as_array().unwrap() {
        let tg = i.as_str().unwrap_or_default();
        println!("{}", tg);
    }
}