//! # Toolbar, Scrollable Text View and File Chooser
//!
//! A simple text file viewer

extern crate gtk;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

use gtk::prelude::*;
use gtk::Builder;

pub fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("text_viewer.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).expect("Couldn't add from string");

    let window: gtk::Window = builder.get_object("window").expect("Couldn't get window");
    let open_button: gtk::ToolButton = builder.get_object("open_button")
                                              .expect("Couldn't get builder");
    let text_view: gtk::TextView = builder.get_object("text_view")
                                          .expect("Couldn't get text_view");

    let window1 = window.clone();
    open_button.connect_clicked(move |_| {
        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"), Some(&window1), gtk::FileChooserAction::Open);
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

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();
    gtk::main();
}
