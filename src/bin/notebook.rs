extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{IconSize, Orientation, ReliefStyle, Widget};

use std::env::args;

struct Notebook {
    notebook: gtk::Notebook,
    tabs: Vec<gtk::Box>,
}

impl Notebook {
    fn new() -> Notebook {
        Notebook {
            notebook: gtk::Notebook::new(),
            tabs: Vec::new(),
        }
    }

    fn create_tab(&mut self, title: &str, widget: Widget) -> u32 {
        let close_image = gtk::Image::new_from_icon_name(Some("window-close"), IconSize::Button);
        let button = gtk::Button::new();
        let label = gtk::Label::new(Some(title));
        let tab = gtk::Box::new(Orientation::Horizontal, 0);

        button.set_relief(ReliefStyle::None);
        button.set_focus_on_click(false);
        button.add(&close_image);

        tab.pack_start(&label, false, false, 0);
        tab.pack_start(&button, false, false, 0);
        tab.show_all();

        let index = self.notebook.append_page(&widget, Some(&tab));

        let notebook = &self.notebook;
        button.connect_clicked(clone!(@weak notebook => move |_| {
            let index = notebook
                .page_num(&widget)
                .expect("Couldn't get page_num from notebook");
            notebook.remove_page(Some(index));
        }));

        self.tabs.push(tab);

        index
    }
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Notebook");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    let mut notebook = Notebook::new();

    for i in 1..4 {
        let title = format!("sheet {}", i);
        let label = gtk::Label::new(Some(&*title));
        notebook.create_tab(&title, label.upcast());
    }

    window.add(&notebook.notebook);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.notebook"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
