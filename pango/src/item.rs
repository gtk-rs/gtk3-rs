// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::ToGlibPtr;

use crate::Analysis;
use crate::Item;

impl Item {
    pub fn offset(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).offset }
    }

    pub fn length(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).length }
    }

    pub fn num_chars(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).num_chars }
    }

    pub fn analysis(&self) -> &Analysis {
        unsafe { &*(&((*self.to_glib_none().0).analysis) as *const _ as *const Analysis) }
    }
}
