// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Atom;
use crate::Device;
use crate::DragAction;
#[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
use crate::DragCancelReason;
use crate::DragProtocol;
use crate::Window;
#[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
use glib::object::IsA;
#[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
#[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
use std::mem::transmute;

glib::glib_wrapper! {
    pub struct DragContext(Object<ffi::GdkDragContext>);

    match fn {
        get_type => || ffi::gdk_drag_context_get_type(),
    }
}

impl DragContext {
    pub fn get_actions(&self) -> DragAction {
        unsafe { from_glib(ffi::gdk_drag_context_get_actions(self.to_glib_none().0)) }
    }

    pub fn get_dest_window(&self) -> Window {
        unsafe { from_glib_none(ffi::gdk_drag_context_get_dest_window(self.to_glib_none().0)) }
    }

    pub fn get_device(&self) -> Device {
        unsafe { from_glib_none(ffi::gdk_drag_context_get_device(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
    pub fn get_drag_window(&self) -> Option<Window> {
        unsafe { from_glib_none(ffi::gdk_drag_context_get_drag_window(self.to_glib_none().0)) }
    }

    pub fn get_protocol(&self) -> DragProtocol {
        unsafe { from_glib(ffi::gdk_drag_context_get_protocol(self.to_glib_none().0)) }
    }

    pub fn get_selected_action(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_selected_action(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_source_window(&self) -> Window {
        unsafe {
            from_glib_none(ffi::gdk_drag_context_get_source_window(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_suggested_action(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_suggested_action(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn list_targets(&self) -> Vec<Atom> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gdk_drag_context_list_targets(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
    pub fn manage_dnd<P: IsA<Window>>(&self, ipc_window: &P, actions: DragAction) -> bool {
        unsafe {
            from_glib(ffi::gdk_drag_context_manage_dnd(
                self.to_glib_none().0,
                ipc_window.as_ref().to_glib_none().0,
                actions.to_glib(),
            ))
        }
    }

    pub fn set_device(&self, device: &Device) {
        unsafe {
            ffi::gdk_drag_context_set_device(self.to_glib_none().0, device.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
    pub fn set_hotspot(&self, hot_x: i32, hot_y: i32) {
        unsafe {
            ffi::gdk_drag_context_set_hotspot(self.to_glib_none().0, hot_x, hot_y);
        }
    }

    #[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
    pub fn connect_action_changed<F: Fn(&DragContext, DragAction) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn action_changed_trampoline<
            F: Fn(&DragContext, DragAction) + 'static,
        >(
            this: *mut ffi::GdkDragContext,
            action: ffi::GdkDragAction,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(action))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"action-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    action_changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
    pub fn connect_cancel<F: Fn(&DragContext, DragCancelReason) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cancel_trampoline<F: Fn(&DragContext, DragCancelReason) + 'static>(
            this: *mut ffi::GdkDragContext,
            reason: ffi::GdkDragCancelReason,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), from_glib(reason))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
    pub fn connect_dnd_finished<F: Fn(&DragContext) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn dnd_finished_trampoline<F: Fn(&DragContext) + 'static>(
            this: *mut ffi::GdkDragContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"dnd-finished\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    dnd_finished_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v3_20", all(not(doctest), doc)))]
    #[cfg_attr(all(not(doctest), doc), doc(cfg(feature = "v3_20")))]
    pub fn connect_drop_performed<F: Fn(&DragContext, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn drop_performed_trampoline<F: Fn(&DragContext, i32) + 'static>(
            this: *mut ffi::GdkDragContext,
            time: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), time)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drop-performed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drop_performed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DragContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DragContext")
    }
}
