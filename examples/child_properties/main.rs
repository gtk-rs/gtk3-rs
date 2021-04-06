use gtk::glib;
use gtk::prelude::*;
use gtk::Orientation::Vertical;
use gtk::{ApplicationWindow, Button, Label, PackType};

fn build_ui(application: &gtk::Application) {
    let vbox = gtk::Box::new(Vertical, 0);

    let plus_button = Button::with_label("+");
    vbox.add(&plus_button);
    // Set some child properties.
    // These calls need to be added after the Widget is added to the Box.
    vbox.set_child_expand(&plus_button, true);
    vbox.set_child_fill(&plus_button, true);
    vbox.set_child_padding(&plus_button, 50);
    vbox.set_child_pack_type(&plus_button, PackType::End);

    let counter_label = Label::new(Some("0"));
    vbox.add(&counter_label);

    let minus_button = Button::with_label("-");
    vbox.add(&minus_button);

    minus_button.connect_clicked(glib::clone!(@weak counter_label => move |_| {
        let nb = counter_label.get_text()
            .parse()
            .unwrap_or(0);
        if nb > 0 {
            counter_label.set_text(&format!("{}", nb - 1));
        }
    }));
    plus_button.connect_clicked(glib::clone!(@weak counter_label => move |_| {
        let nb = counter_label.get_text()
            .parse()
            .unwrap_or(0);
        counter_label.set_text(&format!("{}", nb + 1));
    }));

    let window = ApplicationWindow::new(application);

    window.set_default_size(200, 200);
    window.add(&vbox);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.child_properties"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(build_ui);

    application.run();
}
