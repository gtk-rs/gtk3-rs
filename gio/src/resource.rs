// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Resource;
use glib::translate::*;
use std::mem;
use std::ptr;

impl Resource {
    #[doc(alias = "g_resource_new_from_data")]
    pub fn from_data(data: &glib::Bytes) -> Result<Resource, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();

            // Create a copy of data if it is not pointer-aligned
            // https://bugzilla.gnome.org/show_bug.cgi?id=790030
            let mut data = data.clone();
            let data_ptr = glib::ffi::g_bytes_get_data(data.to_glib_none().0, ptr::null_mut());
            if data_ptr as usize % mem::align_of::<*const u8>() != 0 {
                data = glib::Bytes::from(&*data);
            }

            let ret = ffi::g_resource_new_from_data(data.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
