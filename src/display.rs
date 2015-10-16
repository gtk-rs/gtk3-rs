// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::*;
use atom::Atom;
use app_launch_context::AppLaunchContext;
use device::Device;
use device_manager::DeviceManager;
use screen::Screen;
use window::Window;

glib_wrapper! {
    /// Controls a set of `Screen`s and their associated input devices.
    ///
    /// Display objects purpose are two fold:
    /// * To manage and provide information about input devices (pointers and keyboards)
    /// * To manage and provide information about the available Screens
    ///
    /// Display objects are the GDK representation of an X Display, which can be described as
    /// a workstation consisting of a keyboard, a pointing device (such as a mouse) and one or
    /// more screens. It is used to open and keep track of various GdkScreen objects currently
    /// instantiated by the application. It is also used to access the keyboard(s) and mouse
    /// pointer(s) of the display.
    ///
    /// Most of the input device handling has been factored out into the separate `DeviceManager`
    /// object. Every display has a device manager, which you can obtain using
    /// `Display::get_device_manager()`.
    pub struct Display(Object<ffi::GdkDisplay>);

    match fn {
        get_type => || ffi::gdk_display_get_type(),
    }
}

impl Display {
    /// Opens a display.
    pub fn open(display_name: &str) -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_display_open(display_name.to_glib_none().0)) }
    }

    /// Gets the default `Display`. This is a convenience function for:
    /// `DisplayManager::get_default_display(DisplayManager::get())`.
    pub fn get_default() -> Option<Display> {
        unsafe { from_glib_none(ffi::gdk_display_get_default()) }
    }

    /// Gets the name of the display.
    pub fn get_name(&self) -> String {
        unsafe { from_glib_none(ffi::gdk_display_get_name(self.to_glib_none().0)) }
    }

    /// Returns a screen object for one of the screens of the display.
    pub fn get_screen(&self, screen_num: i32) -> Screen {
        unsafe { from_glib_none(ffi::gdk_display_get_screen(self.to_glib_none().0, screen_num)) }
    }

    /// Get the default `Screen` for `self`.
    pub fn get_default_screen(&self) -> Screen {
        unsafe {
            from_glib_none(ffi::gdk_display_get_default_screen(self.to_glib_none().0))
        }
    }

    /// Returns the `DeviceManager` associated to `self`.
    pub fn get_device_manager(&self) -> Option<DeviceManager> {
        unsafe { from_glib_none(ffi::gdk_display_get_device_manager(self.to_glib_none().0)) }
    }

    /// Returns true if there is an ongoing grab on `device` for `self`.
    pub fn device_is_grabbed(&self, device: &Device) -> bool {
        unsafe {
            from_glib(ffi::gdk_display_device_is_grabbed(self.to_glib_none().0,
                      device.to_glib_none().0))
        }
    }

    /// Emits a short beep on `self`.
    pub fn beep(&self) {
        unsafe { ffi::gdk_display_beep(self.to_glib_none().0) }
    }

    /// Flushes any requests queued for the windowing system and waits until all requests
    /// have been handled. This is often used for making sure that the display is
    /// synchronized with the current state of the program. Calling `Display::sync()`
    /// before `gdk::error_trap_pop()` makes sure that any errors generated from earlier
    /// requests are handled before the error trap is removed.
    ///
    /// This is most useful for X11. On windowing systems where requests are handled
    /// synchronously, this function will do nothing.
    pub fn sync(&self) {
        unsafe { ffi::gdk_display_sync(self.to_glib_none().0) }
    }

    /// Flushes any requests queued for the windowing system; this happens automatically
    /// when the main loop blocks waiting for new events, but if your application is drawing
    /// without returning control to the main loop, you may need to call this function
    /// explicitly. A common case where this function needs to be called is when an
    /// application is executing drawing commands from a thread other than the thread where
    /// the main loop is running.
    ///
    /// This is most useful for X11. On windowing systems where requests are handled
    /// synchronously, this function will do nothing.
    pub fn flush(&self) {
        unsafe { ffi::gdk_display_flush(self.to_glib_none().0) }
    }

    /// Closes the connection to the windowing system for the given display, and cleans
    /// up associated resources.
    pub fn close(&self) {
        unsafe { ffi::gdk_display_close(self.to_glib_none().0) }
    }

    /// Finds out if the display has been closed.
    pub fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_is_closed(self.to_glib_none().0)) }
    }

    /*pub fn get_event(&self) -> Option<::Event> {
        unsafe { ffi::gdk_display_get_event(self.to_glib_none().0) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::Event::wrap(tmp)) }
        }
    }

    pub fn peek_event(&self) -> Option<::Event> {
        unsafe { ffi::gdk_display_peek_event(self.to_glib_none().0) };

        if tmp.is_null() {
            None
        } else {
            unsafe { Some(::Event::wrap(tmp)) }
        }
    }

    pub fn put_event(&self, event: &::Event) {
        unsafe { ffi::gdk_display_put_event(self.to_glib_none().0, event.to_glib_none().0 as *const ffi::GdkEvent) }
    }*/

    /// Returns whether the display has events that are waiting to be processed.
    pub fn has_pending(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_has_pending(self.to_glib_none().0)) }
    }

    /// Sets the double click time (two clicks within this time interval count as a
    /// double click and result in a GDK_2BUTTON_PRESS event). Applications should not
    /// set this, it is a global user-configured setting.
    pub fn set_double_click_time(&self, msec: u32) {
        unsafe { ffi::gdk_display_set_double_click_time(self.to_glib_none().0, msec) }
    }

    /// Sets the double click distance (two clicks within this distance count as a double
    /// click and result in a GDK_2BUTTON_PRESS event). See also
    /// `Display::set_double_click_time()`. Applications should not set this, it is a
    /// global user-configured setting.
    pub fn set_double_click_distance(&self, msec: u32) {
        unsafe { ffi::gdk_display_set_double_click_distance(self.to_glib_none().0, msec) }
    }

    /// Returns true if multicolored cursors are supported on `self`. Otherwise, cursors
    /// have only a forground and a background color.
    pub fn supports_cursor_color(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_cursor_color(self.to_glib_none().0)) }
    }

    /// Returns true if cursors can use an 8bit alpha channel on `self`. Otherwise, cursors
    /// are restricted to bilevel alpha (i.e. a mask).
    pub fn supports_cursor_alpha(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_cursor_alpha(self.to_glib_none().0)) }
    }

    /// Returns the default size to use for cursors on `self`.
    pub fn get_default_cursor_size(&self) -> u32 {
        unsafe { ffi::gdk_display_get_default_cursor_size(self.to_glib_none().0) }
    }

    /// Gets the maximal size to use for cursors on `self`.
    pub fn get_maximal_cursor_size(&self, width: &mut u32, height: &mut u32) {
        unsafe { ffi::gdk_display_get_maximal_cursor_size(self.to_glib_none().0, width, height) }
    }

    /// Returns the default group leader window for all toplevel windows on `self`. This
    /// window is implicitly created by GDK. See `Window::set_group()`.
    pub fn get_default_group(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_display_get_default_group(self.to_glib_none().0)) }
    }

    /// Returns whether `EventOwnerChange` events will be sent when the owner of a
    /// selection changes.
    pub fn supports_selection_notification(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_selection_notification(self.to_glib_none().0)) }
    }

    /// Request `EventOwnerChange` events for ownership changes of the selection named
    /// by the given atom.
    pub fn request_selection_notification(&self, selection: &Atom) -> bool {
        unsafe {
            from_glib(
                ffi::gdk_display_request_selection_notification(self.to_glib_none().0,
                    selection.to_glib_none().0))
        }
    }

    /// Returns whether the speicifed display supports clipboard persistance; i.e. if
    /// it’s possible to store the clipboard data after an application has quit. On X11
    /// this checks if a clipboard daemon is running.
    pub fn supports_clipboard_persistence(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_clipboard_persistence(self.to_glib_none().0)) }
    }

    /*pub fn store_clipboard(&self, clipboard_window: &::Window, time_: u32, targets: Vec<Atom>) {
        unsafe { ffi::gdk_display_store_clipboard(self.to_glib_none().0, clipboard_window.to_glib_none().0, time_, targets.as_mut_pointer(),
            targets.len() as c_int) }
    }*/

    /// Returns true if `Window::shape_combine_mask()` can be used to create shaped windows
    /// on `self`.
    pub fn supports_shapes(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_shapes(self.to_glib_none().0)) }
    }

    /// Returns true if `Window::input_shape_combine_mask()` can be used to modify the input
    /// shape of windows on `self`.
    pub fn supports_input_shapes(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_input_shapes(self.to_glib_none().0)) }
    }

    /// Returns true if `Window::set_composited()` can be used to redirect drawing on the
    /// window using compositing.
    ///
    /// Currently this only works on X11 with XComposite and XDamage extensions available.
    ///
    /// Deprecated since GDK 3.16
    pub fn supports_composite(&self) -> bool {
        unsafe { from_glib(ffi::gdk_display_supports_composite(self.to_glib_none().0)) }
    }

    /// Returns a `AppLaunchContext` suitable for launching applications on the given
    /// display.
    pub fn get_app_launch_context(&self) -> AppLaunchContext {
        unsafe { from_glib_full(ffi::gdk_display_get_app_launch_context(self.to_glib_none().0)) }
    }

    /// Indicates to the GUI environment that the application has finished loading, using
    /// a given identifier.
    ///
    /// GTK+ will call this function automatically for GtkWindow with custom startup-notification
    /// identifier unless `gtk::Window::set_auto_startup_notification()` is called to disable
    /// that feature.
    pub fn notify_startup_complete(&self, startup_id: &str) {
        unsafe {
            ffi::gdk_display_notify_startup_complete(self.to_glib_none().0,
                                                     startup_id.to_glib_none().0)
        }
    }
}
