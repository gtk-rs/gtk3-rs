#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate glib;
extern crate gdk;
extern crate gio;
extern crate gtk;

#[cfg(feature = "gtk_3_10")]
mod example {
    use gdk;
    use gio;
    use gio::prelude::*;
    use gtk::prelude::*;
    use gtk::{
        self, AboutDialog, ApplicationWindow, AppChooserDialog, Builder, Button, Dialog, Entry,
        FileChooserAction, FileChooserDialog, FontChooserDialog, Scale, SpinButton,
        RecentChooserDialog, ResponseType, Spinner, Switch, Window,
    };

    use std::env::args;

    // make moving clones into closures more convenient
    macro_rules! clone {
        (@param _) => ( _ );
        (@param $x:ident) => ( $x );
        ($($n:ident),+ => move || $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move || $body
            }
        );
        ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
            {
                $( let $n = $n.clone(); )+
                move |$(clone!(@param $p),)+| $body
            }
        );
    }

    // upgrade weak reference or return
    #[macro_export]
    macro_rules! upgrade_weak {
        ($x:ident, $r:expr) => {{
            match $x.upgrade() {
                Some(o) => o,
                None => return $r,
            }
        }};
        ($x:ident) => {
            upgrade_weak!($x, ())
        };
    }

    fn about_clicked(button: &Button, dialog: &AboutDialog) {
        if let Some(window) = button.get_toplevel().and_then(|w| w.downcast::<Window>().ok()) {
            dialog.set_transient_for(Some(&window));
        }

        println!("Authors: {:?}", dialog.get_authors());
        println!("Artists: {:?}", dialog.get_artists());
        println!("Documenters: {:?}", dialog.get_documenters());

        // Since we only have once instance of this object with Glade, we only show/hide it.
        dialog.show();
        dialog.run();
        dialog.hide();
    }

    fn build_ui(application: &gtk::Application) {
        println!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
        let glade_src = include_str!("gtktest.glade");
        let builder = Builder::new_from_string(glade_src);

        let spinner: Spinner = builder.get_object("spinner").expect("Couldn't get spinner");
        spinner.start();

        let scale: Scale = builder.get_object("scale").expect("Couldn't get scale");
        scale.connect_format_value(|scale, value| {
            let digits = scale.get_digits() as usize;
            format!("<{:.*}>", digits, value)
        });

        let spin_button: SpinButton = builder.get_object("spin_button")
                                             .expect("Couldn't get spin_button");
        spin_button.connect_input(|spin_button| {
            let text = spin_button.get_text().expect("Couldn't get text from spin_button");
            println!("spin_button_input: \"{}\"", text);
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

        let window: ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
        window.set_application(application);

        let window_weak = window.downgrade();

        let button: Button = builder.get_object("button").expect("Couldn't get button");
        let entry: Entry = builder.get_object("entry").expect("Couldn't get entry");

        let entry_weak = entry.downgrade();
        button.connect_clicked(clone!(window_weak, entry_weak => move |_| {
            let window = upgrade_weak!(window_weak);
            let entry = upgrade_weak!(entry_weak);

            let dialog = Dialog::new_with_buttons(Some("Hello!"),
                                                  Some(&window),
                                                  gtk::DialogFlags::MODAL,
                                                  &[("No", ResponseType::No),
                                                    ("Yes", ResponseType::Yes),
                                                    ("Custom", ResponseType::Other(0))]);

            let ret = dialog.run();

            dialog.destroy();

            entry.set_text(&format!("Clicked {}", ret));
        }));

        let button_font: Button = builder.get_object("button_font")
                                         .expect("Couldn't get button_font");
        button_font.connect_clicked(clone!(window_weak => move |_| {
            let window = upgrade_weak!(window_weak);

            let dialog = FontChooserDialog::new(Some("Font chooser test"), Some(&window));

            dialog.run();
            dialog.destroy();
        }));

        let button_recent: Button = builder.get_object("button_recent")
                                           .expect("Couldn't get button_recent");
        button_recent.connect_clicked(clone!(window_weak => move |_| {
            let window = upgrade_weak!(window_weak);

            let dialog = RecentChooserDialog::new(Some("Recent chooser test"), Some(&window));
            dialog.add_buttons(&[
                ("Ok", ResponseType::Ok.into()),
                ("Cancel", ResponseType::Cancel.into())
            ]);

            dialog.run();
            dialog.destroy();
        }));

        let file_button: Button = builder.get_object("file_button")
                                         .expect("Couldn't get file_button");
        file_button.connect_clicked(clone!(window_weak => move |_| {
            let window = upgrade_weak!(window_weak);

            //entry.set_text("Clicked!");
            let dialog = FileChooserDialog::new(Some("Choose a file"), Some(&window),
                                                FileChooserAction::Open);
            dialog.add_buttons(&[
                ("Open", ResponseType::Ok.into()),
                ("Cancel", ResponseType::Cancel.into())
            ]);

            dialog.set_select_multiple(true);
            dialog.run();
            let files = dialog.get_filenames();
            dialog.destroy();

            println!("Files: {:?}", files);
        }));

        let app_button: Button = builder.get_object("app_button").expect("Couldn't get app_button");
        app_button.connect_clicked(clone!(window_weak => move |_| {
            let window = upgrade_weak!(window_weak);

            //entry.set_text("Clicked!");
            let dialog = AppChooserDialog::new_for_content_type(Some(&window), gtk::DialogFlags::MODAL,
                "sh");

            dialog.run();
            dialog.destroy();
        }));

        let switch: Switch = builder.get_object("switch").expect("Couldn't get switch");
        switch.connect_changed_active(clone!(entry_weak => move |switch| {
            let entry = upgrade_weak!(entry_weak);

            if switch.get_active() {
                entry.set_text("Switch On");
            } else {
                entry.set_text("Switch Off");
            }
        }));

        let button_about: Button = builder.get_object("button_about")
                                          .expect("Couldn't get button_about");
        let dialog: AboutDialog = builder.get_object("dialog").expect("Couldn't get dialog");
        button_about.connect_clicked(move |x| {
            about_clicked(x, &dialog)
        });

        window.connect_key_press_event(clone!(entry_weak => move |_, key| {
            let entry = upgrade_weak!(entry_weak, Inhibit(false));

            let keyval = key.get_keyval();
            let keystate = key.get_state();

            println!("key pressed: {} / {:?}", keyval, keystate);
            println!("text: {}", entry.get_text().expect("Couldn't get text from entry"));

            if keystate.intersects(gdk::ModifierType::CONTROL_MASK) {
                println!("You pressed Ctrl!");
            }

            Inhibit(false)
        }));

        window.connect_delete_event(|win, _| {
            win.destroy();
            Inhibit(false)
        });

        window.show_all();
    }

    pub fn main() {
        let application = gtk::Application::new("com.github.gtk-rs.examples.gtktest",
                                                gio::ApplicationFlags::empty())
                                           .expect("Initialization failed...");

        application.connect_activate(|app| {
            build_ui(app);
        });

        application.run(&args().collect::<Vec<_>>());
    }
}

#[cfg(feature = "gtk_3_10")]
fn main() {
    example::main()
}

#[cfg(not(feature = "gtk_3_10"))]
fn main() {
    println!("This example requires GTK 3.10 or later");
    println!("Did you forget to build with `--features gtk_3_10`?");
}
