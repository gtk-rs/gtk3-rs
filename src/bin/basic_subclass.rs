//! # Basic Subclass example
//!
//! This file creates a `GtkApplication` and a `GtkApplicationWindow` subclass
//! and showcases how you can override virtual funcitons such as `startup`
//! and `activate` and how to interact with the GObjects and their private
//! structs.

#[macro_use]
extern crate glib;
extern crate gio;
extern crate gtk;

extern crate once_cell;

use gio::prelude::*;
use gtk::prelude::*;

use gio::subclass::prelude::*;
use gio::ApplicationFlags;
use glib::subclass;
use gtk::subclass::prelude::*;

use once_cell::unsync::OnceCell;
use std::cell::Cell;

#[derive(Debug)]
struct WindowWidgets {
    headerbar: gtk::HeaderBar,
    increment: gtk::Button,
    label: gtk::Label,
}

mod imp_win {
    use super::*;

    // This is the private part of our `SimpleWindow` object.
    // Its where state and widgets are stored when they don't
    // need to be publicly accesible.
    #[derive(Debug)]
    pub struct SimpleWindow {
        widgets: OnceCell<WindowWidgets>,
        counter: Cell<u64>,
    }

    impl ObjectSubclass for SimpleWindow {
        const NAME: &'static str = "SimpleWindow";
        type ParentType = gtk::ApplicationWindow;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new() -> Self {
            Self {
                widgets: OnceCell::new(),
                counter: Cell::new(0),
            }
        }
    }

    impl ObjectImpl for SimpleWindow {
        // Here we are overriding the glib::Objcet::contructed
        // method. Its what gets called when we create our Object
        // and where we can initialize things.
        fn constructed(&self, obj: &glib::Object) {
            self.parent_constructed(obj);

            let self_ = obj.downcast_ref::<super::SimpleWindow>().unwrap();

            let headerbar = gtk::HeaderBar::new();
            let increment = gtk::Button::with_label("Increment!");
            let label = gtk::Label::new(Some("Press the Increment Button!"));

            headerbar.set_title(Some("Hello World!"));
            headerbar.set_show_close_button(true);
            headerbar.pack_start(&increment);

            // Connect our method `on_increment_clicked` to be called
            // when the increment button is clicked.
            increment.connect_clicked(clone!(@weak self_ => move |_| {
                let priv_ = SimpleWindow::from_instance(&self_);
                priv_.on_increment_clicked();
            }));

            self_.add(&label);
            self_.set_titlebar(Some(&headerbar));
            self_.set_default_size(640, 480);

            self.widgets
                .set(WindowWidgets {
                    headerbar,
                    label,
                    increment,
                })
                .expect("Failed to initialize window state");
        }
    }

    impl SimpleWindow {
        fn on_increment_clicked(&self) {
            self.counter.set(self.counter.get() + 1);
            let w = self.widgets.get().unwrap();
            w.label
                .set_text(&format!("Counter is {}", self.counter.get()));
        }
    }

    impl WidgetImpl for SimpleWindow {}
    impl ContainerImpl for SimpleWindow {}
    impl BinImpl for SimpleWindow {}
    impl WindowImpl for SimpleWindow {}
    impl ApplicationWindowImpl for SimpleWindow {}
}

glib_wrapper! {
    pub struct SimpleWindow(ObjectSubclass<imp_win::SimpleWindow, SimpleWindowClass>)
        @extends gtk::Widget, gtk::Container, gtk::Bin, gtk::Window, gtk::ApplicationWindow;
}

impl SimpleWindow {
    pub fn new(app: &gtk::Application) -> Self {
        glib::Object::new(Self::static_type(), &[("application", app)])
            .expect("Failed to create SimpleWindow")
            .downcast::<SimpleWindow>()
            .expect("Created SimpleWindow is of wrong type")
    }
}

mod imp_app {
    use super::*;

    #[derive(Debug)]
    pub struct SimpleApplication {
        window: OnceCell<SimpleWindow>,
    }

    impl ObjectSubclass for SimpleApplication {
        const NAME: &'static str = "SimpleApplication";
        type ParentType = gtk::Application;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        fn new() -> Self {
            Self {
                window: OnceCell::new(),
            }
        }
    }

    impl ObjectImpl for SimpleApplication {}

    // When our application starts, the `startup` signal will be fired.
    // This gives us a chance to perform initialisation tasks that are not directly
    // related to showing a new window. After this, depending on how
    // the application is started, either `activate` or `open` will be called next.
    impl ApplicationImpl for SimpleApplication {
        // `gio::Application::activate` is what gets called when the
        // application is launched by the desktop environment and
        // aksed to present itself.
        fn activate(&self, app: &gio::Application) {
            let app = app.downcast_ref::<gtk::Application>().unwrap();
            let priv_ = SimpleApplication::from_instance(app);
            let window = priv_
                .window
                .get()
                .expect("Should always be initiliazed in gio_application_startup");
            window.show_all();
            window.present();
        }

        // `gio::Application` is bit special. It does not get initialized
        // when `new` is called and the object created, but rather
        // once the `startup` signal is emitted and the `gio::Application::startup`
        // is called.
        //
        // Due to this, we create and initialize the `SimpleWindow` widget
        // here. Widgets can't be created before `startup` has been called.
        fn startup(&self, app: &gio::Application) {
            self.parent_startup(app);

            let app = app.downcast_ref::<gtk::Application>().unwrap();
            let priv_ = SimpleApplication::from_instance(app);
            let window = SimpleWindow::new(&app);
            priv_
                .window
                .set(window)
                .expect("Failed to initialize application window");
        }
    }

    impl GtkApplicationImpl for SimpleApplication {}
}

glib_wrapper! {
    pub struct SimpleApplication(ObjectSubclass<imp_app::SimpleApplication, SimpleApplicationClass>)
        @extends gio::Application, gtk::Application;
}

impl SimpleApplication {
    pub fn new() -> Self {
        glib::Object::new(
            Self::static_type(),
            &[
                ("application-id", &"org.gtk-rs.SimpleApplication"),
                ("flags", &ApplicationFlags::empty()),
            ],
        )
        .expect("Failed to create SimpleApp")
        .downcast()
        .expect("Created simpleapp is of wrong type")
    }
}

fn main() {
    gtk::init().expect("Failed to initialize gtk");

    let app = SimpleApplication::new();

    let args: Vec<String> = std::env::args().collect();
    app.run(&args);
}
