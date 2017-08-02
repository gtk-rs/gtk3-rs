use DateTime;
use ffi;
use translate::*;

impl DateTime {
    pub fn compare(&self, other: &Self) -> i32 {
        unsafe {ffi::g_date_time_compare(self.to_glib_none().0 as *const _, other.to_glib_none().0 as *const _) }
    }

    pub fn equal(&self, other: &Self) -> bool {
        unsafe { from_glib(ffi::g_date_time_equal(self.to_glib_none().0 as *const _, other.to_glib_none().0 as *const _)) }
    }
}
