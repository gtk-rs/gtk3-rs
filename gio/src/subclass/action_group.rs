// Take a look at the license at the top of the repository in the LICENSE file.

use crate::ActionGroup;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString, ObjectExt, Quark, Variant, VariantType};
use once_cell::sync::Lazy;
use std::ptr;

pub trait ActionGroupImpl: ObjectImpl {
    fn action_added(&self, action_group: &Self::Type, action_name: &str) {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).action_added.as_ref() {
                f(
                    action_group
                        .unsafe_cast_ref::<ActionGroup>()
                        .to_glib_none()
                        .0,
                    action_name.to_glib_none().0,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn action_enabled_changed(&self, action_group: &Self::Type, action_name: &str, enabled: bool) {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).action_enabled_changed.as_ref() {
                f(
                    action_group
                        .unsafe_cast_ref::<ActionGroup>()
                        .to_glib_none()
                        .0,
                    action_name.to_glib_none().0,
                    enabled.to_glib(),
                );
            }
            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn action_removed(&self, action_group: &Self::Type, action_name: &str) {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).action_removed.as_ref() {
                f(
                    action_group
                        .unsafe_cast_ref::<ActionGroup>()
                        .to_glib_none()
                        .0,
                    action_name.to_glib_none().0,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn action_state_changed(&self, action_group: &Self::Type, action_name: &str, state: &Variant) {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).action_state_changed.as_ref() {
                f(
                    action_group
                        .unsafe_cast_ref::<ActionGroup>()
                        .to_glib_none()
                        .0,
                    action_name.to_glib_none().0,
                    state.to_glib_none().0,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn activate_action(
        &self,
        action_group: &Self::Type,
        action_name: &str,
        parameter: Option<&Variant>,
    ) {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            let f = (*iface).activate_action.as_ref().unwrap();

            f(
                action_group
                    .unsafe_cast_ref::<ActionGroup>()
                    .to_glib_none()
                    .0,
                action_name.to_glib_none().0,
                parameter.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn change_action_state(&self, action_group: &Self::Type, action_name: &str, value: &Variant) {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            let f = (*iface).change_action_state.as_ref().unwrap();

            f(
                action_group
                    .unsafe_cast_ref::<ActionGroup>()
                    .to_glib_none()
                    .0,
                action_name.to_glib_none().0,
                value.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn get_action_enabled(&self, action_group: &Self::Type, action_name: &str) -> bool {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            let f = (*iface).get_action_enabled.as_ref().unwrap();

            let ret = f(
                action_group
                    .unsafe_cast_ref::<ActionGroup>()
                    .to_glib_none()
                    .0,
                action_name.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            from_glib(ret)
        }
    }

    fn get_action_parameter_type(
        &self,
        action_group: &Self::Type,
        action_name: &str,
    ) -> Option<VariantType> {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            let f = (*iface).get_action_parameter_type.as_ref().unwrap();

            let ret = f(
                action_group
                    .unsafe_cast_ref::<ActionGroup>()
                    .to_glib_none()
                    .0,
                action_name.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            from_glib_full(ret)
        }
    }

    fn get_action_state(&self, action_group: &Self::Type, action_name: &str) -> Option<Variant> {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            let f = (*iface).get_action_state.as_ref().unwrap();

            let ret = f(
                action_group
                    .unsafe_cast_ref::<ActionGroup>()
                    .to_glib_none()
                    .0,
                action_name.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            from_glib_full(ret)
        }
    }

    fn get_action_state_hint(
        &self,
        action_group: &Self::Type,
        action_name: &str,
    ) -> Option<Variant> {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            let f = (*iface).get_action_state_hint.as_ref().unwrap();

            let ret = f(
                action_group
                    .unsafe_cast_ref::<ActionGroup>()
                    .to_glib_none()
                    .0,
                action_name.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            from_glib_full(ret)
        }
    }

    fn get_action_state_type(
        &self,
        action_group: &Self::Type,
        action_name: &str,
    ) -> Option<VariantType> {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            let f = (*iface).get_action_state_type.as_ref().unwrap();

            let ret = f(
                action_group
                    .unsafe_cast_ref::<ActionGroup>()
                    .to_glib_none()
                    .0,
                action_name.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            from_glib_full(ret)
        }
    }

    fn has_action(&self, action_group: &Self::Type, action_name: &str) -> bool {
        unsafe {
            let type_ = ffi::g_action_group_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GActionGroupInterface;
            assert!(!iface.is_null());

            let f = (*iface).has_action.as_ref().unwrap();

            let ret = f(
                action_group
                    .unsafe_cast_ref::<ActionGroup>()
                    .to_glib_none()
                    .0,
                action_name.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            from_glib(ret)
        }
    }

    fn list_actions(&self, action_group: &Self::Type) -> Vec<String>;
    fn query_action(
        &self,
        action_group: &Self::Type,
        action_name: &str,
    ) -> Option<(
        bool,
        Option<VariantType>,
        Option<VariantType>,
        Option<Variant>,
        Option<Variant>,
    )>;
}

unsafe impl<T: ActionGroupImpl> IsImplementable<T> for ActionGroup {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let action_group_iface = &mut *(iface as *mut ffi::GActionGroupInterface);

        action_group_iface.action_added = Some(action_group_action_added::<T>);
        action_group_iface.action_enabled_changed = Some(action_group_action_enabled_changed::<T>);
        action_group_iface.action_removed = Some(action_group_action_removed::<T>);
        action_group_iface.action_state_changed = Some(action_group_action_state_changed::<T>);
        action_group_iface.activate_action = Some(action_group_activate_action::<T>);
        action_group_iface.change_action_state = Some(action_group_change_action_state::<T>);
        action_group_iface.get_action_enabled = Some(action_group_get_action_enabled::<T>);
        action_group_iface.get_action_parameter_type =
            Some(action_group_get_action_parameter_type::<T>);
        action_group_iface.get_action_state = Some(action_group_get_action_state::<T>);
        action_group_iface.get_action_state_hint = Some(action_group_get_action_state_hint::<T>);
        action_group_iface.get_action_state_type = Some(action_group_get_action_state_type::<T>);
        action_group_iface.has_action = Some(action_group_has_action::<T>);
        action_group_iface.list_actions = Some(action_group_list_actions::<T>);
        action_group_iface.query_action = Some(action_group_query_action::<T>);
    }
}

unsafe extern "C" fn action_group_has_action<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
) -> glib::ffi::gboolean {
    let instance = &*(action_group as *mut T::Instance);
    let action_name = GString::from_glib_borrow(action_nameptr);
    let imp = instance.get_impl();

    imp.has_action(
        from_glib_borrow::<_, ActionGroup>(action_group).unsafe_cast_ref(),
        &action_name,
    )
    .to_glib()
}

unsafe extern "C" fn action_group_get_action_enabled<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
) -> glib::ffi::gboolean {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);

    imp.get_action_enabled(
        from_glib_borrow::<_, ActionGroup>(action_group).unsafe_cast_ref(),
        &action_name,
    )
    .to_glib()
}

static ACTION_GROUP_GET_ACTION_PARAMETER_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-group-get-action-parameter"));

unsafe extern "C" fn action_group_get_action_parameter_type<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
) -> *const glib::ffi::GVariantType {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);
    let wrap = from_glib_borrow::<_, ActionGroup>(action_group);

    let ret = imp.get_action_parameter_type(wrap.unsafe_cast_ref(), &action_name);

    if let Some(param_type) = ret {
        let param_type = param_type.to_glib_full();
        wrap.set_qdata(*ACTION_GROUP_GET_ACTION_PARAMETER_QUARK, param_type);
        param_type
    } else {
        ptr::null()
    }
}

static ACTION_GROUP_GET_ACTION_STATE_TYPE_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-group-get-action-state-type"));

unsafe extern "C" fn action_group_get_action_state_type<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
) -> *const glib::ffi::GVariantType {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);
    let wrap = from_glib_borrow::<_, ActionGroup>(action_group);

    let ret = imp.get_action_state_type(wrap.unsafe_cast_ref(), &action_name);

    if let Some(state_type) = ret {
        let state_type = state_type.to_glib_full();
        wrap.set_qdata(*ACTION_GROUP_GET_ACTION_STATE_TYPE_QUARK, state_type);
        state_type
    } else {
        ptr::null()
    }
}

static ACTION_GROUP_GET_ACTION_STATE_HINT_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-group-get-action-state-hint"));

unsafe extern "C" fn action_group_get_action_state_hint<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
) -> *mut glib::ffi::GVariant {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);

    let wrap = from_glib_borrow::<_, ActionGroup>(action_group);

    let ret = imp.get_action_state_hint(wrap.unsafe_cast_ref(), &action_name);
    if let Some(state_hint) = ret {
        let state_hint_ptr = state_hint.to_glib_full();
        wrap.set_qdata(*ACTION_GROUP_GET_ACTION_STATE_HINT_QUARK, state_hint_ptr);
        state_hint_ptr
    } else {
        ptr::null_mut()
    }
}
static ACTION_GROUP_GET_ACTION_STATE_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-group-get-action-state"));

unsafe extern "C" fn action_group_get_action_state<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
) -> *mut glib::ffi::GVariant {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);
    let wrap = from_glib_borrow::<_, ActionGroup>(action_group);

    let ret = imp.get_action_state(wrap.unsafe_cast_ref(), &action_name);
    if let Some(state) = ret {
        let state_ptr = state.to_glib_full();
        wrap.set_qdata(*ACTION_GROUP_GET_ACTION_STATE_QUARK, state_ptr);
        state_ptr
    } else {
        ptr::null_mut()
    }
}

