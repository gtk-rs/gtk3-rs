// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::mem;

glib::glib_wrapper! {
    pub struct Editable(Interface<ffi::GtkEditable>);

    match fn {
        get_type => || ffi::gtk_editable_get_type(),
    }
}

pub const NONE_EDITABLE: Option<&Editable> = None;

pub trait EditableExt: 'static {
    fn copy_clipboard(&self);

    fn cut_clipboard(&self);

    fn delete_selection(&self);

    fn delete_text(&self, start_pos: i32, end_pos: i32);

    fn get_chars(&self, start_pos: i32, end_pos: i32) -> Option<glib::GString>;

    fn get_editable(&self) -> bool;

    fn get_position(&self) -> i32;

    fn get_selection_bounds(&self) -> Option<(i32, i32)>;

    fn insert_text(&self, new_text: &str, position: &mut i32);

    fn paste_clipboard(&self);

    fn select_region(&self, start_pos: i32, end_pos: i32);

    fn set_editable(&self, is_editable: bool);

    fn set_position(&self, position: i32);
}

impl<O: IsA<Editable>> EditableExt for O {
    fn copy_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_copy_clipboard(self.as_ref().to_glib_none().0);
        }
    }

    fn cut_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_cut_clipboard(self.as_ref().to_glib_none().0);
        }
    }

    fn delete_selection(&self) {
        unsafe {
            ffi::gtk_editable_delete_selection(self.as_ref().to_glib_none().0);
        }
    }

    fn delete_text(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_delete_text(self.as_ref().to_glib_none().0, start_pos, end_pos);
        }
    }

    fn get_chars(&self, start_pos: i32, end_pos: i32) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_editable_get_chars(
                self.as_ref().to_glib_none().0,
                start_pos,
                end_pos,
            ))
        }
    }

    fn get_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_editable_get_editable(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_position(&self) -> i32 {
        unsafe { ffi::gtk_editable_get_position(self.as_ref().to_glib_none().0) }
    }

    fn get_selection_bounds(&self) -> Option<(i32, i32)> {
        unsafe {
            let mut start_pos = mem::MaybeUninit::uninit();
            let mut end_pos = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_editable_get_selection_bounds(
                self.as_ref().to_glib_none().0,
                start_pos.as_mut_ptr(),
                end_pos.as_mut_ptr(),
            ));
            let start_pos = start_pos.assume_init();
            let end_pos = end_pos.assume_init();
            if ret {
                Some((start_pos, end_pos))
            } else {
                None
            }
        }
    }

    fn insert_text(&self, new_text: &str, position: &mut i32) {
        let new_text_length = new_text.len() as i32;
        unsafe {
            ffi::gtk_editable_insert_text(
                self.as_ref().to_glib_none().0,
                new_text.to_glib_none().0,
                new_text_length,
                position,
            );
        }
    }

    fn paste_clipboard(&self) {
        unsafe {
            ffi::gtk_editable_paste_clipboard(self.as_ref().to_glib_none().0);
        }
    }

    fn select_region(&self, start_pos: i32, end_pos: i32) {
        unsafe {
            ffi::gtk_editable_select_region(self.as_ref().to_glib_none().0, start_pos, end_pos);
        }
    }

    fn set_editable(&self, is_editable: bool) {
        unsafe {
            ffi::gtk_editable_set_editable(self.as_ref().to_glib_none().0, is_editable.to_glib());
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_editable_set_position(self.as_ref().to_glib_none().0, position);
        }
    }
}

impl fmt::Display for Editable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Editable")
    }
}
