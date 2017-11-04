// Copyright 2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;

#[derive(Clone, Debug)]
pub struct EventSetting(::Event);

event_wrapper!(EventSetting, GdkEventSetting);
event_subtype!(EventSetting, ffi::GDK_SETTING);
