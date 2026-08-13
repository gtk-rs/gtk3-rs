mod imp;

use gtk::{gio, glib};

glib::wrapper! {
    pub struct SimpleApplication(ObjectSubclass<imp::SimpleApplication>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl SimpleApplication {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::builder()
            .property("application-id", "org.gtk-rs.SimpleApplication")
            .build()
    }
}
