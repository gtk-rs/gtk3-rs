//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

extern crate gio;
extern crate gtk;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;

// upgrade weak reference or return
#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("text_viewer.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).expect("Couldn't add from string");

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(application);
    let open_button: gtk::ToolButton = builder.get_object("open_button")
                                              .expect("Couldn't get builder");
    let text_view: gtk::TextView = builder.get_object("text_view")
                                          .expect("Couldn't get text_view");

    let window_weak = window.downgrade();
    open_button.connect_clicked(move |_| {
        let window = upgrade_weak!(window_weak);

        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"), Some(&window), gtk::FileChooserAction::Open);
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok.into()),
            ("Cancel", gtk::ResponseType::Cancel.into()),
        ]);
        if file_chooser.run() == gtk::ResponseType::Ok.into() {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            let file = File::open(&filename).expect("Couldn't open file");

            let mut reader = BufReader::new(file);
            let mut contents = String::new();
            let _ = reader.read_to_string(&mut contents);

            text_view.get_buffer().expect("Couldn't get window").set_text(&contents);
        }

        file_chooser.destroy();
    });

    window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.text_viewer",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
