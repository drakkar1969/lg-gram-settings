mod app;
mod window;
mod gram_setting_widget;
mod lg_gram;
mod battery_limit_object;

use gtk::{gio, glib};
use gtk::prelude::*;

use app::Application;

const APP_ID: &str = "com.github.LG-GramSettings";

fn main() -> glib::ExitCode {
    // Register and include resources
    gio::resources_register_include!("resources.gresource")
        .expect("Failed to register resources");

    // Run app
    let app = Application::new(APP_ID, gio::ApplicationFlags::default());

    app.run()
}
