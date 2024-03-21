use slint::ComponentHandle;

slint::include_modules!();
mod cg_fetch;
mod cg_download;

fn main() -> Result<(), slint::PlatformError> {
    let ui = match AppWindow::new() {
        Ok(aw) => {aw},
        Err(err) => {panic!("Slint error! {}", err)},
    };
    
    let uiw = ui.as_weak();
    ui.on_fetch(move || {
        cg_fetch::fetch_new(&uiw);
    });
    
    ui.on_download(move || {
        cg_download::download_catgirl();
    });

    let uiw = ui.as_weak();
    ui.on_cp_to_clipboard(move || {

    });

    let uiw = ui.as_weak();
    ui.on_info(move || {

    });

    ui.run()
}
