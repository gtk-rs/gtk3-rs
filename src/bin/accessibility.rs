//! # Accessibility example
//!
//! This sample demonstrates how to make an application more accessible.

extern crate atk;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use atk::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Accessibility");
    window.set_position(gtk::WindowPosition::Center);

    let button = gtk::Button::new_with_label("Click me!");
    let label = gtk::Label::new("0");
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    if let (Some(button_obj), Some(label_obj)) = (button.get_accessible(), label.get_accessible()) {
        // We set the description
        button_obj.set_description("Button to increase label value");

        // Then we setup the relation saying that the label is linked to the button.
        let relation_set = label_obj.ref_relation_set().expect("Failed to get relation for label");
        let relation = atk::Relation::new(&[button_obj], atk::RelationType::LabelFor);

        relation_set.add(&relation);
    }

    vbox.add(&button);
    vbox.add(&label);

    window.add(&vbox);

    button.connect_clicked(move |_| {
        let value = label.get_text().unwrap_or_else(|| "0".to_owned());
        let value = u32::from_str_radix(&value, 10).unwrap_or_else(|_| 0) + 1;
        label.set_text(&value.to_string());
    });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.github.accessibility",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_activate(|app| {
        // We build the application UI.
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
