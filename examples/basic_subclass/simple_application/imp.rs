use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::unsync::OnceCell;

use crate::simple_window::SimpleWindow;

#[derive(Debug, Default)]
pub struct SimpleApplication {
    window: OnceCell<SimpleWindow>,
}

#[glib::object_subclass]
impl ObjectSubclass for SimpleApplication {
    const NAME: &'static str = "SimpleApplication";
    type Type = super::SimpleApplication;
    type ParentType = gtk::Application;
}

impl ObjectImpl for SimpleApplication {}

/// When our application starts, the `startup` signal will be fired.
/// This gives us a chance to perform initialisation tasks that are not directly
/// related to showing a new window. After this, depending on how
/// the application is started, either `activate` or `open` will be called next.
impl ApplicationImpl for SimpleApplication {
    /// `gio::Application::activate` is what gets called when the
    /// application is launched by the desktop environment and
    /// aksed to present itself.
    fn activate(&self) {
        let window = self
            .window
            .get()
            .expect("Should always be initiliazed in gio_application_startup");
        window.show_all();
        window.present();
    }

    /// `gio::Application` is bit special. It does not get initialized
    /// when `new` is called and the object created, but rather
    /// once the `startup` signal is emitted and the `gio::Application::startup`
    /// is called.
    ///
    /// Due to this, we create and initialize the `SimpleWindow` widget
    /// here. Widgets can't be created before `startup` has been called.
    fn startup(&self) {
        self.parent_startup();

        let window = SimpleWindow::new(&self.obj());
        self.window
            .set(window)
            .expect("Failed to initialize application window");
    }
}

impl GtkApplicationImpl for SimpleApplication {}
