//! # Tree Model Sort example
//!
//! This sample demonstrates how to use the `TreeModelSort` widget.

use gtk::prelude::*;
use gtk::{gio, glib};
use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Tree Model Sort Window");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let store = gtk::TreeStore::new(&[glib::Type::STRING]);
    store.insert_with_values(None, None, &[(0, &"One")]);
    store.insert_with_values(None, None, &[(0, &"Two")]);
    store.insert_with_values(None, None, &[(0, &"Three")]);
    store.insert_with_values(None, None, &[(0, &"Four")]);

    // We create the `TreeModelSort` and we give it the `TreeStore` as
    // parameter.
    let sortable_store = gtk::TreeModelSort::new(&store);

    // Then we create the `TreeView` from the `TreeModelSort`.
    let treeview = gtk::TreeView::with_model(&sortable_store);

    let column = gtk::TreeViewColumn::new();
    column.set_title("Value");
    column.set_clickable(true);
    column.set_sort_indicator(true);
    column.set_sort_column_id(0);

    let renderer = gtk::CellRendererText::new();
    column.pack_end(&renderer, true);
    column.add_attribute(&renderer, "text", 0);

    treeview.append_column(&column);

    treeview.connect_row_activated(move |_, path, _column| {
        let real_path = sortable_store
            .convert_path_to_child_path(&path)
            .expect("Sorted path does not correspond to real path");
        println!(
            "Clicked on sorted: {:?}, real: {:?}",
            path.get_indices(),
            real_path.get_indices()
        );
    });

    // We finally add the `TreeView` to the window.
    window.add(&treeview);
    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.basic"), gio::ApplicationFlags::empty())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&args().collect::<Vec<_>>());
}
