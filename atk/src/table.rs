// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Table;
use glib::object::IsA;
use glib::translate::*;

pub trait TableExtManual: 'static {
    #[doc(alias = "atk_table_get_selected_columns")]
    fn get_selected_columns(&self) -> Vec<i32>;

    #[doc(alias = "atk_table_get_selected_rows")]
    fn get_selected_rows(&self) -> Vec<i32>;
}

impl<O: IsA<Table>> TableExtManual for O {
    fn get_selected_columns(&self) -> Vec<i32> {
        unsafe {
            let mut selected = ::std::ptr::null_mut();
            let nb =
                ffi::atk_table_get_selected_columns(self.as_ref().to_glib_none().0, &mut selected);
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
                ffi::atk_table_get_selected_rows(self.as_ref().to_glib_none().0, &mut selected);
            if nb <= 0 {
                Vec::new()
            } else {
                Vec::from_raw_parts(selected, nb as usize, nb as usize)
            }
        }
    }
}