unsafe extern "C" fn action_group_change_action_state<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
    stateptr: *mut glib::ffi::GVariant,
) {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);
    let state = Variant::from_glib_borrow(stateptr);

    imp.change_action_state(
        from_glib_borrow::<_, ActionGroup>(action_group).unsafe_cast_ref(),
        &action_name,
        &state,
    )
}

unsafe extern "C" fn action_group_activate_action<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
    parameterptr: *mut glib::ffi::GVariant,
) {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);
    let param: Borrowed<Option<Variant>> = from_glib_borrow(parameterptr);

    imp.activate_action(
        from_glib_borrow::<_, ActionGroup>(action_group).unsafe_cast_ref(),
        &action_name,
        param.as_ref().as_ref(),
    )
}

unsafe extern "C" fn action_group_action_added<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
) {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);

    imp.action_added(
        from_glib_borrow::<_, ActionGroup>(action_group).unsafe_cast_ref(),
        &action_name,
    )
}

unsafe extern "C" fn action_group_action_removed<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
) {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);

    imp.action_removed(
        from_glib_borrow::<_, ActionGroup>(action_group).unsafe_cast_ref(),
        &action_name,
    )
}

unsafe extern "C" fn action_group_action_enabled_changed<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
    enabled: glib::ffi::gboolean,
) {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);

    imp.action_enabled_changed(
        from_glib_borrow::<_, ActionGroup>(action_group).unsafe_cast_ref(),
        &action_name,
        from_glib(enabled),
    )
}

