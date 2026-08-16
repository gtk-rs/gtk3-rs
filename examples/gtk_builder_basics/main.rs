use gtk::glib;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, MessageDialog};

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("builder_basics.ui");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    let bigbutton: Button = builder.object("button1").expect("Couldn't get button1");
    let dialog: MessageDialog = builder
        .object("messagedialog1")
        .expect("Couldn't get messagedialog1");

    dialog.connect_delete_event(|dialog, _| {
        dialog.hide();
        glib::Propagation::Stop
    });

    bigbutton.connect_clicked(glib::clone!(
        #[weak]
        dialog,
        move |_| dialog.show_all()
    ));
    window.show_all();
}
