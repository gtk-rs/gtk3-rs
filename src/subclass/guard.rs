// Copyright (C) 2016-2018 Sebastian Dröge <sebastian@centricular.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::ptr;

use ffi;
use gobject_ffi;

#[macro_export]
/// Macro for creating a [`FloatingReferenceGuard`]
///
/// This is creating a guard type for keeping the `GObject` floating reference flag intact inside
/// virtual method implementations.
///
/// Pass a valid, C `GObject` pointer to the macro. It can only be used inside `unsafe` blocks.
///
/// [`FloatingReferenceGuard`]: subclass/guard/struct.FloatingReferenceGuard.html
macro_rules! glib_floating_reference_guard {
    ($obj:ident) => {
        let _guard = $crate::subclass::guard::FloatingReferenceGuard::new($obj as *mut _);
    };
}

/// Guard type for keeping the `GObject` floating reference flag intact
/// inside virtual method implementations
///
/// This should be created via the [`floating_reference_guard!`] macro.
///
/// [`floating_reference_guard!`]: ../../macro.floating_reference_guard.html
pub struct FloatingReferenceGuard(ptr::NonNull<gobject_ffi::GObject>);

impl FloatingReferenceGuard {
    #[doc(hidden)]
    pub unsafe fn new(obj: *mut gobject_ffi::GObject) -> Option<FloatingReferenceGuard> {
        assert!(!obj.is_null());
        if gobject_ffi::g_object_is_floating(obj) != ffi::GFALSE {
            gobject_ffi::g_object_ref_sink(obj);
            Some(FloatingReferenceGuard(ptr::NonNull::new_unchecked(obj)))
        } else {
            None
        }
    }
}

impl Drop for FloatingReferenceGuard {
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_object_force_floating(self.0.as_ptr());
        }
    }
}
