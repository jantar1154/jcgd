use slint::ComponentHandle;

slint::include_modules!();
mod cg_fetch;
mod cg_download;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    let uiw = ui.as_weak();
    ui.on_fetch(move || {
        cg_fetch::fetch_new(&uiw);
    });
    
    let uiw = ui.as_weak();
    ui.on_download(move || {
        cg_download::download_catgirl(&uiw);
    });

    let uiw = ui.as_weak();
    ui.on_cp_to_clipboard(move || {

    });

    let uiw = ui.as_weak();
    ui.on_info(move || {

    });

    ui.run()
}
