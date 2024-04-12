use std::thread;

// #![windows_subsystem = "windows"]
use slint::ComponentHandle;

slint::include_modules!();

mod cg_fetch;
mod cg_download;
mod cg_info;

fn main() -> Result<(), slint::PlatformError> {
    let ui = match MainWindow::new() {
        Ok(window) => window,
        Err(err) => {panic!("Slint error! {}", err)},
    };

    let uiw = ui.as_weak();
    ui.on_fetch(move || {
        thread::spawn(move || {
            cg_fetch::fetch_new();
        });
        cg_fetch::update_ui(&uiw);
    });
    
    ui.on_download(move || {
        thread::spawn(|| {
            cg_download::download_catgirl();
        });
    });
    let uiw = ui.as_weak();
    ui.on_info(move || {
        cg_info::display_info(&uiw);
    });

    ui.run()
}
