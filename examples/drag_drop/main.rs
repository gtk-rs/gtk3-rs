use gtk::gdk;
use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    // Configure button as drag source for text
    let button = gtk::Button::with_label("Drag here");
    let targets = vec![
        gtk::TargetEntry::new("STRING", gtk::TargetFlags::SAME_APP, 0),
        gtk::TargetEntry::new("text/plain", gtk::TargetFlags::SAME_APP, 0),
    ];
    button.drag_source_set(
        gdk::ModifierType::MODIFIER_MASK,
        &targets,
        gdk::DragAction::COPY,
    );
    button.connect_drag_data_get(|_, _, s, _, _| {
        let data = "I'm data!";
        s.set_text(data);
    });

    // Configure label as drag destination to receive text
    let label = gtk::Label::new(Some("Drop here"));
    label.drag_dest_set(gtk::DestDefaults::ALL, &targets, gdk::DragAction::COPY);
    label.connect_drag_data_received(|w, _, _, _, s, _, _| {
        w.set_text(&s.get_text().expect("Couldn't get text"));
    });

    // Stack the button and label horizontally
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    hbox.pack_start(&button, true, true, 0);
    hbox.pack_start(&label, true, true, 0);

    // Finish populating the window and display everything
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Simple Drag and Drop Example");
    window.set_default_size(200, 100);
    window.add(&hbox);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.drag_and_drop"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(build_ui);

    application.run();
}
