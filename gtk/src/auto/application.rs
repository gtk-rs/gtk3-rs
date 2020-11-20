// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use crate::ApplicationInhibitFlags;
use crate::Window;
use gio;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib::Value;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct Application(Object<ffi::GtkApplication, ffi::GtkApplicationClass>) @extends gio::Application, @implements gio::ActionGroup, gio::ActionMap;

    match fn {
        get_type => || ffi::gtk_application_get_type(),
    }
}

#[derive(Clone, Default)]
pub struct ApplicationBuilder {
    app_menu: Option<gio::MenuModel>,
    menubar: Option<gio::MenuModel>,
    register_session: Option<bool>,
    action_group: Option<gio::ActionGroup>,
    application_id: Option<String>,
    flags: Option<gio::ApplicationFlags>,
    inactivity_timeout: Option<u32>,
    resource_base_path: Option<String>,
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Application {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref app_menu) = self.app_menu {
            properties.push(("app-menu", app_menu));
        }
        if let Some(ref menubar) = self.menubar {
            properties.push(("menubar", menubar));
        }
        if let Some(ref register_session) = self.register_session {
            properties.push(("register-session", register_session));
        }
        if let Some(ref action_group) = self.action_group {
            properties.push(("action-group", action_group));
        }
        if let Some(ref application_id) = self.application_id {
            properties.push(("application-id", application_id));
        }
        if let Some(ref flags) = self.flags {
            properties.push(("flags", flags));
        }
        if let Some(ref inactivity_timeout) = self.inactivity_timeout {
            properties.push(("inactivity-timeout", inactivity_timeout));
        }
        if let Some(ref resource_base_path) = self.resource_base_path {
            properties.push(("resource-base-path", resource_base_path));
        }
        let ret = glib::Object::new(Application::static_type(), &properties)
            .expect("object new")
            .downcast::<Application>()
            .expect("downcast");
        {
            Application::register_startup_hook(&ret);
        }
        ret
    }

    pub fn app_menu<P: IsA<gio::MenuModel>>(mut self, app_menu: &P) -> Self {
        self.app_menu = Some(app_menu.clone().upcast());
        self
    }

    pub fn menubar<P: IsA<gio::MenuModel>>(mut self, menubar: &P) -> Self {
        self.menubar = Some(menubar.clone().upcast());
        self
    }

    pub fn register_session(mut self, register_session: bool) -> Self {
        self.register_session = Some(register_session);
        self
    }

    pub fn action_group<P: IsA<gio::ActionGroup>>(mut self, action_group: &P) -> Self {
        self.action_group = Some(action_group.clone().upcast());
        self
    }

    pub fn application_id(mut self, application_id: &str) -> Self {
        self.application_id = Some(application_id.to_string());
        self
    }

    pub fn flags(mut self, flags: gio::ApplicationFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn inactivity_timeout(mut self, inactivity_timeout: u32) -> Self {
        self.inactivity_timeout = Some(inactivity_timeout);
        self
    }

    pub fn resource_base_path(mut self, resource_base_path: &str) -> Self {
        self.resource_base_path = Some(resource_base_path.to_string());
        self
    }
}

pub const NONE_APPLICATION: Option<&Application> = None;

