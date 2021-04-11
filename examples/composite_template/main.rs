mod example_application_window;

use gtk::prelude::*;

use example_application_window::ExampleApplicationWindow;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.composite_template"),
        Default::default(),
    );

    application.connect_activate(|app| {
        let win = ExampleApplicationWindow::new(app);
        win.show();
    });

    application.run();
}