unsafe extern "C" fn action_group_action_state_changed<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
    stateptr: *mut glib::ffi::GVariant,
) {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);
    let state = Variant::from_glib_borrow(stateptr);

    imp.action_state_changed(
        from_glib_borrow::<_, ActionGroup>(action_group).unsafe_cast_ref(),
        &action_name,
        &state,
    )
}

static ACTION_GROUP_LIST_ACTIONS_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-group-list-actions"));

unsafe extern "C" fn action_group_list_actions<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
) -> *mut *mut libc::c_char {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow::<_, ActionGroup>(action_group);

    let actions = imp.list_actions(wrap.unsafe_cast_ref());

    {
        let actionsptr = actions.to_glib_full();
        wrap.set_qdata(*ACTION_GROUP_LIST_ACTIONS_QUARK, actionsptr);
        actionsptr
    }
}

static ACTION_GROUP_QUERY_ACTION_PARAM_TYPE_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-group-query-action-parameter-type"));

static ACTION_GROUP_QUERY_ACTION_STATE_TYPE_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-group-query-action-state-type"));

static ACTION_GROUP_QUERY_ACTION_STATE_HINT_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-group-query-action-state-hint"));

static ACTION_GROUP_QUERY_ACTION_STATE_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk-rs-subclass-action-group-query-action-state"));

unsafe extern "C" fn action_group_query_action<T: ActionGroupImpl>(
    action_group: *mut ffi::GActionGroup,
    action_nameptr: *const libc::c_char,
    enabled: *mut glib::ffi::gboolean,
    parameter_type: *mut *const glib::ffi::GVariantType,
    state_type: *mut *const glib::ffi::GVariantType,
    state_hint: *mut *mut glib::ffi::GVariant,
    state: *mut *mut glib::ffi::GVariant,
) -> glib::ffi::gboolean {
    let instance = &*(action_group as *mut T::Instance);
    let imp = instance.get_impl();
    let action_name = GString::from_glib_borrow(action_nameptr);
    let wrap = from_glib_borrow::<_, ActionGroup>(action_group);

    let ret = imp.query_action(wrap.unsafe_cast_ref(), &action_name);
    if let Some((rs_enabled, rs_parameter_type, rs_state_type, rs_state_hint, rs_state)) = ret {
        if !enabled.is_null() {
            *enabled = rs_enabled.to_glib();
        }
        if !parameter_type.is_null() {
            if let Some(rs_parameter_type) = rs_parameter_type {
                let ret = rs_parameter_type.to_glib_full();
                wrap.set_qdata(*ACTION_GROUP_QUERY_ACTION_PARAM_TYPE_QUARK, ret);
                *parameter_type = ret;
            } else {
                *parameter_type = ptr::null_mut();
            }
        }
        if !state_type.is_null() {
            if let Some(rs_state_type) = rs_state_type {
                let ret = rs_state_type.to_glib_full();
                wrap.set_qdata(*ACTION_GROUP_QUERY_ACTION_STATE_TYPE_QUARK, ret);
                *state_type = ret;
            } else {
                *state_type = ptr::null_mut();
            }
        }
        if !state_hint.is_null() {
            if let Some(rs_state_hint) = rs_state_hint {
                let ret = rs_state_hint.to_glib_full();
                wrap.set_qdata(*ACTION_GROUP_QUERY_ACTION_STATE_HINT_QUARK, ret);
                *state_hint = ret;
            } else {
                *state_hint = ptr::null_mut();
            }
        }
        if !state.is_null() {
            if let Some(rs_state) = rs_state {
                let ret = rs_state.to_glib_full();
                wrap.set_qdata(*ACTION_GROUP_QUERY_ACTION_STATE_QUARK, ret);
                *state = ret;
            } else {
                *state = ptr::null_mut();
            }
        }
        true
    } else {
        false
    }
    .to_glib()
}
