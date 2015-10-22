// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use screen::Screen;
use display::Display;
use ffi;

glib_wrapper! {
    /// Application launching — startup notification for applications.
    pub struct AppLaunchContext(Object<ffi::GdkAppLaunchContext>);

    match fn {
        get_type => || ffi::gdk_app_launch_context_get_type(),
    }
}

// FIXME: should inherit from GAppLaunchContext

impl AppLaunchContext {
    /// Creates a new AppLaunchContext.
    pub fn new() -> AppLaunchContext {
        unsafe { from_glib_full(ffi::gdk_app_launch_context_new()) }
    }

    /// Sets the display on which applications will be launched when using this context. See also
    /// `AppLaunchContext::set_screen()`.
    pub fn set_display(&self, display: &Display) {
        unsafe {
            ffi::gdk_app_launch_context_set_display(self.to_glib_none().0, display.to_glib_none().0)
        }
    }

    /// Sets the screen on which applications will be launched when using this context. See also
    /// `AppLaunchContext::set_display()`.
    ///
    /// If both screen and display are set, the screen takes priority. If neither screen or display
    /// are set, the default screen and display are used.
    pub fn set_screen(&self, screen: &Screen) {
        unsafe {
            ffi::gdk_app_launch_context_set_screen(self.to_glib_none().0, screen.to_glib_none().0)
        }
    }

    /// Sets the workspace on which applications will be launched when using this context when running
    /// under a window manager that supports multiple workspaces, as described in the
    /// Extended Window Manager Hints.
    ///
    /// When the workspace is not specified or desktop is set to -1, it is up to the window manager to
    /// pick one, typically it will be the current workspace.
    pub fn set_desktop(&self, desktop: i32) {
        unsafe { ffi::gdk_app_launch_context_set_desktop(self.to_glib_none().0, desktop) }
    }

    /// Sets the timestamp of self. The timestamp should ideally be taken from the event that triggered
    /// the launch.
    ///
    /// Window managers can use this information to avoid moving the focus to the newly launched application
    /// when the user is busy typing in another window. This is also known as 'focus stealing prevention'.
    pub fn set_timestamp(&self, timestamp: u32) {
        unsafe { ffi::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, timestamp) }
    }

    /*pub fn set_icon(&self, icon: GIO::Icon) {
        unsafe { ffi::gdk_app_launch_context_set_timestamp(self.to_glib_none().0, icon.to_glib_none().0) }
    }*/

    /// Sets the icon for applications that are launched with this context. The icon_name will be
    /// interpreted in the same way as the Icon field in desktop files. See also `AppLaunchContext::set_icon()`.
    ///
    /// If both icon and icon_name are set, the icon_name takes priority. If neither icon or icon_name is set,
    /// the icon is taken from either the file that is passed to launched application or from the GAppInfo for
    /// the launched application itself.
    pub fn set_icon_name(&self, icon_name: &str) {
        unsafe {
            ffi::gdk_app_launch_context_set_icon_name(self.to_glib_none().0,
                                                      icon_name.to_glib_none().0)
        }
    }
}
