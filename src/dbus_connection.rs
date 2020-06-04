// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::num::NonZeroU32;
use ActionGroup;
use DBusConnection;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use DBusInterfaceInfo;
use DBusMessage;
#[cfg(any(feature = "v2_46", feature = "dox"))]
use DBusMethodInvocation;
use DBusSignalFlags;
use MenuModel;

#[derive(Debug, Eq, PartialEq)]
pub struct RegistrationId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct WatcherId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct ActionGroupExportId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct MenuModelExportId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct FilterId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct SignalSubscriptionId(NonZeroU32);

impl DBusConnection {
    #[cfg(any(feature = "v2_46", feature = "dox"))]
    pub fn register_object<MethodCall, SetProperty, GetProperty>(
        &self,
        object_path: &str,
        interface_info: &DBusInterfaceInfo,
        method_call: MethodCall,
        get_property: GetProperty,
        set_property: SetProperty,
    ) -> Result<RegistrationId, glib::Error>
    where
        MethodCall: Fn(DBusConnection, &str, &str, &str, &str, glib::Variant, DBusMethodInvocation)
            + Send
            + Sync
            + 'static,
        GetProperty:
            Fn(DBusConnection, &str, &str, &str, &str) -> glib::Variant + Send + Sync + 'static,
        SetProperty: Fn(DBusConnection, &str, &str, &str, &str, glib::Variant) -> bool
            + Send
            + Sync
            + 'static,
    {
        use glib::ToValue;
        unsafe {
            let mut error = std::ptr::null_mut();
            let id = gio_sys::g_dbus_connection_register_object_with_closures(
                self.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_info.to_glib_none().0,
                glib::Closure::new(move |args| {
                    let conn = args[0].get::<DBusConnection>().unwrap().unwrap();
                    let sender = args[1].get::<&str>().unwrap().unwrap();
                    let object_path = args[2].get::<&str>().unwrap().unwrap();
                    let interface_name = args[3].get::<&str>().unwrap().unwrap();
                    let method_name = args[4].get::<&str>().unwrap().unwrap();
                    let parameters = args[5].get::<glib::Variant>().unwrap().unwrap();
                    let invocation = args[6].get::<DBusMethodInvocation>().unwrap().unwrap();
                    method_call(
                        conn,
                        sender,
                        object_path,
                        interface_name,
                        method_name,
                        parameters,
                        invocation,
                    );
                    None
                })
                .to_glib_none()
                .0,
                glib::Closure::new(move |args| {
                    let conn = args[0].get::<DBusConnection>().unwrap().unwrap();
                    let sender = args[1].get::<&str>().unwrap().unwrap();
                    let object_path = args[2].get::<&str>().unwrap().unwrap();
                    let interface_name = args[3].get::<&str>().unwrap().unwrap();
                    let property_name = args[4].get::<&str>().unwrap().unwrap();
                    let result =
                        get_property(conn, sender, object_path, interface_name, property_name);
                    Some(result.to_value())
                })
                .to_glib_none()
                .0,
                glib::Closure::new(move |args| {
                    let conn = args[0].get::<DBusConnection>().unwrap().unwrap();
                    let sender = args[1].get::<&str>().unwrap().unwrap();
                    let object_path = args[2].get::<&str>().unwrap().unwrap();
                    let interface_name = args[3].get::<&str>().unwrap().unwrap();
                    let property_name = args[4].get::<&str>().unwrap().unwrap();
                    let value = args[5].get::<glib::Variant>().unwrap().unwrap();
                    let result = set_property(
                        conn,
                        sender,
                        object_path,
                        interface_name,
                        property_name,
                        value,
                    );
                    Some(result.to_value())
                })
                .to_glib_none()
                .0,
                &mut error,
            );
            if error.is_null() {
                Ok(RegistrationId(NonZeroU32::new_unchecked(id)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn unregister_object(
        &self,
        registration_id: RegistrationId,
    ) -> Result<(), glib::error::BoolError> {
        unsafe {
            glib_result_from_gboolean!(
                gio_sys::g_dbus_connection_unregister_object(
                    self.to_glib_none().0,
                    registration_id.0.into()
                ),
                "Failed to unregister D-Bus object"
            )
        }
    }

    pub fn export_action_group<P: IsA<ActionGroup>>(
        &self,
        object_path: &str,
        action_group: &P,
    ) -> Result<ActionGroupExportId, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let id = gio_sys::g_dbus_connection_export_action_group(
                self.to_glib_none().0,
                object_path.to_glib_none().0,
                action_group.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ActionGroupExportId(NonZeroU32::new_unchecked(id)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn unexport_action_group(&self, export_id: ActionGroupExportId) {
        unsafe {
            gio_sys::g_dbus_connection_unexport_action_group(
                self.to_glib_none().0,
                export_id.0.into(),
            );
        }
    }

    pub fn export_menu_model<P: IsA<MenuModel>>(
        &self,
        object_path: &str,
        menu: &P,
    ) -> Result<MenuModelExportId, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let id = gio_sys::g_dbus_connection_export_menu_model(
                self.to_glib_none().0,
                object_path.to_glib_none().0,
                menu.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(MenuModelExportId(NonZeroU32::new_unchecked(id)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn unexport_menu_model(&self, export_id: MenuModelExportId) {
        unsafe {
            gio_sys::g_dbus_connection_unexport_menu_model(
                self.to_glib_none().0,
                export_id.0.into(),
            );
        }
    }

    pub fn add_filter<
        P: Fn(&DBusConnection, &DBusMessage, bool) -> Option<DBusMessage> + 'static,
    >(
        &self,
        filter_function: P,
    ) -> FilterId {
        let filter_function_data: Box_<P> = Box_::new(filter_function);
        unsafe extern "C" fn filter_function_func<
            P: Fn(&DBusConnection, &DBusMessage, bool) -> Option<DBusMessage> + 'static,
        >(
            connection: *mut gio_sys::GDBusConnection,
            message: *mut gio_sys::GDBusMessage,
            incoming: glib_sys::gboolean,
            user_data: glib_sys::gpointer,
        ) -> *mut gio_sys::GDBusMessage {
            let connection = from_glib_borrow(connection);
            let message = from_glib_full(message);
            let incoming = from_glib(incoming);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&connection, &message, incoming);
            res.to_glib_full()
        }
        let filter_function = Some(filter_function_func::<P> as _);
        unsafe extern "C" fn user_data_free_func_func<
            P: Fn(&DBusConnection, &DBusMessage, bool) -> Option<DBusMessage> + 'static,
        >(
            data: glib_sys::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(user_data_free_func_func::<P> as _);
        let super_callback0: Box_<P> = filter_function_data;
        unsafe {
            let id = gio_sys::g_dbus_connection_add_filter(
                self.to_glib_none().0,
                filter_function,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
            FilterId(NonZeroU32::new_unchecked(id))
        }
    }

    pub fn remove_filter(&self, filter_id: FilterId) {
        unsafe {
            gio_sys::g_dbus_connection_remove_filter(self.to_glib_none().0, filter_id.0.into());
        }
    }

    pub fn signal_subscribe<
        P: Fn(&DBusConnection, &str, &str, &str, &str, &glib::Variant) + 'static,
    >(
        &self,
        sender: Option<&str>,
        interface_name: Option<&str>,
        member: Option<&str>,
        object_path: Option<&str>,
        arg0: Option<&str>,
        flags: DBusSignalFlags,
        callback: P,
    ) -> SignalSubscriptionId {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: Fn(&DBusConnection, &str, &str, &str, &str, &glib::Variant) + 'static,
        >(
            connection: *mut gio_sys::GDBusConnection,
            sender_name: *const libc::c_char,
            object_path: *const libc::c_char,
            interface_name: *const libc::c_char,
            signal_name: *const libc::c_char,
            parameters: *mut glib_sys::GVariant,
            user_data: glib_sys::gpointer,
        ) {
            let connection = from_glib_borrow(connection);
            let sender_name: Borrowed<glib::GString> = from_glib_borrow(sender_name);
            let object_path: Borrowed<glib::GString> = from_glib_borrow(object_path);
            let interface_name: Borrowed<glib::GString> = from_glib_borrow(interface_name);
            let signal_name: Borrowed<glib::GString> = from_glib_borrow(signal_name);
            let parameters = from_glib_borrow(parameters);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(
                &connection,
                sender_name.as_str(),
                object_path.as_str(),
                interface_name.as_str(),
                signal_name.as_str(),
                &parameters,
            );
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn user_data_free_func_func<
            P: Fn(&DBusConnection, &str, &str, &str, &str, &glib::Variant) + 'static,
        >(
            data: glib_sys::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call9 = Some(user_data_free_func_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            let id = gio_sys::g_dbus_connection_signal_subscribe(
                self.to_glib_none().0,
                sender.to_glib_none().0,
                interface_name.to_glib_none().0,
                member.to_glib_none().0,
                object_path.to_glib_none().0,
                arg0.to_glib_none().0,
                flags.to_glib(),
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call9,
            );
            SignalSubscriptionId(NonZeroU32::new_unchecked(id))
        }
    }

    pub fn signal_unsubscribe(&self, subscription_id: SignalSubscriptionId) {
        unsafe {
            gio_sys::g_dbus_connection_signal_unsubscribe(
                self.to_glib_none().0,
                subscription_id.0.into(),
            );
        }
    }
}
