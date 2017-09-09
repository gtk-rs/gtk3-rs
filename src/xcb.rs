// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[cfg(feature = "use_glib")]
use glib::translate::*;
use ffi;

use surface::Surface;


pub struct XCBDrawable(pub u32);

impl XCBDrawable {
    fn to_raw_none(&self) -> u32 {
        self.0
    }
}


pub struct XCBPixmap(pub u32);

impl XCBPixmap {
    fn to_raw_none(&self) -> u32 {
        self.0
    }
}


pub struct XCBConnection(pub *mut ffi::xcb_connection_t);

impl XCBConnection {
    pub fn to_raw_none(&self) -> *mut ffi::xcb_connection_t {
        self.0
    }

    pub unsafe fn from_raw_none(ptr: *mut ffi::xcb_connection_t) -> XCBConnection {
        assert!(!ptr.is_null());
        XCBConnection(ptr)
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::xcb_connection_t) -> XCBConnection {
        assert!(!ptr.is_null());
        XCBConnection(ptr)
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::xcb_connection_t) -> XCBConnection {
        assert!(!ptr.is_null());
        XCBConnection(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::xcb_connection_t> for &'a XCBConnection {
    type Storage = &'a XCBConnection;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::xcb_connection_t, &'a XCBConnection> {
        Stash(self.to_raw_none(), *self)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::xcb_connection_t> for XCBConnection {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::xcb_connection_t) -> XCBConnection {
        Self::from_raw_none(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::xcb_connection_t> for XCBConnection {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::xcb_connection_t) -> XCBConnection {
        Self::from_raw_borrow(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::xcb_connection_t> for XCBConnection {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::xcb_connection_t) -> XCBConnection {
        Self::from_raw_full(ptr)
    }
}

impl AsRef<XCBConnection> for XCBConnection {
    fn as_ref(&self) -> &XCBConnection {
        self
    }
}

impl Clone for XCBConnection {
    fn clone(&self) -> XCBConnection {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}


pub struct XCBRenderPictFormInfo(pub *mut ffi::xcb_render_pictforminfo_t);

impl XCBRenderPictFormInfo {
    pub fn to_raw_none(&self) ->  *mut ffi::xcb_render_pictforminfo_t {
        self.0
    }

    pub unsafe fn from_raw_none(ptr: *mut ffi::xcb_render_pictforminfo_t) -> XCBRenderPictFormInfo {
        assert!(!ptr.is_null());
        XCBRenderPictFormInfo(ptr)
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::xcb_render_pictforminfo_t) -> XCBRenderPictFormInfo {
        assert!(!ptr.is_null());
        XCBRenderPictFormInfo(ptr)
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::xcb_render_pictforminfo_t) -> XCBRenderPictFormInfo {
        assert!(!ptr.is_null());
        XCBRenderPictFormInfo(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::xcb_render_pictforminfo_t> for &'a XCBRenderPictFormInfo {
    type Storage = &'a XCBRenderPictFormInfo;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::xcb_render_pictforminfo_t, &'a XCBRenderPictFormInfo> {
        Stash(self.to_raw_none(), *self)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::xcb_render_pictforminfo_t> for XCBRenderPictFormInfo {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::xcb_render_pictforminfo_t) -> XCBRenderPictFormInfo {
        Self::from_raw_none(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::xcb_render_pictforminfo_t> for XCBRenderPictFormInfo {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::xcb_render_pictforminfo_t) -> XCBRenderPictFormInfo {
        Self::from_raw_borrow(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::xcb_render_pictforminfo_t> for XCBRenderPictFormInfo {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::xcb_render_pictforminfo_t) -> XCBRenderPictFormInfo {
        Self::from_raw_full(ptr)
    }
}

impl AsRef<XCBRenderPictFormInfo> for XCBRenderPictFormInfo {
    fn as_ref(&self) -> &XCBRenderPictFormInfo {
        self
    }
}

impl Clone for XCBRenderPictFormInfo {
    fn clone(&self) -> XCBRenderPictFormInfo {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}


pub struct XCBScreen(pub *mut ffi::xcb_screen_t);

impl XCBScreen {
    pub fn to_raw_none(&self) -> *mut ffi::xcb_screen_t {
        self.0
    }

    pub unsafe fn from_raw_none(ptr: *mut ffi::xcb_screen_t) -> XCBScreen {
        assert!(!ptr.is_null());
        XCBScreen(ptr)
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::xcb_screen_t) -> XCBScreen {
        assert!(!ptr.is_null());
        XCBScreen(ptr)
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::xcb_screen_t) -> XCBScreen {
        assert!(!ptr.is_null());
        XCBScreen(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::xcb_screen_t> for &'a XCBScreen {
    type Storage = &'a XCBScreen;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::xcb_screen_t, &'a XCBScreen> {
        Stash(self.to_raw_none(), *self)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::xcb_screen_t> for XCBScreen {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::xcb_screen_t) -> XCBScreen {
        Self::from_raw_none(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::xcb_screen_t> for XCBScreen {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::xcb_screen_t) -> XCBScreen {
        Self::from_raw_borrow(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::xcb_screen_t> for XCBScreen {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::xcb_screen_t) -> XCBScreen {
        Self::from_raw_full(ptr)
    }
}

impl AsRef<XCBScreen> for XCBScreen {
    fn as_ref(&self) -> &XCBScreen {
        self
    }
}

impl Clone for XCBScreen {
    fn clone(&self) -> XCBScreen {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}


pub trait XCBSurface {
    fn create(connection: &XCBConnection, drawable: &XCBDrawable, visual: &XCBVisualType,
              width: i32, height: i32) -> Surface;
    fn create_for_bitmap(connection: &XCBConnection, screen: &XCBScreen, bitmap: &XCBPixmap,
                         width: i32, height: i32) -> Surface;
    fn create_with_xrender_format(connection: &XCBConnection, screen: &XCBScreen,
                                  bitmap: &XCBPixmap, format: &XCBRenderPictFormInfo,
                                  width: i32, height: i32) -> Surface;
    fn set_size(&self, width: i32, height: i32);
    fn set_drawable(&self, drawable: &XCBDrawable, width: i32, height: i32);
}

impl XCBSurface for Surface {
    fn create(connection: &XCBConnection, drawable: &XCBDrawable, visual: &XCBVisualType,
              width: i32, height: i32) -> Surface {
        unsafe {
            Surface::from_raw_full(ffi::cairo_xcb_surface_create(connection.to_raw_none(),
                                                                 drawable.to_raw_none(),
                                                                 visual.to_raw_none(),
                                                                 width,
                                                                 height))
        }
    }

    fn create_for_bitmap(connection: &XCBConnection, screen: &XCBScreen, bitmap: &XCBPixmap,
                         width: i32, height: i32) -> Surface {
        unsafe {
            Surface::from_raw_full(ffi::cairo_xcb_surface_create_for_bitmap(connection.to_raw_none(),
                                                                            screen.to_raw_none(),
                                                                            bitmap.to_raw_none(),
                                                                            width,
                                                                            height))
        }
    }

    fn create_with_xrender_format(connection: &XCBConnection, screen: &XCBScreen,
                                  bitmap: &XCBPixmap, format: &XCBRenderPictFormInfo,
                                  width: i32, height: i32) -> Surface {
        unsafe {
            Surface::from_raw_full(ffi::cairo_xcb_surface_create_with_xrender_format(connection.to_raw_none(),
                                                                                     screen.to_raw_none(),
                                                                                     bitmap.to_raw_none(),
                                                                                     format.to_raw_none(),
                                                                                     width,
                                                                                     height))
        }
    }

    fn set_size(self: &Surface, width: i32, height: i32) {
        unsafe {
            ffi::cairo_xcb_surface_set_size(self.to_raw_none(), width, height)
        }
    }

    fn set_drawable(self: &Surface, drawable: &XCBDrawable, width: i32, height: i32) {
        unsafe {
            ffi::cairo_xcb_surface_set_drawable(self.to_raw_none(), drawable.to_raw_none(),
                                                width, height)
        }
    }
}


pub struct XCBVisualType(pub *mut ffi::xcb_visualtype_t);

impl XCBVisualType {
    pub fn to_raw_none(&self) -> *mut ffi::xcb_visualtype_t {
        self.0
    }

    pub unsafe fn from_raw_none(ptr: *mut ffi::xcb_visualtype_t) -> XCBVisualType {
        assert!(!ptr.is_null());
        XCBVisualType(ptr)
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::xcb_visualtype_t) -> XCBVisualType {
        assert!(!ptr.is_null());
        XCBVisualType(ptr)
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::xcb_visualtype_t) -> XCBVisualType {
        assert!(!ptr.is_null());
        XCBVisualType(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::xcb_visualtype_t> for &'a XCBVisualType {
    type Storage = &'a XCBVisualType;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::xcb_visualtype_t, &'a XCBVisualType> {
        Stash(self.to_raw_none(), *self)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::xcb_visualtype_t> for XCBVisualType {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::xcb_visualtype_t) -> XCBVisualType {
        Self::from_raw_none(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::xcb_visualtype_t> for XCBVisualType {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::xcb_visualtype_t) -> XCBVisualType {
        Self::from_raw_borrow(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::xcb_visualtype_t> for XCBVisualType {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::xcb_visualtype_t) -> XCBVisualType {
        Self::from_raw_full(ptr)
    }
}

impl AsRef<XCBVisualType> for XCBVisualType {
    fn as_ref(&self) -> &XCBVisualType {
        self
    }
}

impl Clone for XCBVisualType {
    fn clone(&self) -> XCBVisualType {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}


pub struct Device(pub *mut ffi::cairo_device_t);

impl Device {
    pub fn to_raw_none(&self) -> *mut ffi::cairo_device_t {
        self.0
    }

    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_device_t) -> Device {
        assert!(!ptr.is_null());
        Device(ptr)
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::cairo_device_t) -> Device {
        assert!(!ptr.is_null());
        Device(ptr)
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_device_t) -> Device {
        assert!(!ptr.is_null());
        Device(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_device_t> for &'a Device {
    type Storage = &'a Device;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::cairo_device_t, &'a Device> {
        Stash(self.to_raw_none(), *self)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrNone<*mut ffi::cairo_device_t> for Device {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_device_t) -> Device {
        Self::from_raw_none(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrBorrow<*mut ffi::cairo_device_t> for Device {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_device_t) -> Device {
        Self::from_raw_borrow(ptr)
    }
}

#[cfg(feature = "use_glib")]
impl FromGlibPtrFull<*mut ffi::cairo_device_t> for Device {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::cairo_device_t) -> Device {
        Self::from_raw_full(ptr)
    }
}

impl AsRef<Device> for Device {
    fn as_ref(&self) -> &Device {
        self
    }
}

impl Clone for Device {
    fn clone(&self) -> Device {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}

impl Device {
    pub fn get_connection(&self) -> XCBConnection {
        unsafe {
            XCBConnection::from_raw_full(ffi::cairo_xcb_device_get_connection(self.to_raw_none()))
        }
    }

    pub fn debug_cap_xrender_version(&self, major_version: i32, minor_version: i32) {
        unsafe {
            ffi::cairo_xcb_device_debug_cap_xrender_version(self.to_raw_none(),
                                                            major_version,
                                                            minor_version)
        }
    }

    pub fn debug_cap_xshm_version(&self, major_version: i32, minor_version: i32) {
        unsafe {
            ffi::cairo_xcb_device_debug_cap_xshm_version(self.to_raw_none(),
                                                         major_version,
                                                         minor_version)
        }
    }

    pub fn debug_get_precision(&self) -> i32 {
        unsafe {
            ffi::cairo_xcb_device_debug_get_precision(self.to_raw_none())
        }
    }

    pub fn debug_set_precision(&self, precision: i32) {
        unsafe {
            ffi::cairo_xcb_device_debug_set_precision(self.to_raw_none(), precision)
        }
    }
}
