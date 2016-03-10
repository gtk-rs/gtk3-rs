//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with either a ListStore or TreeStore.

extern crate glib;
extern crate gtk;

use gtk::prelude::*;
use gtk::{
    CellRendererText, TreeStore, TreeView, TreeViewColumn,
    Window, WindowPosition, WindowType
};

fn append_column(tree: &TreeView, id: i32) {
    let column = TreeViewColumn::new();
    let cell = CellRendererText::new();

    column.pack_start(&cell, true);
    // we create the column at the position `id`
    column.add_attribute(&cell, "text", id);
    tree.append_column(&column);
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);

    window.set_title("Simple TreeView example");
    window.set_position(WindowPosition::Center);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // we create a view
    let tree = TreeView::new();
    // we create a model with two rows
    let store = TreeStore::new(&[String::static_type(), i32::static_type()]);

    // when a line is selected, this signal is called
    tree.connect_cursor_changed(move |tree_view| {
        let selection = tree_view.get_selection();
        if let Some((model, iter)) = selection.get_selected() {
            // We are now getting back the values from the line corresponding to the
            // iterator `iter`.
            //
            // The `get_value` method do the conversion between the gtk type and Rust.
            println!("Hello '{}' from line {}",
                     model.get_value(&iter, 0).get::<String>().unwrap(),
                     model.get_value(&iter, 1).get::<i32>().unwrap() + 1);
        }
    });

    // we set the model into the view
    tree.set_model(Some(&store));
    tree.set_headers_visible(false);
    // we create the two columns inside the view
    append_column(&tree, 0);
    append_column(&tree, 1);

    // we fill the treeview
    let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
    for (i, entry) in entries.iter().enumerate() {
        store.insert_with_values(None, None, &[0, 1], &[&entry, &(i as i32)]);
    }

    // we add the view to the window
    window.add(&tree);

    window.show_all();
    gtk::main();
}
