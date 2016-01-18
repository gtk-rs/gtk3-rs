//! # TreeView Sample
//!
//! This sample demonstrates how to create a TreeView with either a ListStore or TreeStore.

extern crate glib;
extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use glib::types::StaticType;

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
    window.set_position(gtk::WindowPosition::Center);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

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
        left_store.set_value(&iter, 0, &"I'm in a list".into());

        // select this row as a test
        //
        left_selection.select_path(&left_store.get_path(&iter));
    }

    // middle pane

    let middle_tree = gtk::TreeView::new();
    let column_types = [glib::Type::String];
    let middle_store = gtk::TreeStore::new(&column_types);

    middle_tree.set_model(Some(&middle_store));
    middle_tree.set_headers_visible(false);
    append_text_column(&middle_tree);

    for i in 0..10 {
        let iter = middle_store.append(None);
        middle_store.set_value(&iter, 0, &format!("Hello {}", i).into());

        for _ in 0..i {
            let child_iter = middle_store.append(Some(&iter));
            middle_store.set_value(&child_iter, 0, &"I'm a child node".into());
        }
    }

    // right pane
    let right_tree = gtk::TreeView::new();
    let right_column_types = [gdk::pixbuf::Pixbuf::static_type(), glib::Type::String];
    let right_store = gtk::TreeStore::new(&right_column_types);
    let renderer = gtk::CellRendererPixbuf::new();
    let col = gtk::TreeViewColumn::new();

    col.set_title("Picture");
    col.pack_start(&renderer, false);

    col.add_attribute(&renderer, "pixbuf", 0);

    let renderer2 = gtk::CellRendererText::new();
    col.pack_start(&renderer2, true);
    col.add_attribute(&renderer2, "text", 1);
    let image = match gdk::pixbuf::Pixbuf::new_from_file("./resources/eye.png") {
        Ok(i) => i,
        Err(e) => {
            println!("Error: {:?}", e);
            return;
        }
    };

    right_tree.append_column(&col);
    right_tree.set_model(Some(&right_store));
    right_tree.set_headers_visible(true);

    let v = &image.into();
    for _ in 0..10 {
        let iter = right_store.append(None);

        right_store.set_value(&iter, 0, v);
        right_store.set_value(&iter, 1, &"I'm a child node with an image".into());
    }

    // display the panes

    let split_pane = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    split_pane.set_size_request(-1, -1);
    split_pane.add(&left_tree);
    split_pane.add(&middle_tree);
    split_pane.add(&right_tree);

    window.add(&split_pane);
    window.show_all();
    gtk::main();
}
