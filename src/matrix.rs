// Copyright 2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use ffi;
use Matrix;

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::PangoMatrix> for Matrix {
    unsafe fn from_glib_none(ptr: *const ffi::PangoMatrix) -> Self {
        from_glib_none(mut_override(ptr))
    }
}
