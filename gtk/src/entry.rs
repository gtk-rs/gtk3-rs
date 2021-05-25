// Take a look at the license at the top of the repository in the LICENSE file.

use glib::object::IsA;
use glib::translate::ToGlibPtr;
use std::convert::TryFrom;

use crate::Entry;

pub trait EntryExtManual: 'static {
    /// Retrieves the character displayed in place of the real characters
    /// for entries with visibility set to false. See [EntryExt::set_invisible_char](crate::prelude::EntryExt::set_invisible_char).
    ///
    /// # Returns
    ///
    /// the current invisible char, or 0, if the entry does not
    ///  show invisible text at all.
    #[doc(alias = "gtk_entry_get_invisible_char")]
    #[doc(alias = "get_invisible_char")]
    fn invisible_char(&self) -> Option<char>;
}

impl<O: IsA<Entry>> EntryExtManual for O {
    fn invisible_char(&self) -> Option<char> {
        let ret = unsafe { ffi::gtk_entry_get_invisible_char(self.as_ref().to_glib_none().0) };

        if ret == 0 {
            return None;
        }

        Some(TryFrom::try_from(ret).expect("conversion from an invalid Unicode value attempted"))
    }
}
