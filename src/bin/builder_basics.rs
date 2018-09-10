//! # Builder Basics Sample
//!
//! This sample demonstrates how to use the builder with an imported glade file

extern crate gio;
extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod example {
    use gio;
    use gtk;

    use gio::prelude::*;
    use gtk::prelude::*;

    use gtk::{ApplicationWindow, Builder, Button, MessageDialog};

    use std::env::args;


    pub fn build_ui(application: &gtk::Application) {
        let glade_src = include_str!("builder_basics.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
        let bigbutton: Button = builder.get_object("button1").expect("Couldn't get button1");
        let dialog: MessageDialog = builder.get_object("messagedialog1")
                                           .expect("Couldn't get messagedialog1");

        window.set_application(application);
        window.connect_delete_event(move |win, _| {
            win.destroy();
            Inhibit(false)
        });

        bigbutton.connect_clicked(move |_| {
            dialog.run();
            dialog.hide();
        });

        window.show_all();
    }

    pub fn main() {
        let application = gtk::Application::new("com.github.builder_basics",
                                                gio::ApplicationFlags::empty())
                                           .expect("Initialization failed...");

        application.connect_startup(move |app| {
            build_ui(app);
        });
        application.connect_activate(|_| {});

        application.run(&args().collect::<Vec<_>>());
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    example::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This example requires GTK 3.10 or later");
    println!("Did you forget to build with `--features gtk_3_10`?");
}
