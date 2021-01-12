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

unsafe impl<T: ActionMapImpl> IsImplementable<T> for ActionMap
where
    <T as ObjectSubclass>::Type: IsA<glib::Object>,
{
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let action_map_iface = &mut *(iface as *mut ffi::GActionMapInterface);

        action_map_iface.lookup_action = Some(action_map_lookup_action::<T>);
        action_map_iface.add_action = Some(action_map_add_action::<T>);
        action_map_iface.remove_action = Some(action_map_remove_action::<T>);
    }
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
