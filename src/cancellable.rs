// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use crate::CancellableExt;

#[test]
fn check_callback() {
    let c = crate::Cancellable::new();
    c.connect_cancelled(|_| {});
    c.cancel(); // if it doesn't crash at this point, then we're good to go!
}
