// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use crate::translate::*;
use crate::Binding;
use crate::Object;
use crate::ObjectType;
use crate::StaticType;

impl Binding {
    pub fn get_source(&self) -> Option<Object> {
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

    pub fn get_target(&self) -> Option<Object> {
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
