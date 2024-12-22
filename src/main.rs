// Disable console if release version is compiled
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;

use crate::app::App;


fn panic_hook(panic_info: &std::panic::PanicHookInfo) {
    app::report_error(panic_info.to_string());
}


fn main() {
    std::panic::set_hook(Box::new(panic_hook));

    let app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            app::report_error(format!("error while creating app: {e}"));
            std::process::exit(1)
        },
    };

    if let Err(e) = app.run() {
        app::report_error(format!("error while running app: {e}"));
    }
}