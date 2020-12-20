// Take a look at the license at the top of the repository in the LICENSE file.

use crate::auto::DialogExt;
use crate::Dialog;
use crate::DialogFlags;
use crate::ResponseType;
use crate::Widget;
use crate::Window;
use glib::object::Cast;
use glib::translate::*;
use glib::IsA;
use std::ptr;

impl Dialog {
    #[doc(alias = "gtk_dialog_new_with_buttons")]
    pub fn with_buttons<T: IsA<Window>>(
        title: Option<&str>,
        parent: Option<&T>,
        flags: DialogFlags,
        buttons: &[(&str, ResponseType)],
    ) -> Dialog {
        assert_initialized_main_thread!();
        let ret: Dialog = unsafe {
            Widget::from_glib_none(ffi::gtk_dialog_new_with_buttons(
                title.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                ptr::null_mut(),
            ))
            .unsafe_cast()
        };

        ret.add_buttons(buttons);
        ret
    }
}

pub trait DialogExtManual: 'static {
    #[doc(alias = "gtk_dialog_add_buttons")]
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]);
}

impl<O: IsA<Dialog>> DialogExtManual for O {
    fn add_buttons(&self, buttons: &[(&str, ResponseType)]) {
        for &(text, id) in buttons {
            //FIXME: self.add_button don't work on 1.8
            O::add_button(self, text, id);
        }
    }
}
