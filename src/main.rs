slint::include_modules!();
use std::env;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    fn update_current_dir(ui: &AppWindow) {
        let ui_handle = ui.as_weak();
        (move || {
            let ui = ui_handle.unwrap();
            let cwd = env::current_dir().unwrap();
            let path: String = String::from(cwd.display().to_string());
            ui.set_current_dir(slint::SharedString::from(path));
        })()
    }
    
    update_current_dir(&ui);
    ui.run()
}
