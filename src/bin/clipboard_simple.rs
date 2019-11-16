//! Simple clipboard example
//!
//! From https://developer.gnome.org/gtkmm-tutorial/stable/sec-clipboard-examples.html.en
extern crate gdk;
extern crate gio;
extern crate gtk;

use std::cell::RefCell;
use std::env::args;

use gio::prelude::*;
use gtk::prelude::*;

struct Ui {
    pub button_a1: gtk::ToggleButton,
    pub button_a2: gtk::ToggleButton,
    pub button_b1: gtk::ToggleButton,
    pub button_b2: gtk::ToggleButton,
}

// Declare a new thread local storage key
thread_local!(
    static GLOBAL: RefCell<Option<Ui>> = RefCell::new(None)
);

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    // Create the whole window
    window.set_title("gtk::Clipboard Simple Example");
    window.connect_delete_event(|window, _| {
        window.destroy();
        Inhibit(false)
    });

    // Create the button grid
    let grid = gtk::Grid::new();
    grid.set_row_homogeneous(true);
    grid.set_column_homogeneous(true);
    let button_a1 = gtk::ToggleButton::new_with_label("A1");
    grid.attach(&button_a1, 0, 0, 1, 1);
    let button_a2 = gtk::ToggleButton::new_with_label("A2");
    grid.attach(&button_a2, 1, 0, 1, 1);
    let button_b1 = gtk::ToggleButton::new_with_label("B1");
    grid.attach(&button_b1, 0, 1, 1, 1);
    let button_b2 = gtk::ToggleButton::new_with_label("B2");
    grid.attach(&button_b2, 1, 1, 1, 1);

    // Add in the action buttons
    let copy_button = gtk::Button::new_with_mnemonic("_Copy");
    let paste_button = gtk::Button::new_with_mnemonic("_Paste");
    let button_box = gtk::ButtonBox::new(gtk::Orientation::Horizontal);
    button_box.set_layout(gtk::ButtonBoxStyle::End);
    button_box.pack_start(&copy_button, false, false, 0);
    button_box.pack_start(&paste_button, false, false, 0);

    // Pack widgets into the window and display everything
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.set_spacing(6);
    let label = gtk::Label::new(Some(
        "Select cells in the grid, click Copy, then \
         open a second instance of this example to try \
         pasting the copied data.",
    ));
    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&grid, true, true, 0);
    vbox.pack_start(&button_box, true, true, 0);
    window.add(&vbox);

    window.show_all();

    // Save out UI in thread-local storage so we can use it in callbacks later
    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some(Ui {
            button_a1: button_a1,
            button_a2: button_a2,
            button_b1: button_b1,
            button_b2: button_b2,
        })
    });

    // Attach signal handlers
    copy_button.connect_clicked(|_| {
        let mut s = String::new();
        GLOBAL.with(|global| {
            if let Some(ref ui) = *global.borrow() {
                if ui.button_a1.get_active() {
                    s.push_str("1");
                } else {
                    s.push_str("0");
                }
                if ui.button_a2.get_active() {
                    s.push_str("1");
                } else {
                    s.push_str("0");
                }
                if ui.button_b1.get_active() {
                    s.push_str("1");
                } else {
                    s.push_str("0");
                }
                if ui.button_b2.get_active() {
                    s.push_str("1");
                } else {
                    s.push_str("0");
                }
            }
        });
        let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
        clipboard.set_text(&s);
    });
    paste_button.connect_clicked(|_| {
        let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
        clipboard.request_text(|_, t| {
            if t.is_some() {
                let t = t.unwrap();
                if t.len() >= 4 {
                    GLOBAL.with(|global| {
                        if let Some(ref ui) = *global.borrow() {
                            ui.button_a1.set_active(t.chars().nth(0).unwrap() == '1');
                            ui.button_a2.set_active(t.chars().nth(1).unwrap() == '1');
                            ui.button_b1.set_active(t.chars().nth(2).unwrap() == '1');
                            ui.button_b2.set_active(t.chars().nth(3).unwrap() == '1');
                        }
                    });
                }
            }
        });
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("org.gtk-rs.example.clipboard_simple"),
        gio::ApplicationFlags::NON_UNIQUE,
    )
    .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
