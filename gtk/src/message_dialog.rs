// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DialogFlags;
use crate::MessageDialog;
use crate::MessageType;
use crate::Widget;
use crate::Window;
use crate::{ButtonsType, ffi};
use glib::object::{Cast, IsA};
use glib::translate::*;
use libc::c_char;
use std::ptr;

impl MessageDialog {
    #[doc(alias = "gtk_message_dialog_new")]
    pub fn new<T: IsA<Window>>(
        parent: Option<&T>,
        flags: DialogFlags,
        type_: MessageType,
        buttons: ButtonsType,
        message: &str,
    ) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            let message: Stash<*const c_char, _> = message.to_glib_none();
            Widget::from_glib_none(ffi::gtk_message_dialog_new(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                type_.into_glib(),
                buttons.into_glib(),
                c"%s".as_ptr() as *const c_char,
                message.0,
                ptr::null::<c_char>(),
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_message_dialog_new_with_markup")]
    #[doc(alias = "new_with_markup")]
    pub fn with_markup<T: IsA<Window>>(
        parent: Option<&T>,
        flags: DialogFlags,
        type_: MessageType,
        buttons: ButtonsType,
        message: Option<&str>,
    ) -> MessageDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_message_dialog_new_with_markup(
                parent.map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
                type_.into_glib(),
                buttons.into_glib(),
                message.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }
}

impl Default for MessageDialog {
    fn default() -> Self {
        assert_initialized_main_thread!();
        glib::Object::new()
    }
}

pub trait MessageDialogExtManual: IsA<MessageDialog> + 'static {
    #[doc(alias = "gtk_message_dialog_format_secondary_markup")]
    fn format_secondary_markup(&self, message: Option<&str>) {
        match message {
            Some(m) => unsafe {
                let message: Stash<*const c_char, _> = m.to_glib_none();
                ffi::gtk_message_dialog_format_secondary_markup(
                    self.as_ref().to_glib_none().0,
                    c"%s".as_ptr() as *const c_char,
                    message.0,
                    ptr::null::<c_char>(),
                )
            },
            None => unsafe {
                ffi::gtk_message_dialog_format_secondary_markup(
                    self.as_ref().to_glib_none().0,
                    ptr::null::<c_char>(),
                )
            },
        }
    }

    #[doc(alias = "gtk_message_dialog_format_secondary_text")]
    fn format_secondary_text(&self, message: Option<&str>) {
        match message {
            Some(m) => unsafe {
                let message: Stash<*const c_char, _> = m.to_glib_none();
                ffi::gtk_message_dialog_format_secondary_text(
                    self.as_ref().to_glib_none().0,
                    c"%s".as_ptr() as *const c_char,
                    message.0,
                    ptr::null::<c_char>(),
                )
            },
            None => unsafe {
                ffi::gtk_message_dialog_format_secondary_text(
                    self.as_ref().to_glib_none().0,
                    ptr::null::<c_char>(),
                )
            },
        }
    }
}

impl<O: IsA<MessageDialog>> MessageDialogExtManual for O {}
