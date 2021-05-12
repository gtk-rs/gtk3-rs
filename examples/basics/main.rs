use gtk::prelude::*;
use gio::prelude::ApplicationExt;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let button = gtk::Button::with_label("Click me!");
    let app = application.clone();
    button.connect_clicked(move |_| {
        let n = gio::Notification::new("Clicked");
        app.send_notification(None, &n);
    });

    window.add(&button);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default());

    application.connect_startup(|app| {
        let n = gio::Notification::new("Startup");
        app.send_notification(None, &n);
    });

    application.connect_activate(build_ui);

    application.run();
}
