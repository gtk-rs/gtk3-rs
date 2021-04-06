use gtk::glib;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, MessageDialog};

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("builder_signal.ui");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    let dialog: MessageDialog = builder
        .get_object("messagedialog1")
        .expect("Couldn't get messagedialog1");
    dialog.connect_delete_event(|dialog, _| {
        dialog.hide();
        gtk::Inhibit(true)
    });

    builder.connect_signals(move |_, handler_name| {
        // This is the one-time callback to register signals.
        // Here we map each handler name to its handler.

        if handler_name == "button1_clicked" {
            // Return the signal handler.
            Box::new(
                glib::clone!(@weak dialog => @default-return None, move |_| {
                    dialog.show_all();
                    None
                }),
            )
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

    application.connect_activate(build_ui);

    application.run();
}
