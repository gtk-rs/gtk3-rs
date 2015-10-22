// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use cursor::Cursor;
use display::Display;
use screen::Screen;
use window::Window;
use ffi;

glib_wrapper! {
    /// Object representing an input device.
    pub struct Device(Object<ffi::GdkDevice>);

    match fn {
        get_type => || ffi::gdk_device_get_type(),
    }
}

pub type Type = ffi::GdkDeviceType;

impl Device {
    /// Determines the name of the device.
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_name(self.to_glib_none().0))
        }
    }

    /// Returns the vendor ID of this device, or NULL if this information couldn't be obtained. This
    /// ID is retrieved from the device, and is thus constant for it.
    ///
    /// This function, together with gdk_device_get_product_id(), can be used to eg. compose GSettings
    /// paths to store settings for this device.
    ///
    /// ```ignore
    /// fn get_device_settings(device: &Device) -> GSettings {
    ///     GSettings *settings;
    ///     GdkDevice *device;
    ///     gchar *path;
    /// 
    ///     let vendor = device.get_vendor_id().unwrap();
    ///     let product = device.get_product_id().unwrap();
    /// 
    ///     let path = format!("/org/example/app/devices/{}:{}/", vendor, product);
    ///     g_settings_new_with_path(DEVICE_SCHEMA, path);
    /// }
    /// ```
    #[cfg(gdk_3_16)]
    pub fn get_vendor_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_vendor_id(self.to_glib_none().0))
        }
    }

    /// Returns the product ID of this device, or None if this information couldn't be obtained. This
    /// ID is retrieved from the device, and is thus constant for it. See Device::get_vendor_id() for
    /// more information.
    #[cfg(gdk_3_16)]
    pub fn get_product_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gdk_device_get_product_id(self.to_glib_none().0))
        }
    }

    /// Determines the type of the device.
    pub fn get_source(&self) -> ::InputSource {
        unsafe { ffi::gdk_device_get_source(self.to_glib_none().0) }
    }

    /// Sets a the mode of an input device. The mode controls if the device is active and whether the
    /// device’s range is mapped to the entire screen or to a single window.
    ///
    /// Note: This is only meaningful for floating devices, master devices (and slaves connected to
    /// these) drive the pointer cursor, which is not limited by the input mode.
    pub fn set_mode(&self, mode: ::InputMode) {
        unsafe { ffi::gdk_device_set_mode(self.to_glib_none().0, mode); }
    }

    /// Determines the mode of the device.
    pub fn get_mode(&self) -> ::InputMode {
        unsafe { ffi::gdk_device_get_mode(self.to_glib_none().0) }
    }

    /// Specifies the X key event to generate when a macro button of a device is pressed.
    pub fn set_key(&self, index_: u32, keyval: u32, modifiers: ::ModifierType) {
        unsafe { ffi::gdk_device_set_key(self.to_glib_none().0, index_, keyval, modifiers) }
    }

    /// If `index_` has a valid keyval, this function will return true and fill in `keyval` and
    /// `modifiers` with the keyval settings.
    pub fn get_key(&self, index_: u32, keyval: &mut u32, modifiers: &mut ::ModifierType) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_key(self.to_glib_none().0, index_, keyval, modifiers)) }
    }

    /// Specifies how an axis of a device is used.
    pub fn set_axis_use(&self, index_: u32, use_: ::AxisUse) {
        unsafe { ffi::gdk_device_set_axis_use(self.to_glib_none().0, index_, use_) }
    }

    /// Returns the axis use for `index_`.
    pub fn get_axis_use(&self, index_: u32) -> ::AxisUse {
        unsafe { ffi::gdk_device_get_axis_use(self.to_glib_none().0, index_) }
    }

    /// Returns the associated `self` to `self` , if `self` is of type DeviceType::Master, it
    /// will return the paired pointer or keyboard.
    ///
    /// If `self` is of type DeviceType::Slave, it will return the master device to which `self`
    /// is attached to.
    ///
    /// If `self` is of type DeviceType::Floating, None will be returned, as there is no
    /// associated device.
    pub fn get_associated_device(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::gdk_device_get_associated_device(self.to_glib_none().0)) }
    }

    /// Returns the device type for `self`.
    pub fn get_device_type(&self) -> Type {
        unsafe { ffi::gdk_device_get_device_type(self.to_glib_none().0) }
    }

    /// Returns the Display to which `self` pertains.
    pub fn get_display(&self) -> Display {
        unsafe { from_glib_none(ffi::gdk_device_get_display(self.to_glib_none().0)) }
    }

    /// Determines whether the pointer follows device motion. This is not meaningful for
    /// keyboard devices, which don't have a pointer.
    pub fn get_has_cursor(&self) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_has_cursor(self.to_glib_none().0)) }
    }

    /// Returns the number of axes the device currently has.
    pub fn get_n_axes(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_axes(self.to_glib_none().0) }
    }

    /// Returns the number of keys the device currently has.
    pub fn get_n_keys(&self) -> i32 {
        unsafe { ffi::gdk_device_get_n_keys(self.to_glib_none().0) }
    }

    /// Warps `self` in display to the point `x`, `y` on the screen `screen`, unless the device
    /// is confined to a window by a grab, in which case it will be moved as far as allowed by
    /// the grab. Warping the pointer creates events as if the user had moved the mouse
    /// instantaneously to the destination.
    ///
    /// Note that the pointer should normally be under the control of the user. This function
    /// was added to cover some rare use cases like keyboard navigation support for the color
    /// picker in the gtk::ColorSelectionDialog.
    pub fn warp(&self, screen: &Screen, x: i32, y: i32) {
        unsafe { ffi::gdk_device_warp(self.to_glib_none().0, screen.to_glib_none().0, x, y) }
    }

    /// Grabs the device so that all events coming from this device are passed to this
    /// application until the device is ungrabbed with Device::ungrab(), or the window becomes
    /// unviewable. This overrides any previous grab on the device by this client.
    ///
    /// Note that `self` and `window` need to be on the same display.
    ///
    /// Device grabs are used for operations which need complete control over the given device
    /// events (either pointer or keyboard). For example in GTK+ this is used for Drag and Drop
    /// operations, popup menus and such.
    ///
    /// Note that if the event mask of an X window has selected both button press and button
    /// release events, then a button press event will cause an automatic pointer grab until
    /// the button is released. X does this automatically since most applications expect to
    /// receive button press and release events in pairs. It is equivalent to a pointer grab
    /// on the window with `owner_events` set to true.
    ///
    /// If you set up anything at the time you take the grab that needs to be cleaned up when
    /// the grab ends, you should handle the GdkEventGrabBroken events that are emitted when
    /// the grab ends unvoluntarily.
    pub fn grab(&self, window: &Window, grab_ownership: ::GrabOwnership, owner_events: bool,
                event_mask: ::EventMask, cursor: &Cursor, time_: u32) -> ::GrabStatus {
        unsafe {
            ffi::gdk_device_grab(self.to_glib_none().0, window.to_glib_none().0, grab_ownership,
                owner_events.to_glib(), event_mask, cursor.to_glib_none().0, time_)
        }
    }

    /// Release any grab on `self`.
    pub fn ungrab(&self, time_: u32) {
        unsafe { ffi::gdk_device_ungrab(self.to_glib_none().0, time_) }
    }

    /*pub fn get_state(&self, window: &::Window, axes: &mut [f64], mask: &mut gdk;:ModifierType) {
        unsafe { ffi::gdk_device_get_state(self.to_glib_none().0, window.unwrap_pointer(), axes.as_mut_ptr(), mask) }
    }

    pub fn get_position(&self, x: &mut i32, y: &mut i32) -> Option<::Screen> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_position(self.to_glib_none().0, &mut ptr, x as *mut c_int, y as *mut c_int) };
        if ptr.is_null() {
            None
        } else {
            Some(::Screen::wrap_pointer(ptr))
        }
    }

    pub fn get_position_double(&self, x: &mut f64, y: &mut f64) -> Option<::Screen> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_position_double(self.to_glib_none().0, &mut ptr, x as *mut c_double, y as *mut c_double) };
        if ptr.is_null() {
            None
        } else {
            Some(::Screen::wrap_pointer(ptr))
        }
    }

    pub fn get_window_at_position(&self, x: &mut i32, y: &mut i32) -> Option<::Window> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_window_at_position(self.to_glib_none().0, &mut ptr, x as *mut c_int, y as *mut c_int) };
        if ptr.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(ptr))
        }
    }

    pub fn get_window_at_position_double(&self, x: &mut f64, y: &mut f64) -> Option<::Window> {
        let mut ptr = ::std::ptr::null_mut();

        unsafe { ffi::gdk_device_get_window_at_position_double(self.to_glib_none().0, &mut ptr, x as *mut c_double, y as *mut c_double) };
        if ptr.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(ptr))
        }
    }

    pub fn get_history(&self, window: &::Window, start: u32, stop: u32) -> Vec<::TimeCoord> {
        let mut ptr = ::std::ptr::null_mut();
        let mut n_events : c_int = 0;

        unsafe { ffi::gdk_device_get_history(self.to_glib_none().0, window.unwrap_pointer(), start, stop, &mut ptr, &mut n_events) };
        
        let mut ret = Vec::with_capacity(n_events as uint);
        
        for i in range(0, n_events) {
            ret.push(::TimeCoord::wrap_pointer(::std::ptr::read(ptr.offset(i))));
        }
        ret
    }

    pub fn free_history(events: &[::TimeCoord]) {
        let mut tmp = Vec::with_capacity(events.len());

        for i in range(0, events.len()) {
            tmp.push(events[i].unwrap_pointer());
        }
        unsafe { ffi::gdk_device_free_history(events.as_mut_ptr(), events.len()) }
    }*/

    /// Interprets an array of double as axis values for a given device, and locates the value
    /// in the array for a given axis use.
    pub fn get_axis(&self, axes: &mut [f64], use_: ::AxisUse, value: &mut f64) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_axis(self.to_glib_none().0, axes.as_mut_ptr(), use_, value)) }
    }

    /*pub fn get_axis_value(&self, axes: &mut [f64], label: &mut ::Atom, value: &mut f64) -> bool {
        unsafe { from_glib(ffi::gdk_device_get_axis_value(self.to_glib_none().0, axes.as_mut_ptr(), label.unwrap_pointer(), value)) }
    }*/

    /*pub fn get_last_event_window(&self) -> Option<::Window> {
        let ptr = unsafe { ffi::gdk_device_get_last_event_window(self.to_glib_none().0) };

        if ptr.is_null() {
            None
        } else {
            Some(::Window::wrap_pointer(ptr))
        }
    }*/
}