pub trait GtkApplicationExt: 'static {
    fn add_window<P: IsA<Window>>(&self, window: &P);

    fn get_accels_for_action(&self, detailed_action_name: &str) -> Vec<glib::GString>;

    fn get_actions_for_accel(&self, accel: &str) -> Vec<glib::GString>;

    fn get_active_window(&self) -> Option<Window>;

    fn get_app_menu(&self) -> Option<gio::MenuModel>;

    fn get_menu_by_id(&self, id: &str) -> Option<gio::Menu>;

    fn get_menubar(&self) -> Option<gio::MenuModel>;

    fn get_window_by_id(&self, id: u32) -> Option<Window>;

    fn get_windows(&self) -> Vec<Window>;

    fn inhibit<P: IsA<Window>>(
        &self,
        window: Option<&P>,
        flags: ApplicationInhibitFlags,
        reason: Option<&str>,
    ) -> u32;

    fn is_inhibited(&self, flags: ApplicationInhibitFlags) -> bool;

    fn list_action_descriptions(&self) -> Vec<glib::GString>;

    fn prefers_app_menu(&self) -> bool;

    fn remove_window<P: IsA<Window>>(&self, window: &P);

    fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]);

    fn set_app_menu<P: IsA<gio::MenuModel>>(&self, app_menu: Option<&P>);

    fn set_menubar<P: IsA<gio::MenuModel>>(&self, menubar: Option<&P>);

    fn uninhibit(&self, cookie: u32);

    fn get_property_register_session(&self) -> bool;

    fn set_property_register_session(&self, register_session: bool);

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    fn get_property_screensaver_active(&self) -> bool;

    #[cfg(any(feature = "v3_24_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24_8")))]
    fn connect_query_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_window_added<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_window_removed<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_window_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_app_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_register_session_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    fn connect_property_screensaver_active_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Application>> GtkApplicationExt for O {
    fn add_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_application_add_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }

    fn get_accels_for_action(&self, detailed_action_name: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_accels_for_action(
                self.as_ref().to_glib_none().0,
                detailed_action_name.to_glib_none().0,
            ))
        }
    }

    fn get_actions_for_accel(&self, accel: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_get_actions_for_accel(
                self.as_ref().to_glib_none().0,
                accel.to_glib_none().0,
            ))
        }
    }

    fn get_active_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_active_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_app_menu(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_app_menu(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_menu_by_id(&self, id: &str) -> Option<gio::Menu> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menu_by_id(
                self.as_ref().to_glib_none().0,
                id.to_glib_none().0,
            ))
        }
    }

    fn get_menubar(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_menubar(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_window_by_id(&self, id: u32) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gtk_application_get_window_by_id(
                self.as_ref().to_glib_none().0,
                id,
            ))
        }
    }

    fn get_windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_application_get_windows(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn inhibit<P: IsA<Window>>(
        &self,
        window: Option<&P>,
        flags: ApplicationInhibitFlags,
        reason: Option<&str>,
    ) -> u32 {
        unsafe {
            ffi::gtk_application_inhibit(
                self.as_ref().to_glib_none().0,
                window.map(|p| p.as_ref()).to_glib_none().0,
                flags.to_glib(),
                reason.to_glib_none().0,
            )
        }
    }

    fn is_inhibited(&self, flags: ApplicationInhibitFlags) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_is_inhibited(
                self.as_ref().to_glib_none().0,
                flags.to_glib(),
            ))
        }
    }

    fn list_action_descriptions(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::gtk_application_list_action_descriptions(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn prefers_app_menu(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_prefers_app_menu(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_window<P: IsA<Window>>(&self, window: &P) {
        unsafe {
            ffi::gtk_application_remove_window(
                self.as_ref().to_glib_none().0,
                window.as_ref().to_glib_none().0,
            );
        }
    }

    fn set_accels_for_action(&self, detailed_action_name: &str, accels: &[&str]) {
        unsafe {
            ffi::gtk_application_set_accels_for_action(
                self.as_ref().to_glib_none().0,
                detailed_action_name.to_glib_none().0,
                accels.to_glib_none().0,
            );
        }
    }

    fn set_app_menu<P: IsA<gio::MenuModel>>(&self, app_menu: Option<&P>) {
        unsafe {
            ffi::gtk_application_set_app_menu(
                self.as_ref().to_glib_none().0,
                app_menu.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_menubar<P: IsA<gio::MenuModel>>(&self, menubar: Option<&P>) {
        unsafe {
            ffi::gtk_application_set_menubar(
                self.as_ref().to_glib_none().0,
                menubar.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn uninhibit(&self, cookie: u32) {
        unsafe {
            ffi::gtk_application_uninhibit(self.as_ref().to_glib_none().0, cookie);
        }
    }

    fn get_property_register_session(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"register-session\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `register-session` getter")
                .unwrap()
        }
    }

    fn set_property_register_session(&self, register_session: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"register-session\0".as_ptr() as *const _,
                Value::from(&register_session).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    fn get_property_screensaver_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"screensaver-active\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `screensaver-active` getter")
                .unwrap()
        }
    }

    #[cfg(any(feature = "v3_24_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24_8")))]
    fn connect_query_end<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn query_end_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"query-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    query_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_window_added<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn window_added_trampoline<P, F: Fn(&P, &Window) + 'static>(
            this: *mut ffi::GtkApplication,
            window: *mut ffi::GtkWindow,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Application::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(window),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    window_added_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_window_removed<F: Fn(&Self, &Window) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn window_removed_trampoline<P, F: Fn(&P, &Window) + 'static>(
            this: *mut ffi::GtkApplication,
            window: *mut ffi::GtkWindow,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Application::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(window),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"window-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    window_removed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_active_window_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_window_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::active-window\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_window_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_app_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_app_menu_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::app-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_app_menu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_menubar_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_menubar_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::menubar\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_menubar_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_register_session_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_register_session_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::register-session\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_register_session_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_24", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_24")))]
    fn connect_property_screensaver_active_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_screensaver_active_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkApplication,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Application>,
        {
            let f: &F = &*(f as *const F);
            f(&Application::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::screensaver-active\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_screensaver_active_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Application")
    }
}
