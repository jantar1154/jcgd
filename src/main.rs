#![windows_subsystem = "windows"]
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
        cg_fetch::fetch_new(&uiw);
    });
    
    ui.on_download(move || {
        cg_download::download_catgirl();
    });

    ui.run()
}
