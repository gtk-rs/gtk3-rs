// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::PageRange;
use crate::PrintSettings;

impl PrintSettings {
    /// Sets the value of [PRINT_SETTINGS_PAGE_RANGES](crate::PRINT_SETTINGS_PAGE_RANGES).
    /// ## `page_ranges`
    /// an array of GtkPageRanges
    #[doc(alias = "gtk_print_settings_set_page_ranges")]
    pub fn set_page_ranges(&self, page_ranges: &[PageRange]) {
        let num_ranges = page_ranges.len() as i32;
        unsafe {
            ffi::gtk_print_settings_set_page_ranges(
                self.to_glib_none().0,
                mut_override(page_ranges.as_ptr() as *const _),
                num_ranges,
            );
        }
    }
}
