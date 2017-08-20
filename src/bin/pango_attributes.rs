//! # Pango text attributes
//!
//! This sample demonstrates how to use various attributes on labels text.

#![crate_type = "bin"]

extern crate gtk;
extern crate pango;

use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("Pango text attributes");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let label = gtk::Label::new("Some text");
    let attr_list = pango::AttrList::new();

    let mut attr = pango::Attribute::new_background(65535, 0, 0)
                                    .expect("Couldn't create new background");
    attr.set_start_index(0);
    attr.set_end_index(2);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_underline(pango::Underline::Single)
                                    .expect("Couldn't create new underline");
    attr.set_start_index(1);
    attr.set_end_index(4);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_strikethrough(true)
                                    .expect("Couldn't create new strikethrough");
    attr.set_start_index(5);
    attr_list.insert(attr);

    let mut attr = pango::Attribute::new_scale(1.2)
                                    .expect("Couldn't create new scale");
    attr.set_start_index(6);
    attr_list.insert(attr);

    label.set_attributes(&attr_list);
    window.add(&label);

    window.show_all();
    gtk::main();
}
