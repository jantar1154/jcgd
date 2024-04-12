use std::{env, fs::File, io::Write, sync::Mutex, thread};

use crate::AppWindow;
use reqwest::blocking;
use serde_json::Value;
use slint::{SharedPixelBuffer, SharedString, Weak};

fn file_to_slint_img(img_path: &str) -> slint::Image {
    // Convert the file into rgb8 array and then the array into into slint::Image
    let img = image::open(format!("{}/jcgd.jpeg", img_path))
        .expect("Error loading image (not found on disk)")
        .into_rgb8();
    let slint_img = SharedPixelBuffer::clone_from_slice (
        img.as_raw(),
        img.width(),
        img.height()
    );
    slint::Image::from_rgb8(slint_img)
}


fn get_random_id() -> (String, Value) {
    let url = "https://nekos.moe/api/v1/random/image";
    let resp = match blocking::get(url) {
        Ok(val) => val,
        Err(err) => panic!("Did not receive a response from {}.\n{}", url, err),
    };
    let value: Value = match resp.json() {
        Ok(val) => val,
        Err(err) => panic!("Error while receiving JSON! {}", err),
    };
    let json = &value["images"][0];
    let id_val = &json["id"];
    let id = format!("{}", id_val).replace("\"", "");
    (id, value)
}

fn get_temp_path() -> String {
    // Download the image into the system's temp folder
    let temp_path = env::temp_dir();
    let temp_path = match temp_path.to_str() {
        Some(path) => path,
        None => panic!("Could not find temp folder!"),
    };
    temp_path.to_owned()
}

pub(crate) fn update_ui(uiw: &Mutex<Weak<AppWindow>>) {
    let uiw = uiw.try_lock().unwrap().upgrade().unwrap();
    uiw.set_enable_dn_cb(true);
    let tmp_path = format!("{}/jcgd", get_temp_path());
    if !std::path::Path::new(&tmp_path).exists() {
        std::fs::create_dir(&tmp_path)
            .unwrap();
    }
    let tmp_path = Mutex::new(tmp_path);
    let tp = tmp_path.try_lock().unwrap();
    let image = file_to_slint_img(&tp);
    // Set the slint image as a source in .slint file
    uiw.set_img(image);
    uiw.set_status(SharedString::from(""));
}

// Handles getting image from https://nekos.moe
// and setting the Image in uiw to the image
pub(crate) fn fetch_new() {
    let json = get_random_id();
    
    // Use the ID to create a new link, which contains only a catgirl image
    let url = format!("https://nekos.moe/image/{}", json.0);

    let tmp_path = format!("{}/jcgd", get_temp_path());
    if !std::path::Path::new(&tmp_path).exists() {
        std::fs::create_dir(&tmp_path)
            .unwrap();
    }

    // Image file
    let mut image_file = match File::create(format!("{}/jcgd.jpeg", tmp_path)) {
        Ok(f) => f,
        Err(e) => {
            panic!("Error creating image file! Error: {}", e);
        },
    };

    // JSON file
    let mut json_file = match File::create(format!("{}/data.json", tmp_path)) {
        Ok(f) => f,
        Err(e) => {
            panic!("Error creating image file! Error: {}", e);
        },
    };

    // Copy JSON to file
    match json_file.write_all(&json.1.to_string().as_bytes()) {
        Err(e) => {
            panic!("Error writing to JSON file!\n{}", e)
        },
        _ => {},
    };
    
    // Save an image from url into the file
    blocking::get(&url)
        .unwrap()
        .copy_to(&mut image_file)
        .unwrap();

}