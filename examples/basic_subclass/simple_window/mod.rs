mod imp;

use gtk::glib;

use crate::simple_application::SimpleApplication;

glib::wrapper! {
    pub struct SimpleWindow(ObjectSubclass<imp::SimpleWindow>)
        @extends gtk::Widget, gtk::Container, gtk::Bin, gtk::Window, gtk::ApplicationWindow;
}

impl SimpleWindow {
    pub fn new(app: &SimpleApplication) -> Self {
        glib::Object::new(&[("application", app)]).expect("Failed to create SimpleWindow")
    }
}
