// Take a look at the license at the top of the repository in the LICENSE file.

use crate::EditableText;
use glib::object::IsA;
use glib::translate::*;

pub trait EditableTextExtManual: 'static {
    fn insert_text(&self, string: &str, position: i32) -> i32;
}

impl<O: IsA<EditableText>> EditableTextExtManual for O {
    fn insert_text(&self, string: &str, mut position: i32) -> i32 {
        let length = string.len() as i32;
        unsafe {
            ffi::atk_editable_text_insert_text(
                self.as_ref().to_glib_none().0,
                string.to_glib_none().0,
                length,
                &mut position,
            );
        }
        position
    }
}
