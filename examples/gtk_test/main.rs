use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use gtk::{
    AboutDialog, AppChooserDialog, ApplicationWindow, Builder, Button, Dialog, Entry,
    FileChooserAction, FileChooserDialog, FontChooserDialog, RecentChooserDialog, ResponseType,
    Scale, SpinButton, Spinner, Switch, Window,
};

fn main() {
    gio::resources_register_include!("compiled.gresource").unwrap();

    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.gtktest"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    println!(
        "Major: {}, Minor: {}",
        gtk::major_version(),
        gtk::minor_version()
    );
    let glade_src = include_str!("gtk_test.ui");
    let builder = Builder::from_string(glade_src);

    let spinner: Spinner = builder.object("spinner").expect("Couldn't get spinner");
    spinner.start();

    let scale: Scale = builder.object("scale").expect("Couldn't get scale");
    scale.connect_format_value(|scale, value| {
        let digits = scale.digits() as usize;
        format!("<{value:.digits$}>")
    });

    let spin_button: SpinButton = builder
        .object("spin_button")
        .expect("Couldn't get spin_button");
    spin_button.connect_input(|spin_button| {
        let text = spin_button.text();
        println!("spin_button_input: \"{text}\"");
        match text.parse::<f64>() {
            Ok(value) if value >= 90. => {
                println!("circular right");
                Some(Ok(10.))
            }
            Ok(value) if value <= 10. => {
                println!("circular left");
                Some(Ok(90.))
            }
            Ok(value) => Some(Ok(value)),
            Err(_) => Some(Err(())),
        }
    });

    let window: ApplicationWindow = builder.object("window").expect("Couldn't get window");
    window.set_application(Some(application));

    let button: Button = builder.object("button").expect("Couldn't get button");
    let entry: Entry = builder.object("entry").expect("Couldn't get entry");

    button.connect_clicked(glib::clone!(@weak window, @weak entry => move |_| {
        let dialog = Dialog::with_buttons(Some("Hello!"),
                                              Some(&window),
                                              gtk::DialogFlags::MODAL,
                                              &[("No", ResponseType::No),
                                                ("Yes", ResponseType::Yes),
                                                ("Custom", ResponseType::Other(0))]);

        dialog.connect_response(glib::clone!(@weak entry => move |dialog, response| {
            entry.set_text(&format!("Clicked {response}"));
            dialog.close();
        }));
        dialog.show_all();
    }));

    let button_font: Button = builder
        .object("button_font")
        .expect("Couldn't get button_font");
    button_font.connect_clicked(glib::clone!(@weak window => move |_| {
        let dialog = FontChooserDialog::new(Some("Font chooser test"), Some(&window));

        dialog.connect_response(|dialog, _| dialog.close());
        dialog.show_all();
    }));

    let button_recent: Button = builder
        .object("button_recent")
        .expect("Couldn't get button_recent");
    button_recent.connect_clicked(glib::clone!(@weak window => move |_| {
        let dialog = RecentChooserDialog::new(Some("Recent chooser test"), Some(&window));
        dialog.add_buttons(&[
            ("Ok", ResponseType::Ok),
            ("Cancel", ResponseType::Cancel)
        ]);

        dialog.connect_response(|dialog, _| dialog.close());
        dialog.show_all();
    }));

    let file_button: Button = builder
        .object("file_button")
        .expect("Couldn't get file_button");
    file_button.connect_clicked(glib::clone!(@weak window => move |_| {
        // entry.set_text("Clicked!");
        let dialog = FileChooserDialog::new(Some("Choose a file"), Some(&window),
                                            FileChooserAction::Open);
        dialog.add_buttons(&[
            ("Open", ResponseType::Ok),
            ("Cancel", ResponseType::Cancel)
        ]);

        dialog.set_select_multiple(true);

        dialog.connect_response(|dialog, response| {
            if response == ResponseType::Ok {
                let files = dialog.filenames();
                println!("Files: {files:?}");
            }
            dialog.close();
        });
        dialog.show_all();
    }));

    let app_button: Button = builder
        .object("app_button")
        .expect("Couldn't get app_button");
    app_button.connect_clicked(glib::clone!(@weak window => move |_| {
        // entry.set_text("Clicked!");
        let dialog = AppChooserDialog::for_content_type(Some(&window),
                                                            gtk::DialogFlags::MODAL,
                                                            "sh");

        dialog.connect_response(|dialog, _| dialog.close());
        dialog.show_all();
    }));

    let switch: Switch = builder.object("switch").expect("Couldn't get switch");
    switch.connect_changed_active(glib::clone!(@weak entry => move |switch| {
        if switch.is_active() {
            entry.set_text("Switch On");
        } else {
            entry.set_text("Switch Off");
        }
    }));

    let button_about: Button = builder
        .object("button_about")
        .expect("Couldn't get button_about");
    let dialog: AboutDialog = builder.object("dialog").expect("Couldn't get dialog");
    button_about.connect_clicked(move |x| about_clicked(x, &dialog));

    window.connect_key_press_event(
        glib::clone!(@weak entry => @default-return glib::ControlFlow::Break, move |_, key| {
            let keyval = key.keyval();
            let keystate = key.state();

            println!("key pressed: {keyval} / {keystate:?}");
            println!("text: {}", entry.text());

            if keystate.intersects(gdk::ModifierType::CONTROL_MASK) {
                println!("You pressed Ctrl!");
            }

            glib::ControlFlow::Break
        }),
    );

    window.show_all();
}

fn about_clicked(button: &Button, dialog: &AboutDialog) {
    if let Some(window) = button.toplevel().and_then(|w| w.downcast::<Window>().ok()) {
        dialog.set_transient_for(Some(&window));
    }

    // We only want to hide the dialog when it's closed and not completely destroy it
    // as otherwise we can't show it again a second time.
    dialog.connect_delete_event(|dialog, _| {
        dialog.hide();
        glib::ControlFlow::Continue
    });

    println!("Authors: {:?}", dialog.authors());
    println!("Artists: {:?}", dialog.artists());
    println!("Documenters: {:?}", dialog.documenters());

    dialog.show_all();
}
