// Copyright 2015-2016, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ffi::CString;
use std::path::Path;

use enums::{Content, DeviceType, ScriptMode, Status};
use ffi;
#[cfg(feature = "use_glib")]
use glib::translate::*;
use recording_surface::RecordingSurface;
use std::fmt;
use surface::Surface;

#[derive(Debug)]
pub struct Device(*mut ffi::cairo_device_t, bool);

impl Device {
    pub unsafe fn from_raw_none(ptr: *mut ffi::cairo_device_t) -> Device {
        assert!(!ptr.is_null());
        ffi::cairo_device_reference(ptr);
        Device(ptr, false)
    }

    pub unsafe fn from_raw_borrow(ptr: *mut ffi::cairo_device_t) -> Device {
        assert!(!ptr.is_null());
        Device(ptr, true)
    }

    pub unsafe fn from_raw_full(ptr: *mut ffi::cairo_device_t) -> Device {
        assert!(!ptr.is_null());
        Device(ptr, false)
    }

    pub fn to_raw_none(&self) -> *mut ffi::cairo_device_t {
        self.0
    }

    pub fn create<P: AsRef<Path>>(filename: P) -> Option<Device> {
        unsafe {
            let filename = filename.as_ref().to_string_lossy().into_owned();
            let filename = CString::new(filename).unwrap();
            let p = ffi::cairo_script_create(filename.as_ptr());
            if p.is_null() {
                None
            } else {
                Some(Self::from_raw_full(p))
            }
        }
    }

    pub fn from_recording_surface(&self, surface: &RecordingSurface) -> Status {
        unsafe {
            Status::from(ffi::cairo_script_from_recording_surface(
                self.to_raw_none(),
                surface.to_raw_none(),
            ))
        }
    }

    pub fn get_mode(&self) -> ScriptMode {
        unsafe { ScriptMode::from(ffi::cairo_script_get_mode(self.to_raw_none())) }
    }

    pub fn set_mode(&self, mode: ScriptMode) {
        unsafe { ffi::cairo_script_set_mode(self.to_raw_none(), mode.into()) }
    }

    pub fn surface_create(
        &self,
        content: Content,
        width: f64,
        height: f64,
    ) -> Result<Surface, Status> {
        unsafe {
            Ok(Surface::from_raw_full(ffi::cairo_script_surface_create(
                self.to_raw_none(),
                content.into(),
                width,
                height,
            ))?)
        }
    }

    pub fn surface_create_for_target(&self, target: &Surface) -> Result<Surface, Status> {
        unsafe {
            Ok(Surface::from_raw_full(
                ffi::cairo_script_surface_create_for_target(
                    self.to_raw_none(),
                    target.to_raw_none(),
                ),
            )?)
        }
    }

    pub fn write_comment(&self, comment: &str) {
        unsafe {
            let len = comment.len();
            let comment = CString::new(comment).unwrap();
            ffi::cairo_script_write_comment(self.to_raw_none(), comment.as_ptr(), len as i32)
        }
    }

    pub fn status(&self) -> Status {
        unsafe { Status::from(ffi::cairo_device_status(self.to_raw_none())) }
    }

    pub fn finish(&self) {
        unsafe { ffi::cairo_device_finish(self.to_raw_none()) }
    }

    pub fn flush(&self) {
        unsafe { ffi::cairo_device_flush(self.to_raw_none()) }
    }

    pub fn get_type(&self) -> DeviceType {
        unsafe { DeviceType::from(ffi::cairo_device_get_type(self.to_raw_none())) }
    }

    // Maybe improve this API?
    pub fn acquire(&self) -> Status {
        unsafe { Status::from(ffi::cairo_device_acquire(self.to_raw_none())) }
    }

    // Maybe improve this API?
    pub fn release(&self) {
        unsafe { ffi::cairo_device_release(self.to_raw_none()) }
    }

