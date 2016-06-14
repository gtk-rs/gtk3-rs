extern crate gtk;

use gtk::prelude::*;
use gtk::{IconSize, Orientation, ReliefStyle, Widget};

struct Notebook {
    notebook: gtk::Notebook,
    tabs: Vec<gtk::Box>
}

impl Notebook {
    fn new() -> Notebook {
        Notebook {
            notebook: gtk::Notebook::new(),
            tabs: Vec::new()
        }
    }

    fn create_tab(&mut self, title: &str, widget: Widget) -> u32 {
        let close_image = gtk::Image::new_from_icon_name("window-close",
                                                         IconSize::Button.into());
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

        let notebook_clone = self.notebook.clone();
        button.connect_clicked(move |_| {
            let index = notebook_clone.page_num(&widget).unwrap();
            notebook_clone.remove_page(Some(index));
        });

        self.tabs.push(tab);

        index
    }
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Notebook");
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(640, 480);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let mut notebook = Notebook::new();

    for i in 1..4 {
        let title = format!("sheet {}", i);
        let label = gtk::Label::new(Some(&title));
        notebook.create_tab(&title, label.upcast());
    }

    window.add(&notebook.notebook);
    window.show_all();
    gtk::main();
}

