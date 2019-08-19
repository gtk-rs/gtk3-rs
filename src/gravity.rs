// Copyright 2019, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use pango_sys;
use Gravity;
use GravityHint;
use Matrix;
use Script;

impl Gravity {
    pub fn to_rotation(&self) -> f64 {
        unsafe { pango_sys::pango_gravity_to_rotation(self.to_glib()) }
    }

    pub fn get_for_matrix(matrix: &Matrix) -> Gravity {
        unsafe {
            from_glib(pango_sys::pango_gravity_get_for_matrix(
                matrix.to_glib_none().0,
            ))
        }
    }

    pub fn get_for_script(script: Script, base_gravity: Gravity, hint: GravityHint) -> Gravity {
        unsafe {
            from_glib(pango_sys::pango_gravity_get_for_script(
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
            from_glib(pango_sys::pango_gravity_get_for_script_and_width(
                script.to_glib(),
                wide.to_glib(),
                base_gravity.to_glib(),
                hint.to_glib(),
            ))
        }
    }
}
