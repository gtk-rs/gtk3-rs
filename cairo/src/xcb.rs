// Take a look at the license at the top of the repository in the LICENSE file.

use crate::enums::SurfaceType;
#[cfg(feature = "use_glib")]
use glib::translate::*;
use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;
use std::ptr;

use crate::error::Error;
use crate::surface::Surface;
use crate::utils::status_to_result;

#[derive(Debug)]
pub struct XCBDrawable(pub u32);

impl XCBDrawable {
    fn to_raw_none(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for XCBDrawable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XCBDrawable")
    }
}

#[derive(Debug)]
pub struct XCBPixmap(pub u32);

impl XCBPixmap {
    fn to_raw_none(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for XCBPixmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XCBPixmap")
    }
}

#[derive(Debug)]
pub struct XCBConnection(pub ptr::NonNull<ffi::xcb_connection_t>);

impl XCBConnection {
    pub fn to_raw_none(&self) -> *mut ffi::xcb_connection_t {
        self.0.as_ptr()
    }

    pub unsafe fn from_raw_none(ptr: *mut ffi::xcb_connection_t) -> XCBConnection {
        assert!(!ptr.is_null());
        XCBConnection(ptr::NonNull::new_unchecked(ptr))
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::xcb_connection_t) -> Borrowed<XCBConnection> {
        assert!(!ptr.is_null());
        Borrowed::new(XCBConnection(ptr::NonNull::new_unchecked(ptr)))
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::xcb_connection_t) -> XCBConnection {
        assert!(!ptr.is_null());
        XCBConnection(ptr::NonNull::new_unchecked(ptr))
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
    unsafe fn from_glib_borrow(ptr: *mut ffi::xcb_connection_t) -> Borrowed<XCBConnection> {
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

impl Clone for XCBConnection {
    fn clone(&self) -> XCBConnection {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}

impl fmt::Display for XCBConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XCBConnection")
    }
}

#[derive(Debug)]
pub struct XCBRenderPictFormInfo(pub ptr::NonNull<ffi::xcb_render_pictforminfo_t>);

impl XCBRenderPictFormInfo {
    pub fn to_raw_none(&self) -> *mut ffi::xcb_render_pictforminfo_t {
        self.0.as_ptr()
    }

    pub unsafe fn from_raw_none(ptr: *mut ffi::xcb_render_pictforminfo_t) -> XCBRenderPictFormInfo {
        assert!(!ptr.is_null());
        XCBRenderPictFormInfo(ptr::NonNull::new_unchecked(ptr))
    }

    pub unsafe fn from_raw_borrow(
        ptr: *mut ffi::xcb_render_pictforminfo_t,
    ) -> Borrowed<XCBRenderPictFormInfo> {
        assert!(!ptr.is_null());
        Borrowed::new(XCBRenderPictFormInfo(ptr::NonNull::new_unchecked(ptr)))
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::xcb_render_pictforminfo_t) -> XCBRenderPictFormInfo {
        assert!(!ptr.is_null());
        XCBRenderPictFormInfo(ptr::NonNull::new_unchecked(ptr))
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::xcb_render_pictforminfo_t> for &'a XCBRenderPictFormInfo {
    type Storage = &'a XCBRenderPictFormInfo;

    #[inline]
    fn to_glib_none(
        &self,
    ) -> Stash<'a, *mut ffi::xcb_render_pictforminfo_t, &'a XCBRenderPictFormInfo> {
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
    unsafe fn from_glib_borrow(
        ptr: *mut ffi::xcb_render_pictforminfo_t,
    ) -> Borrowed<XCBRenderPictFormInfo> {
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

impl Clone for XCBRenderPictFormInfo {
    fn clone(&self) -> XCBRenderPictFormInfo {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}

impl fmt::Display for XCBRenderPictFormInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XCBRenderPictFormInfo")
    }
}

#[derive(Debug)]
pub struct XCBScreen(pub ptr::NonNull<ffi::xcb_screen_t>);

impl XCBScreen {
    pub fn to_raw_none(&self) -> *mut ffi::xcb_screen_t {
        self.0.as_ptr()
    }

    pub unsafe fn from_raw_none(ptr: *mut ffi::xcb_screen_t) -> XCBScreen {
        assert!(!ptr.is_null());
        XCBScreen(ptr::NonNull::new_unchecked(ptr))
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::xcb_screen_t) -> Borrowed<XCBScreen> {
        assert!(!ptr.is_null());
        Borrowed::new(XCBScreen(ptr::NonNull::new_unchecked(ptr)))
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::xcb_screen_t) -> XCBScreen {
        assert!(!ptr.is_null());
        XCBScreen(ptr::NonNull::new_unchecked(ptr))
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
    unsafe fn from_glib_borrow(ptr: *mut ffi::xcb_screen_t) -> Borrowed<XCBScreen> {
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

impl Clone for XCBScreen {
    fn clone(&self) -> XCBScreen {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}

impl fmt::Display for XCBScreen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XCBScreen")
    }
}

declare_surface!(XCBSurface, SurfaceType::Xcb);

impl XCBSurface {
    #[doc(alias = "cairo_xcb_surface_create")]
    pub fn create(
        connection: &XCBConnection,
        drawable: &XCBDrawable,
        visual: &XCBVisualType,
        width: i32,
        height: i32,
    ) -> Result<Self, Error> {
        unsafe {
            Ok(Self::from_raw_full(ffi::cairo_xcb_surface_create(
                connection.to_raw_none(),
                drawable.to_raw_none(),
                visual.to_raw_none(),
                width,
                height,
            ))?)
        }
    }

    #[doc(alias = "cairo_xcb_surface_create_for_bitmap")]
    pub fn create_for_bitmap(
        connection: &XCBConnection,
        screen: &XCBScreen,
        bitmap: &XCBPixmap,
        width: i32,
        height: i32,
    ) -> Result<Self, Error> {
        unsafe {
            Ok(Self(Surface::from_raw_full(
                ffi::cairo_xcb_surface_create_for_bitmap(
                    connection.to_raw_none(),
                    screen.to_raw_none(),
                    bitmap.to_raw_none(),
                    width,
                    height,
                ),
            )?))
        }
    }

    #[doc(alias = "cairo_xcb_surface_create_with_xrender_format")]
    pub fn create_with_xrender_format(
        connection: &XCBConnection,
        screen: &XCBScreen,
        bitmap: &XCBPixmap,
        format: &XCBRenderPictFormInfo,
        width: i32,
        height: i32,
    ) -> Result<Self, Error> {
        unsafe {
            Ok(Self(Surface::from_raw_full(
                ffi::cairo_xcb_surface_create_with_xrender_format(
                    connection.to_raw_none(),
                    screen.to_raw_none(),
                    bitmap.to_raw_none(),
                    format.to_raw_none(),
                    width,
                    height,
                ),
            )?))
        }
    }

    fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_surface_status(self.to_raw_none()) };
        status_to_result(status)
    }

    #[doc(alias = "cairo_xcb_surface_set_size")]
    pub fn set_size(&self, width: i32, height: i32) -> Result<(), Error> {
        unsafe { ffi::cairo_xcb_surface_set_size(self.to_raw_none(), width, height) }
        self.status()
    }

    #[doc(alias = "cairo_xcb_surface_set_drawable")]
    pub fn set_drawable(
        &self,
        drawable: &XCBDrawable,
        width: i32,
        height: i32,
    ) -> Result<(), Error> {
        unsafe {
            ffi::cairo_xcb_surface_set_drawable(
                self.to_raw_none(),
                drawable.to_raw_none(),
                width,
                height,
            )
        }
        self.status()
    }
}

#[derive(Debug)]
pub struct XCBVisualType(pub ptr::NonNull<ffi::xcb_visualtype_t>);

impl XCBVisualType {
    pub fn to_raw_none(&self) -> *mut ffi::xcb_visualtype_t {
        self.0.as_ptr()
    }

    pub unsafe fn from_raw_none(ptr: *mut ffi::xcb_visualtype_t) -> XCBVisualType {
        assert!(!ptr.is_null());
        XCBVisualType(ptr::NonNull::new_unchecked(ptr))
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::xcb_visualtype_t) -> Borrowed<XCBVisualType> {
        assert!(!ptr.is_null());
        Borrowed::new(XCBVisualType(ptr::NonNull::new_unchecked(ptr)))
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::xcb_visualtype_t) -> XCBVisualType {
        assert!(!ptr.is_null());
        XCBVisualType(ptr::NonNull::new_unchecked(ptr))
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
    unsafe fn from_glib_borrow(ptr: *mut ffi::xcb_visualtype_t) -> Borrowed<XCBVisualType> {
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

impl Clone for XCBVisualType {
    fn clone(&self) -> XCBVisualType {
        unsafe { Self::from_raw_none(self.to_raw_none()) }
    }
}

impl fmt::Display for XCBVisualType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XCBVisualType")
    }
}

impl crate::device::Device {
    #[doc(alias = "cairo_xcb_device_get_connection")]
    pub fn get_connection(&self) -> XCBConnection {
        unsafe {
            XCBConnection::from_raw_full(ffi::cairo_xcb_device_get_connection(self.to_raw_none()))
        }
    }

    #[doc(alias = "cairo_xcb_device_debug_cap_xshm_version")]
    pub fn debug_cap_xshm_version(&self, major_version: i32, minor_version: i32) {
        unsafe {
            ffi::cairo_xcb_device_debug_cap_xshm_version(
                self.to_raw_none(),
                major_version,
                minor_version,
            )
        }
    }
}
