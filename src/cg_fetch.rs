use crate::AppWindow;
use reqwest::blocking;
use serde_json::Value;
use slint::{Image, SharedPixelBuffer, SharedString, Weak};

fn file_to_slint_img(img_path: &str, filename: &str) -> slint::Image {
    // Convert the file into rgb8 array and then the array into into slint::Image
    let img = image::open(format!("{}/{}", img_path, filename))
        .expect("Error loading image (not found on disk)")
        .into_rgb8();
    let slint_img = SharedPixelBuffer::clone_from_slice (
        img.as_raw(),
        img.width(),
        img.height()
    );
    Image::from_rgb8(slint_img)
}


fn get_random_id() -> Result<String, reqwest::Error> {
    let url = "https://nekos.moe/api/v1/random/image";
    let resp: Value = blocking::get(url)?.json()?;
    let image = &resp["images"][0];
    let id_val = &image["id"];
    let id = format!("{}", id_val).replace("\"", "");
    Ok(id)
}

fn get_temp_path() -> String {
    // Download the image into the system's temp folder
    let img_path = std::env::temp_dir();
    let temp_path = match img_path.to_str() {
        Some(path) => path,
        None => panic!("Could not find temp folder!"),
    };
    temp_path.to_owned()
}

// Handles getting image from 
pub(crate) fn fetch_new(uiw: &Weak<AppWindow>) {
    let uiw = uiw.upgrade().unwrap();
    
    let temp_file_name = "jcgd.jpeg";
    
    let id = match get_random_id() {
        Ok(id) => id,
        Err(err) => panic!("Could not receive ID from nekos.moe!\n{}", err),
    };
    
    // Use the ID to create a new link, which contains only a catgirl image
    let url = format!("https://nekos.moe/image/{}", id);

    let img_path = get_temp_path();
    let mut file = match std::fs::File::create(format!("{}/{}", img_path, temp_file_name)) {
        Ok(f) => f,
        Err(err) => panic!("Error creating file! Error: {}", err),
    };
    
    // Save an image from url into the file
    blocking::get(&url)
        .unwrap()
        .copy_to(&mut file)
        .unwrap();

    let slint_image = file_to_slint_img(&img_path, temp_file_name);

    // Set the slint image as a source in .slint file
    uiw.set_img(slint_image);
    uiw.set_status(SharedString::from(""));
}