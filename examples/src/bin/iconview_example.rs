//! # IconView Sample
//!
//! This sample demonstrates how to create a toplevel `window`, set its title, size and
//! position, how to add a `IconView` to this `window` and how to set `model` of the `IconView`
//!
//! A Gtk.IconView is a widget that displays a collection of icons in a grid view.
//! It supports features such as drag and drop, multiple selections and item reordering.
//! Similarly to Gtk.TreeView, Gtk.IconView uses a Gtk.ListStore for its model.
//!
//! Instead of using cell renderers, Gtk.IconView requires that one of the columns in its
//! Gtk.ListStore contains GdkPixbuf.Pixbuf objects.
//!
//! The example is using icons from the current icon theme. To view all icons and their names please
//! install gtk3-icon-browser: https://developer.gnome.org/gtk3/stable/gtk3-icon-browser.html

use gtk::prelude::*;
use gtk::{gdk_pixbuf, glib};

use std::env::args;
use std::process;

// Convenience Enum for IconView column types
enum IconViewColumnType {
    TextColumn = 0,
    PixbufColumn = 1,
}

fn create_list_store_model() -> gtk::ListStore {
    // Initialize array of icon names, these can be found using gtk3-icon-browser app
    let icons: [&'static str; 3] = ["edit-cut", "edit-paste", "edit-copy"];

    // Initialize an array of column types for ListStore object. Here we say that the first item
    // must always be of glib::Type String and the second item is of glib::Type Pixbuf.
    let col_types: [glib::Type; 2] = [glib::Type::STRING, gdk_pixbuf::Pixbuf::static_type()];
    let icon_view_model = gtk::ListStore::new(&col_types);

    // IconTheme provides a facility for looking up icons by name and size.
    //
    // Get default icon theme
    let icon_theme: Option<gtk::IconTheme> = gtk::IconTheme::get_default();
    if let Some(it) = icon_theme {
        for x in &icons {
            // Looks up an icon in an icon theme, scales it to the given size and renders it into
            // a pixbuf.
            let result = it.load_icon(x, 64, gtk::IconLookupFlags::empty());
            match result {
                Ok(r) => {
                    // Notice how we specified the first column to be Text and second to be Pixbuf
                    // just like in col_types var.
                    //
                    // The values also follow the same order, &[&String::from("Label"), &r].
                    // First item is text, second is pixbuf
                    icon_view_model.insert_with_values(
                        None,
                        &[
                            (
                                IconViewColumnType::TextColumn as u32,
                                &String::from("Label"),
                            ),
                            (IconViewColumnType::PixbufColumn as u32, &r),
                        ],
                    );
                }
                Err(err) => {
                    println!("Error: {}", err);
                    process::exit(1);
                }
            }
        }
    }

    icon_view_model
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("IconView Example");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let icon_view = gtk::IconView::new();
    icon_view.set_item_padding(0);
    icon_view.set_columns(3);
    icon_view.set_column_spacing(0);
    // User can select only one item at a time
    icon_view.set_selection_mode(gtk::SelectionMode::Single);

    // Create a model for our IconView
    let icon_view_model = create_list_store_model();
    // Set IconView model
    icon_view.set_model(Some(&icon_view_model));

    // And finally set text column and pixbuf column using enum
    icon_view.set_text_column(IconViewColumnType::TextColumn as i32);
    icon_view.set_pixbuf_column(IconViewColumnType::PixbufColumn as i32);

    window.add(&icon_view);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.iconview_example"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
