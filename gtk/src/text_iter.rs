// Take a look at the license at the top of the repository in the LICENSE file.

use crate::TextAttributes;
use crate::TextIter;
use glib::translate::*;
use std::convert::TryFrom;

impl TextIter {
    #[doc(alias = "gtk_text_iter_get_attributes")]
    pub fn get_attributes(&self, values: &TextAttributes) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_iter_get_attributes(
                self.to_glib_none().0,
                mut_override(values.to_glib_none().0),
            ))
        }
    }

    #[doc(alias = "gtk_text_iter_get_char")]
    pub fn get_char(&self) -> Option<char> {
        let ret = unsafe { ffi::gtk_text_iter_get_char(self.to_glib_none().0) };

        if ret == 0 {
            return None;
        }

        Some(TryFrom::try_from(ret).expect("conversion from an invalid Unicode value attempted"))
    }
}
