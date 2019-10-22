extern crate glib;

use std::cell::RefCell;
use std::rc::Rc;

use glib::clone;

struct State {
    started: bool,
}

impl State {
    fn new() -> Self {
        Self {
            started: false,
        }
    }
}

#[test]
fn clone_closure() {
    let state = Rc::new(RefCell::new(State::new()));
    assert_eq!(state.borrow().started, false);

    let closure = {
        clone!(@weak state => move || {
            state.borrow_mut().started = true;
        })
    };

    assert_eq!(closure(), ());

    assert_eq!(state.borrow().started, true);
}

#[test]
fn clone_default_value() {
    let closure =
        {
            let state = Rc::new(RefCell::new(State::new()));
            clone!(@weak state => move |_| {
                state.borrow_mut().started = true;
                10
            }, 42)
        };

    assert_eq!(42, closure(50));
}
