pub mod simple_application;
pub mod simple_window;

use gtk::prelude::*;

fn main() {
    gtk::init().expect("Failed to initialize gtk");

    let app = simple_application::SimpleApplication::new();

    app.run();
}
