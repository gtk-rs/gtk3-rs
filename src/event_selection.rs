// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;

#[derive(Clone, Debug)]
pub struct EventSelection(::Event);

event_wrapper!(EventSelection, GdkEventSelection);
event_subtype!(EventSelection, SelectionClear | SelectionNotify | SelectionRequest);
