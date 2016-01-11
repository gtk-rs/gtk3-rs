//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with either a ListStore or TreeStore.

extern crate glib;
extern crate gtk;

use gtk::prelude::*;

fn append_text_column(tree: &gtk::TreeView) {
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel);

    window.set_title("TreeView Sample");
    window.set_window_position(gtk::WindowPosition::Center);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // test Value

    let hello = String::from("Hello world !");
    let value = unsafe {
        let mut value = glib::Value::new();
        value.init(glib::Type::String);
        value.set(&hello);
        println!("gvalue.get example : {}", value.get::<String>());
        value
    };

    // left pane

    let left_tree = gtk::TreeView::new();
    let column_types = [glib::Type::String];
    let left_store = gtk::ListStore::new(&column_types);

    left_tree.set_model(Some(&left_store));
    left_tree.set_headers_visible(false);
    append_text_column(&left_tree);

    // print out when a row is selected

    let left_selection = left_tree.get_selection().unwrap();
    left_selection.connect_changed(|tree_selection| {
        let (left_store, iter) = tree_selection.get_selected().unwrap();
        println!("selected row {}", left_store.get_path(&iter));
    });

    for _ in 0..10 {
        let iter = left_store.append();
        left_store.set_string(&iter, 0, "I'm in a list");

        // select this row as a test
        //
        left_selection.select_path(&left_store.get_path(&iter));
    }

    // right pane

    let right_tree = gtk::TreeView::new();
    let column_types = [glib::Type::String];
    let right_store = gtk::TreeStore::new(&column_types);

    right_tree.set_model(Some(&right_store));
    right_tree.set_headers_visible(false);
    append_text_column(&right_tree);

    for i in 0..10 {
        let iter = right_store.append(None);
        right_store.set_value(&iter, 0, &value);

        for _ in 0..i {
            let child_iter = right_store.append(Some(&iter));
            right_store.set_string(&child_iter, 0, "I'm a child node");
        }
    }

    // display the panes

    let split_pane = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&right_tree);

    window.add(&split_pane);
    window.show_all();
    gtk::main();
}
