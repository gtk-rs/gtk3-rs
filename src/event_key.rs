// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventKey(::Event);

event_wrapper!(EventKey, GdkEventKey);
event_subtype!(EventKey, KeyPress | KeyRelease);
