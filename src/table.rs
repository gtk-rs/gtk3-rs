// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use atk_sys;
use glib::object::IsA;
use glib::translate::*;
use Table;

pub trait TableExtManual: 'static {
    fn get_selected_columns(&self) -> Vec<i32>;
    fn get_selected_rows(&self) -> Vec<i32>;
}

impl<O: IsA<Table>> TableExtManual for O {
    fn get_selected_columns(&self) -> Vec<i32> {
        unsafe {
            let mut selected = ::std::ptr::null_mut();
            let nb = atk_sys::atk_table_get_selected_columns(
                self.as_ref().to_glib_none().0,
                &mut selected,
            );
            if nb <= 0 {
                Vec::new()
            } else {
                Vec::from_raw_parts(selected, nb as usize, nb as usize)
            }
        }
    }

    fn get_selected_rows(&self) -> Vec<i32> {
        unsafe {
            let mut selected = ::std::ptr::null_mut();
            let nb =
                atk_sys::atk_table_get_selected_rows(self.as_ref().to_glib_none().0, &mut selected);
            if nb <= 0 {
                Vec::new()
            } else {
                Vec::from_raw_parts(selected, nb as usize, nb as usize)
            }
        }
    }
}
