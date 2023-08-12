// Take a look at the license at the top of the repository in the LICENSE file.

use gdk::Atom;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GdkWaylandWindow")]
    pub struct WaylandWindow(Object<ffi::GdkWaylandWindow>) @extends gdk::Window;

    match fn {
        type_ => || ffi::gdk_wayland_window_get_type(),
    }
}

impl WaylandWindow {
    #[doc(alias = "gdk_wayland_selection_add_targets")]
    pub fn selection_add_targets(&self, selection: &Atom, targets: &[Atom]) {
        unsafe {
            ffi::gdk_wayland_selection_add_targets(
                self.to_glib_none().0,
                selection.to_glib_none().0,
                targets.len() as _,
                targets.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gdk_wayland_window_set_use_custom_surface")]
    pub fn set_use_custom_surface(&self) {
        unsafe { ffi::gdk_wayland_window_set_use_custom_surface(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_wayland_window_unexport_handle")]
    pub fn unexport_handle(&self) {
        unsafe { ffi::gdk_wayland_window_unexport_handle(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_wayland_window_export_handle")]
    pub fn export_handle<P: Fn(&Self, &str) + 'static>(&self, callback: P) -> bool {
        unsafe extern "C" fn callback_trampoline<P: Fn(&WaylandWindow, &str) + 'static>(
            window: *mut ffi::GdkWaylandWindow,
            handle: *const libc::c_char,
            user_data: glib::ffi::gpointer,
        ) {
            let window = from_glib_borrow(window);
            let handle: Borrowed<glib::GString> = from_glib_borrow(handle);
            let callback = &*(user_data as *mut P);
            (*callback)(&window, handle.as_str());
        }
        unsafe extern "C" fn destroy_notify<P: Fn(&WaylandWindow, &str) + 'static>(
            data: glib::ffi::gpointer,
        ) {
            let _ = Box::from_raw(data as *mut P);
        }
        unsafe {
            from_glib(ffi::gdk_wayland_window_export_handle(
                self.to_glib_none().0,
                Some(callback_trampoline::<P> as _),
                Box::into_raw(Box::new(callback)) as *mut _,
                Some(destroy_notify::<P> as _),
            ))
        }
    }

    #[doc(alias = "gdk_wayland_window_set_dbus_properties_libgtk_only")]
    pub fn set_dbus_properties_libgtk_only(
        &self,
        application_id: &str,
        app_menu_path: &str,
        menubar_path: &str,
        window_object_path: &str,
        application_object_path: &str,
        unique_bus_name: &str,
    ) {
        unsafe {
            ffi::gdk_wayland_window_set_dbus_properties_libgtk_only(
                self.to_glib_none().0,
                application_id.to_glib_none().0,
                app_menu_path.to_glib_none().0,
                menubar_path.to_glib_none().0,
                window_object_path.to_glib_none().0,
                application_object_path.to_glib_none().0,
                unique_bus_name.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gdk_wayland_window_set_transient_for_exported")]
    pub fn set_transient_for_exported(&self, parent_handle: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_wayland_window_set_transient_for_exported(
                self.to_glib_none().0,
                parent_handle.to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v3_24_22")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_24_22")))]
    #[doc(alias = "gdk_wayland_window_set_application_id")]
    pub fn set_application_id(&self, application_id: &str) -> bool {
        unsafe {
            from_glib(ffi::gdk_wayland_window_set_application_id(
                self.to_glib_none().0,
                application_id.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_wayland_window_announce_csd")]
    pub fn announce_csd(&self) {
        unsafe { ffi::gdk_wayland_window_announce_csd(self.to_glib_none().0) }
    }

    #[cfg(feature = "v3_24")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v3_24")))]
    #[doc(alias = "gdk_wayland_window_announce_ssd")]
    pub fn announce_ssd(&self) {
        unsafe { ffi::gdk_wayland_window_announce_ssd(self.to_glib_none().0) }
    }
}
