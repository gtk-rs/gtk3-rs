use gtk::prelude::*;
use gtk::{gdk, gio};

fn main() {
    let application = gtk::Application::new(Some("com.github.css"), gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = gtk::CssProvider::new();
        // Load the CSS file
        let style = include_bytes!("style.css");
        provider.load_from_data(style).expect("Failed to load CSS");
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

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("CSS");
    window.set_position(gtk::WindowPosition::Center);

    // The container container.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let label = gtk::Button::with_label("hover me!");
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
