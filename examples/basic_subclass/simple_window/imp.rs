use glib::clone;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::unsync::OnceCell;
use std::cell::Cell;

#[derive(Debug)]
struct WindowWidgets {
    headerbar: gtk::HeaderBar,
    increment: gtk::Button,
    label: gtk::Label,
}

#[derive(Debug, Default)]
pub struct SimpleWindow {
    widgets: OnceCell<WindowWidgets>,
    counter: Cell<u64>,
}

#[glib::object_subclass]
impl ObjectSubclass for SimpleWindow {
    const NAME: &'static str = "SimpleWindow";
    type Type = super::SimpleWindow;
    type ParentType = gtk::ApplicationWindow;
}

impl ObjectImpl for SimpleWindow {
    // Here we are overriding the glib::Objcet::contructed
    // method. Its what gets called when we create our Object
    // and where we can initialize things.
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);

        let headerbar = gtk::HeaderBar::new();
        let increment = gtk::Button::with_label("Increment!");
        let label = gtk::Label::new(Some("Press the Increment Button!"));

        headerbar.set_title(Some("Hello World!"));
        headerbar.set_show_close_button(true);
        headerbar.pack_start(&increment);

        // Connect our method `on_increment_clicked` to be called
        // when the increment button is clicked.
        increment.connect_clicked(clone!(@weak obj => move |_| {
            let priv_ = SimpleWindow::from_instance(&obj);
            priv_.on_increment_clicked();
        }));

        obj.add(&label);
        obj.set_titlebar(Some(&headerbar));
        obj.set_default_size(640, 480);

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
