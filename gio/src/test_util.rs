// Take a look at the license at the top of the repository in the LICENSE file.

use glib::*;
use std::sync::mpsc::{channel, Sender};

#[allow(dead_code)]
pub fn run_async<T: Send + 'static, Q: FnOnce(Sender<T>, MainLoop) + Send + 'static>(
    start: Q,
) -> T {
    let c = MainContext::new();
    let l = MainLoop::new(Some(&c), false);
    let l_clone = l.clone();

    let (tx, rx) = channel();

    c.push_thread_default();
    c.invoke(move || {
        start(tx, l_clone);
    });

    l.run();
    c.pop_thread_default();

    rx.recv().unwrap()
}

#[allow(dead_code)]
pub fn run_async_local<T: 'static, Q: FnOnce(Sender<T>, MainLoop) + Send + 'static>(start: Q) -> T {
    let c = MainContext::new();
    let l = MainLoop::new(Some(&c), false);
    let l_clone = l.clone();

    let (tx, rx) = channel();

    c.push_thread_default();
    c.invoke_local(move || {
        start(tx, l_clone);
    });

    l.run();
    c.pop_thread_default();

    rx.recv().unwrap()
}
