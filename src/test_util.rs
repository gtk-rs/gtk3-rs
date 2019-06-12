// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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

    let ret = rx.recv().unwrap();
    ret
}
