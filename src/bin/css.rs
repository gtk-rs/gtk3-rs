//! # CSS example
//!
//! This sample demonstrates how to use CSS with gtk-rs.

extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

// Basic CSS: we change background color, we set font color to black and we set it as bold.
const STYLE: &str = "
#entry1 {
    background-image: -gtk-gradient (linear,
                                     0 0, 1 0,
                                     color-stop(0, #f00),
                                     color-stop(1, #0f0));
    color: blue;
    font-weight: bold;
}

button {
    /* If we don't put it, the yellow background won't be visible */
    background-image: none;
}
#label1:hover {
    transition: 500ms;
    color: red;
    background-color: yellow;
}

combobox button.combo box {
    padding: 5px;
}
combobox box arrow {
    -gtk-icon-source: none;
    border-left: 5px solid transparent;
    border-right: 5px solid transparent;
    border-top: 5px solid black;
}";

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("CSS");
    window.set_position(gtk::WindowPosition::Center);

    // The container container.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let label = gtk::Button::new_with_label("hover me!");
    // We need to name it in order to be able to use its name as a CSS label to
    // apply CSS on it.
    gtk::WidgetExt::set_widget_name(&label, "label1");

    let entry = gtk::Entry::new();
    // We need to name it in order to apply CSS on it.
    gtk::WidgetExt::set_widget_name(&entry, "entry1");
    entry.set_text("Some text");

    let combo = gtk::ComboBoxText::new();
    combo.append_text("option 1");
    combo.append_text("option 2");
    combo.append_text("option 3");
    combo.set_active(Some(0));

    vbox.add(&label);
    vbox.add(&entry);
    vbox.add(&combo);
    // Then we add the container inside our window.
    window.add(&vbox);

    application.connect_activate(move |_| {
        window.show_all();
    });
}

fn main() {
    let application = gtk::Application::new(Some("com.github.css"), gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // We build the application UI.
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
