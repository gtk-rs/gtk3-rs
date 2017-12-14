// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


use std::sync::mpsc::{channel, Sender};
use std::thread;
use glib::*;

#[allow(dead_code)]
pub fn run_async<T: Send + 'static, Q: FnOnce(Sender<T>, MainLoop) + Send + 'static>(start: Q) -> T {
    let l = MainLoop::new(None, false);
    let c = MainContext::default().unwrap();
    let l_clone = l.clone();

    let (tx, rx) = channel();

    thread::spawn(move || {
        c.invoke(move || {
            start(tx, l_clone);
        });
    });

    l.run();

    let ret = rx.recv().unwrap();
    ret
}
