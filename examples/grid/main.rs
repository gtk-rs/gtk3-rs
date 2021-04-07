#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Grid};

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("grid.ui");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let grid: Grid = builder.get_object("grid").expect("Couldn't get grid");
    let button6: Button = builder.get_object("button6").expect("Couldn't get button6");
    button6.connect_clicked(glib::clone!(@weak grid => move |button| {
        let height = grid.get_cell_height(button);
        let new_height = if height == 2 { 1 } else { 2 };
        grid.set_cell_height(button, new_height);
    }));
    let button7: Button = builder.get_object("button7").expect("Couldn't get button7");
    button7.connect_clicked(glib::clone!(@weak grid => move |button| {
        let left_attach = grid.get_cell_left_attach(button);
        let new_left_attach = if left_attach == 2 { 0 } else { left_attach + 1 };
        grid.set_cell_left_attach(button, new_left_attach);
    }));

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.grid"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(build_ui);

    application.run();
}