    pub fn observer_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_elapsed(self.to_raw_none()) }
    }

    pub fn observer_fill_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_fill_elapsed(self.to_raw_none()) }
    }

    pub fn observer_glyphs_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_glyphs_elapsed(self.to_raw_none()) }
    }

    pub fn observer_mask_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_mask_elapsed(self.to_raw_none()) }
    }

    pub fn observer_paint_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_paint_elapsed(self.to_raw_none()) }
    }

    pub fn observer_stroke_elapsed(&self) -> f64 {
        unsafe { ffi::cairo_device_observer_stroke_elapsed(self.to_raw_none()) }
    }

    #[cfg(any(feature = "xlib", feature = "xcb", feature = "dox"))]
    pub fn debug_cap_xrender_version(&self, major_version: i32, minor_version: i32) {
        unsafe {
            match self.get_type() {
                DeviceType::Xlib => {
                    #[cfg(feature = "xlib")]
                    {
                        ffi::cairo_xlib_device_debug_cap_xrender_version(
                            self.to_raw_none(),
                            major_version,
                            minor_version,
                        )
                    }
                    #[cfg(not(feature = "xlib"))]
                    {
                        panic!("you need to enable \"xlib\" feature")
                    }
                }
                DeviceType::Xcb => {
                    #[cfg(feature = "xcb")]
                    {
                        ffi::cairo_xcb_device_debug_cap_xrender_version(
                            self.to_raw_none(),
                            major_version,
                            minor_version,
                        )
                    }
                    #[cfg(not(feature = "xcb"))]
                    {
                        panic!("you need to enable \"xcb\" feature")
                    }
                }
                d => panic!("invalid device type: {}", d),
            }
        }
    }

    #[cfg(any(feature = "xlib", feature = "xcb", feature = "dox"))]
    pub fn debug_get_precision(&self) -> i32 {
        unsafe {
            match self.get_type() {
                DeviceType::Xlib => {
                    #[cfg(feature = "xlib")]
                    {
                        ffi::cairo_xlib_device_debug_get_precision(self.to_raw_none())
                    }
                    #[cfg(not(feature = "xlib"))]
                    {
                        panic!("you need to enable \"xlib\" feature")
                    }
                }
                DeviceType::Xcb => {
                    #[cfg(feature = "xcb")]
                    {
                        ffi::cairo_xcb_device_debug_get_precision(self.to_raw_none())
                    }
                    #[cfg(not(feature = "xcb"))]
                    {
                        panic!("you need to enable \"xcb\" feature")
                    }
                }
                d => panic!("invalid device type: {}", d),
            }
        }
    }

    #[cfg(any(feature = "xlib", feature = "xcb", feature = "dox"))]
    pub fn debug_set_precision(&self, precision: i32) {
        unsafe {
            match self.get_type() {
                DeviceType::Xlib => {
                    #[cfg(feature = "xlib")]
                    {
                        ffi::cairo_xlib_device_debug_set_precision(self.to_raw_none(), precision)
                    }
                    #[cfg(not(feature = "xlib"))]
                    {
                        panic!("you need to enable \"xlib\" feature")
                    }
                }
                DeviceType::Xcb => {
                    #[cfg(feature = "xcb")]
                    {
                        ffi::cairo_xcb_device_debug_set_precision(self.to_raw_none(), precision)
                    }
                    #[cfg(not(feature = "xcb"))]
                    {
                        panic!("you need to enable \"xcb\" feature")
                    }
                }
                d => panic!("invalid device type: {}", d),
            }
        }
    }

    user_data_methods! {
        ffi::cairo_device_get_user_data,
        ffi::cairo_device_set_user_data,
    }
}

#[cfg(feature = "use_glib")]
impl<'a> ToGlibPtr<'a, *mut ffi::cairo_device_t> for Device {
    type Storage = &'a Device;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::cairo_device_t, Self> {
        Stash(self.to_raw_none(), self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::cairo_device_t {
        unsafe { ffi::cairo_device_reference(self.to_raw_none()) }
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

#[cfg(feature = "use_glib")]
gvalue_impl!(
    Device,
    ffi::cairo_device_t,
    ffi::gobject::cairo_gobject_device_get_type
);

impl Clone for Device {
    fn clone(&self) -> Device {
        unsafe { Self::from_raw_none(ffi::cairo_device_reference(self.0)) }
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        if !self.1 {
            unsafe {
                ffi::cairo_device_destroy(self.0);
            }
        }
    }
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Device")
    }
}
