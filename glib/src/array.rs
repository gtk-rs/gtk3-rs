// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::fmt;

wrapper! {
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Array(Shared<ffi::GArray>);

    match fn {
        ref => |ptr| ffi::g_array_ref(ptr),
        unref => |ptr| ffi::g_array_unref(ptr),
        type_ => || ffi::g_array_get_type(),
    }
}

impl Array {
    pub fn len(&self) -> usize {
        unsafe { (*self.to_glib_none().0).len as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn data(&self) -> *mut libc::c_void {
        unsafe { (*self.to_glib_none().0).data as _ }
    }

    pub fn element_size(&self) -> usize {
        unsafe { ffi::g_array_get_element_size(self.to_glib_none().0) as usize }
    }
}

impl fmt::Debug for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Array")
            .field("len", &self.len())
            .field("data", &self.data())
            .field("element_size", &self.element_size())
            .finish()
    }
}
