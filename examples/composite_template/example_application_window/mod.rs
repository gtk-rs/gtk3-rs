mod imp;

use glib::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct ExampleApplicationWindow(ObjectSubclass<imp::ExampleApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, @implements gio::ActionMap, gio::ActionGroup;
}

impl ExampleApplicationWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(app: &P) -> Self {
        glib::Object::new(&[("application", app)])
            .expect("Failed to create ExampleApplicationWindow")
    }

    fn init_label(&self) {
        // To access fields such as template children, you must get
        // the private struct.
        let self_ = imp::ExampleApplicationWindow::from_instance(self);
        self_
            .subtitle
            .set_text("This is an example window made using composite templates");
    }
}
