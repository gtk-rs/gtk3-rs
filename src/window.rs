// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::ptr;
use libc::{c_char, c_int};
use cairo::Surface;
use gdk_pixbuf;
use glib::object::IsA;
use glib::translate::*;
use ffi;
use Cursor;
use Visual;
use Window;

use {
    WindowHints,
    WindowType,
    WindowTypeHint,
    WindowWindowClass,
};

pub struct WindowAttr {
    pub title: Option<String>,
    pub event_mask: i32,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub width: i32,
    pub height: i32,
    pub wclass: WindowWindowClass,
    pub visual: Option<Visual>,
    pub window_type: WindowType,
    pub cursor: Option<Cursor>,
    pub override_redirect: bool,
    pub type_hint: Option<WindowTypeHint>,
}

impl Default for WindowAttr {
    fn default() -> WindowAttr {
        skip_assert_initialized!();
        WindowAttr {
            title: None,
            event_mask: 0,
            x: None,
            y: None,
            width: 400,
            height: 300,
            wclass: WindowWindowClass::InputOutput,
            visual: None,
            window_type: WindowType::Toplevel,
            cursor: None,
            override_redirect: false,
            type_hint: None,
        }
    }
}

impl WindowAttr {
    fn get_mask(&self) -> u32 {
        let mut mask = ffi::GdkWindowAttributesType::empty();
        if self.title.is_some() { mask.insert(ffi::GDK_WA_TITLE); }
        if self.x.is_some() { mask.insert(ffi::GDK_WA_X); }
        if self.y.is_some() { mask.insert(ffi::GDK_WA_Y); }
        if self.cursor.is_some() { mask.insert(ffi::GDK_WA_CURSOR); }
        if self.visual.is_some() { mask.insert(ffi::GDK_WA_VISUAL); }
        if self.override_redirect { mask.insert(ffi::GDK_WA_NOREDIR); }
        if self.type_hint.is_some() { mask.insert(ffi::GDK_WA_TYPE_HINT); }
        mask.bits()
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GdkWindowAttr> for WindowAttr {
    type Storage = (
        Box<ffi::GdkWindowAttr>,
        Stash<'a, *mut ffi::GdkVisual, Option<Visual>>,
        Stash<'a, *mut ffi::GdkCursor, Option<Cursor>>,
        Stash<'a, *const c_char, Option<String>>,
    );

    fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GdkWindowAttr, Self> {
        let title = self.title.to_glib_none();
        let visual = self.visual.to_glib_none();
        let cursor = self.cursor.to_glib_none();

        let mut attrs = Box::new(ffi::GdkWindowAttr {
            title: title.0 as *mut c_char,
            event_mask: self.event_mask,
            x: self.x.unwrap_or(0),
            y: self.y.unwrap_or(0),
            width: self.width,
            height: self.height,
            wclass: self.wclass.to_glib(),
            visual: visual.0,
            window_type: self.window_type.to_glib(),
            cursor: cursor.0,
            wmclass_name: ptr::null_mut(),
            wmclass_class: ptr::null_mut(),
            override_redirect: self.override_redirect.to_glib(),
            type_hint: self.type_hint.unwrap_or(WindowTypeHint::Normal).to_glib(),
        });

        Stash(&mut *attrs, (attrs, visual, cursor, title))
    }
}

impl Window {
    pub fn new(parent: Option<&Window>, attributes: &WindowAttr) -> Window {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gdk_window_new(
                parent.to_glib_none().0,
                attributes.to_glib_none().0,
                attributes.get_mask() as c_int))
        }
    }
}

pub trait WindowExtManual {
    unsafe fn set_user_data<T>(&self, user_data: &mut T);

    unsafe fn get_user_data<'a, T>(&'a self) -> &'a mut T;

    fn set_geometry_hints(&self, geometry: &ffi::GdkGeometry, geom_mask: WindowHints);

    fn get_default_root_window() -> Window;

    fn offscreen_window_set_embedder(&self, embedder: &Window);

    fn offscreen_window_get_embedder(&self) -> Option<Window>;

    fn offscreen_window_get_surface(&self) -> Option<Surface>;

    fn get_pixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) -> Option<gdk_pixbuf::Pixbuf>;
}

impl<O: IsA<Window>> WindowExtManual for O {
    unsafe fn set_user_data<T>(&self, user_data: &mut T) {
        ffi::gdk_window_set_user_data(self.to_glib_none().0, ::std::mem::transmute(user_data))
    }

    unsafe fn get_user_data<'a, T>(&'a self) -> &'a mut T {
        let mut pointer = ::std::ptr::null_mut();
        ffi::gdk_window_get_user_data(self.to_glib_none().0, &mut pointer);
        ::std::mem::transmute(pointer)
    }

    fn set_geometry_hints(&self, geometry: &ffi::GdkGeometry, geom_mask: WindowHints) {
        unsafe { ffi::gdk_window_set_geometry_hints(self.to_glib_none().0, geometry, geom_mask.to_glib()) }
    }

    fn get_default_root_window() -> Window {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gdk_get_default_root_window()) }
    }

    fn offscreen_window_set_embedder(&self, embedder: &Window) {
        unsafe {
            ffi::gdk_offscreen_window_set_embedder(self.to_glib_none().0, embedder.to_glib_none().0)
        }
    }

    fn offscreen_window_get_embedder(&self) -> Option<Window> {
        unsafe { from_glib_none(ffi::gdk_offscreen_window_get_embedder(self.to_glib_none().0)) }
    }

    fn offscreen_window_get_surface(&self) -> Option<Surface> {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::gdk_offscreen_window_get_surface(self.to_glib_none().0))
        }
    }

    fn get_pixbuf(&self, src_x: i32, src_y: i32, width: i32, height: i32) -> Option<gdk_pixbuf::Pixbuf> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gdk_pixbuf_get_from_window(self.to_glib_none().0, src_x, src_y, width, height))
        }
    }
}
