//! # Transparent main window example
//!
//! This example demonstrates how to create a main window with a transparent background.

extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Button, Fixed};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = ApplicationWindow::new(application);
    set_visual(&window, None);

    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    window.set_title("Alpha Demo");
    window.set_default_size(500, 500);
    window.set_app_paintable(true); // crucial for transparency

    let fixed = Fixed::new();
    window.add(&fixed);
    let button = Button::new_with_label("Dummy");
    button.set_size_request(100, 30);
    fixed.add(&button);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.transparent_main_window"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

fn set_visual(window: &ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(ref visual) = screen.get_rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

fn draw(_window: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    // crucial for transparency
    ctx.set_source_rgba(1.0, 0.0, 0.0, 0.4);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint();
    Inhibit(false)
}
