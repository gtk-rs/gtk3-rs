// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(any(feature = "v3_24", all(not(doctest), doc)))]
use crate::GestureStylus;
use gdk::AxisUse;
use glib::object::IsA;
use glib::translate::*;

pub trait GestureStylusExtManual: 'static {
    #[cfg(any(feature = "v3_24", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_24")))]
    fn get_axes(&self, axes: Vec<AxisUse>) -> Option<Vec<f64>>;
}

impl<O: IsA<GestureStylus>> GestureStylusExtManual for O {
    #[cfg(any(feature = "v3_24", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_24")))]
    fn get_axes(&self, axes: Vec<AxisUse>) -> Option<Vec<f64>> {
        let mut values: Vec<f64> = Vec::new();
        unsafe {
            let mut axes1: Vec<gdk::ffi::GdkAxisUse> = axes.iter().map(|a| a.to_glib()).collect();
            axes1.push(gdk::ffi::GDK_AXIS_IGNORE);
            if from_glib(ffi::gtk_gesture_stylus_get_axes(
                self.as_ref().to_glib_none().0,
                axes1.as_mut_ptr(),
                values.as_mut_ptr() as *mut *mut f64,
            )) {
                Some(values)
            } else {
                None
            }
        }
    }
}
