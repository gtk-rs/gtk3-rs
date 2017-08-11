//! # Child Properties
//!
//! This sample demonstrates how to set child properties.

#![crate_type = "bin"]

extern crate gtk;

use gtk::{BoxExt, Button, ContainerExt, Inhibit, Label, PackType, WidgetExt, Window, WindowType};
use gtk::Orientation::Vertical;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let vbox = gtk::Box::new(Vertical, 0);

    let plus_button = Button::new_with_label("+");
    vbox.add(&plus_button);
    // Set some child properties.
    // These calls need to be added after the Widget is added to the Box.
    vbox.set_child_expand(&plus_button, true);
    vbox.set_child_fill(&plus_button, true);
    vbox.set_child_padding(&plus_button, 50);
    vbox.set_child_pack_type(&plus_button, PackType::End);

    let counter_label = Label::new("0");
    vbox.add(&counter_label);

    let minus_button = Button::new_with_label("-");
    vbox.add(&minus_button);

    let window = Window::new(WindowType::Toplevel);

    window.add(&vbox);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.show_all();

    gtk::main();
}
