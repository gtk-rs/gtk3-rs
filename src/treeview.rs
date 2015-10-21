//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with either a ListStore or TreeStore.

extern crate glib;
extern crate gtk;

use gtk::traits::*;
use gtk::signal::Inhibit;

fn append_text_column(tree: &gtk::TreeView) {
    let column = gtk::TreeViewColumn::new().unwrap();
    let cell = gtk::CellRendererText::new().unwrap();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = gtk::Window::new(gtk::WindowType::Toplevel).unwrap();

    window.set_title("TreeView Sample");
    window.set_window_position(gtk::WindowPosition::Center);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // test Value

    let hello = String::from("Hello world !");
    let mut value = glib::Value::new();

    value.init(glib::Type::String);
    value.set(&hello);
    println!("gvalue.get example : {}", value.get::<String>());

    // left pane

    let left_tree = gtk::TreeView::new().unwrap();
    let column_types = [glib::Type::String];
    let left_store = gtk::ListStore::new(&column_types).unwrap();
    let left_model = left_store.get_model().unwrap();

    left_tree.set_model(&left_model);
    left_tree.set_headers_visible(false);
    append_text_column(&left_tree);

    // print out when a row is selected

    let left_selection = left_tree.get_selection().unwrap();
    left_selection.connect_changed(|tree_selection| {
        let (left_model, iter) = tree_selection.get_selected().unwrap();
        if let Some(path) = left_model.get_path(&iter) {
            println!("selected row {}", path.to_string().unwrap());
        }
    });

    for _ in 0..10 {
        let iter = left_store.append();
        left_store.set_string(&iter, 0, "I'm in a list");

        // select this row as a test

        if let Some(path) = left_model.get_path(&iter) {
            left_selection.select_path(&path);
        }
    }

    // right pane

    let right_tree = gtk::TreeView::new().unwrap();
    let column_types = [glib::Type::String];
    let right_store = gtk::TreeStore::new(&column_types).unwrap();
    let right_model = right_store.get_model().unwrap();

    right_tree.set_model(&right_model);
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

    let split_pane = gtk::Box::new(gtk::Orientation::Horizontal, 10).unwrap();

    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&right_tree);

    window.add(&split_pane);
    window.show_all();
    gtk::main();
}
