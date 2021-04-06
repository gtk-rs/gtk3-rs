//! # Overlay example
//!
//! This example demonstrates how to create an element "floating" above others.

use gtk::prelude::*;
use gtk::{gdk, gio, glib};

// Basic CSS: we change background color, we set font color to black and we set it as bold.
const STYLE: &str = "
#overlay-label {
    background-color: rgba(192, 192, 192, 0.8);
    color: black;
    font-weight: bold;
}";

fn button_clicked(button: &gtk::Button, overlay_text: &gtk::Label) {
    overlay_text.set_text(&button.get_label().expect("Couldn't get button label"));
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Overlay");
    window.set_position(gtk::WindowPosition::Center);

    // The overlay container.
    let overlay = gtk::Overlay::new();

    // The overlay label.
    let overlay_text = gtk::Label::new(Some("0"));
    // We need to name it in order to apply CSS on it.
    gtk::WidgetExt::set_widget_name(&overlay_text, "overlay-label");
    // We put the overlay in the top-right corner of the window.
    overlay_text.set_halign(gtk::Align::End);
    overlay_text.set_valign(gtk::Align::Start);

    // We add into the overlay container as the overlay element.
    overlay.add_overlay(&overlay_text);

    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let but1 = gtk::Button::with_label("Click me!");
    let but2 = gtk::Button::with_label("Or me!");
    let but3 = gtk::Button::with_label("Why not me?");

    // When a button is clicked on, we set its label to the overlay label.
    but1.connect_clicked(glib::clone!(@weak overlay_text => move |b| {
        button_clicked(b, &overlay_text);
    }));
    but2.connect_clicked(glib::clone!(@weak overlay_text => move |b| {
        button_clicked(b, &overlay_text);
    }));
    but3.connect_clicked(glib::clone!(@weak overlay_text => move |b| {
        button_clicked(b, &overlay_text);
    }));

    hbox.add(&but1);
    hbox.add(&but2);
    hbox.add(&but3);

    // We add the horizontal box into the overlay container "normally" (so this won't be an overlay
    // element).
    overlay.add(&hbox);
    // Then we add the overlay container inside our window.
    window.add(&overlay);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.overlay"), gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

    application.connect_startup(|_| {
        // We add a bit of CSS in order to make the overlay label easier to be seen.
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });

    application.connect_activate(|app| {
        // We build the application UI.
        build_ui(app);
    });

    application.run();
}
