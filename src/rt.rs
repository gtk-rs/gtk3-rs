// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>


macro_rules! assert_initialized_main_thread {
    () => ()
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
        if cfg!(debug_assertions) {
            assert_initialized_main_thread!();
        }
    )
}
