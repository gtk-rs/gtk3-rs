// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Action, ActionMap};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString, IsA, ObjectExt, Quark};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub trait ActionMapImpl: ObjectImpl {
    fn lookup_action(&self, action_map: &Self::Type, action_name: &str) -> Option<Action>;
    fn add_action(&self, action_map: &Self::Type, action: &Action);
    fn remove_action(&self, action_map: &Self::Type, action_name: &str);
}

pub trait ActionMapImplExt: ObjectSubclass {
    fn parent_lookup_action(&self, action_map: &Self::Type, action_name: &str) -> Option<Action>;
    fn parent_add_action(&self, action_map: &Self::Type, action: &Action);
    fn parent_remove_action(&self, action_map: &Self::Type, action_name: &str);
}

impl<T: ActionMapImpl> ActionMapImplExt for T {
    fn parent_lookup_action(&self, action_map: &Self::Type, name: &str) -> Option<Action> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<ActionMap>()
                as *const ffi::GActionMapInterface;

            let func = (*parent_iface)
                .lookup_action
                .expect("no parent \"lookup_action\" implementation");
            let ret = func(
                action_map.unsafe_cast_ref::<ActionMap>().to_glib_none().0,
                name.to_glib_none().0,
            );
            from_glib_none(ret)
        }
    }

    fn parent_add_action(&self, action_map: &Self::Type, action: &Action) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<ActionMap>()
                as *const ffi::GActionMapInterface;

            let func = (*parent_iface)
                .add_action
                .expect("no parent \"add_action\" implementation");
            func(
                action_map.unsafe_cast_ref::<ActionMap>().to_glib_none().0,
                action.to_glib_none().0,
            );
        }
    }

    fn parent_remove_action(&self, action_map: &Self::Type, action_name: &str) {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().get_parent_interface::<ActionMap>()
                as *const ffi::GActionMapInterface;

            let func = (*parent_iface)
                .remove_action
                .expect("no parent \"remove_action\" implementation");
            func(
                action_map.unsafe_cast_ref::<ActionMap>().to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }
}

unsafe impl<T: ActionMapImpl> IsImplementable<T> for ActionMap
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.lookup_action = Some(action_map_lookup_action::<T>);
        iface.add_action = Some(action_map_add_action::<T>);
        iface.remove_action = Some(action_map_remove_action::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

static ACTION_MAP_LOOKUP_ACTION_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-map-lookup-action"));

unsafe extern "C" fn action_map_lookup_action<T: ActionMapImpl>(
    action_map: *mut ffi::GActionMap,
    action_nameptr: *const libc::c_char,
) -> *mut ffi::GAction {
    let instance = &*(action_map as *mut T::Instance);
    let action_name = GString::from_glib_borrow(action_nameptr);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow::<_, ActionMap>(action_map);

    let ret = imp.lookup_action(wrap.unsafe_cast_ref(), &action_name);
    if let Some(action) = ret {
        let actionptr = action.to_glib_full();

        let mut map = wrap
            .steal_qdata::<HashMap<String, *mut ffi::GAction>>(*ACTION_MAP_LOOKUP_ACTION_QUARK)
            .unwrap_or_else(HashMap::new);
        map.insert(action_name.to_string(), actionptr);
        wrap.set_qdata(*ACTION_MAP_LOOKUP_ACTION_QUARK, map);

        actionptr
    } else {
        std::ptr::null_mut()
    }
}

unsafe extern "C" fn action_map_add_action<T: ActionMapImpl>(
    action_map: *mut ffi::GActionMap,
    actionptr: *mut ffi::GAction,
) {
    let instance = &*(action_map as *mut T::Instance);
    let imp = instance.get_impl();
    let action: Borrowed<Action> = from_glib_borrow(actionptr);

    imp.add_action(
        from_glib_borrow::<_, ActionMap>(action_map).unsafe_cast_ref(),
        &action,
    );
}

unsafe extern "C" fn action_map_remove_action<T: ActionMapImpl>(
    action_map: *mut ffi::GActionMap,
    action_nameptr: *const libc::c_char,
) {
    let instance = &*(action_map as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);

    imp.remove_action(
        from_glib_borrow::<_, ActionMap>(action_map).unsafe_cast_ref(),
        &action_name,
    );
}
