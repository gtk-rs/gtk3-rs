// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Script;
use glib::translate::*;
use glib::GString;

pub struct Language(*mut ffi::PangoLanguage);

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::PangoLanguage> for &'a Language {
    type Storage = &'a Language;

    fn to_glib_none(&self) -> Stash<'a, *mut ffi::PangoLanguage, Self> {
        Stash(self.0, *self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::PangoLanguage> for Language {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::PangoLanguage, Self> {
        StashMut(self.0, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::PangoLanguage> for Language {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Self(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::PangoLanguage> for Language {
    unsafe fn from_glib_full(ptr: *mut ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Self(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::PangoLanguage> for Language {
    unsafe fn from_glib_none(ptr: *const ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Self(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const ffi::PangoLanguage> for Language {
    unsafe fn from_glib_full(ptr: *const ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Self(ptr as *mut _)
    }
}

impl Default for Language {
    fn default() -> Self {
        unsafe { from_glib_full(ffi::pango_language_get_default()) }
    }
}

impl Language {
    pub fn from_string(language: &str) -> Self {
        unsafe { from_glib_full(ffi::pango_language_from_string(language.to_glib_none().0)) }
    }

    pub fn to_string(&self) -> GString {
        unsafe { from_glib_none(ffi::pango_language_to_string(self.to_glib_none().0)) }
    }

    pub fn matches(&self, range_list: &str) -> bool {
        unsafe {
            from_glib(ffi::pango_language_matches(
                self.to_glib_none().0,
                range_list.to_glib_none().0,
            ))
        }
    }

    pub fn includes_script(&self, script: Script) -> bool {
        unsafe {
            from_glib(ffi::pango_language_includes_script(
                self.to_glib_none().0,
                script.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_scripts")]
    pub fn scripts(&self) -> Vec<Script> {
        let mut num_scripts = 0;
        let mut ret = Vec::new();

        unsafe {
            let scripts: *const ffi::PangoScript =
                ffi::pango_language_get_scripts(self.to_glib_none().0, &mut num_scripts);
            if num_scripts > 0 {
                for x in 0..num_scripts {
                    ret.push(from_glib(
                        *(scripts.offset(x as isize) as *const ffi::PangoScript),
                    ));
                }
            }
            ret
        }
    }

    #[doc(alias = "get_sample_string")]
    pub fn sample_string(&self) -> GString {
        unsafe { from_glib_none(ffi::pango_language_get_sample_string(self.to_glib_none().0)) }
    }
}
