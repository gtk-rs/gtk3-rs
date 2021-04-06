use gtk::prelude::*;
use gtk::{gdk, gio};
use gtk::{DestDefaults, TargetFlags};

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Drag and Drop Example with a TextView");

    // Give a nice text description for the user
    let label = gtk::Label::new(Some("Drag files and/or folders onto the TextView below."));

    // Create scrollable text view as our drag target
    let text_view = gtk::TextView::new();
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_cursor_visible(false);
    let scrolled_text_view = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    scrolled_text_view.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_text_view.add(&text_view);

    // Configure the text view to accept URI lists from other applications. This allows
    // dragging files & folders from a file browser program onto the textview.
    let targets = vec![gtk::TargetEntry::new(
        "text/uri-list",
        TargetFlags::OTHER_APP,
        0,
    )];
    text_view.drag_dest_set(DestDefaults::HIGHLIGHT, &targets, gdk::DragAction::COPY);

    // Process any `drag-data-received` events received by the textview. These events include
    // the URL list we're looking for.
    text_view.connect_drag_data_received(|w, _, _, _, d, _, _| {
        // Get the text buffer for the TextView and clear it to make it ready to accept new text.
        let buffer = w.get_buffer().unwrap();
        buffer.set_text("");

        // Since we only accept `text/uri-list`s here, we don't need to check first, we can simply
        // iterate through all of the accepted URIs.
        for file in d.get_uris() {
            let file = gio::File::new_for_uri(&file);
            let display_name = if file.is_native() {
                file.get_path().unwrap().display().to_string()
            } else {
                file.get_uri().into()
            };
            let bulleted_file_path = format!(" • {}\n", &display_name);
            // We make sure to always insert this at the end of the text buffer so they're in
            // order.
            buffer.insert_at_cursor(&bulleted_file_path);
        }
    });

    // Pack widgets vertically.
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&label, false, false, 0);
    vbox.pack_start(&scrolled_text_view, true, true, 0);

    // Create a new window
    window.add(&vbox);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.drag_and_drop_textview"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(build_ui);

    application.run();
}
