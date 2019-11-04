//! # Printing
//!
//! This sample reads text from two Entry fields,
//! shows a print dialog and prints both texts one below
//! the other.

extern crate cairo;
extern crate gio;
extern crate gtk;
extern crate pango;
extern crate pangocairo;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn print(window: &gtk::Window, value1: String, value2: String) {
    let print_operation = gtk::PrintOperation::new();

    // Currently unused
    // Could be used to check whether there was a success in printing
    //let print_operation_result: gtk::PrintOperationResult;

    print_operation.connect_begin_print(move |print_operation, _| {
        // This sets the number of pages of the document.
        // You most likely will calculate this, but for this example
        // it's hardcoded as 1
        print_operation.set_n_pages(1);
    });

    print_operation.connect_draw_page(move |_, print_context, _| {
        let cairo = print_context
            .get_cairo_context()
            .expect("Couldn't get cairo context");

        // This allows you to get the width of the page
        // Currently unused in this example
        //let width = print_context.get_width();

        //Initi pango and set a font
        let font_description = pango::FontDescription::from_string("sans 14");
        let pango_layout = print_context
            .create_pango_layout()
            .expect("Couldn't create pango layout");
        pango_layout.set_font_description(Option::from(&font_description));

        // Draw text1
        pango_layout.set_text(&value1);
        cairo.move_to(10.0, 10.0);
        pangocairo::functions::show_layout(&cairo, &pango_layout);

        //Draw text2 below text1
        pango_layout.set_text(&value2);
        cairo.rel_move_to(0.0, 20.0);
        pangocairo::functions::show_layout(&cairo, &pango_layout);
    });

    //Open Print dialog setting up main window as its parent
    print_operation
        .run(gtk::PrintOperationAction::PrintDialog, Option::from(window))
        .expect("Couldn't print");
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("printing.glade");
    let builder = gtk::Builder::new_from_string(glade_src);

    let window: gtk::Window = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let entry1: gtk::Entry = builder.get_object("entry1").expect("Couldn't get entry1");
    let entry2: gtk::Entry = builder.get_object("entry2").expect("Couldn't get entry2");
    let button_print: gtk::Button = builder
        .get_object("buttonprint")
        .expect("Couldn't get buttonprint");

    let window_clone = window.clone();
    button_print.connect_clicked(move |_| {
        let text1 = entry1.get_text().expect("Couldn't get text1").to_string();
        let text2 = entry2.get_text().expect("Couldn't get text2").to_string();
        print(&window_clone, text1, text2);
    });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.printing"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
