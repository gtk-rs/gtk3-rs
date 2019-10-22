extern crate gio;
extern crate glib;
extern crate gtk;

use std::cell::RefCell;
use std::rc::Rc;

use gio::{ApplicationExt, ApplicationExtManual};
use gtk::{
    Application,
    ApplicationWindow,
    Button,
    ButtonExt,
    ContainerExt,
    GtkWindowExt,
    WidgetExt,
};
use glib::clone;

struct State {
    started: bool,
    count: i32,
}

impl State {
    fn new() -> Self {
        Self {
            started: false,
            count: 0,
        }
    }
}

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    let state = Rc::new(RefCell::new(State::new()));

    application.connect_activate(clone!(state => move |app| {
        state.borrow_mut().started = true;

        let window = ApplicationWindow::new(app);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);

        let button = Button::new_with_label("Click me!");
        button.connect_clicked(clone!(state => move |_| {
            let mut state = state.borrow_mut();
            println!("Clicked (started: {}): {}!", state.started, state.count);
            state.count += 1;
        }));
        window.add(&button);

        window.show_all();
    }));

    application.run(&[]);
}
