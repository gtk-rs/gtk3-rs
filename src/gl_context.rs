// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

//! OpenGL context

use glib::translate::*;
use glib::types;
use cursor::Cursor;
use display::Display;
use object::Object;
use screen::Screen;
use window::Window;
use ffi;

/// GLContext is an object representing the platform-specific OpenGL drawing context.
///
/// GLContexts are created for a GdkWindow using gdk_window_create_gl_context(), and the context
/// will match the GdkVisual of the window.
///
/// A GLContext is not tied to any particular normal framebuffer. For instance, it cannot draw to
/// the GdkWindow back buffer. The GDK repaint system is in full control of the painting to that.
/// Instead, you can create render buffers or textures and use gdk_cairo_draw_from_gl() in the draw
/// function of your widget to draw them. Then GDK will handle the integration of your rendering
/// with that of other widgets.
///
/// Support for GLContext is platform-specific, context creation can fail, returning None context.
///
/// A GLContext has to be made "current" in order to start using it, otherwise any OpenGL call will
/// be ignored.
///
/// ##Creating a new OpenGL context
///
/// In order to create a new GLContext instance you need a GdkWindow, which you typically get during
/// the realize call of a widget.
///
/// A GLContext is not realized until either gdk_gl_context_make_current(), or until it is realized
/// using gdk_gl_context_realize(). It is possible to specify details of the GL context like the
/// OpenGL version to be used, or whether the GL context should have extra state validation enabled
/// after calling gdk_window_create_gl_context() by calling gdk_gl_context_realize(). If the
/// realization fails you have the option to change the settings of the GLContext and try again.
///
/// ##Using a GLContext
///
/// You will need to make the GLContext the current context before issuing OpenGL calls; the system
/// sends OpenGL commands to whichever context is current. It is possible to have multiple contexts,
/// so you always need to ensure that the one which you want to draw with is the current one before
/// issuing commands:
///
/// ```Rust,ignore
/// GLContext::make_current(context);
/// ```
///
/// You can now perform your drawing using OpenGL commands.
///
/// You can check which GLContext is the current one by using gdk_gl_context_get_current(); you can
/// also unset any GLContext that is currently set by calling gdk_gl_context_clear_current().
pub type GLContext = Object<ffi::GdkGLContext>;

impl GLContext {
    /// Retrieves the Display the context is created for
    pub fn get_display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_gl_context_get_display(self.to_glib_none().0)) }
    }

    /// Retrieves the GdkWindow used by self.
    pub fn get_window(&self) -> Option<Window> {
        unsafe { from_glib_none(ffi::gdk_gl_context_get_window(self.to_glib_none().0)) }
    }

    /// Retrieves the GLContext that this context share data with.
    pub fn get_shared_context(&self) -> Option<Window> {
        unsafe { from_glib_none(ffi::gdk_gl_context_get_shared_context(self.to_glib_none().0)) }
    }

    /// Retrieves the OpenGL version of the context.
    ///
    /// The context must be realized prior to calling this function.
    pub fn get_version(&self) -> (i32, i32) {
        let mut major = 0;
        let mut minor = 0;

        unsafe { ffi::gdk_gl_context_get_version(self.to_glib_none().0, &mut major, &mut minor) };
        (major, minor)
    }

    /// Sets the major and minor version of OpenGL to request.
    ///
    /// Setting major and minor to zero will use the default values.
    ///
    /// The GLContext must not be realized or made current prior to calling this function.
    pub fn set_required_version(&self, major: i32, minor: i32) {
        unsafe { ffi::gdk_gl_context_set_required_version(self.to_glib_none().0, major, minor) }
    }

    /// Sets whether the GLContext should perform extra validations and run time checking.
    /// This is useful during development, but has additional overhead.
    ///
    /// The GLContext must not be realized or made current prior to calling this function.
    pub fn set_debug_enabled(&self, enabled: bool) {
        unsafe { ffi::gdk_gl_context_set_debug_enabled(self.to_glib_none().0, enabled.to_glib()) }
    }

    /// Retrieves the value set using set_debug_enabled().
    pub fn get_debug_enabled(&self) -> bool {
        unsafe { from_glib(ffi::gdk_gl_context_get_debug_enabled(self.to_glib_none().0)) }
    }

    /// Sets whether the GLContext should be forward compatible.
    ///
    /// Forward compatibile contexts must not support OpenGL functionality that has been
    /// marked as deprecated in the requested version; non-forward compatible contexts, on
    /// the other hand, must support both deprecated and non deprecated functionality.
    ///
    /// The GLContext must not be realized or made current prior to calling this function.
    pub fn set_forward_compatible(&self, compatible: bool) {
        unsafe { ffi::gdk_gl_context_set_forward_compatible(self.to_glib_none().0, compatible.to_glib()) }
    }

    /// Retrieves the value set using set_forward_compatible().
    pub fn get_forward_compatible(&self) -> bool {
        unsafe { from_glib(ffi::gdk_gl_context_get_forward_compatible(self.to_glib_none().0)) }
    }

    /*
    /// Realizes the given GLContext.
    ///
    /// It is safe to call this function on a realized GLContext.
    pub fn realize(&self, *mut *mut glib::error) -> bool;
    */

    /// Makes self the current context.
    pub fn make_current(&self) {
        unsafe { ffi::gdk_gl_context_make_current(self.to_glib_none().0) }
    }

    /// Retrieves the current GLContext.
    pub fn get_current() -> Option<GLContext> {
        unsafe { from_glib_none(ffi::gdk_gl_context_get_current()) }
    }

    /// Clears the current GLContext.
    ///
    /// Any OpenGL call after this function returns will be ignored until GLContext::make_current()
    /// is called.
    pub fn clear_current() {
        unsafe { ffi::gdk_gl_context_clear_current() }
    }
}
