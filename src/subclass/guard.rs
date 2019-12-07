// Copyright 2017-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gobject_sys;

#[macro_export]
/// Macro for creating a [`FloatingReferenceGuard`].
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
/// inside virtual method implementations.
///
/// This should be created via the [`floating_reference_guard!`] macro.
///
/// [`floating_reference_guard!`]: ../../macro.floating_reference_guard.html
pub struct FloatingReferenceGuard;

impl FloatingReferenceGuard {
    #[doc(hidden)]
    pub unsafe fn new(_obj: *mut gobject_sys::GObject) -> Option<FloatingReferenceGuard> {
        None
    }
}
