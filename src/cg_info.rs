use crate::MainWindow;
use std::{env::temp_dir, io::Read};

use serde_json::Value;
use slint::Weak;

pub fn display_info(uiw: &Weak<MainWindow>) {
    let ui = uiw.upgrade().unwrap();
    let tmp_dir = format!("{}/jcgd/data.json", temp_dir().to_str().unwrap());
    // Load JSON
    let mut json_file = match std::fs::File::open(tmp_dir) {
        Ok(json) => json,
        Err(_) => {
            return;
        },
    };
    let mut json_string: String = String::new();
    match json_file.read_to_string(&mut json_string) {
        Err(e) => {panic!("Error converting JSON file to string!\n{}", e)},
        _ => {},
    };
    let json: Value = match serde_json::from_str(&json_string) {
        Ok(json) => json,
        Err(e) => {panic!("Error parsing string to JSON!\n{}", e)},
    };
    let image = &json["images"][0];
    let _tags = &image["tags"];

    println!("{:?}", _tags);
}