// Copyright 2013-2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use crate::StateFlags;
use crate::StyleContext;
use crate::StyleContextExt;
use glib::object::IsA;
use pango::FontDescription;

pub trait StyleContextExtManual: 'static {
    fn get_font(&self, state: StateFlags) -> FontDescription;
}

impl<O: IsA<StyleContext>> StyleContextExtManual for O {
    fn get_font(&self, state: StateFlags) -> FontDescription {
        self.get_property("font", state)
            .get()
            .expect("font property is not pango::FontDescription")
            .expect("font property is empty")
    }
}
