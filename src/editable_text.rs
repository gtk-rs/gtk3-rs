// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use ffi;
use EditableText;

pub trait EditableTextExtManual: 'static {
    fn insert_text(&self, string: &str, position: i32) -> i32;
}

impl<O: IsA<EditableText>> EditableTextExtManual for O {
    fn insert_text(&self, string: &str, mut position: i32) -> i32 {
        let length = string.len() as i32;
        unsafe {
            ffi::atk_editable_text_insert_text(self.as_ref().to_glib_none().0, string.to_glib_none().0, length, &mut position);
        }
        position
    }
}
