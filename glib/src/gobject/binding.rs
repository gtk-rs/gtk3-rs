// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::Binding;
use crate::Object;
use crate::ObjectType;
use crate::StaticType;

impl Binding {
    #[doc(alias = "get_source")]
    pub fn source(&self) -> Option<Object> {
        unsafe {
            let mut value = crate::Value::from_type(<Object as StaticType>::static_type());
            crate::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut crate::gobject_ffi::GObject,
                b"source\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `source` getter")
        }
    }

    #[doc(alias = "get_target")]
    pub fn target(&self) -> Option<Object> {
        unsafe {
            let mut value = crate::Value::from_type(<Object as StaticType>::static_type());
            crate::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut crate::gobject_ffi::GObject,
                b"target\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `target` getter")
        }
    }
}
