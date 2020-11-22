// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use crate::Gravity;
use crate::GravityHint;
use crate::Matrix;
use crate::Script;
use glib::translate::*;

impl Gravity {
    pub fn to_rotation(&self) -> f64 {
        unsafe { ffi::pango_gravity_to_rotation(self.to_glib()) }
    }

    pub fn get_for_matrix(matrix: &Matrix) -> Gravity {
        unsafe { from_glib(ffi::pango_gravity_get_for_matrix(matrix.to_glib_none().0)) }
    }

    pub fn get_for_script(script: Script, base_gravity: Gravity, hint: GravityHint) -> Gravity {
        unsafe {
            from_glib(ffi::pango_gravity_get_for_script(
                script.to_glib(),
                base_gravity.to_glib(),
                hint.to_glib(),
            ))
        }
    }

    pub fn get_for_script_and_width(
        script: Script,
        wide: bool,
        base_gravity: Gravity,
        hint: GravityHint,
    ) -> Gravity {
        unsafe {
            from_glib(ffi::pango_gravity_get_for_script_and_width(
                script.to_glib(),
                wide.to_glib(),
                base_gravity.to_glib(),
                hint.to_glib(),
            ))
        }
    }
}
