//! # Builder Basics Sample
//!
//! This sample demonstrates how to use the builder with an imported glade file

extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod example {
    use gtk;
    use gtk::prelude::*;
    use gtk::{Builder, Button, MessageDialog, Window};


    pub fn main() {
        if gtk::init().is_err() {
            println!("Failed to initialize GTK.");
            return;
        }
        let glade_src = include_str!("builder_basics.glade");
        let builder = Builder::new_from_string(glade_src);

        let window: Window = builder.get_object("window1").unwrap();
        let bigbutton: Button = builder.get_object("button1").unwrap();
        let dialog: MessageDialog = builder.get_object("messagedialog1").unwrap();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        bigbutton.connect_clicked(move |_| {
            dialog.run();
            dialog.hide();
        });

        window.show_all();

        gtk::main();
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    example::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This example only work with GTK 3.10 and later");
    println!("Did you forget to build with `--features gtk_3_10`?");
}

