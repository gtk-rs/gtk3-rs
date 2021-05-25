// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

/// A [KeymapKey](crate::KeymapKey) is a hardware key that can be mapped to a keyval.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct KeymapKey {
    pub keycode: u32,
    pub group: i32,
    pub level: i32,
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GdkKeymapKey> for KeymapKey {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GdkKeymapKey, Self> {
        let ptr: *const KeymapKey = &*self;
        Stash(ptr as *const ffi::GdkKeymapKey, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GdkKeymapKey> for KeymapKey {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GdkKeymapKey, Self> {
        let ptr: *mut KeymapKey = &mut *self;
        StashMut(ptr as *mut ffi::GdkKeymapKey, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GdkKeymapKey> for KeymapKey {
    unsafe fn from_glib_none(ptr: *const ffi::GdkKeymapKey) -> Self {
        *(ptr as *const KeymapKey)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkKeymapKey> for KeymapKey {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkKeymapKey) -> Self {
        *(ptr as *mut KeymapKey)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkKeymapKey> for KeymapKey {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GdkKeymapKey) -> Self {
        let geom = *(ptr as *mut KeymapKey);
        glib::ffi::g_free(ptr as *mut _);
        geom
    }
}
