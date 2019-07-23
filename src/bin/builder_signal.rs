//! # Builder Signal Sample
//!
//! This sample demonstrates how to handle signals in builder

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, MessageDialog};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("builder_signal.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    let dialog: MessageDialog = builder
        .get_object("messagedialog1")
        .expect("Couldn't get messagedialog1");

    builder.connect_signals(move |_, handler_name| {
        // This is the one-time callback to register signals.
        // Here we map each handler name to its handler.

        if handler_name == "button1_clicked" {
            let dialog = dialog.clone();

            // Return the signal handler.
            Box::new(move |_| {
                dialog.run();
                dialog.hide();
                None
            })
        } else {
            panic!("Unknown handler name {}", handler_name)
        }
    });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_signal"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
