// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::MenuModel;
use glib;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;
use std::ptr;

glib::glib_wrapper! {
    pub struct MenuLinkIter(Object<ffi::GMenuLinkIter, ffi::GMenuLinkIterClass>);

    match fn {
        get_type => || ffi::g_menu_link_iter_get_type(),
    }
}

pub const NONE_MENU_LINK_ITER: Option<&MenuLinkIter> = None;

pub trait MenuLinkIterExt: 'static {
    fn get_name(&self) -> Option<glib::GString>;

    fn get_next(&self) -> Option<(glib::GString, MenuModel)>;

    fn get_value(&self) -> Option<MenuModel>;

    fn next(&self) -> bool;
}

impl<O: IsA<MenuLinkIter>> MenuLinkIterExt for O {
    fn get_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_menu_link_iter_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_next(&self) -> Option<(glib::GString, MenuModel)> {
        unsafe {
            let mut out_link = ptr::null();
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::g_menu_link_iter_get_next(
                self.as_ref().to_glib_none().0,
                &mut out_link,
                &mut value,
            ));
            if ret {
                Some((from_glib_none(out_link), from_glib_full(value)))
            } else {
                None
            }
        }
    }

    fn get_value(&self) -> Option<MenuModel> {
        unsafe {
            from_glib_full(ffi::g_menu_link_iter_get_value(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn next(&self) -> bool {
        unsafe { from_glib(ffi::g_menu_link_iter_next(self.as_ref().to_glib_none().0)) }
    }
}

impl fmt::Display for MenuLinkIter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MenuLinkIter")
    }
}
