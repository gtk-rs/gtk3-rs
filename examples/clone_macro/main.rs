use std::cell::RefCell;
use std::rc::Rc;

use gtk::glib;
use gtk::{prelude::*, Application, ApplicationWindow, Button};

#[derive(Default)]
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
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    let state = Rc::new(RefCell::new(State::new()));

    {
        let state2 = Rc::new(RefCell::new(State::new()));

        application.connect_activate(glib::clone!(@weak state, @strong state2 => move |app| {
            state.borrow_mut().started = true;

            let window = ApplicationWindow::new(app);
            window.set_title("First GTK+ Program");
            window.set_default_size(350, 70);

            let button = Button::with_label("Click me!");
            button.connect_clicked(glib::clone!(@weak state, @weak state2 => move |_| {
                let mut state = state.borrow_mut();
                let mut state2 = state2.borrow_mut();
                println!("Clicked (started: {}): {} - {}!", state.started, state.count, state2.count);
                state.count += 1;
                state2.count += 1;
            }));
            window.add(&button);

            window.show_all();
        }));
    }

    application.run();
}
