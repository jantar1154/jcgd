use serde_json::Value;
use reqwest::blocking::{self};
use slint::{ComponentHandle, Image, SharedPixelBuffer};

slint::include_modules!();

fn fetch_new(uiw: &AppWindow) -> Result<(), reqwest::Error> {
    let url = "https://nekos.moe/api/v1/random/image";
    let resp: serde_json::Value = blocking::get(url)?.json()?;
    let image: &Value = &resp["images"][0];
    let id: &Value = &image["id"];
    let id = format!("{}", id).replace("\"", "");

    let url = format!("https://nekos.moe/image/{}", id);
    let mut sex = std::env::temp_dir();
    let mut file = std::fs::File::create(format!("{}/jcgd.jpeg", sex.as_path().to_str().unwrap())).unwrap();
    blocking::get(&url)
        .unwrap()
        .copy_to(&mut file)
        .unwrap();
    let img = image::open("img.jpeg").expect("Error loading image (not found on disk)").into_rgb8();
    let buffer = SharedPixelBuffer::clone_from_slice (
        img.as_raw(),
        img.width(),
        img.height()
    );
    let slint_image = Image::from_rgb8(buffer);
    uiw.set_img(slint_image);

    Ok(())
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let uiw = ui.as_weak();
    
    ui.on_fetch(move || {
        let uiw = uiw.upgrade().unwrap();
        let _ = fetch_new(&uiw);
    });
    ui.run()
}