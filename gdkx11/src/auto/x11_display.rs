// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use gdk;
use glib;
#[cfg(any(feature = "v3_16", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;
use std::ptr;

glib::glib_wrapper! {
    pub struct X11Display(Object<ffi::GdkX11Display, ffi::GdkX11DisplayClass>) @extends gdk::Display;

    match fn {
        get_type => || ffi::gdk_x11_display_get_type(),
    }
}

impl X11Display {
    //pub fn broadcast_startup_message(&self, message_type: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gdk_x11_display_broadcast_startup_message() }
    //}

    pub fn error_trap_pop(&self) -> i32 {
        unsafe { ffi::gdk_x11_display_error_trap_pop(self.to_glib_none().0) }
    }

    pub fn error_trap_pop_ignored(&self) {
        unsafe {
            ffi::gdk_x11_display_error_trap_pop_ignored(self.to_glib_none().0);
        }
    }

    pub fn error_trap_push(&self) {
        unsafe {
            ffi::gdk_x11_display_error_trap_push(self.to_glib_none().0);
        }
    }

    pub fn get_startup_notification_id(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gdk_x11_display_get_startup_notification_id(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_user_time(&self) -> u32 {
        unsafe { ffi::gdk_x11_display_get_user_time(self.to_glib_none().0) }
    }

    pub fn grab(&self) {
        unsafe {
            ffi::gdk_x11_display_grab(self.to_glib_none().0);
        }
    }

    pub fn set_cursor_theme(&self, theme: Option<&str>, size: i32) {
        unsafe {
            ffi::gdk_x11_display_set_cursor_theme(
                self.to_glib_none().0,
                theme.to_glib_none().0,
                size,
            );
        }
    }

    pub fn set_startup_notification_id(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_x11_display_set_startup_notification_id(
                self.to_glib_none().0,
                startup_id.to_glib_none().0,
            );
        }
    }

    pub fn set_window_scale(&self, scale: i32) {
        unsafe {
            ffi::gdk_x11_display_set_window_scale(self.to_glib_none().0, scale);
        }
    }

    pub fn string_to_compound_text(&self, str: &str) -> (i32, gdk::Atom, i32, Vec<u8>) {
        unsafe {
            let mut encoding = gdk::Atom::uninitialized();
            let mut format = mem::MaybeUninit::uninit();
            let mut ctext = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = ffi::gdk_x11_display_string_to_compound_text(
                self.to_glib_none().0,
                str.to_glib_none().0,
                encoding.to_glib_none_mut().0,
                format.as_mut_ptr(),
                &mut ctext,
                length.as_mut_ptr(),
            );
            let format = format.assume_init();
            (
                ret,
                encoding,
                format,
                FromGlibContainer::from_glib_full_num(ctext, length.assume_init() as usize),
            )
        }
    }

    pub fn ungrab(&self) {
        unsafe {
            ffi::gdk_x11_display_ungrab(self.to_glib_none().0);
        }
    }

    pub fn utf8_to_compound_text(&self, str: &str) -> Option<(gdk::Atom, i32, Vec<u8>)> {
        unsafe {
            let mut encoding = gdk::Atom::uninitialized();
            let mut format = mem::MaybeUninit::uninit();
            let mut ctext = ptr::null_mut();
            let mut length = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_x11_display_utf8_to_compound_text(
                self.to_glib_none().0,
                str.to_glib_none().0,
                encoding.to_glib_none_mut().0,
                format.as_mut_ptr(),
                &mut ctext,
                length.as_mut_ptr(),
            ));
            let format = format.assume_init();
            if ret {
                Some((
                    encoding,
                    format,
                    FromGlibContainer::from_glib_full_num(ctext, length.assume_init() as usize),
                ))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_16")))]
    pub fn get_glx_version<P: IsA<gdk::Display>>(display: &P) -> Option<(i32, i32)> {
        assert_initialized_main_thread!();
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gdk_x11_display_get_glx_version(
                display.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            ));
            let major = major.assume_init();
            let minor = minor.assume_init();
            if ret {
                Some((major, minor))
            } else {
                None
            }
        }
    }
}

impl fmt::Display for X11Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11Display")
    }
}
