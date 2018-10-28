//! # Child Properties
//!
//! This sample demonstrates how to set child properties.

#![crate_type = "bin"]

extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Label, PackType};
use gtk::Orientation::Vertical;

use std::env::args;
use std::str::FromStr;

// make moving clones into closures more convenient
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn build_ui(application: &gtk::Application) {
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

    minus_button.connect_clicked(clone!(counter_label => move |_| {
        let nb = u32::from_str(counter_label.get_text()
                                           .unwrap_or("0".to_owned())
                                           .as_str()).unwrap_or(0);
        if nb > 0 {
            counter_label.set_text(&format!("{}", nb - 1));
        }
    }));
    plus_button.connect_clicked(clone!(counter_label => move |_| {
        let nb = u32::from_str(counter_label.get_text()
                                           .unwrap_or("0".to_owned())
                                           .as_str()).unwrap_or(0);
        counter_label.set_text(&format!("{}", nb + 1));
    }));

    let window = ApplicationWindow::new(application);

    window.set_default_size(200, 200);
    window.add(&vbox);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.gtk-rs.examples.child_properties",
                                            Default::default())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
