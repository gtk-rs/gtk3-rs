#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gdk::enums::modifier_type;

/// Expands to its argument if GTK+ 3.10 support is configured and to `()` otherwise
#[cfg(not(feature = "gtk_3_10"))]
macro_rules! with_gtk_3_10 {
    ($ex:expr) => (
        ()
    );
    ($bl:block) => {
        ()
    }
}

/// Expands to its argument if GTK+ 3.10 support is configured and to `()` otherwise
#[cfg(feature = "gtk_3_10")]
macro_rules! with_gtk_3_10 {
    ($ex:expr) => (
        $ex
    );
    ($bl:block) => {
        $bl
    }
}

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

fn about_clicked(button: &gtk::Button) {
    let dialog = gtk::AboutDialog::new();
    if let Some(window) = button.get_toplevel().and_then(|w| w.downcast::<gtk::Window>().ok()) {
        dialog.set_transient_for(Some(&window));
    }

    let crew = [
        "James T. Kirk",
        "Spock",
        "Leonard McCoy",
    ];

    dialog.set_authors(&crew);
    dialog.set_artists(&crew[1..]);

    println!("Authors: {:?}", dialog.get_authors());
    println!("Artists: {:?}", dialog.get_artists());
    println!("Documenters: {:?}", dialog.get_documenters());

    dialog.run();
    dialog.destroy();
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    println!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    let frame = gtk::Frame::new(Some("Yep a frame"));
    let _box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let v_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    let label = gtk::Label::new(Some("Yeah a wonderful label too !"));
    let button = gtk::Button::new_with_label("Whattttt a button !");
    let button_about = gtk::Button::new_with_label("About?");
    let button_recent = gtk::Button::new_with_label("Choose a recent one !");
    let button_font = gtk::Button::new_with_label("Choose a font !");
    let app_button = gtk::Button::new_with_label("App ?");
    let file_button = gtk::Button::new_with_label("file ?");
    let font_button = gtk::FontButton::new();
    let toggle_button = gtk::ToggleButton::new_with_label("Toggle Me !");
    let check_button = gtk::CheckButton::new_with_label("Labeled check button");
    let color_button = gtk::ColorButton::new();
    let menu_button = with_gtk_3_10!(
        gtk::MenuButton::new()
    );
    let link_button = gtk::LinkButton::new("www.rust-lang.org");
    let volume_button = gtk::VolumeButton::new();
    let entry = gtk::Entry::new();
    let search_entry = with_gtk_3_10!(
        gtk::SearchEntry::new()
    );
    let separator = gtk::Separator::new(gtk::Orientation::Horizontal);
    let separator2 = gtk::Separator::new(gtk::Orientation::Horizontal);
    let switch = gtk::Switch::new();
    let switch2 = gtk::Switch::new();
    let scale = gtk::Scale::new_with_range(gtk::Orientation::Horizontal, 0., 100., 1.);
    let level_bar = with_gtk_3_10!(
        gtk::LevelBar::new_for_interval(0., 100.)
    );
    let spin_button = gtk::SpinButton::new_with_range(0., 100., 1.);
    let spinner = gtk::Spinner::new();
    let image = gtk::Image::new_from_file("./test/resources/gtk.jpg");
    let progress_bar = gtk::ProgressBar::new();
    let arrow = gtk::Arrow::new(gtk::ArrowType::Right, gtk::ShadowType::EtchedOut);
    let calendar = gtk::Calendar::new();
    let info_bar = gtk::InfoBar::new();
    let tmp_button = with_gtk_3_10!(
        gtk::Button::new_from_icon_name("edit-clear", gtk::IconSize::Button as i32)
    );

    println!("test");

    with_gtk_3_10! {{
        info_bar.set_show_close_button(true);
    }}

    /*info_bar.connect(signals::Response::new(|response_id| {
        info_bar.hide()
    }));*/ //TODO: Why does this not work?

    progress_bar.set_fraction(0.7);
    spinner.start();
    with_gtk_3_10! {{
        level_bar.set_value(37.);
    }}
    switch2.set_active(true);
    frame.set_border_width(10);
    _box.set_border_width(5);
    entry.set_placeholder_text(Some("An Entry with a placeholder !"));
    volume_button.set_orientation(gtk::Orientation::Horizontal);
    label.set_justify(gtk::Justification::Left);
    window.set_title("Yeah a beautiful window with gtk !");
    window.set_position(gtk::WindowPosition::Center);
    window.add(&frame);

    scale.set_digits(1);
    scale.connect_format_value(|scale, value| {
        let digits = scale.get_digits() as usize;
        format!("<{:.*}>", digits, value)
    });

    let entry_clone = entry.clone();
    button.connect_clicked(clone!(window => move |_| {
        let dialog = gtk::Dialog::new_with_buttons(Some("Hello!"), Some(&window), gtk::DIALOG_MODAL,
            &[("No", 0), ("Yes", 1), ("Yes!", 2)]);

        let ret = dialog.run();

        dialog.destroy();

        entry_clone.set_text(&format!("Clicked {}", ret));
    }));

    // use a plain function instead of a closure
    button_about.connect_clicked(about_clicked);

    button_font.connect_clicked(clone!(window => move |_| {
        let dialog = gtk::FontChooserDialog::new(Some("Font chooser test"), Some(&window));

        dialog.run();
        dialog.destroy();
    }));

    button_recent.connect_clicked(clone!(window => move |_| {
        let dialog = gtk::RecentChooserDialog::new(Some("Recent chooser test"), Some(&window));
        dialog.add_buttons(&[
            ("Ok", gtk::ResponseType::Ok as i32),
            ("Cancel", gtk::ResponseType::Cancel as i32)
        ]);

        dialog.run();
        dialog.destroy();
    }));

    file_button.connect_clicked(clone!(window => move |_| {
        //entry.set_text("Clicked!");
        let dialog = gtk::FileChooserDialog::new(Some("Choose a file"), Some(&window),
            gtk::FileChooserAction::Open);
        dialog.add_buttons(&[
            ("Open", gtk::ResponseType::Ok as i32),
            ("Cancel", gtk::ResponseType::Cancel as i32)
        ]);

        dialog.set_select_multiple(true);
        dialog.run();
        let files = dialog.get_filenames();
        dialog.destroy();

        println!("Files: {:?}", files);
    }));

    app_button.connect_clicked(clone!(window => move |_| {
        //entry.set_text("Clicked!");
        let dialog = gtk::AppChooserDialog::new_for_content_type(Some(&window), gtk::DIALOG_MODAL,
            "sh");

        dialog.run();
        dialog.destroy();
    }));

    let entry_clone = entry.clone();
    window.connect_key_press_event(move |_, key| {
        let keyval = key.as_ref().keyval;
        let keystate = key.as_ref().state;

        println!("key pressed: {} / {:?}", keyval, keystate);
        println!("text: {}", entry_clone.get_text().unwrap());

        if keystate.intersects(modifier_type::ControlMask) {
            println!("You pressed Ctrl!");
        }

        Inhibit(false)
    });

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    frame.add(&_box);
    with_gtk_3_10! {{
        button_box.add(&tmp_button)
    }};
    button_box.add(&button);
    button_box.add(&button_about);
    button_box.add(&button_font);
    button_box.add(&button_recent);
    button_box.add(&file_button);
    button_box.add(&app_button);
    button_box.add(&font_button);
    button_box.add(&toggle_button);
    button_box.add(&color_button);
    button_box.add(&volume_button);
    v_box.add(&switch);
    with_gtk_3_10! {{
        v_box.add(&menu_button);
    }}
    v_box.add(&switch2);
    v_box.add(&check_button);
    v_box.add(&link_button);
    v_box.add(&spin_button);
    _box.add(&info_bar);
    _box.add(&v_box);
    _box.add(&scale);
    with_gtk_3_10! {{
        _box.add(&level_bar);
    }}
    _box.add(&button_box);
    _box.add(&progress_bar);
    _box.add(&separator);
    _box.add(&label);
    _box.add(&entry);
    _box.add(&separator2);
    with_gtk_3_10! {{
        _box.add(&search_entry);
    }}
    _box.add(&spinner);
    _box.add(&image);
    _box.add(&arrow);
    _box.add(&calendar);
    _box.set_orientation(gtk::Orientation::Vertical);
    // window.set_decorated(false);
    window.set_decorated(true);
    window.show_all();
    gtk::main();
}
