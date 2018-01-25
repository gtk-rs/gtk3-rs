// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! `IMPL` Low level signal support.

use libc::{c_void, c_uint, c_ulong};

use gobject_ffi::{self, GCallback};
use ffi::{gboolean, GQuark};
use object::{IsA, Object};
use source::CallbackGuard;
use translate::{from_glib, FromGlib, ToGlib, ToGlibPtr};

/// The id of a signal that is returned by `connect`.
#[derive(Debug, Eq, PartialEq)]
pub struct SignalHandlerId(c_ulong);

impl ToGlib for SignalHandlerId {
    type GlibType = c_ulong;

    #[inline]
    fn to_glib(&self) -> c_ulong {
        self.0
    }
}

impl FromGlib<c_ulong> for SignalHandlerId {
    #[inline]
    fn from_glib(val: c_ulong) -> SignalHandlerId {
        assert_ne!(val, 0);
        SignalHandlerId(val)
    }
}

/// Whether to propagate the signal to the default handler.
///
/// Don't inhibit default handlers without a reason, they're usually helpful.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Inhibit(pub bool);

#[doc(hidden)]
impl ToGlib for Inhibit {
    type GlibType = gboolean;

    #[inline]
    fn to_glib(&self) -> gboolean {
        self.0.to_glib()
    }
}

pub unsafe fn connect(receiver: *mut gobject_ffi::GObject, signal_name: &str, trampoline: GCallback,
                      closure: *mut Box<Fn() + 'static>) -> SignalHandlerId {
    let handle = gobject_ffi::g_signal_connect_data(receiver, signal_name.to_glib_none().0,
        trampoline, closure as *mut _, Some(destroy_closure),
        gobject_ffi::GConnectFlags::empty());
    assert!(handle > 0);
    from_glib(handle)
}

pub fn signal_handler_block<T: IsA<Object>>(instance: &T, handler_id: &SignalHandlerId) {
    unsafe {
        gobject_ffi::g_signal_handler_block(instance.to_glib_none().0, handler_id.to_glib());
    }
}

pub fn signal_handler_unblock<T: IsA<Object>>(instance: &T, handler_id: &SignalHandlerId) {
    unsafe {
        gobject_ffi::g_signal_handler_unblock(instance.to_glib_none().0, handler_id.to_glib());
    }
}

pub fn signal_handler_disconnect<T: IsA<Object>>(instance: &T, handler_id: SignalHandlerId) {
    unsafe {
        gobject_ffi::g_signal_handler_disconnect(instance.to_glib_none().0, handler_id.to_glib());
    }
}

pub fn signal_stop_emission<T: IsA<Object>>(instance: &T, signal_id: u32, detail: GQuark) {
    unsafe {
        gobject_ffi::g_signal_stop_emission(instance.to_glib_none().0, signal_id as c_uint, detail);
    }
}

pub fn signal_stop_emission_by_name<T: IsA<Object>>(instance: &T, signal_name: &str) {
    unsafe {
        gobject_ffi::g_signal_stop_emission_by_name(instance.to_glib_none().0, signal_name.to_glib_none().0);
    }
}

unsafe extern "C" fn destroy_closure(ptr: *mut c_void, _: *mut gobject_ffi::GClosure) {
    let _guard = CallbackGuard::new();
    // destroy
    Box::<Box<Fn()>>::from_raw(ptr as *mut _);
}
