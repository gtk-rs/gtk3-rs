use gtk::prelude::*;
use gtk::{cairo, gdk};
use gtk::{ApplicationWindow, Button, Fixed};

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
    let button = Button::with_label("Dummy");
    button.set_size_request(100, 30);
    fixed.add(&button);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.transparent_main_window"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn set_visual(window: &ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = GtkWindowExt::screen(window) {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual)); // crucial for transparency
        }
    }
}

fn draw(_window: &ApplicationWindow, ctx: &cairo::Context) -> glib::Propagation {
    // crucial for transparency
    ctx.set_source_rgba(1.0, 0.0, 0.0, 0.4);
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("Invalid cairo surface state");
    glib::Propagation::Stop
}
