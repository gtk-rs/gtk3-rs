// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Gravity;
use glib::translate::*;

/// The [Geometry](crate::Geometry) struct gives the window manager information about
/// a window’s geometry constraints. Normally you would set these on
/// the GTK+ level using `gtk_window_set_geometry_hints`. GtkWindow
/// then sets the hints on the [Window](crate::Window) it creates.
///
/// [Window::set_geometry_hints](crate::Window::set_geometry_hints) expects the hints to be fully valid already
/// and simply passes them to the window manager; in contrast,
/// `gtk_window_set_geometry_hints` performs some interpretation. For example,
/// GtkWindow will apply the hints to the geometry widget instead of the
/// toplevel window, if you set a geometry widget. Also, the
/// `min_width`/`min_height`/`max_width`/`max_height` fields may be set to -1, and
/// GtkWindow will substitute the size request of the window or geometry widget.
/// If the minimum size hint is not provided, GtkWindow will use its requisition
/// as the minimum size. If the minimum size is provided and a geometry widget is
/// set, GtkWindow will take the minimum size as the minimum size of the
/// geometry widget rather than the entire window. The base size is treated
/// similarly.
///
/// The canonical use-case for `gtk_window_set_geometry_hints` is to get a
/// terminal widget to resize properly. Here, the terminal text area should be
/// the geometry widget; GtkWindow will then automatically set the base size to
/// the size of other widgets in the terminal window, such as the menubar and
/// scrollbar. Then, the `width_inc` and `height_inc` fields should be set to the
/// size of one character in the terminal. Finally, the base size should be set
/// to the size of one character. The net effect is that the minimum size of the
/// terminal will have a 1x1 character terminal area, and only terminal sizes on
/// the “character grid” will be allowed.
///
/// Here’s an example of how the terminal example would be implemented, assuming
/// a terminal area widget called “terminal” and a toplevel window “toplevel”:
///
///
/// ```C
///     GdkGeometry hints;
///
///     hints.base_width = terminal->char_width;
///         hints.base_height = terminal->char_height;
///         hints.min_width = terminal->char_width;
///         hints.min_height = terminal->char_height;
///         hints.width_inc = terminal->char_width;
///         hints.height_inc = terminal->char_height;
///
///  gtk_window_set_geometry_hints (GTK_WINDOW (toplevel),
///                                 GTK_WIDGET (terminal),
///                                 &hints,
///                                 GDK_HINT_RESIZE_INC |
///                                 GDK_HINT_MIN_SIZE |
///                                 GDK_HINT_BASE_SIZE);
/// ```
///
/// The other useful fields are the `min_aspect` and `max_aspect` fields; these
/// contain a width/height ratio as a floating point number. If a geometry widget
/// is set, the aspect applies to the geometry widget rather than the entire
/// window. The most common use of these hints is probably to set `min_aspect` and
/// `max_aspect` to the same value, thus forcing the window to keep a constant
/// aspect ratio.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Geometry {
    pub min_width: i32,
    pub min_height: i32,
    pub max_width: i32,
    pub max_height: i32,
    pub base_width: i32,
    pub base_height: i32,
    pub width_inc: i32,
    pub height_inc: i32,
    pub min_aspect: f64,
    pub max_aspect: f64,
    pub win_gravity: Gravity,
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GdkGeometry> for Geometry {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GdkGeometry, Self> {
        let ptr: *const Geometry = &*self;
        Stash(ptr as *const ffi::GdkGeometry, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GdkGeometry> for Geometry {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GdkGeometry, Self> {
        let ptr: *mut Geometry = &mut *self;
        StashMut(ptr as *mut ffi::GdkGeometry, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GdkGeometry> for Geometry {
    unsafe fn from_glib_none(ptr: *const ffi::GdkGeometry) -> Self {
        *(ptr as *const Geometry)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkGeometry> for Geometry {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkGeometry) -> Self {
        *(ptr as *mut Geometry)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkGeometry> for Geometry {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GdkGeometry) -> Self {
        let geom = *(ptr as *mut Geometry);
        glib::ffi::g_free(ptr as *mut _);
        geom
    }
}
