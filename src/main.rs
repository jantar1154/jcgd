use slint::ComponentHandle;

slint::include_modules!();
mod cg_fetch;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let uiw = ui.as_weak();
        
    ui.on_fetch(move || {
        cg_fetch::fetch_new(&uiw);
    });

    ui.run()
}
