// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use gio_sys;
use glib::translate::*;
use std::num::NonZeroU32;
use BusNameOwnerFlags;
use BusNameWatcherFlags;
use BusType;
use DBusConnection;

#[derive(Debug, Eq, PartialEq)]
pub struct OwnerId(NonZeroU32);
#[derive(Debug, Eq, PartialEq)]
pub struct WatcherId(NonZeroU32);

fn own_closure<F>(f: F) -> glib::Closure
where
    F: Fn(DBusConnection, &str) + Send + Sync + 'static,
{
    glib::Closure::new(move |args| {
        let conn = args[0].get::<DBusConnection>().unwrap().unwrap();
        let name = args[1].get::<&str>().unwrap().unwrap();
        f(conn, name);
        None
    })
}

fn watch_closure<F>(f: F) -> glib::Closure
where
    F: Fn(DBusConnection, &str, &str) + Send + Sync + 'static,
{
    glib::Closure::new(move |args| {
        let conn = args[0].get::<DBusConnection>().unwrap().unwrap();
        let name = args[1].get::<&str>().unwrap().unwrap();
        let name_owner = args[2].get::<&str>().unwrap().unwrap();
        f(conn, name, name_owner);
        None
    })
}

pub fn bus_own_name_on_connection<NameAcquired, NameLost>(
    connection: &DBusConnection,
    name: &str,
    flags: BusNameOwnerFlags,
    name_acquired: NameAcquired,
    name_lost: NameLost,
) -> OwnerId
where
    NameAcquired: Fn(DBusConnection, &str) + Send + Sync + 'static,
    NameLost: Fn(DBusConnection, &str) + Send + Sync + 'static,
{
    unsafe {
        let id = gio_sys::g_bus_own_name_on_connection_with_closures(
            connection.to_glib_none().0,
            name.to_glib_none().0,
            flags.to_glib(),
            own_closure(name_acquired).to_glib_none().0,
            own_closure(name_lost).to_glib_none().0,
        );
        OwnerId(NonZeroU32::new_unchecked(id))
    }
}

pub fn bus_own_name<BusAcquired, NameAcquired, NameLost>(
    bus_type: BusType,
    name: &str,
    flags: BusNameOwnerFlags,
    bus_acquired: BusAcquired,
    name_acquired: NameAcquired,
    name_lost: NameLost,
) -> OwnerId
where
    BusAcquired: Fn(DBusConnection, &str) + Send + Sync + 'static,
    NameAcquired: Fn(DBusConnection, &str) + Send + Sync + 'static,
    NameLost: Fn(Option<DBusConnection>, &str) + Send + Sync + 'static,
{
    unsafe {
        let id = gio_sys::g_bus_own_name_with_closures(
            bus_type.to_glib(),
            name.to_glib_none().0,
            flags.to_glib(),
            own_closure(bus_acquired).to_glib_none().0,
            own_closure(name_acquired).to_glib_none().0,
            glib::Closure::new(move |args| {
                let conn = args[0].get::<DBusConnection>().unwrap();
                let name = args[1].get::<&str>().unwrap().unwrap();
                name_lost(conn, name);
                None
            })
            .to_glib_none()
            .0,
        );
        OwnerId(NonZeroU32::new_unchecked(id))
    }
}

pub fn bus_unown_name(owner_id: OwnerId) {
    unsafe {
        gio_sys::g_bus_unown_name(owner_id.0.into());
    }
}

pub fn bus_watch_name_on_connection<NameAppeared, NameVanished>(
    connection: &DBusConnection,
    name: &str,
    flags: BusNameWatcherFlags,
    name_appeared: NameAppeared,
    name_vanished: NameVanished,
) -> WatcherId
where
    NameAppeared: Fn(DBusConnection, &str, &str) + Send + Sync + 'static,
    NameVanished: Fn(DBusConnection, &str, &str) + Send + Sync + 'static,
{
    unsafe {
        let id = gio_sys::g_bus_watch_name_on_connection_with_closures(
            connection.to_glib_none().0,
            name.to_glib_none().0,
            flags.to_glib(),
            watch_closure(name_appeared).to_glib_none().0,
            watch_closure(name_vanished).to_glib_none().0,
        );
        WatcherId(NonZeroU32::new_unchecked(id))
    }
}

pub fn bus_watch_name<NameAppeared, NameVanished>(
    bus_type: BusType,
    name: &str,
    flags: BusNameWatcherFlags,
    name_appeared: NameAppeared,
    name_vanished: NameVanished,
) -> WatcherId
where
    NameAppeared: Fn(DBusConnection, &str, &str) + Send + Sync + 'static,
    NameVanished: Fn(DBusConnection, &str, &str) + Send + Sync + 'static,
{
    unsafe {
        let id = gio_sys::g_bus_watch_name_with_closures(
            bus_type.to_glib(),
            name.to_glib_none().0,
            flags.to_glib(),
            watch_closure(name_appeared).to_glib_none().0,
            watch_closure(name_vanished).to_glib_none().0,
        );
        WatcherId(NonZeroU32::new_unchecked(id))
    }
}

pub fn bus_unwatch_name(watcher_id: WatcherId) {
    unsafe {
        gio_sys::g_bus_unwatch_name(watcher_id.0.into());
    }
}
