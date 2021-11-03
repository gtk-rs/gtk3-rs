#[cfg(any(feature = "wayland", feature = "dox"))]
use glib_sys::{gconstpointer, GType};
#[cfg(any(feature = "wayland", feature = "dox"))]
use libc::{c_char, c_void};

#[repr(C)]
#[cfg(any(feature = "wayland", feature = "dox"))]
pub struct GdkWaylandWindow(c_void);

#[cfg(any(feature = "wayland", feature = "dox"))]
extern "C" {

    //=========================================================================
    // GdkWaylandWindow
    //=========================================================================
    pub fn gdk_wayland_window_get_type() -> GType;

    pub fn gdk_wayland_window_set_use_custom_surface(window: *mut GdkWaylandWindow);

    pub fn gdk_wayland_window_get_wl_surface(window: *mut GdkWaylandWindow) -> gconstpointer;

    pub fn gdk_wayland_window_set_dbus_properties_libgtk_only(
        window: *mut GdkWaylandWindow,
        application_id: *const c_char,
        app_menu_path: *const c_char,
        menubar_path: *const c_char,
        window_object_path: *const c_char,
        application_object_path: *const c_char,
        unique_bus_name: *const c_char,
    );

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn gdk_wayland_window_unexport_handle(window: *mut GdkWaylandWindow);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn gdk_wayland_window_export_handle(
        window: *mut GdkWaylandWindow,
        cb: Option<unsafe extern "C" fn(*mut GdkWaylandWindow, *const c_char, *mut c_void)>,
        user_data: *mut c_void,
        destroy_notify: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> glib_sys::gboolean;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn gdk_wayland_window_set_transient_for_exported(
        window: *mut GdkWaylandWindow,
        parent_handle: *const c_char,
    ) -> glib_sys::gboolean;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    pub fn gdk_wayland_window_set_application_id(
        window: *mut GdkWaylandWindow,
        application_id: *const c_char,
    ) -> glib_sys::gboolean;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_22")))]
    pub fn gdk_wayland_window_announce_csd(window: *mut GdkWaylandWindow);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    pub fn gdk_wayland_window_announce_ssd(window: *mut GdkWaylandWindow);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    pub fn gdk_wayland_window_add_frame_callback_surface(
        window: *mut GdkWaylandWindow,
        surface: gconstpointer,
    );

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    pub fn gdk_wayland_window_remove_frame_callback_surface(
        window: *mut GdkWaylandWindow,
        surface: gconstpointer,
    );
}
