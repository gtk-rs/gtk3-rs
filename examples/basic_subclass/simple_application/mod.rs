mod imp;

use gio::ApplicationFlags;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct SimpleApplication(ObjectSubclass<imp::SimpleApplication>)
        @extends gio::Application, gtk::Application;
}

impl SimpleApplication {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &"org.gtk-rs.SimpleApplication"),
            ("flags", &ApplicationFlags::empty()),
        ])
        .expect("Failed to create SimpleApp")
    }
}
