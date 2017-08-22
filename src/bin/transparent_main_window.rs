//! # Transparent main window example
//!
//! This example demonstrates how to create a main window with a transparent background.

extern crate gtk;
extern crate gdk;
extern crate cairo;

use gtk::prelude::*;
use gtk::{Window, WindowType, Fixed, Button};
use gdk::ScreenExt; //import get_rgba_visual

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    set_visual(&window, &None);

    window.connect_delete_event(quit);
    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    window.set_title("Alpha Demo");
    window.set_default_size(500, 500);
    window.set_app_paintable(true); //crucial for transparency

    let fixed = Fixed::new();
    window.add(&fixed);
    let button = Button::new_with_label("Dummy");
    button.set_size_request(100, 30);
    fixed.add(&button);

    window.show_all();
    gtk::main();
}

fn set_visual(window: &Window, _screen: &Option<gdk::Screen>) {
    if let Some(screen) = window.get_screen() {
        if let Some(visual) = screen.get_rgba_visual() {
            window.set_visual(&visual); //crucial for transparency
        }
    }
}

fn draw(_window: &Window, ctx: &cairo::Context) -> Inhibit {
    //crucial for transparency
    ctx.set_source_rgba(1.0, 0.0, 0.0, 0.4);
    ctx.set_operator(cairo::enums::Operator::Screen);
    ctx.paint();
    Inhibit(false)
}

fn quit(_window: &Window, _event: &gdk::Event) -> Inhibit {
    gtk::main_quit();
    Inhibit(false)
}
