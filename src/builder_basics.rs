//! # Builder Basics Sample
//!
//! This sample demonstrates how to use the builder with a simple glade file

extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod example {
    use gtk;
    use gtk::traits::*;
    use gtk::signal::Inhibit;
    use gtk::widgets::Builder;
    use gtk::Window;

    pub fn main() {
        gtk::init();
        let builder = Builder::new_from_file("./builder_basics.glade").unwrap();
        let window: Window = builder.get_object("window1").unwrap();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(true)
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

