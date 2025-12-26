mod application;
mod color;
mod widgets;
mod window;

use adw::prelude::*;
use application::LonApplication;
use gtk::gio;

fn main() {
    gio::resources_register_include!("lon.gresource").expect("Failed to register resources");

    let app = LonApplication::new();
    app.run();
}
