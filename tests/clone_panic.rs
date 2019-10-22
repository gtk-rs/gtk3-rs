extern crate glib;

use std::cell::RefCell;
use std::rc::Rc;

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

#[test]
fn clone_closure() {
    let state = Rc::new(RefCell::new(State::new()));
    assert_eq!(state.borrow().started, false);

    let closure = {

        clone!(@weak state => move || {
            state.borrow_mut().started = true;

        })
    };

    closure();

    assert_eq!(state.borrow().started, true);
}

#[test]
#[should_panic]
fn clone_panic() {
    let closure = {
        let state = Rc::new(RefCell::new(State::new()));

        clone!(@weak state => move |_| {
            state.borrow_mut().started = true;

        })
    };

    closure(10);
}
