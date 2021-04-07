use gtk::prelude::*;
use gtk::{glib, Builder};

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("sync_widgets.ui");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Couldn't add from string");

    let slider: gtk::Scale = builder.get_object("slider").expect("Couldn't get slider");
    let spin_button: gtk::SpinButton = builder
        .get_object("spin_button")
        .expect("Couldn't get spin_button");
    let slider_adj = slider.get_adjustment();
    let spin_button_adj = spin_button.get_adjustment();
    slider_adj
        .bind_property("value", &spin_button_adj, "value")
        .flags(
            glib::BindingFlags::DEFAULT
                | glib::BindingFlags::SYNC_CREATE
                | glib::BindingFlags::BIDIRECTIONAL,
        )
        .build();

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.sync_widgets"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(build_ui);

    application.run();
}
