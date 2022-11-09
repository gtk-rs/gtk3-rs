use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use gtk::glib;
use gtk::prelude::*;
use gtk::Builder;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("text_viewer.glade");
    let builder = Builder::new();
    builder
        .add_from_string(glade_src)
        .expect("Couldn't add from string");

    let window: gtk::ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));
    let open_button: gtk::ToolButton = builder.object("open_button").expect("Couldn't get builder");
    let text_view: gtk::TextView = builder.object("text_view").expect("Couldn't get text_view");

    open_button.connect_clicked(glib::clone!(@weak window => move |_| {
        // TODO move this to a impl?
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"),
            Some(&window),
            gtk::FileChooserAction::Open,
        );
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok),
            ("Cancel", gtk::ResponseType::Cancel),
        ]);
        file_chooser.connect_response(glib::clone!(@weak text_view => move |file_chooser, response| {
            if response == gtk::ResponseType::Ok {
                let filename = file_chooser.filename().expect("Couldn't get filename");
                let file = File::open(filename).expect("Couldn't open file");

                let mut reader = BufReader::new(file);
                let mut contents = String::new();
                let _ = reader.read_to_string(&mut contents);

                text_view
                    .buffer()
                    .expect("Couldn't get window")
                    .set_text(&contents);
            }
            file_chooser.close();
        }));

        file_chooser.show_all();
    }));

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.text_viewer"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}
