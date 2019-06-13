// Copyright 2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use glib::GString;
use pango_sys;
use Script;

pub struct Language(*mut pango_sys::PangoLanguage);

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut pango_sys::PangoLanguage> for &'a Language {
    type Storage = &'a Language;

    fn to_glib_none(&self) -> Stash<'a, *mut pango_sys::PangoLanguage, Self> {
        Stash(self.0, *self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut pango_sys::PangoLanguage> for Language {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut pango_sys::PangoLanguage, Self> {
        StashMut(self.0, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut pango_sys::PangoLanguage> for Language {
    unsafe fn from_glib_none(ptr: *mut pango_sys::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Language(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut pango_sys::PangoLanguage> for Language {
    unsafe fn from_glib_full(ptr: *mut pango_sys::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Language(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const pango_sys::PangoLanguage> for Language {
    unsafe fn from_glib_none(ptr: *const pango_sys::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Language(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const pango_sys::PangoLanguage> for Language {
    unsafe fn from_glib_full(ptr: *const pango_sys::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Language(ptr as *mut _)
    }
}

impl Default for Language {
    fn default() -> Language {
        unsafe { from_glib_full(pango_sys::pango_language_get_default()) }
    }
}

impl Language {
    pub fn from_string(language: &str) -> Language {
        unsafe {
            from_glib_full(pango_sys::pango_language_from_string(
                language.to_glib_none().0,
            ))
        }
    }

    pub fn to_string(&self) -> GString {
        unsafe { from_glib_none(pango_sys::pango_language_to_string(self.to_glib_none().0)) }
    }

    pub fn matches(&self, range_list: &str) -> bool {
        unsafe {
            pango_sys::pango_language_matches(self.to_glib_none().0, range_list.to_glib_none().0)
                .to_bool()
        }
    }

    pub fn includes_script(&self, script: Script) -> bool {
        unsafe {
            pango_sys::pango_language_includes_script(self.to_glib_none().0, script.to_glib())
                .to_bool()
        }
    }

    pub fn get_scripts(&self) -> Vec<Script> {
        let mut num_scripts = 0;
        let mut ret = Vec::new();

        unsafe {
            let scripts: *const pango_sys::PangoScript =
                pango_sys::pango_language_get_scripts(self.to_glib_none().0, &mut num_scripts);
            if num_scripts > 0 {
                for x in 0..num_scripts {
                    ret.push(from_glib(
                        *(scripts.offset(x as isize) as *const pango_sys::PangoScript),
                    ));
                }
            }
            ret
        }
    }

    pub fn get_sample_string(&self) -> GString {
        unsafe {
            from_glib_none(pango_sys::pango_language_get_sample_string(
                self.to_glib_none().0,
            ))
        }
    }
}
